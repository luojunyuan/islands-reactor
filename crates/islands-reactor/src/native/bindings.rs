#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBar, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(
    AppBar,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl windows_core::RuntimeType for AppBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppBar>();
}
unsafe impl windows_core::Interface for AppBar {
    type Vtable = <IAppBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppBar {
    type Target = IAppBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.AppBar";
}
unsafe impl Send for AppBar {}
unsafe impl Sync for AppBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Application(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Application,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Application {
    pub fn new() -> windows_core::Result<Application> {
        Self::IApplicationFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<Application>
    where
        T: windows_core::Compose,
    {
        Self::IApplicationFactory(|this| unsafe {
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
    pub fn get_Current() -> windows_core::Result<Application> {
        Self::IApplicationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_Current)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IApplicationFactory<R, F: FnOnce(&IApplicationFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Application, IApplicationFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl core::ops::Deref for Application {
    type Target = IApplication;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Application {
    const NAME: &'static str = "Windows.UI.Xaml.Application";
}
unsafe impl Send for Application {}
unsafe impl Sync for Application {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoSuggestBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AutoSuggestBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    AutoSuggestBox,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl AutoSuggestBox {
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
            AutoSuggestBox,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AutoSuggestBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAutoSuggestBox>();
}
unsafe impl windows_core::Interface for AutoSuggestBox {
    type Vtable = <IAutoSuggestBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAutoSuggestBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for AutoSuggestBox {
    type Target = IAutoSuggestBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AutoSuggestBox {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.AutoSuggestBox";
}
unsafe impl Send for AutoSuggestBox {}
unsafe impl Sync for AutoSuggestBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoSuggestBoxQuerySubmittedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AutoSuggestBoxQuerySubmittedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(AutoSuggestBoxQuerySubmittedEventArgs, DependencyObject);
impl AutoSuggestBoxQuerySubmittedEventArgs {
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
            AutoSuggestBoxQuerySubmittedEventArgs,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AutoSuggestBoxQuerySubmittedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAutoSuggestBoxQuerySubmittedEventArgs>();
}
unsafe impl windows_core::Interface for AutoSuggestBoxQuerySubmittedEventArgs {
    type Vtable = <IAutoSuggestBoxQuerySubmittedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IAutoSuggestBoxQuerySubmittedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for AutoSuggestBoxQuerySubmittedEventArgs {
    type Target = IAutoSuggestBoxQuerySubmittedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AutoSuggestBoxQuerySubmittedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.AutoSuggestBoxQuerySubmittedEventArgs";
}
unsafe impl Send for AutoSuggestBoxQuerySubmittedEventArgs {}
unsafe impl Sync for AutoSuggestBoxQuerySubmittedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoSuggestBoxSuggestionChosenEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AutoSuggestBoxSuggestionChosenEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(AutoSuggestBoxSuggestionChosenEventArgs, DependencyObject);
impl AutoSuggestBoxSuggestionChosenEventArgs {
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
            AutoSuggestBoxSuggestionChosenEventArgs,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AutoSuggestBoxSuggestionChosenEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        IAutoSuggestBoxSuggestionChosenEventArgs,
    >();
}
unsafe impl windows_core::Interface for AutoSuggestBoxSuggestionChosenEventArgs {
    type Vtable = <IAutoSuggestBoxSuggestionChosenEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IAutoSuggestBoxSuggestionChosenEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for AutoSuggestBoxSuggestionChosenEventArgs {
    type Target = IAutoSuggestBoxSuggestionChosenEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AutoSuggestBoxSuggestionChosenEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.AutoSuggestBoxSuggestionChosenEventArgs";
}
unsafe impl Send for AutoSuggestBoxSuggestionChosenEventArgs {}
unsafe impl Sync for AutoSuggestBoxSuggestionChosenEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoSuggestBoxTextChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AutoSuggestBoxTextChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(AutoSuggestBoxTextChangedEventArgs, DependencyObject);
impl AutoSuggestBoxTextChangedEventArgs {
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
            AutoSuggestBoxTextChangedEventArgs,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AutoSuggestBoxTextChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAutoSuggestBoxTextChangedEventArgs>();
}
unsafe impl windows_core::Interface for AutoSuggestBoxTextChangedEventArgs {
    type Vtable = <IAutoSuggestBoxTextChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IAutoSuggestBoxTextChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for AutoSuggestBoxTextChangedEventArgs {
    type Target = IAutoSuggestBoxTextChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AutoSuggestBoxTextChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.AutoSuggestBoxTextChangedEventArgs";
}
unsafe impl Send for AutoSuggestBoxTextChangedEventArgs {}
unsafe impl Sync for AutoSuggestBoxTextChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AutoSuggestionBoxTextChangeReason(pub i32);
impl AutoSuggestionBoxTextChangeReason {
    pub const UserInput: Self = Self(0);
    pub const ProgrammaticChange: Self = Self(1);
    pub const SuggestionChosen: Self = Self(2);
}
impl windows_core::TypeKind for AutoSuggestionBoxTextChangeReason {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AutoSuggestionBoxTextChangeReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.AutoSuggestionBoxTextChangeReason;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapImage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BitmapImage,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(BitmapImage, BitmapSource, ImageSource, DependencyObject);
impl BitmapImage {
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
            BitmapImage,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BitmapImage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBitmapImage>();
}
unsafe impl windows_core::Interface for BitmapImage {
    type Vtable = <IBitmapImage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapImage as windows_core::Interface>::IID;
}
impl core::ops::Deref for BitmapImage {
    type Target = IBitmapImage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BitmapImage {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.BitmapImage";
}
unsafe impl Send for BitmapImage {}
unsafe impl Sync for BitmapImage {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BitmapSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(BitmapSource, ImageSource, DependencyObject);
impl windows_core::RuntimeType for BitmapSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBitmapSource>();
}
unsafe impl windows_core::Interface for BitmapSource {
    type Vtable = <IBitmapSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for BitmapSource {
    type Target = IBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.BitmapSource";
}
unsafe impl Send for BitmapSource {}
unsafe impl Sync for BitmapSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Block, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Block, TextElement, DependencyObject);
impl windows_core::RuntimeType for Block {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBlock>();
}
unsafe impl windows_core::Interface for Block {
    type Vtable = <IBlock as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBlock as windows_core::Interface>::IID;
}
impl core::ops::Deref for Block {
    type Target = IBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Block {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Block";
}
unsafe impl Send for Block {}
unsafe impl Sync for Block {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BlockCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IVector<Block>
);
impl windows_core::RuntimeType for BlockCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<Block>>();
}
unsafe impl windows_core::Interface for BlockCollection {
    type Vtable = <windows_collections::IVector<Block> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows_collections::IVector<Block> as windows_core::Interface>::IID;
}
impl core::ops::Deref for BlockCollection {
    type Target = windows_collections::IVector<Block>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BlockCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.BlockCollection";
}
unsafe impl Send for BlockCollection {}
unsafe impl Sync for BlockCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Border(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Border, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Border, FrameworkElement, UIElement, DependencyObject);
impl Border {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Border, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Border {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBorder>();
}
unsafe impl windows_core::Interface for Border {
    type Vtable = <IBorder as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBorder as windows_core::Interface>::IID;
}
impl core::ops::Deref for Border {
    type Target = IBorder;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Border {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Border";
}
unsafe impl Send for Border {}
unsafe impl Sync for Border {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Brush(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Brush, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Brush, DependencyObject);
impl windows_core::RuntimeType for Brush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBrush>();
}
unsafe impl windows_core::Interface for Brush {
    type Vtable = <IBrush as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBrush as windows_core::Interface>::IID;
}
impl core::ops::Deref for Brush {
    type Target = IBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Brush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Brush";
}
unsafe impl Send for Brush {}
unsafe impl Sync for Brush {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Button(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Button, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(
    Button,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Button {
    pub fn new() -> windows_core::Result<Button> {
        Self::IButtonFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<Button>
    where
        T: windows_core::Compose,
    {
        Self::IButtonFactory(|this| unsafe {
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
    fn IButtonFactory<R, F: FnOnce(&IButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Button, IButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Button {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IButton>();
}
unsafe impl windows_core::Interface for Button {
    type Vtable = <IButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for Button {
    type Target = IButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Button {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Button";
}
unsafe impl Send for Button {}
unsafe impl Sync for Button {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ButtonBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ButtonBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl windows_core::RuntimeType for ButtonBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IButtonBase>();
}
unsafe impl windows_core::Interface for ButtonBase {
    type Vtable = <IButtonBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IButtonBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for ButtonBase {
    type Target = IButtonBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ButtonBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ButtonBase";
}
unsafe impl Send for ButtonBase {}
unsafe impl Sync for ButtonBase {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CachedFileUpdaterActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CachedFileUpdaterActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable,
    ICachedFileUpdaterActivatedEventArgs
);
windows_core::imp::required_hierarchy!(CachedFileUpdaterActivatedEventArgs, IActivatedEventArgs);
impl windows_core::RuntimeType for CachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICachedFileUpdaterActivatedEventArgs>();
}
unsafe impl windows_core::Interface for CachedFileUpdaterActivatedEventArgs {
    type Vtable = <ICachedFileUpdaterActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ICachedFileUpdaterActivatedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for CachedFileUpdaterActivatedEventArgs {
    type Target = ICachedFileUpdaterActivatedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str =
        "Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs";
}
unsafe impl Send for CachedFileUpdaterActivatedEventArgs {}
unsafe impl Sync for CachedFileUpdaterActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarDatePicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CalendarDatePicker,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CalendarDatePicker,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl CalendarDatePicker {
    pub fn new() -> windows_core::Result<CalendarDatePicker> {
        Self::ICalendarDatePickerFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<CalendarDatePicker>
    where
        T: windows_core::Compose,
    {
        Self::ICalendarDatePickerFactory(|this| unsafe {
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
    fn ICalendarDatePickerFactory<
        R,
        F: FnOnce(&ICalendarDatePickerFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            CalendarDatePicker,
            ICalendarDatePickerFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CalendarDatePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICalendarDatePicker>();
}
unsafe impl windows_core::Interface for CalendarDatePicker {
    type Vtable = <ICalendarDatePicker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICalendarDatePicker as windows_core::Interface>::IID;
}
impl core::ops::Deref for CalendarDatePicker {
    type Target = ICalendarDatePicker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CalendarDatePicker {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.CalendarDatePicker";
}
unsafe impl Send for CalendarDatePicker {}
unsafe impl Sync for CalendarDatePicker {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CalendarView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CalendarView,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl CalendarView {
    pub fn new() -> windows_core::Result<CalendarView> {
        Self::ICalendarViewFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<CalendarView>
    where
        T: windows_core::Compose,
    {
        Self::ICalendarViewFactory(|this| unsafe {
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
    fn ICalendarViewFactory<R, F: FnOnce(&ICalendarViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CalendarView, ICalendarViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CalendarView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICalendarView>();
}
unsafe impl windows_core::Interface for CalendarView {
    type Vtable = <ICalendarView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICalendarView as windows_core::Interface>::IID;
}
impl core::ops::Deref for CalendarView {
    type Target = ICalendarView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CalendarView {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.CalendarView";
}
unsafe impl Send for CalendarView {}
unsafe impl Sync for CalendarView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarViewSelectedDatesChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CalendarViewSelectedDatesChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for CalendarViewSelectedDatesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        ICalendarViewSelectedDatesChangedEventArgs,
    >();
}
unsafe impl windows_core::Interface for CalendarViewSelectedDatesChangedEventArgs {
    type Vtable = <ICalendarViewSelectedDatesChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ICalendarViewSelectedDatesChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for CalendarViewSelectedDatesChangedEventArgs {
    type Target = ICalendarViewSelectedDatesChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CalendarViewSelectedDatesChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.CalendarViewSelectedDatesChangedEventArgs";
}
unsafe impl Send for CalendarViewSelectedDatesChangedEventArgs {}
unsafe impl Sync for CalendarViewSelectedDatesChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Canvas(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Canvas, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(
    Canvas,
    Panel,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Canvas {
    pub fn new() -> windows_core::Result<Canvas> {
        Self::ICanvasFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<Canvas>
    where
        T: windows_core::Compose,
    {
        Self::ICanvasFactory(|this| unsafe {
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
    pub fn SetLeft<P0>(element: P0, length: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::ICanvasStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetLeft)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                length,
            )
            .ok()
        })
    }
    pub fn SetTop<P0>(element: P0, length: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::ICanvasStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetTop)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                length,
            )
            .ok()
        })
    }
    pub fn SetZIndex<P0>(element: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::ICanvasStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetZIndex)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    fn ICanvasFactory<R, F: FnOnce(&ICanvasFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Canvas, ICanvasFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ICanvasStatics<R, F: FnOnce(&ICanvasStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Canvas, ICanvasStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Canvas {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICanvas>();
}
unsafe impl windows_core::Interface for Canvas {
    type Vtable = <ICanvas as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICanvas as windows_core::Interface>::IID;
}
impl core::ops::Deref for Canvas {
    type Target = ICanvas;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Canvas {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Canvas";
}
unsafe impl Send for Canvas {}
unsafe impl Sync for Canvas {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CheckBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CheckBox,
    ToggleButton,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl CheckBox {
    pub fn new() -> windows_core::Result<CheckBox> {
        Self::ICheckBoxFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<CheckBox>
    where
        T: windows_core::Compose,
    {
        Self::ICheckBoxFactory(|this| unsafe {
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
    fn ICheckBoxFactory<R, F: FnOnce(&ICheckBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CheckBox, ICheckBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CheckBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICheckBox>();
}
unsafe impl windows_core::Interface for CheckBox {
    type Vtable = <ICheckBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICheckBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for CheckBox {
    type Target = ICheckBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CheckBox {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.CheckBox";
}
unsafe impl Send for CheckBox {}
unsafe impl Sync for CheckBox {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub A: u8,
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl windows_core::TypeKind for Color {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Color {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ColorChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for ColorChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IColorChangedEventArgs>();
}
unsafe impl windows_core::Interface for ColorChangedEventArgs {
    type Vtable = <IColorChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IColorChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ColorChangedEventArgs {
    type Target = IColorChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ColorChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ColorChangedEventArgs";
}
unsafe impl Send for ColorChangedEventArgs {}
unsafe impl Sync for ColorChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorPicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ColorPicker,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ColorPicker,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ColorPicker {
    pub fn new() -> windows_core::Result<ColorPicker> {
        Self::IColorPickerFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<ColorPicker>
    where
        T: windows_core::Compose,
    {
        Self::IColorPickerFactory(|this| unsafe {
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
    fn IColorPickerFactory<R, F: FnOnce(&IColorPickerFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ColorPicker, IColorPickerFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ColorPicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IColorPicker>();
}
unsafe impl windows_core::Interface for ColorPicker {
    type Vtable = <IColorPicker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IColorPicker as windows_core::Interface>::IID;
}
impl core::ops::Deref for ColorPicker {
    type Target = IColorPicker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ColorPicker {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ColorPicker";
}
unsafe impl Send for ColorPicker {}
unsafe impl Sync for ColorPicker {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ColumnDefinition,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ColumnDefinition, DependencyObject);
impl ColumnDefinition {
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
            ColumnDefinition,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ColumnDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IColumnDefinition>();
}
unsafe impl windows_core::Interface for ColumnDefinition {
    type Vtable = <IColumnDefinition as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IColumnDefinition as windows_core::Interface>::IID;
}
impl core::ops::Deref for ColumnDefinition {
    type Target = IColumnDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ColumnDefinition {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ColumnDefinition";
}
unsafe impl Send for ColumnDefinition {}
unsafe impl Sync for ColumnDefinition {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnDefinitionCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ColumnDefinitionCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IVector<ColumnDefinition>
);
impl windows_core::RuntimeType for ColumnDefinitionCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        windows_collections::IVector<ColumnDefinition>,
    >();
}
unsafe impl windows_core::Interface for ColumnDefinitionCollection {
    type Vtable =
        <windows_collections::IVector<ColumnDefinition> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows_collections::IVector<ColumnDefinition> as windows_core::Interface>::IID;
}
impl core::ops::Deref for ColumnDefinitionCollection {
    type Target = windows_collections::IVector<ColumnDefinition>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ColumnDefinitionCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ColumnDefinitionCollection";
}
unsafe impl Send for ColumnDefinitionCollection {}
unsafe impl Sync for ColumnDefinitionCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComboBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ComboBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ComboBox,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ComboBox {
    pub fn new() -> windows_core::Result<ComboBox> {
        Self::IComboBoxFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<ComboBox>
    where
        T: windows_core::Compose,
    {
        Self::IComboBoxFactory(|this| unsafe {
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
    fn IComboBoxFactory<R, F: FnOnce(&IComboBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ComboBox, IComboBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ComboBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IComboBox>();
}
unsafe impl windows_core::Interface for ComboBox {
    type Vtable = <IComboBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IComboBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for ComboBox {
    type Target = IComboBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ComboBox {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ComboBox";
}
unsafe impl Send for ComboBox {}
unsafe impl Sync for ComboBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CommandBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CommandBar,
    AppBar,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl CommandBar {
    pub fn new() -> windows_core::Result<CommandBar> {
        Self::ICommandBarFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<CommandBar>
    where
        T: windows_core::Compose,
    {
        Self::ICommandBarFactory(|this| unsafe {
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
    fn ICommandBarFactory<R, F: FnOnce(&ICommandBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CommandBar, ICommandBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CommandBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICommandBar>();
}
unsafe impl windows_core::Interface for CommandBar {
    type Vtable = <ICommandBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICommandBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for CommandBar {
    type Target = ICommandBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CommandBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.CommandBar";
}
unsafe impl Send for CommandBar {}
unsafe impl Sync for CommandBar {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CommandBarDefaultLabelPosition(pub i32);
impl CommandBarDefaultLabelPosition {
    pub const Bottom: Self = Self(0);
    pub const Right: Self = Self(1);
    pub const Collapsed: Self = Self(2);
}
impl windows_core::TypeKind for CommandBarDefaultLabelPosition {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CommandBarDefaultLabelPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.CommandBarDefaultLabelPosition;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionTarget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionTarget,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl CompositionTarget {
    pub fn add_Rendering<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<windows_core::IInspectable>,
            ) + Send
            + 'static,
    {
        let handler =
            <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| {
                handler(a0, a1);
                Ok(())
            });
        Self::ICompositionTargetStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).add_Rendering)(
                windows_core::Interface::as_raw(this),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                this.clone(),
                token__,
                windows_core::Interface::vtable(this).remove_Rendering,
            ))
        })
    }
    fn ICompositionTargetStatics<
        R,
        F: FnOnce(&ICompositionTargetStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            CompositionTarget,
            ICompositionTargetStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CompositionTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionTarget>();
}
unsafe impl windows_core::Interface for CompositionTarget {
    type Vtable = <ICompositionTarget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionTarget as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionTarget {
    type Target = ICompositionTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionTarget {
    const NAME: &'static str = "Windows.UI.Xaml.Media.CompositionTarget";
}
unsafe impl Send for CompositionTarget {}
unsafe impl Sync for CompositionTarget {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContentControl,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl windows_core::RuntimeType for ContentControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContentControl>();
}
unsafe impl windows_core::Interface for ContentControl {
    type Vtable = <IContentControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContentControl as windows_core::Interface>::IID;
}
impl core::ops::Deref for ContentControl {
    type Target = IContentControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ContentControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ContentControl";
}
unsafe impl Send for ContentControl {}
unsafe impl Sync for ContentControl {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentDialog(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContentDialog,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ContentDialog,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ContentDialog {
    pub fn new() -> windows_core::Result<ContentDialog> {
        Self::IContentDialogFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<ContentDialog>
    where
        T: windows_core::Compose,
    {
        Self::IContentDialogFactory(|this| unsafe {
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
    fn IContentDialogFactory<R, F: FnOnce(&IContentDialogFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContentDialog, IContentDialogFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ContentDialog {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContentDialog>();
}
unsafe impl windows_core::Interface for ContentDialog {
    type Vtable = <IContentDialog as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContentDialog as windows_core::Interface>::IID;
}
impl core::ops::Deref for ContentDialog {
    type Target = IContentDialog;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ContentDialog {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ContentDialog";
}
unsafe impl Send for ContentDialog {}
unsafe impl Sync for ContentDialog {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Control(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Control,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(Control, FrameworkElement, UIElement, DependencyObject);
impl windows_core::RuntimeType for Control {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IControl>();
}
unsafe impl windows_core::Interface for Control {
    type Vtable = <IControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IControl as windows_core::Interface>::IID;
}
impl core::ops::Deref for Control {
    type Target = IControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Control {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Control";
}
unsafe impl Send for Control {}
unsafe impl Sync for Control {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CornerRadius {
    pub TopLeft: f64,
    pub TopRight: f64,
    pub BottomRight: f64,
    pub BottomLeft: f64,
}
impl windows_core::TypeKind for CornerRadius {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CornerRadius {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Xaml.CornerRadius;f8;f8;f8;f8)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataTemplate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DataTemplate,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(DataTemplate, FrameworkTemplate, DependencyObject);
impl DataTemplate {
    pub fn new() -> windows_core::Result<DataTemplate> {
        Self::IDataTemplateFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<DataTemplate>
    where
        T: windows_core::Compose,
    {
        Self::IDataTemplateFactory(|this| unsafe {
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
    fn IDataTemplateFactory<R, F: FnOnce(&IDataTemplateFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataTemplate, IDataTemplateFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DataTemplate {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDataTemplate>();
}
unsafe impl windows_core::Interface for DataTemplate {
    type Vtable = <IDataTemplate as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataTemplate as windows_core::Interface>::IID;
}
impl core::ops::Deref for DataTemplate {
    type Target = IDataTemplate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DataTemplate {
    const NAME: &'static str = "Windows.UI.Xaml.DataTemplate";
}
unsafe impl Send for DataTemplate {}
unsafe impl Sync for DataTemplate {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatePicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DatePicker,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    DatePicker,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl DatePicker {
    pub fn new() -> windows_core::Result<DatePicker> {
        Self::IDatePickerFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<DatePicker>
    where
        T: windows_core::Compose,
    {
        Self::IDatePickerFactory(|this| unsafe {
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
    fn IDatePickerFactory<R, F: FnOnce(&IDatePickerFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DatePicker, IDatePickerFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DatePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDatePicker>();
}
unsafe impl windows_core::Interface for DatePicker {
    type Vtable = <IDatePicker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDatePicker as windows_core::Interface>::IID;
}
impl core::ops::Deref for DatePicker {
    type Target = IDatePicker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DatePicker {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.DatePicker";
}
unsafe impl Send for DatePicker {}
unsafe impl Sync for DatePicker {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatePickerValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DatePickerValueChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DatePickerValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDatePickerValueChangedEventArgs>();
}
unsafe impl windows_core::Interface for DatePickerValueChangedEventArgs {
    type Vtable = <IDatePickerValueChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IDatePickerValueChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for DatePickerValueChangedEventArgs {
    type Target = IDatePickerValueChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DatePickerValueChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.DatePickerValueChangedEventArgs";
}
unsafe impl Send for DatePickerValueChangedEventArgs {}
unsafe impl Sync for DatePickerValueChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DependencyObject,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDependencyObject>();
}
unsafe impl windows_core::Interface for DependencyObject {
    type Vtable = <IDependencyObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDependencyObject as windows_core::Interface>::IID;
}
impl core::ops::Deref for DependencyObject {
    type Target = IDependencyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DependencyObject {
    const NAME: &'static str = "Windows.UI.Xaml.DependencyObject";
}
unsafe impl Send for DependencyObject {}
unsafe impl Sync for DependencyObject {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesktopWindowXamlSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DesktopWindowXamlSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DesktopWindowXamlSource {
    pub fn new() -> windows_core::Result<DesktopWindowXamlSource> {
        Self::IDesktopWindowXamlSourceFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<DesktopWindowXamlSource>
    where
        T: windows_core::Compose,
    {
        Self::IDesktopWindowXamlSourceFactory(|this| unsafe {
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
    fn IDesktopWindowXamlSourceFactory<
        R,
        F: FnOnce(&IDesktopWindowXamlSourceFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            DesktopWindowXamlSource,
            IDesktopWindowXamlSourceFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DesktopWindowXamlSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDesktopWindowXamlSource>();
}
unsafe impl windows_core::Interface for DesktopWindowXamlSource {
    type Vtable = <IDesktopWindowXamlSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDesktopWindowXamlSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for DesktopWindowXamlSource {
    type Target = IDesktopWindowXamlSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DesktopWindowXamlSource {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.DesktopWindowXamlSource";
}
unsafe impl Send for DesktopWindowXamlSource {}
unsafe impl Sync for DesktopWindowXamlSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatcherQueue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DispatcherQueue,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DispatcherQueue {
    pub fn GetForCurrentThread() -> windows_core::Result<DispatcherQueue> {
        Self::IDispatcherQueueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentThread)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IDispatcherQueueStatics<
        R,
        F: FnOnce(&IDispatcherQueueStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DispatcherQueue, IDispatcherQueueStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DispatcherQueue {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDispatcherQueue>();
}
unsafe impl windows_core::Interface for DispatcherQueue {
    type Vtable = <IDispatcherQueue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDispatcherQueue as windows_core::Interface>::IID;
}
impl core::ops::Deref for DispatcherQueue {
    type Target = IDispatcherQueue;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DispatcherQueue {
    const NAME: &'static str = "Windows.System.DispatcherQueue";
}
unsafe impl Send for DispatcherQueue {}
unsafe impl Sync for DispatcherQueue {}
windows_core::imp::define_interface!(
    DispatcherQueueHandler,
    DispatcherQueueHandler_Vtbl,
    0xdfa2dc9c_1a2d_4917_98f2_939af1d6e0c8
);
impl windows_core::RuntimeType for DispatcherQueueHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl DispatcherQueueHandler {
    pub fn new<F: Fn() + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<DispatcherQueueHandler, F>::new(
            &DispatcherQueueHandlerBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
}
#[repr(C)]
pub struct DispatcherQueueHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct DispatcherQueueHandlerBox<F: Fn() + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn() + 'static> DispatcherQueueHandlerBox<F> {
    const VTABLE: DispatcherQueueHandler_Vtbl = DispatcherQueueHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<DispatcherQueueHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<DispatcherQueueHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<DispatcherQueueHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<DispatcherQueueHandler, F>);
            (this.invoke)();
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: Self = Self(-10);
    pub const Normal: Self = Self(0);
    pub const High: Self = Self(10);
}
impl windows_core::TypeKind for DispatcherQueuePriority {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DispatcherQueuePriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.System.DispatcherQueuePriority;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatcherTimer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DispatcherTimer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DispatcherTimer {
    pub fn new() -> windows_core::Result<DispatcherTimer> {
        Self::IDispatcherTimerFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<DispatcherTimer>
    where
        T: windows_core::Compose,
    {
        Self::IDispatcherTimerFactory(|this| unsafe {
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
    fn IDispatcherTimerFactory<
        R,
        F: FnOnce(&IDispatcherTimerFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DispatcherTimer, IDispatcherTimerFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DispatcherTimer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDispatcherTimer>();
}
unsafe impl windows_core::Interface for DispatcherTimer {
    type Vtable = <IDispatcherTimer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDispatcherTimer as windows_core::Interface>::IID;
}
impl core::ops::Deref for DispatcherTimer {
    type Target = IDispatcherTimer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DispatcherTimer {
    const NAME: &'static str = "Windows.UI.Xaml.DispatcherTimer";
}
unsafe impl Send for DispatcherTimer {}
unsafe impl Sync for DispatcherTimer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DropDownButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DropDownButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    DropDownButton,
    Button,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl DropDownButton {
    pub fn new() -> windows_core::Result<DropDownButton> {
        Self::IDropDownButtonFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<DropDownButton>
    where
        T: windows_core::Compose,
    {
        Self::IDropDownButtonFactory(|this| unsafe {
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
    fn IDropDownButtonFactory<R, F: FnOnce(&IDropDownButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DropDownButton, IDropDownButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DropDownButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDropDownButton>();
}
unsafe impl windows_core::Interface for DropDownButton {
    type Vtable = <IDropDownButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDropDownButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for DropDownButton {
    type Target = IDropDownButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DropDownButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.DropDownButton";
}
unsafe impl Send for DropDownButton {}
unsafe impl Sync for DropDownButton {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ElementTheme(pub i32);
impl ElementTheme {
    pub const Default: Self = Self(0);
    pub const Light: Self = Self(1);
    pub const Dark: Self = Self(2);
}
impl windows_core::TypeKind for ElementTheme {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ElementTheme {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.ElementTheme;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ellipse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Ellipse,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    Ellipse,
    Shape,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Ellipse {
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
            Ellipse,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Ellipse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IEllipse>();
}
unsafe impl windows_core::Interface for Ellipse {
    type Vtable = <IEllipse as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IEllipse as windows_core::Interface>::IID;
}
impl core::ops::Deref for Ellipse {
    type Target = IEllipse;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Ellipse {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Ellipse";
}
unsafe impl Send for Ellipse {}
unsafe impl Sync for Ellipse {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FileActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IFileActivatedEventArgs
);
windows_core::imp::required_hierarchy!(FileActivatedEventArgs, IActivatedEventArgs);
impl windows_core::RuntimeType for FileActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFileActivatedEventArgs>();
}
unsafe impl windows_core::Interface for FileActivatedEventArgs {
    type Vtable = <IFileActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileActivatedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for FileActivatedEventArgs {
    type Target = IFileActivatedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileActivatedEventArgs";
}
unsafe impl Send for FileActivatedEventArgs {}
unsafe impl Sync for FileActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileOpenPickerActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FileOpenPickerActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IFileOpenPickerActivatedEventArgs
);
windows_core::imp::required_hierarchy!(FileOpenPickerActivatedEventArgs, IActivatedEventArgs);
impl windows_core::RuntimeType for FileOpenPickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFileOpenPickerActivatedEventArgs>();
}
unsafe impl windows_core::Interface for FileOpenPickerActivatedEventArgs {
    type Vtable = <IFileOpenPickerActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IFileOpenPickerActivatedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for FileOpenPickerActivatedEventArgs {
    type Target = IFileOpenPickerActivatedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FileOpenPickerActivatedEventArgs {
    const NAME: &'static str =
        "Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs";
}
unsafe impl Send for FileOpenPickerActivatedEventArgs {}
unsafe impl Sync for FileOpenPickerActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileSavePickerActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FileSavePickerActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IFileSavePickerActivatedEventArgs
);
windows_core::imp::required_hierarchy!(FileSavePickerActivatedEventArgs, IActivatedEventArgs);
impl windows_core::RuntimeType for FileSavePickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFileSavePickerActivatedEventArgs>();
}
unsafe impl windows_core::Interface for FileSavePickerActivatedEventArgs {
    type Vtable = <IFileSavePickerActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IFileSavePickerActivatedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for FileSavePickerActivatedEventArgs {
    type Target = IFileSavePickerActivatedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FileSavePickerActivatedEventArgs {
    const NAME: &'static str =
        "Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs";
}
unsafe impl Send for FileSavePickerActivatedEventArgs {}
unsafe impl Sync for FileSavePickerActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlipView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FlipView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    FlipView,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl FlipView {
    pub fn new() -> windows_core::Result<FlipView> {
        Self::IFlipViewFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<FlipView>
    where
        T: windows_core::Compose,
    {
        Self::IFlipViewFactory(|this| unsafe {
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
    fn IFlipViewFactory<R, F: FnOnce(&IFlipViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FlipView, IFlipViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FlipView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFlipView>();
}
unsafe impl windows_core::Interface for FlipView {
    type Vtable = <IFlipView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFlipView as windows_core::Interface>::IID;
}
impl core::ops::Deref for FlipView {
    type Target = IFlipView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FlipView {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.FlipView";
}
unsafe impl Send for FlipView {}
unsafe impl Sync for FlipView {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FlyoutPlacementMode(pub i32);
impl FlyoutPlacementMode {
    pub const Top: Self = Self(0);
    pub const Bottom: Self = Self(1);
    pub const Left: Self = Self(2);
    pub const Right: Self = Self(3);
    pub const Full: Self = Self(4);
    pub const TopEdgeAlignedLeft: Self = Self(5);
    pub const TopEdgeAlignedRight: Self = Self(6);
    pub const BottomEdgeAlignedLeft: Self = Self(7);
    pub const BottomEdgeAlignedRight: Self = Self(8);
    pub const LeftEdgeAlignedTop: Self = Self(9);
    pub const LeftEdgeAlignedBottom: Self = Self(10);
    pub const RightEdgeAlignedTop: Self = Self(11);
    pub const RightEdgeAlignedBottom: Self = Self(12);
    pub const Auto: Self = Self(13);
}
impl windows_core::TypeKind for FlyoutPlacementMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FlyoutPlacementMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Primitives.FlyoutPlacementMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontFamily(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FontFamily,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FontFamily {
    pub fn CreateInstanceWithName(familyname: &str) -> windows_core::Result<FontFamily> {
        Self::IFontFamilyFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(familyname)),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstanceWithName_compose<T>(
        familyname: &str,
        compose: T,
    ) -> windows_core::Result<FontFamily>
    where
        T: windows_core::Compose,
    {
        Self::IFontFamilyFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(familyname)),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IFontFamilyFactory<R, F: FnOnce(&IFontFamilyFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FontFamily, IFontFamilyFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FontFamily {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFontFamily>();
}
unsafe impl windows_core::Interface for FontFamily {
    type Vtable = <IFontFamily as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFontFamily as windows_core::Interface>::IID;
}
impl core::ops::Deref for FontFamily {
    type Target = IFontFamily;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FontFamily {
    const NAME: &'static str = "Windows.UI.Xaml.Media.FontFamily";
}
unsafe impl Send for FontFamily {}
unsafe impl Sync for FontFamily {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FontWeight {
    pub Weight: u16,
}
impl windows_core::TypeKind for FontWeight {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FontWeight {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Text.FontWeight;u2)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameworkElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FrameworkElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(FrameworkElement, UIElement, DependencyObject);
impl windows_core::RuntimeType for FrameworkElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFrameworkElement>();
}
unsafe impl windows_core::Interface for FrameworkElement {
    type Vtable = <IFrameworkElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFrameworkElement as windows_core::Interface>::IID;
}
impl core::ops::Deref for FrameworkElement {
    type Target = IFrameworkElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FrameworkElement {
    const NAME: &'static str = "Windows.UI.Xaml.FrameworkElement";
}
unsafe impl Send for FrameworkElement {}
unsafe impl Sync for FrameworkElement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameworkTemplate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FrameworkTemplate,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(FrameworkTemplate, DependencyObject);
impl windows_core::RuntimeType for FrameworkTemplate {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFrameworkTemplate>();
}
unsafe impl windows_core::Interface for FrameworkTemplate {
    type Vtable = <IFrameworkTemplate as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFrameworkTemplate as windows_core::Interface>::IID;
}
impl core::ops::Deref for FrameworkTemplate {
    type Target = IFrameworkTemplate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FrameworkTemplate {
    const NAME: &'static str = "Windows.UI.Xaml.FrameworkTemplate";
}
unsafe impl Send for FrameworkTemplate {}
unsafe impl Sync for FrameworkTemplate {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Grid, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Grid, Panel, FrameworkElement, UIElement, DependencyObject);
impl Grid {
    pub fn new() -> windows_core::Result<Grid> {
        Self::IGridFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<Grid>
    where
        T: windows_core::Compose,
    {
        Self::IGridFactory(|this| unsafe {
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
    pub fn SetRow<P0>(element: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IGridStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetRow)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetColumn<P0>(element: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IGridStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetColumn)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetRowSpan<P0>(element: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IGridStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetRowSpan)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetColumnSpan<P0>(element: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IGridStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetColumnSpan)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    fn IGridFactory<R, F: FnOnce(&IGridFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Grid, IGridFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IGridStatics<R, F: FnOnce(&IGridStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Grid, IGridStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Grid {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IGrid>();
}
unsafe impl windows_core::Interface for Grid {
    type Vtable = <IGrid as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGrid as windows_core::Interface>::IID;
}
impl core::ops::Deref for Grid {
    type Target = IGrid;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Grid {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Grid";
}
unsafe impl Send for Grid {}
unsafe impl Sync for Grid {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GridLength {
    pub Value: f64,
    pub GridUnitType: GridUnitType,
}
impl windows_core::TypeKind for GridLength {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GridLength {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Xaml.GridLength;f8;enum(Windows.UI.Xaml.GridUnitType;i4))",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GridUnitType(pub i32);
impl GridUnitType {
    pub const Auto: Self = Self(0);
    pub const Pixel: Self = Self(1);
    pub const Star: Self = Self(2);
}
impl windows_core::TypeKind for GridUnitType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GridUnitType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.GridUnitType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    GridView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    GridView,
    ListViewBase,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl GridView {
    pub fn new() -> windows_core::Result<GridView> {
        Self::IGridViewFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<GridView>
    where
        T: windows_core::Compose,
    {
        Self::IGridViewFactory(|this| unsafe {
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
    fn IGridViewFactory<R, F: FnOnce(&IGridViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GridView, IGridViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GridView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IGridView>();
}
unsafe impl windows_core::Interface for GridView {
    type Vtable = <IGridView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGridView as windows_core::Interface>::IID;
}
impl core::ops::Deref for GridView {
    type Target = IGridView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for GridView {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.GridView";
}
unsafe impl Send for GridView {}
unsafe impl Sync for GridView {}
pub type HWND = *mut core::ffi::c_void;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HorizontalAlignment(pub i32);
impl HorizontalAlignment {
    pub const Left: Self = Self(0);
    pub const Center: Self = Self(1);
    pub const Right: Self = Self(2);
    pub const Stretch: Self = Self(3);
}
impl windows_core::TypeKind for HorizontalAlignment {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HorizontalAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.HorizontalAlignment;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HyperlinkButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    HyperlinkButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    HyperlinkButton,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl HyperlinkButton {
    pub fn new() -> windows_core::Result<HyperlinkButton> {
        Self::IHyperlinkButtonFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<HyperlinkButton>
    where
        T: windows_core::Compose,
    {
        Self::IHyperlinkButtonFactory(|this| unsafe {
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
    fn IHyperlinkButtonFactory<
        R,
        F: FnOnce(&IHyperlinkButtonFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HyperlinkButton, IHyperlinkButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HyperlinkButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IHyperlinkButton>();
}
unsafe impl windows_core::Interface for HyperlinkButton {
    type Vtable = <IHyperlinkButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHyperlinkButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for HyperlinkButton {
    type Target = IHyperlinkButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for HyperlinkButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.HyperlinkButton";
}
unsafe impl Send for HyperlinkButton {}
unsafe impl Sync for HyperlinkButton {}
windows_core::imp::define_interface!(
    IActivatedEventArgs,
    IActivatedEventArgs_Vtbl,
    0xcf651713_cd08_4fd8_b697_a281b6544e2e
);
impl windows_core::RuntimeType for IActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
#[repr(C)]
pub struct IActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Kind: usize,
    get_PreviousExecutionState: usize,
    get_SplashScreen: usize,
}
windows_core::imp::define_interface!(
    IAppBar,
    IAppBar_Vtbl,
    0x7b0fc253_86a5_4b43_9872_0b8a6234b74b
);
impl windows_core::RuntimeType for IAppBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsOpen: usize,
    put_IsOpen: usize,
    get_IsSticky: usize,
    put_IsSticky: usize,
    add_Opened: usize,
    remove_Opened: usize,
    add_Closed: usize,
    remove_Closed: usize,
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
impl IApplication {
    pub fn get_Resources(&self) -> windows_core::Result<ResourceDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Resources)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Resources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ResourceDictionary>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Resources)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn add_UnhandledException<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<UnhandledExceptionEventArgs>,
            ) + 'static,
    {
        let handler: UnhandledExceptionEventHandler = {
            let com = windows_core::imp::DelegateBox::<UnhandledExceptionEventHandler, F>::new(
                &UnhandledExceptionEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_UnhandledException)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_UnhandledException,
            ))
        }
    }
}
#[repr(C)]
pub struct IApplication_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Resources: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Resources: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_DebugSettings: usize,
    get_RequestedTheme: usize,
    put_RequestedTheme: usize,
    pub add_UnhandledException: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_UnhandledException:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_Suspending: usize,
    remove_Suspending: usize,
    add_Resuming: usize,
    remove_Resuming: usize,
    Exit: usize,
}
windows_core::imp::define_interface!(
    IApplicationFactory,
    IApplicationFactory_Vtbl,
    0x93bbe361_be5a_4ee3_b4a3_95118dc97a89
);
impl windows_core::RuntimeType for IApplicationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IApplicationOverrides,
    IApplicationOverrides_Vtbl,
    0x25f99ff7_9347_459a_9fac_b2d0e11c1a0f
);
impl windows_core::RuntimeType for IApplicationOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Xaml.IApplicationOverrides");
}
impl IApplicationOverrides {
    pub fn OnActivated<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActivatedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnActivated)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnLaunched<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LaunchActivatedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnLaunched)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnFileActivated<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FileActivatedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnFileActivated)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnSearchActivated<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SearchActivatedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnSearchActivated)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnShareTargetActivated<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ShareTargetActivatedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnShareTargetActivated)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnFileOpenPickerActivated<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FileOpenPickerActivatedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnFileOpenPickerActivated)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnFileSavePickerActivated<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FileSavePickerActivatedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnFileSavePickerActivated)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnCachedFileUpdaterActivated<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CachedFileUpdaterActivatedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnCachedFileUpdaterActivated)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnWindowCreated<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WindowCreatedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnWindowCreated)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IApplicationOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationOverrides";
}
pub trait IApplicationOverrides_Impl: windows_core::IUnknownImpl {
    fn OnActivated(&self, args: windows_core::Ref<IActivatedEventArgs>)
    -> windows_core::Result<()>;
    fn OnLaunched(
        &self,
        args: windows_core::Ref<LaunchActivatedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnFileActivated(
        &self,
        args: windows_core::Ref<FileActivatedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnSearchActivated(
        &self,
        args: windows_core::Ref<SearchActivatedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnShareTargetActivated(
        &self,
        args: windows_core::Ref<ShareTargetActivatedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnFileOpenPickerActivated(
        &self,
        args: windows_core::Ref<FileOpenPickerActivatedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnFileSavePickerActivated(
        &self,
        args: windows_core::Ref<FileSavePickerActivatedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnCachedFileUpdaterActivated(
        &self,
        args: windows_core::Ref<CachedFileUpdaterActivatedEventArgs>,
    ) -> windows_core::Result<()>;
    fn OnWindowCreated(
        &self,
        args: windows_core::Ref<WindowCreatedEventArgs>,
    ) -> windows_core::Result<()>;
}
impl IApplicationOverrides_Vtbl {
    pub const fn new<Identity: IApplicationOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnActivated<
            Identity: IApplicationOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationOverrides_Impl::OnActivated(this, core::mem::transmute_copy(&args))
                    .into()
            }
        }
        unsafe extern "system" fn OnLaunched<
            Identity: IApplicationOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationOverrides_Impl::OnLaunched(this, core::mem::transmute_copy(&args))
                    .into()
            }
        }
        unsafe extern "system" fn OnFileActivated<
            Identity: IApplicationOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationOverrides_Impl::OnFileActivated(this, core::mem::transmute_copy(&args))
                    .into()
            }
        }
        unsafe extern "system" fn OnSearchActivated<
            Identity: IApplicationOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationOverrides_Impl::OnSearchActivated(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        unsafe extern "system" fn OnShareTargetActivated<
            Identity: IApplicationOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationOverrides_Impl::OnShareTargetActivated(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        unsafe extern "system" fn OnFileOpenPickerActivated<
            Identity: IApplicationOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationOverrides_Impl::OnFileOpenPickerActivated(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        unsafe extern "system" fn OnFileSavePickerActivated<
            Identity: IApplicationOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationOverrides_Impl::OnFileSavePickerActivated(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        unsafe extern "system" fn OnCachedFileUpdaterActivated<
            Identity: IApplicationOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationOverrides_Impl::OnCachedFileUpdaterActivated(
                    this,
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        unsafe extern "system" fn OnWindowCreated<
            Identity: IApplicationOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationOverrides_Impl::OnWindowCreated(this, core::mem::transmute_copy(&args))
                    .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IApplicationOverrides, OFFSET>(
            ),
            OnActivated: OnActivated::<Identity, OFFSET>,
            OnLaunched: OnLaunched::<Identity, OFFSET>,
            OnFileActivated: OnFileActivated::<Identity, OFFSET>,
            OnSearchActivated: OnSearchActivated::<Identity, OFFSET>,
            OnShareTargetActivated: OnShareTargetActivated::<Identity, OFFSET>,
            OnFileOpenPickerActivated: OnFileOpenPickerActivated::<Identity, OFFSET>,
            OnFileSavePickerActivated: OnFileSavePickerActivated::<Identity, OFFSET>,
            OnCachedFileUpdaterActivated: OnCachedFileUpdaterActivated::<Identity, OFFSET>,
            OnWindowCreated: OnWindowCreated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplicationOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IApplicationOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnActivated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnLaunched: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnFileActivated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnSearchActivated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnShareTargetActivated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnFileOpenPickerActivated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnFileSavePickerActivated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnCachedFileUpdaterActivated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnWindowCreated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
#[repr(C)]
pub struct IApplicationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Current: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Start: usize,
    LoadComponent: usize,
    LoadComponentWithResourceLocation: usize,
}
windows_core::imp::define_interface!(
    IAutoSuggestBox,
    IAutoSuggestBox_Vtbl,
    0x103e9b13_3400_4a16_90b9_6912bf06974f
);
impl windows_core::RuntimeType for IAutoSuggestBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAutoSuggestBox {
    pub fn get_Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn put_Text(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Text)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn add_SuggestionChosen<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<AutoSuggestBox>,
                windows_core::Ref<AutoSuggestBoxSuggestionChosenEventArgs>,
            ) + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            AutoSuggestBox,
            AutoSuggestBoxSuggestionChosenEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SuggestionChosen)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SuggestionChosen,
            ))
        }
    }
    pub fn add_TextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<AutoSuggestBox>,
                windows_core::Ref<AutoSuggestBoxTextChangedEventArgs>,
            ) + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            AutoSuggestBox,
            AutoSuggestBoxTextChangedEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_TextChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_TextChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IAutoSuggestBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_MaxSuggestionListHeight: usize,
    put_MaxSuggestionListHeight: usize,
    get_IsSuggestionListOpen: usize,
    put_IsSuggestionListOpen: usize,
    get_TextMemberPath: usize,
    put_TextMemberPath: usize,
    pub get_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_UpdateTextOnSelect: usize,
    put_UpdateTextOnSelect: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_AutoMaximizeSuggestionArea: usize,
    put_AutoMaximizeSuggestionArea: usize,
    get_TextBoxStyle: usize,
    put_TextBoxStyle: usize,
    pub add_SuggestionChosen: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SuggestionChosen:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_TextChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_TextChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAutoSuggestBox2,
    IAutoSuggestBox2_Vtbl,
    0xaa87ddde_e679_45b2_a7c9_9aedc39db886
);
impl windows_core::RuntimeType for IAutoSuggestBox2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAutoSuggestBox2 {
    pub fn add_QuerySubmitted<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<AutoSuggestBox>,
                windows_core::Ref<AutoSuggestBoxQuerySubmittedEventArgs>,
            ) + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            AutoSuggestBox,
            AutoSuggestBoxQuerySubmittedEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_QuerySubmitted)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_QuerySubmitted,
            ))
        }
    }
}
#[repr(C)]
pub struct IAutoSuggestBox2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_QueryIcon: usize,
    put_QueryIcon: usize,
    pub add_QuerySubmitted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_QuerySubmitted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAutoSuggestBoxQuerySubmittedEventArgs,
    IAutoSuggestBoxQuerySubmittedEventArgs_Vtbl,
    0x78dcb116_818a_4cb5_bca7_382ce6ddc90d
);
impl windows_core::RuntimeType for IAutoSuggestBoxQuerySubmittedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAutoSuggestBoxQuerySubmittedEventArgs {
    pub fn get_QueryText(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_QueryText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
}
#[repr(C)]
pub struct IAutoSuggestBoxQuerySubmittedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_QueryText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ChosenSuggestion: usize,
}
windows_core::imp::define_interface!(
    IAutoSuggestBoxSuggestionChosenEventArgs,
    IAutoSuggestBoxSuggestionChosenEventArgs_Vtbl,
    0x396f7254_1ed5_4bc5_a060_655530bca6ba
);
impl windows_core::RuntimeType for IAutoSuggestBoxSuggestionChosenEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAutoSuggestBoxSuggestionChosenEventArgs {
    pub fn get_SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IAutoSuggestBoxSuggestionChosenEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAutoSuggestBoxTextChangedEventArgs,
    IAutoSuggestBoxTextChangedEventArgs_Vtbl,
    0x3a6f7254_1ed5_4bc5_a060_655530bca6ba
);
impl windows_core::RuntimeType for IAutoSuggestBoxTextChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAutoSuggestBoxTextChangedEventArgs {
    pub fn get_Reason(&self) -> windows_core::Result<AutoSuggestionBoxTextChangeReason> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Reason)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IAutoSuggestBoxTextChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Reason: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AutoSuggestionBoxTextChangeReason,
    ) -> windows_core::HRESULT,
    put_Reason: usize,
    CheckCurrent: usize,
}
windows_core::imp::define_interface!(
    IBitmapImage,
    IBitmapImage_Vtbl,
    0x31af3271_e3b4_442d_a341_4c0226b2725b
);
impl windows_core::RuntimeType for IBitmapImage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IBitmapImage {
    pub fn put_UriSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_UriSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IBitmapImage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_CreateOptions: usize,
    put_CreateOptions: usize,
    get_UriSource: usize,
    pub put_UriSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_DecodePixelWidth: usize,
    put_DecodePixelWidth: usize,
    get_DecodePixelHeight: usize,
    put_DecodePixelHeight: usize,
    add_DownloadProgress: usize,
    remove_DownloadProgress: usize,
    add_ImageOpened: usize,
    remove_ImageOpened: usize,
    add_ImageFailed: usize,
    remove_ImageFailed: usize,
}
windows_core::imp::define_interface!(
    IBitmapSource,
    IBitmapSource_Vtbl,
    0x23d86411_202f_41b2_8c5b_a8a3b333800b
);
impl windows_core::RuntimeType for IBitmapSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBitmapSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PixelWidth: usize,
    get_PixelHeight: usize,
    SetSource: usize,
    SetSourceAsync: usize,
}
windows_core::imp::define_interface!(IBlock, IBlock_Vtbl, 0x4bce0016_dd47_4350_8cb0_e171600ac896);
impl windows_core::RuntimeType for IBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBlock_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_TextAlignment: usize,
    put_TextAlignment: usize,
    get_LineHeight: usize,
    put_LineHeight: usize,
    get_LineStackingStrategy: usize,
    put_LineStackingStrategy: usize,
    get_Margin: usize,
    put_Margin: usize,
}
windows_core::imp::define_interface!(
    IBorder,
    IBorder_Vtbl,
    0x797c4539_45bd_4633_a044_bfb02ef5170f
);
impl windows_core::RuntimeType for IBorder {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IBorder {
    pub fn put_BorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_BorderBrush)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_BorderThickness(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_BorderThickness)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Background<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Background)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_CornerRadius(&self, value: CornerRadius) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_CornerRadius)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Padding(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Padding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Child<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Child)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IBorder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_BorderBrush: usize,
    pub put_BorderBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_BorderThickness: usize,
    pub put_BorderThickness:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    get_Background: usize,
    pub put_Background: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_CornerRadius: usize,
    pub put_CornerRadius:
        unsafe extern "system" fn(*mut core::ffi::c_void, CornerRadius) -> windows_core::HRESULT,
    get_Padding: usize,
    pub put_Padding:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    get_Child: usize,
    pub put_Child: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ChildTransitions: usize,
    put_ChildTransitions: usize,
}
windows_core::imp::define_interface!(IBrush, IBrush_Vtbl, 0x8806a321_1e06_422c_a1cc_01696559e021);
impl windows_core::RuntimeType for IBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Opacity: usize,
    put_Opacity: usize,
    get_Transform: usize,
    put_Transform: usize,
    get_RelativeTransform: usize,
    put_RelativeTransform: usize,
}
windows_core::imp::define_interface!(
    IButton,
    IButton_Vtbl,
    0x280335ae_5570_46c7_8e0b_602be71229a2
);
impl windows_core::RuntimeType for IButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IButtonBase,
    IButtonBase_Vtbl,
    0xfa002c1a_494e_46cf_91d4_e14a8d798674
);
impl windows_core::RuntimeType for IButtonBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IButtonBase {
    pub fn add_Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Click)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Click,
            ))
        }
    }
}
#[repr(C)]
pub struct IButtonBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ClickMode: usize,
    put_ClickMode: usize,
    get_IsPointerOver: usize,
    get_IsPressed: usize,
    get_Command: usize,
    put_Command: usize,
    get_CommandParameter: usize,
    put_CommandParameter: usize,
    pub add_Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Click:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IButtonFactory,
    IButtonFactory_Vtbl,
    0x80a13c19_843a_451c_8cf5_44c701b0e216
);
impl windows_core::RuntimeType for IButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICachedFileUpdaterActivatedEventArgs,
    ICachedFileUpdaterActivatedEventArgs_Vtbl,
    0xd06eb1c7_3805_4ecb_b757_6cf15e26fef3
);
impl windows_core::RuntimeType for ICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ICachedFileUpdaterActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ICachedFileUpdaterActivatedEventArgs, IActivatedEventArgs);
#[repr(C)]
pub struct ICachedFileUpdaterActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_CachedFileUpdaterUI: usize,
}
windows_core::imp::define_interface!(
    ICalendarDatePicker,
    ICalendarDatePicker_Vtbl,
    0x63c9c16f_668c_4491_9444_d45d8bf4fa29
);
impl windows_core::RuntimeType for ICalendarDatePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICalendarDatePicker {
    pub fn put_IsCalendarOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsCalendarOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsTodayHighlighted(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsTodayHighlighted)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICalendarDatePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Date: usize,
    put_Date: usize,
    get_IsCalendarOpen: usize,
    pub put_IsCalendarOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_DateFormat: usize,
    put_DateFormat: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_CalendarViewStyle: usize,
    put_CalendarViewStyle: usize,
    get_MinDate: usize,
    put_MinDate: usize,
    get_MaxDate: usize,
    put_MaxDate: usize,
    get_IsTodayHighlighted: usize,
    pub put_IsTodayHighlighted:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_DisplayMode: usize,
    put_DisplayMode: usize,
    get_FirstDayOfWeek: usize,
    put_FirstDayOfWeek: usize,
    get_DayOfWeekFormat: usize,
    put_DayOfWeekFormat: usize,
    get_CalendarIdentifier: usize,
    put_CalendarIdentifier: usize,
    get_IsOutOfScopeEnabled: usize,
    put_IsOutOfScopeEnabled: usize,
    get_IsGroupLabelVisible: usize,
    put_IsGroupLabelVisible: usize,
    add_CalendarViewDayItemChanging: usize,
    remove_CalendarViewDayItemChanging: usize,
    add_DateChanged: usize,
    remove_DateChanged: usize,
    add_Opened: usize,
    remove_Opened: usize,
    add_Closed: usize,
    remove_Closed: usize,
    SetDisplayDate: usize,
    SetYearDecadeDisplayDimensions: usize,
}
windows_core::imp::define_interface!(
    ICalendarDatePickerFactory,
    ICalendarDatePickerFactory_Vtbl,
    0x107aad5d_38be_42af_a957_fc86a5cf1e9a
);
impl windows_core::RuntimeType for ICalendarDatePickerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICalendarDatePickerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICalendarView,
    ICalendarView_Vtbl,
    0xcd639203_dfb5_4312_ac07_c0391824607b
);
impl windows_core::RuntimeType for ICalendarView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICalendarView {
    pub fn put_IsGroupLabelVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsGroupLabelVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsTodayHighlighted(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsTodayHighlighted)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_SelectedDatesChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<CalendarView>,
                windows_core::Ref<CalendarViewSelectedDatesChangedEventArgs>,
            ) + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            CalendarView,
            CalendarViewSelectedDatesChangedEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectedDatesChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectedDatesChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ICalendarView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_CalendarIdentifier: usize,
    put_CalendarIdentifier: usize,
    get_DayOfWeekFormat: usize,
    put_DayOfWeekFormat: usize,
    get_IsGroupLabelVisible: usize,
    pub put_IsGroupLabelVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_DisplayMode: usize,
    put_DisplayMode: usize,
    get_FirstDayOfWeek: usize,
    put_FirstDayOfWeek: usize,
    get_IsOutOfScopeEnabled: usize,
    put_IsOutOfScopeEnabled: usize,
    get_IsTodayHighlighted: usize,
    pub put_IsTodayHighlighted:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_MaxDate: usize,
    put_MaxDate: usize,
    get_MinDate: usize,
    put_MinDate: usize,
    get_NumberOfWeeksInView: usize,
    put_NumberOfWeeksInView: usize,
    get_SelectedDates: usize,
    get_SelectionMode: usize,
    put_SelectionMode: usize,
    get_TemplateSettings: usize,
    get_FocusBorderBrush: usize,
    put_FocusBorderBrush: usize,
    get_SelectedHoverBorderBrush: usize,
    put_SelectedHoverBorderBrush: usize,
    get_SelectedPressedBorderBrush: usize,
    put_SelectedPressedBorderBrush: usize,
    get_SelectedBorderBrush: usize,
    put_SelectedBorderBrush: usize,
    get_HoverBorderBrush: usize,
    put_HoverBorderBrush: usize,
    get_PressedBorderBrush: usize,
    put_PressedBorderBrush: usize,
    get_CalendarItemBorderBrush: usize,
    put_CalendarItemBorderBrush: usize,
    get_OutOfScopeBackground: usize,
    put_OutOfScopeBackground: usize,
    get_CalendarItemBackground: usize,
    put_CalendarItemBackground: usize,
    get_PressedForeground: usize,
    put_PressedForeground: usize,
    get_TodayForeground: usize,
    put_TodayForeground: usize,
    get_BlackoutForeground: usize,
    put_BlackoutForeground: usize,
    get_SelectedForeground: usize,
    put_SelectedForeground: usize,
    get_OutOfScopeForeground: usize,
    put_OutOfScopeForeground: usize,
    get_CalendarItemForeground: usize,
    put_CalendarItemForeground: usize,
    get_DayItemFontFamily: usize,
    put_DayItemFontFamily: usize,
    get_DayItemFontSize: usize,
    put_DayItemFontSize: usize,
    get_DayItemFontStyle: usize,
    put_DayItemFontStyle: usize,
    get_DayItemFontWeight: usize,
    put_DayItemFontWeight: usize,
    get_TodayFontWeight: usize,
    put_TodayFontWeight: usize,
    get_FirstOfMonthLabelFontFamily: usize,
    put_FirstOfMonthLabelFontFamily: usize,
    get_FirstOfMonthLabelFontSize: usize,
    put_FirstOfMonthLabelFontSize: usize,
    get_FirstOfMonthLabelFontStyle: usize,
    put_FirstOfMonthLabelFontStyle: usize,
    get_FirstOfMonthLabelFontWeight: usize,
    put_FirstOfMonthLabelFontWeight: usize,
    get_MonthYearItemFontFamily: usize,
    put_MonthYearItemFontFamily: usize,
    get_MonthYearItemFontSize: usize,
    put_MonthYearItemFontSize: usize,
    get_MonthYearItemFontStyle: usize,
    put_MonthYearItemFontStyle: usize,
    get_MonthYearItemFontWeight: usize,
    put_MonthYearItemFontWeight: usize,
    get_FirstOfYearDecadeLabelFontFamily: usize,
    put_FirstOfYearDecadeLabelFontFamily: usize,
    get_FirstOfYearDecadeLabelFontSize: usize,
    put_FirstOfYearDecadeLabelFontSize: usize,
    get_FirstOfYearDecadeLabelFontStyle: usize,
    put_FirstOfYearDecadeLabelFontStyle: usize,
    get_FirstOfYearDecadeLabelFontWeight: usize,
    put_FirstOfYearDecadeLabelFontWeight: usize,
    get_HorizontalDayItemAlignment: usize,
    put_HorizontalDayItemAlignment: usize,
    get_VerticalDayItemAlignment: usize,
    put_VerticalDayItemAlignment: usize,
    get_HorizontalFirstOfMonthLabelAlignment: usize,
    put_HorizontalFirstOfMonthLabelAlignment: usize,
    get_VerticalFirstOfMonthLabelAlignment: usize,
    put_VerticalFirstOfMonthLabelAlignment: usize,
    get_CalendarItemBorderThickness: usize,
    put_CalendarItemBorderThickness: usize,
    get_CalendarViewDayItemStyle: usize,
    put_CalendarViewDayItemStyle: usize,
    add_CalendarViewDayItemChanging: usize,
    remove_CalendarViewDayItemChanging: usize,
    pub add_SelectedDatesChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectedDatesChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    SetDisplayDate: usize,
    SetYearDecadeDisplayDimensions: usize,
}
windows_core::imp::define_interface!(
    ICalendarViewFactory,
    ICalendarViewFactory_Vtbl,
    0x3d8f82e3_6cc6_423e_8d7c_7014d954ddef
);
impl windows_core::RuntimeType for ICalendarViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICalendarViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICalendarViewSelectedDatesChangedEventArgs,
    ICalendarViewSelectedDatesChangedEventArgs_Vtbl,
    0xee6069f6_13ef_4896_8ffc_5302b1b17539
);
impl windows_core::RuntimeType for ICalendarViewSelectedDatesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICalendarViewSelectedDatesChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_AddedDates: usize,
    get_RemovedDates: usize,
}
windows_core::imp::define_interface!(
    ICanvas,
    ICanvas_Vtbl,
    0x79190e19_cd38_4823_aeae_64a77132f519
);
impl windows_core::RuntimeType for ICanvas {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICanvas_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICanvasFactory,
    ICanvasFactory_Vtbl,
    0x1b328bd1_b400_4a8e_943b_5ad2c45be0df
);
impl windows_core::RuntimeType for ICanvasFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICanvasFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICanvasStatics,
    ICanvasStatics_Vtbl,
    0x40ce5c46_2962_446f_aafb_4cdc486939c9
);
impl windows_core::RuntimeType for ICanvasStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICanvasStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_LeftProperty: usize,
    GetLeft: usize,
    pub SetLeft: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        f64,
    ) -> windows_core::HRESULT,
    get_TopProperty: usize,
    GetTop: usize,
    pub SetTop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        f64,
    ) -> windows_core::HRESULT,
    get_ZIndexProperty: usize,
    GetZIndex: usize,
    pub SetZIndex: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICheckBox,
    ICheckBox_Vtbl,
    0x2294c894_7e2a_4b70_b088_8f5d814875ba
);
impl windows_core::RuntimeType for ICheckBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICheckBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICheckBoxFactory,
    ICheckBoxFactory_Vtbl,
    0x4fa6aabb_3f4b_4301_be07_1172ea61eefb
);
impl windows_core::RuntimeType for ICheckBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICheckBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IColorChangedEventArgs,
    IColorChangedEventArgs_Vtbl,
    0x34f7201f_aad0_4c3a_b97b_2abf36455539
);
impl windows_core::RuntimeType for IColorChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IColorChangedEventArgs {
    pub fn get_NewColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IColorChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OldColor: usize,
    pub get_NewColor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IColorPicker,
    IColorPicker_Vtbl,
    0x6232e371_5c64_43cb_8b35_7f82dde36740
);
impl windows_core::RuntimeType for IColorPicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IColorPicker {
    pub fn put_Color(&self, value: Color) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Color)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsAlphaEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsAlphaEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsColorSliderVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsColorSliderVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsColorChannelTextInputVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsColorChannelTextInputVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsHexInputVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsHexInputVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_ColorChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<ColorPicker>, windows_core::Ref<ColorChangedEventArgs>)
            + Send
            + 'static,
    {
        let handler =
            <windows::Foundation::TypedEventHandler<ColorPicker, ColorChangedEventArgs>>::new(
                move |a0, a1| {
                    handler(a0, a1);
                    Ok(())
                },
            );
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ColorChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ColorChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IColorPicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Color: usize,
    pub put_Color:
        unsafe extern "system" fn(*mut core::ffi::c_void, Color) -> windows_core::HRESULT,
    get_PreviousColor: usize,
    put_PreviousColor: usize,
    get_IsAlphaEnabled: usize,
    pub put_IsAlphaEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsColorSpectrumVisible: usize,
    put_IsColorSpectrumVisible: usize,
    get_IsColorPreviewVisible: usize,
    put_IsColorPreviewVisible: usize,
    get_IsColorSliderVisible: usize,
    pub put_IsColorSliderVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsAlphaSliderVisible: usize,
    put_IsAlphaSliderVisible: usize,
    get_IsMoreButtonVisible: usize,
    put_IsMoreButtonVisible: usize,
    get_IsColorChannelTextInputVisible: usize,
    pub put_IsColorChannelTextInputVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsAlphaTextInputVisible: usize,
    put_IsAlphaTextInputVisible: usize,
    get_IsHexInputVisible: usize,
    pub put_IsHexInputVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_MinHue: usize,
    put_MinHue: usize,
    get_MaxHue: usize,
    put_MaxHue: usize,
    get_MinSaturation: usize,
    put_MinSaturation: usize,
    get_MaxSaturation: usize,
    put_MaxSaturation: usize,
    get_MinValue: usize,
    put_MinValue: usize,
    get_MaxValue: usize,
    put_MaxValue: usize,
    get_ColorSpectrumShape: usize,
    put_ColorSpectrumShape: usize,
    get_ColorSpectrumComponents: usize,
    put_ColorSpectrumComponents: usize,
    pub add_ColorChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ColorChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IColorPickerFactory,
    IColorPickerFactory_Vtbl,
    0xabae07ff_aecf_481d_9204_201c3894cd1b
);
impl windows_core::RuntimeType for IColorPickerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IColorPickerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IColumnDefinition,
    IColumnDefinition_Vtbl,
    0xf7f1b229_f024_467f_970a_7e705615db7b
);
impl windows_core::RuntimeType for IColumnDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IColumnDefinition {
    pub fn put_Width(&self, value: GridLength) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Width)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IColumnDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Width: usize,
    pub put_Width:
        unsafe extern "system" fn(*mut core::ffi::c_void, GridLength) -> windows_core::HRESULT,
    get_MaxWidth: usize,
    put_MaxWidth: usize,
    get_MinWidth: usize,
    put_MinWidth: usize,
    get_ActualWidth: usize,
}
windows_core::imp::define_interface!(
    IComboBox,
    IComboBox_Vtbl,
    0xb9a8d05c_ac97_47f1_a5f4_3f9f4d4b116c
);
impl windows_core::RuntimeType for IComboBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IComboBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsDropDownOpen: usize,
    put_IsDropDownOpen: usize,
    get_IsEditable: usize,
    get_IsSelectionBoxHighlighted: usize,
    get_MaxDropDownHeight: usize,
    put_MaxDropDownHeight: usize,
    get_SelectionBoxItem: usize,
    get_SelectionBoxItemTemplate: usize,
    get_TemplateSettings: usize,
    add_DropDownClosed: usize,
    remove_DropDownClosed: usize,
    add_DropDownOpened: usize,
    remove_DropDownOpened: usize,
}
windows_core::imp::define_interface!(
    IComboBox2,
    IComboBox2_Vtbl,
    0xea0cbf91_ca36_4fad_972a_2e53a6718b9f
);
impl windows_core::RuntimeType for IComboBox2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IComboBox2 {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IComboBox2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IComboBox6,
    IComboBox6_Vtbl,
    0x61aad6a5_81fb_5f87_bae3_369fbe2ea1f3
);
impl windows_core::RuntimeType for IComboBox6 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IComboBox6 {
    pub fn put_IsEditable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsEditable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IComboBox6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub put_IsEditable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_Text: usize,
    put_Text: usize,
    get_TextBoxStyle: usize,
    put_TextBoxStyle: usize,
    get_Description: usize,
    put_Description: usize,
    add_TextSubmitted: usize,
    remove_TextSubmitted: usize,
}
windows_core::imp::define_interface!(
    IComboBoxFactory,
    IComboBoxFactory_Vtbl,
    0xac0d0444_a65b_4abd_86df_3016049efedc
);
impl windows_core::RuntimeType for IComboBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IComboBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICommandBar,
    ICommandBar_Vtbl,
    0x98bc4280_4a3d_4cee_bd07_22ce94c5af76
);
impl windows_core::RuntimeType for ICommandBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommandBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PrimaryCommands: usize,
    get_SecondaryCommands: usize,
}
windows_core::imp::define_interface!(
    ICommandBar3,
    ICommandBar3_Vtbl,
    0x40ebbc23_2a79_48b3_9a67_649b852d8589
);
impl windows_core::RuntimeType for ICommandBar3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICommandBar3 {
    pub fn put_DefaultLabelPosition(
        &self,
        value: CommandBarDefaultLabelPosition,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_DefaultLabelPosition)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICommandBar3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_DefaultLabelPosition: usize,
    pub put_DefaultLabelPosition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        CommandBarDefaultLabelPosition,
    ) -> windows_core::HRESULT,
    get_OverflowButtonVisibility: usize,
    put_OverflowButtonVisibility: usize,
    get_IsDynamicOverflowEnabled: usize,
    put_IsDynamicOverflowEnabled: usize,
    add_DynamicOverflowItemsChanging: usize,
    remove_DynamicOverflowItemsChanging: usize,
}
windows_core::imp::define_interface!(
    ICommandBarFactory,
    ICommandBarFactory_Vtbl,
    0x67bdeb44_20e1_4177_ad44_f617b374e8e8
);
impl windows_core::RuntimeType for ICommandBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommandBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionTarget,
    ICompositionTarget_Vtbl,
    0x26cfbff0_713c_4bec_8803_e101f7b14ed3
);
impl windows_core::RuntimeType for ICompositionTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionTargetStatics,
    ICompositionTargetStatics_Vtbl,
    0x2b1af03d_1ed2_4b59_bd00_7594ee92832b
);
impl windows_core::RuntimeType for ICompositionTargetStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionTargetStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub add_Rendering: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Rendering:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_SurfaceContentsLost: usize,
    remove_SurfaceContentsLost: usize,
}
windows_core::imp::define_interface!(
    IContentControl,
    IContentControl_Vtbl,
    0xa26dd1dc_cd44_435c_be94_01d6241c231c
);
impl windows_core::RuntimeType for IContentControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContentControl {
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IContentControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Content: usize,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ContentTemplate: usize,
    put_ContentTemplate: usize,
    get_ContentTemplateSelector: usize,
    put_ContentTemplateSelector: usize,
    get_ContentTransitions: usize,
    put_ContentTransitions: usize,
}
windows_core::imp::define_interface!(
    IContentDialog,
    IContentDialog_Vtbl,
    0x38dc4404_d24e_40d8_9415_349464c1afdc
);
impl windows_core::RuntimeType for IContentDialog {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContentDialog {
    pub fn put_Title<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_PrimaryButtonText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PrimaryButtonText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_SecondaryButtonText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SecondaryButtonText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_IsPrimaryButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPrimaryButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsSecondaryButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsSecondaryButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IContentDialog_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TitleTemplate: usize,
    put_TitleTemplate: usize,
    get_FullSizeDesired: usize,
    put_FullSizeDesired: usize,
    get_PrimaryButtonText: usize,
    pub put_PrimaryButtonText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SecondaryButtonText: usize,
    pub put_SecondaryButtonText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_PrimaryButtonCommand: usize,
    put_PrimaryButtonCommand: usize,
    get_SecondaryButtonCommand: usize,
    put_SecondaryButtonCommand: usize,
    get_PrimaryButtonCommandParameter: usize,
    put_PrimaryButtonCommandParameter: usize,
    get_SecondaryButtonCommandParameter: usize,
    put_SecondaryButtonCommandParameter: usize,
    get_IsPrimaryButtonEnabled: usize,
    pub put_IsPrimaryButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsSecondaryButtonEnabled: usize,
    pub put_IsSecondaryButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    add_Closing: usize,
    remove_Closing: usize,
    add_Closed: usize,
    remove_Closed: usize,
    add_Opened: usize,
    remove_Opened: usize,
    add_PrimaryButtonClick: usize,
    remove_PrimaryButtonClick: usize,
    add_SecondaryButtonClick: usize,
    remove_SecondaryButtonClick: usize,
    Hide: usize,
    ShowAsync: usize,
}
windows_core::imp::define_interface!(
    IContentDialog2,
    IContentDialog2_Vtbl,
    0x2f93eb45_ee43_4303_9b38_3fe1a111ecbf
);
impl windows_core::RuntimeType for IContentDialog2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContentDialog2 {
    pub fn put_CloseButtonText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_CloseButtonText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IContentDialog2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_CloseButtonText: usize,
    pub put_CloseButtonText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_CloseButtonCommand: usize,
    put_CloseButtonCommand: usize,
    get_CloseButtonCommandParameter: usize,
    put_CloseButtonCommandParameter: usize,
    get_PrimaryButtonStyle: usize,
    put_PrimaryButtonStyle: usize,
    get_SecondaryButtonStyle: usize,
    put_SecondaryButtonStyle: usize,
    get_CloseButtonStyle: usize,
    put_CloseButtonStyle: usize,
    get_DefaultButton: usize,
    put_DefaultButton: usize,
    add_CloseButtonClick: usize,
    remove_CloseButtonClick: usize,
}
windows_core::imp::define_interface!(
    IContentDialogFactory,
    IContentDialogFactory_Vtbl,
    0x05557178_9d8e_4315_b37d_680c14012c35
);
impl windows_core::RuntimeType for IContentDialogFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContentDialogFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControl,
    IControl_Vtbl,
    0xa8912263_2951_4f58_a9c5_5a134eaa7f07
);
impl windows_core::RuntimeType for IControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IControl {
    pub fn put_FontSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_FontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FontFamily>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontFamily)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_FontWeight(&self, value: FontWeight) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontWeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Foreground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Foreground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Padding(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Padding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Background<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Background)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_FontSize: usize,
    pub put_FontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_FontFamily: usize,
    pub put_FontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FontWeight: usize,
    pub put_FontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FontWeight) -> windows_core::HRESULT,
    get_FontStyle: usize,
    put_FontStyle: usize,
    get_FontStretch: usize,
    put_FontStretch: usize,
    get_CharacterSpacing: usize,
    put_CharacterSpacing: usize,
    get_Foreground: usize,
    pub put_Foreground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsTabStop: usize,
    put_IsTabStop: usize,
    get_IsEnabled: usize,
    pub put_IsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_TabIndex: usize,
    put_TabIndex: usize,
    get_TabNavigation: usize,
    put_TabNavigation: usize,
    get_Template: usize,
    put_Template: usize,
    get_Padding: usize,
    pub put_Padding:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    get_HorizontalContentAlignment: usize,
    put_HorizontalContentAlignment: usize,
    get_VerticalContentAlignment: usize,
    put_VerticalContentAlignment: usize,
    get_Background: usize,
    pub put_Background: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_BorderThickness: usize,
    put_BorderThickness: usize,
    get_BorderBrush: usize,
    put_BorderBrush: usize,
    get_FocusState: usize,
    add_IsEnabledChanged: usize,
    remove_IsEnabledChanged: usize,
    ApplyTemplate: usize,
    Focus: usize,
}
windows_core::imp::define_interface!(
    IDataTemplate,
    IDataTemplate_Vtbl,
    0x9910aec7_8ab5_4118_9bc6_09f45a35073d
);
impl windows_core::RuntimeType for IDataTemplate {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDataTemplate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    LoadContent: usize,
}
windows_core::imp::define_interface!(
    IDataTemplateFactory,
    IDataTemplateFactory_Vtbl,
    0x51ed9d7e_2b53_475b_9c88_0c1832c8351a
);
impl windows_core::RuntimeType for IDataTemplateFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDataTemplateFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDatePicker,
    IDatePicker_Vtbl,
    0x06da3946_08b8_4103_8b8a_093efd6a7657
);
impl windows_core::RuntimeType for IDatePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDatePicker {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_DayVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_DayVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MonthVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MonthVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_YearVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_YearVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_DateChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<DatePickerValueChangedEventArgs>,
            ) + Send
            + 'static,
    {
        let handler = <windows::Foundation::EventHandler<DatePickerValueChangedEventArgs>>::new(
            move |a0, a1| {
                handler(a0, a1);
                Ok(())
            },
        );
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_DateChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_DateChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IDatePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_CalendarIdentifier: usize,
    put_CalendarIdentifier: usize,
    get_Date: usize,
    put_Date: usize,
    get_DayVisible: usize,
    pub put_DayVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_MonthVisible: usize,
    pub put_MonthVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_YearVisible: usize,
    pub put_YearVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_DayFormat: usize,
    put_DayFormat: usize,
    get_MonthFormat: usize,
    put_MonthFormat: usize,
    get_YearFormat: usize,
    put_YearFormat: usize,
    get_MinYear: usize,
    put_MinYear: usize,
    get_MaxYear: usize,
    put_MaxYear: usize,
    get_Orientation: usize,
    put_Orientation: usize,
    pub add_DateChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_DateChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDatePickerFactory,
    IDatePickerFactory_Vtbl,
    0xeec3ca84_9896_4a7d_bb35_6fb21eaeca11
);
impl windows_core::RuntimeType for IDatePickerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDatePickerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDatePickerValueChangedEventArgs,
    IDatePickerValueChangedEventArgs_Vtbl,
    0x1ae661b2_b1b4_4273_96e0_19daff187446
);
impl windows_core::RuntimeType for IDatePickerValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDatePickerValueChangedEventArgs {
    pub fn get_NewDate(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewDate)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDatePickerValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OldDate: usize,
    pub get_NewDate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::DateTime,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDependencyObject,
    IDependencyObject_Vtbl,
    0x5c526665_f60e_4912_af59_5fe0680f089d
);
impl windows_core::RuntimeType for IDependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDependencyObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    GetValue: usize,
    SetValue: usize,
    ClearValue: usize,
    ReadLocalValue: usize,
    GetAnimationBaseValue: usize,
    get_Dispatcher: usize,
}
windows_core::imp::define_interface!(
    IDesktopWindowXamlSource,
    IDesktopWindowXamlSource_Vtbl,
    0xd585bfe1_00ff_51be_ba1d_a1329956ea0a
);
impl windows_core::RuntimeType for IDesktopWindowXamlSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDesktopWindowXamlSource {
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IDesktopWindowXamlSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Content: usize,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HasFocus: usize,
    add_TakeFocusRequested: usize,
    remove_TakeFocusRequested: usize,
    add_GotFocus: usize,
    remove_GotFocus: usize,
    NavigateFocus: usize,
}
windows_core::imp::define_interface!(
    IDesktopWindowXamlSourceFactory,
    IDesktopWindowXamlSourceFactory_Vtbl,
    0x5cd61dc0_2561_56e1_8e75_6e44173805e3
);
impl windows_core::RuntimeType for IDesktopWindowXamlSourceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDesktopWindowXamlSourceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDesktopWindowXamlSourceNative,
    IDesktopWindowXamlSourceNative_Vtbl,
    0x3cbcf1bf_2f76_4e9c_96ab_e84b37972554
);
windows_core::imp::interface_hierarchy!(IDesktopWindowXamlSourceNative, windows_core::IUnknown);
impl IDesktopWindowXamlSourceNative {
    pub unsafe fn AttachToWindow(&self, parentwnd: HWND) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).AttachToWindow)(
                windows_core::Interface::as_raw(self),
                parentwnd,
            )
            .ok()
        }
    }
    pub unsafe fn get_WindowHandle(&self) -> windows_core::Result<HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WindowHandle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDesktopWindowXamlSourceNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AttachToWindow:
        unsafe extern "system" fn(*mut core::ffi::c_void, HWND) -> windows_core::HRESULT,
    pub get_WindowHandle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut HWND) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherQueue,
    IDispatcherQueue_Vtbl,
    0x603e88e4_a338_4ffe_a457_a5cfb9ceb899
);
impl windows_core::RuntimeType for IDispatcherQueue {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDispatcherQueue {
    pub fn TryEnqueue<P0>(&self, callback: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<DispatcherQueueHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryEnqueue)(
                windows_core::Interface::as_raw(self),
                callback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn TryEnqueueWithPriority<P1>(
        &self,
        priority: DispatcherQueuePriority,
        callback: P1,
    ) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<DispatcherQueueHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryEnqueueWithPriority)(
                windows_core::Interface::as_raw(self),
                priority,
                callback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDispatcherQueue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CreateTimer: usize,
    pub TryEnqueue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub TryEnqueueWithPriority: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DispatcherQueuePriority,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    add_ShutdownStarting: usize,
    remove_ShutdownStarting: usize,
    add_ShutdownCompleted: usize,
    remove_ShutdownCompleted: usize,
}
windows_core::imp::define_interface!(
    IDispatcherQueueStatics,
    IDispatcherQueueStatics_Vtbl,
    0xa96d83d7_9371_4517_9245_d0824ac12c74
);
impl windows_core::RuntimeType for IDispatcherQueueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDispatcherQueueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentThread: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherTimer,
    IDispatcherTimer_Vtbl,
    0xd160ce46_cd22_4f5f_8c97_40e61da3e2dc
);
impl windows_core::RuntimeType for IDispatcherTimer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDispatcherTimer {
    pub fn put_Interval(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Interval)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_Tick<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<windows_core::IInspectable>,
            ) + Send
            + 'static,
    {
        let handler =
            <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| {
                handler(a0, a1);
                Ok(())
            });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Tick)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Tick,
            ))
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
        }
    }
}
#[repr(C)]
pub struct IDispatcherTimer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Interval: usize,
    pub put_Interval: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    get_IsEnabled: usize,
    pub add_Tick: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Tick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherTimerFactory,
    IDispatcherTimerFactory_Vtbl,
    0xe9961e6e_3626_403a_afe0_040d58165632
);
impl windows_core::RuntimeType for IDispatcherTimerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDispatcherTimerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDropDownButton,
    IDropDownButton_Vtbl,
    0x671f74e6_2a27_5fa8_b0a2_79b2e71ebd87
);
impl windows_core::RuntimeType for IDropDownButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDropDownButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IDropDownButtonFactory,
    IDropDownButtonFactory_Vtbl,
    0x0d9f8ab5_e70d_52bd_9ca0_36ceecaa642a
);
impl windows_core::RuntimeType for IDropDownButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDropDownButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IEllipse,
    IEllipse_Vtbl,
    0x70e05ac4_d38d_4bab_831f_4a22ef52ac86
);
impl windows_core::RuntimeType for IEllipse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEllipse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IFileActivatedEventArgs,
    IFileActivatedEventArgs_Vtbl,
    0xbb2afc33_93b1_42ed_8b26_236dd9c78496
);
impl windows_core::RuntimeType for IFileActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IFileActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IFileActivatedEventArgs, IActivatedEventArgs);
#[repr(C)]
pub struct IFileActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Files: usize,
    get_Verb: usize,
}
windows_core::imp::define_interface!(
    IFileOpenPickerActivatedEventArgs,
    IFileOpenPickerActivatedEventArgs_Vtbl,
    0x72827082_5525_4bf2_bc09_1f5095d4964d
);
impl windows_core::RuntimeType for IFileOpenPickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IFileOpenPickerActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IFileOpenPickerActivatedEventArgs, IActivatedEventArgs);
#[repr(C)]
pub struct IFileOpenPickerActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_FileOpenPickerUI: usize,
}
windows_core::imp::define_interface!(
    IFileSavePickerActivatedEventArgs,
    IFileSavePickerActivatedEventArgs_Vtbl,
    0x81c19cf1_74e6_4387_82eb_bb8fd64b4346
);
impl windows_core::RuntimeType for IFileSavePickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IFileSavePickerActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IFileSavePickerActivatedEventArgs, IActivatedEventArgs);
#[repr(C)]
pub struct IFileSavePickerActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_FileSavePickerUI: usize,
}
windows_core::imp::define_interface!(
    IFlipView,
    IFlipView_Vtbl,
    0xa1582f68_3d7d_4d3b_b71d_488eed1e3493
);
impl windows_core::RuntimeType for IFlipView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFlipView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IFlipViewFactory,
    IFlipViewFactory_Vtbl,
    0xf1dea9be_9ae8_4d4b_ab43_16d31e05f4f3
);
impl windows_core::RuntimeType for IFlipViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFlipViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFontFamily,
    IFontFamily_Vtbl,
    0x92467e64_d66a_4cf4_9322_3d23b3c0c361
);
impl windows_core::RuntimeType for IFontFamily {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFontFamily_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Source: usize,
}
windows_core::imp::define_interface!(
    IFontFamilyFactory,
    IFontFamilyFactory_Vtbl,
    0xd5603377_3dae_4dcd_af09_f9498e9ec659
);
impl windows_core::RuntimeType for IFontFamilyFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFontFamilyFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElement,
    IFrameworkElement_Vtbl,
    0xa391d09b_4a99_4b7c_9d8d_6fa5d01f6fbf
);
impl windows_core::RuntimeType for IFrameworkElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFrameworkElement {
    pub fn get_Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Tag)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Tag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Tag)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Width(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Width)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Height(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Height)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MinWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MinWidth)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MaxWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MaxWidth)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MinHeight(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MinHeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MaxHeight(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MaxHeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_HorizontalAlignment(&self, value: HorizontalAlignment) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_HorizontalAlignment)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_VerticalAlignment(&self, value: VerticalAlignment) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_VerticalAlignment)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Margin(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Margin)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Style<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Style>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Style)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFrameworkElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Triggers: usize,
    get_Resources: usize,
    put_Resources: usize,
    pub get_Tag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Tag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Language: usize,
    put_Language: usize,
    get_ActualWidth: usize,
    get_ActualHeight: usize,
    get_Width: usize,
    pub put_Width: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Height: usize,
    pub put_Height: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_MinWidth: usize,
    pub put_MinWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_MaxWidth: usize,
    pub put_MaxWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_MinHeight: usize,
    pub put_MinHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_MaxHeight: usize,
    pub put_MaxHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_HorizontalAlignment: usize,
    pub put_HorizontalAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HorizontalAlignment,
    ) -> windows_core::HRESULT,
    get_VerticalAlignment: usize,
    pub put_VerticalAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VerticalAlignment,
    ) -> windows_core::HRESULT,
    get_Margin: usize,
    pub put_Margin:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    get_Name: usize,
    put_Name: usize,
    get_BaseUri: usize,
    get_DataContext: usize,
    put_DataContext: usize,
    get_Style: usize,
    pub put_Style: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Parent: usize,
    get_FlowDirection: usize,
    put_FlowDirection: usize,
    add_Loaded: usize,
    remove_Loaded: usize,
    add_Unloaded: usize,
    remove_Unloaded: usize,
    add_SizeChanged: usize,
    remove_SizeChanged: usize,
    add_LayoutUpdated: usize,
    remove_LayoutUpdated: usize,
    FindName: usize,
    SetBinding: usize,
}
windows_core::imp::define_interface!(
    IFrameworkElement2,
    IFrameworkElement2_Vtbl,
    0xf19104be_422a_4904_a52f_ee72010429e5
);
impl windows_core::RuntimeType for IFrameworkElement2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFrameworkElement2 {
    pub fn put_RequestedTheme(&self, value: ElementTheme) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_RequestedTheme)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFrameworkElement2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_RequestedTheme: usize,
    pub put_RequestedTheme:
        unsafe extern "system" fn(*mut core::ffi::c_void, ElementTheme) -> windows_core::HRESULT,
    add_DataContextChanged: usize,
    remove_DataContextChanged: usize,
    GetBindingExpression: usize,
}
windows_core::imp::define_interface!(
    IFrameworkElement6,
    IFrameworkElement6_Vtbl,
    0x792a5d91_62a1_40bf_a0ce_f9c131fcb7a7
);
impl windows_core::RuntimeType for IFrameworkElement6 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFrameworkElement6 {
    pub fn get_ActualTheme(&self) -> windows_core::Result<ElementTheme> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ActualTheme)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn add_ActualThemeChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<FrameworkElement>, windows_core::Ref<windows_core::IInspectable>)
            + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            FrameworkElement,
            windows_core::IInspectable,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ActualThemeChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ActualThemeChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IFrameworkElement6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_ActualTheme: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ElementTheme,
    ) -> windows_core::HRESULT,
    pub add_ActualThemeChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ActualThemeChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkTemplate,
    IFrameworkTemplate_Vtbl,
    0xa1e254d8_a446_4a27_9a9d_a0f59e1258a5
);
impl windows_core::RuntimeType for IFrameworkTemplate {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameworkTemplate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IGrid, IGrid_Vtbl, 0xfd104460_2e15_4ba3_8b8f_fa693a4161e9);
impl windows_core::RuntimeType for IGrid {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IGrid {
    pub fn get_RowDefinitions(&self) -> windows_core::Result<RowDefinitionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RowDefinitions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_ColumnDefinitions(&self) -> windows_core::Result<ColumnDefinitionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ColumnDefinitions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IGrid_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_RowDefinitions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_ColumnDefinitions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGrid3, IGrid3_Vtbl, 0x12dfc5fc_2342_4dd2_9e7d_2090a171d1ef);
impl windows_core::RuntimeType for IGrid3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IGrid3 {
    pub fn put_RowSpacing(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_RowSpacing)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_ColumnSpacing(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_ColumnSpacing)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IGrid3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_RowSpacing: usize,
    pub put_RowSpacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_ColumnSpacing: usize,
    pub put_ColumnSpacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IGridFactory,
    IGridFactory_Vtbl,
    0xae814041_c531_43b4_bf99_12f506f7b01c
);
impl windows_core::RuntimeType for IGridFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGridFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IGridStatics,
    IGridStatics_Vtbl,
    0x64fe2e9f_f951_42b6_a9ce_bb179af11595
);
impl windows_core::RuntimeType for IGridStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGridStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_RowProperty: usize,
    GetRow: usize,
    pub SetRow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
    get_ColumnProperty: usize,
    GetColumn: usize,
    pub SetColumn: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
    get_RowSpanProperty: usize,
    GetRowSpan: usize,
    pub SetRowSpan: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
    get_ColumnSpanProperty: usize,
    GetColumnSpan: usize,
    pub SetColumnSpan: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IGridView,
    IGridView_Vtbl,
    0x026ae934_b67e_4d80_8f72_8aa64b4d827b
);
impl windows_core::RuntimeType for IGridView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGridView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IGridViewFactory,
    IGridViewFactory_Vtbl,
    0xd9bcca89_09f9_4c6e_a83e_f199146f0e7d
);
impl windows_core::RuntimeType for IGridViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGridViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IHyperlinkButton,
    IHyperlinkButton_Vtbl,
    0xccebaca3_3b5c_4f4c_9bfd_86887bc79772
);
impl windows_core::RuntimeType for IHyperlinkButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHyperlinkButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_NavigateUri: usize,
    put_NavigateUri: usize,
}
windows_core::imp::define_interface!(
    IHyperlinkButtonFactory,
    IHyperlinkButtonFactory_Vtbl,
    0x43521bad_4e97_4da9_a64d_935dfd8cedf2
);
impl windows_core::RuntimeType for IHyperlinkButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHyperlinkButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IIconElement,
    IIconElement_Vtbl,
    0x9af0803b_d04c_467a_bbd5_9b81f02d9a56
);
impl windows_core::RuntimeType for IIconElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIconElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Foreground: usize,
    put_Foreground: usize,
}
windows_core::imp::define_interface!(IImage, IImage_Vtbl, 0x495b7402_9af3_4e50_aa90_03388f3086d2);
impl windows_core::RuntimeType for IImage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IImage {
    pub fn put_Source<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ImageSource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Source)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Stretch(&self, value: Stretch) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Stretch)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IImage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Source: usize,
    pub put_Source: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Stretch: usize,
    pub put_Stretch:
        unsafe extern "system" fn(*mut core::ffi::c_void, Stretch) -> windows_core::HRESULT,
    get_NineGrid: usize,
    put_NineGrid: usize,
    get_PlayToSource: usize,
    add_ImageFailed: usize,
    remove_ImageFailed: usize,
    add_ImageOpened: usize,
    remove_ImageOpened: usize,
}
windows_core::imp::define_interface!(
    IImageSource,
    IImageSource_Vtbl,
    0x737ef309_ea41_4d96_a71c_98e98efcab07
);
impl windows_core::RuntimeType for IImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInline,
    IInline_Vtbl,
    0x0c92712d_1bc9_4931_8cb1_1aeadf1cc685
);
impl windows_core::RuntimeType for IInline {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInline_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IItemsControl,
    IItemsControl_Vtbl,
    0xf4a91dd8_d979_4381_8652_bda0342a765e
);
impl windows_core::RuntimeType for IItemsControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IItemsControl {
    pub fn put_ItemsSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_ItemsSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn get_Items(&self) -> windows_core::Result<ItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IItemsControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ItemsSource: usize,
    pub put_ItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ItemTemplate: usize,
    put_ItemTemplate: usize,
    get_ItemTemplateSelector: usize,
    put_ItemTemplateSelector: usize,
    get_ItemsPanel: usize,
    put_ItemsPanel: usize,
    get_DisplayMemberPath: usize,
    put_DisplayMemberPath: usize,
    get_ItemContainerStyle: usize,
    put_ItemContainerStyle: usize,
    get_ItemContainerStyleSelector: usize,
    put_ItemContainerStyleSelector: usize,
    get_ItemContainerGenerator: usize,
    get_ItemContainerTransitions: usize,
    put_ItemContainerTransitions: usize,
    get_GroupStyle: usize,
    get_GroupStyleSelector: usize,
    put_GroupStyleSelector: usize,
    get_IsGrouping: usize,
}
windows_core::imp::define_interface!(
    ILaunchActivatedEventArgs,
    ILaunchActivatedEventArgs_Vtbl,
    0xfbc93e26_a14a_4b4f_82b0_33bed920af52
);
impl windows_core::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ILaunchActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ILaunchActivatedEventArgs, IActivatedEventArgs);
#[repr(C)]
pub struct ILaunchActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Arguments: usize,
    get_TileId: usize,
}
windows_core::imp::define_interface!(ILine, ILine_Vtbl, 0x46a5433d_4ffb_48df_8732_4e15c834816b);
impl windows_core::RuntimeType for ILine {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ILine {
    pub fn put_X1(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_X1)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Y1(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Y1)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_X2(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_X2)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Y2(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Y2)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ILine_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_X1: usize,
    pub put_X1: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Y1: usize,
    pub put_Y1: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_X2: usize,
    pub put_X2: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Y2: usize,
    pub put_Y2: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IListBox,
    IListBox_Vtbl,
    0xe9f3b9ff_8e91_4ecf_a707_c927f694f881
);
impl windows_core::RuntimeType for IListBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IListBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_SelectedItems: usize,
    get_SelectionMode: usize,
    put_SelectionMode: usize,
    ScrollIntoView: usize,
    SelectAll: usize,
}
windows_core::imp::define_interface!(
    IListBoxFactory,
    IListBoxFactory_Vtbl,
    0x60cdfda2_2f44_444b_9c94_b8c9fda46f59
);
impl windows_core::RuntimeType for IListBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IListBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IListView,
    IListView_Vtbl,
    0xf6ce8c6d_fe96_41ad_a64a_c2b81c4af7f8
);
impl windows_core::RuntimeType for IListView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IListView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IListViewBase,
    IListViewBase_Vtbl,
    0x3d0813ba_6890_4537_bfe5_796d9458edd6
);
impl windows_core::RuntimeType for IListViewBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IListViewBase {
    pub fn put_SelectionMode(&self, value: ListViewSelectionMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectionMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IListViewBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_SelectedItems: usize,
    get_SelectionMode: usize,
    pub put_SelectionMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ListViewSelectionMode,
    ) -> windows_core::HRESULT,
    get_IsSwipeEnabled: usize,
    put_IsSwipeEnabled: usize,
    get_CanDragItems: usize,
    put_CanDragItems: usize,
    get_CanReorderItems: usize,
    put_CanReorderItems: usize,
    get_IsItemClickEnabled: usize,
    put_IsItemClickEnabled: usize,
    get_DataFetchSize: usize,
    put_DataFetchSize: usize,
    get_IncrementalLoadingThreshold: usize,
    put_IncrementalLoadingThreshold: usize,
    get_IncrementalLoadingTrigger: usize,
    put_IncrementalLoadingTrigger: usize,
    add_ItemClick: usize,
    remove_ItemClick: usize,
    add_DragItemsStarting: usize,
    remove_DragItemsStarting: usize,
    ScrollIntoView: usize,
    SelectAll: usize,
    LoadMoreItemsAsync: usize,
    ScrollIntoViewWithAlignment: usize,
    get_Header: usize,
    put_Header: usize,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_HeaderTransitions: usize,
    put_HeaderTransitions: usize,
}
windows_core::imp::define_interface!(
    IListViewFactory,
    IListViewFactory_Vtbl,
    0xbdff696d_3f22_41f9_97a6_883134a76113
);
impl windows_core::RuntimeType for IListViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IListViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuBar,
    IMenuBar_Vtbl,
    0xc8f67b0c_0e76_5af2_bade_785049c80a41
);
impl windows_core::RuntimeType for IMenuBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMenuBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Items: usize,
}
windows_core::imp::define_interface!(
    IMenuBarFactory,
    IMenuBarFactory_Vtbl,
    0xdc619e50_72ba_513f_80aa_ddd093825dde
);
impl windows_core::RuntimeType for IMenuBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMenuBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPanel, IPanel_Vtbl, 0xa50a4bbd_8361_469c_90da_e9a40c7474df);
impl windows_core::RuntimeType for IPanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPanel {
    pub fn get_Children(&self) -> windows_core::Result<UIElementCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Children)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Background<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Background)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IPanel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Children: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Background: usize,
    pub put_Background: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsItemsHost: usize,
    get_ChildrenTransitions: usize,
    put_ChildrenTransitions: usize,
}
windows_core::imp::define_interface!(
    IParagraph,
    IParagraph_Vtbl,
    0xf83ef59a_fa61_4bef_ae33_0b0ad756a84d
);
impl windows_core::RuntimeType for IParagraph {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IParagraph {
    pub fn get_Inlines(&self) -> windows_core::Result<InlineCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Inlines)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IParagraph_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Inlines: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TextIndent: usize,
    put_TextIndent: usize,
}
windows_core::imp::define_interface!(
    IPasswordBox,
    IPasswordBox_Vtbl,
    0x02b9aa11_0b47_4e7d_ad91_3a4168ed230d
);
impl windows_core::RuntimeType for IPasswordBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPasswordBox {
    pub fn get_Password(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Password)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn put_IsPasswordRevealButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPasswordRevealButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_PasswordChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_PasswordChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_PasswordChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IPasswordBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Password: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    put_Password: usize,
    get_PasswordChar: usize,
    put_PasswordChar: usize,
    get_IsPasswordRevealButtonEnabled: usize,
    pub put_IsPasswordRevealButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_MaxLength: usize,
    put_MaxLength: usize,
    pub add_PasswordChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_PasswordChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_ContextMenuOpening: usize,
    remove_ContextMenuOpening: usize,
    SelectAll: usize,
}
windows_core::imp::define_interface!(
    IPasswordBox2,
    IPasswordBox2_Vtbl,
    0x5ed738df_212f_4aeb_b5b8_2c219aec3c0c
);
impl windows_core::RuntimeType for IPasswordBox2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPasswordBox2 {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IPasswordBox2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SelectionHighlightColor: usize,
    put_SelectionHighlightColor: usize,
    get_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    put_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    add_Paste: usize,
    remove_Paste: usize,
}
windows_core::imp::define_interface!(
    IPasswordBox3,
    IPasswordBox3_Vtbl,
    0x6024d9d1_56b7_41f0_9558_3934c14244d6
);
impl windows_core::RuntimeType for IPasswordBox3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPasswordBox3 {
    pub fn put_PasswordRevealMode(&self, value: PasswordRevealMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PasswordRevealMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IPasswordBox3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PasswordRevealMode: usize,
    pub put_PasswordRevealMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PasswordRevealMode,
    ) -> windows_core::HRESULT,
    get_TextReadingOrder: usize,
    put_TextReadingOrder: usize,
    get_InputScope: usize,
    put_InputScope: usize,
}
windows_core::imp::define_interface!(
    IPersonPicture,
    IPersonPicture_Vtbl,
    0x6c230b6d_0d75_4059_91bc_7b174d1d7315
);
impl windows_core::RuntimeType for IPersonPicture {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPersonPicture {
    pub fn put_DisplayName(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_DisplayName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Initials(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Initials)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IPersonPicture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_BadgeNumber: usize,
    put_BadgeNumber: usize,
    get_BadgeGlyph: usize,
    put_BadgeGlyph: usize,
    get_BadgeImageSource: usize,
    put_BadgeImageSource: usize,
    get_BadgeText: usize,
    put_BadgeText: usize,
    get_IsGroup: usize,
    put_IsGroup: usize,
    get_Contact: usize,
    put_Contact: usize,
    get_DisplayName: usize,
    pub put_DisplayName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Initials: usize,
    pub put_Initials: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_PreferSmallImage: usize,
    put_PreferSmallImage: usize,
    get_ProfilePicture: usize,
    put_ProfilePicture: usize,
}
windows_core::imp::define_interface!(
    IPersonPictureFactory,
    IPersonPictureFactory_Vtbl,
    0x4f18330d_0416_4b92_bfd3_bf5780b46ab2
);
impl windows_core::RuntimeType for IPersonPictureFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPersonPictureFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPivot, IPivot_Vtbl, 0x103e9b13_3400_4a16_90b9_6912bf06974e);
impl windows_core::RuntimeType for IPivot {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPivot {
    pub fn put_Title<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_SelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IPivot_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TitleTemplate: usize,
    put_TitleTemplate: usize,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_SelectedIndex: usize,
    pub put_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_SelectedItem: usize,
    put_SelectedItem: usize,
    get_IsLocked: usize,
    put_IsLocked: usize,
    add_SelectionChanged: usize,
    remove_SelectionChanged: usize,
    add_PivotItemLoading: usize,
    remove_PivotItemLoading: usize,
    add_PivotItemLoaded: usize,
    remove_PivotItemLoaded: usize,
    add_PivotItemUnloading: usize,
    remove_PivotItemUnloading: usize,
    add_PivotItemUnloaded: usize,
    remove_PivotItemUnloaded: usize,
}
windows_core::imp::define_interface!(
    IPivotFactory,
    IPivotFactory_Vtbl,
    0x1b0a818e_2529_4762_ba44_9abc68c3ceca
);
impl windows_core::RuntimeType for IPivotFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPivotFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPivotItem,
    IPivotItem_Vtbl,
    0xa4764371_a502_47a3_915e_4aa096daf87f
);
impl windows_core::RuntimeType for IPivotItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPivotItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    put_Header: usize,
}
windows_core::imp::define_interface!(
    IPivotItemFactory,
    IPivotItemFactory_Vtbl,
    0x0dced981_636e_4a34_8a3f_8ee018639285
);
impl windows_core::RuntimeType for IPivotItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPivotItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IProgressBar,
    IProgressBar_Vtbl,
    0xae752c89_0067_4963_bf4c_29db0c4a507e
);
impl windows_core::RuntimeType for IProgressBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IProgressBar {
    pub fn put_IsIndeterminate(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsIndeterminate)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IProgressBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsIndeterminate: usize,
    pub put_IsIndeterminate:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_ShowError: usize,
    put_ShowError: usize,
    get_ShowPaused: usize,
    put_ShowPaused: usize,
    get_TemplateSettings: usize,
}
windows_core::imp::define_interface!(
    IProgressBarFactory,
    IProgressBarFactory_Vtbl,
    0xda9a8c11_1591_400b_a993_0f1c5cc12f3b
);
impl windows_core::RuntimeType for IProgressBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProgressBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IProgressRing,
    IProgressRing_Vtbl,
    0x6da5e49e_6e9d_425c_bd7c_02173e39763f
);
impl windows_core::RuntimeType for IProgressRing {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IProgressRing {
    pub fn put_IsActive(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsActive)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IProgressRing_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsActive: usize,
    pub put_IsActive:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_TemplateSettings: usize,
}
windows_core::imp::define_interface!(
    IRadioButton,
    IRadioButton_Vtbl,
    0x325c44e0_9a03_4bf3_abd6_6fbb46c9a486
);
impl windows_core::RuntimeType for IRadioButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRadioButton {
    pub fn put_GroupName(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_GroupName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRadioButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_GroupName: usize,
    pub put_GroupName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRadioButtonFactory,
    IRadioButtonFactory_Vtbl,
    0xf1d04933_34e1_4a5c_b2ae_ca3b1c0b20de
);
impl windows_core::RuntimeType for IRadioButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadioButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRangeBase,
    IRangeBase_Vtbl,
    0xfa002c1a_494e_46cf_91d4_e14a8d798675
);
impl windows_core::RuntimeType for IRangeBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRangeBase {
    pub fn put_Minimum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Minimum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Maximum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Maximum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_Value(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_Value(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Value)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_ValueChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<RangeBaseValueChangedEventArgs>,
            ) + 'static,
    {
        let handler: RangeBaseValueChangedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RangeBaseValueChangedEventHandler, F>::new(
                &RangeBaseValueChangedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ValueChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ValueChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IRangeBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Minimum: usize,
    pub put_Minimum:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Maximum: usize,
    pub put_Maximum:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_SmallChange: usize,
    put_SmallChange: usize,
    get_LargeChange: usize,
    put_LargeChange: usize,
    pub get_Value:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub add_ValueChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ValueChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRangeBaseValueChangedEventArgs,
    IRangeBaseValueChangedEventArgs_Vtbl,
    0xa1921777_d5c1_4f9c_a7b0_0401b7e6dc5c
);
impl windows_core::RuntimeType for IRangeBaseValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRangeBaseValueChangedEventArgs {
    pub fn get_NewValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IRangeBaseValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OldValue: usize,
    pub get_NewValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRatingControl,
    IRatingControl_Vtbl,
    0xa7d91ca7_e5cf_4963_a24e_9673fe5ffdd5
);
impl windows_core::RuntimeType for IRatingControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRatingControl {
    pub fn put_Caption(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Caption)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_IsReadOnly(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsReadOnly)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MaxRating(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MaxRating)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_PlaceholderValue(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderValue)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_Value(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_Value(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Value)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_ValueChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<RatingControl>, windows_core::Ref<windows_core::IInspectable>)
            + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            RatingControl,
            windows_core::IInspectable,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ValueChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ValueChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IRatingControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Caption: usize,
    pub put_Caption: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_InitialSetValue: usize,
    put_InitialSetValue: usize,
    get_IsClearEnabled: usize,
    put_IsClearEnabled: usize,
    get_IsReadOnly: usize,
    pub put_IsReadOnly:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_MaxRating: usize,
    pub put_MaxRating:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_PlaceholderValue: usize,
    pub put_PlaceholderValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_ItemInfo: usize,
    put_ItemInfo: usize,
    pub get_Value:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub add_ValueChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ValueChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRatingControlFactory,
    IRatingControlFactory_Vtbl,
    0x18d81716_c542_4ccb_b347_5e62c5db782e
);
impl windows_core::RuntimeType for IRatingControlFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRatingControlFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRectangle,
    IRectangle_Vtbl,
    0x855bc230_8a11_4e18_a136_4bc21c7827b0
);
impl windows_core::RuntimeType for IRectangle {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRectangle {
    pub fn put_RadiusX(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_RadiusX)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_RadiusY(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_RadiusY)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRectangle_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_RadiusX: usize,
    pub put_RadiusX:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_RadiusY: usize,
    pub put_RadiusY:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRelativePanel,
    IRelativePanel_Vtbl,
    0x2eabfaeb_b35a_4035_acea_3c4a3730683f
);
impl windows_core::RuntimeType for IRelativePanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRelativePanel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_BorderBrush: usize,
    put_BorderBrush: usize,
    get_BorderThickness: usize,
    put_BorderThickness: usize,
    get_CornerRadius: usize,
    put_CornerRadius: usize,
    get_Padding: usize,
    put_Padding: usize,
}
windows_core::imp::define_interface!(
    IRelativePanelFactory,
    IRelativePanelFactory_Vtbl,
    0x8460193c_361b_44ba_a17e_b84c9dcdc772
);
impl windows_core::RuntimeType for IRelativePanelFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRelativePanelFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRelativePanelStatics,
    IRelativePanelStatics_Vtbl,
    0x15903c27_f18c_4c35_8e19_6a7459d907b6
);
impl windows_core::RuntimeType for IRelativePanelStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRelativePanelStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_LeftOfProperty: usize,
    GetLeftOf: usize,
    SetLeftOf: usize,
    get_AboveProperty: usize,
    GetAbove: usize,
    SetAbove: usize,
    get_RightOfProperty: usize,
    GetRightOf: usize,
    SetRightOf: usize,
    get_BelowProperty: usize,
    GetBelow: usize,
    SetBelow: usize,
    get_AlignHorizontalCenterWithProperty: usize,
    GetAlignHorizontalCenterWith: usize,
    SetAlignHorizontalCenterWith: usize,
    get_AlignVerticalCenterWithProperty: usize,
    GetAlignVerticalCenterWith: usize,
    SetAlignVerticalCenterWith: usize,
    get_AlignLeftWithProperty: usize,
    GetAlignLeftWith: usize,
    SetAlignLeftWith: usize,
    get_AlignTopWithProperty: usize,
    GetAlignTopWith: usize,
    SetAlignTopWith: usize,
    get_AlignRightWithProperty: usize,
    GetAlignRightWith: usize,
    SetAlignRightWith: usize,
    get_AlignBottomWithProperty: usize,
    GetAlignBottomWith: usize,
    SetAlignBottomWith: usize,
    get_AlignLeftWithPanelProperty: usize,
    GetAlignLeftWithPanel: usize,
    pub SetAlignLeftWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_AlignTopWithPanelProperty: usize,
    GetAlignTopWithPanel: usize,
    pub SetAlignTopWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_AlignRightWithPanelProperty: usize,
    GetAlignRightWithPanel: usize,
    pub SetAlignRightWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_AlignBottomWithPanelProperty: usize,
    GetAlignBottomWithPanel: usize,
    pub SetAlignBottomWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_AlignHorizontalCenterWithPanelProperty: usize,
    GetAlignHorizontalCenterWithPanel: usize,
    pub SetAlignHorizontalCenterWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_AlignVerticalCenterWithPanelProperty: usize,
    GetAlignVerticalCenterWithPanel: usize,
    pub SetAlignVerticalCenterWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_BorderBrushProperty: usize,
    get_BorderThicknessProperty: usize,
    get_CornerRadiusProperty: usize,
    get_PaddingProperty: usize,
}
windows_core::imp::define_interface!(
    IRepeatButton,
    IRepeatButton_Vtbl,
    0x02200df9_021a_484a_a93b_0f31020314e5
);
impl windows_core::RuntimeType for IRepeatButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRepeatButton {
    pub fn put_Delay(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Delay)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Interval(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Interval)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRepeatButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Delay: usize,
    pub put_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_Interval: usize,
    pub put_Interval:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IResourceDictionary,
    IResourceDictionary_Vtbl,
    0xc1ea4f24_d6de_4191_8e3a_f48601f7489c
);
impl windows_core::RuntimeType for IResourceDictionary {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IResourceDictionary {
    pub fn get_MergedDictionaries(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<ResourceDictionary>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MergedDictionaries)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IResourceDictionary_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Source: usize,
    put_Source: usize,
    pub get_MergedDictionaries: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ThemeDictionaries: usize,
}
windows_core::imp::define_interface!(
    IResourceDictionaryFactory,
    IResourceDictionaryFactory_Vtbl,
    0xea3639b5_31b7_4271_92c9_7c9584a91c22
);
impl windows_core::RuntimeType for IResourceDictionaryFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceDictionaryFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IResourceManager,
    IResourceManager_Vtbl,
    0xf744d97b_9988_44fb_abd6_5378844cfa8b
);
impl windows_core::RuntimeType for IResourceManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IResourceManager {
    pub fn LoadPriFiles<P0>(&self, files: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<IStorageFile>>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).LoadPriFiles)(
                windows_core::Interface::as_raw(self),
                files.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IResourceManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_MainResourceMap: usize,
    get_AllResourceMaps: usize,
    get_DefaultContext: usize,
    pub LoadPriFiles: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    UnloadPriFiles: usize,
}
windows_core::imp::define_interface!(
    IResourceManagerStatics,
    IResourceManagerStatics_Vtbl,
    0x1cc0fdfc_69ee_4e43_9901_47f12687baf7
);
impl windows_core::RuntimeType for IResourceManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Current: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    IsResourceReference: usize,
}
windows_core::imp::define_interface!(
    IRichEditBox,
    IRichEditBox_Vtbl,
    0x90a57a40_80b6_4fce_b1ec_e3c616284b6a
);
impl windows_core::RuntimeType for IRichEditBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRichEditBox {
    pub fn put_IsReadOnly(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsReadOnly)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRichEditBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsReadOnly: usize,
    pub put_IsReadOnly:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_AcceptsReturn: usize,
    put_AcceptsReturn: usize,
    get_TextAlignment: usize,
    put_TextAlignment: usize,
    get_TextWrapping: usize,
    put_TextWrapping: usize,
    get_IsSpellCheckEnabled: usize,
    put_IsSpellCheckEnabled: usize,
    get_IsTextPredictionEnabled: usize,
    put_IsTextPredictionEnabled: usize,
    get_Document: usize,
    get_InputScope: usize,
    put_InputScope: usize,
    add_TextChanged: usize,
    remove_TextChanged: usize,
    add_SelectionChanged: usize,
    remove_SelectionChanged: usize,
    add_ContextMenuOpening: usize,
    remove_ContextMenuOpening: usize,
}
windows_core::imp::define_interface!(
    IRichEditBox2,
    IRichEditBox2_Vtbl,
    0xbbea6ead_e805_47a4_bbe7_47e59b8f74a7
);
impl windows_core::RuntimeType for IRichEditBox2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRichEditBox2 {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRichEditBox2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SelectionHighlightColor: usize,
    put_SelectionHighlightColor: usize,
    get_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    put_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    get_IsColorFontEnabled: usize,
    put_IsColorFontEnabled: usize,
    add_Paste: usize,
    remove_Paste: usize,
}
windows_core::imp::define_interface!(
    IRichEditBoxFactory,
    IRichEditBoxFactory_Vtbl,
    0x61a1df62_2806_41ed_88ed_ae21f47ab422
);
impl windows_core::RuntimeType for IRichEditBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRichEditBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRichTextBlock,
    IRichTextBlock_Vtbl,
    0xe5fff9e2_b968_49e7_97d4_8cca2ac3ae7c
);
impl windows_core::RuntimeType for IRichTextBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRichTextBlock {
    pub fn put_FontSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_FontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FontFamily>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontFamily)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Foreground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Foreground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn get_Blocks(&self) -> windows_core::Result<BlockCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Blocks)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IRichTextBlock_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_FontSize: usize,
    pub put_FontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_FontFamily: usize,
    pub put_FontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FontWeight: usize,
    put_FontWeight: usize,
    get_FontStyle: usize,
    put_FontStyle: usize,
    get_FontStretch: usize,
    put_FontStretch: usize,
    get_Foreground: usize,
    pub put_Foreground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TextWrapping: usize,
    put_TextWrapping: usize,
    get_TextTrimming: usize,
    put_TextTrimming: usize,
    get_TextAlignment: usize,
    put_TextAlignment: usize,
    pub get_Blocks: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Padding: usize,
    put_Padding: usize,
    get_LineHeight: usize,
    put_LineHeight: usize,
    get_LineStackingStrategy: usize,
    put_LineStackingStrategy: usize,
    get_CharacterSpacing: usize,
    put_CharacterSpacing: usize,
    get_OverflowContentTarget: usize,
    put_OverflowContentTarget: usize,
    get_IsTextSelectionEnabled: usize,
    put_IsTextSelectionEnabled: usize,
    get_HasOverflowContent: usize,
    get_SelectedText: usize,
    get_ContentStart: usize,
    get_ContentEnd: usize,
    get_SelectionStart: usize,
    get_SelectionEnd: usize,
    get_BaselineOffset: usize,
    add_SelectionChanged: usize,
    remove_SelectionChanged: usize,
    add_ContextMenuOpening: usize,
    remove_ContextMenuOpening: usize,
    SelectAll: usize,
    Select: usize,
    GetPositionFromPoint: usize,
    Focus: usize,
    get_TextIndent: usize,
    put_TextIndent: usize,
}
windows_core::imp::define_interface!(
    IRoutedEventArgs,
    IRoutedEventArgs_Vtbl,
    0x5c985ac6_d802_4b38_a223_bf070c43fedf
);
impl windows_core::RuntimeType for IRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OriginalSource: usize,
}
windows_core::imp::define_interface!(
    IRowDefinition,
    IRowDefinition_Vtbl,
    0x4abae829_d80c_4a5e_a48c_f8b3d3b6533d
);
impl windows_core::RuntimeType for IRowDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRowDefinition {
    pub fn put_Height(&self, value: GridLength) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Height)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRowDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Height: usize,
    pub put_Height:
        unsafe extern "system" fn(*mut core::ffi::c_void, GridLength) -> windows_core::HRESULT,
    get_MaxHeight: usize,
    put_MaxHeight: usize,
    get_MinHeight: usize,
    put_MinHeight: usize,
    get_ActualHeight: usize,
}
windows_core::imp::define_interface!(IRun, IRun_Vtbl, 0x59553c83_0e14_49bd_b84b_c526f3034349);
impl windows_core::RuntimeType for IRun {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRun {
    pub fn put_Text(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Text)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRun_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Text: usize,
    pub put_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FlowDirection: usize,
    put_FlowDirection: usize,
}
windows_core::imp::define_interface!(
    IScrollViewer,
    IScrollViewer_Vtbl,
    0x64e9be00_4dc1_493d_abe7_cbd3c577490d
);
impl windows_core::RuntimeType for IScrollViewer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IScrollViewer {
    pub fn put_HorizontalScrollBarVisibility(
        &self,
        value: ScrollBarVisibility,
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
        value: ScrollBarVisibility,
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
pub struct IScrollViewer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_HorizontalScrollBarVisibility: usize,
    pub put_HorizontalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollBarVisibility,
    ) -> windows_core::HRESULT,
    get_VerticalScrollBarVisibility: usize,
    pub put_VerticalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollBarVisibility,
    ) -> windows_core::HRESULT,
    get_IsHorizontalRailEnabled: usize,
    put_IsHorizontalRailEnabled: usize,
    get_IsVerticalRailEnabled: usize,
    put_IsVerticalRailEnabled: usize,
    get_IsHorizontalScrollChainingEnabled: usize,
    put_IsHorizontalScrollChainingEnabled: usize,
    get_IsVerticalScrollChainingEnabled: usize,
    put_IsVerticalScrollChainingEnabled: usize,
    get_IsZoomChainingEnabled: usize,
    put_IsZoomChainingEnabled: usize,
    get_IsScrollInertiaEnabled: usize,
    put_IsScrollInertiaEnabled: usize,
    get_IsZoomInertiaEnabled: usize,
    put_IsZoomInertiaEnabled: usize,
    get_HorizontalScrollMode: usize,
    put_HorizontalScrollMode: usize,
    get_VerticalScrollMode: usize,
    put_VerticalScrollMode: usize,
    get_ZoomMode: usize,
    put_ZoomMode: usize,
    get_HorizontalSnapPointsAlignment: usize,
    put_HorizontalSnapPointsAlignment: usize,
    get_VerticalSnapPointsAlignment: usize,
    put_VerticalSnapPointsAlignment: usize,
    get_HorizontalSnapPointsType: usize,
    put_HorizontalSnapPointsType: usize,
    get_VerticalSnapPointsType: usize,
    put_VerticalSnapPointsType: usize,
    get_ZoomSnapPointsType: usize,
    put_ZoomSnapPointsType: usize,
    get_HorizontalOffset: usize,
    get_ViewportWidth: usize,
    get_ScrollableWidth: usize,
    get_ComputedHorizontalScrollBarVisibility: usize,
    get_ExtentWidth: usize,
    get_VerticalOffset: usize,
    get_ViewportHeight: usize,
    get_ScrollableHeight: usize,
    get_ComputedVerticalScrollBarVisibility: usize,
    get_ExtentHeight: usize,
    get_MinZoomFactor: usize,
    put_MinZoomFactor: usize,
    get_MaxZoomFactor: usize,
    put_MaxZoomFactor: usize,
    get_ZoomFactor: usize,
    get_ZoomSnapPoints: usize,
    add_ViewChanged: usize,
    remove_ViewChanged: usize,
    ScrollToHorizontalOffset: usize,
    ScrollToVerticalOffset: usize,
    ZoomToFactor: usize,
    InvalidateScrollInfo: usize,
    get_IsDeferredScrollingEnabled: usize,
    put_IsDeferredScrollingEnabled: usize,
    get_BringIntoViewOnFocusChange: usize,
    put_BringIntoViewOnFocusChange: usize,
}
windows_core::imp::define_interface!(
    ISearchActivatedEventArgs,
    ISearchActivatedEventArgs_Vtbl,
    0x8cb36951_58c8_43e3_94bc_41d33f8b630e
);
impl windows_core::RuntimeType for ISearchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ISearchActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ISearchActivatedEventArgs, IActivatedEventArgs);
#[repr(C)]
pub struct ISearchActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_QueryText: usize,
    get_Language: usize,
}
windows_core::imp::define_interface!(
    ISelectionChangedEventArgs,
    ISelectionChangedEventArgs_Vtbl,
    0xc972d2dc_b609_4758_851e_a799c21de97d
);
impl windows_core::RuntimeType for ISelectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISelectionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_AddedItems: usize,
    get_RemovedItems: usize,
}
windows_core::imp::define_interface!(
    ISelector,
    ISelector_Vtbl,
    0xe30eb3a5_b36b_42dc_8527_cd25136c083c
);
impl windows_core::RuntimeType for ISelector {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISelector {
    pub fn get_SelectedIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_SelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_SelectionChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SelectionChangedEventArgs>,
            ) + 'static,
    {
        let handler: SelectionChangedEventHandler = {
            let com = windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::new(
                &SelectionChangedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectionChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ISelector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_SelectedItem: usize,
    put_SelectedItem: usize,
    get_SelectedValue: usize,
    put_SelectedValue: usize,
    get_SelectedValuePath: usize,
    put_SelectedValuePath: usize,
    get_IsSynchronizedWithCurrentItem: usize,
    put_IsSynchronizedWithCurrentItem: usize,
    pub add_SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IShape, IShape_Vtbl, 0x786f2b75_9aa0_454d_ae06_a2466e37c832);
impl windows_core::RuntimeType for IShape {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IShape {
    pub fn put_Fill<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Fill)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Stroke<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Stroke)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_StrokeThickness(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_StrokeThickness)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IShape_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Fill: usize,
    pub put_Fill: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Stroke: usize,
    pub put_Stroke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_StrokeMiterLimit: usize,
    put_StrokeMiterLimit: usize,
    get_StrokeThickness: usize,
    pub put_StrokeThickness:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_StrokeStartLineCap: usize,
    put_StrokeStartLineCap: usize,
    get_StrokeEndLineCap: usize,
    put_StrokeEndLineCap: usize,
    get_StrokeLineJoin: usize,
    put_StrokeLineJoin: usize,
    get_StrokeDashOffset: usize,
    put_StrokeDashOffset: usize,
    get_StrokeDashCap: usize,
    put_StrokeDashCap: usize,
    get_StrokeDashArray: usize,
    put_StrokeDashArray: usize,
    get_Stretch: usize,
    put_Stretch: usize,
    get_GeometryTransform: usize,
}
windows_core::imp::define_interface!(
    IShareTargetActivatedEventArgs,
    IShareTargetActivatedEventArgs_Vtbl,
    0x4bdaf9c8_cdb2_4acb_bfc3_6648563378ec
);
impl windows_core::RuntimeType for IShareTargetActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IShareTargetActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IShareTargetActivatedEventArgs, IActivatedEventArgs);
#[repr(C)]
pub struct IShareTargetActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ShareOperation: usize,
}
windows_core::imp::define_interface!(
    ISlider,
    ISlider_Vtbl,
    0x89572027_4c48_4700_8076_497ba73d9c18
);
impl windows_core::RuntimeType for ISlider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISlider {
    pub fn put_StepFrequency(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_StepFrequency)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Orientation(&self, value: Orientation) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Orientation)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ISlider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IntermediateValue: usize,
    put_IntermediateValue: usize,
    get_StepFrequency: usize,
    pub put_StepFrequency:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_SnapsTo: usize,
    put_SnapsTo: usize,
    get_TickFrequency: usize,
    put_TickFrequency: usize,
    get_TickPlacement: usize,
    put_TickPlacement: usize,
    get_Orientation: usize,
    pub put_Orientation:
        unsafe extern "system" fn(*mut core::ffi::c_void, Orientation) -> windows_core::HRESULT,
    get_IsDirectionReversed: usize,
    put_IsDirectionReversed: usize,
    get_IsThumbToolTipEnabled: usize,
    put_IsThumbToolTipEnabled: usize,
    get_ThumbToolTipValueConverter: usize,
    put_ThumbToolTipValueConverter: usize,
}
windows_core::imp::define_interface!(
    ISlider2,
    ISlider2_Vtbl,
    0x40a3c50e_87d6_4d2f_b1cf_b279cc996f26
);
impl windows_core::RuntimeType for ISlider2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISlider2 {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ISlider2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
}
windows_core::imp::define_interface!(
    ISliderFactory,
    ISliderFactory_Vtbl,
    0x03a67b37_c7bf_437c_848f_8cb5b753eab4
);
impl windows_core::RuntimeType for ISliderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISliderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISolidColorBrush,
    ISolidColorBrush_Vtbl,
    0x9d850850_66f3_48df_9a8f_824bd5e070af
);
impl windows_core::RuntimeType for ISolidColorBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISolidColorBrush {
    pub fn put_Color(&self, value: Color) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Color)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ISolidColorBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Color: usize,
    pub put_Color:
        unsafe extern "system" fn(*mut core::ffi::c_void, Color) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISplitButton,
    ISplitButton_Vtbl,
    0x58bbb066_c2ea_5499_8150_40faa75f6bb5
);
impl windows_core::RuntimeType for ISplitButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISplitButton {
    pub fn add_Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<SplitButton>, windows_core::Ref<SplitButtonClickEventArgs>)
            + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            SplitButton,
            SplitButtonClickEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Click)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Click,
            ))
        }
    }
}
#[repr(C)]
pub struct ISplitButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Flyout: usize,
    put_Flyout: usize,
    get_Command: usize,
    put_Command: usize,
    get_CommandParameter: usize,
    put_CommandParameter: usize,
    pub add_Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Click:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISplitButtonClickEventArgs,
    ISplitButtonClickEventArgs_Vtbl,
    0xc227c2ca_26f4_5960_98d5_919149d1b525
);
impl windows_core::RuntimeType for ISplitButtonClickEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISplitButtonClickEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ISplitButtonFactory,
    ISplitButtonFactory_Vtbl,
    0x3201c32f_4d55_589d_97dd_617fa3642137
);
impl windows_core::RuntimeType for ISplitButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISplitButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISplitView,
    ISplitView_Vtbl,
    0x97222f31_3844_429e_939c_1673155322a1
);
impl windows_core::RuntimeType for ISplitView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISplitView {
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Pane<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Pane)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsPaneOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPaneOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_OpenPaneLength(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_OpenPaneLength)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_CompactPaneLength(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_CompactPaneLength)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_DisplayMode(&self, value: SplitViewDisplayMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_DisplayMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_PaneClosed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<SplitView>, windows_core::Ref<windows_core::IInspectable>)
            + Send
            + 'static,
    {
        let handler =
            <windows::Foundation::TypedEventHandler<SplitView, windows_core::IInspectable>>::new(
                move |a0, a1| {
                    handler(a0, a1);
                    Ok(())
                },
            );
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_PaneClosed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_PaneClosed,
            ))
        }
    }
}
#[repr(C)]
pub struct ISplitView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Content: usize,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Pane: usize,
    pub put_Pane: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsPaneOpen: usize,
    pub put_IsPaneOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_OpenPaneLength: usize,
    pub put_OpenPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_CompactPaneLength: usize,
    pub put_CompactPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_PanePlacement: usize,
    put_PanePlacement: usize,
    get_DisplayMode: usize,
    pub put_DisplayMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SplitViewDisplayMode,
    ) -> windows_core::HRESULT,
    get_TemplateSettings: usize,
    get_PaneBackground: usize,
    put_PaneBackground: usize,
    add_PaneClosing: usize,
    remove_PaneClosing: usize,
    pub add_PaneClosed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_PaneClosed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISplitViewFactory,
    ISplitViewFactory_Vtbl,
    0xf101773a_084e_4fb9_8442_63221b44533f
);
impl windows_core::RuntimeType for ISplitViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISplitViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStackPanel,
    IStackPanel_Vtbl,
    0xb8ae8fe2_d641_4fd7_80b4_7439207d2798
);
impl windows_core::RuntimeType for IStackPanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IStackPanel {
    pub fn put_Orientation(&self, value: Orientation) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Orientation)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IStackPanel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_AreScrollSnapPointsRegular: usize,
    put_AreScrollSnapPointsRegular: usize,
    get_Orientation: usize,
    pub put_Orientation:
        unsafe extern "system" fn(*mut core::ffi::c_void, Orientation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStackPanel4,
    IStackPanel4_Vtbl,
    0x43ebf7f6_3196_412e_8a95_add002ff43f0
);
impl windows_core::RuntimeType for IStackPanel4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IStackPanel4 {
    pub fn put_Spacing(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Spacing)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IStackPanel4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Spacing: usize,
    pub put_Spacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStackPanelFactory,
    IStackPanelFactory_Vtbl,
    0x63d8248a_8b34_445a_808f_b6ecd62a27d9
);
impl windows_core::RuntimeType for IStackPanelFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStackPanelFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStorageFile,
    IStorageFile_Vtbl,
    0xfa3f6186_4214_428c_a64c_14c9ac7315ea
);
impl windows_core::RuntimeType for IStorageFile {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IStorageFile,
    windows_core::IUnknown,
    windows_core::IInspectable
);
#[repr(C)]
pub struct IStorageFile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_FileType: usize,
    get_ContentType: usize,
    OpenAsync: usize,
    OpenTransactedWriteAsync: usize,
    CopyOverloadDefaultNameAndOptions: usize,
    CopyOverloadDefaultOptions: usize,
    CopyOverload: usize,
    CopyAndReplaceAsync: usize,
    MoveOverloadDefaultNameAndOptions: usize,
    MoveOverloadDefaultOptions: usize,
    MoveOverload: usize,
    MoveAndReplaceAsync: usize,
}
windows_core::imp::define_interface!(
    IStorageFileStatics,
    IStorageFileStatics_Vtbl,
    0x5984c710_daf2_43c8_8bb4_a4d3eacfd03f
);
impl windows_core::RuntimeType for IStorageFileStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageFileStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFileFromPathAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetFileFromApplicationUriAsync: usize,
    CreateStreamedFileAsync: usize,
    ReplaceWithStreamedFileAsync: usize,
    CreateStreamedFileFromUriAsync: usize,
    ReplaceWithStreamedFileFromUriAsync: usize,
}
windows_core::imp::define_interface!(IStyle, IStyle_Vtbl, 0xc4a9f225_9db7_4a7d_b6d1_f74edb9293c2);
impl windows_core::RuntimeType for IStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStyle_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsSealed: usize,
    get_Setters: usize,
    get_TargetType: usize,
    put_TargetType: usize,
    get_BasedOn: usize,
    put_BasedOn: usize,
    Seal: usize,
}
windows_core::imp::define_interface!(
    ISymbolIcon,
    ISymbolIcon_Vtbl,
    0x7a4774c9_a6a3_4b30_8ff1_9081d70e9a5c
);
impl windows_core::RuntimeType for ISymbolIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISymbolIcon_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Symbol: usize,
    put_Symbol: usize,
}
windows_core::imp::define_interface!(
    ISymbolIconFactory,
    ISymbolIconFactory_Vtbl,
    0xc7252b88_e76c_4b44_8a05_046b9dc772b8
);
impl windows_core::RuntimeType for ISymbolIconFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISymbolIconFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithSymbol: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        Symbol,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextBlock,
    ITextBlock_Vtbl,
    0xae2d9271_3b4a_45fc_8468_f7949548f4d5
);
impl windows_core::RuntimeType for ITextBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITextBlock {
    pub fn put_FontSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_FontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FontFamily>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontFamily)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_FontWeight(&self, value: FontWeight) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontWeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Foreground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Foreground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_TextWrapping(&self, value: TextWrapping) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_TextWrapping)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Text(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Text)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_IsTextSelectionEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsTextSelectionEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ITextBlock_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_FontSize: usize,
    pub put_FontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_FontFamily: usize,
    pub put_FontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FontWeight: usize,
    pub put_FontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FontWeight) -> windows_core::HRESULT,
    get_FontStyle: usize,
    put_FontStyle: usize,
    get_FontStretch: usize,
    put_FontStretch: usize,
    get_CharacterSpacing: usize,
    put_CharacterSpacing: usize,
    get_Foreground: usize,
    pub put_Foreground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TextWrapping: usize,
    pub put_TextWrapping:
        unsafe extern "system" fn(*mut core::ffi::c_void, TextWrapping) -> windows_core::HRESULT,
    get_TextTrimming: usize,
    put_TextTrimming: usize,
    get_TextAlignment: usize,
    put_TextAlignment: usize,
    get_Text: usize,
    pub put_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Inlines: usize,
    get_Padding: usize,
    put_Padding: usize,
    get_LineHeight: usize,
    put_LineHeight: usize,
    get_LineStackingStrategy: usize,
    put_LineStackingStrategy: usize,
    get_IsTextSelectionEnabled: usize,
    pub put_IsTextSelectionEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_SelectedText: usize,
    get_ContentStart: usize,
    get_ContentEnd: usize,
    get_SelectionStart: usize,
    get_SelectionEnd: usize,
    get_BaselineOffset: usize,
    add_SelectionChanged: usize,
    remove_SelectionChanged: usize,
    add_ContextMenuOpening: usize,
    remove_ContextMenuOpening: usize,
    SelectAll: usize,
    Select: usize,
    Focus: usize,
}
windows_core::imp::define_interface!(
    ITextBox,
    ITextBox_Vtbl,
    0xe48f5a8b_1dff_4352_a1f4_e516514ec882
);
impl windows_core::RuntimeType for ITextBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITextBox {
    pub fn get_Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn put_Text(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Text)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_AcceptsReturn(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_AcceptsReturn)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_TextWrapping(&self, value: TextWrapping) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_TextWrapping)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_TextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<TextChangedEventArgs>,
            ) + 'static,
    {
        let handler: TextChangedEventHandler = {
            let com = windows_core::imp::DelegateBox::<TextChangedEventHandler, F>::new(
                &TextChangedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_TextChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_TextChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ITextBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SelectedText: usize,
    put_SelectedText: usize,
    get_SelectionLength: usize,
    put_SelectionLength: usize,
    get_SelectionStart: usize,
    put_SelectionStart: usize,
    get_MaxLength: usize,
    put_MaxLength: usize,
    get_IsReadOnly: usize,
    put_IsReadOnly: usize,
    get_AcceptsReturn: usize,
    pub put_AcceptsReturn:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_TextAlignment: usize,
    put_TextAlignment: usize,
    get_TextWrapping: usize,
    pub put_TextWrapping:
        unsafe extern "system" fn(*mut core::ffi::c_void, TextWrapping) -> windows_core::HRESULT,
    get_IsSpellCheckEnabled: usize,
    put_IsSpellCheckEnabled: usize,
    get_IsTextPredictionEnabled: usize,
    put_IsTextPredictionEnabled: usize,
    get_InputScope: usize,
    put_InputScope: usize,
    pub add_TextChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_TextChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_SelectionChanged: usize,
    remove_SelectionChanged: usize,
    add_ContextMenuOpening: usize,
    remove_ContextMenuOpening: usize,
    Select: usize,
    SelectAll: usize,
    GetRectFromCharacterIndex: usize,
}
windows_core::imp::define_interface!(
    ITextBox2,
    ITextBox2_Vtbl,
    0xf7168c00_1432_462a_9405_38f385bfc37c
);
impl windows_core::RuntimeType for ITextBox2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITextBox2 {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ITextBox2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SelectionHighlightColor: usize,
    put_SelectionHighlightColor: usize,
    get_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    put_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    get_IsColorFontEnabled: usize,
    put_IsColorFontEnabled: usize,
    add_Paste: usize,
    remove_Paste: usize,
}
windows_core::imp::define_interface!(
    ITextBoxFactory,
    ITextBoxFactory_Vtbl,
    0x710e4278_8529_47d3_8d8e_307e34cff081
);
impl windows_core::RuntimeType for ITextBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextChangedEventArgs,
    ITextChangedEventArgs_Vtbl,
    0x4dd04f7d_7a11_4b2e_9933_577df39252b6
);
impl windows_core::RuntimeType for ITextChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ITextElement,
    ITextElement_Vtbl,
    0xe83b0062_d776_4f92_baea_40e77d4791d5
);
impl windows_core::RuntimeType for ITextElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Name: usize,
    get_FontSize: usize,
    put_FontSize: usize,
    get_FontFamily: usize,
    put_FontFamily: usize,
    get_FontWeight: usize,
    put_FontWeight: usize,
    get_FontStyle: usize,
    put_FontStyle: usize,
    get_FontStretch: usize,
    put_FontStretch: usize,
    get_CharacterSpacing: usize,
    put_CharacterSpacing: usize,
    get_Foreground: usize,
    put_Foreground: usize,
    get_Language: usize,
    put_Language: usize,
    get_ContentStart: usize,
    get_ContentEnd: usize,
    get_ElementStart: usize,
    get_ElementEnd: usize,
    FindName: usize,
}
windows_core::imp::define_interface!(
    ITimePicker,
    ITimePicker_Vtbl,
    0xe39099f2_3aff_4792_909e_2d9941ec0357
);
impl windows_core::RuntimeType for ITimePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITimePicker {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_ClockIdentifier(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_ClockIdentifier)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_MinuteIncrement(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MinuteIncrement)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_TimeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<TimePickerValueChangedEventArgs>,
            ) + Send
            + 'static,
    {
        let handler = <windows::Foundation::EventHandler<TimePickerValueChangedEventArgs>>::new(
            move |a0, a1| {
                handler(a0, a1);
                Ok(())
            },
        );
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_TimeChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_TimeChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ITimePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_ClockIdentifier: usize,
    pub put_ClockIdentifier: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_MinuteIncrement: usize,
    pub put_MinuteIncrement:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_Time: usize,
    put_Time: usize,
    pub add_TimeChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_TimeChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITimePickerFactory,
    ITimePickerFactory_Vtbl,
    0x553fe413_6cd7_46a9_a97b_a18bdc4b4ca3
);
impl windows_core::RuntimeType for ITimePickerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITimePickerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITimePickerValueChangedEventArgs,
    ITimePickerValueChangedEventArgs_Vtbl,
    0x2f4edb8d_b995_4e31_8ba9_c4dcdeb21ca3
);
impl windows_core::RuntimeType for ITimePickerValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITimePickerValueChangedEventArgs {
    pub fn get_NewTime(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ITimePickerValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OldTime: usize,
    pub get_NewTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToggleButton,
    IToggleButton_Vtbl,
    0x589877fb_0fc7_4036_9d8b_127dfa75c16d
);
impl windows_core::RuntimeType for IToggleButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IToggleButton {
    pub fn put_IsChecked(&self, value: Option<bool>) -> windows_core::Result<()> {
        let value__ =
            value.map(<windows_reference::IReference<bool> as core::convert::From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).put_IsChecked)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn add_Checked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Checked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Checked,
            ))
        }
    }
    pub fn add_Unchecked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Unchecked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Unchecked,
            ))
        }
    }
}
#[repr(C)]
pub struct IToggleButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsChecked: usize,
    pub put_IsChecked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsThreeState: usize,
    put_IsThreeState: usize,
    pub add_Checked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Checked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_Unchecked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Unchecked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_Indeterminate: usize,
    remove_Indeterminate: usize,
}
windows_core::imp::define_interface!(
    IToggleButtonFactory,
    IToggleButtonFactory_Vtbl,
    0xd56aa2fc_fc7f_449c_9855_7a1055d668a8
);
impl windows_core::RuntimeType for IToggleButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToggleButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToggleSwitch,
    IToggleSwitch_Vtbl,
    0x331d8f00_c5f9_46a5_b6c8_ede539304567
);
impl windows_core::RuntimeType for IToggleSwitch {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IToggleSwitch {
    pub fn get_IsOn(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsOn)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_IsOn(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsOn)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_OnContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_OnContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_OffContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_OffContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn add_Toggled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Toggled)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Toggled,
            ))
        }
    }
}
#[repr(C)]
pub struct IToggleSwitch_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_IsOn:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub put_IsOn: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_OnContent: usize,
    pub put_OnContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_OnContentTemplate: usize,
    put_OnContentTemplate: usize,
    get_OffContent: usize,
    pub put_OffContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_OffContentTemplate: usize,
    put_OffContentTemplate: usize,
    get_TemplateSettings: usize,
    pub add_Toggled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Toggled:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToolTipService,
    IToolTipService_Vtbl,
    0x03a55f87_bfcc_4a1e_8fea_98f610832cea
);
impl windows_core::RuntimeType for IToolTipService {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToolTipService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IToolTipServiceStatics,
    IToolTipServiceStatics_Vtbl,
    0x86e649f8_e245_48aa_a8c8_d1073ed76319
);
impl windows_core::RuntimeType for IToolTipServiceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToolTipServiceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PlacementProperty: usize,
    GetPlacement: usize,
    SetPlacement: usize,
    get_PlacementTargetProperty: usize,
    GetPlacementTarget: usize,
    SetPlacementTarget: usize,
    get_ToolTipProperty: usize,
    GetToolTip: usize,
    pub SetToolTip: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITreeView,
    ITreeView_Vtbl,
    0x9353cc5c_dd6e_453c_aedd_0c3ac993978b
);
impl windows_core::RuntimeType for ITreeView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITreeView {
    pub fn put_SelectionMode(&self, value: TreeViewSelectionMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectionMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_ItemInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TreeView>, windows_core::Ref<TreeViewItemInvokedEventArgs>)
            + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            TreeView,
            TreeViewItemInvokedEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ItemInvoked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ItemInvoked,
            ))
        }
    }
}
#[repr(C)]
pub struct ITreeView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_RootNodes: usize,
    get_SelectionMode: usize,
    pub put_SelectionMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TreeViewSelectionMode,
    ) -> windows_core::HRESULT,
    get_SelectedNodes: usize,
    Expand: usize,
    Collapse: usize,
    SelectAll: usize,
    pub add_ItemInvoked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ItemInvoked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_Expanding: usize,
    remove_Expanding: usize,
    add_Collapsed: usize,
    remove_Collapsed: usize,
}
windows_core::imp::define_interface!(
    ITreeViewFactory,
    ITreeViewFactory_Vtbl,
    0xcc5267c3_6c69_49ce_b445_753acee7948b
);
impl windows_core::RuntimeType for ITreeViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITreeViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITreeViewItemInvokedEventArgs,
    ITreeViewItemInvokedEventArgs_Vtbl,
    0x472ab521_0242_4290_9363_ab4fe704527f
);
impl windows_core::RuntimeType for ITreeViewItemInvokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITreeViewItemInvokedEventArgs {
    pub fn get_InvokedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InvokedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ITreeViewItemInvokedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_InvokedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    put_Handled: usize,
    get_Handled: usize,
}
windows_core::imp::define_interface!(
    ITreeViewNode,
    ITreeViewNode_Vtbl,
    0xc04c8ed3_9af2_4e75_a329_7497a110e7a8
);
impl windows_core::RuntimeType for ITreeViewNode {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITreeViewNode {
    pub fn get_Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ITreeViewNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    put_Content: usize,
    get_Parent: usize,
    get_IsExpanded: usize,
    put_IsExpanded: usize,
    get_HasChildren: usize,
    get_Depth: usize,
    get_HasUnrealizedChildren: usize,
    put_HasUnrealizedChildren: usize,
    get_Children: usize,
}
windows_core::imp::define_interface!(
    IUIElement,
    IUIElement_Vtbl,
    0x676d0be9_b65c_41c6_ba40_58cf87f201c1
);
impl windows_core::RuntimeType for IUIElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUIElement {
    pub fn put_Opacity(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Opacity)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IUIElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_DesiredSize: usize,
    get_AllowDrop: usize,
    put_AllowDrop: usize,
    get_Opacity: usize,
    pub put_Opacity:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Clip: usize,
    put_Clip: usize,
    get_RenderTransform: usize,
    put_RenderTransform: usize,
    get_Projection: usize,
    put_Projection: usize,
    get_RenderTransformOrigin: usize,
    put_RenderTransformOrigin: usize,
    get_IsHitTestVisible: usize,
    put_IsHitTestVisible: usize,
    get_Visibility: usize,
    put_Visibility: usize,
    get_RenderSize: usize,
    get_UseLayoutRounding: usize,
    put_UseLayoutRounding: usize,
    get_Transitions: usize,
    put_Transitions: usize,
    get_CacheMode: usize,
    put_CacheMode: usize,
    get_IsTapEnabled: usize,
    put_IsTapEnabled: usize,
    get_IsDoubleTapEnabled: usize,
    put_IsDoubleTapEnabled: usize,
    get_IsRightTapEnabled: usize,
    put_IsRightTapEnabled: usize,
    get_IsHoldingEnabled: usize,
    put_IsHoldingEnabled: usize,
    get_ManipulationMode: usize,
    put_ManipulationMode: usize,
    get_PointerCaptures: usize,
    add_KeyUp: usize,
    remove_KeyUp: usize,
    add_KeyDown: usize,
    remove_KeyDown: usize,
    add_GotFocus: usize,
    remove_GotFocus: usize,
    add_LostFocus: usize,
    remove_LostFocus: usize,
    add_DragEnter: usize,
    remove_DragEnter: usize,
    add_DragLeave: usize,
    remove_DragLeave: usize,
    add_DragOver: usize,
    remove_DragOver: usize,
    add_Drop: usize,
    remove_Drop: usize,
    add_PointerPressed: usize,
    remove_PointerPressed: usize,
    add_PointerMoved: usize,
    remove_PointerMoved: usize,
    add_PointerReleased: usize,
    remove_PointerReleased: usize,
    add_PointerEntered: usize,
    remove_PointerEntered: usize,
    add_PointerExited: usize,
    remove_PointerExited: usize,
    add_PointerCaptureLost: usize,
    remove_PointerCaptureLost: usize,
    add_PointerCanceled: usize,
    remove_PointerCanceled: usize,
    add_PointerWheelChanged: usize,
    remove_PointerWheelChanged: usize,
    add_Tapped: usize,
    remove_Tapped: usize,
    add_DoubleTapped: usize,
    remove_DoubleTapped: usize,
    add_Holding: usize,
    remove_Holding: usize,
    add_RightTapped: usize,
    remove_RightTapped: usize,
    add_ManipulationStarting: usize,
    remove_ManipulationStarting: usize,
    add_ManipulationInertiaStarting: usize,
    remove_ManipulationInertiaStarting: usize,
    add_ManipulationStarted: usize,
    remove_ManipulationStarted: usize,
    add_ManipulationDelta: usize,
    remove_ManipulationDelta: usize,
    add_ManipulationCompleted: usize,
    remove_ManipulationCompleted: usize,
    Measure: usize,
    Arrange: usize,
    CapturePointer: usize,
    ReleasePointerCapture: usize,
    ReleasePointerCaptures: usize,
    AddHandler: usize,
    RemoveHandler: usize,
    TransformToVisual: usize,
    InvalidateMeasure: usize,
    InvalidateArrange: usize,
    UpdateLayout: usize,
}
windows_core::imp::define_interface!(
    IUISettings,
    IUISettings_Vtbl,
    0x85361600_1c63_4627_bcb1_3a89e0bc9c55
);
impl windows_core::RuntimeType for IUISettings {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_HandPreference: usize,
    get_CursorSize: usize,
    get_ScrollBarSize: usize,
    get_ScrollBarArrowSize: usize,
    get_ScrollBarThumbBoxSize: usize,
    get_MessageDuration: usize,
    get_AnimationsEnabled: usize,
    get_CaretBrowsingEnabled: usize,
    get_CaretBlinkRate: usize,
    get_CaretWidth: usize,
    get_DoubleClickTime: usize,
    get_MouseHoverTime: usize,
    UIElementColor: usize,
}
windows_core::imp::define_interface!(
    IUISettings3,
    IUISettings3_Vtbl,
    0x03021be4_5254_4781_8194_5168f7d06d7b
);
impl windows_core::RuntimeType for IUISettings3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUISettings3 {
    pub fn GetColorValue(&self, desiredcolor: UIColorType) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorValue)(
                windows_core::Interface::as_raw(self),
                desiredcolor,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn add_ColorValuesChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UISettings>, windows_core::Ref<windows_core::IInspectable>)
            + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            UISettings,
            windows_core::IInspectable,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ColorValuesChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ColorValuesChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IUISettings3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetColorValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        UIColorType,
        *mut Color,
    ) -> windows_core::HRESULT,
    pub add_ColorValuesChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ColorValuesChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUnhandledExceptionEventArgs,
    IUnhandledExceptionEventArgs_Vtbl,
    0x7230269c_054e_4cf3_86c5_be90eb6863d5
);
impl windows_core::RuntimeType for IUnhandledExceptionEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUnhandledExceptionEventArgs {
    pub fn get_Exception(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Exception)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_Message(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Message)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
}
#[repr(C)]
pub struct IUnhandledExceptionEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Exception: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::HRESULT,
    ) -> windows_core::HRESULT,
    pub get_Message: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Handled: usize,
    put_Handled: usize,
}
windows_core::imp::define_interface!(
    IViewbox,
    IViewbox_Vtbl,
    0x05252c58_ba9d_4809_9ec3_fa0d16710ba1
);
impl windows_core::RuntimeType for IViewbox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IViewbox {
    pub fn put_Child<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Child)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Stretch(&self, value: Stretch) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Stretch)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IViewbox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Child: usize,
    pub put_Child: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Stretch: usize,
    pub put_Stretch:
        unsafe extern "system" fn(*mut core::ffi::c_void, Stretch) -> windows_core::HRESULT,
    get_StretchDirection: usize,
    put_StretchDirection: usize,
}
windows_core::imp::define_interface!(
    IWindowCreatedEventArgs,
    IWindowCreatedEventArgs_Vtbl,
    0x31b71470_feff_4654_af48_9b398ab5772b
);
impl windows_core::RuntimeType for IWindowCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowCreatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Window: usize,
}
windows_core::imp::define_interface!(
    IWindowsXamlManager,
    IWindowsXamlManager_Vtbl,
    0x56096c31_1aa0_5288_8818_6e74a2dcaff5
);
impl windows_core::RuntimeType for IWindowsXamlManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsXamlManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IWindowsXamlManagerStatics,
    IWindowsXamlManagerStatics_Vtbl,
    0x28258a12_7d82_505b_b210_712b04a58882
);
impl windows_core::RuntimeType for IWindowsXamlManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsXamlManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InitializeForCurrentThread: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlMember,
    IXamlMember_Vtbl,
    0xc541f58c_43a9_4216_b718_e0b11b14e93e
);
impl windows_core::RuntimeType for IXamlMember {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IXamlMember,
    windows_core::IUnknown,
    windows_core::IInspectable
);
#[repr(C)]
pub struct IXamlMember_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsAttachable: usize,
    get_IsDependencyProperty: usize,
    get_IsReadOnly: usize,
    get_Name: usize,
    get_TargetType: usize,
    get_Type: usize,
    GetValue: usize,
    SetValue: usize,
}
windows_core::imp::define_interface!(
    IXamlMetadataProvider,
    IXamlMetadataProvider_Vtbl,
    0xb3765d69_68a5_4b32_8861_fdb90c1f5836
);
impl windows_core::RuntimeType for IXamlMetadataProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Xaml.Markup.IXamlMetadataProvider");
}
windows_core::imp::interface_hierarchy!(
    IXamlMetadataProvider,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IXamlMetadataProvider {
    pub fn GetXamlType(&self, r#type: &TypeName) -> windows_core::Result<IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXamlType)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(r#type),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXamlTypeByFullName(&self, fullname: &str) -> windows_core::Result<IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXamlTypeByFullName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(fullname)),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> windows_core::Result<windows_core::Array<XmlnsDefinition>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetXmlnsDefinitions)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<XmlnsDefinition>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeName for IXamlMetadataProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlMetadataProvider";
}
pub trait IXamlMetadataProvider_Impl: windows_core::IUnknownImpl {
    fn GetXamlType(&self, r#type: &TypeName) -> windows_core::Result<IXamlType>;
    fn GetXamlTypeByFullName(
        &self,
        fullName: &windows_core::HSTRING,
    ) -> windows_core::Result<IXamlType>;
    fn GetXmlnsDefinitions(&self) -> windows_core::Result<windows_core::Array<XmlnsDefinition>>;
}
impl IXamlMetadataProvider_Vtbl {
    pub const fn new<Identity: IXamlMetadataProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetXamlType<
            Identity: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            r#type: core::mem::MaybeUninit<TypeName>,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlMetadataProvider_Impl::GetXamlType(this, core::mem::transmute(&r#type)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetXamlTypeByFullName<
            Identity: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fullname: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlMetadataProvider_Impl::GetXamlTypeByFullName(
                    this,
                    core::mem::transmute(&fullname),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetXmlnsDefinitions<
            Identity: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut core::mem::MaybeUninit<XmlnsDefinition>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlMetadataProvider_Impl::GetXmlnsDefinitions(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXamlMetadataProvider, OFFSET>(
            ),
            GetXamlType: GetXamlType::<Identity, OFFSET>,
            GetXamlTypeByFullName: GetXamlTypeByFullName::<Identity, OFFSET>,
            GetXmlnsDefinitions: GetXmlnsDefinitions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlMetadataProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IXamlMetadataProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetXamlType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        core::mem::MaybeUninit<TypeName>,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetXamlTypeByFullName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetXmlnsDefinitions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut core::mem::MaybeUninit<XmlnsDefinition>,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlReader,
    IXamlReader_Vtbl,
    0x24374cf1_cceb_48bf_a514_41b0186f84c2
);
impl windows_core::RuntimeType for IXamlReader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXamlReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IXamlReaderStatics,
    IXamlReaderStatics_Vtbl,
    0x9891c6bd_534f_4955_b85a_8a8dc0dca602
);
impl windows_core::RuntimeType for IXamlReaderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXamlReaderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Load: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    LoadWithInitialTemplateValidation: usize,
}
windows_core::imp::define_interface!(
    IXamlType,
    IXamlType_Vtbl,
    0x7920eab1_a2e5_479a_bd50_6cef3c0b4970
);
impl windows_core::RuntimeType for IXamlType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IXamlType,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IXamlType {
    pub fn get_BaseType(&self) -> windows_core::Result<IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BaseType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_ContentProperty(&self) -> windows_core::Result<IXamlMember> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ContentProperty)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_FullName(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FullName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn get_IsArray(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsArray)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsCollection(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsCollection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsConstructible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsConstructible)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsDictionary(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsDictionary)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsMarkupExtension(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsMarkupExtension)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsBindable(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsBindable)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_ItemType(&self) -> windows_core::Result<IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ItemType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_KeyType(&self) -> windows_core::Result<IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeyType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_UnderlyingType(&self) -> windows_core::Result<TypeName> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UnderlyingType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ActivateInstance(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateInstance)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromString(
        &self,
        value: &str,
    ) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFromString)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMember(&self, name: &str) -> windows_core::Result<IXamlMember> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMember)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(name)),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddToVector<P0, P1>(&self, instance: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddToVector)(
                windows_core::Interface::as_raw(self),
                instance.param().abi(),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AddToMap<P0, P1, P2>(&self, instance: P0, key: P1, value: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
        P2: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddToMap)(
                windows_core::Interface::as_raw(self),
                instance.param().abi(),
                key.param().abi(),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RunInitializer(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RunInitializer)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
}
#[repr(C)]
pub struct IXamlType_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_BaseType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_ContentProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_FullName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_IsArray:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_IsCollection:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_IsConstructible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_IsDictionary:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_IsMarkupExtension:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_IsBindable:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_ItemType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_KeyType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_UnderlyingType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<TypeName>,
    ) -> windows_core::HRESULT,
    pub ActivateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFromString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetMember: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddToVector: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddToMap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RunInitializer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IconElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    IconElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IconElement, FrameworkElement, UIElement, DependencyObject);
impl windows_core::RuntimeType for IconElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IIconElement>();
}
unsafe impl windows_core::Interface for IconElement {
    type Vtable = <IIconElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIconElement as windows_core::Interface>::IID;
}
impl core::ops::Deref for IconElement {
    type Target = IIconElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for IconElement {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.IconElement";
}
unsafe impl Send for IconElement {}
unsafe impl Sync for IconElement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Image, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Image, FrameworkElement, UIElement, DependencyObject);
impl Image {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Image, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Image {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IImage>();
}
unsafe impl windows_core::Interface for Image {
    type Vtable = <IImage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IImage as windows_core::Interface>::IID;
}
impl core::ops::Deref for Image {
    type Target = IImage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Image {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Image";
}
unsafe impl Send for Image {}
unsafe impl Sync for Image {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ImageSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ImageSource, DependencyObject);
impl windows_core::RuntimeType for ImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IImageSource>();
}
unsafe impl windows_core::Interface for ImageSource {
    type Vtable = <IImageSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IImageSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for ImageSource {
    type Target = IImageSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.ImageSource";
}
unsafe impl Send for ImageSource {}
unsafe impl Sync for ImageSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inline(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Inline, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Inline, TextElement, DependencyObject);
impl windows_core::RuntimeType for Inline {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInline>();
}
unsafe impl windows_core::Interface for Inline {
    type Vtable = <IInline as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInline as windows_core::Interface>::IID;
}
impl core::ops::Deref for Inline {
    type Target = IInline;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Inline {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Inline";
}
unsafe impl Send for Inline {}
unsafe impl Sync for Inline {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InlineCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IVector<Inline>
);
impl windows_core::RuntimeType for InlineCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<Inline>>();
}
unsafe impl windows_core::Interface for InlineCollection {
    type Vtable = <windows_collections::IVector<Inline> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows_collections::IVector<Inline> as windows_core::Interface>::IID;
}
impl core::ops::Deref for InlineCollection {
    type Target = windows_collections::IVector<Inline>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InlineCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.InlineCollection";
}
unsafe impl Send for InlineCollection {}
unsafe impl Sync for InlineCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ItemCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IObservableVector<windows_core::IInspectable>
);
impl windows_core::RuntimeType for ItemCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        windows_collections::IObservableVector<windows_core::IInspectable>,
    >();
}
unsafe impl windows_core::Interface for ItemCollection {
    type Vtable = < windows_collections::IObservableVector < windows_core::IInspectable > as windows_core::Interface >::Vtable ;
    const IID: windows_core::GUID = <windows_collections::IObservableVector<
        windows_core::IInspectable,
    > as windows_core::Interface>::IID;
}
impl core::ops::Deref for ItemCollection {
    type Target = windows_collections::IObservableVector<windows_core::IInspectable>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ItemCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ItemCollection";
}
unsafe impl Send for ItemCollection {}
unsafe impl Sync for ItemCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemsControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ItemsControl,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl windows_core::RuntimeType for ItemsControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IItemsControl>();
}
unsafe impl windows_core::Interface for ItemsControl {
    type Vtable = <IItemsControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IItemsControl as windows_core::Interface>::IID;
}
impl core::ops::Deref for ItemsControl {
    type Target = IItemsControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ItemsControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ItemsControl";
}
unsafe impl Send for ItemsControl {}
unsafe impl Sync for ItemsControl {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaunchActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    LaunchActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable,
    ILaunchActivatedEventArgs
);
windows_core::imp::required_hierarchy!(LaunchActivatedEventArgs, IActivatedEventArgs);
impl windows_core::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ILaunchActivatedEventArgs>();
}
unsafe impl windows_core::Interface for LaunchActivatedEventArgs {
    type Vtable = <ILaunchActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILaunchActivatedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for LaunchActivatedEventArgs {
    type Target = ILaunchActivatedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
}
unsafe impl Send for LaunchActivatedEventArgs {}
unsafe impl Sync for LaunchActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Line(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Line, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Line, Shape, FrameworkElement, UIElement, DependencyObject);
impl Line {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Line, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Line {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ILine>();
}
unsafe impl windows_core::Interface for Line {
    type Vtable = <ILine as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILine as windows_core::Interface>::IID;
}
impl core::ops::Deref for Line {
    type Target = ILine;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Line {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Line";
}
unsafe impl Send for Line {}
unsafe impl Sync for Line {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ListBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ListBox,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ListBox {
    pub fn new() -> windows_core::Result<ListBox> {
        Self::IListBoxFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<ListBox>
    where
        T: windows_core::Compose,
    {
        Self::IListBoxFactory(|this| unsafe {
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
    fn IListBoxFactory<R, F: FnOnce(&IListBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ListBox, IListBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ListBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IListBox>();
}
unsafe impl windows_core::Interface for ListBox {
    type Vtable = <IListBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IListBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for ListBox {
    type Target = IListBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ListBox {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ListBox";
}
unsafe impl Send for ListBox {}
unsafe impl Sync for ListBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ListView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ListView,
    ListViewBase,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ListView {
    pub fn new() -> windows_core::Result<ListView> {
        Self::IListViewFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<ListView>
    where
        T: windows_core::Compose,
    {
        Self::IListViewFactory(|this| unsafe {
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
    fn IListViewFactory<R, F: FnOnce(&IListViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ListView, IListViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ListView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IListView>();
}
unsafe impl windows_core::Interface for ListView {
    type Vtable = <IListView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IListView as windows_core::Interface>::IID;
}
impl core::ops::Deref for ListView {
    type Target = IListView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ListView {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ListView";
}
unsafe impl Send for ListView {}
unsafe impl Sync for ListView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListViewBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ListViewBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ListViewBase,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl windows_core::RuntimeType for ListViewBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IListViewBase>();
}
unsafe impl windows_core::Interface for ListViewBase {
    type Vtable = <IListViewBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IListViewBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for ListViewBase {
    type Target = IListViewBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ListViewBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ListViewBase";
}
unsafe impl Send for ListViewBase {}
unsafe impl Sync for ListViewBase {}
struct SelectionChangedEventHandlerResultBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> SelectionChangedEventHandlerResultBox<F>
{
    const VTABLE: SelectionChangedEventHandler_Vtbl = SelectionChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<SelectionChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            )
            .into()
        }
    }
}
impl SelectionChangedEventHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SelectionChangedEventArgs>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    {
        let com = windows_core::imp::DelegateBox::<SelectionChangedEventHandler, _>::new(
            &SelectionChangedEventHandlerResultBox::<F>::VTABLE,
            handler,
        );
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ListViewSelectionMode(pub i32);
impl ListViewSelectionMode {
    pub const None: Self = Self(0);
    pub const Single: Self = Self(1);
    pub const Multiple: Self = Self(2);
    pub const Extended: Self = Self(3);
}
impl windows_core::TypeKind for ListViewSelectionMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ListViewSelectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.ListViewSelectionMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MenuBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    MenuBar,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl MenuBar {
    pub fn new() -> windows_core::Result<MenuBar> {
        Self::IMenuBarFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<MenuBar>
    where
        T: windows_core::Compose,
    {
        Self::IMenuBarFactory(|this| unsafe {
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
    fn IMenuBarFactory<R, F: FnOnce(&IMenuBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MenuBar, IMenuBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MenuBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMenuBar>();
}
unsafe impl windows_core::Interface for MenuBar {
    type Vtable = <IMenuBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMenuBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for MenuBar {
    type Target = IMenuBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MenuBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.MenuBar";
}
unsafe impl Send for MenuBar {}
unsafe impl Sync for MenuBar {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Orientation(pub i32);
impl Orientation {
    pub const Vertical: Self = Self(0);
    pub const Horizontal: Self = Self(1);
}
impl windows_core::TypeKind for Orientation {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Orientation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.Orientation;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Panel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Panel, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Panel, FrameworkElement, UIElement, DependencyObject);
impl windows_core::RuntimeType for Panel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPanel>();
}
unsafe impl windows_core::Interface for Panel {
    type Vtable = <IPanel as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPanel as windows_core::Interface>::IID;
}
impl core::ops::Deref for Panel {
    type Target = IPanel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Panel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Panel";
}
unsafe impl Send for Panel {}
unsafe impl Sync for Panel {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Paragraph(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Paragraph,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(Paragraph, Block, TextElement, DependencyObject);
impl Paragraph {
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
            Paragraph,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Paragraph {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IParagraph>();
}
unsafe impl windows_core::Interface for Paragraph {
    type Vtable = <IParagraph as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IParagraph as windows_core::Interface>::IID;
}
impl core::ops::Deref for Paragraph {
    type Target = IParagraph;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Paragraph {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Paragraph";
}
unsafe impl Send for Paragraph {}
unsafe impl Sync for Paragraph {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasswordBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PasswordBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    PasswordBox,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl PasswordBox {
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
            PasswordBox,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PasswordBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPasswordBox>();
}
unsafe impl windows_core::Interface for PasswordBox {
    type Vtable = <IPasswordBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPasswordBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for PasswordBox {
    type Target = IPasswordBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PasswordBox {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.PasswordBox";
}
unsafe impl Send for PasswordBox {}
unsafe impl Sync for PasswordBox {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PasswordRevealMode(pub i32);
impl PasswordRevealMode {
    pub const Peek: Self = Self(0);
    pub const Hidden: Self = Self(1);
    pub const Visible: Self = Self(2);
}
impl windows_core::TypeKind for PasswordRevealMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PasswordRevealMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.PasswordRevealMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersonPicture(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PersonPicture,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    PersonPicture,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl PersonPicture {
    pub fn new() -> windows_core::Result<PersonPicture> {
        Self::IPersonPictureFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<PersonPicture>
    where
        T: windows_core::Compose,
    {
        Self::IPersonPictureFactory(|this| unsafe {
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
    fn IPersonPictureFactory<R, F: FnOnce(&IPersonPictureFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PersonPicture, IPersonPictureFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PersonPicture {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPersonPicture>();
}
unsafe impl windows_core::Interface for PersonPicture {
    type Vtable = <IPersonPicture as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPersonPicture as windows_core::Interface>::IID;
}
impl core::ops::Deref for PersonPicture {
    type Target = IPersonPicture;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PersonPicture {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.PersonPicture";
}
unsafe impl Send for PersonPicture {}
unsafe impl Sync for PersonPicture {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pivot(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Pivot, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(
    Pivot,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Pivot {
    pub fn new() -> windows_core::Result<Pivot> {
        Self::IPivotFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<Pivot>
    where
        T: windows_core::Compose,
    {
        Self::IPivotFactory(|this| unsafe {
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
    fn IPivotFactory<R, F: FnOnce(&IPivotFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Pivot, IPivotFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Pivot {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPivot>();
}
unsafe impl windows_core::Interface for Pivot {
    type Vtable = <IPivot as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPivot as windows_core::Interface>::IID;
}
impl core::ops::Deref for Pivot {
    type Target = IPivot;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Pivot {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Pivot";
}
unsafe impl Send for Pivot {}
unsafe impl Sync for Pivot {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PivotItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PivotItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    PivotItem,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl PivotItem {
    pub fn new() -> windows_core::Result<PivotItem> {
        Self::IPivotItemFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<PivotItem>
    where
        T: windows_core::Compose,
    {
        Self::IPivotItemFactory(|this| unsafe {
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
    fn IPivotItemFactory<R, F: FnOnce(&IPivotItemFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PivotItem, IPivotItemFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PivotItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPivotItem>();
}
unsafe impl windows_core::Interface for PivotItem {
    type Vtable = <IPivotItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPivotItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for PivotItem {
    type Target = IPivotItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PivotItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.PivotItem";
}
unsafe impl Send for PivotItem {}
unsafe impl Sync for PivotItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgressBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ProgressBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ProgressBar,
    RangeBase,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ProgressBar {
    pub fn new() -> windows_core::Result<ProgressBar> {
        Self::IProgressBarFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<ProgressBar>
    where
        T: windows_core::Compose,
    {
        Self::IProgressBarFactory(|this| unsafe {
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
    fn IProgressBarFactory<R, F: FnOnce(&IProgressBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProgressBar, IProgressBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProgressBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IProgressBar>();
}
unsafe impl windows_core::Interface for ProgressBar {
    type Vtable = <IProgressBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProgressBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for ProgressBar {
    type Target = IProgressBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ProgressBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ProgressBar";
}
unsafe impl Send for ProgressBar {}
unsafe impl Sync for ProgressBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgressRing(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ProgressRing,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ProgressRing,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ProgressRing {
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
            ProgressRing,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProgressRing {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IProgressRing>();
}
unsafe impl windows_core::Interface for ProgressRing {
    type Vtable = <IProgressRing as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProgressRing as windows_core::Interface>::IID;
}
impl core::ops::Deref for ProgressRing {
    type Target = IProgressRing;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ProgressRing {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ProgressRing";
}
unsafe impl Send for ProgressRing {}
unsafe impl Sync for ProgressRing {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadioButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RadioButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RadioButton,
    ToggleButton,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RadioButton {
    pub fn new() -> windows_core::Result<RadioButton> {
        Self::IRadioButtonFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<RadioButton>
    where
        T: windows_core::Compose,
    {
        Self::IRadioButtonFactory(|this| unsafe {
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
    fn IRadioButtonFactory<R, F: FnOnce(&IRadioButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RadioButton, IRadioButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RadioButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRadioButton>();
}
unsafe impl windows_core::Interface for RadioButton {
    type Vtable = <IRadioButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRadioButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for RadioButton {
    type Target = IRadioButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RadioButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.RadioButton";
}
unsafe impl Send for RadioButton {}
unsafe impl Sync for RadioButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangeBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RangeBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RangeBase,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl windows_core::RuntimeType for RangeBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRangeBase>();
}
unsafe impl windows_core::Interface for RangeBase {
    type Vtable = <IRangeBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRangeBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for RangeBase {
    type Target = IRangeBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RangeBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RangeBase";
}
unsafe impl Send for RangeBase {}
unsafe impl Sync for RangeBase {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangeBaseValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RangeBaseValueChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(RangeBaseValueChangedEventArgs, RoutedEventArgs);
impl windows_core::RuntimeType for RangeBaseValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRangeBaseValueChangedEventArgs>();
}
unsafe impl windows_core::Interface for RangeBaseValueChangedEventArgs {
    type Vtable = <IRangeBaseValueChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IRangeBaseValueChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for RangeBaseValueChangedEventArgs {
    type Target = IRangeBaseValueChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RangeBaseValueChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RangeBaseValueChangedEventArgs";
}
unsafe impl Send for RangeBaseValueChangedEventArgs {}
unsafe impl Sync for RangeBaseValueChangedEventArgs {}
windows_core::imp::define_interface!(
    RangeBaseValueChangedEventHandler,
    RangeBaseValueChangedEventHandler_Vtbl,
    0xe3906fd9_4d1b_4ac8_a43c_c3b908742799
);
impl windows_core::RuntimeType for RangeBaseValueChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct RangeBaseValueChangedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct RangeBaseValueChangedEventHandlerBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<RangeBaseValueChangedEventArgs>,
        ) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<RangeBaseValueChangedEventArgs>,
        ) + 'static,
> RangeBaseValueChangedEventHandlerBox<F>
{
    const VTABLE: RangeBaseValueChangedEventHandler_Vtbl = RangeBaseValueChangedEventHandler_Vtbl {
        base__:
            windows_core::IUnknown_Vtbl {
                QueryInterface: windows_core::imp::DelegateBox::<
                    RangeBaseValueChangedEventHandler,
                    F,
                >::QueryInterface,
                AddRef:
                    windows_core::imp::DelegateBox::<RangeBaseValueChangedEventHandler, F>::AddRef,
                Release:
                    windows_core::imp::DelegateBox::<RangeBaseValueChangedEventHandler, F>::Release,
            },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<RangeBaseValueChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RatingControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RatingControl,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RatingControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RatingControl {
    pub fn new() -> windows_core::Result<RatingControl> {
        Self::IRatingControlFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<RatingControl>
    where
        T: windows_core::Compose,
    {
        Self::IRatingControlFactory(|this| unsafe {
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
    fn IRatingControlFactory<R, F: FnOnce(&IRatingControlFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RatingControl, IRatingControlFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RatingControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRatingControl>();
}
unsafe impl windows_core::Interface for RatingControl {
    type Vtable = <IRatingControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRatingControl as windows_core::Interface>::IID;
}
impl core::ops::Deref for RatingControl {
    type Target = IRatingControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RatingControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.RatingControl";
}
unsafe impl Send for RatingControl {}
unsafe impl Sync for RatingControl {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rectangle(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Rectangle,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    Rectangle,
    Shape,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Rectangle {
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
            Rectangle,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Rectangle {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRectangle>();
}
unsafe impl windows_core::Interface for Rectangle {
    type Vtable = <IRectangle as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRectangle as windows_core::Interface>::IID;
}
impl core::ops::Deref for Rectangle {
    type Target = IRectangle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Rectangle {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Rectangle";
}
unsafe impl Send for Rectangle {}
unsafe impl Sync for Rectangle {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelativePanel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RelativePanel,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RelativePanel,
    Panel,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RelativePanel {
    pub fn new() -> windows_core::Result<RelativePanel> {
        Self::IRelativePanelFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<RelativePanel>
    where
        T: windows_core::Compose,
    {
        Self::IRelativePanelFactory(|this| unsafe {
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
    pub fn SetAlignLeftWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignLeftWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetAlignTopWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignTopWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetAlignRightWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignRightWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetAlignBottomWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignBottomWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetAlignHorizontalCenterWithPanel<P0>(
        element: P0,
        value: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignHorizontalCenterWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetAlignVerticalCenterWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignVerticalCenterWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    fn IRelativePanelFactory<R, F: FnOnce(&IRelativePanelFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RelativePanel, IRelativePanelFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IRelativePanelStatics<R, F: FnOnce(&IRelativePanelStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RelativePanel, IRelativePanelStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RelativePanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRelativePanel>();
}
unsafe impl windows_core::Interface for RelativePanel {
    type Vtable = <IRelativePanel as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRelativePanel as windows_core::Interface>::IID;
}
impl core::ops::Deref for RelativePanel {
    type Target = IRelativePanel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RelativePanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.RelativePanel";
}
unsafe impl Send for RelativePanel {}
unsafe impl Sync for RelativePanel {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepeatButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RepeatButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RepeatButton,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RepeatButton {
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
            RepeatButton,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RepeatButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRepeatButton>();
}
unsafe impl windows_core::Interface for RepeatButton {
    type Vtable = <IRepeatButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRepeatButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for RepeatButton {
    type Target = IRepeatButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RepeatButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RepeatButton";
}
unsafe impl Send for RepeatButton {}
unsafe impl Sync for RepeatButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceDictionary(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ResourceDictionary,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ResourceDictionary, DependencyObject);
impl ResourceDictionary {
    pub fn new() -> windows_core::Result<ResourceDictionary> {
        Self::IResourceDictionaryFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<ResourceDictionary>
    where
        T: windows_core::Compose,
    {
        Self::IResourceDictionaryFactory(|this| unsafe {
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
    fn IResourceDictionaryFactory<
        R,
        F: FnOnce(&IResourceDictionaryFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ResourceDictionary,
            IResourceDictionaryFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ResourceDictionary {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IResourceDictionary>();
}
unsafe impl windows_core::Interface for ResourceDictionary {
    type Vtable = <IResourceDictionary as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IResourceDictionary as windows_core::Interface>::IID;
}
impl core::ops::Deref for ResourceDictionary {
    type Target = IResourceDictionary;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ResourceDictionary {
    const NAME: &'static str = "Windows.UI.Xaml.ResourceDictionary";
}
unsafe impl Send for ResourceDictionary {}
unsafe impl Sync for ResourceDictionary {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ResourceManager,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ResourceManager {
    pub fn get_Current() -> windows_core::Result<ResourceManager> {
        Self::IResourceManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_Current)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IResourceManagerStatics<
        R,
        F: FnOnce(&IResourceManagerStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceManager, IResourceManagerStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ResourceManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IResourceManager>();
}
unsafe impl windows_core::Interface for ResourceManager {
    type Vtable = <IResourceManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IResourceManager as windows_core::Interface>::IID;
}
impl core::ops::Deref for ResourceManager {
    type Target = IResourceManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ResourceManager {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceManager";
}
unsafe impl Send for ResourceManager {}
unsafe impl Sync for ResourceManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichEditBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RichEditBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RichEditBox,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RichEditBox {
    pub fn new() -> windows_core::Result<RichEditBox> {
        Self::IRichEditBoxFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<RichEditBox>
    where
        T: windows_core::Compose,
    {
        Self::IRichEditBoxFactory(|this| unsafe {
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
    fn IRichEditBoxFactory<R, F: FnOnce(&IRichEditBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RichEditBox, IRichEditBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RichEditBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRichEditBox>();
}
unsafe impl windows_core::Interface for RichEditBox {
    type Vtable = <IRichEditBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRichEditBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for RichEditBox {
    type Target = IRichEditBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RichEditBox {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.RichEditBox";
}
unsafe impl Send for RichEditBox {}
unsafe impl Sync for RichEditBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichTextBlock(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RichTextBlock,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RichTextBlock,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RichTextBlock {
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
            RichTextBlock,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RichTextBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRichTextBlock>();
}
unsafe impl windows_core::Interface for RichTextBlock {
    type Vtable = <IRichTextBlock as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRichTextBlock as windows_core::Interface>::IID;
}
impl core::ops::Deref for RichTextBlock {
    type Target = IRichTextBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RichTextBlock {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.RichTextBlock";
}
unsafe impl Send for RichTextBlock {}
unsafe impl Sync for RichTextBlock {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RoutedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for RoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRoutedEventArgs>();
}
unsafe impl windows_core::Interface for RoutedEventArgs {
    type Vtable = <IRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRoutedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for RoutedEventArgs {
    type Target = IRoutedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.RoutedEventArgs";
}
unsafe impl Send for RoutedEventArgs {}
unsafe impl Sync for RoutedEventArgs {}
windows_core::imp::define_interface!(
    RoutedEventHandler,
    RoutedEventHandler_Vtbl,
    0xa856e674_b0b6_4bc3_bba8_1ba06e40d4b5
);
impl windows_core::RuntimeType for RoutedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct RoutedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct RoutedEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) + 'static,
> RoutedEventHandlerBox<F>
{
    const VTABLE: RoutedEventHandler_Vtbl = RoutedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<RoutedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RowDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RowDefinition,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(RowDefinition, DependencyObject);
impl RowDefinition {
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
            RowDefinition,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RowDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRowDefinition>();
}
unsafe impl windows_core::Interface for RowDefinition {
    type Vtable = <IRowDefinition as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRowDefinition as windows_core::Interface>::IID;
}
impl core::ops::Deref for RowDefinition {
    type Target = IRowDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RowDefinition {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.RowDefinition";
}
unsafe impl Send for RowDefinition {}
unsafe impl Sync for RowDefinition {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RowDefinitionCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RowDefinitionCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IVector<RowDefinition>
);
impl windows_core::RuntimeType for RowDefinitionCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        windows_collections::IVector<RowDefinition>,
    >();
}
unsafe impl windows_core::Interface for RowDefinitionCollection {
    type Vtable = <windows_collections::IVector<RowDefinition> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows_collections::IVector<RowDefinition> as windows_core::Interface>::IID;
}
impl core::ops::Deref for RowDefinitionCollection {
    type Target = windows_collections::IVector<RowDefinition>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RowDefinitionCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.RowDefinitionCollection";
}
unsafe impl Send for RowDefinitionCollection {}
unsafe impl Sync for RowDefinitionCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Run(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Run, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Run, Inline, TextElement, DependencyObject);
impl Run {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Run, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Run {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRun>();
}
unsafe impl windows_core::Interface for Run {
    type Vtable = <IRun as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRun as windows_core::Interface>::IID;
}
impl core::ops::Deref for Run {
    type Target = IRun;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Run {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Run";
}
unsafe impl Send for Run {}
unsafe impl Sync for Run {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScrollBarVisibility(pub i32);
impl ScrollBarVisibility {
    pub const Disabled: Self = Self(0);
    pub const Auto: Self = Self(1);
    pub const Hidden: Self = Self(2);
    pub const Visible: Self = Self(3);
}
impl windows_core::TypeKind for ScrollBarVisibility {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ScrollBarVisibility {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.ScrollBarVisibility;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScrollViewer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ScrollViewer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ScrollViewer,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ScrollViewer {
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
            ScrollViewer,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ScrollViewer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IScrollViewer>();
}
unsafe impl windows_core::Interface for ScrollViewer {
    type Vtable = <IScrollViewer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IScrollViewer as windows_core::Interface>::IID;
}
impl core::ops::Deref for ScrollViewer {
    type Target = IScrollViewer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ScrollViewer {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ScrollViewer";
}
unsafe impl Send for ScrollViewer {}
unsafe impl Sync for ScrollViewer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SearchActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable,
    ISearchActivatedEventArgs
);
windows_core::imp::required_hierarchy!(SearchActivatedEventArgs, IActivatedEventArgs);
impl windows_core::RuntimeType for SearchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISearchActivatedEventArgs>();
}
unsafe impl windows_core::Interface for SearchActivatedEventArgs {
    type Vtable = <ISearchActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchActivatedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for SearchActivatedEventArgs {
    type Target = ISearchActivatedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SearchActivatedEventArgs";
}
unsafe impl Send for SearchActivatedEventArgs {}
unsafe impl Sync for SearchActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SelectionChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SelectionChangedEventArgs, RoutedEventArgs);
impl windows_core::RuntimeType for SelectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISelectionChangedEventArgs>();
}
unsafe impl windows_core::Interface for SelectionChangedEventArgs {
    type Vtable = <ISelectionChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISelectionChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for SelectionChangedEventArgs {
    type Target = ISelectionChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SelectionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.SelectionChangedEventArgs";
}
unsafe impl Send for SelectionChangedEventArgs {}
unsafe impl Sync for SelectionChangedEventArgs {}
windows_core::imp::define_interface!(
    SelectionChangedEventHandler,
    SelectionChangedEventHandler_Vtbl,
    0xe1a05352_5aa0_42ca_9cd9_068a14db6e68
);
impl windows_core::RuntimeType for SelectionChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct SelectionChangedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct SelectionChangedEventHandlerBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) + 'static,
> SelectionChangedEventHandlerBox<F>
{
    const VTABLE: SelectionChangedEventHandler_Vtbl = SelectionChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<SelectionChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selector(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Selector,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl windows_core::RuntimeType for Selector {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISelector>();
}
unsafe impl windows_core::Interface for Selector {
    type Vtable = <ISelector as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISelector as windows_core::Interface>::IID;
}
impl core::ops::Deref for Selector {
    type Target = ISelector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Selector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.Selector";
}
unsafe impl Send for Selector {}
unsafe impl Sync for Selector {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Shape(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Shape, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Shape, FrameworkElement, UIElement, DependencyObject);
impl windows_core::RuntimeType for Shape {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IShape>();
}
unsafe impl windows_core::Interface for Shape {
    type Vtable = <IShape as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IShape as windows_core::Interface>::IID;
}
impl core::ops::Deref for Shape {
    type Target = IShape;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Shape {
    const NAME: &'static str = "Windows.UI.Xaml.Shapes.Shape";
}
unsafe impl Send for Shape {}
unsafe impl Sync for Shape {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShareTargetActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ShareTargetActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IShareTargetActivatedEventArgs
);
windows_core::imp::required_hierarchy!(ShareTargetActivatedEventArgs, IActivatedEventArgs);
impl windows_core::RuntimeType for ShareTargetActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IShareTargetActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ShareTargetActivatedEventArgs {
    type Vtable = <IShareTargetActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IShareTargetActivatedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ShareTargetActivatedEventArgs {
    type Target = IShareTargetActivatedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs";
}
unsafe impl Send for ShareTargetActivatedEventArgs {}
unsafe impl Sync for ShareTargetActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Slider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Slider, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(
    Slider,
    RangeBase,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Slider {
    pub fn new() -> windows_core::Result<Slider> {
        Self::ISliderFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<Slider>
    where
        T: windows_core::Compose,
    {
        Self::ISliderFactory(|this| unsafe {
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
    fn ISliderFactory<R, F: FnOnce(&ISliderFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Slider, ISliderFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Slider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISlider>();
}
unsafe impl windows_core::Interface for Slider {
    type Vtable = <ISlider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISlider as windows_core::Interface>::IID;
}
impl core::ops::Deref for Slider {
    type Target = ISlider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Slider {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Slider";
}
unsafe impl Send for Slider {}
unsafe impl Sync for Slider {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolidColorBrush(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SolidColorBrush,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SolidColorBrush, Brush, DependencyObject);
impl SolidColorBrush {
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
            SolidColorBrush,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SolidColorBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISolidColorBrush>();
}
unsafe impl windows_core::Interface for SolidColorBrush {
    type Vtable = <ISolidColorBrush as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISolidColorBrush as windows_core::Interface>::IID;
}
impl core::ops::Deref for SolidColorBrush {
    type Target = ISolidColorBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SolidColorBrush {
    const NAME: &'static str = "Windows.UI.Xaml.Media.SolidColorBrush";
}
unsafe impl Send for SolidColorBrush {}
unsafe impl Sync for SolidColorBrush {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SplitButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    SplitButton,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl SplitButton {
    pub fn new() -> windows_core::Result<SplitButton> {
        Self::ISplitButtonFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<SplitButton>
    where
        T: windows_core::Compose,
    {
        Self::ISplitButtonFactory(|this| unsafe {
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
    fn ISplitButtonFactory<R, F: FnOnce(&ISplitButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SplitButton, ISplitButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SplitButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISplitButton>();
}
unsafe impl windows_core::Interface for SplitButton {
    type Vtable = <ISplitButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISplitButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for SplitButton {
    type Target = ISplitButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SplitButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.SplitButton";
}
unsafe impl Send for SplitButton {}
unsafe impl Sync for SplitButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitButtonClickEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SplitButtonClickEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for SplitButtonClickEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISplitButtonClickEventArgs>();
}
unsafe impl windows_core::Interface for SplitButtonClickEventArgs {
    type Vtable = <ISplitButtonClickEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISplitButtonClickEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for SplitButtonClickEventArgs {
    type Target = ISplitButtonClickEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SplitButtonClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.SplitButtonClickEventArgs";
}
unsafe impl Send for SplitButtonClickEventArgs {}
unsafe impl Sync for SplitButtonClickEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SplitView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    SplitView,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl SplitView {
    pub fn new() -> windows_core::Result<SplitView> {
        Self::ISplitViewFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<SplitView>
    where
        T: windows_core::Compose,
    {
        Self::ISplitViewFactory(|this| unsafe {
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
    fn ISplitViewFactory<R, F: FnOnce(&ISplitViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SplitView, ISplitViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SplitView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISplitView>();
}
unsafe impl windows_core::Interface for SplitView {
    type Vtable = <ISplitView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISplitView as windows_core::Interface>::IID;
}
impl core::ops::Deref for SplitView {
    type Target = ISplitView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SplitView {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.SplitView";
}
unsafe impl Send for SplitView {}
unsafe impl Sync for SplitView {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SplitViewDisplayMode(pub i32);
impl SplitViewDisplayMode {
    pub const Overlay: Self = Self(0);
    pub const Inline: Self = Self(1);
    pub const CompactOverlay: Self = Self(2);
    pub const CompactInline: Self = Self(3);
}
impl windows_core::TypeKind for SplitViewDisplayMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SplitViewDisplayMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.SplitViewDisplayMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackPanel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    StackPanel,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    StackPanel,
    Panel,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl StackPanel {
    pub fn new() -> windows_core::Result<StackPanel> {
        Self::IStackPanelFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<StackPanel>
    where
        T: windows_core::Compose,
    {
        Self::IStackPanelFactory(|this| unsafe {
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
    fn IStackPanelFactory<R, F: FnOnce(&IStackPanelFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StackPanel, IStackPanelFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StackPanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IStackPanel>();
}
unsafe impl windows_core::Interface for StackPanel {
    type Vtable = <IStackPanel as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStackPanel as windows_core::Interface>::IID;
}
impl core::ops::Deref for StackPanel {
    type Target = IStackPanel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for StackPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.StackPanel";
}
unsafe impl Send for StackPanel {}
unsafe impl Sync for StackPanel {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageFile(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    StorageFile,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IStorageFile
);
impl StorageFile {
    pub fn GetFileFromPathAsync(
        path: &str,
    ) -> windows_core::Result<windows_future::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileFromPathAsync)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(path)),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IStorageFileStatics<R, F: FnOnce(&IStorageFileStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageFile, IStorageFileStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StorageFile {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IStorageFile>();
}
unsafe impl windows_core::Interface for StorageFile {
    type Vtable = <IStorageFile as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageFile as windows_core::Interface>::IID;
}
impl core::ops::Deref for StorageFile {
    type Target = IStorageFile;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for StorageFile {
    const NAME: &'static str = "Windows.Storage.StorageFile";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Stretch(pub i32);
impl Stretch {
    pub const None: Self = Self(0);
    pub const Fill: Self = Self(1);
    pub const Uniform: Self = Self(2);
    pub const UniformToFill: Self = Self(3);
}
impl windows_core::TypeKind for Stretch {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Stretch {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Stretch;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Style(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Style, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Style, DependencyObject);
impl Style {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Style, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Style {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IStyle>();
}
unsafe impl windows_core::Interface for Style {
    type Vtable = <IStyle as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStyle as windows_core::Interface>::IID;
}
impl core::ops::Deref for Style {
    type Target = IStyle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Style {
    const NAME: &'static str = "Windows.UI.Xaml.Style";
}
unsafe impl Send for Style {}
unsafe impl Sync for Style {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Symbol(pub i32);
impl Symbol {
    pub const Previous: Self = Self(57600);
    pub const Next: Self = Self(57601);
    pub const Play: Self = Self(57602);
    pub const Pause: Self = Self(57603);
    pub const Edit: Self = Self(57604);
    pub const Save: Self = Self(57605);
    pub const Clear: Self = Self(57606);
    pub const Delete: Self = Self(57607);
    pub const Remove: Self = Self(57608);
    pub const Add: Self = Self(57609);
    pub const Cancel: Self = Self(57610);
    pub const Accept: Self = Self(57611);
    pub const More: Self = Self(57612);
    pub const Redo: Self = Self(57613);
    pub const Undo: Self = Self(57614);
    pub const Home: Self = Self(57615);
    pub const Up: Self = Self(57616);
    pub const Forward: Self = Self(57617);
    pub const Back: Self = Self(57618);
    pub const Favorite: Self = Self(57619);
    pub const Camera: Self = Self(57620);
    pub const Setting: Self = Self(57621);
    pub const Video: Self = Self(57622);
    pub const Sync: Self = Self(57623);
    pub const Download: Self = Self(57624);
    pub const Mail: Self = Self(57625);
    pub const Find: Self = Self(57626);
    pub const Help: Self = Self(57627);
    pub const Upload: Self = Self(57628);
    pub const Emoji: Self = Self(57629);
    pub const TwoPage: Self = Self(57630);
    pub const LeaveChat: Self = Self(57631);
    pub const MailForward: Self = Self(57632);
    pub const Clock: Self = Self(57633);
    pub const Send: Self = Self(57634);
    pub const Crop: Self = Self(57635);
    pub const RotateCamera: Self = Self(57636);
    pub const People: Self = Self(57637);
    pub const OpenPane: Self = Self(57638);
    pub const ClosePane: Self = Self(57639);
    pub const World: Self = Self(57640);
    pub const Flag: Self = Self(57641);
    pub const PreviewLink: Self = Self(57642);
    pub const Globe: Self = Self(57643);
    pub const Trim: Self = Self(57644);
    pub const AttachCamera: Self = Self(57645);
    pub const ZoomIn: Self = Self(57646);
    pub const Bookmarks: Self = Self(57647);
    pub const Document: Self = Self(57648);
    pub const ProtectedDocument: Self = Self(57649);
    pub const Page: Self = Self(57650);
    pub const Bullets: Self = Self(57651);
    pub const Comment: Self = Self(57652);
    pub const MailFilled: Self = Self(57653);
    pub const ContactInfo: Self = Self(57654);
    pub const HangUp: Self = Self(57655);
    pub const ViewAll: Self = Self(57656);
    pub const MapPin: Self = Self(57657);
    pub const Phone: Self = Self(57658);
    pub const VideoChat: Self = Self(57659);
    pub const Switch: Self = Self(57660);
    pub const Contact: Self = Self(57661);
    pub const Rename: Self = Self(57662);
    pub const Pin: Self = Self(57665);
    pub const MusicInfo: Self = Self(57666);
    pub const Go: Self = Self(57667);
    pub const Keyboard: Self = Self(57668);
    pub const DockLeft: Self = Self(57669);
    pub const DockRight: Self = Self(57670);
    pub const DockBottom: Self = Self(57671);
    pub const Remote: Self = Self(57672);
    pub const Refresh: Self = Self(57673);
    pub const Rotate: Self = Self(57674);
    pub const Shuffle: Self = Self(57675);
    pub const List: Self = Self(57676);
    pub const Shop: Self = Self(57677);
    pub const SelectAll: Self = Self(57678);
    pub const Orientation: Self = Self(57679);
    pub const Import: Self = Self(57680);
    pub const ImportAll: Self = Self(57681);
    pub const BrowsePhotos: Self = Self(57685);
    pub const WebCam: Self = Self(57686);
    pub const Pictures: Self = Self(57688);
    pub const SaveLocal: Self = Self(57689);
    pub const Caption: Self = Self(57690);
    pub const Stop: Self = Self(57691);
    pub const ShowResults: Self = Self(57692);
    pub const Volume: Self = Self(57693);
    pub const Repair: Self = Self(57694);
    pub const Message: Self = Self(57695);
    pub const Page2: Self = Self(57696);
    pub const CalendarDay: Self = Self(57697);
    pub const CalendarWeek: Self = Self(57698);
    pub const Calendar: Self = Self(57699);
    pub const Character: Self = Self(57700);
    pub const MailReplyAll: Self = Self(57701);
    pub const Read: Self = Self(57702);
    pub const Link: Self = Self(57703);
    pub const Account: Self = Self(57704);
    pub const ShowBcc: Self = Self(57705);
    pub const HideBcc: Self = Self(57706);
    pub const Cut: Self = Self(57707);
    pub const Attach: Self = Self(57708);
    pub const Paste: Self = Self(57709);
    pub const Filter: Self = Self(57710);
    pub const Copy: Self = Self(57711);
    pub const Emoji2: Self = Self(57712);
    pub const Important: Self = Self(57713);
    pub const MailReply: Self = Self(57714);
    pub const SlideShow: Self = Self(57715);
    pub const Sort: Self = Self(57716);
    pub const Manage: Self = Self(57720);
    pub const AllApps: Self = Self(57721);
    pub const DisconnectDrive: Self = Self(57722);
    pub const MapDrive: Self = Self(57723);
    pub const NewWindow: Self = Self(57724);
    pub const OpenWith: Self = Self(57725);
    pub const ContactPresence: Self = Self(57729);
    pub const Priority: Self = Self(57730);
    pub const GoToToday: Self = Self(57732);
    pub const Font: Self = Self(57733);
    pub const FontColor: Self = Self(57734);
    pub const Contact2: Self = Self(57735);
    pub const Folder: Self = Self(57736);
    pub const Audio: Self = Self(57737);
    pub const Placeholder: Self = Self(57738);
    pub const View: Self = Self(57739);
    pub const SetLockScreen: Self = Self(57740);
    pub const SetTile: Self = Self(57741);
    pub const ClosedCaption: Self = Self(57744);
    pub const StopSlideShow: Self = Self(57745);
    pub const Permissions: Self = Self(57746);
    pub const Highlight: Self = Self(57747);
    pub const DisableUpdates: Self = Self(57748);
    pub const UnFavorite: Self = Self(57749);
    pub const UnPin: Self = Self(57750);
    pub const OpenLocal: Self = Self(57751);
    pub const Mute: Self = Self(57752);
    pub const Italic: Self = Self(57753);
    pub const Underline: Self = Self(57754);
    pub const Bold: Self = Self(57755);
    pub const MoveToFolder: Self = Self(57756);
    pub const LikeDislike: Self = Self(57757);
    pub const Dislike: Self = Self(57758);
    pub const Like: Self = Self(57759);
    pub const AlignRight: Self = Self(57760);
    pub const AlignCenter: Self = Self(57761);
    pub const AlignLeft: Self = Self(57762);
    pub const Zoom: Self = Self(57763);
    pub const ZoomOut: Self = Self(57764);
    pub const OpenFile: Self = Self(57765);
    pub const OtherUser: Self = Self(57766);
    pub const Admin: Self = Self(57767);
    pub const Street: Self = Self(57795);
    pub const Map: Self = Self(57796);
    pub const ClearSelection: Self = Self(57797);
    pub const FontDecrease: Self = Self(57798);
    pub const FontIncrease: Self = Self(57799);
    pub const FontSize: Self = Self(57800);
    pub const CellPhone: Self = Self(57801);
    pub const ReShare: Self = Self(57802);
    pub const Tag: Self = Self(57803);
    pub const RepeatOne: Self = Self(57804);
    pub const RepeatAll: Self = Self(57805);
    pub const OutlineStar: Self = Self(57806);
    pub const SolidStar: Self = Self(57807);
    pub const Calculator: Self = Self(57808);
    pub const Directions: Self = Self(57809);
    pub const Target: Self = Self(57810);
    pub const Library: Self = Self(57811);
    pub const PhoneBook: Self = Self(57812);
    pub const Memo: Self = Self(57813);
    pub const Microphone: Self = Self(57814);
    pub const PostUpdate: Self = Self(57815);
    pub const BackToWindow: Self = Self(57816);
    pub const FullScreen: Self = Self(57817);
    pub const NewFolder: Self = Self(57818);
    pub const CalendarReply: Self = Self(57819);
    pub const UnSyncFolder: Self = Self(57821);
    pub const ReportHacked: Self = Self(57822);
    pub const SyncFolder: Self = Self(57823);
    pub const BlockContact: Self = Self(57824);
    pub const SwitchApps: Self = Self(57825);
    pub const AddFriend: Self = Self(57826);
    pub const TouchPointer: Self = Self(57827);
    pub const GoToStart: Self = Self(57828);
    pub const ZeroBars: Self = Self(57829);
    pub const OneBar: Self = Self(57830);
    pub const TwoBars: Self = Self(57831);
    pub const ThreeBars: Self = Self(57832);
    pub const FourBars: Self = Self(57833);
    pub const Scan: Self = Self(58004);
    pub const Preview: Self = Self(58005);
    pub const GlobalNavigationButton: Self = Self(59136);
    pub const Share: Self = Self(59181);
    pub const Print: Self = Self(59209);
    pub const XboxOneConsole: Self = Self(59792);
}
impl windows_core::TypeKind for Symbol {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Symbol {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Symbol;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolIcon(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SymbolIcon,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    SymbolIcon,
    IconElement,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl SymbolIcon {
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
            SymbolIcon,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CreateInstanceWithSymbol(symbol: Symbol) -> windows_core::Result<SymbolIcon> {
        Self::ISymbolIconFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithSymbol)(
                windows_core::Interface::as_raw(this),
                symbol,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISymbolIconFactory<R, F: FnOnce(&ISymbolIconFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SymbolIcon, ISymbolIconFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SymbolIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISymbolIcon>();
}
unsafe impl windows_core::Interface for SymbolIcon {
    type Vtable = <ISymbolIcon as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISymbolIcon as windows_core::Interface>::IID;
}
impl core::ops::Deref for SymbolIcon {
    type Target = ISymbolIcon;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SymbolIcon {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.SymbolIcon";
}
unsafe impl Send for SymbolIcon {}
unsafe impl Sync for SymbolIcon {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextBlock(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TextBlock,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(TextBlock, FrameworkElement, UIElement, DependencyObject);
impl TextBlock {
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
            TextBlock,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TextBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextBlock>();
}
unsafe impl windows_core::Interface for TextBlock {
    type Vtable = <ITextBlock as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextBlock as windows_core::Interface>::IID;
}
impl core::ops::Deref for TextBlock {
    type Target = ITextBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TextBlock {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.TextBlock";
}
unsafe impl Send for TextBlock {}
unsafe impl Sync for TextBlock {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TextBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    TextBox,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl TextBox {
    pub fn new() -> windows_core::Result<TextBox> {
        Self::ITextBoxFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<TextBox>
    where
        T: windows_core::Compose,
    {
        Self::ITextBoxFactory(|this| unsafe {
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
    fn ITextBoxFactory<R, F: FnOnce(&ITextBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TextBox, ITextBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TextBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextBox>();
}
unsafe impl windows_core::Interface for TextBox {
    type Vtable = <ITextBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for TextBox {
    type Target = ITextBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TextBox {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.TextBox";
}
unsafe impl Send for TextBox {}
unsafe impl Sync for TextBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TextChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(TextChangedEventArgs, RoutedEventArgs);
impl windows_core::RuntimeType for TextChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextChangedEventArgs>();
}
unsafe impl windows_core::Interface for TextChangedEventArgs {
    type Vtable = <ITextChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TextChangedEventArgs {
    type Target = ITextChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TextChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.TextChangedEventArgs";
}
unsafe impl Send for TextChangedEventArgs {}
unsafe impl Sync for TextChangedEventArgs {}
windows_core::imp::define_interface!(
    TextChangedEventHandler,
    TextChangedEventHandler_Vtbl,
    0x8eb35b97_ad87_40e8_818b_77db24759566
);
impl windows_core::RuntimeType for TextChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct TextChangedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct TextChangedEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<TextChangedEventArgs>)
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<TextChangedEventArgs>)
        + 'static,
> TextChangedEventHandlerBox<F>
{
    const VTABLE: TextChangedEventHandler_Vtbl = TextChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<TextChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<TextChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<TextChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<TextChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TextElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(TextElement, DependencyObject);
impl windows_core::RuntimeType for TextElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextElement>();
}
unsafe impl windows_core::Interface for TextElement {
    type Vtable = <ITextElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextElement as windows_core::Interface>::IID;
}
impl core::ops::Deref for TextElement {
    type Target = ITextElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TextElement {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.TextElement";
}
unsafe impl Send for TextElement {}
unsafe impl Sync for TextElement {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextWrapping(pub i32);
impl TextWrapping {
    pub const NoWrap: Self = Self(1);
    pub const Wrap: Self = Self(2);
    pub const WrapWholeWords: Self = Self(3);
}
impl windows_core::TypeKind for TextWrapping {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextWrapping {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.TextWrapping;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Thickness {
    pub Left: f64,
    pub Top: f64,
    pub Right: f64,
    pub Bottom: f64,
}
impl windows_core::TypeKind for Thickness {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Thickness {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Xaml.Thickness;f8;f8;f8;f8)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimePicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TimePicker,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    TimePicker,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl TimePicker {
    pub fn new() -> windows_core::Result<TimePicker> {
        Self::ITimePickerFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<TimePicker>
    where
        T: windows_core::Compose,
    {
        Self::ITimePickerFactory(|this| unsafe {
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
    fn ITimePickerFactory<R, F: FnOnce(&ITimePickerFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TimePicker, ITimePickerFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TimePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITimePicker>();
}
unsafe impl windows_core::Interface for TimePicker {
    type Vtable = <ITimePicker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITimePicker as windows_core::Interface>::IID;
}
impl core::ops::Deref for TimePicker {
    type Target = ITimePicker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TimePicker {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.TimePicker";
}
unsafe impl Send for TimePicker {}
unsafe impl Sync for TimePicker {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimePickerValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TimePickerValueChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for TimePickerValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITimePickerValueChangedEventArgs>();
}
unsafe impl windows_core::Interface for TimePickerValueChangedEventArgs {
    type Vtable = <ITimePickerValueChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ITimePickerValueChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TimePickerValueChangedEventArgs {
    type Target = ITimePickerValueChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TimePickerValueChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.TimePickerValueChangedEventArgs";
}
unsafe impl Send for TimePickerValueChangedEventArgs {}
unsafe impl Sync for TimePickerValueChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToggleButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ToggleButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ToggleButton,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ToggleButton {
    pub fn new() -> windows_core::Result<ToggleButton> {
        Self::IToggleButtonFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<ToggleButton>
    where
        T: windows_core::Compose,
    {
        Self::IToggleButtonFactory(|this| unsafe {
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
    fn IToggleButtonFactory<R, F: FnOnce(&IToggleButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToggleButton, IToggleButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToggleButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IToggleButton>();
}
unsafe impl windows_core::Interface for ToggleButton {
    type Vtable = <IToggleButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToggleButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for ToggleButton {
    type Target = IToggleButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ToggleButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ToggleButton";
}
unsafe impl Send for ToggleButton {}
unsafe impl Sync for ToggleButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToggleSwitch(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ToggleSwitch,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ToggleSwitch,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ToggleSwitch {
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
            ToggleSwitch,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToggleSwitch {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IToggleSwitch>();
}
unsafe impl windows_core::Interface for ToggleSwitch {
    type Vtable = <IToggleSwitch as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToggleSwitch as windows_core::Interface>::IID;
}
impl core::ops::Deref for ToggleSwitch {
    type Target = IToggleSwitch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ToggleSwitch {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ToggleSwitch";
}
unsafe impl Send for ToggleSwitch {}
unsafe impl Sync for ToggleSwitch {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolTipService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ToolTipService,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ToolTipService {
    pub fn SetToolTip<P0, P1>(element: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        Self::IToolTipServiceStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetToolTip)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value.param().abi(),
            )
            .ok()
        })
    }
    fn IToolTipServiceStatics<R, F: FnOnce(&IToolTipServiceStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToolTipService, IToolTipServiceStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToolTipService {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IToolTipService>();
}
unsafe impl windows_core::Interface for ToolTipService {
    type Vtable = <IToolTipService as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToolTipService as windows_core::Interface>::IID;
}
impl core::ops::Deref for ToolTipService {
    type Target = IToolTipService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ToolTipService {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.ToolTipService";
}
unsafe impl Send for ToolTipService {}
unsafe impl Sync for ToolTipService {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TreeView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    TreeView,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl TreeView {
    pub fn new() -> windows_core::Result<TreeView> {
        Self::ITreeViewFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<TreeView>
    where
        T: windows_core::Compose,
    {
        Self::ITreeViewFactory(|this| unsafe {
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
    fn ITreeViewFactory<R, F: FnOnce(&ITreeViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TreeView, ITreeViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TreeView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITreeView>();
}
unsafe impl windows_core::Interface for TreeView {
    type Vtable = <ITreeView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITreeView as windows_core::Interface>::IID;
}
impl core::ops::Deref for TreeView {
    type Target = ITreeView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TreeView {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.TreeView";
}
unsafe impl Send for TreeView {}
unsafe impl Sync for TreeView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeViewItemInvokedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TreeViewItemInvokedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for TreeViewItemInvokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITreeViewItemInvokedEventArgs>();
}
unsafe impl windows_core::Interface for TreeViewItemInvokedEventArgs {
    type Vtable = <ITreeViewItemInvokedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITreeViewItemInvokedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TreeViewItemInvokedEventArgs {
    type Target = ITreeViewItemInvokedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TreeViewItemInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.TreeViewItemInvokedEventArgs";
}
unsafe impl Send for TreeViewItemInvokedEventArgs {}
unsafe impl Sync for TreeViewItemInvokedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TreeViewSelectionMode(pub i32);
impl TreeViewSelectionMode {
    pub const None: Self = Self(0);
    pub const Single: Self = Self(1);
    pub const Multiple: Self = Self(2);
}
impl windows_core::TypeKind for TreeViewSelectionMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TreeViewSelectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.Xaml.Controls.TreeViewSelectionMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TypeKind(pub i32);
impl TypeKind {
    pub const Primitive: Self = Self(0);
    pub const Metadata: Self = Self(1);
    pub const Custom: Self = Self(2);
}
impl windows_core::TypeKind for TypeKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TypeKind {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Interop.TypeKind;i4)");
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TypeName {
    pub Name: windows_core::HSTRING,
    pub Kind: TypeKind,
}
impl windows_core::TypeKind for TypeName {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for TypeName {
    const SIGNATURE : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"struct(Windows.UI.Xaml.Interop.TypeName;string;enum(Windows.UI.Xaml.Interop.TypeKind;i4))") ;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UIColorType(pub i32);
impl UIColorType {
    pub const Background: Self = Self(0);
    pub const Foreground: Self = Self(1);
    pub const AccentDark3: Self = Self(2);
    pub const AccentDark2: Self = Self(3);
    pub const AccentDark1: Self = Self(4);
    pub const Accent: Self = Self(5);
    pub const AccentLight1: Self = Self(6);
    pub const AccentLight2: Self = Self(7);
    pub const AccentLight3: Self = Self(8);
    pub const Complement: Self = Self(9);
}
impl windows_core::TypeKind for UIColorType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for UIColorType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.ViewManagement.UIColorType;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UIElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UIElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(UIElement, DependencyObject);
impl windows_core::RuntimeType for UIElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUIElement>();
}
unsafe impl windows_core::Interface for UIElement {
    type Vtable = <IUIElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUIElement as windows_core::Interface>::IID;
}
impl core::ops::Deref for UIElement {
    type Target = IUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for UIElement {
    const NAME: &'static str = "Windows.UI.Xaml.UIElement";
}
unsafe impl Send for UIElement {}
unsafe impl Sync for UIElement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UIElementCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UIElementCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IVector<UIElement>
);
impl windows_core::RuntimeType for UIElementCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<UIElement>>(
        );
}
unsafe impl windows_core::Interface for UIElementCollection {
    type Vtable = <windows_collections::IVector<UIElement> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows_collections::IVector<UIElement> as windows_core::Interface>::IID;
}
impl core::ops::Deref for UIElementCollection {
    type Target = windows_collections::IVector<UIElement>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for UIElementCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.UIElementCollection";
}
unsafe impl Send for UIElementCollection {}
unsafe impl Sync for UIElementCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UISettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UISettings,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl UISettings {
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
            UISettings,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UISettings {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUISettings>();
}
unsafe impl windows_core::Interface for UISettings {
    type Vtable = <IUISettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUISettings as windows_core::Interface>::IID;
}
impl core::ops::Deref for UISettings {
    type Target = IUISettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for UISettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettings";
}
unsafe impl Send for UISettings {}
unsafe impl Sync for UISettings {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnhandledExceptionEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UnhandledExceptionEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for UnhandledExceptionEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUnhandledExceptionEventArgs>();
}
unsafe impl windows_core::Interface for UnhandledExceptionEventArgs {
    type Vtable = <IUnhandledExceptionEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUnhandledExceptionEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for UnhandledExceptionEventArgs {
    type Target = IUnhandledExceptionEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for UnhandledExceptionEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.UnhandledExceptionEventArgs";
}
unsafe impl Send for UnhandledExceptionEventArgs {}
unsafe impl Sync for UnhandledExceptionEventArgs {}
windows_core::imp::define_interface!(
    UnhandledExceptionEventHandler,
    UnhandledExceptionEventHandler_Vtbl,
    0x9274e6bd_49a1_4958_beee_d0e19587b6e3
);
impl windows_core::RuntimeType for UnhandledExceptionEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct UnhandledExceptionEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct UnhandledExceptionEventHandlerBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<UnhandledExceptionEventArgs>,
        ) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<UnhandledExceptionEventArgs>,
        ) + 'static,
> UnhandledExceptionEventHandlerBox<F>
{
    const VTABLE: UnhandledExceptionEventHandler_Vtbl = UnhandledExceptionEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<UnhandledExceptionEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<UnhandledExceptionEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<UnhandledExceptionEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<UnhandledExceptionEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VerticalAlignment(pub i32);
impl VerticalAlignment {
    pub const Top: Self = Self(0);
    pub const Center: Self = Self(1);
    pub const Bottom: Self = Self(2);
    pub const Stretch: Self = Self(3);
}
impl windows_core::TypeKind for VerticalAlignment {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for VerticalAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.VerticalAlignment;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Viewbox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Viewbox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(Viewbox, FrameworkElement, UIElement, DependencyObject);
impl Viewbox {
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
            Viewbox,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Viewbox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IViewbox>();
}
unsafe impl windows_core::Interface for Viewbox {
    type Vtable = <IViewbox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IViewbox as windows_core::Interface>::IID;
}
impl core::ops::Deref for Viewbox {
    type Target = IViewbox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Viewbox {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Viewbox";
}
unsafe impl Send for Viewbox {}
unsafe impl Sync for Viewbox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowCreatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WindowCreatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for WindowCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWindowCreatedEventArgs>();
}
unsafe impl windows_core::Interface for WindowCreatedEventArgs {
    type Vtable = <IWindowCreatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowCreatedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for WindowCreatedEventArgs {
    type Target = IWindowCreatedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for WindowCreatedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.WindowCreatedEventArgs";
}
unsafe impl Send for WindowCreatedEventArgs {}
unsafe impl Sync for WindowCreatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsXamlManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WindowsXamlManager,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WindowsXamlManager {
    pub fn InitializeForCurrentThread() -> windows_core::Result<WindowsXamlManager> {
        Self::IWindowsXamlManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitializeForCurrentThread)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWindowsXamlManagerStatics<
        R,
        F: FnOnce(&IWindowsXamlManagerStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            WindowsXamlManager,
            IWindowsXamlManagerStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsXamlManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWindowsXamlManager>();
}
unsafe impl windows_core::Interface for WindowsXamlManager {
    type Vtable = <IWindowsXamlManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowsXamlManager as windows_core::Interface>::IID;
}
impl core::ops::Deref for WindowsXamlManager {
    type Target = IWindowsXamlManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for WindowsXamlManager {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.WindowsXamlManager";
}
unsafe impl Send for WindowsXamlManager {}
unsafe impl Sync for WindowsXamlManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlReader,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl XamlReader {
    pub fn Load(xaml: &str) -> windows_core::Result<windows_core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Load)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(xaml)),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IXamlReaderStatics<R, F: FnOnce(&IXamlReaderStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XamlReader, IXamlReaderStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XamlReader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXamlReader>();
}
unsafe impl windows_core::Interface for XamlReader {
    type Vtable = <IXamlReader as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlReader as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlReader {
    type Target = IXamlReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlReader {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.XamlReader";
}
unsafe impl Send for XamlReader {}
unsafe impl Sync for XamlReader {}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct XmlnsDefinition {
    pub XmlNamespace: windows_core::HSTRING,
    pub Namespace: windows_core::HSTRING,
}
impl windows_core::TypeKind for XmlnsDefinition {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for XmlnsDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Xaml.Markup.XmlnsDefinition;string;string)",
    );
}
