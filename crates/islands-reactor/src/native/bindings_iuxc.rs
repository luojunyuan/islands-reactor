#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppWindowTitleBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppWindowTitleBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AppWindowTitleBar {
    pub fn new() -> windows_core::Result<AppWindowTitleBar> {
        Self::IAppWindowTitleBarFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<AppWindowTitleBar>
    where
        T: windows_core::Compose,
    {
        Self::IAppWindowTitleBarFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IAppWindowTitleBarFactory<
        R,
        F: FnOnce(&IAppWindowTitleBarFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            AppWindowTitleBar,
            IAppWindowTitleBarFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowTitleBar>();
}
unsafe impl windows_core::Interface for AppWindowTitleBar {
    type Vtable = <IAppWindowTitleBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppWindowTitleBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppWindowTitleBar {
    type Target = IAppWindowTitleBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Islands.UI.Xaml.Controls.AppWindowTitleBar";
}
unsafe impl Send for AppWindowTitleBar {}
unsafe impl Sync for AppWindowTitleBar {}
windows_core::imp::define_interface!(
    IAppWindowTitleBar,
    IAppWindowTitleBar_Vtbl,
    0xfa626e9e_8a39_5313_a78e_3c119a8d959c
);
impl windows_core::RuntimeType for IAppWindowTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindowTitleBar {
    pub fn put_ExtendsContentIntoTitleBar(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_ExtendsContentIntoTitleBar)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_LeftInset(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_LeftInset)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_RightInset(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_RightInset)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppWindowTitleBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ExtendsContentIntoTitleBar: usize,
    pub put_ExtendsContentIntoTitleBar:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_LeftInset: usize,
    pub put_LeftInset:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_RightInset: usize,
    pub put_RightInset:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowTitleBarFactory,
    IAppWindowTitleBarFactory_Vtbl,
    0x32c51a9c_f3d1_552c_9d5a_e0d87aff666a
);
impl windows_core::RuntimeType for IAppWindowTitleBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowTitleBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IScrollView,
    IScrollView_Vtbl,
    0xadba30ae_84d1_5a0b_9809_551a5561a464
);
impl windows_core::RuntimeType for IScrollView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IScrollView {
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<crate::bindings::UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_HorizontalScrollBarVisibility(
        &self,
        value: ScrollingScrollBarVisibility,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_HorizontalScrollBarVisibility)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_VerticalScrollBarVisibility(
        &self,
        value: ScrollingScrollBarVisibility,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_VerticalScrollBarVisibility)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IScrollView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Content: usize,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_CurrentAnchor: usize,
    get_ScrollPresenter: usize,
    get_ExpressionAnimationSources: usize,
    get_HorizontalOffset: usize,
    get_VerticalOffset: usize,
    get_ZoomFactor: usize,
    get_ExtentWidth: usize,
    get_ExtentHeight: usize,
    get_ViewportWidth: usize,
    get_ViewportHeight: usize,
    get_ScrollableWidth: usize,
    get_ScrollableHeight: usize,
    get_State: usize,
    get_HorizontalScrollBarVisibility: usize,
    pub put_HorizontalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollingScrollBarVisibility,
    ) -> windows_core::HRESULT,
    get_VerticalScrollBarVisibility: usize,
    pub put_VerticalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollingScrollBarVisibility,
    ) -> windows_core::HRESULT,
    get_ContentOrientation: usize,
    put_ContentOrientation: usize,
    get_HorizontalScrollChainMode: usize,
    put_HorizontalScrollChainMode: usize,
    get_VerticalScrollChainMode: usize,
    put_VerticalScrollChainMode: usize,
    get_HorizontalScrollRailMode: usize,
    put_HorizontalScrollRailMode: usize,
    get_VerticalScrollRailMode: usize,
    put_VerticalScrollRailMode: usize,
    get_HorizontalScrollMode: usize,
    put_HorizontalScrollMode: usize,
    get_VerticalScrollMode: usize,
    put_VerticalScrollMode: usize,
    get_ComputedHorizontalScrollBarVisibility: usize,
    get_ComputedVerticalScrollBarVisibility: usize,
    get_ComputedHorizontalScrollMode: usize,
    get_ComputedVerticalScrollMode: usize,
    get_ZoomChainMode: usize,
    put_ZoomChainMode: usize,
    get_ZoomMode: usize,
    put_ZoomMode: usize,
    get_IgnoredInputKinds: usize,
    put_IgnoredInputKinds: usize,
    get_MinZoomFactor: usize,
    put_MinZoomFactor: usize,
    get_MaxZoomFactor: usize,
    put_MaxZoomFactor: usize,
    get_HorizontalAnchorRatio: usize,
    put_HorizontalAnchorRatio: usize,
    get_VerticalAnchorRatio: usize,
    put_VerticalAnchorRatio: usize,
    RegisterAnchorCandidate: usize,
    UnregisterAnchorCandidate: usize,
    ScrollTo: usize,
    ScrollToWithOptions: usize,
    ScrollBy: usize,
    ScrollByWithOptions: usize,
    AddScrollVelocity: usize,
    ZoomTo: usize,
    ZoomToWithOptions: usize,
    ZoomBy: usize,
    ZoomByWithOptions: usize,
    AddZoomVelocity: usize,
    add_ExtentChanged: usize,
    remove_ExtentChanged: usize,
    add_StateChanged: usize,
    remove_StateChanged: usize,
    add_ViewChanged: usize,
    remove_ViewChanged: usize,
    add_ScrollAnimationStarting: usize,
    remove_ScrollAnimationStarting: usize,
    add_ZoomAnimationStarting: usize,
    remove_ZoomAnimationStarting: usize,
    add_ScrollCompleted: usize,
    remove_ScrollCompleted: usize,
    add_ZoomCompleted: usize,
    remove_ZoomCompleted: usize,
    add_BringingIntoView: usize,
    remove_BringingIntoView: usize,
    add_AnchorRequested: usize,
    remove_AnchorRequested: usize,
    add_ScrollStarting: usize,
    remove_ScrollStarting: usize,
    add_ZoomStarting: usize,
    remove_ZoomStarting: usize,
}
windows_core::imp::define_interface!(
    IScrollViewFactory,
    IScrollViewFactory_Vtbl,
    0x5b90ee42_7aa5_5e6f_a8b5_06b544d3981a
);
impl windows_core::RuntimeType for IScrollViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScrollViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITitleBar,
    ITitleBar_Vtbl,
    0x68575d85_6884_52a1_a84c_c95972e38768
);
impl windows_core::RuntimeType for ITitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITitleBar {
    pub fn put_Title(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Subtitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Subtitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<crate::bindings::UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_RightHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<crate::bindings::UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_RightHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsBackButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsBackButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsBackButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsBackButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsPaneToggleButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPaneToggleButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_BackRequested<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TitleBar>, windows_core::Ref<windows_core::IInspectable>)
            + Send
            + 'static,
    {
        let handler =
            <windows::Foundation::TypedEventHandler<TitleBar, windows_core::IInspectable>>::new(
                move |a0, a1| {
                    handler(a0, a1);
                    Ok(())
                },
            );
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_BackRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_BackRequested,
            ))
        }
    }
    pub fn add_PaneToggleRequested<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TitleBar>, windows_core::Ref<windows_core::IInspectable>)
            + Send
            + 'static,
    {
        let handler =
            <windows::Foundation::TypedEventHandler<TitleBar, windows_core::IInspectable>>::new(
                move |a0, a1| {
                    handler(a0, a1);
                    Ok(())
                },
            );
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_PaneToggleRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_PaneToggleRequested,
            ))
        }
    }
}
#[repr(C)]
pub struct ITitleBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Subtitle: usize,
    pub put_Subtitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IconSource: usize,
    put_IconSource: usize,
    get_LeftHeader: usize,
    put_LeftHeader: usize,
    get_Content: usize,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_RightHeader: usize,
    pub put_RightHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsBackButtonVisible: usize,
    pub put_IsBackButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsBackButtonEnabled: usize,
    pub put_IsBackButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsPaneToggleButtonVisible: usize,
    pub put_IsPaneToggleButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_TemplateSettings: usize,
    pub add_BackRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_BackRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_PaneToggleRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_PaneToggleRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITitleBar2,
    ITitleBar2_Vtbl,
    0x32f6d4ba_8de2_58dd_9d13_3b566e2b8fa2
);
impl windows_core::RuntimeType for ITitleBar2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITitleBar2 {
    pub fn put_AutoRefreshDragRegions(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_AutoRefreshDragRegions)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn RecomputeDragRegions(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RecomputeDragRegions)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ITitleBar2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_AutoRefreshDragRegions: usize,
    pub put_AutoRefreshDragRegions:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RecomputeDragRegions:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITitleBarFactory,
    ITitleBarFactory_Vtbl,
    0xc6f4b2fa_3b1d_56dd_9df7_a798669f0741
);
impl windows_core::RuntimeType for ITitleBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITitleBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITitleBarWindowAdapter,
    ITitleBarWindowAdapter_Vtbl,
    0x191403b9_eea6_5bae_a418_1dedf5902d66
);
impl windows_core::RuntimeType for ITitleBarWindowAdapter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITitleBarWindowAdapter {
    pub fn put_WindowHandle(&self, value: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_WindowHandle)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_WindowTitleBar(&self) -> windows_core::Result<AppWindowTitleBar> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WindowTitleBar)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTitleBar<P0>(&self, titlebar: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<crate::bindings::UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitleBar)(
                windows_core::Interface::as_raw(self),
                titlebar.param().abi(),
            )
            .ok()
        }
    }
    pub fn NotifyWindowActivated(&self, active: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).NotifyWindowActivated)(
                windows_core::Interface::as_raw(self),
                active,
            )
            .ok()
        }
    }
    pub fn SetCaptionInsets(&self, left: f64, right: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCaptionInsets)(
                windows_core::Interface::as_raw(self),
                left,
                right,
            )
            .ok()
        }
    }
    pub fn HitTest(
        &self,
        screenx: i32,
        screeny: i32,
        xamlrootscreenx: i32,
        xamlrootscreeny: i32,
    ) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HitTest)(
                windows_core::Interface::as_raw(self),
                screenx,
                screeny,
                xamlrootscreenx,
                xamlrootscreeny,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ApplyTitleBarWindowRegion(
        &self,
        titlebarwindowhandle: i64,
        xamlrootscreenx: i32,
        xamlrootscreeny: i32,
    ) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplyTitleBarWindowRegion)(
                windows_core::Interface::as_raw(self),
                titlebarwindowhandle,
                xamlrootscreenx,
                xamlrootscreeny,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ITitleBarWindowAdapter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_WindowHandle: usize,
    pub put_WindowHandle:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub get_WindowTitleBar: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_NonClientPointerSource: usize,
    pub SetTitleBar: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NotifyWindowActivated:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetCaptionInsets:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub HitTest: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        i32,
        i32,
        i32,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub ApplyTitleBarWindowRegion: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i64,
        i32,
        i32,
        *mut bool,
    ) -> windows_core::HRESULT,
    add_NonClientRegionsChanged: usize,
    remove_NonClientRegionsChanged: usize,
}
windows_core::imp::define_interface!(
    ITitleBarWindowAdapterFactory,
    ITitleBarWindowAdapterFactory_Vtbl,
    0xb07ec0ab_b432_5b1b_8847_551b47fff6a5
);
impl windows_core::RuntimeType for ITitleBarWindowAdapterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITitleBarWindowAdapterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScrollView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ScrollView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ScrollView {
    pub fn new() -> windows_core::Result<ScrollView> {
        Self::IScrollViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ScrollView>
    where
        T: windows_core::Compose,
    {
        Self::IScrollViewFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IScrollViewFactory<R, F: FnOnce(&IScrollViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ScrollView, IScrollViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ScrollView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IScrollView>();
}
unsafe impl windows_core::Interface for ScrollView {
    type Vtable = <IScrollView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IScrollView as windows_core::Interface>::IID;
}
impl core::ops::Deref for ScrollView {
    type Target = IScrollView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ScrollView {
    const NAME: &'static str = "Islands.UI.Xaml.Controls.ScrollView";
}
unsafe impl Send for ScrollView {}
unsafe impl Sync for ScrollView {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScrollingScrollBarVisibility(pub i32);
impl ScrollingScrollBarVisibility {
    pub const Auto: Self = Self(0);
    pub const Visible: Self = Self(1);
    pub const Hidden: Self = Self(2);
}
impl windows_core::TypeKind for ScrollingScrollBarVisibility {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ScrollingScrollBarVisibility {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Islands.UI.Xaml.Controls.ScrollingScrollBarVisibility;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TitleBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TitleBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl TitleBar {
    pub fn new() -> windows_core::Result<TitleBar> {
        Self::ITitleBarFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<TitleBar>
    where
        T: windows_core::Compose,
    {
        Self::ITitleBarFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ITitleBarFactory<R, F: FnOnce(&ITitleBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TitleBar, ITitleBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITitleBar>();
}
unsafe impl windows_core::Interface for TitleBar {
    type Vtable = <ITitleBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITitleBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for TitleBar {
    type Target = ITitleBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TitleBar {
    const NAME: &'static str = "Islands.UI.Xaml.Controls.TitleBar";
}
unsafe impl Send for TitleBar {}
unsafe impl Sync for TitleBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TitleBarWindowAdapter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TitleBarWindowAdapter,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl TitleBarWindowAdapter {
    pub fn CreateInstance<P0>(titlebar: P0) -> windows_core::Result<TitleBarWindowAdapter>
    where
        P0: windows_core::Param<TitleBar>,
    {
        Self::ITitleBarWindowAdapterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                titlebar.param().abi(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance_compose<P0, T>(
        titlebar: P0,
        compose: T,
    ) -> windows_core::Result<TitleBarWindowAdapter>
    where
        P0: windows_core::Param<TitleBar>,
        T: windows_core::Compose,
    {
        Self::ITitleBarWindowAdapterFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                titlebar.param().abi(),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ITitleBarWindowAdapterFactory<
        R,
        F: FnOnce(&ITitleBarWindowAdapterFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            TitleBarWindowAdapter,
            ITitleBarWindowAdapterFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TitleBarWindowAdapter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITitleBarWindowAdapter>();
}
unsafe impl windows_core::Interface for TitleBarWindowAdapter {
    type Vtable = <ITitleBarWindowAdapter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITitleBarWindowAdapter as windows_core::Interface>::IID;
}
impl core::ops::Deref for TitleBarWindowAdapter {
    type Target = ITitleBarWindowAdapter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TitleBarWindowAdapter {
    const NAME: &'static str = "Islands.UI.Xaml.Controls.TitleBarWindowAdapter";
}
unsafe impl Send for TitleBarWindowAdapter {}
unsafe impl Sync for TitleBarWindowAdapter {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlMetaDataProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlMetaDataProvider,
    windows_core::IUnknown,
    windows_core::IInspectable,
    crate::bindings::IXamlMetadataProvider
);
impl XamlMetaDataProvider {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            XamlMetaDataProvider,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XamlMetaDataProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, crate::bindings::IXamlMetadataProvider>();
}
unsafe impl windows_core::Interface for XamlMetaDataProvider {
    type Vtable = <crate::bindings::IXamlMetadataProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <crate::bindings::IXamlMetadataProvider as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlMetaDataProvider {
    type Target = crate::bindings::IXamlMetadataProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlMetaDataProvider {
    const NAME: &'static str = "Islands.UI.Xaml.Controls.XamlMetaDataProvider";
}
unsafe impl Send for XamlMetaDataProvider {}
unsafe impl Sync for XamlMetaDataProvider {}
