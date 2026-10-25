#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Color scheme", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let scheme = cx.use_color_scheme();
    let name = match scheme {
        ColorScheme::Light => "Light",
        ColorScheme::Dark => "Dark",
    };
    border(
        vstack((
            text_block(format!("Current color scheme: {name}"))
                .font_size(20.0)
                .bold(),
            text_block("This value follows the host window's effective WinUI theme."),
            border(text_block("Theme-bound card surface").foreground(ThemeRef::PrimaryText))
                .padding(12.0)
                .background(ThemeRef::CardBackground),
        ))
        .spacing(12.0),
    )
    .padding(16.0)
    .into()
}
