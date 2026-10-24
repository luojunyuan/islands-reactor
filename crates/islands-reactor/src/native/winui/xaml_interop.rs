#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod interop {
    use std::ffi::c_void;

    use windows::Win32::windef::HWND;

    windows_core::imp::define_interface!(
        ICoreWindow,
        ICoreWindow_Vtbl,
        0x79b9d5f2_879e_4b89_b798_79e47598030c
    );
    impl windows_core::RuntimeType for ICoreWindow {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        ICoreWindow,
        windows_core::IUnknown,
        windows_core::IInspectable
    );

    #[repr(C)]
    pub struct ICoreWindow_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
    }

    windows_core::imp::define_interface!(
        ICoreWindowStatic,
        ICoreWindowStatic_Vtbl,
        0x4d239005_3c2a_41b1_9022_536bb9cf93b1
    );
    impl windows_core::RuntimeType for ICoreWindowStatic {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        ICoreWindowStatic,
        windows_core::IUnknown,
        windows_core::IInspectable
    );

    #[repr(C)]
    pub struct ICoreWindowStatic_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub GetForCurrentThread:
            unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> windows_core::HRESULT,
    }

    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct CoreWindow(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        CoreWindow,
        windows_core::IUnknown,
        windows_core::IInspectable,
        ICoreWindow
    );

    impl CoreWindow {
        pub fn GetForCurrentThread() -> windows_core::Result<Self> {
            Self::ICoreWindowStatic(|this| unsafe {
                let mut result__ = std::ptr::null_mut();
                (windows_core::Interface::vtable(this).GetForCurrentThread)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .and_then(|| windows_core::Type::from_abi(result__))
            })
        }

        fn ICoreWindowStatic<R, F: FnOnce(&ICoreWindowStatic) -> windows_core::Result<R>>(
            callback: F,
        ) -> windows_core::Result<R> {
            static SHARED: windows_core::imp::FactoryCache<CoreWindow, ICoreWindowStatic> =
                windows_core::imp::FactoryCache::new();
            SHARED.call(callback)
        }
    }

    impl windows_core::RuntimeType for CoreWindow {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_class::<Self, ICoreWindow>();
    }
    unsafe impl windows_core::Interface for CoreWindow {
        type Vtable = <ICoreWindow as windows_core::Interface>::Vtable;
        const IID: windows_core::GUID = <ICoreWindow as windows_core::Interface>::IID;
    }
    impl windows_core::RuntimeName for CoreWindow {
        const NAME: &'static str = "Windows.UI.Core.CoreWindow";
    }

    windows_core::imp::define_interface!(
        IApplication,
        IApplication_Vtbl,
        0x74b861a1_7487_46a9_9a6e_c78b512726c5
    );
    impl windows_core::RuntimeType for IApplication {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IApplication,
        windows_core::IUnknown,
        windows_core::IInspectable
    );

    #[repr(C)]
    pub struct IApplication_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
    }

    windows_core::imp::define_interface!(
        IApplicationStatics,
        IApplicationStatics_Vtbl,
        0x06499997_f7b4_45fe_b763_7577d1d3cb4a
    );
    impl windows_core::RuntimeType for IApplicationStatics {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IApplicationStatics,
        windows_core::IUnknown,
        windows_core::IInspectable
    );

    #[repr(C)]
    pub struct IApplicationStatics_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Current:
            unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> windows_core::HRESULT,
    }

    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Application(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        Application,
        windows_core::IUnknown,
        windows_core::IInspectable,
        IApplication
    );

    impl Application {
        pub fn Current() -> windows_core::Result<Self> {
            Self::IApplicationStatics(|this| unsafe {
                let mut result__ = std::ptr::null_mut();
                (windows_core::Interface::vtable(this).Current)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .and_then(|| windows_core::Type::from_abi(result__))
            })
        }

        fn IApplicationStatics<R, F: FnOnce(&IApplicationStatics) -> windows_core::Result<R>>(
            callback: F,
        ) -> windows_core::Result<R> {
            static SHARED: windows_core::imp::FactoryCache<Application, IApplicationStatics> =
                windows_core::imp::FactoryCache::new();
            SHARED.call(callback)
        }
    }

    impl windows_core::RuntimeType for Application {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_class::<Self, IApplication>();
    }
    unsafe impl windows_core::Interface for Application {
        type Vtable = <IApplication as windows_core::Interface>::Vtable;
        const IID: windows_core::GUID = <IApplication as windows_core::Interface>::IID;
    }
    impl windows_core::RuntimeName for Application {
        const NAME: &'static str = "Windows.UI.Xaml.Application";
    }

    windows_core::imp::define_interface!(
        ICoreWindowInterop,
        ICoreWindowInterop_Vtbl,
        0x45d64a29_a63e_4cb6_b498_5781d298cb4f
    );
    windows_core::imp::interface_hierarchy!(ICoreWindowInterop, windows_core::IUnknown);

    impl ICoreWindowInterop {
        pub unsafe fn GetWindowHandle(&self) -> windows_core::Result<HWND> {
            let mut result__ = unsafe { std::mem::zeroed() };

            unsafe {
                (windows_core::Interface::vtable(self).GetWindowHandle)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
            }
            .map(|| result__)
        }
    }

    #[repr(C)]
    pub struct ICoreWindowInterop_Vtbl {
        pub base__: windows_core::IUnknown_Vtbl,
        pub GetWindowHandle:
            unsafe extern "system" fn(*mut c_void, *mut HWND) -> windows_core::HRESULT,
    }

    windows_core::imp::define_interface!(
        IWindow,
        IWindow_Vtbl,
        0x3276167d_c9f6_462d_9de2_ae4c1fd8c2e5
    );
    impl windows_core::RuntimeType for IWindow {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IWindow,
        windows_core::IUnknown,
        windows_core::IInspectable
    );

    #[repr(C)]
    pub struct IWindow_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        Bounds: usize,
        Visible: usize,
        Content: usize,
        SetContent: usize,
        CoreWindow: usize,
        Dispatcher: usize,
        Activated: usize,
        RemoveActivated: usize,
        Closed: usize,
        RemoveClosed: usize,
        SizeChanged: usize,
        RemoveSizeChanged: usize,
        VisibilityChanged: usize,
        RemoveVisibilityChanged: usize,
        Activate: usize,
        Close: usize,
    }

    windows_core::imp::define_interface!(
        IWindowStatics,
        IWindowStatics_Vtbl,
        0x93328409_4ea1_4afa_83dc_0c4e73e88bb1
    );
    impl windows_core::RuntimeType for IWindowStatics {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IWindowStatics,
        windows_core::IUnknown,
        windows_core::IInspectable
    );

    #[repr(C)]
    pub struct IWindowStatics_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Current:
            unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> windows_core::HRESULT,
    }

    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Window(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        Window,
        windows_core::IUnknown,
        windows_core::IInspectable,
        IWindow
    );

    impl Window {
        pub fn Current() -> windows_core::Result<Self> {
            Self::IWindowStatics(|this| unsafe {
                let mut result__ = std::ptr::null_mut();
                (windows_core::Interface::vtable(this).Current)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .and_then(|| windows_core::Type::from_abi(result__))
            })
        }

        fn IWindowStatics<R, F: FnOnce(&IWindowStatics) -> windows_core::Result<R>>(
            callback: F,
        ) -> windows_core::Result<R> {
            static SHARED: windows_core::imp::FactoryCache<Window, IWindowStatics> =
                windows_core::imp::FactoryCache::new();
            SHARED.call(callback)
        }
    }

    impl windows_core::RuntimeType for Window {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_class::<Self, IWindow>();
    }
    unsafe impl windows_core::Interface for Window {
        type Vtable = <IWindow as windows_core::Interface>::Vtable;
        const IID: windows_core::GUID = <IWindow as windows_core::Interface>::IID;
    }
    impl windows_core::RuntimeName for Window {
        const NAME: &'static str = "Windows.UI.Xaml.Window";
    }

    windows_core::imp::define_interface!(
        IXamlSourceTransparency,
        IXamlSourceTransparency_Vtbl,
        0x06636c29_5a17_458d_8ea2_2422d997a922
    );
    impl windows_core::RuntimeType for IXamlSourceTransparency {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IXamlSourceTransparency,
        windows_core::IUnknown,
        windows_core::IInspectable
    );

    impl IXamlSourceTransparency {
        pub unsafe fn SetIsBackgroundTransparent(&self, value: bool) -> windows_core::Result<()> {
            unsafe {
                (windows_core::Interface::vtable(self).SetIsBackgroundTransparent)(
                    windows_core::Interface::as_raw(self),
                    value,
                )
            }
            .ok()
        }
    }

    #[repr(C)]
    pub struct IXamlSourceTransparency_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub GetIsBackgroundTransparent:
            unsafe extern "system" fn(*mut c_void, *mut bool) -> windows_core::HRESULT,
        pub SetIsBackgroundTransparent:
            unsafe extern "system" fn(*mut c_void, bool) -> windows_core::HRESULT,
    }

    windows_core::imp::define_interface!(
        IFrameworkApplicationPrivate,
        IFrameworkApplicationPrivate_Vtbl,
        0xb3ab45d8_6a4e_4e76_a00d_32d4643a9f1a
    );
    windows_core::imp::interface_hierarchy!(IFrameworkApplicationPrivate, windows_core::IUnknown);

    impl IFrameworkApplicationPrivate {
        pub unsafe fn SetSynchronizationWindow(&self, hwnd: HWND) -> windows_core::Result<()> {
            unsafe {
                (windows_core::Interface::vtable(self).SetSynchronizationWindow)(
                    windows_core::Interface::as_raw(self),
                    hwnd,
                )
            }
            .ok()
        }
    }

    #[repr(C)]
    pub struct IFrameworkApplicationPrivate_Vtbl {
        pub base__: windows_core::IUnknown_Vtbl,
        pub Reserved3: unsafe extern "system" fn(*mut c_void),
        pub Reserved4: unsafe extern "system" fn(*mut c_void),
        pub Reserved5: unsafe extern "system" fn(*mut c_void),
        pub Reserved6: unsafe extern "system" fn(*mut c_void),
        pub Reserved7: unsafe extern "system" fn(*mut c_void),
        pub Reserved8: unsafe extern "system" fn(*mut c_void),
        pub Reserved9: unsafe extern "system" fn(*mut c_void),
        pub Reserved10: unsafe extern "system" fn(*mut c_void),
        pub SetSynchronizationWindow:
            unsafe extern "system" fn(*mut c_void, HWND) -> windows_core::HRESULT,
    }
}
