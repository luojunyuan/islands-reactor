pub(crate) mod widgets {
    //! Islands widget catalogue emitted by `tool-reactor`.

    // Make the core types each widget module depends on visible via
    // `use super::*;` inside the per-widget files.
    pub(crate) use super::*;

    /// Emit the boilerplate `kind`/`key`/`modifiers` accessors shared by
    /// every `impl Widget for X`; `attached` falls through to the default
    /// impl on the trait.
    macro_rules! widget_header {
        ($kind:expr) => {
            fn kind(&self) -> ControlKind {
                $kind
            }
            fn key(&self) -> Option<&str> {
                self.key.as_deref()
            }
            fn modifiers(&self) -> &Modifiers {
                &self.modifiers
            }
        };
    }
    pub(crate) use widget_header;

    pub(crate) mod icon {
        pub use crate::bindings::Symbol;
        /// Compatibility alias for `Symbol`.
        pub type SymbolGlyph = Symbol;
    }
    pub use icon::*;
    pub(crate) mod auto_suggest_box {
        use super::*;

        /// `Windows.UI.Xaml.Controls.AutoSuggestBox`. A text input that displays
        /// a filtered suggestion list as the user types.
        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct AutoSuggestBox {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub text: String,
            pub items: Vec<String>,
            pub placeholder_text: String,
            pub header: Option<String>,
            pub is_enabled: bool,
            pub on_text_changed: Option<Callback<String>>,
            pub on_query_submitted: Option<Callback<String>>,
            pub on_suggestion_chosen: Option<Callback<String>>,
        }

        impl AutoSuggestBox {
            pub fn new(text: impl Into<String>) -> Self {
                Self {
                    text: text.into(),
                    is_enabled: true,
                    ..Default::default()
                }
            }

            pub fn items<I, S>(mut self, items: I) -> Self
            where
                I: IntoIterator<Item = S>,
                S: Into<String>,
            {
                self.items = items.into_iter().map(Into::into).collect();
                self
            }

            pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
                self.placeholder_text = s.into();
                self
            }

            pub fn header(mut self, s: impl Into<String>) -> Self {
                self.header = Some(s.into());
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }

            pub fn on_text_changed<F: Fn(String) + 'static>(mut self, f: F) -> Self {
                self.on_text_changed = Some(Callback::new(f));
                self
            }

            pub fn on_query_submitted<F: Fn(String) + 'static>(mut self, f: F) -> Self {
                self.on_query_submitted = Some(Callback::new(f));
                self
            }

            pub fn on_suggestion_chosen<F: Fn(String) + 'static>(mut self, f: F) -> Self {
                self.on_suggestion_chosen = Some(Callback::new(f));
                self
            }
        }

        impl Widget for AutoSuggestBox {
            widget_header!(ControlKind::AutoSuggestBox);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::auto_suggest_box_bindings(self);
                out.push(Binding::Prop(Prop::Text, PropValue::Str(self.text.clone())));
                out.push(Binding::Prop(
                    Prop::Items,
                    PropValue::StrList(self.items.clone()),
                ));
                out
            }
        }

        pub fn auto_suggest_box(text: impl Into<String>) -> AutoSuggestBox {
            AutoSuggestBox::new(text)
        }
    }
    pub use auto_suggest_box::*;
    pub(crate) mod border {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct Border {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub corner_radius: Option<f64>,
            pub border_brush: Option<BrushBinding>,
            pub border_thickness: Option<Thickness>,
            pub child: Box<Element>,
        }
        impl Border {
            pub fn new(child: impl Into<Element>) -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    corner_radius: None,
                    border_brush: None,
                    border_thickness: None,
                    child: Box::new(child.into()),
                }
            }

            /// Uniform corner radius in DIPs. Maps to WinUI `Border.CornerRadius`
            /// (all four corners). Useful for card / pill / chip layouts.
            pub fn corner_radius(mut self, v: f64) -> Self {
                self.corner_radius = Some(v);
                self
            }

            /// Brush used to paint the border stroke. Maps to WinUI
            /// `Border.BorderBrush`. Pair with [`Border::border_thickness`] for
            /// the stroke to actually render. Accepts a direct [`Brush`] or a
            /// [`ThemeRef`] for theme-aware strokes.
            pub fn border_brush(mut self, v: impl Into<BrushBinding>) -> Self {
                match v.into() {
                    BrushBinding::Direct(b) => {
                        self.border_brush = Some(BrushBinding::Direct(b));
                    }
                    BrushBinding::Theme(t) => {
                        self.border_brush = None;
                        self.modifiers
                            .theme_bindings
                            .get_or_insert_with(|| Box::new(std::collections::HashMap::default()))
                            .insert(Prop::BorderBrush, t);
                    }
                }
                self
            }

            /// Stroke thickness, in DIPs, on each side. Maps to WinUI
            /// `Border.BorderThickness`.
            pub fn border_thickness(mut self, v: Thickness) -> Self {
                self.border_thickness = Some(v);
                self
            }
        }
        impl Default for Border {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    corner_radius: None,
                    border_brush: None,
                    border_thickness: None,
                    child: Box::new(Element::Empty),
                }
            }
        }

        impl Widget for Border {
            widget_header!(ControlKind::Border);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::border_bindings(self);
                if let Some(BrushBinding::Direct(br)) = &self.border_brush {
                    out.push(Binding::Prop(
                        Prop::BorderBrush,
                        PropValue::Brush(br.clone()),
                    ));
                }
                if let Some(v) = self.border_thickness {
                    out.push(Binding::Prop(
                        Prop::BorderThickness,
                        PropValue::Thickness(v),
                    ));
                }
                if let Some(v) = self.corner_radius {
                    out.push(Binding::Prop(Prop::CornerRadius, PropValue::F64(v)));
                }
                out
            }
            fn children(&self) -> Children<'_> {
                Children::PositionalSingle(&self.child)
            }
        }

        pub fn border(child: impl Into<Element>) -> Border {
            Border::new(child)
        }
    }
    pub use border::*;
    pub(crate) mod button {
        use super::command_bar::CommandBarCommandDef;
        use super::menu_bar::MenuItemDef;
        use super::*;

        /// Visual style for a [`Button`]. Not a WinRT enum — maps to resource key strings.
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
        pub struct ButtonStyle(pub i32);
        #[allow(non_upper_case_globals)]
        impl ButtonStyle {
            /// Standard button (framework default).
            pub const Default: Self = Self(0);
            /// Accent-colored button (primary action).
            pub const Accent: Self = Self(1);
            /// Chromeless subtle button (secondary action).
            pub const Subtle: Self = Self(2);
            /// Borderless text-link style (inline hyperlink pattern).
            pub const TextLink: Self = Self(3);
        }

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct Button {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub content: String,
            pub is_enabled: bool,
            pub style: ButtonStyle,
            pub icon: Option<SymbolGlyph>,
            pub on_click: Option<Callback<()>>,
            pub flyout: Option<FlyoutDef>,
            pub menu_flyout_items: Option<Vec<MenuItemDef>>,
            pub on_item_clicked: Option<Callback<String>>,
            pub command_bar_flyout_primary: Option<Vec<CommandBarCommandDef>>,
            pub command_bar_flyout_secondary: Option<Vec<CommandBarCommandDef>>,
            pub on_command_bar_flyout_click: Option<Callback<String>>,
        }
        impl Button {
            pub fn new(content: impl Into<String>) -> Self {
                Self {
                    content: content.into(),
                    is_enabled: true,
                    ..Default::default()
                }
            }
        }

        impl Widget for Button {
            widget_header!(ControlKind::Button);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::button_bindings(self);
                out.push(Binding::Prop(
                    Prop::Content,
                    PropValue::Str(self.content.clone()),
                ));
                if let Some(v) = self.icon {
                    out.push(Binding::Prop(Prop::Icon, PropValue::I32(v.0)));
                }
                if let Some(v) = &self.menu_flyout_items {
                    out.push(Binding::Prop(
                        Prop::MenuFlyoutItems,
                        PropValue::MenuFlyoutItems(v.clone()),
                    ));
                }
                if self.style != ButtonStyle::Default {
                    out.push(Binding::Prop(
                        Prop::StyleVariant,
                        PropValue::I32(self.style.0),
                    ));
                }
                // Flyout and CommandBarFlyout are compound types not in TOML.
                if let Some(ref fly) = self.flyout {
                    out.push(Binding::Prop(
                        Prop::FlyoutContent,
                        PropValue::Str(fly.text.clone()),
                    ));
                    if fly.placement != FlyoutPlacement::default() {
                        out.push(Binding::Prop(
                            Prop::FlyoutPlacement,
                            PropValue::I32(fly.placement.0),
                        ));
                    }
                }
                if let Some(ref primary) = self.command_bar_flyout_primary {
                    out.push(Binding::Prop(
                        Prop::CommandBarFlyoutCommands,
                        PropValue::CommandBarFlyoutDef {
                            primary: primary.clone(),
                            secondary: self
                                .command_bar_flyout_secondary
                                .clone()
                                .unwrap_or_default(),
                        },
                    ));
                }
                out
            }
        }

        impl Button {
            pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self {
                self.on_click = Some(crate::callback::Callback::new(move |()| f()));
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }

            pub fn accent(mut self) -> Self {
                self.style = ButtonStyle::Accent;
                self
            }

            pub fn subtle(mut self) -> Self {
                self.style = ButtonStyle::Subtle;
                self
            }

            pub fn text_link(mut self) -> Self {
                self.style = ButtonStyle::TextLink;
                self
            }

            pub fn icon(mut self, sym: SymbolGlyph) -> Self {
                self.icon = Some(sym);
                self
            }

            pub fn flyout(mut self, text: impl Into<String>) -> Self {
                self.flyout = Some(FlyoutDef {
                    text: text.into(),
                    placement: FlyoutPlacement::default(),
                });
                self
            }

            pub fn flyout_with_placement(
                mut self,
                text: impl Into<String>,
                placement: FlyoutPlacement,
            ) -> Self {
                self.flyout = Some(FlyoutDef {
                    text: text.into(),
                    placement,
                });
                self
            }

            pub fn menu_flyout(mut self, items: Vec<MenuItemDef>) -> Self {
                self.menu_flyout_items = Some(items);
                self
            }

            pub fn on_item_clicked<F: Fn(String) + 'static>(mut self, f: F) -> Self {
                self.on_item_clicked = Some(Callback::new(f));
                self
            }

            pub fn command_bar_flyout(mut self, primary: Vec<CommandBarCommandDef>) -> Self {
                self.command_bar_flyout_primary = Some(primary);
                self
            }

            pub fn command_bar_flyout_secondary(
                mut self,
                secondary: Vec<CommandBarCommandDef>,
            ) -> Self {
                self.command_bar_flyout_secondary = Some(secondary);
                self
            }

            pub fn on_command_bar_flyout_click<F: Fn(String) + 'static>(mut self, f: F) -> Self {
                self.on_command_bar_flyout_click = Some(Callback::new(f));
                self
            }
        }

        pub fn button(content: impl Into<String>) -> Button {
            Button::new(content)
        }
    }
    pub use button::*;
    pub(crate) mod calendar_date_picker {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct CalendarDatePicker {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub header: Option<String>,
            pub placeholder_text: String,
            pub is_today_highlighted: bool,
            pub is_calendar_open: bool,
            pub is_enabled: bool,
            pub on_changed: Option<Callback<Option<windows_time::DateTime>>>,
        }

        impl CalendarDatePicker {
            pub fn new() -> Self {
                Self {
                    is_today_highlighted: true,
                    is_enabled: true,
                    ..Default::default()
                }
            }

            pub fn header(mut self, s: impl Into<String>) -> Self {
                self.header = Some(s.into());
                self
            }

            pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
                self.placeholder_text = s.into();
                self
            }

            pub fn today_highlighted(mut self, v: bool) -> Self {
                self.is_today_highlighted = v;
                self
            }

            pub fn calendar_open(mut self, v: bool) -> Self {
                self.is_calendar_open = v;
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }

            pub fn on_changed(
                mut self,
                f: impl IntoCallback<Option<windows_time::DateTime>>,
            ) -> Self {
                self.on_changed = Some(f.into_callback());
                self
            }
        }

        impl Widget for CalendarDatePicker {
            widget_header!(ControlKind::CalendarDatePicker);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::calendar_date_picker_bindings(self)
            }
        }

        pub fn calendar_date_picker() -> CalendarDatePicker {
            CalendarDatePicker::new()
        }
    }
    pub use calendar_date_picker::*;
    pub(crate) mod calendar_view {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct CalendarView {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub is_today_highlighted: bool,
            pub is_group_label_visible: bool,
            pub is_enabled: bool,
            pub on_selected_dates_changed: Option<Callback<()>>,
        }

        impl CalendarView {
            pub fn new() -> Self {
                Self {
                    is_today_highlighted: true,
                    is_group_label_visible: true,
                    is_enabled: true,
                    ..Default::default()
                }
            }

            pub fn today_highlighted(mut self, v: bool) -> Self {
                self.is_today_highlighted = v;
                self
            }

            pub fn group_label_visible(mut self, v: bool) -> Self {
                self.is_group_label_visible = v;
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }

            pub fn on_selected_dates_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
                self.on_selected_dates_changed = Some(Callback::new(move |()| f()));
                self
            }
        }

        impl Widget for CalendarView {
            widget_header!(ControlKind::CalendarView);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::calendar_view_bindings(self)
            }
        }

        pub fn calendar_view() -> CalendarView {
            CalendarView::new()
        }
    }
    pub use calendar_view::*;
    pub(crate) mod canvas {
        use super::*;

        /// W5 — `Windows.UI.Xaml.Controls.Canvas`. Free-positioning panel
        /// where each child is placed via the [`CanvasPosition`] attached
        /// property (`canvas_left` / `canvas_top` on [`Element`]).
        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct Canvas {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub children: Vec<Element>,
        }
        impl Canvas {
            pub fn new<I>(children: I) -> Self
            where
                I: IntoIterator,
                I::Item: Into<Element>,
            {
                Self {
                    children: children.into_iter().map(Into::into).collect(),
                    ..Default::default()
                }
            }
        }
        /// Attached property for children of [`Canvas`]. Set via
        /// [`Element::canvas_left`] / [`Element::canvas_top`] / [`Element::canvas_z_index`].
        #[derive(Copy, Clone, Debug, PartialEq, Default)]
        pub struct CanvasPosition {
            pub left: f64,
            pub top: f64,
            pub z_index: i32,
        }

        impl Widget for Canvas {
            widget_header!(ControlKind::Canvas);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::canvas_bindings(self)
            }
            fn children(&self) -> Children<'_> {
                Children::Keyed(&self.children)
            }
        }
    }
    pub use canvas::*;
    pub(crate) mod check_box {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct CheckBox {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub is_checked: bool,
            pub on_checked: Option<Callback<bool>>,
            pub content: Option<String>,
            pub is_enabled: bool,
        }
        impl CheckBox {
            pub fn new(is_checked: bool) -> Self {
                Self {
                    is_checked,
                    is_enabled: true,
                    ..Default::default()
                }
            }
        }

        impl Widget for CheckBox {
            widget_header!(ControlKind::CheckBox);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::check_box_bindings(self)
            }
        }

        impl CheckBox {
            pub fn on_checked(mut self, f: impl IntoCallback<bool>) -> Self {
                self.on_checked = Some(f.into_callback());
                self
            }

            pub fn content(mut self, s: impl Into<String>) -> Self {
                self.content = Some(s.into());
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }
        }

        pub fn check_box(is_checked: bool) -> CheckBox {
            CheckBox::new(is_checked)
        }
    }
    pub use check_box::*;
    pub(crate) mod color_picker {
        use super::*;

        /// ARGB color value for the `ColorPicker` widget.
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct ColorArgb {
            pub a: u8,
            pub r: u8,
            pub g: u8,
            pub b: u8,
        }

        impl ColorArgb {
            pub fn new(r: u8, g: u8, b: u8) -> Self {
                Self { a: 255, r, g, b }
            }

            pub fn with_alpha(a: u8, r: u8, g: u8, b: u8) -> Self {
                Self { a, r, g, b }
            }
        }

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct ColorPicker {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub color: ColorArgb,
            pub is_alpha_enabled: bool,
            pub is_hex_input_visible: bool,
            pub is_color_slider_visible: bool,
            pub is_color_channel_text_input_visible: bool,
            pub on_color_changed: Option<Callback<(u8, u8, u8, u8)>>,
        }

        impl ColorPicker {
            pub fn new(color: ColorArgb) -> Self {
                Self {
                    color,
                    is_alpha_enabled: true,
                    is_hex_input_visible: true,
                    is_color_slider_visible: true,
                    is_color_channel_text_input_visible: true,
                    ..Default::default()
                }
            }

            pub fn alpha_enabled(mut self, v: bool) -> Self {
                self.is_alpha_enabled = v;
                self
            }

            pub fn hex_input_visible(mut self, v: bool) -> Self {
                self.is_hex_input_visible = v;
                self
            }

            pub fn color_slider_visible(mut self, v: bool) -> Self {
                self.is_color_slider_visible = v;
                self
            }

            pub fn color_channel_text_input_visible(mut self, v: bool) -> Self {
                self.is_color_channel_text_input_visible = v;
                self
            }

            pub fn on_color_changed(mut self, f: impl IntoCallback<(u8, u8, u8, u8)>) -> Self {
                self.on_color_changed = Some(f.into_callback());
                self
            }
        }

        impl Widget for ColorPicker {
            widget_header!(ControlKind::ColorPicker);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::color_picker_bindings(self);
                // ColorValue is a compound ARGB type not expressible in TOML.
                out.push(Binding::Prop(
                    Prop::ColorValue,
                    PropValue::Color {
                        a: self.color.a,
                        r: self.color.r,
                        g: self.color.g,
                        b: self.color.b,
                    },
                ));
                out
            }
        }

        pub fn color_picker(color: ColorArgb) -> ColorPicker {
            ColorPicker::new(color)
        }
    }
    pub use color_picker::*;
    pub(crate) mod combo_box {
        use super::*;

        /// W4 — `Windows.UI.Xaml.Controls.ComboBox`. Drop-down picker over a
        /// flat list of string items.
        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct ComboBox {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub items: Vec<String>,
            pub selected_index: i32,
            pub header: Option<String>,
            pub placeholder_text: String,
            pub is_editable: bool,
            pub is_enabled: bool,
            pub on_selection_changed: Option<Callback<i32>>,
        }
        impl ComboBox {
            pub fn new<I, S>(items: I) -> Self
            where
                I: IntoIterator<Item = S>,
                S: Into<String>,
            {
                Self {
                    items: items.into_iter().map(Into::into).collect(),
                    selected_index: -1,
                    is_enabled: true,
                    ..Default::default()
                }
            }
            pub fn selected_index(mut self, i: i32) -> Self {
                self.selected_index = i;
                self
            }
            pub fn header(mut self, s: impl Into<String>) -> Self {
                self.header = Some(s.into());
                self
            }
            pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
                self.placeholder_text = s.into();
                self
            }
            /// Enable the text-entry mode (`IComboBox::IsEditable`).
            pub fn editable(mut self, v: bool) -> Self {
                self.is_editable = v;
                self
            }
            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }
            pub fn on_selection_changed(mut self, f: impl IntoCallback<i32>) -> Self {
                self.on_selection_changed = Some(f.into_callback());
                self
            }
        }

        impl Widget for ComboBox {
            widget_header!(ControlKind::ComboBox);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::combo_box_bindings(self);
                out.push(Binding::Prop(
                    Prop::Items,
                    PropValue::StrList(self.items.clone()),
                ));
                out
            }
        }
    }
    pub use combo_box::*;
    pub(crate) mod command_bar {
        use super::*;

        /// Definition of a single command in a [`CommandBar`].
        #[derive(Clone, Debug, PartialEq)]
        pub enum CommandBarCommandDef {
            /// A clickable button with optional icon and label.
            Button {
                label: String,
                icon: Option<SymbolGlyph>,
            },
            /// A toggle button with optional icon and label.
            Toggle {
                label: String,
                icon: Option<SymbolGlyph>,
            },
            /// A visual separator.
            Separator,
        }

        /// Builder for a command bar button.
        pub fn app_bar_button(label: impl Into<String>) -> CommandBarCommandDef {
            CommandBarCommandDef::Button {
                label: label.into(),
                icon: None,
            }
        }

        /// Builder for a command bar button with icon.
        pub fn app_bar_button_icon(
            label: impl Into<String>,
            icon: SymbolGlyph,
        ) -> CommandBarCommandDef {
            CommandBarCommandDef::Button {
                label: label.into(),
                icon: Some(icon),
            }
        }

        /// Builder for a command bar toggle button.
        pub fn app_bar_toggle(label: impl Into<String>) -> CommandBarCommandDef {
            CommandBarCommandDef::Toggle {
                label: label.into(),
                icon: None,
            }
        }

        /// Builder for a command bar toggle button with icon.
        pub fn app_bar_toggle_icon(
            label: impl Into<String>,
            icon: SymbolGlyph,
        ) -> CommandBarCommandDef {
            CommandBarCommandDef::Toggle {
                label: label.into(),
                icon: Some(icon),
            }
        }

        /// Builder for a command bar separator.
        pub fn app_bar_separator() -> CommandBarCommandDef {
            CommandBarCommandDef::Separator
        }

        pub use crate::bindings::CommandBarDefaultLabelPosition;

        /// `Windows.UI.Xaml.Controls.CommandBar`. A toolbar with primary and secondary commands.
        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct CommandBar {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub primary_commands: Vec<CommandBarCommandDef>,
            pub secondary_commands: Vec<CommandBarCommandDef>,
            pub default_label_position: CommandBarDefaultLabelPosition,
            pub on_click: Option<Callback<String>>,
        }

        impl CommandBar {
            pub fn new(primary: Vec<CommandBarCommandDef>) -> Self {
                Self {
                    primary_commands: primary,
                    ..Default::default()
                }
            }

            pub fn secondary_commands(mut self, cmds: Vec<CommandBarCommandDef>) -> Self {
                self.secondary_commands = cmds;
                self
            }

            pub fn default_label_position(mut self, pos: CommandBarDefaultLabelPosition) -> Self {
                self.default_label_position = pos;
                self
            }

            pub fn on_click(mut self, f: impl IntoCallback<String>) -> Self {
                self.on_click = Some(f.into_callback());
                self
            }
        }

        impl Widget for CommandBar {
            widget_header!(ControlKind::CommandBar);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::command_bar_bindings(self);
                out.push(Binding::Prop(
                    Prop::PrimaryCommands,
                    PropValue::CommandBarCommands(self.primary_commands.clone()),
                ));
                out.push(Binding::Prop(
                    Prop::SecondaryCommands,
                    PropValue::CommandBarCommands(self.secondary_commands.clone()),
                ));
                out
            }
        }

        pub fn command_bar(primary: Vec<CommandBarCommandDef>) -> CommandBar {
            CommandBar::new(primary)
        }
    }
    pub use command_bar::*;
    pub(crate) mod content_dialog {
        use super::*;

        /// `Windows.UI.Xaml.Controls.ContentDialog` — modal popup with up to
        /// three buttons (primary / secondary / close). Not part of the visual
        /// tree: when `is_open` flips to true the backend calls `ShowAsync` and
        /// raises `on_closed(result)` once the user dismisses it.
        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct ContentDialog {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub title: Option<String>,
            pub content: Option<String>,
            pub primary_button_text: String,
            pub secondary_button_text: String,
            pub close_button_text: String,
            pub is_primary_button_enabled: bool,
            pub is_secondary_button_enabled: bool,
            pub is_open: bool,
            pub on_closed: Option<Callback<ContentDialogResult>>,
        }

        /// Mirrors `Windows.UI.Xaml.Controls.ContentDialogResult`. WinUI
        /// reports `None` for both close-button clicks and system dismissals
        /// (Escape, back gesture).
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
        pub enum ContentDialogResult {
            #[default]
            None,
            Primary,
            Secondary,
        }

        impl ContentDialogResult {
            pub fn from_i32(v: i32) -> Self {
                match v {
                    1 => Self::Primary,
                    2 => Self::Secondary,
                    _ => Self::None,
                }
            }
        }

        impl ContentDialog {
            pub fn new(title: impl Into<String>) -> Self {
                Self {
                    title: Some(title.into()),
                    is_primary_button_enabled: true,
                    is_secondary_button_enabled: true,
                    ..Default::default()
                }
            }
            pub fn content(mut self, s: impl Into<String>) -> Self {
                self.content = Some(s.into());
                self
            }
            pub fn primary_button_text(mut self, s: impl Into<String>) -> Self {
                self.primary_button_text = s.into();
                self
            }
            pub fn secondary_button_text(mut self, s: impl Into<String>) -> Self {
                self.secondary_button_text = s.into();
                self
            }
            pub fn close_button_text(mut self, s: impl Into<String>) -> Self {
                self.close_button_text = s.into();
                self
            }
            pub fn is_primary_button_enabled(mut self, v: bool) -> Self {
                self.is_primary_button_enabled = v;
                self
            }
            pub fn is_secondary_button_enabled(mut self, v: bool) -> Self {
                self.is_secondary_button_enabled = v;
                self
            }
            pub fn is_open(mut self, v: bool) -> Self {
                self.is_open = v;
                self
            }
            pub fn on_closed(mut self, f: impl IntoCallback<ContentDialogResult>) -> Self {
                self.on_closed = Some(f.into_callback());
                self
            }
        }

        impl Widget for ContentDialog {
            widget_header!(ControlKind::ContentDialog);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::content_dialog_bindings(self);
                // Closed event needs ContentDialogResult wrapping (i32 → enum).
                let closed_cb = self.on_closed.clone().map(|cb| {
                    EventHandler::I32(Callback::new(move |i: i32| {
                        cb.invoke(ContentDialogResult::from_i32(i));
                    }))
                });
                out.push(Binding::Event(Event::Closed, closed_cb));
                out.push(Binding::Prop(Prop::IsOpen, PropValue::Bool(self.is_open)));
                out
            }
        }
    }
    pub use content_dialog::*;
    pub(crate) mod date_picker {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct DatePicker {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub header: Option<String>,
            pub day_visible: bool,
            pub month_visible: bool,
            pub year_visible: bool,
            pub is_enabled: bool,
            pub on_selected_date_changed: Option<Callback<windows_time::DateTime>>,
        }

        impl DatePicker {
            pub fn new() -> Self {
                Self {
                    day_visible: true,
                    month_visible: true,
                    year_visible: true,
                    is_enabled: true,
                    ..Default::default()
                }
            }

            pub fn header(mut self, s: impl Into<String>) -> Self {
                self.header = Some(s.into());
                self
            }

            pub fn day_visible(mut self, v: bool) -> Self {
                self.day_visible = v;
                self
            }

            pub fn month_visible(mut self, v: bool) -> Self {
                self.month_visible = v;
                self
            }

            pub fn year_visible(mut self, v: bool) -> Self {
                self.year_visible = v;
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }

            pub fn on_selected_date_changed(
                mut self,
                f: impl IntoCallback<windows_time::DateTime>,
            ) -> Self {
                self.on_selected_date_changed = Some(f.into_callback());
                self
            }
        }

        impl Widget for DatePicker {
            widget_header!(ControlKind::DatePicker);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::date_picker_bindings(self)
            }
        }

        pub fn date_picker() -> DatePicker {
            DatePicker::new()
        }
    }
    pub use date_picker::*;
    pub(crate) mod drop_down_button {
        use super::menu_bar::MenuItemDef;
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct DropDownButton {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub content: Option<String>,
            pub is_enabled: bool,
            pub on_click: Option<Callback<()>>,
            pub menu_flyout_items: Option<Vec<MenuItemDef>>,
            pub on_item_clicked: Option<Callback<String>>,
        }

        impl DropDownButton {
            pub fn new(content: impl Into<String>) -> Self {
                Self {
                    content: Some(content.into()),
                    is_enabled: true,
                    ..Default::default()
                }
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }

            pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self {
                self.on_click = Some(Callback::new(move |()| f()));
                self
            }

            pub fn menu_flyout(mut self, items: Vec<MenuItemDef>) -> Self {
                self.menu_flyout_items = Some(items);
                self
            }

            pub fn on_item_clicked<F: Fn(String) + 'static>(mut self, f: F) -> Self {
                self.on_item_clicked = Some(Callback::new(f));
                self
            }
        }

        impl Widget for DropDownButton {
            widget_header!(ControlKind::DropDownButton);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::drop_down_button_bindings(self);
                if let Some(v) = &self.menu_flyout_items {
                    out.push(Binding::Prop(
                        Prop::MenuFlyoutItems,
                        PropValue::MenuFlyoutItems(v.clone()),
                    ));
                }
                out
            }
        }

        pub fn drop_down_button(content: impl Into<String>) -> DropDownButton {
            DropDownButton::new(content)
        }
    }
    pub use drop_down_button::*;
    pub(crate) mod flyout {
        pub use crate::bindings::FlyoutPlacementMode;
        /// Compatibility alias for `FlyoutPlacementMode`.
        pub type FlyoutPlacement = FlyoutPlacementMode;

        /// Describes the content for a Flyout attached to a button.
        /// Currently supports text content; element-tree content is planned.
        #[derive(Clone, Debug, PartialEq)]
        pub struct FlyoutDef {
            pub text: String,
            pub placement: FlyoutPlacement,
        }
    }
    pub use flyout::*;
    pub(crate) mod grid {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct Grid {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub rows: Vec<GridLength>,
            pub columns: Vec<GridLength>,
            pub row_spacing: f64,
            pub column_spacing: f64,
            pub children: Vec<Element>,
        }

        impl Widget for Grid {
            widget_header!(ControlKind::Grid);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::grid_bindings(self);
                if !self.rows.is_empty() {
                    out.push(Binding::Prop(
                        Prop::GridRows,
                        PropValue::GridLengths(self.rows.clone()),
                    ));
                }
                if !self.columns.is_empty() {
                    out.push(Binding::Prop(
                        Prop::GridColumns,
                        PropValue::GridLengths(self.columns.clone()),
                    ));
                }
                out
            }
            fn children(&self) -> Children<'_> {
                Children::Keyed(&self.children)
            }
        }

        impl Grid {
            pub fn rows<I: IntoIterator<Item = GridLength>>(mut self, it: I) -> Self {
                self.rows = it.into_iter().collect();
                self
            }

            pub fn columns<I: IntoIterator<Item = GridLength>>(mut self, it: I) -> Self {
                self.columns = it.into_iter().collect();
                self
            }

            pub fn row_spacing(mut self, v: f64) -> Self {
                self.row_spacing = v;
                self
            }

            pub fn column_spacing(mut self, v: f64) -> Self {
                self.column_spacing = v;
                self
            }
        }

        pub fn grid(children: impl crate::into_elements::IntoElements) -> Grid {
            Grid {
                children: children.into_elements(),
                ..Grid::default()
            }
        }
    }
    pub use grid::*;
    pub(crate) mod hyperlink_button {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct HyperlinkButton {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub content: String,
            pub navigate_uri: Option<String>,
            pub on_click: Option<Callback<()>>,
            pub is_enabled: bool,
        }
        impl HyperlinkButton {
            pub fn new(content: impl Into<String>) -> Self {
                Self {
                    content: content.into(),
                    is_enabled: true,
                    ..Default::default()
                }
            }
            pub fn navigate_uri(mut self, uri: impl Into<String>) -> Self {
                self.navigate_uri = Some(uri.into());
                self
            }
            pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self {
                self.on_click = Some(Callback::new(move |()| f()));
                self
            }
            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }
        }

        impl Widget for HyperlinkButton {
            widget_header!(ControlKind::HyperlinkButton);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::hyperlink_button_bindings(self);
                if let Some(v) = &self.navigate_uri {
                    out.push(Binding::Prop(Prop::NavigateUri, PropValue::Str(v.clone())));
                }
                out
            }
        }
    }
    pub use hyperlink_button::*;
    pub(crate) mod image {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct Image {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub source: ImageSource,
            pub stretch: Stretch,
        }
        impl Default for Image {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    source: ImageSource::default(),
                    stretch: Stretch::Uniform,
                }
            }
        }
        #[derive(Clone, Default, Debug, PartialEq)]
        pub enum ImageSource {
            #[default]
            None,
            Uri(String),
        }
        pub use crate::bindings::Stretch;
        impl Image {
            pub fn new(source: ImageSource) -> Self {
                Self {
                    source,
                    ..Default::default()
                }
            }

            pub fn new_with_uri(source: impl Into<String>) -> Self {
                Self::new(ImageSource::Uri(source.into()))
            }

            pub fn stretch(mut self, v: Stretch) -> Self {
                self.stretch = v;
                self
            }
        }

        impl Widget for Image {
            widget_header!(ControlKind::Image);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::image_bindings(self);
                // ImageSource is a compound type not expressible in TOML.
                match &self.source {
                    ImageSource::Uri(uri) => {
                        out.push(Binding::Prop(
                            Prop::ImageSource,
                            PropValue::Str(uri.clone()),
                        ));
                    }
                    ImageSource::None => {}
                }
                out
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            fn image_source(image: &Image) -> Option<PropValue> {
                image.bindings().into_iter().find_map(|b| match b {
                    Binding::Prop(Prop::ImageSource, v) => Some(v),
                    _ => None,
                })
            }

            #[test]
            fn new_emits_uri_image_source() {
                let image = Image::new_with_uri("file:///pic.png");
                assert_eq!(
                    image_source(&image),
                    Some(PropValue::Str("file:///pic.png".into()))
                );
            }

            #[test]
            fn default_emits_no_image_source() {
                // An `Image` with no source emits no `ImageSource` binding.
                // Stretch is always emitted (it's a non-optional enum field).
                let image = Image::default();
                assert_eq!(image_source(&image), None);
                assert!(
                    !image
                        .bindings()
                        .iter()
                        .any(|b| matches!(b, Binding::Prop(Prop::ImageSource, ..)))
                );
            }
        }
    }
    pub use image::*;
    pub(crate) mod list_box {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct ListBox {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub items: Vec<String>,
            pub selected_index: i32,
            pub is_enabled: bool,
            pub on_selection_changed: Option<Callback<i32>>,
        }

        impl Default for ListBox {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    items: Vec::new(),
                    selected_index: -1,
                    is_enabled: true,
                    on_selection_changed: None,
                }
            }
        }

        impl ListBox {
            pub fn new() -> Self {
                Self {
                    is_enabled: true,
                    ..Default::default()
                }
            }

            pub fn items(mut self, items: impl IntoIterator<Item = impl Into<String>>) -> Self {
                self.items = items.into_iter().map(Into::into).collect();
                self
            }

            pub fn selected_index(mut self, idx: i32) -> Self {
                self.selected_index = idx;
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }

            pub fn on_selection_changed(mut self, f: impl IntoCallback<i32>) -> Self {
                self.on_selection_changed = Some(f.into_callback());
                self
            }
        }

        impl Widget for ListBox {
            widget_header!(ControlKind::ListBox);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::list_box_bindings(self);
                out.push(Binding::Prop(
                    Prop::Items,
                    PropValue::StrList(self.items.clone()),
                ));
                out
            }
        }

        pub fn list_box() -> ListBox {
            ListBox::new()
        }
    }
    pub use list_box::*;
    pub(crate) mod menu_bar {
        use super::*;

        /// Definition of a single item within a menu (used by both [`MenuBar`]
        /// and the menu flyout modifier on buttons).
        #[derive(Clone, Debug, PartialEq)]
        pub enum MenuItemDef {
            /// A clickable menu item with a text label.
            Item { text: String },
            /// A visual separator line.
            Separator,
            /// A submenu containing nested items.
            SubItem {
                text: String,
                children: Vec<MenuItemDef>,
            },
        }

        /// Builder for a [`MenuItemDef::Item`].
        pub fn menu_item(text: impl Into<String>) -> MenuItemDef {
            MenuItemDef::Item { text: text.into() }
        }

        /// Builder for a [`MenuItemDef::Separator`].
        pub fn menu_separator() -> MenuItemDef {
            MenuItemDef::Separator
        }

        /// Builder for a [`MenuItemDef::SubItem`].
        pub fn menu_sub_item(text: impl Into<String>, children: Vec<MenuItemDef>) -> MenuItemDef {
            MenuItemDef::SubItem {
                text: text.into(),
                children,
            }
        }

        /// Definition of a top-level menu in a [`MenuBar`].
        #[derive(Clone, Debug, PartialEq)]
        pub struct MenuBarItemDef {
            pub title: String,
            pub items: Vec<MenuItemDef>,
        }

        impl MenuBarItemDef {
            pub fn new(title: impl Into<String>, items: Vec<MenuItemDef>) -> Self {
                Self {
                    title: title.into(),
                    items,
                }
            }
        }

        /// Builder for a [`MenuBarItemDef`].
        pub fn menu_bar_item(title: impl Into<String>, items: Vec<MenuItemDef>) -> MenuBarItemDef {
            MenuBarItemDef::new(title, items)
        }

        /// `Windows.UI.Xaml.Controls.MenuBar`. A horizontal bar of dropdown menus.
        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct MenuBar {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub items: Vec<MenuBarItemDef>,
            pub on_item_clicked: Option<Callback<String>>,
        }

        impl MenuBar {
            pub fn new(items: Vec<MenuBarItemDef>) -> Self {
                Self {
                    items,
                    ..Default::default()
                }
            }

            pub fn on_item_clicked<F: Fn(String) + 'static>(mut self, f: F) -> Self {
                self.on_item_clicked = Some(Callback::new(f));
                self
            }
        }

        impl Widget for MenuBar {
            widget_header!(ControlKind::MenuBar);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::menu_bar_bindings(self);
                out.push(Binding::Prop(
                    Prop::Items,
                    PropValue::MenuBarItems(self.items.clone()),
                ));
                out
            }
        }

        pub fn menu_bar(items: Vec<MenuBarItemDef>) -> MenuBar {
            MenuBar::new(items)
        }
    }
    pub use menu_bar::*;
    pub(crate) mod password_box {
        use super::*;

        /// W2 — `Windows.UI.Xaml.Controls.PasswordBox`. Single-line password
        /// entry with an optional reveal button.
        #[derive(Clone, Debug, PartialEq)]
        pub struct PasswordBox {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub value: String,
            pub placeholder_text: String,
            pub header: Option<String>,
            pub is_enabled: bool,
            pub is_password_reveal_button_enabled: bool,
            pub password_reveal_mode: PasswordRevealMode,
            pub on_password_changed: Option<Callback<String>>,
        }
        pub use crate::bindings::PasswordRevealMode;
        impl Default for PasswordBox {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    value: String::new(),
                    placeholder_text: String::new(),
                    header: None,
                    is_enabled: true,
                    is_password_reveal_button_enabled: true,
                    password_reveal_mode: PasswordRevealMode::Peek,
                    on_password_changed: None,
                }
            }
        }
        impl PasswordBox {
            pub fn new() -> Self {
                Self::default()
            }
            pub fn value(mut self, s: impl Into<String>) -> Self {
                self.value = s.into();
                self
            }
            pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
                self.placeholder_text = s.into();
                self
            }
            pub fn header(mut self, s: impl Into<String>) -> Self {
                self.header = Some(s.into());
                self
            }
            pub fn password_reveal_mode(mut self, mode: PasswordRevealMode) -> Self {
                self.password_reveal_mode = mode;
                self
            }
            pub fn reveal_button_enabled(mut self, v: bool) -> Self {
                self.is_password_reveal_button_enabled = v;
                self
            }
            pub fn on_password_changed(mut self, f: impl IntoCallback<String>) -> Self {
                self.on_password_changed = Some(f.into_callback());
                self
            }
            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }
        }

        impl Widget for PasswordBox {
            widget_header!(ControlKind::PasswordBox);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::password_box_bindings(self);
                out.push(Binding::Prop(
                    Prop::Value,
                    PropValue::Str(self.value.clone()),
                ));
                out
            }
        }
    }
    pub use password_box::*;
    pub(crate) mod person_picture {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct PersonPicture {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub display_name: String,
            pub initials: String,
        }
        impl PersonPicture {
            pub fn new() -> Self {
                Self::default()
            }
            pub fn display_name(mut self, s: impl Into<String>) -> Self {
                self.display_name = s.into();
                self
            }
            pub fn initials(mut self, s: impl Into<String>) -> Self {
                self.initials = s.into();
                self
            }
        }

        impl Widget for PersonPicture {
            widget_header!(ControlKind::PersonPicture);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::person_picture_bindings(self)
            }
        }
    }
    pub use person_picture::*;
    pub(crate) mod pivot {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct PivotItem {
            pub key: Option<String>,
            pub header: String,
            pub content: Element,
        }
        impl PivotItem {
            pub fn new(header: impl Into<String>, content: impl Into<Element>) -> Self {
                Self {
                    key: None,
                    header: header.into(),
                    content: content.into(),
                }
            }
        }
        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct Pivot {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub items: Vec<PivotItem>,
            pub selected_index: i32,
            pub on_selection_changed: Option<Callback<i32>>,
            pub title: Option<String>,
        }
        impl Pivot {
            pub fn new<I: IntoIterator<Item = PivotItem>>(items: I) -> Self {
                Self {
                    items: items.into_iter().collect(),
                    ..Default::default()
                }
            }
            pub fn selected_index(mut self, i: i32) -> Self {
                self.selected_index = i;
                self
            }
            pub fn on_selection_changed(mut self, f: impl IntoCallback<i32>) -> Self {
                self.on_selection_changed = Some(f.into_callback());
                self
            }
            pub fn title(mut self, s: impl Into<String>) -> Self {
                self.title = Some(s.into());
                self
            }
        }

        impl PivotItem {
            pub fn with_key(mut self, key: impl Into<String>) -> Self {
                self.key = Some(key.into());
                self
            }
        }

        impl Widget for Pivot {
            widget_header!(ControlKind::Pivot);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::pivot_bindings(self)
            }
            fn children(&self) -> Children<'_> {
                Children::PivotItems(&self.items)
            }
        }
    }
    pub use pivot::*;
    pub(crate) mod progress_bar {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct ProgressBar {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub value: f64,
            pub minimum: f64,
            pub maximum: f64,
            pub is_indeterminate: bool,
        }
        impl Default for ProgressBar {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    value: 0.0,
                    minimum: 0.0,
                    maximum: 100.0,
                    is_indeterminate: false,
                }
            }
        }
        impl ProgressBar {
            pub fn new(value: f64) -> Self {
                Self {
                    value,
                    ..Default::default()
                }
            }
            pub fn indeterminate() -> Self {
                Self {
                    is_indeterminate: true,
                    ..Default::default()
                }
            }
            pub fn range(mut self, min: f64, max: f64) -> Self {
                self.minimum = min;
                self.maximum = max;
                self
            }
        }

        impl Widget for ProgressBar {
            widget_header!(ControlKind::ProgressBar);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::progress_bar_bindings(self)
            }
        }
    }
    pub use progress_bar::*;
    pub(crate) mod progress_ring {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct ProgressRing {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub is_active: bool,
        }
        impl Default for ProgressRing {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    is_active: true,
                }
            }
        }
        impl ProgressRing {
            pub fn new() -> Self {
                Self::default()
            }
            pub fn indeterminate() -> Self {
                Self::default()
            }
            pub fn active(mut self, active: bool) -> Self {
                self.is_active = active;
                self
            }
        }

        impl Widget for ProgressRing {
            widget_header!(ControlKind::ProgressRing);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::progress_ring_bindings(self)
            }
        }
    }
    pub use progress_ring::*;
    pub(crate) mod radio_button {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct RadioButton {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub content: Option<String>,
            pub is_checked: bool,
            pub on_checked: Option<Callback<()>>,
            pub group_name: String,
            pub is_enabled: bool,
        }
        impl RadioButton {
            pub fn new(content: impl Into<String>) -> Self {
                Self {
                    content: Some(content.into()),
                    is_enabled: true,
                    ..Default::default()
                }
            }
            pub fn checked(mut self, v: bool) -> Self {
                self.is_checked = v;
                self
            }
            pub fn on_checked<F: Fn() + 'static>(mut self, f: F) -> Self {
                self.on_checked = Some(Callback::new(move |()| f()));
                self
            }
            pub fn group(mut self, s: impl Into<String>) -> Self {
                self.group_name = s.into();
                self
            }
            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }
        }

        impl Widget for RadioButton {
            widget_header!(ControlKind::RadioButton);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::radio_button_bindings(self)
            }
        }
    }
    pub use radio_button::*;
    pub(crate) mod rating_control {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct RatingControl {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub value: f64,
            pub max_rating: i32,
            pub caption: String,
            pub placeholder_value: f64,
            pub is_read_only: bool,
            pub on_value_changed: Option<Callback<f64>>,
        }

        impl Default for RatingControl {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    value: 0.0,
                    max_rating: 5,
                    caption: String::new(),
                    placeholder_value: -1.0,
                    is_read_only: false,
                    on_value_changed: None,
                }
            }
        }

        impl RatingControl {
            pub fn new(value: f64) -> Self {
                Self {
                    value,
                    ..Default::default()
                }
            }

            pub fn max_rating(mut self, max: i32) -> Self {
                self.max_rating = max;
                self
            }

            pub fn caption(mut self, s: impl Into<String>) -> Self {
                self.caption = s.into();
                self
            }

            pub fn placeholder_value(mut self, v: f64) -> Self {
                self.placeholder_value = v;
                self
            }

            pub fn read_only(mut self) -> Self {
                self.is_read_only = true;
                self
            }

            pub fn on_value_changed(mut self, f: impl IntoCallback<f64>) -> Self {
                self.on_value_changed = Some(f.into_callback());
                self
            }
        }

        impl Widget for RatingControl {
            widget_header!(ControlKind::RatingControl);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::rating_control_bindings(self)
            }
        }

        pub fn rating_control(value: f64) -> RatingControl {
            RatingControl::new(value)
        }
    }
    pub use rating_control::*;
    pub(crate) mod relative_panel {
        use super::*;

        /// Attached properties for a child of [`RelativePanel`]. Controls
        /// alignment relative to the panel edges/center.
        #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
        pub struct RelativePanelAlignment {
            pub align_left_with_panel: bool,
            pub align_right_with_panel: bool,
            pub align_top_with_panel: bool,
            pub align_bottom_with_panel: bool,
            pub align_h_center_with_panel: bool,
            pub align_v_center_with_panel: bool,
        }

        /// `Windows.UI.Xaml.Controls.RelativePanel`. A constraint-based layout
        /// where children are positioned relative to the panel edges or center.
        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct RelativePanel {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub children: Vec<Element>,
        }

        impl RelativePanel {
            pub fn new<I>(children: I) -> Self
            where
                I: IntoIterator,
                I::Item: Into<Element>,
            {
                Self {
                    children: children.into_iter().map(Into::into).collect(),
                    ..Default::default()
                }
            }
        }

        impl Widget for RelativePanel {
            widget_header!(ControlKind::RelativePanel);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::relative_panel_bindings(self)
            }
            fn children(&self) -> Children<'_> {
                Children::Keyed(&self.children)
            }
        }

        pub fn relative_panel<I>(children: I) -> RelativePanel
        where
            I: IntoIterator,
            I::Item: Into<Element>,
        {
            RelativePanel::new(children)
        }
    }
    pub use relative_panel::*;
    pub(crate) mod repeat_button {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct RepeatButton {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub content: String,
            pub on_click: Option<Callback<()>>,
            pub is_enabled: bool,
            pub delay: i32,
            pub interval: i32,
        }

        impl Default for RepeatButton {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    content: String::new(),
                    on_click: None,
                    is_enabled: true,
                    delay: 500,
                    interval: 33,
                }
            }
        }

        impl RepeatButton {
            pub fn new(content: impl Into<String>) -> Self {
                Self {
                    content: content.into(),
                    is_enabled: true,
                    ..Default::default()
                }
            }

            pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self {
                self.on_click = Some(Callback::new(move |()| f()));
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }

            /// Initial delay in milliseconds before repeating begins (default 500).
            pub fn delay(mut self, ms: i32) -> Self {
                self.delay = ms;
                self
            }

            /// Interval in milliseconds between repeats (default 33).
            pub fn interval(mut self, ms: i32) -> Self {
                self.interval = ms;
                self
            }
        }

        impl Widget for RepeatButton {
            widget_header!(ControlKind::RepeatButton);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::repeat_button_bindings(self)
            }
        }

        pub fn repeat_button(content: impl Into<String>) -> RepeatButton {
            RepeatButton::new(content)
        }
    }
    pub use repeat_button::*;
    pub(crate) mod rich_edit_box {
        use super::*;

        /// `Windows.UI.Xaml.Controls.RichEditBox`. A multi-line rich text
        /// editor with optional header and placeholder text.
        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct RichEditBox {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub text: String,
            pub placeholder_text: String,
            pub header: Option<String>,
            pub is_read_only: bool,
            pub on_text_changed: Option<Callback<String>>,
        }

        impl RichEditBox {
            pub fn new(text: impl Into<String>) -> Self {
                Self {
                    text: text.into(),
                    ..Default::default()
                }
            }

            pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
                self.placeholder_text = s.into();
                self
            }

            pub fn header(mut self, s: impl Into<String>) -> Self {
                self.header = Some(s.into());
                self
            }

            pub fn read_only(mut self) -> Self {
                self.is_read_only = true;
                self
            }

            pub fn on_text_changed(mut self, f: impl IntoCallback<String>) -> Self {
                self.on_text_changed = Some(f.into_callback());
                self
            }
        }

        impl Widget for RichEditBox {
            widget_header!(ControlKind::RichEditBox);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::rich_edit_box_bindings(self);
                out.push(Binding::Prop(Prop::Text, PropValue::Str(self.text.clone())));
                out
            }
        }

        pub fn rich_edit_box(text: impl Into<String>) -> RichEditBox {
            RichEditBox::new(text)
        }
    }
    pub use rich_edit_box::*;
    pub(crate) mod scroll_view {
        use super::*;

        #[non_exhaustive]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct ScrollingScrollBarVisibility(pub i32);

        #[allow(non_upper_case_globals)]
        impl ScrollingScrollBarVisibility {
            pub const Auto: Self = Self(1);
            pub const Visible: Self = Self(3);
            pub const Hidden: Self = Self(2);
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct ScrollView {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub child: Box<Element>,
            pub horizontal_scroll_bar_visibility: ScrollingScrollBarVisibility,
            pub vertical_scroll_bar_visibility: ScrollingScrollBarVisibility,
        }

        impl Default for ScrollView {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    child: Box::new(Element::Empty),
                    horizontal_scroll_bar_visibility: ScrollingScrollBarVisibility::Hidden,
                    vertical_scroll_bar_visibility: ScrollingScrollBarVisibility::Auto,
                }
            }
        }

        impl ScrollView {
            pub fn new(child: impl Into<Element>) -> Self {
                Self {
                    child: Box::new(child.into()),
                    ..Default::default()
                }
            }

            pub fn horizontal_scroll_bar_visibility(
                mut self,
                v: ScrollingScrollBarVisibility,
            ) -> Self {
                self.horizontal_scroll_bar_visibility = v;
                self
            }

            pub fn vertical_scroll_bar_visibility(
                mut self,
                v: ScrollingScrollBarVisibility,
            ) -> Self {
                self.vertical_scroll_bar_visibility = v;
                self
            }
        }

        impl Widget for ScrollView {
            widget_header!(ControlKind::ScrollView);

            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::scroll_view_bindings(self)
            }

            fn children(&self) -> Children<'_> {
                Children::PositionalSingle(&self.child)
            }
        }

        pub fn scroll_view(child: impl Into<Element>) -> ScrollView {
            ScrollView::new(child)
        }
    }
    pub use scroll_view::*;
    pub(crate) mod scroll_viewer {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct ScrollViewer {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub child: Box<Element>,
            pub horizontal_scroll_bar_visibility: ScrollBarVisibility,
            pub vertical_scroll_bar_visibility: ScrollBarVisibility,
        }
        impl Default for ScrollViewer {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    child: Box::new(Element::Empty),
                    horizontal_scroll_bar_visibility: ScrollBarVisibility::Disabled,
                    vertical_scroll_bar_visibility: ScrollBarVisibility::Auto,
                }
            }
        }
        impl ScrollViewer {
            pub fn new(child: impl Into<Element>) -> Self {
                Self {
                    child: Box::new(child.into()),
                    ..Default::default()
                }
            }
        }

        impl Widget for ScrollViewer {
            widget_header!(ControlKind::ScrollViewer);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::scroll_viewer_bindings(self)
            }
            fn children(&self) -> Children<'_> {
                Children::PositionalSingle(&self.child)
            }
        }

        impl ScrollViewer {
            pub fn horizontal_scroll_bar_visibility(mut self, v: ScrollBarVisibility) -> Self {
                self.horizontal_scroll_bar_visibility = v;
                self
            }

            pub fn vertical_scroll_bar_visibility(mut self, v: ScrollBarVisibility) -> Self {
                self.vertical_scroll_bar_visibility = v;
                self
            }
        }

        pub fn scroll_viewer(child: impl Into<Element>) -> ScrollViewer {
            ScrollViewer::new(child)
        }
    }
    pub use scroll_viewer::*;
    pub(crate) mod selector_bar {
        use super::*;

        /// Definition of a single item in a [`SelectorBar`].
        #[derive(Clone, Debug, PartialEq)]
        pub struct SelectorBarItemDef {
            /// Display text.
            pub text: String,
            /// Optional symbol icon.
            pub icon: Option<SymbolGlyph>,
        }

        impl SelectorBarItemDef {
            pub fn new(text: impl Into<String>) -> Self {
                Self {
                    text: text.into(),
                    icon: None,
                }
            }

            pub fn icon(mut self, icon: SymbolGlyph) -> Self {
                self.icon = Some(icon);
                self
            }
        }

        /// Builder for a [`SelectorBarItemDef`].
        pub fn selector_bar_item(text: impl Into<String>) -> SelectorBarItemDef {
            SelectorBarItemDef::new(text)
        }

        /// A horizontal tab-like selector backed by system XAML toggle buttons.
        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct SelectorBar {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub items: Vec<SelectorBarItemDef>,
            pub on_selection_changed: Option<Callback<String>>,
        }

        impl SelectorBar {
            pub fn new(items: Vec<SelectorBarItemDef>) -> Self {
                Self {
                    items,
                    ..Default::default()
                }
            }

            pub fn on_selection_changed(mut self, f: impl IntoCallback<String>) -> Self {
                self.on_selection_changed = Some(f.into_callback());
                self
            }
        }

        impl Widget for SelectorBar {
            widget_header!(ControlKind::SelectorBar);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::selector_bar_bindings(self);
                out.push(Binding::Prop(
                    Prop::Items,
                    PropValue::SelectorBarItems(self.items.clone()),
                ));
                out
            }
        }

        pub fn selector_bar(items: Vec<SelectorBarItemDef>) -> SelectorBar {
            SelectorBar::new(items)
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn bindings_include_upstream_selector_bar_surface() {
                let selector = selector_bar(vec![
                    selector_bar_item("Recent"),
                    selector_bar_item("Shared"),
                ])
                .on_selection_changed(|_: String| {});

                let bindings = selector.bindings();

                assert!(bindings.iter().any(|binding| matches!(
                    binding,
                    Binding::Event(Event::SelectionChanged, Some(EventHandler::Str(_)))
                )));
                let items = bindings.iter().find_map(|binding| match binding {
                    Binding::Prop(Prop::Items, PropValue::SelectorBarItems(items)) => Some(items),
                    _ => None,
                });
                let items = items.expect("SelectorBar should emit Items");
                assert_eq!(items[0].text, "Recent");
                assert_eq!(items[1].text, "Shared");
            }
        }
    }
    pub use selector_bar::*;
    pub(crate) mod shape {
        use super::*;

        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum ShapeKind {
            Rectangle,
            Ellipse,
            Line,
        }
        #[derive(Clone, Debug, PartialEq)]
        pub struct Shape {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub kind: ShapeKind,
            pub fill: Option<Brush>,
            pub stroke: Option<Brush>,
            pub stroke_thickness: Option<f64>,
            pub corner_radius: Option<f64>,
            pub line: LineEndpoints,
        }
        #[derive(Copy, Clone, Debug, PartialEq, Default)]
        pub struct LineEndpoints {
            pub x1: f64,
            pub y1: f64,
            pub x2: f64,
            pub y2: f64,
        }
        impl Default for Shape {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    kind: ShapeKind::Rectangle,
                    fill: None,
                    stroke: None,
                    stroke_thickness: None,
                    corner_radius: None,
                    line: LineEndpoints::default(),
                }
            }
        }
        impl Shape {
            pub fn rectangle() -> Self {
                Self {
                    kind: ShapeKind::Rectangle,
                    ..Default::default()
                }
            }
            pub fn ellipse() -> Self {
                Self {
                    kind: ShapeKind::Ellipse,
                    ..Default::default()
                }
            }
            pub fn line(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
                Self {
                    kind: ShapeKind::Line,
                    line: LineEndpoints { x1, y1, x2, y2 },
                    ..Default::default()
                }
            }
            pub fn fill(mut self, v: impl Into<Brush>) -> Self {
                self.fill = Some(v.into());
                self
            }
            pub fn fill_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
                self.fill = Some(Brush::Solid(Color::rgb(r, g, b)));
                self
            }
            pub fn stroke(mut self, v: impl Into<Brush>) -> Self {
                self.stroke = Some(v.into());
                self
            }
            pub fn stroke_thickness(mut self, v: f64) -> Self {
                self.stroke_thickness = Some(v);
                self
            }
            pub fn corner_radius(mut self, v: f64) -> Self {
                self.corner_radius = Some(v);
                self
            }
        }

        impl Widget for Shape {
            fn kind(&self) -> ControlKind {
                match self.kind {
                    ShapeKind::Rectangle => ControlKind::Rectangle,
                    ShapeKind::Ellipse => ControlKind::Ellipse,
                    ShapeKind::Line => ControlKind::Line,
                }
            }
            fn key(&self) -> Option<&str> {
                self.key.as_deref()
            }
            fn modifiers(&self) -> &Modifiers {
                &self.modifiers
            }
            fn bindings(&self) -> PropBindings {
                let mut out = Vec::with_capacity(5);
                if let Some(fill) = &self.fill {
                    out.push(Binding::Prop(Prop::Fill, PropValue::Brush(fill.clone())));
                }
                if let Some(stroke) = &self.stroke {
                    out.push(Binding::Prop(
                        Prop::Stroke,
                        PropValue::Brush(stroke.clone()),
                    ));
                }
                if let Some(th) = self.stroke_thickness {
                    out.push(Binding::Prop(Prop::StrokeThickness, PropValue::F64(th)));
                }
                if let Some(cr) = self.corner_radius
                    && matches!(self.kind, ShapeKind::Rectangle)
                {
                    out.push(Binding::Prop(Prop::CornerRadius, PropValue::F64(cr)));
                }
                if matches!(self.kind, ShapeKind::Line) {
                    out.push(Binding::Prop(
                        Prop::LineEndpoints,
                        PropValue::LineEndpoints(self.line),
                    ));
                }
                out
            }
        }
    }
    pub use shape::*;
    pub(crate) mod slider {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct Slider {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub value: f64,
            pub minimum: f64,
            pub maximum: f64,
            pub step: Option<f64>,
            pub on_value_changed: Option<Callback<f64>>,
            pub header: Option<String>,
            pub orientation: Orientation,
            pub is_enabled: bool,
        }
        impl Default for Slider {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    value: 0.0,
                    minimum: 0.0,
                    maximum: 100.0,
                    step: None,
                    on_value_changed: None,
                    header: None,
                    orientation: Orientation::Horizontal,
                    is_enabled: true,
                }
            }
        }
        impl Slider {
            pub fn new(value: f64) -> Self {
                Self {
                    value,
                    ..Default::default()
                }
            }
            pub fn range(mut self, min: f64, max: f64) -> Self {
                self.minimum = min;
                self.maximum = max;
                self
            }
            pub fn step(mut self, v: f64) -> Self {
                self.step = Some(v);
                self
            }
            pub fn on_value_changed(mut self, f: impl IntoCallback<f64>) -> Self {
                self.on_value_changed = Some(f.into_callback());
                self
            }
            pub fn header(mut self, s: impl Into<String>) -> Self {
                self.header = Some(s.into());
                self
            }
            /// Switch to a vertical slider (`ISlider::Orientation = Vertical`).
            pub fn vertical(mut self) -> Self {
                self.orientation = Orientation::Vertical;
                self
            }
            /// Switch to a horizontal slider (default).
            pub fn horizontal(mut self) -> Self {
                self.orientation = Orientation::Horizontal;
                self
            }
            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }
        }

        impl Widget for Slider {
            widget_header!(ControlKind::Slider);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::slider_bindings(self);
                if let Some(v) = self.step {
                    out.push(Binding::Prop(Prop::Step, PropValue::F64(v)));
                }
                out
            }
        }
    }
    pub use slider::*;
    pub(crate) mod split_button {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct SplitButton {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub content: Option<String>,
            pub is_enabled: bool,
            pub on_click: Option<Callback<()>>,
        }

        impl SplitButton {
            pub fn new(content: impl Into<String>) -> Self {
                Self {
                    content: Some(content.into()),
                    is_enabled: true,
                    ..Default::default()
                }
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }

            pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self {
                self.on_click = Some(Callback::new(move |()| f()));
                self
            }
        }

        impl Widget for SplitButton {
            widget_header!(ControlKind::SplitButton);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::split_button_bindings(self)
            }
        }

        pub fn split_button(content: impl Into<String>) -> SplitButton {
            SplitButton::new(content)
        }
    }
    pub use split_button::*;
    pub(crate) mod split_view {
        use super::*;

        /// Display mode for a [`SplitView`].
        #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
        pub enum SplitViewDisplayMode {
            /// Pane overlays content when open, hidden when closed.
            Overlay,
            /// Pane is always visible and pushes content aside.
            #[default]
            Inline,
            /// Pane shows as a compact strip when closed, overlays content when open.
            CompactOverlay,
            /// Pane shows as a compact strip when closed, pushes content when open.
            CompactInline,
        }

        /// `Windows.UI.Xaml.Controls.SplitView`. A container with a collapsible pane
        /// and a main content area.
        #[derive(Clone, Debug, PartialEq)]
        pub struct SplitView {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub content: Box<Element>,
            pub pane: Box<Element>,
            pub display_mode: SplitViewDisplayMode,
            pub is_pane_open: bool,
            pub open_pane_length: f64,
            pub compact_pane_length: f64,
            pub on_pane_closed: Option<Callback<()>>,
        }

        impl Default for SplitView {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    content: Box::new(Element::Empty),
                    pane: Box::new(Element::Empty),
                    display_mode: SplitViewDisplayMode::default(),
                    is_pane_open: true,
                    open_pane_length: 320.0,
                    compact_pane_length: 48.0,
                    on_pane_closed: None,
                }
            }
        }

        impl SplitView {
            pub fn new(content: impl Into<Element>) -> Self {
                Self {
                    content: Box::new(content.into()),
                    ..Default::default()
                }
            }

            pub fn pane(mut self, pane: impl Into<Element>) -> Self {
                self.pane = Box::new(pane.into());
                self
            }

            pub fn display_mode(mut self, mode: SplitViewDisplayMode) -> Self {
                self.display_mode = mode;
                self
            }

            pub fn is_pane_open(mut self, open: bool) -> Self {
                self.is_pane_open = open;
                self
            }

            pub fn open_pane_length(mut self, len: f64) -> Self {
                self.open_pane_length = len;
                self
            }

            pub fn compact_pane_length(mut self, len: f64) -> Self {
                self.compact_pane_length = len;
                self
            }

            pub fn on_pane_closed<F: Fn() + 'static>(mut self, f: F) -> Self {
                self.on_pane_closed = Some(Callback::new(move |()| f()));
                self
            }
        }

        impl Widget for SplitView {
            widget_header!(ControlKind::SplitView);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::split_view_bindings(self)
            }
            fn children(&self) -> Children<'_> {
                Children::PositionalSingle(&self.content)
            }
            fn pane_element(&self) -> Option<&Element> {
                Some(&self.pane)
            }
        }

        pub fn split_view(content: impl Into<Element>) -> SplitView {
            SplitView::new(content)
        }
    }
    pub use split_view::*;
    pub(crate) mod stack_panel {
        use super::*;

        pub use crate::bindings::Orientation;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct StackPanel {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub orientation: Orientation,
            pub spacing: f64,
            pub children: Vec<Element>,
        }
        impl StackPanel {
            pub fn vertical() -> Self {
                Self {
                    orientation: Orientation::Vertical,
                    ..Self::default()
                }
            }
            pub fn horizontal() -> Self {
                Self {
                    orientation: Orientation::Horizontal,
                    ..Self::default()
                }
            }
        }

        impl Widget for StackPanel {
            widget_header!(ControlKind::StackPanel);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::stack_panel_bindings(self)
            }
            fn children(&self) -> Children<'_> {
                Children::Keyed(&self.children)
            }
        }

        impl StackPanel {
            pub fn spacing(mut self, v: f64) -> Self {
                self.spacing = v;
                self
            }
        }

        pub fn vstack(children: impl crate::into_elements::IntoElements) -> StackPanel {
            let mut s = StackPanel::vertical();
            s.children = children.into_elements();
            s
        }

        pub fn hstack(children: impl crate::into_elements::IntoElements) -> StackPanel {
            let mut s = StackPanel::horizontal();
            s.children = children.into_elements();
            s
        }
    }
    pub use stack_panel::*;
    pub(crate) mod text_block {
        use super::*;

        pub use crate::bindings::TextWrapping;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct TextBlock {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub text: String,
            pub font_size: Option<f64>,
            pub font_weight: Option<u16>,
            pub text_wrapping: TextWrapping,
            pub is_text_selection_enabled: bool,
        }
        impl TextBlock {
            pub fn new(content: impl Into<String>) -> Self {
                Self {
                    text: content.into(),
                    ..Self::default()
                }
            }
        }

        impl Widget for TextBlock {
            widget_header!(ControlKind::TextBlock);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::text_block_bindings(self);
                if let Some(v) = self.font_size {
                    out.push(Binding::Prop(Prop::FontSize, PropValue::F64(v)));
                }
                if let Some(v) = self.font_weight {
                    out.push(Binding::Prop(Prop::FontWeight, PropValue::U16(v)));
                }
                out
            }
        }

        impl TextBlock {
            pub fn bold(mut self) -> Self {
                self.font_weight = Some(700);
                self
            }

            pub fn semibold(mut self) -> Self {
                self.font_weight = Some(600);
                self
            }

            pub fn font_weight(mut self, weight: u16) -> Self {
                self.font_weight = Some(weight);
                self
            }

            pub fn font_size(mut self, v: f64) -> Self {
                self.font_size = Some(v);
                self
            }

            pub fn wrap(mut self) -> Self {
                self.text_wrapping = TextWrapping::Wrap;
                self
            }

            pub fn selectable(mut self) -> Self {
                self.is_text_selection_enabled = true;
                self
            }
        }

        pub fn text_block(content: impl Into<String>) -> TextBlock {
            TextBlock::new(content)
        }

        // ── Type-ramp factories (Windows.UI.Xaml spec) ──────────────────────────────────────

        /// 28 px Semibold — page/section titles.
        pub fn title(content: impl Into<String>) -> TextBlock {
            TextBlock::new(content).font_size(28.0).semibold()
        }

        /// 20 px Semibold — secondary headings.
        pub fn subtitle(content: impl Into<String>) -> TextBlock {
            TextBlock::new(content).font_size(20.0).semibold()
        }

        /// 18 px Normal — larger body text.
        pub fn body_large(content: impl Into<String>) -> TextBlock {
            TextBlock::new(content).font_size(18.0)
        }

        /// 14 px Semibold — emphasized body text.
        pub fn body_strong(content: impl Into<String>) -> TextBlock {
            TextBlock::new(content).font_size(14.0).semibold()
        }

        /// 14 px Normal — standard body text.
        pub fn body(content: impl Into<String>) -> TextBlock {
            TextBlock::new(content).font_size(14.0)
        }

        /// 12 px Normal — captions and secondary labels.
        pub fn caption(content: impl Into<String>) -> TextBlock {
            TextBlock::new(content).font_size(12.0)
        }
    }
    pub use text_block::*;
    pub(crate) mod text_box {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct TextBox {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub value: String,
            pub on_text_changed: Option<Callback<String>>,
            pub placeholder_text: String,
            pub header: Option<String>,
            pub is_enabled: bool,
            pub accepts_return: bool,
            pub text_wrapping: TextWrapping,
        }
        impl TextBox {
            pub fn new(value: impl Into<String>) -> Self {
                Self {
                    value: value.into(),
                    is_enabled: true,
                    ..Default::default()
                }
            }
        }

        impl Widget for TextBox {
            widget_header!(ControlKind::TextBox);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::text_box_bindings(self);
                out.push(Binding::Prop(
                    Prop::Value,
                    PropValue::Str(self.value.clone()),
                ));
                out
            }
        }

        impl TextBox {
            pub fn on_text_changed(mut self, f: impl IntoCallback<String>) -> Self {
                self.on_text_changed = Some(f.into_callback());
                self
            }

            pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
                self.placeholder_text = s.into();
                self
            }

            pub fn header(mut self, s: impl Into<String>) -> Self {
                self.header = Some(s.into());
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }

            pub fn accepts_return(mut self, v: bool) -> Self {
                self.accepts_return = v;
                self
            }

            pub fn text_wrapping(mut self, v: TextWrapping) -> Self {
                self.text_wrapping = v;
                self
            }

            pub fn multiline(mut self) -> Self {
                self.accepts_return = true;
                self.text_wrapping = TextWrapping::Wrap;
                self
            }
        }

        pub fn text_box(value: impl Into<String>) -> TextBox {
            TextBox::new(value)
        }
    }
    pub use text_box::*;
    pub(crate) mod time_picker {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct TimePicker {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub header: Option<String>,
            pub clock_identifier: String,
            pub minute_increment: i32,
            pub is_enabled: bool,
            pub on_selected_time_changed: Option<Callback<windows_time::TimeSpan>>,
        }

        impl Default for TimePicker {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    header: None,
                    clock_identifier: "12HourClock".to_string(),
                    minute_increment: 1,
                    is_enabled: true,
                    on_selected_time_changed: None,
                }
            }
        }

        impl TimePicker {
            pub fn new() -> Self {
                Self {
                    is_enabled: true,
                    ..Default::default()
                }
            }

            pub fn header(mut self, s: impl Into<String>) -> Self {
                self.header = Some(s.into());
                self
            }

            /// Clock format: `"12HourClock"` or `"24HourClock"`.
            pub fn clock_identifier(mut self, s: impl Into<String>) -> Self {
                self.clock_identifier = s.into();
                self
            }

            pub fn minute_increment(mut self, v: i32) -> Self {
                self.minute_increment = v;
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }

            pub fn on_selected_time_changed(
                mut self,
                f: impl IntoCallback<windows_time::TimeSpan>,
            ) -> Self {
                self.on_selected_time_changed = Some(f.into_callback());
                self
            }
        }

        impl Widget for TimePicker {
            widget_header!(ControlKind::TimePicker);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::time_picker_bindings(self)
            }
        }

        pub fn time_picker() -> TimePicker {
            TimePicker::new()
        }
    }
    pub use time_picker::*;
    pub(crate) mod title_bar {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct TitleBar {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub title: String,
            pub subtitle: String,
            pub is_back_button_visible: bool,
            pub is_back_button_enabled: bool,
            pub is_pane_toggle_button_visible: bool,
            pub on_back_requested: Option<Callback<()>>,
            pub on_pane_toggle_requested: Option<Callback<()>>,
            pub is_tall: bool,
            /// Element placed in the TitleBar's center `Content` slot.
            pub content_element: Option<Box<Element>>,
            /// Element placed in the TitleBar's `RightHeader` slot (trailing area).
            pub footer_element: Option<Box<Element>>,
        }

        impl TitleBar {
            pub fn new(title: impl Into<String>) -> Self {
                Self {
                    title: title.into(),
                    ..Default::default()
                }
            }

            pub fn subtitle(mut self, s: impl Into<String>) -> Self {
                self.subtitle = s.into();
                self
            }

            pub fn back_button_visible(mut self, v: bool) -> Self {
                self.is_back_button_visible = v;
                self
            }

            pub fn back_button_enabled(mut self, v: bool) -> Self {
                self.is_back_button_enabled = v;
                self
            }

            pub fn pane_toggle_button_visible(mut self, v: bool) -> Self {
                self.is_pane_toggle_button_visible = v;
                self
            }

            pub fn on_back_requested<F: Fn() + 'static>(mut self, f: F) -> Self {
                self.on_back_requested = Some(Callback::new(move |()| f()));
                self
            }

            pub fn on_pane_toggle_requested<F: Fn() + 'static>(mut self, f: F) -> Self {
                self.on_pane_toggle_requested = Some(Callback::new(move |()| f()));
                self
            }

            pub fn content(mut self, el: impl Into<Element>) -> Self {
                self.content_element = Some(Box::new(el.into()));
                self
            }

            pub fn footer(mut self, el: impl Into<Element>) -> Self {
                self.footer_element = Some(Box::new(el.into()));
                self
            }

            pub fn tall(mut self, v: bool) -> Self {
                self.is_tall = v;
                self
            }
        }

        impl Widget for TitleBar {
            widget_header!(ControlKind::TitleBar);

            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::title_bar_bindings(self);
                out.push(Binding::Prop(Prop::Tall, PropValue::Bool(self.is_tall)));
                out
            }

            /// Maps to WinUI `TitleBar.Content`.
            fn header_element(&self) -> Option<&Element> {
                self.content_element.as_deref()
            }

            /// Maps to WinUI `TitleBar.RightHeader`.
            fn pane_element(&self) -> Option<&Element> {
                self.footer_element.as_deref()
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn bindings_include_upstream_title_bar_surface() {
                let title_bar = TitleBar::new("Primary")
                    .subtitle("Secondary")
                    .back_button_visible(true)
                    .back_button_enabled(false)
                    .pane_toggle_button_visible(true)
                    .on_back_requested(|| {})
                    .on_pane_toggle_requested(|| {})
                    .tall(true);

                let bindings = title_bar.bindings();

                assert_eq!(
                    find_prop(&bindings, Prop::Title),
                    Some(&PropValue::Str("Primary".into()))
                );
                assert_eq!(
                    find_prop(&bindings, Prop::Subtitle),
                    Some(&PropValue::Str("Secondary".into()))
                );
                assert_eq!(
                    find_prop(&bindings, Prop::IsBackButtonVisible),
                    Some(&PropValue::Bool(true))
                );
                assert_eq!(
                    find_prop(&bindings, Prop::IsBackButtonEnabled),
                    Some(&PropValue::Bool(false))
                );
                assert_eq!(
                    find_prop(&bindings, Prop::IsPaneToggleButtonVisible),
                    Some(&PropValue::Bool(true))
                );
                assert_eq!(
                    find_prop(&bindings, Prop::Tall),
                    Some(&PropValue::Bool(true))
                );
                assert!(matches!(
                    find_event(&bindings, Event::BackRequested),
                    Some(Some(_))
                ));
                assert!(matches!(
                    find_event(&bindings, Event::PaneToggleRequested),
                    Some(Some(_))
                ));
            }
        }
    }
    pub use title_bar::*;
    pub(crate) mod toggle_button {
        use super::*;

        /// `Windows.UI.Xaml.Controls.Primitives.ToggleButton`. A button that
        /// toggles between checked and unchecked states.
        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct ToggleButton {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub content: String,
            pub is_checked: bool,
            pub on_checked: Option<Callback<bool>>,
            pub is_enabled: bool,
        }

        impl ToggleButton {
            pub fn new(content: impl Into<String>, is_checked: bool) -> Self {
                Self {
                    content: content.into(),
                    is_checked,
                    is_enabled: true,
                    ..Default::default()
                }
            }

            pub fn on_checked(mut self, f: impl IntoCallback<bool>) -> Self {
                self.on_checked = Some(f.into_callback());
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }
        }

        impl Widget for ToggleButton {
            widget_header!(ControlKind::ToggleButton);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::toggle_button_bindings(self)
            }
        }

        /// Creates a [`ToggleButton`] with a label and initial checked state.
        pub fn toggle_button(content: impl Into<String>, is_checked: bool) -> ToggleButton {
            ToggleButton::new(content, is_checked)
        }
    }
    pub use toggle_button::*;
    pub(crate) mod toggle_switch {
        use super::*;

        #[derive(Clone, Default, Debug, PartialEq)]
        pub struct ToggleSwitch {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub is_on: bool,
            pub on_toggled: Option<Callback<bool>>,
            pub on_content: Option<String>,
            pub off_content: Option<String>,
            pub header: Option<String>,
            pub is_enabled: bool,
        }
        impl ToggleSwitch {
            pub fn new(is_on: bool) -> Self {
                Self {
                    is_on,
                    is_enabled: true,
                    ..Default::default()
                }
            }
            pub fn on_toggled(mut self, f: impl IntoCallback<bool>) -> Self {
                self.on_toggled = Some(f.into_callback());
                self
            }
            pub fn on_content(mut self, s: impl Into<String>) -> Self {
                self.on_content = Some(s.into());
                self
            }
            pub fn off_content(mut self, s: impl Into<String>) -> Self {
                self.off_content = Some(s.into());
                self
            }
            pub fn header(mut self, s: impl Into<String>) -> Self {
                self.header = Some(s.into());
                self
            }
            pub fn enabled(mut self, enabled: bool) -> Self {
                self.is_enabled = enabled;
                self
            }
        }

        impl Widget for ToggleSwitch {
            widget_header!(ControlKind::ToggleSwitch);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::toggle_switch_bindings(self)
            }
        }
    }
    pub use toggle_switch::*;
    pub(crate) mod tree_view {
        use super::*;

        /// Definition of a single node in a [`TreeView`].
        #[derive(Clone, Debug, PartialEq)]
        pub struct TreeNodeDef {
            /// Display text of this node.
            pub text: String,
            /// Whether this node is expanded by default.
            pub is_expanded: bool,
            /// Child nodes.
            pub children: Vec<TreeNodeDef>,
        }

        impl TreeNodeDef {
            pub fn new(text: impl Into<String>) -> Self {
                Self {
                    text: text.into(),
                    is_expanded: false,
                    children: Vec::new(),
                }
            }

            pub fn expanded(mut self) -> Self {
                self.is_expanded = true;
                self
            }

            pub fn child(mut self, node: TreeNodeDef) -> Self {
                self.children.push(node);
                self
            }

            pub fn children(mut self, nodes: Vec<TreeNodeDef>) -> Self {
                self.children = nodes;
                self
            }
        }

        /// Builder for a [`TreeNodeDef`].
        pub fn tree_node(text: impl Into<String>) -> TreeNodeDef {
            TreeNodeDef::new(text)
        }

        pub use crate::bindings::TreeViewSelectionMode;

        /// `Windows.UI.Xaml.Controls.TreeView`. A hierarchical list view.
        #[derive(Clone, Debug, PartialEq)]
        pub struct TreeView {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub nodes: Vec<TreeNodeDef>,
            pub selection_mode: TreeViewSelectionMode,
            pub on_item_invoked: Option<Callback<String>>,
        }
        impl Default for TreeView {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    nodes: Vec::new(),
                    selection_mode: TreeViewSelectionMode::Single,
                    on_item_invoked: None,
                }
            }
        }

        impl TreeView {
            pub fn new(nodes: Vec<TreeNodeDef>) -> Self {
                Self {
                    nodes,
                    ..Default::default()
                }
            }

            pub fn selection_mode(mut self, mode: TreeViewSelectionMode) -> Self {
                self.selection_mode = mode;
                self
            }

            pub fn on_item_invoked<F: Fn(String) + 'static>(mut self, f: F) -> Self {
                self.on_item_invoked = Some(Callback::new(f));
                self
            }
        }

        impl Widget for TreeView {
            widget_header!(ControlKind::TreeView);
            fn bindings(&self) -> PropBindings {
                let mut out = crate::generated_declarations::tree_view_bindings(self);
                out.push(Binding::Prop(
                    Prop::Nodes,
                    PropValue::TreeViewNodes(self.nodes.clone()),
                ));
                out
            }
        }

        pub fn tree_view(nodes: Vec<TreeNodeDef>) -> TreeView {
            TreeView::new(nodes)
        }
    }
    pub use tree_view::*;
    pub(crate) mod viewbox {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        pub struct Viewbox {
            pub key: Option<String>,
            pub modifiers: Modifiers,
            pub child: Box<Element>,
            pub stretch: Stretch,
        }

        impl Default for Viewbox {
            fn default() -> Self {
                Self {
                    key: None,
                    modifiers: Modifiers::default(),
                    child: Box::new(Element::Empty),
                    stretch: Stretch::Uniform,
                }
            }
        }

        impl Viewbox {
            pub fn new(child: impl Into<Element>) -> Self {
                Self {
                    child: Box::new(child.into()),
                    ..Default::default()
                }
            }

            pub fn stretch(mut self, v: Stretch) -> Self {
                self.stretch = v;
                self
            }
        }

        impl Widget for Viewbox {
            widget_header!(ControlKind::Viewbox);
            fn bindings(&self) -> PropBindings {
                crate::generated_declarations::viewbox_bindings(self)
            }
            fn children(&self) -> Children<'_> {
                Children::PositionalSingle(&self.child)
            }
        }

        pub fn viewbox(child: impl Into<Element>) -> Viewbox {
            Viewbox::new(child)
        }
    }
    pub use viewbox::*;
}
