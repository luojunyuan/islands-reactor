#![windows_subsystem = "windows"]

use islands_reactor::*;

fn main() -> Result<()> {
    App::new()
        .title("Window theme")
        .backdrop(Backdrop::Mica)
        .render(app)
}

fn app(cx: &mut RenderCx) -> Element {
    let scheme = cx.use_color_scheme();
    let description = match scheme {
        ColorScheme::Light => "The window is using a light theme.",
        ColorScheme::Dark => "The window is using a dark theme.",
    };
    border(
        vstack((
            text_block(description).font_size(20.0).bold(),
            button("System default").on_click(|| set_requested_theme(RequestedTheme::Default)),
            button("Light theme").on_click(|| set_requested_theme(RequestedTheme::Light)),
            button("Dark theme").on_click(|| set_requested_theme(RequestedTheme::Dark)),
            button("Acrylic backdrop").on_click(|| set_backdrop(Some(Backdrop::Acrylic))),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
