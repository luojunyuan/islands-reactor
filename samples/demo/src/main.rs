#![windows_subsystem = "windows"]

use islands_reactor::*;

fn main() -> Result<()> {
    App::new()
        .title("Islands Reactor Demo")
        .backdrop(Backdrop::Mica)
        .render(app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (custom_titlebar, set_custom_titlebar) = cx.use_state(true);
    let (theme, set_theme) = cx.use_state(0_i32);
    let (backdrop, set_backdrop_index) = cx.use_state(2_i32);

    let titlebar: Element = if custom_titlebar {
        TitleBar::new("Islands Reactor Demo")
            .subtitle("XAML Islands")
            .content(text_block("Windows.UI.Xaml Islands").opacity(0.72))
            .footer(
                text_block("Demo")
                    .opacity(0.72)
                    .padding(Thickness::xy(12.0, 0.0)),
            )
            .into()
    } else {
        Element::Empty
    };

    grid((
        titlebar.grid_row(0),
        border(settings_panel(
            custom_titlebar,
            set_custom_titlebar,
            theme,
            set_theme,
            backdrop,
            set_backdrop_index,
        ))
        .width(320.0)
        .padding(Thickness {
            left: 40.0,
            top: 32.0,
            right: 40.0,
            bottom: 32.0,
        })
        .background(ThemeRef::CardBackground)
        .border_brush(ThemeRef::CardStroke)
        .border_thickness(Thickness::uniform(1.0))
        .corner_radius(8.0)
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
        .grid_row(1),
    ))
    .rows([GridLength::Auto, GridLength::Star(1.0)])
    .columns([GridLength::Star(1.0)])
    .into()
}

fn settings_panel(
    custom_titlebar: bool,
    set_custom_titlebar: SetState<bool>,
    theme: i32,
    set_theme: SetState<i32>,
    backdrop: i32,
    set_backdrop_index: SetState<i32>,
) -> Element {
    vstack((
        setting_row(
            "Custom titlebar",
            ToggleSwitch::new(custom_titlebar)
                .on_toggled(move |value| set_custom_titlebar.call(value))
                .width(88.0)
                .into(),
        ),
        setting_row(
            "Theme",
            ComboBox::new(["System", "Light", "Dark"])
                .selected_index(theme)
                .on_selection_changed(move |index| {
                    let requested = match index {
                        1 => RequestedTheme::Light,
                        2 => RequestedTheme::Dark,
                        _ => RequestedTheme::Default,
                    };
                    set_requested_theme(requested);
                    set_theme.call(index);
                })
                .width(140.0)
                .into(),
        ),
        setting_row(
            "Background",
            ComboBox::new(["Solid", "Acrylic", "Mica", "Mica Alt"])
                .selected_index(backdrop)
                .on_selection_changed(move |index| {
                    let next = match index {
                        1 => Some(Backdrop::Acrylic),
                        2 => Some(Backdrop::Mica),
                        3 => Some(Backdrop::MicaAlt),
                        _ => None,
                    };
                    set_backdrop(next);
                    set_backdrop_index.call(index);
                })
                .width(140.0)
                .into(),
        ),
        border(vstack(()).height(1.0))
            .background(ThemeRef::DividerStroke)
            .margin(Thickness {
                left: -8.0,
                top: 4.0,
                right: -8.0,
                bottom: 4.0,
            }),
        text_block("Loaded string resource"),
        text_block("Hello from islands-reactor")
            .horizontal_alignment(HorizontalAlignment::Center)
            .foreground(ThemeRef::AccentText),
        text_block("Custom control"),
        custom_control_sample(),
    ))
    .spacing(8.0)
    .into()
}

fn setting_row(label: &'static str, control: Element) -> Element {
    grid((
        text_block(label)
            .vertical_alignment(VerticalAlignment::Center)
            .grid_column(0),
        control
            .horizontal_alignment(HorizontalAlignment::Right)
            .vertical_alignment(VerticalAlignment::Center)
            .grid_column(1),
    ))
    .columns([GridLength::Star(1.0), GridLength::Auto])
    .into()
}

fn custom_control_sample() -> Element {
    border(
        hstack((
            border(text_block("IR").font_size(12.0).bold())
                .padding(Thickness::xy(8.0, 5.0))
                .background(ThemeRef::Accent)
                .corner_radius(4.0),
            text_block("UserControl")
                .font_size(13.0)
                .vertical_alignment(VerticalAlignment::Center),
        ))
        .spacing(8.0),
    )
    .padding(Thickness::xy(10.0, 8.0))
    .background(ThemeRef::SubtleFill)
    .corner_radius(6.0)
    .horizontal_alignment(HorizontalAlignment::Center)
    .into()
}
