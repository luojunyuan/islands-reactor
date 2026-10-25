#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
mod bindings;
use bindings::*;
use std::cell::RefCell;
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use windows_core::{Error, PCWSTR, Result};
use windows_window::{Window, WindowBuilder};

const CALLBACK_MESSAGE: u32 = WM_USER as u32 + 1;
const ICON_ID: u32 = 1;

/// A point in screen coordinates.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScreenPoint {
    pub x: i32,
    pub y: i32,
}

/// Events delivered by a notification-area icon.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotifyIconEvent {
    /// The icon was selected with the mouse or keyboard.
    Activate { position: ScreenPoint },
    /// The user requested the icon's context menu.
    ContextMenu { position: ScreenPoint },
    /// The Windows Shell restarted and the registration was restored.
    Recover,
}

type EventHandler = Box<dyn Fn(NotifyIconEvent)>;

struct OwnedIcon {
    handle: *mut core::ffi::c_void,
    owned: bool,
}

impl OwnedIcon {
    fn load(path: &Path) -> Result<Self> {
        let path = wide_path(path)?;
        let icon = unsafe {
            LoadImageW(
                core::ptr::null_mut(),
                PCWSTR(path.as_ptr()),
                IMAGE_ICON as u32,
                GetSystemMetrics(SM_CXSMICON),
                GetSystemMetrics(SM_CYSMICON),
                LR_LOADFROMFILE as u32,
            )
        };
        if icon.is_null() {
            Err(Error::from_thread())
        } else {
            Ok(Self {
                handle: icon,
                owned: true,
            })
        }
    }

    #[cfg(test)]
    fn borrowed(handle: usize) -> Self {
        Self {
            handle: handle as _,
            owned: false,
        }
    }
}

impl Drop for OwnedIcon {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                _ = DestroyIcon(self.handle.cast());
            }
        }
    }
}

struct Registration {
    icon: OwnedIcon,
    tooltip: Option<[u16; 128]>,
}

impl Registration {
    fn add(&self, hwnd: *mut core::ffi::c_void) -> Result<()> {
        self.add_with(hwnd, &mut shell_notify)
    }

    fn add_with<F>(&self, hwnd: *mut core::ffi::c_void, notify: &mut F) -> Result<()>
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let data = self.data(hwnd);
        if !notify(NIM_ADD as u32, &data) {
            _ = notify(NIM_DELETE as u32, &data);
            return Err(shell_error("failed to add notification-area icon"));
        }

        let version = NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd.cast(),
            uID: ICON_ID,
            Anonymous: NOTIFYICONDATAW_0 {
                uVersion: NOTIFYICON_VERSION_4 as u32,
            },
            ..Default::default()
        };
        if !notify(NIM_SETVERSION as u32, &version) {
            _ = notify(NIM_DELETE as u32, &data);
            return Err(shell_error(
                "failed to enable notification-area icon version 4",
            ));
        }
        Ok(())
    }

    fn update_with<F>(&self, hwnd: *mut core::ffi::c_void, notify: &mut F) -> Result<()>
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let data = self.data(hwnd);
        if notify(NIM_MODIFY as u32, &data) {
            Ok(())
        } else {
            Err(shell_error("failed to update notification-area icon"))
        }
    }

    fn recover(&self, hwnd: *mut core::ffi::c_void) -> Result<()> {
        self.recover_with(hwnd, &mut shell_notify)
    }

    fn recover_with<F>(&self, hwnd: *mut core::ffi::c_void, notify: &mut F) -> Result<()>
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        self.delete_with(hwnd, notify);
        self.add_with(hwnd, notify)
    }

    fn delete(&self, hwnd: *mut core::ffi::c_void) {
        self.delete_with(hwnd, &mut shell_notify);
    }

    fn delete_with<F>(&self, hwnd: *mut core::ffi::c_void, notify: &mut F)
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let data = NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd.cast(),
            uID: ICON_ID,
            ..Default::default()
        };
        _ = notify(NIM_DELETE as u32, &data);
    }

    fn replace_icon_with<F>(
        &mut self,
        icon: OwnedIcon,
        hwnd: *mut core::ffi::c_void,
        notify: &mut F,
    ) -> Result<()>
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let previous = std::mem::replace(&mut self.icon, icon);
        if let Err(error) = self.update_with(hwnd, notify) {
            self.icon = previous;
            return Err(error);
        }
        Ok(())
    }

    fn set_tooltip_with<F>(
        &mut self,
        tooltip: Option<[u16; 128]>,
        hwnd: *mut core::ffi::c_void,
        notify: &mut F,
    ) -> Result<()>
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let previous = std::mem::replace(&mut self.tooltip, tooltip);
        if let Err(error) = self.update_with(hwnd, notify) {
            self.tooltip = previous;
            return Err(error);
        }
        Ok(())
    }

    fn data(&self, hwnd: *mut core::ffi::c_void) -> NOTIFYICONDATAW {
        let flags = NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_SHOWTIP;
        NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd.cast(),
            uID: ICON_ID,
            uFlags: flags as u32,
            uCallbackMessage: CALLBACK_MESSAGE,
            hIcon: self.icon.handle.cast(),
            szTip: self.tooltip.unwrap_or([0; 128]),
            ..Default::default()
        }
    }
}

/// A notification-area icon and its hidden callback window.
pub struct NotifyIcon {
    callback_window: Window,
    registration: Rc<RefCell<Registration>>,
}

impl NotifyIcon {
    /// Begins configuring a notification-area icon loaded from an `.ico` file.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(path: impl Into<PathBuf>) -> NotifyIconBuilder {
        NotifyIconBuilder {
            handler: None,
            icon: path.into(),
            tooltip: None,
        }
    }

    /// Replaces the icon using an `.ico` file.
    pub fn set_icon(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let icon = OwnedIcon::load(path.as_ref())?;
        self.registration.borrow_mut().replace_icon_with(
            icon,
            self.callback_window.hwnd(),
            &mut shell_notify,
        )
    }

    /// Sets or clears the standard tooltip.
    pub fn set_tooltip(&mut self, tooltip: Option<&str>) -> Result<()> {
        let tooltip = tooltip.map(tooltip_text).transpose()?;
        self.registration.borrow_mut().set_tooltip_with(
            tooltip,
            self.callback_window.hwnd(),
            &mut shell_notify,
        )
    }

    /// Re-registers the icon after Explorer restarts.
    pub fn recover(&self) -> Result<()> {
        self.registration
            .borrow()
            .recover(self.callback_window.hwnd())
    }
}

impl Drop for NotifyIcon {
    fn drop(&mut self) {
        self.registration
            .borrow()
            .delete(self.callback_window.hwnd());
    }
}

/// Configures a [`NotifyIcon`].
pub struct NotifyIconBuilder {
    handler: Option<EventHandler>,
    icon: PathBuf,
    tooltip: Option<String>,
}

impl NotifyIconBuilder {
    /// Sets the standard tooltip shown for the icon.
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Sets the event sink. Keep this callback short and enqueue app work for later.
    pub fn on_event<F>(mut self, handler: F) -> Self
    where
        F: Fn(NotifyIconEvent) + 'static,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    /// Creates the hidden callback window and adds the icon to the notification area.
    pub fn build(self) -> Result<NotifyIcon> {
        let taskbar_created = register_taskbar_created()?;
        let registration = Rc::new(RefCell::new(Registration {
            icon: OwnedIcon::load(&self.icon)?,
            tooltip: self.tooltip.as_deref().map(tooltip_text).transpose()?,
        }));
        let callback_window =
            callback_window(self.handler, taskbar_created, Rc::downgrade(&registration))
                .create()?;
        allow_message(callback_window.hwnd(), CALLBACK_MESSAGE)?;
        allow_message(callback_window.hwnd(), taskbar_created)?;
        registration.borrow().add(callback_window.hwnd())?;
        Ok(NotifyIcon {
            callback_window,
            registration,
        })
    }
}

fn callback_window(
    handler: Option<EventHandler>,
    taskbar_created: u32,
    registration: Weak<RefCell<Registration>>,
) -> WindowBuilder {
    Window::new("islands-notifyicon")
        .style(0)
        .visible(false)
        .quit_on_close(false)
        .on_message(move |hwnd, message, wparam, lparam| {
            let event = if message == taskbar_created {
                if let Some(registration) = registration.upgrade() {
                    let recovery = registration.borrow().recover(hwnd);
                    if let Err(error) = recovery {
                        crate::diagnostics::emit(&format!(
                            "islands_reactor: failed to restore notification-area icon: {error}\n"
                        ));
                    }
                }
                Some(NotifyIconEvent::Recover)
            } else if message == CALLBACK_MESSAGE {
                decode_event(wparam, lparam)
            } else {
                return None;
            };
            if let Some(event) = event
                && let Some(handler) = &handler
            {
                handler(event);
            }
            Some(0)
        })
}

fn allow_message(hwnd: *mut core::ffi::c_void, message: u32) -> Result<()> {
    if unsafe {
        ChangeWindowMessageFilterEx(
            hwnd.cast(),
            message,
            MSGFLT_ALLOW as u32,
            core::ptr::null_mut(),
        )
    }
    .as_bool()
    {
        Ok(())
    } else {
        let error = Error::from_thread();
        if error.code().is_ok() {
            Err(shell_error(
                "failed to allow a notification-area window message",
            ))
        } else {
            Err(error)
        }
    }
}

fn decode_event(wparam: usize, lparam: isize) -> Option<NotifyIconEvent> {
    let event = lparam as u16 as u32;
    let position = ScreenPoint {
        x: wparam as u16 as i16 as i32,
        y: (wparam >> 16) as u16 as i16 as i32,
    };
    match event as i32 {
        NIN_SELECT | NIN_KEYSELECT => Some(NotifyIconEvent::Activate { position }),
        WM_CONTEXTMENU => Some(NotifyIconEvent::ContextMenu { position }),
        _ => None,
    }
}

fn register_taskbar_created() -> Result<u32> {
    let name = wide("TaskbarCreated");
    let message = unsafe { RegisterWindowMessageW(PCWSTR(name.as_ptr())) };
    if message == 0 {
        Err(Error::from_thread())
    } else {
        Ok(message)
    }
}

fn shell_notify(message: u32, data: &NOTIFYICONDATAW) -> bool {
    unsafe { Shell_NotifyIconW(message, core::ptr::from_ref(data).cast_mut()) }.as_bool()
}

fn wide_path(value: &Path) -> Result<Vec<u16>> {
    let mut value = value.as_os_str().encode_wide().collect::<Vec<_>>();
    if value.contains(&0) {
        return Err(Error::new(
            E_INVALIDARG,
            "icon path contains a null character",
        ));
    }
    value.push(0);
    Ok(value)
}

fn tooltip_text(value: &str) -> Result<[u16; 128]> {
    let value = value.encode_utf16().collect::<Vec<_>>();
    if value.contains(&0) {
        return Err(Error::new(
            E_INVALIDARG,
            "notification-area tooltip contains a null character",
        ));
    }
    if value.len() >= 128 {
        return Err(Error::new(
            E_INVALIDARG,
            "notification-area tooltip exceeds 127 UTF-16 code units",
        ));
    }
    let mut result = [0; 128];
    result[..value.len()].copy_from_slice(&value);
    Ok(result)
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(Some(0)).collect()
}

fn shell_error(message: &'static str) -> Error {
    Error::new(E_FAIL, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_accepts_the_public_configuration() {
        let _builder =
            NotifyIcon::new("icon.ico")
                .tooltip("Example")
                .on_event(|event| match event {
                    NotifyIconEvent::Activate { position }
                    | NotifyIconEvent::ContextMenu { position } => {
                        let _ = position;
                    }
                    NotifyIconEvent::Recover => {}
                });
    }

    #[test]
    fn decodes_version_four_events_and_signed_coordinates() {
        let position = ((-20_i16 as u16 as usize) << 16) | (-10_i16 as u16 as usize);
        assert_eq!(
            decode_event(position, NIN_SELECT as isize),
            Some(NotifyIconEvent::Activate {
                position: ScreenPoint { x: -10, y: -20 }
            })
        );
        assert_eq!(
            decode_event(position, WM_CONTEXTMENU as isize),
            Some(NotifyIconEvent::ContextMenu {
                position: ScreenPoint { x: -10, y: -20 }
            })
        );
        assert_eq!(decode_event(position, 0), None);
    }

    #[test]
    fn tooltip_validation_uses_utf16_capacity_and_rejects_nuls() {
        let value = tooltip_text("hello").unwrap();
        assert_eq!(&value[..6], &[104, 101, 108, 108, 111, 0]);
        assert!(tooltip_text("before\0after").is_err());
        assert!(tooltip_text(&"x".repeat(128)).is_err());
        assert!(tooltip_text(&"🦀".repeat(64)).is_err());
    }

    #[test]
    fn preserves_windows_paths_that_are_not_utf8() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        let path = PathBuf::from(OsString::from_wide(&[b'a' as u16, 0xd800]));
        assert_eq!(wide_path(&path).unwrap(), [b'a' as u16, 0xd800, 0]);
    }

    fn registration(icon: usize) -> Registration {
        Registration {
            icon: OwnedIcon::borrowed(icon),
            tooltip: None,
        }
    }

    #[test]
    fn failed_icon_replacement_preserves_the_previous_handle() {
        let mut registration = registration(1);
        let mut notify = |_, _: &NOTIFYICONDATAW| false;
        assert!(
            registration
                .replace_icon_with(OwnedIcon::borrowed(2), core::ptr::null_mut(), &mut notify)
                .is_err()
        );
        assert_eq!(registration.icon.handle as usize, 1);
    }

    #[test]
    fn updates_and_clears_the_tooltip_payload() {
        let mut registration = registration(1);
        let mut payloads = Vec::new();
        let mut notify = |message, data: &NOTIFYICONDATAW| {
            payloads.push((message, data.uFlags, data.szTip));
            true
        };

        registration
            .set_tooltip_with(
                Some(tooltip_text("updated").unwrap()),
                core::ptr::null_mut(),
                &mut notify,
            )
            .unwrap();
        registration
            .set_tooltip_with(None, core::ptr::null_mut(), &mut notify)
            .unwrap();

        assert_eq!(payloads[0].0, NIM_MODIFY as u32);
        assert_ne!(payloads[0].1 & NIF_TIP as u32, 0);
        assert_eq!(&payloads[0].2[..8], &[117, 112, 100, 97, 116, 101, 100, 0]);
        assert_eq!(payloads[1].0, NIM_MODIFY as u32);
        assert!(payloads[1].2.iter().all(|value| *value == 0));
    }

    #[test]
    fn failed_version_selection_removes_the_added_icon() {
        let registration = registration(1);
        let mut results = [true, false, true].into_iter();
        let mut calls = Vec::new();
        let mut notify = |message, _: &NOTIFYICONDATAW| {
            calls.push(message);
            results.next().unwrap()
        };
        assert!(
            registration
                .add_with(core::ptr::null_mut(), &mut notify)
                .is_err()
        );
        assert_eq!(
            calls,
            [NIM_ADD as u32, NIM_SETVERSION as u32, NIM_DELETE as u32]
        );
    }

    #[test]
    fn failed_modify_does_not_attempt_registration() {
        let registration = registration(1);
        let mut calls = Vec::new();
        let mut notify = |message, _: &NOTIFYICONDATAW| {
            calls.push(message);
            false
        };

        assert!(
            registration
                .update_with(core::ptr::null_mut(), &mut notify)
                .is_err()
        );
        assert_eq!(calls, [NIM_MODIFY as u32]);
    }

    #[test]
    fn recovery_deletes_then_adds_and_sets_version() {
        let registration = registration(1);
        let mut calls = Vec::new();
        let mut notify = |message, _: &NOTIFYICONDATAW| {
            calls.push(message);
            true
        };
        registration
            .recover_with(core::ptr::null_mut(), &mut notify)
            .unwrap();
        assert_eq!(
            calls,
            [NIM_DELETE as u32, NIM_ADD as u32, NIM_SETVERSION as u32]
        );
    }
}
