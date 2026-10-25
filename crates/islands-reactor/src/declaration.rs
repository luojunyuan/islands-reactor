pub(crate) mod accessibility {
    /// UI Automation properties applied to every widget kind via
    /// [`Modifiers::accessibility`](super::Modifiers::accessibility).
    #[derive(Clone, Default, Debug, PartialEq, Eq)]
    pub struct AccessibilityModifiers {
        pub automation_name: Option<String>,
        pub automation_id: Option<String>,
        pub help_text: Option<String>,
        pub live_setting: Option<LiveSetting>,
        pub heading_level: Option<HeadingLevel>,
    }

    impl AccessibilityModifiers {
        pub fn is_empty(&self) -> bool {
            self.automation_name.is_none()
                && self.automation_id.is_none()
                && self.help_text.is_none()
                && self.live_setting.is_none()
                && self.heading_level.is_none()
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum LiveSetting {
        Off,
        Polite,
        Assertive,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum HeadingLevel {
        Level1,
        Level2,
        Level3,
        Level4,
        Level5,
        Level6,
        Level7,
        Level8,
        Level9,
    }
}
pub(crate) mod animation {
    use std::time::Duration;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ScalarTransition {
        pub duration: Duration,
    }

    impl ScalarTransition {
        pub const fn new(duration: Duration) -> Self {
            Self { duration }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Vector3Transition {
        pub duration: Duration,
        pub components: Vector3Axes,
    }

    impl Vector3Transition {
        pub const fn new(duration: Duration) -> Self {
            Self {
                duration,
                components: Vector3Axes::All,
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Vector3Axes {
        X,
        Y,
        Z,
        Xy,
        All,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
    pub struct ImplicitTransitions {
        pub opacity: Option<ScalarTransition>,
        pub rotation: Option<ScalarTransition>,
        pub scale: Option<Vector3Transition>,
        pub translation: Option<Vector3Transition>,
    }

    impl ImplicitTransitions {
        pub fn is_empty(&self) -> bool {
            self.opacity.is_none()
                && self.rotation.is_none()
                && self.scale.is_none()
                && self.translation.is_none()
        }
    }

    /// Implicit transitions applied to layout-managed properties of an
    /// element (offset/size). Fed to the backend via `set_layout_animation`.
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct LayoutAnimationConfig {
        pub duration: Duration,
        pub use_spring: bool,
        pub damping_ratio: f32,
        pub period: f32,
        pub animate_offset: bool,
        pub animate_size: bool,
    }

    impl Default for LayoutAnimationConfig {
        fn default() -> Self {
            Self {
                duration: Duration::from_millis(300),
                use_spring: false,
                damping_ratio: 0.6,
                period: 0.08,
                animate_offset: true,
                animate_size: false,
            }
        }
    }

    impl LayoutAnimationConfig {
        pub fn linear(duration: Duration) -> Self {
            Self {
                duration,
                ..Self::default()
            }
        }

        pub fn spring() -> Self {
            Self {
                use_spring: true,
                ..Self::default()
            }
        }

        pub fn animate_size(mut self, v: bool) -> Self {
            self.animate_size = v;
            self
        }

        pub fn animate_offset(mut self, v: bool) -> Self {
            self.animate_offset = v;
            self
        }
    }

    /// One-shot property animation (opacity / scale / …) driven by
    /// `Backend::run_property_animation`.
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct AnimationConfig {
        pub opacity: Option<f64>,
        pub scale: Option<f64>,
        pub duration: Duration,
        pub easing: Easing,
    }

    impl Default for AnimationConfig {
        fn default() -> Self {
            Self {
                opacity: None,
                scale: None,
                duration: Duration::from_millis(300),
                easing: Easing::EaseOut,
            }
        }
    }

    impl AnimationConfig {
        pub fn fade_in(duration: Duration) -> Self {
            Self {
                opacity: Some(1.0),
                duration,
                easing: Easing::EaseOut,
                ..Self::default()
            }
        }

        pub fn fade_out(duration: Duration) -> Self {
            Self {
                opacity: Some(0.0),
                duration,
                easing: Easing::EaseIn,
                ..Self::default()
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Easing {
        Linear,
        EaseOut,
        EaseIn,
        EaseInOut,
    }

    /// Combined animation block stored on [`Modifiers`](crate::Modifiers)`.animations`. All
    /// fields are optional and applied independently by the backend.
    #[derive(Copy, Clone, Debug, PartialEq, Default)]
    pub struct AnimationModifiers {
        pub implicit_transitions: Option<ImplicitTransitions>,
        pub layout_animation: Option<LayoutAnimationConfig>,
        pub property_animation: Option<AnimationConfig>,
        pub enter_transition: Option<AnimationConfig>,
        pub exit_transition: Option<AnimationConfig>,
    }

    impl AnimationModifiers {
        pub fn is_empty(&self) -> bool {
            self.implicit_transitions
                .as_ref()
                .is_none_or(|t| t.is_empty())
                && self.layout_animation.is_none()
                && self.property_animation.is_none()
                && self.enter_transition.is_none()
                && self.exit_transition.is_none()
        }
    }
}
pub(crate) mod callback {
    #[cfg(test)]
    use std::rc::Rc;

    crate::impl_rc_fn_wrapper! {
        /// Cheap-to-clone reference-counted callback. Two clones of the same
        /// `Callback` compare equal (`Rc` pointer equality), letting the
        /// reconciler skip rebinding when the same handler is re-rendered.
        pub struct Callback<T>(dyn Fn(T));
    }

    impl<T> Callback<T> {
        pub fn invoke(&self, arg: T) {
            (self.inner)(arg);
        }

        /// Construct a `Callback` from a raw `Rc<dyn Fn(T)>`. Used internally
        /// to bridge `SetState` / `Dispatch` into `Callback` without cloning.
        pub(crate) fn from_rc(inner: std::rc::Rc<dyn Fn(T)>) -> Self {
            Self { inner }
        }

        #[cfg(test)]
        pub(crate) fn strong_count(&self) -> usize {
            Rc::strong_count(&self.inner)
        }
    }

    // Windows event delegate constructors require captured closures to be `Send`.
    // Reactor callbacks are only invoked on the UI thread by this crate's XAML
    // backend, so this marker is limited to satisfying that delegate boundary.
    unsafe impl<T> Send for Callback<T> {}

    /// Trait for types that can be converted into a [`Callback<T>`].
    ///
    /// Implemented for closures (`Fn(T) + 'static`) and state setters
    /// ([`SetState<T>`](super::render_context::SetState),
    /// [`Dispatch<A>`](super::render_context::Dispatch)), allowing them
    /// to be passed directly to event handler methods without wrapping
    /// in a manual closure.
    ///
    /// # Examples
    /// ```ignore
    /// // Before: manual adapter closure
    /// text_box(name).on_text_changed(move |v| set_name.call(v))
    ///
    /// // After: pass the setter directly
    /// text_box(name).on_text_changed(set_name)
    /// ```
    pub trait IntoCallback<T> {
        fn into_callback(self) -> Callback<T>;
    }

    impl<T, F> IntoCallback<T> for F
    where
        F: Fn(T) + 'static,
    {
        fn into_callback(self) -> Callback<T> {
            Callback::new(self)
        }
    }

    impl<T: 'static> IntoCallback<T> for Callback<T> {
        fn into_callback(self) -> Callback<T> {
            self
        }
    }

    impl<T: 'static> IntoCallback<T> for super::render_context::SetState<T> {
        fn into_callback(self) -> Callback<T> {
            self.into()
        }
    }

    impl<T: 'static> IntoCallback<T> for super::render_context::Dispatch<T> {
        fn into_callback(self) -> Callback<T> {
            self.into()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cell::Cell;

        #[test]
        fn clone_of_same_callback_compares_equal() {
            let a = Callback::<i32>::new(|_| {});
            let b = a.clone();
            assert_eq!(a, b);

            assert_eq!(a.strong_count(), 2);
        }

        #[test]
        fn independently_constructed_callbacks_are_not_equal() {
            let a = Callback::<i32>::new(|_| {});
            let b = Callback::<i32>::new(|_| {});
            assert_ne!(a, b);
        }

        #[test]
        fn invoke_delivers_argument() {
            let seen = Rc::new(Cell::new(0_i32));
            let seen_c = Rc::clone(&seen);
            let cb = Callback::<i32>::new(move |v| seen_c.set(v));
            cb.invoke(7);
            assert_eq!(seen.get(), 7);
            cb.invoke(-3);
            assert_eq!(seen.get(), -3);
        }

        #[test]
        fn invoke_through_clone_touches_same_state() {
            let seen = Rc::new(Cell::new(0_i32));
            let seen_c = Rc::clone(&seen);
            let a = Callback::<i32>::new(move |v| seen_c.set(seen_c.get() + v));
            let b = a.clone();
            a.invoke(2);
            b.invoke(5);
            assert_eq!(seen.get(), 7);
        }

        #[test]
        fn debug_renders_pointer() {
            let cb = Callback::<()>::new(|()| {});
            let s = format!("{cb:?}");
            assert!(
                s.starts_with("Callback(") && s.ends_with(')'),
                "unexpected debug output: {s}"
            );
        }
    }
}
pub(crate) mod element {
    use crate::component_element::ComponentElement;
    // Re-exports for backward compatibility: the types below were previously
    // declared in this module and are now sourced from sibling modules
    // (`geometry`, `keyboard`, `modifiers`, `accessibility`, generated widgets). Both
    // `crate::element::X` and `crate::X` continue to resolve.
    use super::accessibility::{AccessibilityModifiers, HeadingLevel, LiveSetting};
    use super::keyboard::KeyboardAccelerator;
    use super::rich_text::RichTextBlock;
    use super::visual_modifiers::{AttachedProps, Modifiers};
    // Widget structs and their support types are generated under `widgets`,
    // but the historical `crate::element::X` path is part of the
    // public API surface. Re-export them here.
    pub use crate::generated_declarations::widgets::*;

    /// Fragment-style element flattened into its parent's child list during
    /// reconciliation; only valid inside multi-child containers.
    #[doc(hidden)]
    #[derive(Clone, Default, Debug, PartialEq)]
    pub struct GroupElement {
        pub key: Option<String>,
        pub children: Vec<Element>,
    }

    impl GroupElement {
        pub fn new(children: Vec<Element>) -> Self {
            Self {
                key: None,
                children,
            }
        }

        pub fn with_key(mut self, key: impl Into<String>) -> Self {
            self.key = Some(key.into());
            self
        }
    }

    /// Convenience constructor: `group([a, b, c].into())` → `Element::Group(...)`.
    pub fn group(children: Vec<Element>) -> Element {
        Element::Group(GroupElement::new(children))
    }

    /// Single source of truth for every built-in widget variant of [`Element`]:
    /// drives the enum arms, `From` impls, and `as_widget` / `modifiers_mut` /
    /// `kind_name` dispatch.
    ///
    /// By convention, every variant here is named after its backing
    /// backing XAML class (e.g. `TextBlock`, `StackPanel`, `TextBox`,
    /// `ScrollViewer`, `RichTextBlock`).
    ///
    /// To add a new built-in widget element: add a row here, define the widget
    /// struct and its `impl Widget`, add the matching `ControlKind` variant in
    /// `ir.rs`, and add the matching row in
    /// `winui/backend.rs::define_handles!`.
    macro_rules! define_element {
    ( $( $variant:ident ),* $(,)? ) => {
        /// Sum type of every element kind the reconciler can mount; widget
        /// variants each correspond to a backend control, plus a few
        /// non-widget variants for composition (`Component`, `Group`,
        /// `Custom`, …).
        #[derive(Clone, Debug, PartialEq, Default)]
        pub enum Element {
            $( $variant($variant), )*
            Component(ComponentElement),
            ErrorBoundary(crate::error_boundary::ErrorBoundaryElement),
            Provider(crate::context::ProviderElement),
            TemplatedList(crate::templated_list::TemplatedListElement),
            Group(GroupElement),
            Custom(crate::custom::CustomElementHandle),
            #[default]
            Empty,
        }

        $(
            impl From<$variant> for Element {
                fn from(v: $variant) -> Self {
                    Element::$variant(v)
                }
            }
        )*

        impl Element {
            pub(crate) fn as_widget(&self) -> Option<&dyn crate::widget::Widget> {
                Some(match self {
                    $( Element::$variant(v) => v, )*
                    Element::Component(_)
                    | Element::ErrorBoundary(_)
                    | Element::Provider(_)
                    | Element::TemplatedList(_)
                    | Element::Group(_)
                    | Element::Custom(_)
                    | Element::Empty => return None,
                })
            }
            pub fn modifiers_mut(&mut self) -> Option<&mut Modifiers> {
                match self {
                    $( Element::$variant(v) => Some(&mut v.modifiers), )*
                    Element::TemplatedList(tl) => Some(&mut tl.modifiers),
                    Element::Component(_)
                    | Element::ErrorBoundary(_)
                    | Element::Provider(_)
                    | Element::Group(_)
                    | Element::Custom(_)
                    | Element::Empty => None,
                }
            }
            pub fn kind_name(&self) -> &'static str {
                match self {
                    $( Element::$variant(_) => stringify!($variant), )*
                    Element::Component(_) => "Component",
                    Element::ErrorBoundary(_) => "ErrorBoundary",
                    Element::Provider(_) => "Provider",
                    Element::TemplatedList(_) => "TemplatedList",
                    Element::Group(_) => "Group",
                    Element::Custom(c) => c.0.kind_name(),
                    Element::Empty => "Empty",
                }
            }
        }
    };
}

    define_element! {
        TextBlock,
        Button,
        StackPanel,
        Border,
        CheckBox,
        TextBox,
        Grid,
        ScrollViewer,
        ScrollView,
        ToggleSwitch,
        Slider,
        RadioButton,
        ProgressBar,
        ProgressRing,
        HyperlinkButton,
        PersonPicture,
        Shape,
        Image,
        Pivot,
        PasswordBox,
        ComboBox,
        Canvas,
        RichTextBlock,
        ContentDialog,
        Viewbox,
        RepeatButton,
        RatingControl,
        ColorPicker,
        DatePicker,
        TimePicker,
        CalendarDatePicker,
        CalendarView,
        ListBox,
        DropDownButton,
        SplitButton,
        AutoSuggestBox,
        SplitView,
        MenuBar,
        TreeView,
        CommandBar,
        RichEditBox,
        RelativePanel,
        TitleBar,
        ToggleButton,
        SelectorBar,
    }

    macro_rules! non_widget_from_table {
    ( $( $variant:ident : $ty:ty ),* $(,)? ) => {
        $(
            impl From<$ty> for Element {
                fn from(v: $ty) -> Self { Element::$variant(v) }
            }
        )*
    };
}

    non_widget_from_table! {
        Component:     ComponentElement,
        ErrorBoundary: crate::error_boundary::ErrorBoundaryElement,
        Provider:      crate::context::ProviderElement,
        TemplatedList: crate::templated_list::TemplatedListElement,
        Group:         GroupElement,
        Custom:        crate::custom::CustomElementHandle,
    }

    impl Element {
        pub fn key(&self) -> Option<&str> {
            if let Some(w) = self.as_widget() {
                return w.key();
            }
            match self {
                Element::Component(c) => c.key.as_deref(),
                Element::ErrorBoundary(eb) => eb.key.as_deref(),
                Element::Provider(p) => p.key.as_deref(),
                Element::TemplatedList(tl) => tl.key.as_deref(),
                Element::Group(g) => g.key.as_deref(),
                Element::Custom(c) => c.0.key(),
                Element::Empty => None,
                _ => unreachable!("covered by as_widget"),
            }
        }
        pub fn modifiers(&self) -> Option<&Modifiers> {
            if let Some(w) = self.as_widget() {
                return Some(w.modifiers());
            }
            match self {
                Element::TemplatedList(tl) => Some(&tl.modifiers),
                Element::Component(_)
                | Element::ErrorBoundary(_)
                | Element::Provider(_)
                | Element::Group(_)
                | Element::Custom(_)
                | Element::Empty => None,
                _ => unreachable!("covered by as_widget"),
            }
        }
        pub fn attached_mut(&mut self) -> Option<&mut AttachedProps> {
            let m = self.modifiers_mut()?;
            Some(m.attached.get_or_insert_with(AttachedProps::default))
        }
        pub fn attached(&self) -> Option<&AttachedProps> {
            self.modifiers().and_then(|m| m.attached.as_ref())
        }
        pub fn grid_row(mut self, row: i32) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let mut p = m.grid.unwrap_or_default();
                p.row = row;
                m.grid = Some(p);
            }
            self
        }
        pub fn grid_column(mut self, column: i32) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let mut p = m.grid.unwrap_or_default();
                p.column = column;
                m.grid = Some(p);
            }
            self
        }
        pub fn grid_row_span(mut self, rs: i32) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let mut p = m.grid.unwrap_or_default();
                p.row_span = rs;
                m.grid = Some(p);
            }
            self
        }
        pub fn grid_column_span(mut self, cs: i32) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let mut p = m.grid.unwrap_or_default();
                p.column_span = cs;
                m.grid = Some(p);
            }
            self
        }
        pub fn canvas_left(mut self, x: f64) -> Self {
            if let Some(bag) = self.attached_mut() {
                let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
                p.left = x;
                bag.set(p);
            }
            self
        }
        pub fn canvas_top(mut self, y: f64) -> Self {
            if let Some(bag) = self.attached_mut() {
                let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
                p.top = y;
                bag.set(p);
            }
            self
        }
        pub fn canvas_z_index(mut self, z: i32) -> Self {
            if let Some(bag) = self.attached_mut() {
                let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
                p.z_index = z;
                bag.set(p);
            }
            self
        }
        pub fn relative_align_left(mut self) -> Self {
            if let Some(bag) = self.attached_mut() {
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_left_with_panel = true;
                bag.set(p);
            }
            self
        }
        pub fn relative_align_right(mut self) -> Self {
            if let Some(bag) = self.attached_mut() {
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_right_with_panel = true;
                bag.set(p);
            }
            self
        }
        pub fn relative_align_top(mut self) -> Self {
            if let Some(bag) = self.attached_mut() {
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_top_with_panel = true;
                bag.set(p);
            }
            self
        }
        pub fn relative_align_bottom(mut self) -> Self {
            if let Some(bag) = self.attached_mut() {
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_bottom_with_panel = true;
                bag.set(p);
            }
            self
        }
        pub fn relative_align_h_center(mut self) -> Self {
            if let Some(bag) = self.attached_mut() {
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_h_center_with_panel = true;
                bag.set(p);
            }
            self
        }
        pub fn relative_align_v_center(mut self) -> Self {
            if let Some(bag) = self.attached_mut() {
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_v_center_with_panel = true;
                bag.set(p);
            }
            self
        }
        pub fn accessibility_mut(&mut self) -> Option<&mut AccessibilityModifiers> {
            let m = self.modifiers_mut()?;
            Some(
                m.accessibility
                    .get_or_insert_with(|| Box::new(AccessibilityModifiers::default())),
            )
        }
        pub fn accessibility(&self) -> Option<&AccessibilityModifiers> {
            self.modifiers().and_then(|m| m.accessibility.as_deref())
        }
        pub fn automation_name(mut self, name: impl Into<String>) -> Self {
            if let Some(a) = self.accessibility_mut() {
                a.automation_name = Some(name.into());
            }
            self
        }
        pub fn automation_id(mut self, id: impl Into<String>) -> Self {
            if let Some(a) = self.accessibility_mut() {
                a.automation_id = Some(id.into());
            }
            self
        }
        pub fn help_text(mut self, text: impl Into<String>) -> Self {
            if let Some(a) = self.accessibility_mut() {
                a.help_text = Some(text.into());
            }
            self
        }
        pub fn accessibility_live_setting(mut self, ls: LiveSetting) -> Self {
            if let Some(a) = self.accessibility_mut() {
                a.live_setting = Some(ls);
            }
            self
        }
        pub fn heading_level(mut self, level: HeadingLevel) -> Self {
            if let Some(a) = self.accessibility_mut() {
                a.heading_level = Some(level);
            }
            self
        }
        pub fn keyboard_accelerator(mut self, accel: KeyboardAccelerator) -> Self {
            if let Some(m) = self.modifiers_mut() {
                m.keyboard_accelerators
                    .get_or_insert_with(|| Box::new(Vec::new()))
                    .push(accel);
            }
            self
        }
        /// `true` when both elements are the same variant (and same shape /
        /// custom type id), so an update can be considered.
        pub fn kind_matches(&self, other: &Element) -> bool {
            if std::mem::discriminant(self) != std::mem::discriminant(other) {
                return false;
            }
            if let (Element::Shape(a), Element::Shape(b)) = (self, other) {
                return a.kind == b.kind;
            }
            if let (Element::Custom(a), Element::Custom(b)) = (self, other) {
                return crate::custom::CustomElement::type_id(&*a.0)
                    == crate::custom::CustomElement::type_id(&*b.0);
            }
            true
        }
        /// `true` when the reconciler may diff `self` against `other` in place
        /// rather than unmounting and remounting.
        pub fn can_update(&self, other: &Element) -> bool {
            if !self.kind_matches(other) {
                return false;
            }
            match (self, other) {
                (Element::Component(a), Element::Component(b)) => {
                    a.obj.component_type_id() == b.obj.component_type_id()
                }
                (Element::Custom(a), Element::Custom(b)) => {
                    crate::custom::CustomElement::type_id(&*a.0)
                        == crate::custom::CustomElement::type_id(&*b.0)
                }
                _ => true,
            }
        }
    }

    /// `true` when the reconciler can skip diffing entirely — old/new are
    /// equal and there are no theme bindings that need re-resolving.
    pub fn can_skip_update(old: &Element, new: &Element) -> bool {
        if old != new {
            return false;
        }
        match new.modifiers() {
            Some(m) => m.theme_bindings.as_ref().is_none_or(|map| map.is_empty()),
            None => true,
        }
    }
}
pub mod factories {
    //! Backward-compatible re-exports for the factories in `widgets/*`.

    pub use crate::declaration::widgets::*;
}
pub(crate) mod geometry {
    /// Cross-axis spacing (margins, padding, etc.) — `f64` per side.
    #[derive(Copy, Clone, Debug, PartialEq, Default)]
    pub struct Thickness {
        pub left: f64,
        pub top: f64,
        pub right: f64,
        pub bottom: f64,
    }

    impl Thickness {
        pub const fn uniform(v: f64) -> Self {
            Self {
                left: v,
                top: v,
                right: v,
                bottom: v,
            }
        }
        pub const fn xy(x: f64, y: f64) -> Self {
            Self {
                left: x,
                top: y,
                right: x,
                bottom: y,
            }
        }
    }

    impl From<f64> for Thickness {
        fn from(v: f64) -> Self {
            Self::uniform(v)
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Color {
        pub a: u8,
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Color {
        pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
            Self { a: 255, r, g, b }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Brush {
        Solid(Color),
    }

    impl From<Color> for Brush {
        fn from(c: Color) -> Self {
            Self::Solid(c)
        }
    }

    pub use crate::bindings::HorizontalAlignment;
    pub use crate::bindings::ScrollBarVisibility;
    pub use crate::bindings::VerticalAlignment;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum GridLength {
        Auto,
        Pixel(f64),
        Star(f64),
    }

    impl GridLength {
        pub const STAR: Self = GridLength::Star(1.0);
    }
}
pub(crate) mod into_elements {
    //! Trait for converting heterogeneous tuples into `Vec<Element>`.
    //!
    //! This enables writing `vstack((title, body, footer))` without macros,
    //! where each tuple element can be a different type that implements
    //! `Into<Element>`.

    use super::element::Element;

    /// Converts a collection of items into a `Vec<Element>`.
    ///
    /// Implemented for:
    /// - Tuples of up to 12 elements, each `Into<Element>` (heterogeneous lists)
    /// - `Vec<Element>` (pre-built dynamic lists)
    /// - Fixed-size arrays `[T; N]` where `T: Into<Element>`
    pub trait IntoElements {
        fn into_elements(self) -> Vec<Element>;
    }

    // Vec<Element> passthrough
    impl IntoElements for Vec<Element> {
        fn into_elements(self) -> Vec<Element> {
            self
        }
    }

    // Empty tuple = no children
    impl IntoElements for () {
        fn into_elements(self) -> Vec<Element> {
            Vec::new()
        }
    }

    // Fixed-size arrays of Into<Element>
    impl<T: Into<Element>, const N: usize> IntoElements for [T; N] {
        fn into_elements(self) -> Vec<Element> {
            self.into_iter().map(Into::into).collect()
        }
    }

    // Tuple impls — each element independently implements Into<Element>.
    macro_rules! impl_into_elements_for_tuple {
    ($($idx:tt : $T:ident),+) => {
        impl<$($T: Into<Element>),+> IntoElements for ($($T,)+) {
            fn into_elements(self) -> Vec<Element> {
                vec![$(self.$idx.into()),+]
            }
        }
    };
}

    impl_into_elements_for_tuple!(0: A);
    impl_into_elements_for_tuple!(0: A, 1: B);
    impl_into_elements_for_tuple!(0: A, 1: B, 2: C);
    impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D);
    impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E);
    impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
    impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);
    impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H);
    impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2);
    impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2, 9: J);
    impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2, 9: J, 10: K);
    impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2, 9: J, 10: K, 11: L);
}
pub(crate) mod keyboard {
    /// A single keyboard shortcut bound to an element via
    /// [`Modifiers`](crate::Modifiers)`.keyboard_accelerators`.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct KeyboardAccelerator {
        pub key: KeyboardKey,
        pub modifiers: KeyModifiers,
        pub on_invoked: crate::callback::Callback<()>,
    }

    impl KeyboardAccelerator {
        pub fn new<F: Fn() + 'static>(
            key: KeyboardKey,
            modifiers: KeyModifiers,
            on_invoked: F,
        ) -> Self {
            Self {
                key,
                modifiers,
                on_invoked: crate::callback::Callback::new(move |()| on_invoked()),
            }
        }
    }

    /// Bit-flags subset of WinRT `VirtualKeyModifiers` (Alt is named `MENU`
    /// upstream; this crate keeps the conventional `ALT` name).
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct KeyModifiers(pub u32);

    impl KeyModifiers {
        pub const NONE: Self = Self(0);
        pub const CONTROL: Self = Self(1);
        pub const ALT: Self = Self(2);
        pub const SHIFT: Self = Self(4);
        pub const WINDOWS: Self = Self(8);

        pub const fn contains(self, other: Self) -> bool {
            (self.0 & other.0) == other.0
        }
        pub const fn is_empty(self) -> bool {
            self.0 == 0
        }
    }

    impl core::ops::BitOr for KeyModifiers {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self {
            Self(self.0 | rhs.0)
        }
    }

    impl core::ops::BitOrAssign for KeyModifiers {
        fn bitor_assign(&mut self, rhs: Self) {
            self.0 |= rhs.0;
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum KeyboardKey {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
        Num0,
        Num1,
        Num2,
        Num3,
        Num4,
        Num5,
        Num6,
        Num7,
        Num8,
        Num9,
        NumPad0,
        NumPad1,
        NumPad2,
        NumPad3,
        NumPad4,
        NumPad5,
        NumPad6,
        NumPad7,
        NumPad8,
        NumPad9,
        NumPadAdd,
        NumPadSubtract,
        NumPadMultiply,
        NumPadDivide,
        NumPadDecimal,
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        Enter,
        Escape,
        Tab,
        Space,
        Backspace,
        Delete,
        Insert,
        Home,
        End,
        PageUp,
        PageDown,
        Left,
        Right,
        Up,
        Down,
    }
}
pub(crate) mod modifiers {
    use std::time::Duration;

    use super::*;

    macro_rules! simple_setter {
        ($name:ident, $field:ident, $ty:ty) => {
            fn $name(mut self, v: $ty) -> Self {
                if let Some(m) = self.modifiers_mut() {
                    m.$field = Some(v);
                }
                self
            }
        };
        ($name:ident, $field:ident, $ty:ty, into) => {
            fn $name(mut self, v: impl Into<$ty>) -> Self {
                if let Some(m) = self.modifiers_mut() {
                    m.$field = Some(v.into());
                }
                self
            }
        };
    }

    /// Builder-style modifier methods (`.margin(..)`, `.background(..)`,
    /// animations, theme bindings, …) implemented for every widget builder
    /// and for [`Element`] itself.
    pub trait ElementExt: Sized {
        fn modifiers_mut(&mut self) -> Option<&mut Modifiers>;

        simple_setter!(margin, margin, Thickness, into);
        simple_setter!(padding, padding, Thickness, into);
        simple_setter!(width, width, f64);
        simple_setter!(height, height, f64);
        simple_setter!(min_width, min_width, f64);
        simple_setter!(max_width, max_width, f64);
        simple_setter!(min_height, min_height, f64);
        simple_setter!(max_height, max_height, f64);

        fn background(mut self, v: impl Into<BrushBinding>) -> Self {
            if let Some(m) = self.modifiers_mut() {
                apply_brush_binding(m, Prop::Background, v.into(), true);
            }
            self
        }

        fn foreground(mut self, v: impl Into<BrushBinding>) -> Self {
            if let Some(m) = self.modifiers_mut() {
                apply_brush_binding(m, Prop::Foreground, v.into(), false);
            }
            self
        }

        simple_setter!(opacity, opacity, f64);
        simple_setter!(font_family, font_family, String, into);
        simple_setter!(font_size, font_size, f64);
        simple_setter!(
            horizontal_alignment,
            horizontal_alignment,
            HorizontalAlignment
        );
        simple_setter!(vertical_alignment, vertical_alignment, VerticalAlignment);

        fn with_key(self, key: impl Into<String>) -> Self;

        fn with_opacity_transition(mut self, duration: Duration) -> Self {
            if let Some(m) = self.modifiers_mut() {
                with_implicit_transition(m, |it| {
                    it.opacity = Some(ScalarTransition::new(duration));
                });
            }
            self
        }

        fn with_scale_transition(mut self, duration: Duration) -> Self {
            if let Some(m) = self.modifiers_mut() {
                with_implicit_transition(m, |it| {
                    it.scale = Some(Vector3Transition::new(duration));
                });
            }
            self
        }

        fn with_translation_transition(mut self, duration: Duration) -> Self {
            if let Some(m) = self.modifiers_mut() {
                with_implicit_transition(m, |it| {
                    it.translation = Some(Vector3Transition::new(duration));
                });
            }
            self
        }

        fn with_layout_animation(mut self, config: LayoutAnimationConfig) -> Self {
            if let Some(m) = self.modifiers_mut() {
                ensure_animations(m).layout_animation = Some(config);
            }
            self
        }

        fn animate(mut self, config: AnimationConfig) -> Self {
            if let Some(m) = self.modifiers_mut() {
                ensure_animations(m).property_animation = Some(config);
            }
            self
        }

        fn transition(
            mut self,
            enter: Option<AnimationConfig>,
            exit: Option<AnimationConfig>,
        ) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let a = ensure_animations(m);
                a.enter_transition = enter;
                a.exit_transition = exit;
            }
            self
        }

        fn grid_row(mut self, row: i32) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let mut p = m.grid.unwrap_or_default();
                p.row = row;
                m.grid = Some(p);
            }
            self
        }

        fn grid_column(mut self, column: i32) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let mut p = m.grid.unwrap_or_default();
                p.column = column;
                m.grid = Some(p);
            }
            self
        }

        fn grid_row_span(mut self, rs: i32) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let mut p = m.grid.unwrap_or_default();
                p.row_span = rs;
                m.grid = Some(p);
            }
            self
        }

        fn grid_column_span(mut self, cs: i32) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let mut p = m.grid.unwrap_or_default();
                p.column_span = cs;
                m.grid = Some(p);
            }
            self
        }

        fn canvas_left(mut self, x: f64) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let bag = m.attached.get_or_insert_with(AttachedProps::default);
                let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
                p.left = x;
                bag.set(p);
            }
            self
        }

        fn canvas_top(mut self, y: f64) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let bag = m.attached.get_or_insert_with(AttachedProps::default);
                let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
                p.top = y;
                bag.set(p);
            }
            self
        }

        fn canvas_z_index(mut self, z: i32) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let bag = m.attached.get_or_insert_with(AttachedProps::default);
                let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
                p.z_index = z;
                bag.set(p);
            }
            self
        }

        fn provide<T>(self, context: &Context<T>, value: T) -> Element
        where
            T: Clone + PartialEq + 'static,
            Self: Into<Element>,
        {
            let mut el: Element = self.into();
            let provision = ContextProvision::new(context.id, value);
            if let Element::Provider(ref mut existing) = el {
                existing.provisions.push(provision);
                el
            } else {
                Element::Provider(ProviderElement {
                    key: None,
                    provisions: vec![provision],
                    child: Box::new(el),
                })
            }
        }

        /// Plain-text tooltip applied via WinUI `ToolTipService`. For rich
        /// content or custom placement, use [`tooltip_with`](Self::tooltip_with).
        fn tooltip(mut self, text: impl Into<String>) -> Self {
            if let Some(m) = self.modifiers_mut() {
                m.tooltip = Some(Box::new(crate::tooltip::Tooltip::text(text)));
            }
            self
        }

        /// Tooltip setter accepting anything convertible into
        /// [`crate::tooltip::Tooltip`].
        fn tooltip_with(mut self, t: impl Into<crate::tooltip::Tooltip>) -> Self {
            if let Some(m) = self.modifiers_mut() {
                m.tooltip = Some(Box::new(t.into()));
            }
            self
        }

        /// Register a `Tapped` (left-tap) handler.
        fn on_tapped<F: Fn() + 'static>(mut self, f: F) -> Self {
            if let Some(m) = self.modifiers_mut() {
                ensure_pointer_handlers(m).on_tapped =
                    Some(crate::callback::Callback::new(move |()| f()));
            }
            self
        }

        /// Register a `RightTapped` handler (right-click / barrel button /
        /// touch-and-hold).
        fn on_right_tapped<F: Fn() + 'static>(mut self, f: F) -> Self {
            if let Some(m) = self.modifiers_mut() {
                ensure_pointer_handlers(m).on_right_tapped =
                    Some(crate::callback::Callback::new(move |()| f()));
            }
            self
        }

        /// Register a `PointerPressed` handler; the callback receives the
        /// current button state.
        fn on_pointer_pressed<F>(mut self, f: F) -> Self
        where
            F: Fn(crate::pointer::PointerEventInfo) + 'static,
        {
            if let Some(m) = self.modifiers_mut() {
                ensure_pointer_handlers(m).on_pointer_pressed =
                    Some(crate::callback::Callback::new(f));
            }
            self
        }

        /// Register a `PointerReleased` handler; the callback receives the
        /// button state at release.
        fn on_pointer_released<F>(mut self, f: F) -> Self
        where
            F: Fn(crate::pointer::PointerEventInfo) + 'static,
        {
            if let Some(m) = self.modifiers_mut() {
                ensure_pointer_handlers(m).on_pointer_released =
                    Some(crate::callback::Callback::new(f));
            }
            self
        }

        /// Register a `PointerExited` handler (pointer leaves hit-test bounds).
        fn on_pointer_exited<F: Fn() + 'static>(mut self, f: F) -> Self {
            if let Some(m) = self.modifiers_mut() {
                ensure_pointer_handlers(m).on_pointer_exited =
                    Some(crate::callback::Callback::new(move |()| f()));
            }
            self
        }

        // ── Accessibility modifiers ──────────────────────────────────────────

        fn automation_name(mut self, name: impl Into<String>) -> Self {
            if let Some(m) = self.modifiers_mut() {
                ensure_accessibility(m).automation_name = Some(name.into());
            }
            self
        }

        fn automation_id(mut self, id: impl Into<String>) -> Self {
            if let Some(m) = self.modifiers_mut() {
                ensure_accessibility(m).automation_id = Some(id.into());
            }
            self
        }

        fn help_text(mut self, text: impl Into<String>) -> Self {
            if let Some(m) = self.modifiers_mut() {
                ensure_accessibility(m).help_text = Some(text.into());
            }
            self
        }

        fn heading_level(mut self, level: HeadingLevel) -> Self {
            if let Some(m) = self.modifiers_mut() {
                ensure_accessibility(m).heading_level = Some(level);
            }
            self
        }

        fn accessibility_live_setting(mut self, ls: LiveSetting) -> Self {
            if let Some(m) = self.modifiers_mut() {
                ensure_accessibility(m).live_setting = Some(ls);
            }
            self
        }

        fn keyboard_accelerator(mut self, accel: KeyboardAccelerator) -> Self {
            if let Some(m) = self.modifiers_mut() {
                m.keyboard_accelerators
                    .get_or_insert_with(|| Box::new(Vec::new()))
                    .push(accel);
            }
            self
        }

        // ── RelativePanel attached property helpers ──────────────────────────

        fn relative_align_left(mut self) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let bag = m.attached.get_or_insert_with(AttachedProps::default);
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_left_with_panel = true;
                bag.set(p);
            }
            self
        }

        fn relative_align_right(mut self) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let bag = m.attached.get_or_insert_with(AttachedProps::default);
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_right_with_panel = true;
                bag.set(p);
            }
            self
        }

        fn relative_align_top(mut self) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let bag = m.attached.get_or_insert_with(AttachedProps::default);
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_top_with_panel = true;
                bag.set(p);
            }
            self
        }

        fn relative_align_bottom(mut self) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let bag = m.attached.get_or_insert_with(AttachedProps::default);
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_bottom_with_panel = true;
                bag.set(p);
            }
            self
        }

        fn relative_align_h_center(mut self) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let bag = m.attached.get_or_insert_with(AttachedProps::default);
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_h_center_with_panel = true;
                bag.set(p);
            }
            self
        }

        fn relative_align_v_center(mut self) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let bag = m.attached.get_or_insert_with(AttachedProps::default);
                let mut p = bag
                    .get::<RelativePanelAlignment>()
                    .copied()
                    .unwrap_or_default();
                p.align_v_center_with_panel = true;
                bag.set(p);
            }
            self
        }

        fn resources(
            mut self,
            entries: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
        ) -> Self {
            if let Some(m) = self.modifiers_mut() {
                let map = entries
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect();
                m.resources = Some(Box::new(map));
            }
            self
        }
    }

    fn ensure_pointer_handlers(m: &mut Modifiers) -> &mut crate::pointer::PointerHandlers {
        m.pointer_handlers
            .get_or_insert_with(|| Box::new(crate::pointer::PointerHandlers::default()))
    }

    fn ensure_accessibility(
        m: &mut Modifiers,
    ) -> &mut crate::accessibility::AccessibilityModifiers {
        m.accessibility.get_or_insert_with(|| {
            Box::new(crate::accessibility::AccessibilityModifiers::default())
        })
    }

    fn apply_brush_binding(
        m: &mut Modifiers,
        prop: Prop,
        binding: BrushBinding,
        is_background: bool,
    ) {
        if let Some(map) = m.theme_bindings.as_deref_mut() {
            map.remove(&prop);
        }
        match binding {
            BrushBinding::Direct(b) => {
                if is_background {
                    m.background = Some(b);
                } else {
                    m.foreground = Some(b);
                }
            }
            BrushBinding::Theme(t) => {
                if is_background {
                    m.background = None;
                } else {
                    m.foreground = None;
                }
                m.theme_bindings
                    .get_or_insert_with(|| Box::new(std::collections::HashMap::default()))
                    .insert(prop, t);
            }
        }

        if m.theme_bindings.as_deref().is_some_and(|m| m.is_empty()) {
            m.theme_bindings = None;
        }
    }

    fn ensure_animations(m: &mut Modifiers) -> &mut AnimationModifiers {
        m.animations
            .get_or_insert_with(|| Box::new(AnimationModifiers::default()))
    }

    fn with_implicit_transition(m: &mut Modifiers, f: impl FnOnce(&mut ImplicitTransitions)) {
        let anim = ensure_animations(m);
        let it = anim
            .implicit_transitions
            .get_or_insert_with(Default::default);
        f(it);
    }

    macro_rules! impl_element_ext {
    ($($ty:ident),* $(,)?) => {
        $(
            impl ElementExt for crate::widgets::$ty {
                fn modifiers_mut(&mut self) -> Option<&mut Modifiers> {
                    Some(&mut self.modifiers)
                }
                fn with_key(mut self, key: impl Into<String>) -> Self {
                    self.key = Some(key.into());
                    self
                }
            }
        )*
    };
}
    impl_element_ext!(
        TextBlock,
        Button,
        StackPanel,
        Border,
        CheckBox,
        TextBox,
        Grid,
        ScrollViewer,
        ScrollView,
        ToggleSwitch,
        Slider,
        RadioButton,
        ProgressBar,
        ProgressRing,
        HyperlinkButton,
        PersonPicture,
        Shape,
        Image,
        Pivot,
        PasswordBox,
        ComboBox,
        Canvas,
        ContentDialog,
        Viewbox,
        RepeatButton,
        RatingControl,
        ColorPicker,
        DatePicker,
        TimePicker,
        CalendarDatePicker,
        CalendarView,
        ListBox,
        DropDownButton,
        SplitButton,
        AutoSuggestBox,
        SplitView,
        MenuBar,
        TreeView,
        CommandBar,
        RichEditBox,
        RelativePanel,
        TitleBar,
        ToggleButton,
        SelectorBar,
    );

    impl ElementExt for crate::rich_text::RichTextBlock {
        fn modifiers_mut(&mut self) -> Option<&mut Modifiers> {
            Some(&mut self.modifiers)
        }
        fn with_key(mut self, key: impl Into<String>) -> Self {
            self.key = Some(key.into());
            self
        }
    }

    impl ElementExt for Element {
        fn modifiers_mut(&mut self) -> Option<&mut Modifiers> {
            Element::modifiers_mut(self)
        }

        fn with_key(mut self, key: impl Into<String>) -> Self {
            let key = key.into();
            match &mut self {
                Element::TextBlock(t) => t.key = Some(key),
                Element::Button(b) => b.key = Some(key),
                Element::StackPanel(s) => s.key = Some(key),
                Element::Border(b) => b.key = Some(key),
                Element::CheckBox(c) => c.key = Some(key),
                Element::TextBox(t) => t.key = Some(key),
                Element::Grid(g) => g.key = Some(key),
                Element::ScrollViewer(s) => s.key = Some(key),
                Element::ScrollView(s) => s.key = Some(key),
                Element::ToggleSwitch(v) => v.key = Some(key),
                Element::Slider(v) => v.key = Some(key),
                Element::RadioButton(v) => v.key = Some(key),
                Element::ProgressBar(v) => v.key = Some(key),
                Element::ProgressRing(v) => v.key = Some(key),
                Element::HyperlinkButton(v) => v.key = Some(key),
                Element::PersonPicture(v) => v.key = Some(key),
                Element::Shape(v) => v.key = Some(key),
                Element::Image(v) => v.key = Some(key),
                Element::Pivot(v) => v.key = Some(key),
                Element::PasswordBox(v) => v.key = Some(key),
                Element::ComboBox(v) => v.key = Some(key),
                Element::Canvas(v) => v.key = Some(key),
                Element::ContentDialog(v) => v.key = Some(key),
                Element::RichTextBlock(v) => v.key = Some(key),
                Element::Viewbox(v) => v.key = Some(key),
                Element::RepeatButton(v) => v.key = Some(key),
                Element::RatingControl(v) => v.key = Some(key),
                Element::ColorPicker(v) => v.key = Some(key),
                Element::DatePicker(v) => v.key = Some(key),
                Element::TimePicker(v) => v.key = Some(key),
                Element::CalendarDatePicker(v) => v.key = Some(key),
                Element::CalendarView(v) => v.key = Some(key),
                Element::ListBox(v) => v.key = Some(key),
                Element::DropDownButton(v) => v.key = Some(key),
                Element::SplitButton(v) => v.key = Some(key),
                Element::AutoSuggestBox(v) => v.key = Some(key),
                Element::SplitView(v) => v.key = Some(key),
                Element::MenuBar(v) => v.key = Some(key),
                Element::TreeView(v) => v.key = Some(key),
                Element::CommandBar(v) => v.key = Some(key),
                Element::RichEditBox(v) => v.key = Some(key),
                Element::RelativePanel(v) => v.key = Some(key),
                Element::TitleBar(v) => v.key = Some(key),
                Element::ToggleButton(v) => v.key = Some(key),
                Element::SelectorBar(v) => v.key = Some(key),
                Element::Component(c) => c.key = Some(key),
                Element::ErrorBoundary(eb) => eb.key = Some(key),
                Element::Provider(pe) => pe.key = Some(key),
                Element::TemplatedList(tl) => tl.key = Some(key),
                Element::Group(g) => g.key = Some(key),
                _ => {}
            }
            self
        }
    }
}
pub(crate) mod pointer {
    //! Per-element pointer / tap event handlers carried on
    //! [`Modifiers::pointer_handlers`](super::Modifiers::pointer_handlers)
    //! and applied via [`Backend::set_pointer_handlers`](super::ir::Backend::set_pointer_handlers).
    //! These mirror the universal `UIElement` pointer events (`Tapped`,
    //! `RightTapped`, `PointerPressed`, `PointerReleased`, `PointerExited`).

    use super::callback::Callback;

    /// Bundle of per-element pointer / tap callbacks; each slot is
    /// individually optional.
    #[derive(Clone, Default, Debug, PartialEq)]
    pub struct PointerHandlers {
        pub on_tapped: Option<Callback<()>>,
        pub on_right_tapped: Option<Callback<()>>,
        pub on_pointer_pressed: Option<Callback<PointerEventInfo>>,
        pub on_pointer_released: Option<Callback<PointerEventInfo>>,
        pub on_pointer_exited: Option<Callback<()>>,
    }

    impl PointerHandlers {
        pub fn is_empty(&self) -> bool {
            self.on_tapped.is_none()
                && self.on_right_tapped.is_none()
                && self.on_pointer_pressed.is_none()
                && self.on_pointer_released.is_none()
                && self.on_pointer_exited.is_none()
        }
    }

    /// Button state captured at a `PointerPressed` / `PointerReleased`
    /// callback. Non-mouse pointer kinds report all three as `false`.
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    pub struct PointerEventInfo {
        pub is_left_button_pressed: bool,
        pub is_right_button_pressed: bool,
        pub is_middle_button_pressed: bool,
    }
}
pub(crate) mod prop_binding {
    use super::*;

    /// One entry of a widget's flat declarative binding list: either a
    /// property value or an event handler slot.
    #[derive(Debug, Clone, PartialEq)]
    pub enum Binding {
        Prop(Prop, PropValue),
        Event(Event, Option<EventHandler>),
    }

    pub type PropBindings = Vec<Binding>;

    /// Converts a widget struct field to an optional `PropValue::Str`.
    ///
    /// Generated binding code calls `w.field.to_prop_str()` for TextBlock
    /// and IReference setters. The trait lets the same generated code work
    /// whether the field is `String` (always `Some`) or `Option<String>`
    /// (mirrors the Option).
    pub(crate) trait ToPropStr {
        fn to_prop_str(&self) -> Option<PropValue>;
    }

    impl ToPropStr for String {
        fn to_prop_str(&self) -> Option<PropValue> {
            Some(PropValue::Str(self.clone()))
        }
    }

    impl ToPropStr for Option<String> {
        fn to_prop_str(&self) -> Option<PropValue> {
            self.as_ref().map(|s| PropValue::Str(s.clone()))
        }
    }

    pub(crate) fn find_prop(bindings: &[Binding], prop: Prop) -> Option<&PropValue> {
        bindings.iter().find_map(|b| match b {
            Binding::Prop(p, v) if *p == prop => Some(v),
            _ => None,
        })
    }

    pub(crate) fn find_event(bindings: &[Binding], event: Event) -> Option<&Option<EventHandler>> {
        bindings.iter().find_map(|b| match b {
            Binding::Event(e, h) if *e == event => Some(h),
            _ => None,
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn find_prop_returns_first_match() {
            let bs = vec![
                Binding::Prop(Prop::Text, PropValue::Str("hi".into())),
                Binding::Prop(Prop::FontSize, PropValue::F64(12.0)),
            ];
            assert_eq!(
                find_prop(&bs, Prop::Text),
                Some(&PropValue::Str("hi".into()))
            );
            assert_eq!(find_prop(&bs, Prop::FontSize), Some(&PropValue::F64(12.0)));
            assert_eq!(find_prop(&bs, Prop::Background), None);
        }

        #[test]
        fn find_event_distinguishes_some_and_none() {
            let bs = vec![Binding::Event(Event::Click, None)];
            assert!(matches!(find_event(&bs, Event::Click), Some(None)));
            assert!(find_event(&bs, Event::Toggled).is_none());
        }
    }
}
pub(crate) mod resource {
    //! Async data primitives: [`Resource`], [`use_resource`], and [`use_mutation`].
    //!
    //! These compose over the lower-level [`use_async_state`] and [`use_effect`]
    //! hooks to provide a batteries-included pattern for loading remote data
    //! and performing async writes.

    use std::sync::Arc;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::render_context::{Deps, RenderCx};

    // ─── Resource enum ──────────────────────────────────────────────────────

    /// Represents the lifecycle of an async data fetch.
    #[derive(Clone, Debug, PartialEq)]
    pub enum Resource<T> {
        /// No fetch has been initiated (initial state before first render cycle).
        Idle,
        /// A fetch is in progress (no prior data available).
        Loading,
        /// Data was loaded successfully.
        Ready(T),
        /// A refetch is in progress but previous data is still available.
        Reloading(T),
        /// The fetch failed.
        Error(String),
    }

    impl<T> Resource<T> {
        /// Returns the data if in `Ready` or `Reloading` state.
        pub fn data(&self) -> Option<&T> {
            match self {
                Self::Ready(v) | Self::Reloading(v) => Some(v),
                _ => None,
            }
        }

        /// Returns `true` if currently loading or reloading.
        pub fn is_loading(&self) -> bool {
            matches!(self, Self::Loading | Self::Reloading(_))
        }

        /// Returns the error string if in `Error` state.
        pub fn error(&self) -> Option<&str> {
            match self {
                Self::Error(e) => Some(e.as_str()),
                _ => None,
            }
        }

        /// Returns `true` if data is available (Ready or Reloading).
        pub fn has_data(&self) -> bool {
            matches!(self, Self::Ready(_) | Self::Reloading(_))
        }

        /// Render the resource with a `ready` closure; loading and error use
        /// sensible defaults (overridable via [`ResourceView::loading`] /
        /// [`ResourceView::error`]).
        ///
        /// # Example
        /// ```ignore
        /// let items = cx.use_resource(fetch_items, page);
        /// items.view(|data| {
        ///     vstack(data.iter().map(|s| -> Element { text_block(s).into() }).collect::<Vec<Element>>())
        ///         .into()
        /// })
        /// ```
        pub fn view<F, R>(
            &self,
            ready: F,
        ) -> ResourceView<'_, T, impl FnOnce(&T) -> super::element::Element>
        where
            F: FnOnce(&T) -> R,
            R: Into<super::element::Element>,
        {
            ResourceView {
                resource: self,
                ready: move |data: &T| ready(data).into(),
                loading: None,
                error: None,
            }
        }
    }

    /// Builder returned by [`Resource::view()`] — call `.loading()` or `.error()`
    /// to override defaults before converting to [`Element`](crate::Element) via `.into()`.
    pub struct ResourceView<'a, T, F>
    where
        F: FnOnce(&T) -> super::element::Element,
    {
        resource: &'a Resource<T>,
        ready: F,
        loading: Option<super::element::Element>,
        error: Option<Box<dyn FnOnce(&str) -> super::element::Element>>,
    }

    impl<'a, T, F> ResourceView<'a, T, F>
    where
        F: FnOnce(&T) -> super::element::Element,
    {
        /// Override the loading state element (default: indeterminate ProgressRing).
        pub fn loading(mut self, el: impl Into<super::element::Element>) -> Self {
            self.loading = Some(el.into());
            self
        }

        /// Override the error state renderer (default: text block showing the error).
        pub fn error<E>(mut self, f: E) -> Self
        where
            E: FnOnce(&str) -> super::element::Element + 'static,
        {
            self.error = Some(Box::new(f));
            self
        }
    }

    impl<T, F> From<ResourceView<'_, T, F>> for super::element::Element
    where
        F: FnOnce(&T) -> super::element::Element,
    {
        fn from(rv: ResourceView<'_, T, F>) -> Self {
            use super::element::ProgressRing;
            match rv.resource {
                Resource::Loading | Resource::Idle => rv
                    .loading
                    .unwrap_or_else(|| ProgressRing::indeterminate().into()),
                Resource::Ready(data) | Resource::Reloading(data) => (rv.ready)(data),
                Resource::Error(e) => rv.error.map_or_else(
                    || super::element::text_block(format!("Error: {e}")).into(),
                    |f| f(e.as_str()),
                ),
            }
        }
    }

    // ─── Mutation state ─────────────────────────────────────────────────────

    /// Represents the lifecycle of an async write/mutation.
    #[derive(Clone, Debug, PartialEq)]
    pub enum MutationState<T> {
        /// No mutation in progress.
        Idle,
        /// A mutation is in progress.
        Loading,
        /// The mutation completed successfully.
        Success(T),
        /// The mutation failed.
        Error(String),
    }

    impl<T> MutationState<T> {
        pub fn is_loading(&self) -> bool {
            matches!(self, Self::Loading)
        }

        pub fn is_idle(&self) -> bool {
            matches!(self, Self::Idle)
        }

        pub fn data(&self) -> Option<&T> {
            match self {
                Self::Success(v) => Some(v),
                _ => None,
            }
        }

        pub fn error(&self) -> Option<&str> {
            match self {
                Self::Error(e) => Some(e.as_str()),
                _ => None,
            }
        }
    }

    // ─── Generation token for cancellation ──────────────────────────────────

    /// Monotonic counter used to discard results from stale fetches.
    /// When deps change, the generation increments; results stamped with
    /// an older generation are silently dropped.
    #[derive(Clone)]
    struct Generation(Arc<AtomicU64>);

    impl Generation {
        fn new() -> Self {
            Self(Arc::new(AtomicU64::new(0)))
        }

        fn advance(&self) -> u64 {
            self.0.fetch_add(1, Ordering::AcqRel) + 1
        }

        fn is_current(&self, value: u64) -> bool {
            self.0.load(Ordering::Acquire) == value
        }
    }

    // ─── use_resource ───────────────────────────────────────────────────────

    impl RenderCx {
        /// Fetch data on a background thread, refetching when `deps` change.
        ///
        /// ```ignore
        /// let users = cx.use_resource(
        ///     |filter| api::search_users_blocking(&filter),
        ///     search_term.clone(),
        /// );
        /// ```
        ///
        /// If you need an async runtime, create one inside the fetcher:
        /// `|deps| { tokio::runtime::Runtime::new()?.block_on(fetch(deps)) }`
        pub fn use_resource<T, D, F>(&mut self, fetcher: F, deps: D) -> Resource<T>
        where
            T: Send + Clone + PartialEq + 'static,
            D: Deps + Clone + Send + 'static,
            F: Fn(D) -> Result<T, String> + Send + 'static,
        {
            // Hook 1: the resource state
            let (state, set_state) = self.use_async_state(Resource::<T>::Loading);

            let generation = self.use_ref(Generation::new());

            let deps_clone = deps.clone();
            let gen_handle = generation;
            self.use_effect(deps, move || {
                let my_gen = gen_handle.borrow().advance();
                let set_state2 = set_state.clone();
                set_state.call(Resource::Loading);

                let fetcher_deps = deps_clone.clone();
                let gen_for_thread = gen_handle.get_cloned();

                windows_threading::submit(move || {
                    let result = fetcher(fetcher_deps);
                    if gen_for_thread.is_current(my_gen) {
                        match result {
                            Ok(data) => set_state2.call(Resource::Ready(data)),
                            Err(e) => set_state2.call(Resource::Error(e)),
                        }
                    }
                });
            });

            state
        }

        /// Mutation hook for async write operations. Returns the mutation state
        /// and a trigger function.
        ///
        /// ```ignore
        /// let (state, mutate) = cx.use_mutation();
        /// // In an event handler:
        /// mutate.fire(|| api::save_user_blocking(&user));
        /// ```
        pub fn use_mutation<T>(&mut self) -> (MutationState<T>, MutationTrigger<T>)
        where
            T: Send + Clone + PartialEq + 'static,
        {
            let (state, set_state) = self.use_async_state(MutationState::<T>::Idle);
            let trigger = MutationTrigger { set_state };
            (state, trigger)
        }
    }

    // ─── MutationTrigger ────────────────────────────────────────────────────

    /// Handle returned by [`RenderCx::use_mutation`] to fire async write
    /// operations from event handlers.
    #[derive(Clone)]
    pub struct MutationTrigger<T: Send + Clone + PartialEq + 'static> {
        set_state: super::render_context::AsyncSetState<MutationState<T>>,
    }

    impl<T: Send + Clone + PartialEq + 'static> MutationTrigger<T> {
        /// Fire a mutation on a background thread.
        pub fn fire<F>(&self, f: F)
        where
            F: FnOnce() -> Result<T, String> + Send + 'static,
        {
            let set = self.set_state.clone();
            set.call(MutationState::Loading);
            let set2 = self.set_state.clone();
            windows_threading::submit(move || {
                let result = f();
                match result {
                    Ok(data) => set2.call(MutationState::Success(data)),
                    Err(e) => set2.call(MutationState::Error(e)),
                }
            });
        }

        /// Reset the mutation state back to Idle (e.g. after showing success).
        pub fn reset(&self) {
            self.set_state.call(MutationState::Idle);
        }
    }

    impl<T: Send + Clone + PartialEq + 'static> std::fmt::Debug for MutationTrigger<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MutationTrigger").finish_non_exhaustive()
        }
    }
}
pub(crate) mod rich_text {
    use super::*;

    #[derive(Clone, Default, Debug, PartialEq, Eq)]
    pub struct RichTextLineBreak;

    #[derive(Clone, Default, Debug, PartialEq)]
    pub struct RichTextRun {
        pub text: String,
        pub is_bold: bool,
        pub is_italic: bool,
        pub is_strikethrough: bool,
        pub font_family: Option<String>,
        pub font_size: Option<f64>,
    }

    impl RichTextRun {
        pub fn plain(text: impl Into<String>) -> Self {
            Self {
                text: text.into(),
                ..Default::default()
            }
        }
    }

    #[derive(Clone, Default, Debug, PartialEq, Eq)]
    pub struct RichTextHyperlink {
        pub text: String,
        pub uri: String,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum RichTextInline {
        Run(RichTextRun),
        LineBreak,
        Hyperlink(RichTextHyperlink),
    }

    impl Default for RichTextInline {
        fn default() -> Self {
            Self::Run(RichTextRun::default())
        }
    }

    #[derive(Clone, Default, Debug, PartialEq)]
    pub struct RichTextParagraph {
        pub inlines: Vec<RichTextInline>,
    }

    impl RichTextParagraph {
        pub fn new(inlines: Vec<RichTextInline>) -> Self {
            Self { inlines }
        }
    }

    #[derive(Clone, Default, Debug, PartialEq)]
    pub struct RichTextBlock {
        pub key: Option<String>,
        pub modifiers: Modifiers,
        pub paragraphs: Vec<RichTextParagraph>,
        pub font_size: Option<f64>,
        pub is_text_selection_enabled: bool,
        pub text_wrapping: TextWrapping,
    }

    /// Backward-compat alias.
    pub type RichText = RichTextBlock;

    impl RichTextBlock {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn single_paragraph(inlines: Vec<RichTextInline>) -> Self {
            Self {
                paragraphs: vec![RichTextParagraph::new(inlines)],
                ..Self::default()
            }
        }

        pub fn font_size(mut self, size: f64) -> Self {
            self.font_size = Some(size);
            self
        }

        pub fn selectable(mut self) -> Self {
            self.is_text_selection_enabled = true;
            self
        }

        pub fn wrap(mut self) -> Self {
            self.text_wrapping = TextWrapping::Wrap;
            self
        }
    }

    impl Widget for RichTextBlock {
        widget_header!(ControlKind::RichTextBlock);
        fn bindings(&self) -> PropBindings {
            let mut out = Vec::with_capacity(3);
            if let Some(fs) = self.font_size {
                out.push(Binding::Prop(Prop::FontSize, PropValue::F64(fs)));
            }
            if self.is_text_selection_enabled {
                out.push(Binding::Prop(
                    Prop::IsTextSelectionEnabled,
                    PropValue::Bool(true),
                ));
            }
            if self.text_wrapping != TextWrapping::NoWrap {
                out.push(Binding::Prop(
                    Prop::TextWrappingWrap,
                    PropValue::I32(self.text_wrapping.0),
                ));
            }
            out
        }
    }
}
pub(crate) mod templated_list {
    use std::any::TypeId;
    use std::rc::Rc;

    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum TemplatedKind {
        ListView,
        GridView,
        FlipView,
    }

    /// Selection behaviour for templated lists (ListView, GridView).
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub enum SelectionMode {
        /// No selection allowed.
        None,
        /// Exactly one item may be selected at a time (default).
        #[default]
        Single,
        /// Multiple items may be selected via Ctrl+Click.
        Multiple,
        /// Range selection via Shift+Click and Ctrl+Click.
        Extended,
    }

    /// Erased adapter exposing the items of a [`TemplatedListElement`] to the
    /// reconciler without leaking the concrete item type.
    pub trait TemplatedListImpl: 'static {
        fn item_count(&self) -> usize;

        fn build_item_view(&self, index: usize) -> Element;

        fn item_key(&self, index: usize) -> Option<String>;

        fn selected_index(&self) -> i32;

        fn invoke_selection_changed(&self, index: i32);

        fn item_type_id(&self) -> TypeId;

        fn items_ptr(&self) -> *const ();

        fn has_selection_handler(&self) -> bool;

        fn raw_selection_callback(&self) -> Option<Callback<i32>>;
    }

    struct TemplatedListCell<T: 'static> {
        items: Rc<Vec<T>>,
        view_builder: Rc<dyn Fn(&T, usize) -> Element>,
        key_selector: Option<Rc<dyn Fn(&T) -> String>>,
        on_selection_changed: Option<Callback<i32>>,
        selected_index: i32,
    }

    impl<T: 'static> TemplatedListImpl for TemplatedListCell<T> {
        fn item_count(&self) -> usize {
            self.items.len()
        }

        fn build_item_view(&self, index: usize) -> Element {
            match self.items.get(index) {
                Some(item) => (self.view_builder)(item, index),
                None => Element::Empty,
            }
        }

        fn item_key(&self, index: usize) -> Option<String> {
            let item = self.items.get(index)?;
            self.key_selector.as_ref().map(|k| k(item))
        }

        fn selected_index(&self) -> i32 {
            self.selected_index
        }

        fn invoke_selection_changed(&self, index: i32) {
            if let Some(cb) = &self.on_selection_changed {
                cb.invoke(index);
            }
        }

        fn item_type_id(&self) -> TypeId {
            TypeId::of::<T>()
        }

        fn items_ptr(&self) -> *const () {
            Rc::as_ptr(&self.items) as *const ()
        }

        fn has_selection_handler(&self) -> bool {
            self.on_selection_changed.is_some()
        }

        fn raw_selection_callback(&self) -> Option<Callback<i32>> {
            self.on_selection_changed.clone()
        }
    }

    /// Virtualised list/grid/flip element backed by a [`TemplatedListImpl`].
    pub struct TemplatedListElement {
        pub key: Option<String>,
        pub kind: TemplatedKind,
        pub selection_mode: SelectionMode,
        pub modifiers: Modifiers,
        pub(crate) items_impl: Rc<dyn TemplatedListImpl>,
    }

    impl Clone for TemplatedListElement {
        fn clone(&self) -> Self {
            Self {
                key: self.key.clone(),
                kind: self.kind,
                selection_mode: self.selection_mode,
                modifiers: self.modifiers.clone(),
                items_impl: Rc::clone(&self.items_impl),
            }
        }
    }

    impl std::fmt::Debug for TemplatedListElement {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TemplatedListElement")
                .field("key", &self.key)
                .field("kind", &self.kind)
                .field("item_count", &self.items_impl.item_count())
                .finish()
        }
    }

    impl PartialEq for TemplatedListElement {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
                && self.kind == other.kind
                && self.selection_mode == other.selection_mode
                && self.modifiers == other.modifiers
                && Rc::ptr_eq(&self.items_impl, &other.items_impl)
        }
    }

    impl TemplatedListElement {
        pub fn same_items_as(&self, other: &TemplatedListElement) -> bool {
            self.items_impl.item_type_id() == other.items_impl.item_type_id()
                && self.items_impl.items_ptr() == other.items_impl.items_ptr()
        }

        pub fn item_count(&self) -> usize {
            self.items_impl.item_count()
        }

        pub fn build_item_view(&self, index: usize) -> Element {
            self.items_impl.build_item_view(index)
        }

        pub fn item_key(&self, index: usize) -> Option<String> {
            self.items_impl.item_key(index)
        }

        pub fn selected_index(&self) -> i32 {
            self.items_impl.selected_index()
        }

        pub fn invoke_selection_changed(&self, index: i32) {
            self.items_impl.invoke_selection_changed(index);
        }

        pub fn has_selection_handler(&self) -> bool {
            self.items_impl.has_selection_handler()
        }

        pub fn raw_selection_callback(&self) -> Option<Callback<i32>> {
            self.items_impl.raw_selection_callback()
        }

        pub fn fully_keyed(&self) -> bool {
            (0..self.item_count()).all(|i| self.items_impl.item_key(i).is_some())
        }

        pub fn collect_keys(&self) -> Vec<String> {
            (0..self.item_count())
                .map(|i| {
                    self.items_impl
                        .item_key(i)
                        .unwrap_or_else(|| format!("__pos_{i}"))
                })
                .collect()
        }
    }

    /// Fluent builder for [`TemplatedListElement`]; pick a kind via
    /// [`list_view`] / [`grid_view`] / [`flip_view`].
    pub struct TemplatedListBuilder<T: 'static> {
        kind: TemplatedKind,
        items: Rc<Vec<T>>,
        view_builder: Rc<dyn Fn(&T, usize) -> Element>,
        key_selector: Option<Rc<dyn Fn(&T) -> String>>,
        on_selection_changed: Option<Callback<i32>>,
        selected_index: i32,
        selection_mode: SelectionMode,
        modifiers: Modifiers,
        element_key: Option<String>,
    }

    impl<T: 'static> TemplatedListBuilder<T> {
        fn new<R: Into<Element>>(
            kind: TemplatedKind,
            items: Vec<T>,
            view: impl Fn(&T, usize) -> R + 'static,
        ) -> Self {
            Self {
                kind,
                items: Rc::new(items),
                view_builder: Rc::new(move |item, idx| view(item, idx).into()),
                key_selector: None,
                on_selection_changed: None,
                selected_index: -1,
                selection_mode: SelectionMode::Single,
                modifiers: Modifiers::default(),
                element_key: None,
            }
        }

        pub fn with_key_selector(mut self, k: impl Fn(&T) -> String + 'static) -> Self {
            self.key_selector = Some(Rc::new(k));
            self
        }

        pub fn selected_index(mut self, i: i32) -> Self {
            self.selected_index = i;
            self
        }

        pub fn on_selection_changed(mut self, cb: impl Fn(i32) + 'static) -> Self {
            self.on_selection_changed = Some(Callback::new(cb));
            self
        }

        pub fn selection_mode(mut self, mode: SelectionMode) -> Self {
            self.selection_mode = mode;
            self
        }

        pub fn with_key(mut self, k: impl Into<String>) -> Self {
            self.element_key = Some(k.into());
            self
        }

        pub fn width(mut self, v: f64) -> Self {
            self.modifiers.width = Some(v);
            self
        }

        pub fn height(mut self, v: f64) -> Self {
            self.modifiers.height = Some(v);
            self
        }

        pub fn build(self) -> Element {
            let cell = TemplatedListCell::<T> {
                items: self.items,
                view_builder: self.view_builder,
                key_selector: self.key_selector,
                on_selection_changed: self.on_selection_changed,
                selected_index: self.selected_index,
            };
            Element::TemplatedList(TemplatedListElement {
                key: self.element_key,
                kind: self.kind,
                selection_mode: self.selection_mode,
                modifiers: self.modifiers,
                items_impl: Rc::new(cell),
            })
        }
    }

    impl<T: 'static> From<TemplatedListBuilder<T>> for Element {
        fn from(b: TemplatedListBuilder<T>) -> Self {
            b.build()
        }
    }

    pub fn list_view<T: 'static, R: Into<Element>>(
        items: Vec<T>,
        view: impl Fn(&T, usize) -> R + 'static,
    ) -> TemplatedListBuilder<T> {
        TemplatedListBuilder::new(TemplatedKind::ListView, items, view)
    }

    pub fn grid_view<T: 'static, R: Into<Element>>(
        items: Vec<T>,
        view: impl Fn(&T, usize) -> R + 'static,
    ) -> TemplatedListBuilder<T> {
        TemplatedListBuilder::new(TemplatedKind::GridView, items, view)
    }

    pub fn flip_view<T: 'static, R: Into<Element>>(
        items: Vec<T>,
        view: impl Fn(&T, usize) -> R + 'static,
    ) -> TemplatedListBuilder<T> {
        TemplatedListBuilder::new(TemplatedKind::FlipView, items, view)
    }

    /// Alias for [`list_view`] — exposes the same virtualised
    /// `Windows.UI.Xaml.Controls.ListView` adapter under the name used in
    /// the public roadmap (W1). Convert / route the result the same way as
    /// any other [`Element`].
    pub fn virtual_list<T: 'static, R: Into<Element>>(
        items: Vec<T>,
        view: impl Fn(&T, usize) -> R + 'static,
    ) -> TemplatedListBuilder<T> {
        list_view(items, view)
    }
}
pub(crate) mod theme {
    use std::borrow::Cow;
    use std::cell::Cell;

    use super::*;

    /// Effective color scheme reported by
    /// [`crate::render_context::RenderCx::use_color_scheme`].
    #[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
    pub enum ColorScheme {
        #[default]
        Light,
        Dark,
    }

    thread_local! {
        static CURRENT_COLOR_SCHEME: Cell<ColorScheme> = const { Cell::new(ColorScheme::Light) };
    }

    /// Read the host's last-known [`ColorScheme`] for the current UI thread.
    pub fn current_color_scheme() -> ColorScheme {
        CURRENT_COLOR_SCHEME.with(|c| c.get())
    }

    /// Update the per-thread [`ColorScheme`]; called by the host on
    /// `ActualThemeChanged` (and once during initial attach).
    pub fn set_current_color_scheme(scheme: ColorScheme) {
        CURRENT_COLOR_SCHEME.with(|c| c.set(scheme));
    }

    /// Symbolic reference to a WinUI XAML theme resource (resolved at apply
    /// time so the binding tracks light/dark switches).
    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub enum ThemeRef {
        Accent,
        AccentSecondary,
        AccentTertiary,
        AccentDisabled,
        PrimaryText,
        SecondaryText,
        TertiaryText,
        DisabledText,
        AccentText,
        SolidBackground,
        CardBackground,
        SmokeFill,
        SubtleFill,
        LayerFill,
        ControlFill,
        ControlFillSecondary,
        ControlFillTertiary,
        ControlFillDisabled,
        ControlFillInputActive,
        CardStroke,
        SurfaceStroke,
        DividerStroke,
        ControlStroke,
        ControlStrokeSecondary,
        SystemAttention,
        SystemSuccess,
        SystemCaution,
        SystemCritical,
        SystemNeutral,
        SystemSolidNeutral,
        SystemAttentionBackground,
        SystemSuccessBackground,
        SystemCautionBackground,
        SystemCriticalBackground,
        SystemNeutralBackground,
        SystemSolidAttention,
        Custom(Cow<'static, str>),
    }

    impl ThemeRef {
        pub fn custom(key: impl Into<Cow<'static, str>>) -> Self {
            ThemeRef::Custom(key.into())
        }

        pub fn resource_key(&self) -> &str {
            match self {
                ThemeRef::Accent => "AccentFillColorDefaultBrush",
                ThemeRef::AccentSecondary => "AccentFillColorSecondaryBrush",
                ThemeRef::AccentTertiary => "AccentFillColorTertiaryBrush",
                ThemeRef::AccentDisabled => "AccentFillColorDisabledBrush",

                ThemeRef::PrimaryText => "TextFillColorPrimaryBrush",
                ThemeRef::SecondaryText => "TextFillColorSecondaryBrush",
                ThemeRef::TertiaryText => "TextFillColorTertiaryBrush",
                ThemeRef::DisabledText => "TextFillColorDisabledBrush",
                ThemeRef::AccentText => "AccentTextFillColorPrimaryBrush",

                ThemeRef::SolidBackground => "SolidBackgroundFillColorBaseBrush",
                ThemeRef::CardBackground => "CardBackgroundFillColorDefaultBrush",
                ThemeRef::SmokeFill => "SmokeFillColorDefaultBrush",
                ThemeRef::SubtleFill => "SubtleFillColorSecondaryBrush",
                ThemeRef::LayerFill => "LayerFillColorDefaultBrush",

                ThemeRef::ControlFill => "ControlFillColorDefaultBrush",
                ThemeRef::ControlFillSecondary => "ControlFillColorSecondaryBrush",
                ThemeRef::ControlFillTertiary => "ControlFillColorTertiaryBrush",
                ThemeRef::ControlFillDisabled => "ControlFillColorDisabledBrush",
                ThemeRef::ControlFillInputActive => "ControlFillColorInputActiveBrush",

                ThemeRef::CardStroke => "CardStrokeColorDefaultBrush",
                ThemeRef::SurfaceStroke => "SurfaceStrokeColorDefaultBrush",
                ThemeRef::DividerStroke => "DividerStrokeColorDefaultBrush",
                ThemeRef::ControlStroke => "ControlStrokeColorDefaultBrush",
                ThemeRef::ControlStrokeSecondary => "ControlStrokeColorSecondaryBrush",

                ThemeRef::SystemAttention => "SystemFillColorAttentionBrush",
                ThemeRef::SystemSuccess => "SystemFillColorSuccessBrush",
                ThemeRef::SystemCaution => "SystemFillColorCautionBrush",
                ThemeRef::SystemCritical => "SystemFillColorCriticalBrush",
                ThemeRef::SystemNeutral => "SystemFillColorNeutralBrush",
                ThemeRef::SystemSolidNeutral => "SystemFillColorSolidNeutralBrush",
                ThemeRef::SystemAttentionBackground => "SystemFillColorAttentionBackgroundBrush",
                ThemeRef::SystemSuccessBackground => "SystemFillColorSuccessBackgroundBrush",
                ThemeRef::SystemCautionBackground => "SystemFillColorCautionBackgroundBrush",
                ThemeRef::SystemCriticalBackground => "SystemFillColorCriticalBackgroundBrush",
                ThemeRef::SystemNeutralBackground => "SystemFillColorNeutralBackgroundBrush",
                ThemeRef::SystemSolidAttention => "SystemFillColorSolidAttentionBackgroundBrush",

                ThemeRef::Custom(s) => s.as_ref(),
            }
        }
    }

    /// Brush slot that can be either a literal [`Brush`]
    /// or a [`ThemeRef`]; used for `background` / `foreground` modifiers.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum BrushBinding {
        Direct(Brush),
        Theme(ThemeRef),
    }

    impl From<Brush> for BrushBinding {
        fn from(v: Brush) -> Self {
            BrushBinding::Direct(v)
        }
    }

    impl From<Color> for BrushBinding {
        fn from(c: Color) -> Self {
            BrushBinding::Direct(Brush::Solid(c))
        }
    }

    impl From<ThemeRef> for BrushBinding {
        fn from(v: ThemeRef) -> Self {
            BrushBinding::Theme(v)
        }
    }

    #[allow(non_upper_case_globals)]
    pub mod tokens {
        use super::ThemeRef;

        pub const Accent: ThemeRef = ThemeRef::Accent;

        pub const AccentSecondary: ThemeRef = ThemeRef::AccentSecondary;

        pub const AccentTertiary: ThemeRef = ThemeRef::AccentTertiary;

        pub const AccentDisabled: ThemeRef = ThemeRef::AccentDisabled;

        pub const PrimaryText: ThemeRef = ThemeRef::PrimaryText;

        pub const SecondaryText: ThemeRef = ThemeRef::SecondaryText;

        pub const TertiaryText: ThemeRef = ThemeRef::TertiaryText;

        pub const DisabledText: ThemeRef = ThemeRef::DisabledText;

        pub const AccentText: ThemeRef = ThemeRef::AccentText;

        pub const SolidBackground: ThemeRef = ThemeRef::SolidBackground;

        pub const CardBackground: ThemeRef = ThemeRef::CardBackground;

        pub const SmokeFill: ThemeRef = ThemeRef::SmokeFill;

        pub const SubtleFill: ThemeRef = ThemeRef::SubtleFill;

        pub const LayerFill: ThemeRef = ThemeRef::LayerFill;

        pub const ControlFill: ThemeRef = ThemeRef::ControlFill;

        pub const ControlFillSecondary: ThemeRef = ThemeRef::ControlFillSecondary;

        pub const ControlFillTertiary: ThemeRef = ThemeRef::ControlFillTertiary;

        pub const ControlFillDisabled: ThemeRef = ThemeRef::ControlFillDisabled;

        pub const ControlFillInputActive: ThemeRef = ThemeRef::ControlFillInputActive;

        pub const CardStroke: ThemeRef = ThemeRef::CardStroke;

        pub const SurfaceStroke: ThemeRef = ThemeRef::SurfaceStroke;

        pub const DividerStroke: ThemeRef = ThemeRef::DividerStroke;

        pub const ControlStroke: ThemeRef = ThemeRef::ControlStroke;

        pub const ControlStrokeSecondary: ThemeRef = ThemeRef::ControlStrokeSecondary;

        pub const SystemAttention: ThemeRef = ThemeRef::SystemAttention;

        pub const SystemSuccess: ThemeRef = ThemeRef::SystemSuccess;

        pub const SystemCaution: ThemeRef = ThemeRef::SystemCaution;

        pub const SystemCritical: ThemeRef = ThemeRef::SystemCritical;

        pub const SystemNeutral: ThemeRef = ThemeRef::SystemNeutral;

        pub const SystemSolidNeutral: ThemeRef = ThemeRef::SystemSolidNeutral;
    }
}
pub(crate) mod tooltip {
    //! Tooltip data model carried on
    //! [`Modifiers::tooltip`](super::Modifiers::tooltip) and applied via
    //! WinUI `ToolTipService` (an attached property on `FrameworkElement`).

    use super::Element;

    /// Tooltip configuration applied via WinUI `ToolTipService`. Build from
    /// a plain string or `Tooltip::rich(element)` for templated content.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Tooltip {
        pub content: TooltipContent,
        pub placement: Option<TooltipPlacement>,
    }

    impl Tooltip {
        /// Plain-text tooltip; WinUI wraps the string in a default
        /// `ToolTip` `TextBlock`.
        pub fn text(s: impl Into<String>) -> Self {
            Self {
                content: TooltipContent::Text(s.into()),
                placement: None,
            }
        }

        /// Rich tooltip; `element` is mounted as the `Content` of a
        /// `ToolTip` instance at apply time.
        pub fn rich(element: impl Into<Element>) -> Self {
            Self {
                content: TooltipContent::Rich(Box::new(element.into())),
                placement: None,
            }
        }

        pub fn placement(mut self, p: TooltipPlacement) -> Self {
            self.placement = Some(p);
            self
        }
    }

    impl<S: Into<String>> From<S> for Tooltip {
        fn from(s: S) -> Self {
            Tooltip::text(s)
        }
    }

    /// Tooltip payload: a plain string or a templated child element.
    #[derive(Clone, Debug, PartialEq)]
    pub enum TooltipContent {
        Text(String),
        Rich(Box<Element>),
    }

    /// Rust mirror of `Windows.UI.Xaml.Controls.Primitives.PlacementMode`.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum TooltipPlacement {
        Top,
        Bottom,
        Left,
        Right,
        Mouse,
    }
}
pub(crate) mod visual_modifiers {
    use std::{
        any::{Any, TypeId},
        collections::HashMap,
    };

    use super::*;

    /// Visual modifiers shared by every widget; carried on each element struct
    /// and applied uniformly via `FrameworkElement`-level setters at the
    /// backend.
    #[derive(Clone, Default, Debug, PartialEq)]
    pub struct Modifiers {
        pub margin: Option<Thickness>,
        pub padding: Option<Thickness>,
        pub width: Option<f64>,
        pub height: Option<f64>,
        pub min_width: Option<f64>,
        pub max_width: Option<f64>,
        pub min_height: Option<f64>,
        pub max_height: Option<f64>,
        pub horizontal_alignment: Option<HorizontalAlignment>,
        pub vertical_alignment: Option<VerticalAlignment>,
        pub opacity: Option<f64>,
        pub background: Option<Brush>,
        pub foreground: Option<Brush>,
        pub font_family: Option<String>,
        pub font_size: Option<f64>,
        pub theme_bindings: Option<Box<std::collections::HashMap<crate::ir::Prop, ThemeRef>>>,
        pub animations: Option<Box<crate::animation::AnimationModifiers>>,
        pub attached: Option<AttachedProps>,
        pub accessibility: Option<Box<AccessibilityModifiers>>,
        pub keyboard_accelerators: Option<Box<Vec<KeyboardAccelerator>>>,
        pub tooltip: Option<Box<crate::tooltip::Tooltip>>,
        pub pointer_handlers: Option<Box<crate::pointer::PointerHandlers>>,
        /// Fast-path for grid row/column placement — avoids the `AttachedProps`
        /// HashMap + Box + thread_local overhead for the most common attached prop.
        pub grid: Option<GridPlacement>,
        pub resources: Option<Box<HashMap<String, String>>>,
    }

    impl Modifiers {
        pub fn is_empty(&self) -> bool {
            self.margin.is_none()
                && self.padding.is_none()
                && self.width.is_none()
                && self.height.is_none()
                && self.min_width.is_none()
                && self.max_width.is_none()
                && self.min_height.is_none()
                && self.max_height.is_none()
                && self.horizontal_alignment.is_none()
                && self.vertical_alignment.is_none()
                && self.opacity.is_none()
                && self.background.is_none()
                && self.foreground.is_none()
                && self.font_family.is_none()
                && self.font_size.is_none()
                && self.theme_bindings.as_ref().is_none_or(|m| m.is_empty())
                && self.animations.as_ref().is_none_or(|a| a.is_empty())
                && self.attached.as_ref().is_none_or(|a| a.is_empty())
                && self.accessibility.as_deref().is_none_or(|a| a.is_empty())
                && self
                    .keyboard_accelerators
                    .as_deref()
                    .is_none_or(|a| a.is_empty())
                && self.tooltip.is_none()
                && self
                    .pointer_handlers
                    .as_deref()
                    .is_none_or(|p| p.is_empty())
                && self.grid.is_none()
                && self.resources.as_deref().is_none_or(|r| r.is_empty())
        }
    }

    /// Type-erased bag of attached properties (e.g. [`GridPlacement`]) keyed
    /// by [`TypeId`]; values must be inserted via [`AttachedProps::set`].
    #[derive(Default, Debug)]
    pub struct AttachedProps(HashMap<TypeId, Box<dyn Any>>);

    impl Clone for AttachedProps {
        fn clone(&self) -> Self {
            let mut copy = HashMap::default();
            for (k, v) in &self.0 {
                let (clone_fn, _) = ops_for(*k).unwrap();
                copy.insert(*k, clone_fn(&**v));
            }
            Self(copy)
        }
    }

    impl PartialEq for AttachedProps {
        fn eq(&self, other: &Self) -> bool {
            if self.0.len() != other.0.len() {
                return false;
            }
            for (k, v) in &self.0 {
                let Some(ov) = other.0.get(k) else {
                    return false;
                };
                let (_, eq_fn) = ops_for(*k).unwrap();
                if !eq_fn(&**v, &**ov) {
                    return false;
                }
            }
            true
        }
    }

    impl AttachedProps {
        pub fn set<T: Clone + PartialEq + 'static>(&mut self, v: T) {
            register_ops::<T>();
            self.0.insert(TypeId::of::<T>(), Box::new(v));
        }
        pub fn get<T: 'static>(&self) -> Option<&T> {
            self.0
                .get(&TypeId::of::<T>())
                .and_then(|b| b.downcast_ref::<T>())
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct GridPlacement {
        pub row: i32,
        pub column: i32,
        pub row_span: i32,
        pub column_span: i32,
    }

    impl Default for GridPlacement {
        fn default() -> Self {
            Self {
                row: 0,
                column: 0,
                row_span: 1,
                column_span: 1,
            }
        }
    }

    type CloneFn = fn(&dyn Any) -> Box<dyn Any>;
    type EqFn = fn(&dyn Any, &dyn Any) -> bool;

    #[derive(Copy, Clone)]
    struct AttachedOps {
        clone: CloneFn,
        eq: EqFn,
    }

    thread_local! {
        static OPS_REGISTRY: std::cell::RefCell<HashMap<TypeId, AttachedOps>> =
            std::cell::RefCell::new(HashMap::default());
    }

    fn register_ops<T: Clone + PartialEq + 'static>() {
        let tid = TypeId::of::<T>();
        OPS_REGISTRY.with(|r| {
            r.borrow_mut().entry(tid).or_insert_with(|| AttachedOps {
                clone: |src| Box::new(src.downcast_ref::<T>().unwrap().clone()),
                eq: |a, b| match (a.downcast_ref::<T>(), b.downcast_ref::<T>()) {
                    (Some(a), Some(b)) => a == b,
                    _ => false,
                },
            });
        });
    }

    fn ops_for(tid: TypeId) -> Option<(CloneFn, EqFn)> {
        OPS_REGISTRY.with(|r| r.borrow().get(&tid).map(|ops| (ops.clone, ops.eq)))
    }
}
pub(crate) mod widget {
    use super::*;

    /// How a widget exposes its children to the child reconciler.
    pub(crate) enum Children<'a> {
        None,
        PositionalSingle(&'a Element),
        Keyed(&'a [Element]),
        PivotItems(&'a [PivotItem]),
    }

    /// Adapter implemented by every built-in [`Element`] widget variant; lets
    /// the reconciler drive backend prop / child operations uniformly.
    pub(crate) trait Widget {
        fn kind(&self) -> ControlKind;
        fn key(&self) -> Option<&str>;
        fn modifiers(&self) -> &Modifiers;
        fn attached(&self) -> Option<&AttachedProps> {
            self.modifiers().attached.as_ref()
        }
        fn bindings(&self) -> PropBindings;
        fn children(&self) -> Children<'_> {
            Children::None
        }
        /// Optional element tree for a complex header slot.
        fn header_element(&self) -> Option<&Element> {
            None
        }
        /// Optional element tree for a pane slot (e.g. SplitView pane).
        fn pane_element(&self) -> Option<&Element> {
            None
        }
        /// Optional post-mount callback. When present, the reconciler invokes it
        /// immediately after creation with the native element (`IInspectable`), or
        /// `None` if the backend exposes no native element for the control.
        fn on_mounted_callback(
            &self,
        ) -> Option<&crate::callback::Callback<Option<windows_core::IInspectable>>> {
            None
        }
        /// Optional pre-unmount callback. When present, the reconciler invokes it
        /// just before the control is destroyed, while the control still exists,
        /// with the native element (`IInspectable`), or `None` if the backend
        /// exposes no native element. Owners use this to tear down external
        /// resources bound to the control (e.g. join a render thread that presents
        /// into the control's swap chain) — teardown runs regardless of whether a
        /// native element is present.
        fn on_unmounted_callback(
            &self,
        ) -> Option<&crate::callback::Callback<Option<windows_core::IInspectable>>> {
            None
        }
    }
}

pub(crate) use crate::*;
pub(crate) use accessibility::*;
pub(crate) use animation::*;
pub(crate) use callback::*;
pub(crate) use element::*;
pub(crate) use geometry::*;
pub(crate) use keyboard::*;
pub(crate) use prop_binding::*;
pub(crate) use templated_list::*;
pub(crate) use theme::*;
pub(crate) use tooltip::*;
pub(crate) use visual_modifiers::*;
pub(crate) use widget::*;

pub use factories::*;
pub use modifiers::*;
