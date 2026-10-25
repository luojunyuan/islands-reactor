enum Handle {
    AutoSuggestBox(Xaml::AutoSuggestBox),
    Border(Xaml::Border),
    Button(Xaml::Button),
    CalendarDatePicker(Xaml::CalendarDatePicker),
    CalendarView(Xaml::CalendarView),
    Canvas(Xaml::Canvas),
    CheckBox(Xaml::CheckBox),
    ColorPicker(Xaml::ColorPicker),
    ComboBox(Xaml::ComboBox),
    CommandBar(Xaml::CommandBar),
    ContentDialog(Xaml::ContentDialog),
    DatePicker(Xaml::DatePicker),
    DropDownButton(Xaml::DropDownButton),
    Grid(Xaml::Grid),
    HyperlinkButton(Xaml::HyperlinkButton),
    Image(Xaml::Image),
    ListBox(Xaml::ListBox),
    MenuBar(Xaml::MenuBar),
    PasswordBox(Xaml::PasswordBox),
    PersonPicture(Xaml::PersonPicture),
    Pivot(Xaml::Pivot),
    PivotItem(Xaml::PivotItem),
    ProgressBar(Xaml::ProgressBar),
    ProgressRing(Xaml::ProgressRing),
    RadioButton(Xaml::RadioButton),
    RatingControl(Xaml::RatingControl),
    Rectangle(Xaml::Rectangle),
    RelativePanel(Xaml::RelativePanel),
    RepeatButton(Xaml::RepeatButton),
    RichEditBox(Xaml::RichEditBox),
    RichTextBlock(Xaml::RichTextBlock),
    ScrollView(Xaml::ScrollViewer),
    ScrollViewer(Xaml::ScrollViewer),
    Slider(Xaml::Slider),
    SplitButton(Xaml::SplitButton),
    SplitView(Xaml::SplitView),
    StackPanel(Xaml::StackPanel),
    TextBlock(Xaml::TextBlock),
    TextBox(Xaml::TextBox),
    TitleBar(IslandTitleBar),
    TimePicker(Xaml::TimePicker),
    ToggleButton(Xaml::ToggleButton),
    ToggleSwitch(Xaml::ToggleSwitch),
    TreeView(Xaml::TreeView),
    Viewbox(Xaml::Viewbox),
    Ellipse(Xaml::Ellipse),
    Line(Xaml::Line),
    ListView(Xaml::ListView),
    GridView(Xaml::GridView),
    FlipView(Xaml::FlipView),
    SelectorBar(IslandSelectorBar),
}

impl Handle {
    fn new(kind: ControlKind) -> Result<Self> {
        if let Some(handle) = create_handle(kind) {
            return handle;
        }
        Ok(match kind {
            ControlKind::TitleBar => Self::TitleBar(IslandTitleBar::new()?),
            ControlKind::SelectorBar => Self::SelectorBar(IslandSelectorBar::new()?),
            _ => unreachable!("tool-reactor has no constructor for {kind:?}"),
        })
    }

    fn cast_inner<T: Interface>(&self) -> Result<T> {
        match self {
            Self::AutoSuggestBox(v) => v.cast(),
            Self::Border(v) => v.cast(),
            Self::Button(v) => v.cast(),
            Self::CalendarDatePicker(v) => v.cast(),
            Self::CalendarView(v) => v.cast(),
            Self::Canvas(v) => v.cast(),
            Self::CheckBox(v) => v.cast(),
            Self::ColorPicker(v) => v.cast(),
            Self::ComboBox(v) => v.cast(),
            Self::CommandBar(v) => v.cast(),
            Self::ContentDialog(v) => v.cast(),
            Self::DatePicker(v) => v.cast(),
            Self::DropDownButton(v) => v.cast(),
            Self::Grid(v) => v.cast(),
            Self::HyperlinkButton(v) => v.cast(),
            Self::Image(v) => v.cast(),
            Self::ListBox(v) => v.cast(),
            Self::MenuBar(v) => v.cast(),
            Self::PasswordBox(v) => v.cast(),
            Self::PersonPicture(v) => v.cast(),
            Self::Pivot(v) => v.cast(),
            Self::PivotItem(v) => v.cast(),
            Self::ProgressBar(v) => v.cast(),
            Self::ProgressRing(v) => v.cast(),
            Self::RadioButton(v) => v.cast(),
            Self::RatingControl(v) => v.cast(),
            Self::Rectangle(v) => v.cast(),
            Self::RelativePanel(v) => v.cast(),
            Self::RepeatButton(v) => v.cast(),
            Self::RichEditBox(v) => v.cast(),
            Self::RichTextBlock(v) => v.cast(),
            Self::ScrollView(v) => v.cast(),
            Self::ScrollViewer(v) => v.cast(),
            Self::Slider(v) => v.cast(),
            Self::SplitButton(v) => v.cast(),
            Self::SplitView(v) => v.cast(),
            Self::StackPanel(v) => v.cast(),
            Self::TextBlock(v) => v.cast(),
            Self::TextBox(v) => v.cast(),
            Self::TitleBar(v) => v.as_ui_element()?.cast(),
            Self::TimePicker(v) => v.cast(),
            Self::ToggleButton(v) => v.cast(),
            Self::ToggleSwitch(v) => v.cast(),
            Self::TreeView(v) => v.cast(),
            Self::Viewbox(v) => v.cast(),
            Self::Ellipse(v) => v.cast(),
            Self::Line(v) => v.cast(),
            Self::ListView(v) => v.cast(),
            Self::GridView(v) => v.cast(),
            Self::FlipView(v) => v.cast(),
            Self::SelectorBar(v) => v.as_ui_element()?.cast(),
        }
    }

    fn as_ui_element(&self) -> Result<Xaml::UIElement> {
        self.cast_inner()
    }

    fn as_framework_element(&self) -> Result<Xaml::FrameworkElement> {
        self.cast_inner()
    }
}

type EventSubscription = windows_core::EventRevoker;

#[derive(Clone)]
pub(crate) struct IslandSelectorBar {
    root: Xaml::StackPanel,
    items: Rc<RefCell<Vec<SelectorBarItemDef>>>,
    buttons: Rc<RefCell<Vec<Xaml::ToggleButton>>>,
    selected_index: Rc<Cell<Option<usize>>>,
    handler: Rc<RefCell<Option<EventHandler>>>,
    item_revokers: Rc<RefCell<Vec<EventSubscription>>>,
}

impl IslandSelectorBar {
    fn new() -> Result<Self> {
        let root = Xaml::StackPanel::new()?;
        root.SetOrientation(Xaml::Orientation::Horizontal)?;
        if let Ok(spacing) = root.cast::<Xaml::IStackPanel4>() {
            let _ = spacing.SetSpacing(4.0);
        }
        Ok(Self {
            root,
            items: Rc::default(),
            buttons: Rc::default(),
            selected_index: Rc::default(),
            handler: Rc::default(),
            item_revokers: Rc::default(),
        })
    }

    fn as_ui_element(&self) -> Result<Xaml::UIElement> {
        self.root.cast()
    }

    fn set_items(&self, items: &[SelectorBarItemDef], handler: Option<EventHandler>) -> Result<()> {
        if let Some(handler) = handler {
            self.handler.replace(Some(handler));
        }
        self.items.replace(items.to_vec());
        self.selected_index.set((!items.is_empty()).then_some(0));
        self.item_revokers.borrow_mut().clear();
        self.buttons.borrow_mut().clear();

        let panel = self.root.cast::<Xaml::Panel>()?;
        let children = panel.Children()?;
        children.Clear()?;

        let mut buttons = Vec::with_capacity(items.len());
        for (index, item) in items.iter().enumerate() {
            let button = self.build_button(item, index == 0)?;
            let ui: Xaml::UIElement = button.cast()?;
            children.Append(&ui)?;
            buttons.push(button);
        }
        self.buttons.replace(buttons);
        self.wire_selection_changed()
    }

    fn set_selection_changed_handler(&self, handler: Option<EventHandler>) -> Result<()> {
        self.handler.replace(handler);
        self.wire_selection_changed()
    }

    fn build_button(
        &self,
        item: &SelectorBarItemDef,
        selected: bool,
    ) -> Result<Xaml::ToggleButton> {
        let button = Xaml::ToggleButton::new()?;
        button.SetIsChecked(Some(selected))?;
        let _ = button
            .cast::<Xaml::IControl>()?
            .SetPadding(xaml_thickness(Thickness::xy(12.0, 6.0)));

        let content: IInspectable = if let Some(icon) = item.icon {
            let panel = Xaml::StackPanel::new()?;
            panel.SetOrientation(Xaml::Orientation::Horizontal)?;
            if let Ok(spacing) = panel.cast::<Xaml::IStackPanel4>() {
                let _ = spacing.SetSpacing(6.0);
            }
            let icon = Xaml::SymbolIcon::CreateInstanceWithSymbol(Xaml::Symbol(icon.0))?;
            append_to_panel(&panel, &icon)?;
            let text = Xaml::TextBlock::new()?;
            text.SetText(&item.text)?;
            append_to_panel(&panel, &text)?;
            panel.cast()?
        } else {
            string_reference(&item.text)
        };
        button
            .cast::<Xaml::ContentControl>()?
            .SetContent(&content)?;
        Ok(button)
    }

    fn wire_selection_changed(&self) -> Result<()> {
        self.item_revokers.borrow_mut().clear();
        let Some(handler) = self.handler.borrow().clone() else {
            return Ok(());
        };

        let mut revokers = Vec::new();
        let buttons = self.buttons.borrow().clone();
        let items = self.items.borrow().clone();
        for (index, button) in buttons.iter().cloned().enumerate() {
            let buttons_for_cb = buttons.clone();
            let selected_index = self.selected_index.clone();
            let selected_text = items
                .get(index)
                .map(|item| item.text.clone())
                .unwrap_or_default();
            let handler = handler.clone();
            let revoker = button.Checked(move |_, _| {
                selected_index.set(Some(index));
                for (other_index, other) in buttons_for_cb.iter().enumerate() {
                    if other_index != index {
                        let _ = other.SetIsChecked(Some(false));
                    }
                }
                handler.invoke_string(selected_text.clone());
            })?;
            revokers.push(revoker);

            let selected_index = self.selected_index.clone();
            let button = button.clone();
            let revoker = button.clone().Unchecked(move |_, _| {
                if selected_index.get() == Some(index) {
                    let _ = button.SetIsChecked(Some(true));
                }
            })?;
            revokers.push(revoker);
        }
        self.item_revokers.replace(revokers);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct TitleBarMetrics {
    pub height_dips: f64,
    pub drag_left_dips: f64,
    pub drag_width_dips: f64,
}

#[derive(Clone, Debug)]
pub(crate) struct TitleBarState {
    pub metrics: TitleBarMetrics,
}

fn titlebar_state_from_handle(handle: &Handle) -> Option<TitleBarState> {
    if let Handle::TitleBar(titlebar) = handle {
        Some(titlebar.state())
    } else {
        None
    }
}

#[derive(Clone)]
pub(crate) struct IslandTitleBar {
    root: Xaml::Grid,
    title: Xaml::TextBlock,
    subtitle: Xaml::TextBlock,
    back_button: Xaml::Button,
    pane_button: Xaml::Button,
    content_host: Xaml::Border,
    footer_host: Xaml::Border,
    _caption_button_revokers: Rc<Vec<EventSubscription>>,
    height_dips: Rc<Cell<f64>>,
    back_button_visible: Rc<Cell<bool>>,
    pane_button_visible: Rc<Cell<bool>>,
}

impl IslandTitleBar {
    const STANDARD_HEIGHT: f64 = 40.0;
    const TALL_HEIGHT: f64 = 48.0;
    const COMMAND_WIDTH: f64 = 40.0;
    const TITLE_AREA_WIDTH: f64 = 180.0;
    const CAPTION_BUTTON_WIDTH: f64 = 46.0;
    const CAPTION_BUTTON_HEIGHT: f64 = 32.0;

    fn new() -> Result<Self> {
        let root = Xaml::Grid::new()?;
        root.cast::<Xaml::IGrid3>()?.SetColumnSpacing(0.0)?;
        let root_fe: Xaml::IFrameworkElement = root.cast()?;
        root_fe.SetHorizontalAlignment(Xaml::HorizontalAlignment::Stretch)?;
        root_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Top)?;

        append_grid_column(&root, grid_length_auto())?;
        append_grid_column(&root, grid_length_pixel(Self::TITLE_AREA_WIDTH))?;
        append_grid_column(&root, grid_length_star())?;
        append_grid_column(&root, grid_length_auto())?;
        append_grid_column(&root, grid_length_pixel(Self::CAPTION_BUTTON_WIDTH * 3.0))?;

        let commands = Xaml::StackPanel::new()?;
        commands
            .cast::<Xaml::IStackPanel>()?
            .SetOrientation(Xaml::Orientation::Horizontal)?;
        let commands_fe: Xaml::IFrameworkElement = commands.cast()?;
        commands_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        place_in_grid(&commands, 0)?;

        let back_button = titlebar_glyph_button("\u{E72B}", Self::COMMAND_WIDTH, 40.0)?;
        set_element_visibility(&back_button, false)?;
        append_to_panel(&commands, &back_button)?;
        let pane_button = titlebar_glyph_button("\u{E700}", Self::COMMAND_WIDTH, 40.0)?;
        set_element_visibility(&pane_button, false)?;
        append_to_panel(&commands, &pane_button)?;
        append_to_panel(&root, &commands)?;

        let heading = Xaml::StackPanel::new()?;
        heading
            .cast::<Xaml::IStackPanel>()?
            .SetOrientation(Xaml::Orientation::Vertical)?;
        let heading_fe: Xaml::IFrameworkElement = heading.cast()?;
        heading_fe.SetWidth(Self::TITLE_AREA_WIDTH)?;
        heading_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;

        let title = Xaml::TextBlock::new()?;
        title.SetFontSize(14.0)?;
        let title_fe: Xaml::IFrameworkElement = title.cast()?;
        title_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        append_to_panel(&heading, &title)?;

        let subtitle = Xaml::TextBlock::new()?;
        subtitle.SetFontSize(11.0)?;
        set_element_visibility(&subtitle, false)?;
        let subtitle_fe: Xaml::IFrameworkElement = subtitle.cast()?;
        subtitle_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        append_to_panel(&heading, &subtitle)?;
        place_in_grid(&heading, 1)?;
        append_to_panel(&root, &heading)?;

        let content_host = Xaml::Border::new()?;
        let content_fe: Xaml::IFrameworkElement = content_host.cast()?;
        content_fe.SetHorizontalAlignment(Xaml::HorizontalAlignment::Stretch)?;
        content_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        place_in_grid(&content_host, 2)?;
        append_to_panel(&root, &content_host)?;

        let footer_host = Xaml::Border::new()?;
        let footer_fe: Xaml::IFrameworkElement = footer_host.cast()?;
        footer_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        place_in_grid(&footer_host, 3)?;
        append_to_panel(&root, &footer_host)?;

        let caption_buttons = Xaml::Grid::new()?;
        for _ in 0..3 {
            append_grid_column(
                &caption_buttons,
                grid_length_pixel(Self::CAPTION_BUTTON_WIDTH),
            )?;
        }
        let caption_fe: Xaml::IFrameworkElement = caption_buttons.cast()?;
        caption_fe.SetHeight(Self::CAPTION_BUTTON_HEIGHT)?;
        caption_fe.SetHorizontalAlignment(Xaml::HorizontalAlignment::Right)?;
        caption_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        place_in_grid(&caption_buttons, 4)?;

        let minimize = titlebar_glyph_button(
            "\u{E921}",
            Self::CAPTION_BUTTON_WIDTH,
            Self::CAPTION_BUTTON_HEIGHT,
        )?;
        place_in_grid(&minimize, 0)?;
        append_to_panel(&caption_buttons, &minimize)?;
        let maximize = titlebar_glyph_button(
            "\u{E922}",
            Self::CAPTION_BUTTON_WIDTH,
            Self::CAPTION_BUTTON_HEIGHT,
        )?;
        place_in_grid(&maximize, 1)?;
        append_to_panel(&caption_buttons, &maximize)?;
        let close = titlebar_glyph_button(
            "\u{E8BB}",
            Self::CAPTION_BUTTON_WIDTH,
            Self::CAPTION_BUTTON_HEIGHT,
        )?;
        place_in_grid(&close, 2)?;
        append_to_panel(&caption_buttons, &close)?;
        append_to_panel(&root, &caption_buttons)?;

        let caption_button_revokers = vec![
            minimize
                .cast::<Xaml::ButtonBase>()?
                .Click(|_, _| hosting::minimize_active_window())?,
            maximize
                .cast::<Xaml::ButtonBase>()?
                .Click(|_, _| hosting::toggle_active_window_maximize())?,
            close
                .cast::<Xaml::ButtonBase>()?
                .Click(|_, _| hosting::close_active_window())?,
        ];

        let titlebar = Self {
            root,
            title,
            subtitle,
            back_button,
            pane_button,
            content_host,
            footer_host,
            _caption_button_revokers: Rc::new(caption_button_revokers),
            height_dips: Rc::new(Cell::new(Self::STANDARD_HEIGHT)),
            back_button_visible: Rc::new(Cell::new(false)),
            pane_button_visible: Rc::new(Cell::new(false)),
        };
        titlebar.apply_height(Self::STANDARD_HEIGHT)?;
        Ok(titlebar)
    }

    fn as_ui_element(&self) -> Result<Xaml::UIElement> {
        self.root.cast()
    }

    fn state(&self) -> TitleBarState {
        TitleBarState {
            metrics: self.metrics(),
        }
    }

    fn metrics(&self) -> TitleBarMetrics {
        let command_width = Self::COMMAND_WIDTH;
        let left = (if self.back_button_visible.get() {
            command_width
        } else {
            0.0
        }) + (if self.pane_button_visible.get() {
            command_width
        } else {
            0.0
        });
        TitleBarMetrics {
            height_dips: self.height_dips.get(),
            drag_left_dips: left,
            drag_width_dips: Self::TITLE_AREA_WIDTH,
        }
    }

    fn set_title(&self, value: &str) -> Result<()> {
        self.title.SetText(value)
    }

    fn set_subtitle(&self, value: &str) -> Result<()> {
        self.subtitle.SetText(value)?;
        set_element_visibility(&self.subtitle, !value.is_empty())
    }

    fn set_tall(&self, value: bool) -> Result<()> {
        let height = if value {
            Self::TALL_HEIGHT
        } else {
            Self::STANDARD_HEIGHT
        };
        self.height_dips.set(height);
        self.apply_height(height)
    }

    fn set_back_button_visible(&self, value: bool) -> Result<()> {
        self.back_button_visible.set(value);
        set_element_visibility(&self.back_button, value)
    }

    fn set_back_button_enabled(&self, value: bool) -> Result<()> {
        self.back_button
            .cast::<Xaml::IControl>()?
            .SetIsEnabled(value)
    }

    fn set_pane_toggle_button_visible(&self, value: bool) -> Result<()> {
        self.pane_button_visible.set(value);
        set_element_visibility(&self.pane_button, value)
    }

    fn set_content(&self, content: Option<&Xaml::UIElement>) -> Result<()> {
        match content {
            Some(content) => self.content_host.SetChild(content),
            None => self.content_host.SetChild(Option::<&Xaml::UIElement>::None),
        }
    }

    fn set_footer(&self, content: Option<&Xaml::UIElement>) -> Result<()> {
        match content {
            Some(content) => self.footer_host.SetChild(content),
            None => self.footer_host.SetChild(Option::<&Xaml::UIElement>::None),
        }
    }

    fn add_back_requested(&self, handler: EventHandler) -> Result<EventSubscription> {
        self.back_button
            .cast::<Xaml::ButtonBase>()?
            .Click(move |_, _| handler.invoke())
    }

    fn add_pane_toggle_requested(&self, handler: EventHandler) -> Result<EventSubscription> {
        self.pane_button
            .cast::<Xaml::ButtonBase>()?
            .Click(move |_, _| handler.invoke())
    }

    fn apply_height(&self, height: f64) -> Result<()> {
        self.root
            .cast::<Xaml::IFrameworkElement>()?
            .SetHeight(height)
    }
}

fn set_element_visibility<T: Interface>(element: &T, visible: bool) -> Result<()> {
    element
        .cast::<Xaml::IUIElement>()?
        .SetVisibility(if visible {
            Xaml::Visibility::Visible
        } else {
            Xaml::Visibility::Collapsed
        })
}

fn grid_length_auto() -> Xaml::GridLength {
    Xaml::GridLength {
        value: 0.0,
        grid_unit_type: Xaml::GridUnitType::Auto,
    }
}

fn grid_length_pixel(value: f64) -> Xaml::GridLength {
    Xaml::GridLength {
        value,
        grid_unit_type: Xaml::GridUnitType::Pixel,
    }
}

fn grid_length_star() -> Xaml::GridLength {
    Xaml::GridLength {
        value: 1.0,
        grid_unit_type: Xaml::GridUnitType::Star,
    }
}

fn append_grid_column(grid: &Xaml::Grid, width: Xaml::GridLength) -> Result<()> {
    let column = Xaml::ColumnDefinition::new()?;
    column.SetWidth(width)?;
    grid.ColumnDefinitions()?.Append(&column)
}

fn place_in_grid<T: Interface>(element: &T, column: i32) -> Result<()> {
    let fe: Xaml::FrameworkElement = element.cast()?;
    Xaml::Grid::SetColumn(&fe, column)
}

fn append_to_panel<P: Interface, C: Interface>(parent: &P, child: &C) -> Result<()> {
    let panel: Xaml::Panel = parent.cast()?;
    let child: Xaml::UIElement = child.cast()?;
    panel.Children()?.Append(&child)
}

fn titlebar_glyph_button(glyph: &str, width: f64, height: f64) -> Result<Xaml::Button> {
    let button = Xaml::Button::new()?;
    let fe: Xaml::IFrameworkElement = button.cast()?;
    fe.SetWidth(width)?;
    fe.SetHeight(height)?;
    fe.SetHorizontalAlignment(Xaml::HorizontalAlignment::Left)?;
    fe.SetVerticalAlignment(Xaml::VerticalAlignment::Top)?;

    let control: Xaml::IControl = button.cast()?;
    control.SetPadding(xaml_thickness(Thickness::default()))?;
    let _ = apply_button_style_variant(&button, 2);

    let text = Xaml::TextBlock::new()?;
    text.SetText(glyph)?;
    text.SetFontSize(10.0)?;
    if let Ok(font) = Xaml::FontFamily::CreateInstanceWithName("Segoe MDL2 Assets") {
        let _ = text.SetFontFamily(&font);
    }
    let text_fe: Xaml::IFrameworkElement = text.cast()?;
    text_fe.SetHorizontalAlignment(Xaml::HorizontalAlignment::Center)?;
    text_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;

    button.cast::<Xaml::ContentControl>()?.SetContent(&text)?;
    Ok(button)
}

/// Build and apply a XAML Style with {ThemeResource} setters to an element.
/// WinUI handles theme-reactive resolution natively.
fn apply_theme_resource_style(handle: &Handle, bindings: &[(Prop, crate::theme::ThemeRef)]) {
    let Some((target_type, fe)) = style_target_for_handle(handle) else {
        return;
    };

    let mut setters = String::new();
    for (prop, theme_ref) in bindings {
        let dp_name = match prop {
            Prop::Background => "Background",
            Prop::Foreground => "Foreground",
            Prop::BorderBrush => "BorderBrush",
            _ => continue,
        };
        let resource_key = theme_ref.resource_key();
        setters.push_str(&format!(
            "<Setter Property='{dp_name}' Value='{{ThemeResource {resource_key}}}'/>"
        ));
    }

    if setters.is_empty() {
        let _ = fe.SetStyle(None);
        return;
    }

    let xaml = format!(
        "<Style xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='{target_type}'>{setters}</Style>"
    );

    match Xaml::XamlReader::Load(&xaml) {
        Ok(obj) => {
            if let Ok(style) = obj.cast::<Xaml::Style>() {
                let _ = fe.SetStyle(None);
                let _ = fe.SetStyle(&style);
            }
        }
        Err(err) => {
            crate::diagnostics::emit(&format!(
                "islands_reactor: theme resource style failed: {err:?}\n"
            ));
        }
    }
}

fn style_target_for_handle(handle: &Handle) -> Option<(&'static str, Xaml::IFrameworkElement)> {
    match handle {
        Handle::Border(v) => v.cast().ok().map(|fe| ("Border", fe)),
        Handle::StackPanel(v) => v.cast().ok().map(|fe| ("StackPanel", fe)),
        Handle::Grid(v) => v.cast().ok().map(|fe| ("Grid", fe)),
        Handle::Canvas(v) => v.cast().ok().map(|fe| ("Canvas", fe)),
        Handle::Button(v) => v.cast().ok().map(|fe| ("Button", fe)),
        Handle::TextBlock(v) => v.cast().ok().map(|fe| ("TextBlock", fe)),
        _ => None,
    }
}

fn xaml_thickness(t: Thickness) -> Xaml::Thickness {
    Xaml::Thickness {
        left: t.left,
        top: t.top,
        right: t.right,
        bottom: t.bottom,
    }
}

fn xaml_color(c: Color) -> Xaml::Color {
    Xaml::Color {
        a: c.a,
        r: c.r,
        g: c.g,
        b: c.b,
    }
}

fn xaml_brush(brush: &Brush) -> Result<Xaml::Brush> {
    match brush {
        Brush::Solid(color) => {
            let b = Xaml::SolidColorBrush::new()?;
            b.SetColor(xaml_color(*color))?;
            b.cast()
        }
    }
}

fn apply_button_style_variant(button: &Xaml::Button, variant: i32) -> Result<()> {
    let fe: Xaml::IFrameworkElement = button.cast()?;
    let style_key = match variant {
        1 => Some("AccentButtonStyle"),
        2 => Some("SubtleButtonStyle"),
        3 => Some("TextBlockButtonStyle"),
        _ => None,
    };
    if let Some(key) = style_key {
        apply_resource_style(&fe, key)?;
    } else {
        fe.SetStyle(None)?;
    }
    Ok(())
}

fn apply_resource_style(fe: &Xaml::IFrameworkElement, resource_key: &str) -> Result<()> {
    let resources = Xaml::Application::Current().and_then(|app| app.Resources())?;
    let key = windows_reference::IReference::from(windows_core::HSTRING::from(resource_key));
    let map = resources
        .cast::<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>>(
        )?;
    let style = map.Lookup(&key)?.cast::<Xaml::Style>()?;
    fe.SetStyle(&style)
}

fn hstring(s: &str) -> windows_core::HSTRING {
    windows_core::HSTRING::from(s)
}

fn string_reference(s: &str) -> IInspectable {
    windows::Foundation::PropertyValue::CreateString(&windows_core::HSTRING::from(s))
        .expect("PropertyValue::CreateString should box string content")
}

fn inspectable_to_string(value: IInspectable) -> Option<String> {
    value
        .cast::<windows::Foundation::IPropertyValue>()
        .ok()
        .and_then(|v| v.GetString().ok())
        .map(|s| s.to_string_lossy())
}

fn set_content_string(handle: &Handle, s: &str) -> Result<()> {
    let tb = Xaml::TextBlock::new()?;
    tb.SetText(s)?;
    if let Ok(cc) = handle.cast_inner::<Xaml::ContentControl>() {
        cc.SetContent(&tb)?;
    } else if let Ok(ddb) = handle.cast_inner::<Xaml::DropDownButton>() {
        ddb.cast::<Xaml::ContentControl>()?
            .SetContent(&string_reference(s))?;
    } else if let Ok(split) = handle.cast_inner::<Xaml::SplitButton>() {
        split
            .cast::<Xaml::ContentControl>()?
            .SetContent(&string_reference(s))?;
    }
    Ok(())
}

fn panel_children_vec(parent: &Handle) -> Option<windows_collections::IVector<Xaml::UIElement>> {
    parent
        .cast_inner::<Xaml::Panel>()
        .ok()?
        .Children()
        .ok()?
        .cast()
        .ok()
}

fn content_control_for(parent: &Handle) -> Option<Xaml::ContentControl> {
    parent.cast_inner().ok()
}

fn items_vector(parent: &Handle) -> Option<windows_collections::IVector<IInspectable>> {
    parent
        .cast_inner::<Xaml::ItemsControl>()
        .ok()?
        .Items()
        .ok()?
        .cast()
        .ok()
}

fn set_string_items(handle: &Handle, items: &[String]) -> Result<()> {
    if let Some(vec) = items_vector(handle) {
        vec.Clear()?;
        for item in items {
            vec.Append(&string_reference(item))?;
        }
    }
    Ok(())
}

fn set_auto_suggest_items(asb: &Xaml::AutoSuggestBox, items: &[String]) -> Result<()> {
    let vec: Vec<Option<IInspectable>> = items
        .iter()
        .map(|item| Some(string_reference(item)))
        .collect();
    let vec: windows_collections::IVector<IInspectable> = vec.into();
    asb.cast::<Xaml::IItemsControl>()?.SetItemsSource(&vec)
}

fn set_grid_rows(grid: &Xaml::Grid, rows: &[GridLength]) -> Result<()> {
    let defs = grid.RowDefinitions()?;
    defs.Clear()?;
    for row in rows {
        let def = Xaml::RowDefinition::new()?;
        def.SetHeight(xaml_grid_length(*row))?;
        defs.Append(&def)?;
    }
    Ok(())
}

fn set_grid_columns(grid: &Xaml::Grid, cols: &[GridLength]) -> Result<()> {
    let defs = grid.ColumnDefinitions()?;
    defs.Clear()?;
    for col in cols {
        let def = Xaml::ColumnDefinition::new()?;
        def.SetWidth(xaml_grid_length(*col))?;
        defs.Append(&def)?;
    }
    Ok(())
}

fn xaml_grid_length(length: GridLength) -> Xaml::GridLength {
    match length {
        GridLength::Auto => Xaml::GridLength {
            value: 0.0,
            grid_unit_type: Xaml::GridUnitType::Auto,
        },
        GridLength::Pixel(v) => Xaml::GridLength {
            value: v,
            grid_unit_type: Xaml::GridUnitType::Pixel,
        },
        GridLength::Star(v) => Xaml::GridLength {
            value: v,
            grid_unit_type: Xaml::GridUnitType::Star,
        },
    }
}

fn set_image_uri(image: &Xaml::Image, uri: &str) -> Result<()> {
    if uri.is_empty() {
        return Ok(());
    }
    let uri = windows::Foundation::Uri::CreateUri(&hstring(uri))?.cast::<Xaml::Uri>()?;
    let bitmap = Xaml::BitmapImage::new()?;
    bitmap.cast::<Xaml::BitmapImage>()?.SetUriSource(&uri)?;
    image.SetSource(&bitmap.cast::<Xaml::ImageSource>()?)
}
