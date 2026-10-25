use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::rc::Rc;
use std::sync::{
    OnceLock,
    atomic::{AtomicBool, AtomicPtr, AtomicU32, Ordering},
};

use windows::{
    Win32::{
        dwmapi::{
            DWM_SYSTEMBACKDROP_TYPE, DWMSBT_AUTO, DWMSBT_MAINWINDOW, DWMSBT_TABBEDWINDOW,
            DWMSBT_TRANSIENTWINDOW, DWMWA_SYSTEMBACKDROP_TYPE, DWMWA_USE_IMMERSIVE_DARK_MODE,
            DwmSetWindowAttribute,
        },
        libloaderapi::{GetModuleHandleW, GetProcAddress},
        minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM},
        windef::{HMENU, HWND, POINT, RECT},
        winuser::{
            CREATESTRUCTW, CW_USEDEFAULT, ClientToScreen, CreateWindowExW, DefWindowProcW,
            DispatchMessageW, GWLP_USERDATA, GetClientRect, GetDpiForWindow, GetMessageW,
            GetMonitorInfoW, GetSystemMetricsForDpi, HTCAPTION, HTCLIENT, HTTOP, HTTOPLEFT,
            HTTOPRIGHT, HTTRANSPARENT, HWND_TOP, IDC_ARROW, IsZoomed, LWA_ALPHA, LoadCursorW,
            MONITOR_DEFAULTTONEAREST, MONITORINFO, MSG, MonitorFromWindow, NCCALCSIZE_PARAMS,
            PostMessageW, PostQuitMessage, RegisterClassW, SC_CLOSE, SC_MAXIMIZE, SC_MINIMIZE,
            SC_RESTORE, SM_CXPADDEDBORDER, SM_CYSIZEFRAME, SW_HIDE, SW_SHOW, SWP_FRAMECHANGED,
            SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW, SendMessageW,
            SetLayeredWindowAttributes, SetWindowLongPtrW, SetWindowPos, ShowWindow,
            TranslateMessage, WM_ACTIVATE, WM_APP, WM_DESTROY, WM_NCCALCSIZE, WM_NCCREATE,
            WM_NCHITTEST, WM_NCLBUTTONDBLCLK, WM_NCLBUTTONDOWN, WM_NCLBUTTONUP, WM_NCRBUTTONDBLCLK,
            WM_NCRBUTTONDOWN, WM_NCRBUTTONUP, WM_SIZE, WM_SYSCOMMAND, WNDCLASSW, WS_CHILD,
            WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_NOPARENTNOTIFY, WS_EX_NOREDIRECTIONBITMAP,
            WS_OVERLAPPEDWINDOW,
        },
    },
    core::{BOOL, PCSTR, PCWSTR},
};
use windows_core::Interface;

use crate::bindings::*;
use crate::winui::xaml_interop::interop::IDesktopWindowXamlSourceNative;
use crate::*;

use super::{TitleBarState, WinUIBackend, WinUIDispatcher};
use crate::native::app_shim::create_island_application;

static XAML_HWND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static CORE_HWND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static ACTIVE_HOST_HWND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static TITLEBAR_HWND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static TITLEBAR_ENABLED: AtomicBool = AtomicBool::new(false);
static TITLEBAR_HEIGHT_DIPS: AtomicU32 = AtomicU32::new(40);
static TITLEBAR_DRAG_LEFT_DIPS: AtomicU32 = AtomicU32::new(0);
static TITLEBAR_DRAG_WIDTH_DIPS: AtomicU32 = AtomicU32::new(1);
const WM_ISLANDS_REACTOR_SYSTEM_THEME_CHANGED: u32 = WM_APP as u32 + 0x491;

thread_local! {
    static ROOT_FRAMEWORK_ELEMENT: RefCell<Option<FrameworkElement>> = const { RefCell::new(None) };
    static REQUESTED_THEME: Cell<RequestedTheme> = const { Cell::new(RequestedTheme::Default) };
    static PENDING_THEME: Cell<Option<RequestedTheme>> = const { Cell::new(None) };
    static PENDING_BACKDROP: Cell<Option<Backdrop>> = const { Cell::new(None) };
    static UNHANDLED_EXCEPTION_HANDLER: RefCell<Option<windows_core::EventRevoker>> =
        const { RefCell::new(None) };
    static THEME_CHANGED_CALLBACK: RefCell<Option<Rc<dyn Fn()>>> = const { RefCell::new(None) };
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequestedTheme {
    Default,
    Light,
    Dark,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum PresenterKind {
    #[default]
    Default,
    FullScreen,
    CompactOverlay,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Backdrop {
    Mica,
    MicaAlt,
    Acrylic,
}

impl Backdrop {
    fn system_backdrop_type(self) -> DWM_SYSTEMBACKDROP_TYPE {
        match self {
            Self::Mica => DWMSBT_MAINWINDOW,
            Self::MicaAlt => DWMSBT_TABBEDWINDOW,
            Self::Acrylic => DWMSBT_TRANSIENTWINDOW,
        }
    }
}

pub fn set_requested_theme(theme: RequestedTheme) {
    REQUESTED_THEME.with(|requested| requested.set(theme));
    ROOT_FRAMEWORK_ELEMENT.with(|cell| {
        if let Some(fe) = cell.borrow().as_ref() {
            apply_requested_theme(fe, theme);
        } else {
            PENDING_THEME.with(|pending| pending.set(Some(theme)));
            update_window_theme_from_requested();
        }
    });
}

pub fn set_backdrop(backdrop: Option<Backdrop>) {
    let raw_hwnd = ACTIVE_HOST_HWND.load(Ordering::Relaxed);
    if raw_hwnd.is_null() {
        PENDING_BACKDROP.with(|pending| pending.set(backdrop));
        return;
    }

    apply_backdrop(raw_hwnd.cast(), backdrop);
}

fn pending_backdrop() -> Option<Backdrop> {
    PENDING_BACKDROP.with(|pending| pending.take())
}

fn install_xaml_exception_logging() {
    let Ok(app) = Application::Current() else {
        return;
    };
    UNHANDLED_EXCEPTION_HANDLER.with(|slot| {
        if slot.borrow().is_some() {
            return;
        }
        let revoker = app.UnhandledException(|_, args| {
            if let Some(args) = args.as_ref() {
                let hr = args.Exception().map(|v| v.0).unwrap_or_default();
                let message = args.Message().unwrap_or_default();
                crate::diagnostics::emit(&format!(
                    "islands_reactor: XAML unhandled exception 0x{hr:08X}: {message}\n"
                ));
            }
        });
        if let Ok(revoker) = revoker {
            *slot.borrow_mut() = Some(revoker);
        }
    });
}

pub struct ReactorHost {
    render_host: RenderHost<WinUIBackend, WinUIDispatcher>,
    hwnd: HWND,
    _system_theme_subscription: Option<SystemThemeSubscription>,
    _application: Application,
    _xaml_source: DesktopWindowXamlSource,
    _xaml_manager: WindowsXamlManager,
    presenter: Cell<PresenterKind>,
    backdrop: Cell<Option<Backdrop>>,
}

struct SystemThemeSubscription {
    _revoker: windows_core::EventRevoker,
}

impl ReactorHost {
    pub fn new(title: impl AsRef<str>, root: Box<dyn Component>) -> windows_core::Result<Self> {
        Self::new_with(title, root, |_| {})
    }

    pub fn new_with<F>(
        title: impl AsRef<str>,
        root: Box<dyn Component>,
        configure: F,
    ) -> windows_core::Result<Self>
    where
        F: FnOnce(&mut crate::reconcile::Reconciler<WinUIBackend>),
    {
        Self::new_with_window_options(title, None, InnerConstraints::default(), root, configure)
    }

    pub fn new_with_window_options<F>(
        title: impl AsRef<str>,
        size: Option<crate::Size>,
        _constraints: InnerConstraints,
        root: Box<dyn Component>,
        configure: F,
    ) -> windows_core::Result<Self>
    where
        F: FnOnce(&mut crate::reconcile::Reconciler<WinUIBackend>),
    {
        Self::new_with_window_options_and_backdrop(title, size, _constraints, root, None, configure)
    }

    pub(crate) fn new_with_window_options_and_backdrop<F>(
        title: impl AsRef<str>,
        size: Option<crate::Size>,
        _constraints: InnerConstraints,
        root: Box<dyn Component>,
        backdrop: Option<Backdrop>,
        configure: F,
    ) -> windows_core::Result<Self>
    where
        F: FnOnce(&mut crate::reconcile::Reconciler<WinUIBackend>),
    {
        let backdrop = backdrop.or_else(pending_backdrop);
        let application = match Application::Current() {
            Ok(application) => application,
            Err(_) => create_island_application()?,
        };
        let xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;
        install_xaml_exception_logging();
        initialize_core_window_handle();

        let hwnd = create_main_window(title.as_ref(), size, backdrop)?;
        ACTIVE_HOST_HWND.store(hwnd.cast(), Ordering::Relaxed);
        let system_theme_subscription = subscribe_system_theme_changed(hwnd);
        update_window_theme_from_requested();
        apply_backdrop(hwnd, backdrop);
        let xaml_source = DesktopWindowXamlSource::new()?;
        let native_source: IDesktopWindowXamlSourceNative = xaml_source.cast()?;
        unsafe {
            native_source.AttachToWindow(hwnd as _)?;
        }
        if backdrop.is_some() && supports_system_backdrop() {
            set_xaml_background_transparency(true);
        }
        let xaml_hwnd = unsafe { native_source.get_WindowHandle()? };
        XAML_HWND.store(xaml_hwnd.cast(), Ordering::Relaxed);
        enable_resize_layout_synchronization(hwnd, true);
        resize_xaml_island(hwnd);

        let backdrop_content_host = create_backdrop_content_host(backdrop)?;
        if let Some(content_host) = backdrop_content_host.as_ref() {
            let content_host_ui: UIElement = content_host.cast()?;
            xaml_source.SetContent(&content_host_ui)?;
        }

        let dispatcher = WinUIDispatcher::for_current_thread()?;
        let marshaller = dispatcher.marshaller();
        let render_host = RenderHost::new(WinUIBackend::new(), root, dispatcher);
        render_host.set_marshaller(Some(marshaller));
        install_theme_changed_callback(render_host.clone_inner());
        render_host.set_inner_size(client_size_dips(hwnd));
        render_host.set_dpi(current_dpi(hwnd));
        render_host.with_reconciler_mut(configure);

        let source_for_post_render = xaml_source.clone();
        let content_host_for_post_render = backdrop_content_host.clone();
        let render_for_post_render = render_host.clone_inner();
        let last_attached: Rc<Cell<Option<ControlId>>> = Rc::new(Cell::new(None));
        let last_for_hook = Rc::clone(&last_attached);
        render_host.set_post_render(move |new_id| {
            sync_custom_titlebar(
                hwnd,
                render_for_post_render.with_backend(|b| b.root_titlebar_state(new_id)),
            );
            if last_for_hook.get() == new_id {
                return;
            }
            if let Some(rid) = new_id {
                if let Some(ui) = render_for_post_render.with_backend(|b| b.get_ui_element(rid))
                    && let Ok(ui_element) = ui.cast::<UIElement>()
                {
                    if let Some(content_host) = content_host_for_post_render.as_ref() {
                        let _ = content_host.SetChild(&ui_element);
                    } else {
                        let _ = source_for_post_render.SetContent(&ui_element);
                    }
                    last_for_hook.set(Some(rid));
                    if let Ok(fe) = ui_element.cast::<FrameworkElement>() {
                        ROOT_FRAMEWORK_ELEMENT.with(|cell| {
                            *cell.borrow_mut() = Some(fe.clone());
                        });
                        if let Some(theme) = PENDING_THEME.with(|pending| pending.take()) {
                            apply_requested_theme(&fe, theme);
                        } else {
                            apply_requested_theme(&fe, requested_theme());
                        }
                        subscribe_actual_theme_changed(&fe);
                    }
                }
            } else {
                last_for_hook.set(None);
            }
        });

        render_host.kick();

        Ok(Self {
            render_host,
            hwnd,
            _system_theme_subscription: system_theme_subscription,
            _application: application,
            _xaml_manager: xaml_manager,
            _xaml_source: xaml_source,
            presenter: Cell::new(PresenterKind::Default),
            backdrop: Cell::new(backdrop),
        })
    }

    pub fn set_presenter(&self, kind: PresenterKind) {
        self.presenter.set(kind);
    }

    pub fn set_backdrop(&self, backdrop: Backdrop) {
        self.backdrop.set(Some(backdrop));
        apply_backdrop(self.hwnd, Some(backdrop));
    }

    pub fn activate(&self) -> windows_core::Result<()> {
        let _ = self.presenter.get();
        unsafe {
            let _ = ShowWindow(self.hwnd, SW_SHOW);
        }
        Ok(())
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    pub fn stats(&self) -> RenderStats {
        self.render_host.stats()
    }

    pub fn set_render_complete<F>(&self, f: F)
    where
        F: Fn(f64, f64, f64) + 'static,
    {
        self.render_host.set_render_complete(f);
    }
}

impl Drop for ReactorHost {
    fn drop(&mut self) {
        self._system_theme_subscription = None;
        self.render_host.clear_callbacks();
        let _ = self._xaml_source.SetContent(None);
        ROOT_FRAMEWORK_ELEMENT.with(|cell| {
            *cell.borrow_mut() = None;
        });
        self.render_host.with_backend(|backend| backend.shutdown());
        UNHANDLED_EXCEPTION_HANDLER.with(|slot| {
            *slot.borrow_mut() = None;
        });
        THEME_CHANGED_CALLBACK.with(|slot| {
            *slot.borrow_mut() = None;
        });
        XAML_HWND.store(std::ptr::null_mut(), Ordering::Relaxed);
        CORE_HWND.store(std::ptr::null_mut(), Ordering::Relaxed);
        ACTIVE_HOST_HWND.store(std::ptr::null_mut(), Ordering::Relaxed);
        TITLEBAR_HWND.store(std::ptr::null_mut(), Ordering::Relaxed);
        TITLEBAR_ENABLED.store(false, Ordering::Relaxed);
        TITLEBAR_HEIGHT_DIPS.store(40, Ordering::Relaxed);
    }
}

impl<B: Backend + 'static, D: Dispatcher + 'static> RenderHost<B, D> {
    pub fn with_backend<R>(&self, f: impl FnOnce(&B) -> R) -> R {
        self.with_reconciler(|r| f(&r.backend))
    }
}

pub(crate) fn message_loop() -> windows_core::Result<()> {
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}

fn create_main_window(
    title: &str,
    inner_size: Option<crate::Size>,
    backdrop: Option<Backdrop>,
) -> windows_core::Result<HWND> {
    unsafe {
        let instance: HINSTANCE = GetModuleHandleW(PCWSTR::null());
        if instance.is_null() {
            return Err(windows_core::Error::from_thread());
        }

        let cursor = LoadCursorW(None, PCWSTR(IDC_ARROW.0 as _));
        if cursor.is_null() {
            return Err(windows_core::Error::from_thread());
        }

        let class_name_buf = wide_null("IslandReactor_Window");
        let title_buf = wide_null(title);
        let class_name = PCWSTR(class_name_buf.as_ptr());

        let window_class = WNDCLASSW {
            hCursor: cursor,
            hInstance: instance,
            lpszClassName: class_name,
            lpfnWndProc: Some(wnd_proc),
            ..Default::default()
        };
        let _ = RegisterClassW(&window_class);

        let (width, height) = inner_size
            .map(|size| (size.width.round() as i32, size.height.round() as i32))
            .unwrap_or_else(|| {
                let default = default_display_size();
                (default.width.round() as i32, default.height.round() as i32)
            });

        let hwnd = CreateWindowExW(
            window_ex_style(backdrop),
            class_name,
            PCWSTR(title_buf.as_ptr()),
            WS_OVERLAPPEDWINDOW as u32,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            None,
            None::<HMENU>,
            Some(instance),
            None,
        );
        if hwnd.is_null() {
            Err(windows_core::Error::from_thread())
        } else {
            Ok(hwnd)
        }
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message as i32 {
        WM_NCCREATE => unsafe {
            let create_struct = lparam as *const CREATESTRUCTW;
            SetWindowLongPtrW(
                hwnd,
                GWLP_USERDATA,
                (*create_struct).lpCreateParams as isize,
            );
            DefWindowProcW(hwnd, message, wparam, lparam)
        },
        WM_NCCALCSIZE if custom_titlebar_enabled() => {
            if wparam == 0 {
                return 0;
            }
            unsafe {
                let params = lparam as *mut NCCALCSIZE_PARAMS;
                let original_top = (*params).rgrc[0].top;
                let result = DefWindowProcW(hwnd, message, wparam, lparam);
                if result != 0 {
                    return result;
                }
                (*params).rgrc[0].top = original_top;
            }
            0
        }
        WM_NCHITTEST if custom_titlebar_enabled() => {
            if let Some(hit) = try_hit_test_titlebar(hwnd, lparam) {
                return hit as isize;
            }
            unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
        }
        WM_ACTIVATE if custom_titlebar_enabled() => unsafe {
            DefWindowProcW(hwnd, message, wparam, lparam)
        },
        WM_SIZE => {
            resize_xaml_island(hwnd);
            resize_titlebar_overlay(hwnd);
            send_size_to_core_window(wparam, lparam);
            set_synchronization_window(hwnd);
            0
        }
        value if value == WM_ISLANDS_REACTOR_SYSTEM_THEME_CHANGED as i32 => {
            apply_current_requested_theme();
            notify_host_theme_changed();
            0
        }
        WM_DESTROY => {
            if ACTIVE_HOST_HWND.load(Ordering::Relaxed) == hwnd.cast() {
                ACTIVE_HOST_HWND.store(std::ptr::null_mut(), Ordering::Relaxed);
            }
            unsafe {
                PostQuitMessage(0);
            }
            TITLEBAR_HWND.store(std::ptr::null_mut(), Ordering::Relaxed);
            TITLEBAR_ENABLED.store(false, Ordering::Relaxed);
            TITLEBAR_HEIGHT_DIPS.store(40, Ordering::Relaxed);
            TITLEBAR_DRAG_LEFT_DIPS.store(0, Ordering::Relaxed);
            TITLEBAR_DRAG_WIDTH_DIPS.store(1, Ordering::Relaxed);
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
    }
}

fn sync_custom_titlebar(hwnd: HWND, titlebar_state: Option<TitleBarState>) {
    let enabled = titlebar_state.is_some();
    let previous = TITLEBAR_ENABLED.swap(enabled, Ordering::Relaxed);

    if enabled {
        if let Some(state) = titlebar_state {
            let metrics = state.metrics;
            TITLEBAR_HEIGHT_DIPS.store(
                metrics.height_dips.max(1.0).ceil() as u32,
                Ordering::Relaxed,
            );
            TITLEBAR_DRAG_LEFT_DIPS.store(
                metrics.drag_left_dips.max(0.0).floor() as u32,
                Ordering::Relaxed,
            );
            TITLEBAR_DRAG_WIDTH_DIPS.store(
                metrics.drag_width_dips.max(1.0).ceil() as u32,
                Ordering::Relaxed,
            );
            if let Some(titlebar_hwnd) = ensure_titlebar_overlay(hwnd) {
                unsafe {
                    let _ = ShowWindow(titlebar_hwnd, SW_SHOW);
                }
                resize_titlebar_overlay(hwnd);
            }
        }
    } else {
        let raw_titlebar = TITLEBAR_HWND.load(Ordering::Relaxed);
        if !raw_titlebar.is_null() {
            unsafe {
                let _ = ShowWindow(raw_titlebar.cast(), SW_HIDE);
            }
        }
        TITLEBAR_HEIGHT_DIPS.store(40, Ordering::Relaxed);
        TITLEBAR_DRAG_LEFT_DIPS.store(0, Ordering::Relaxed);
        TITLEBAR_DRAG_WIDTH_DIPS.store(1, Ordering::Relaxed);
    }

    if previous != enabled {
        unsafe {
            let _ = SetWindowPos(
                hwnd,
                None,
                0,
                0,
                0,
                0,
                (SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE) as u32,
            );
        }
        resize_xaml_island(hwnd);
        resize_titlebar_overlay(hwnd);
    }
}

fn custom_titlebar_enabled() -> bool {
    TITLEBAR_ENABLED.load(Ordering::Relaxed)
}

fn ensure_titlebar_overlay(parent: HWND) -> Option<HWND> {
    let raw = TITLEBAR_HWND.load(Ordering::Relaxed);
    if !raw.is_null() {
        return Some(raw.cast());
    }

    unsafe {
        let instance: HINSTANCE = GetModuleHandleW(PCWSTR::null());
        if instance.is_null() {
            return None;
        }
        let cursor = LoadCursorW(None, PCWSTR(IDC_ARROW.0 as _));
        if cursor.is_null() {
            return None;
        }

        let class_name_buf = wide_null("IslandReactor_TitleBarOverlay");
        let class_name = PCWSTR(class_name_buf.as_ptr());
        let window_class = WNDCLASSW {
            hCursor: cursor,
            hInstance: instance,
            lpszClassName: class_name,
            lpfnWndProc: Some(titlebar_overlay_wnd_proc),
            ..Default::default()
        };
        let _ = RegisterClassW(&window_class);

        let hwnd = CreateWindowExW(
            (WS_EX_LAYERED | WS_EX_NOPARENTNOTIFY | WS_EX_NOREDIRECTIONBITMAP | WS_EX_NOACTIVATE)
                as u32,
            class_name,
            PCWSTR::null(),
            WS_CHILD as u32,
            0,
            0,
            0,
            0,
            Some(parent),
            None::<HMENU>,
            Some(instance),
            None,
        );
        if hwnd.is_null() {
            return None;
        }
        let _ = SetLayeredWindowAttributes(hwnd, 0, 255, LWA_ALPHA as u32);
        TITLEBAR_HWND.store(hwnd.cast(), Ordering::Relaxed);
        Some(hwnd)
    }
}

fn resize_titlebar_overlay(parent: HWND) {
    let raw_titlebar = TITLEBAR_HWND.load(Ordering::Relaxed);
    if raw_titlebar.is_null() || !custom_titlebar_enabled() {
        return;
    }

    unsafe {
        if let Some(bounds) = titlebar_overlay_bounds_px(parent) {
            let _ = SetWindowPos(
                raw_titlebar.cast(),
                Some(HWND_TOP),
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                (SWP_NOACTIVATE | SWP_SHOWWINDOW) as u32,
            );
        }
    }
}

fn titlebar_overlay_height_px(parent: HWND) -> i32 {
    let dips = TITLEBAR_HEIGHT_DIPS.load(Ordering::Relaxed).max(1) as f64;
    (dips * current_dpi(parent) as f64 / 96.0).ceil() as i32
}

fn titlebar_overlay_bounds_px(parent: HWND) -> Option<PixelRect> {
    let mut rect = RECT::default();
    let client_width = unsafe {
        if GetClientRect(parent, &mut rect).as_bool() {
            rect.right - rect.left
        } else {
            return None;
        }
    };
    Some(PixelRect {
        x: 0,
        y: 0,
        width: client_width.max(1),
        height: titlebar_overlay_height_px(parent) + 1,
    })
}

fn manual_titlebar_drag_bounds_px(parent: HWND) -> Option<PixelRect> {
    let mut rect = RECT::default();
    let client_width = unsafe {
        if GetClientRect(parent, &mut rect).as_bool() {
            rect.right - rect.left
        } else {
            return None;
        }
    };
    let scale = current_dpi(parent) as f64 / 96.0;
    let left = (TITLEBAR_DRAG_LEFT_DIPS.load(Ordering::Relaxed) as f64 * scale).floor() as i32;
    let requested_width =
        (TITLEBAR_DRAG_WIDTH_DIPS.load(Ordering::Relaxed).max(1) as f64 * scale).ceil() as i32;
    let width = requested_width.min((client_width - left).max(1)).max(1);
    Some(PixelRect {
        x: left,
        y: 0,
        width,
        height: titlebar_overlay_height_px(parent) + 1,
    })
}

unsafe extern "system" fn titlebar_overlay_wnd_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message as i32 {
        WM_NCHITTEST => {
            let hit = try_hit_test_titlebar(parent_hwnd(), lparam).unwrap_or(HTCLIENT as i32);
            if hit == HTCLIENT as i32 {
                HTTRANSPARENT as isize
            } else {
                hit as isize
            }
        }
        WM_NCLBUTTONDOWN | WM_NCLBUTTONDBLCLK | WM_NCLBUTTONUP | WM_NCRBUTTONDOWN
        | WM_NCRBUTTONDBLCLK | WM_NCRBUTTONUP => {
            forward_titlebar_nc_message(message, wparam, lparam)
        }
        _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
    }
}

fn try_hit_test_titlebar(hwnd: HWND, lparam: LPARAM) -> Option<i32> {
    let screen_point = POINT {
        x: lparam_x(lparam),
        y: lparam_y(lparam),
    };
    let client_rect = client_rect_in_screen(hwnd)?;

    if !contains_point(client_rect, screen_point) {
        return None;
    }

    if !unsafe { IsZoomed(hwnd).as_bool() } {
        let resize = resize_handle_height_px(hwnd);
        if screen_point.y < client_rect.top + resize {
            if screen_point.x < client_rect.left + resize {
                return Some(HTTOPLEFT as i32);
            }
            if screen_point.x >= client_rect.right - resize {
                return Some(HTTOPRIGHT as i32);
            }
            return Some(HTTOP as i32);
        }
    }

    let titlebar_rect = offset_pixel_rect(
        manual_titlebar_drag_bounds_px(hwnd)?,
        client_rect.left,
        client_rect.top,
    );
    if contains_point(titlebar_rect, screen_point) {
        Some(HTCAPTION as i32)
    } else {
        Some(HTCLIENT as i32)
    }
}

fn client_rect_in_screen(hwnd: HWND) -> Option<RECT> {
    unsafe {
        let mut rect = RECT::default();
        if !GetClientRect(hwnd, &mut rect).as_bool() {
            return None;
        }
        let mut top_left = POINT { x: 0, y: 0 };
        let _ = ClientToScreen(hwnd, &mut top_left);
        rect.left += top_left.x;
        rect.right += top_left.x;
        rect.top += top_left.y;
        rect.bottom += top_left.y;
        Some(rect)
    }
}

fn offset_pixel_rect(rect: PixelRect, x: i32, y: i32) -> RECT {
    RECT {
        left: rect.x + x,
        top: rect.y + y,
        right: rect.x + x + rect.width,
        bottom: rect.y + y + rect.height,
    }
}

fn contains_point(rect: RECT, point: POINT) -> bool {
    point.x >= rect.left && point.x < rect.right && point.y >= rect.top && point.y < rect.bottom
}

fn resize_handle_height_px(hwnd: HWND) -> i32 {
    let dpi = current_dpi(hwnd);
    unsafe {
        GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi) + GetSystemMetricsForDpi(SM_CYSIZEFRAME, dpi)
    }
}

pub(crate) fn minimize_active_window() {
    send_titlebar_syscommand(SC_MINIMIZE);
}

pub(crate) fn toggle_active_window_maximize() {
    send_titlebar_syscommand(toggle_maximize_command(parent_hwnd()));
}

pub(crate) fn close_active_window() {
    send_titlebar_syscommand(SC_CLOSE);
}

fn toggle_maximize_command(parent: HWND) -> i32 {
    if unsafe { IsZoomed(parent).as_bool() } {
        SC_RESTORE
    } else {
        SC_MAXIMIZE
    }
}

fn send_titlebar_syscommand(command: i32) {
    let parent = parent_hwnd();
    if parent.is_null() {
        return;
    }
    unsafe {
        let _ = PostMessageW(Some(parent), WM_SYSCOMMAND as u32, command as usize, 0);
    }
}

fn forward_titlebar_nc_message(message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let parent = parent_hwnd();
    if parent.is_null() {
        return 0;
    }
    unsafe { SendMessageW(parent, message, wparam, lparam) }
}

fn parent_hwnd() -> HWND {
    ACTIVE_HOST_HWND.load(Ordering::Relaxed).cast()
}

fn lparam_x(lparam: LPARAM) -> i32 {
    (lparam as u32 as u16 as i16) as i32
}

fn lparam_y(lparam: LPARAM) -> i32 {
    ((lparam as u32 >> 16) as u16 as i16) as i32
}

fn window_ex_style(_backdrop: Option<Backdrop>) -> u32 {
    WS_EX_NOREDIRECTIONBITMAP as u32
}

#[derive(Copy, Clone)]
struct PixelRect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn subscribe_system_theme_changed(hwnd: HWND) -> Option<SystemThemeSubscription> {
    let settings = match UISettings::new() {
        Ok(settings) => settings,
        Err(err) => {
            crate::diagnostics::emit(&format!(
                "islands_reactor: UISettings creation failed: {err:?}\n"
            ));
            return None;
        }
    };
    let settings = match settings.cast::<IUISettings3>() {
        Ok(settings) => settings,
        Err(err) => {
            crate::diagnostics::emit(&format!(
                "islands_reactor: UISettings theme interface unavailable: {err:?}\n"
            ));
            return None;
        }
    };
    let raw_hwnd = hwnd as isize;
    let revoker = match settings.ColorValuesChanged(move |_, _| {
        let hwnd: HWND = raw_hwnd as *mut c_void as _;
        unsafe {
            let _ = PostMessageW(Some(hwnd), WM_ISLANDS_REACTOR_SYSTEM_THEME_CHANGED, 0, 0);
        }
    }) {
        Ok(revoker) => revoker,
        Err(err) => {
            crate::diagnostics::emit(&format!(
                "islands_reactor: UISettings ColorValuesChanged hookup failed: {err:?}\n"
            ));
            return None;
        }
    };
    Some(SystemThemeSubscription { _revoker: revoker })
}

fn install_theme_changed_callback(render_host: RenderHost<WinUIBackend, WinUIDispatcher>) {
    THEME_CHANGED_CALLBACK.with(|slot| {
        *slot.borrow_mut() = Some(Rc::new(move || {
            render_host.with_reconciler_mut(|r| r.notify_theme_changed());
            render_host.request_render();
        }));
    });
}

fn notify_host_theme_changed() {
    THEME_CHANGED_CALLBACK.with(|slot| {
        if let Some(callback) = slot.borrow().as_ref() {
            callback();
        }
    });
}

fn requested_theme() -> RequestedTheme {
    REQUESTED_THEME.with(Cell::get)
}

fn apply_current_requested_theme() {
    let theme = requested_theme();
    ROOT_FRAMEWORK_ELEMENT.with(|cell| {
        if let Some(fe) = cell.borrow().as_ref() {
            apply_requested_theme(fe, theme);
        } else {
            update_window_theme_from_requested();
        }
    });
}

fn apply_requested_theme(fe: &FrameworkElement, theme: RequestedTheme) {
    let element_theme = resolve_requested_theme(theme);
    let _ = fe
        .cast::<IFrameworkElement2>()
        .and_then(|fe| fe.SetRequestedTheme(element_theme));
    update_color_scheme(element_theme);
}

fn resolve_requested_theme(theme: RequestedTheme) -> ElementTheme {
    match theme {
        RequestedTheme::Light => ElementTheme::Light,
        RequestedTheme::Dark => ElementTheme::Dark,
        RequestedTheme::Default => system_element_theme(),
    }
}

fn system_element_theme() -> ElementTheme {
    if system_uses_light_theme() {
        ElementTheme::Light
    } else {
        ElementTheme::Dark
    }
}

fn system_uses_light_theme() -> bool {
    UISettings::new()
        .and_then(|settings| settings.cast::<IUISettings3>())
        .and_then(|settings| settings.GetColorValue(UIColorType::Foreground))
        .map(|foreground| !is_color_light(foreground))
        .unwrap_or(true)
}

fn is_color_light(color: crate::bindings::Color) -> bool {
    5 * color.g as u16 + 2 * color.r as u16 + color.b as u16 > 8 * 128
}

fn update_window_theme_from_requested() {
    let raw_hwnd = ACTIVE_HOST_HWND.load(Ordering::Relaxed);
    if raw_hwnd.is_null() {
        return;
    }
    update_window_theme(raw_hwnd.cast(), resolve_requested_theme(requested_theme()));
}

fn update_window_theme(hwnd: HWND, theme: ElementTheme) {
    set_window_dark_mode(hwnd, matches!(theme, ElementTheme::Dark));
}

fn apply_backdrop(hwnd: HWND, backdrop: Option<Backdrop>) {
    update_window_theme(hwnd, resolve_requested_theme(requested_theme()));

    if backdrop.is_some() && !supports_system_backdrop() {
        crate::diagnostics::emit(
            "islands_reactor: system backdrop requires Windows 11 22H2 or newer\n",
        );
        return;
    }

    set_xaml_background_transparency(backdrop.is_some());
    let value = backdrop.map_or(DWMSBT_AUTO, Backdrop::system_backdrop_type);
    if let Err(err) = unsafe {
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_SYSTEMBACKDROP_TYPE as u32,
            (&raw const value).cast(),
            std::mem::size_of_val(&value) as u32,
        )
        .ok()
    } {
        crate::diagnostics::emit(&format!(
            "islands_reactor: DWM system backdrop setup failed: {err:?}\n"
        ));
    }
}

fn set_window_dark_mode(hwnd: HWND, enabled: bool) {
    let value = BOOL::from(enabled);
    let _ = unsafe {
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE as u32,
            (&raw const value).cast(),
            std::mem::size_of_val(&value) as u32,
        )
        .ok()
    };
}

fn create_backdrop_content_host(
    backdrop: Option<Backdrop>,
) -> windows_core::Result<Option<crate::bindings::Border>> {
    if backdrop.is_none() || !supports_system_backdrop() {
        return Ok(None);
    }

    let content_host = crate::bindings::Border::new()?;
    Ok(Some(content_host))
}

fn resize_xaml_island(parent: HWND) {
    let raw_xaml_hwnd = XAML_HWND.load(Ordering::Relaxed);
    if raw_xaml_hwnd.is_null() {
        return;
    }
    unsafe {
        let mut rect = RECT::default();
        if GetClientRect(parent, &mut rect).as_bool() {
            let _ = SetWindowPos(
                raw_xaml_hwnd.cast(),
                None,
                0,
                0,
                rect.right - rect.left,
                rect.bottom - rect.top,
                (SWP_NOZORDER | SWP_NOACTIVATE | SWP_SHOWWINDOW) as u32,
            );
        }
    }
}

fn initialize_core_window_handle() {
    let Ok(core_window) = crate::winui::xaml_interop::interop::CoreWindow::GetForCurrentThread()
    else {
        return;
    };
    let Ok(core_window_interop) =
        core_window.cast::<crate::winui::xaml_interop::interop::ICoreWindowInterop>()
    else {
        return;
    };
    let Ok(core_hwnd) = (unsafe { core_window_interop.GetWindowHandle() }) else {
        return;
    };
    CORE_HWND.store(core_hwnd.cast(), Ordering::Relaxed);
    unsafe {
        let _ = ShowWindow(core_hwnd, SW_HIDE);
    }
}

fn set_xaml_background_transparency(enabled: bool) {
    let result = crate::winui::xaml_interop::interop::Window::Current()
        .and_then(|window| {
            window.cast::<crate::winui::xaml_interop::interop::IXamlSourceTransparency>()
        })
        .and_then(|transparency| unsafe { transparency.SetIsBackgroundTransparent(enabled) });
    if let Err(err) = result {
        crate::diagnostics::emit(&format!(
            "islands_reactor: XAML background transparency setup failed: {err:?}\n"
        ));
    }
}

fn supports_system_backdrop() -> bool {
    windows_build_number() >= 22621
}

fn windows_build_number() -> u32 {
    static BUILD: OnceLock<u32> = OnceLock::new();
    *BUILD.get_or_init(query_windows_build_number)
}

fn query_windows_build_number() -> u32 {
    #[repr(C)]
    struct RtlOsVersionInfoW {
        dw_os_version_info_size: u32,
        dw_major_version: u32,
        dw_minor_version: u32,
        dw_build_number: u32,
        dw_platform_id: u32,
        sz_csd_version: [u16; 128],
    }

    type RtlGetVersion = unsafe extern "system" fn(*mut RtlOsVersionInfoW) -> i32;

    unsafe {
        let ntdll = wide_null("ntdll.dll");
        let module = GetModuleHandleW(PCWSTR(ntdll.as_ptr()));
        if module.is_null() {
            return 0;
        }
        let Some(proc) = GetProcAddress(module, PCSTR(b"RtlGetVersion\0".as_ptr())) else {
            return 0;
        };
        let rtl_get_version: RtlGetVersion = std::mem::transmute(proc);
        let mut info = RtlOsVersionInfoW {
            dw_os_version_info_size: std::mem::size_of::<RtlOsVersionInfoW>() as u32,
            dw_major_version: 0,
            dw_minor_version: 0,
            dw_build_number: 0,
            dw_platform_id: 0,
            sz_csd_version: [0; 128],
        };
        if rtl_get_version(&mut info) >= 0 {
            info.dw_build_number
        } else {
            0
        }
    }
}

fn enable_resize_layout_synchronization(hwnd: HWND, enabled: bool) {
    type EnableResizeLayoutSynchronization = unsafe extern "system" fn(HWND, BOOL);
    unsafe {
        let user32 = wide_null("user32.dll");
        let user32 = GetModuleHandleW(PCWSTR(user32.as_ptr()));
        if user32.is_null() {
            return;
        }
        let Some(proc) = GetProcAddress(user32, PCSTR(2615usize as *const u8)) else {
            return;
        };
        let enable_resize_layout_synchronization: EnableResizeLayoutSynchronization =
            std::mem::transmute(proc);
        enable_resize_layout_synchronization(hwnd, BOOL::from(enabled));
    }
}

fn send_size_to_core_window(wparam: WPARAM, lparam: LPARAM) {
    let raw_core_hwnd = CORE_HWND.load(Ordering::Relaxed);
    if raw_core_hwnd.is_null() {
        return;
    }
    unsafe {
        let _ = SendMessageW(raw_core_hwnd.cast(), WM_SIZE as u32, wparam, lparam);
    }
}

fn set_synchronization_window(hwnd: HWND) {
    let Ok(application) = crate::winui::xaml_interop::interop::Application::Current() else {
        return;
    };
    let Ok(framework_app_private) =
        application.cast::<crate::winui::xaml_interop::interop::IFrameworkApplicationPrivate>()
    else {
        return;
    };
    unsafe {
        let _ = framework_app_private.SetSynchronizationWindow(hwnd);
    }
}

fn client_size_dips(hwnd: HWND) -> crate::Size {
    unsafe {
        let mut rect = RECT::default();
        let dpi = current_dpi(hwnd) as f64;
        if GetClientRect(hwnd, &mut rect).as_bool() {
            crate::Size {
                width: (rect.right - rect.left) as f64 * 96.0 / dpi,
                height: (rect.bottom - rect.top) as f64 * 96.0 / dpi,
            }
        } else {
            crate::Size::default()
        }
    }
}

fn current_dpi(hwnd: HWND) -> u32 {
    let dpi = unsafe { GetDpiForWindow(hwnd) };
    if dpi == 0 { 96 } else { dpi }
}

fn default_display_size() -> crate::Size {
    unsafe {
        let monitor = MonitorFromWindow(std::ptr::null_mut(), MONITOR_DEFAULTTONEAREST as u32);
        let mut monitor_info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if GetMonitorInfoW(monitor, &mut monitor_info).as_bool() {
            let work = monitor_info.rcWork;
            crate::Size {
                width: (work.right - work.left) as f64 / 2.0,
                height: (work.bottom - work.top) as f64 / 2.0,
            }
        } else {
            crate::Size {
                width: 900.0,
                height: 560.0,
            }
        }
    }
}

fn subscribe_actual_theme_changed(fe: &FrameworkElement) {
    update_color_scheme_from(fe);

    let _ = fe
        .cast::<IFrameworkElement6>()
        .and_then(|fe| {
            fe.ActualThemeChanged(move |sender, _| {
                if let Some(fe) = sender.as_ref() {
                    update_color_scheme_from(fe);
                }
                notify_host_theme_changed();
            })
        })
        .ok()
        .map(|revoker| revoker.into_token());
}

fn update_color_scheme_from(fe: &FrameworkElement) {
    if let Ok(theme) = fe
        .cast::<IFrameworkElement6>()
        .and_then(|fe| fe.ActualTheme())
    {
        update_color_scheme(theme);
    }
}

fn update_color_scheme(theme: ElementTheme) {
    let scheme = match theme {
        ElementTheme::Dark => crate::theme::ColorScheme::Dark,
        _ => crate::theme::ColorScheme::Light,
    };
    crate::theme::set_current_color_scheme(scheme);

    let raw_hwnd = ACTIVE_HOST_HWND.load(Ordering::Relaxed);
    if !raw_hwnd.is_null() {
        update_window_theme(raw_hwnd.cast(), theme);
    }
}

fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
