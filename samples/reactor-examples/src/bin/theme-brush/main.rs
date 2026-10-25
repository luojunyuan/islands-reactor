#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Theme brushes", app)
}

fn app(_: &mut RenderCx) -> Element {
    let swatch = |label, background, foreground| {
        border(text_block(label).foreground(foreground))
            .background(background)
            .padding(12.0)
            .width(280.0)
    };
    vstack((
        swatch(
            "Accent / Accent text",
            ThemeRef::Accent,
            ThemeRef::AccentText,
        ),
        swatch(
            "Card / Primary text",
            ThemeRef::CardBackground,
            ThemeRef::PrimaryText,
        ),
        swatch(
            "System critical surface",
            ThemeRef::SystemCriticalBackground,
            ThemeRef::SystemCritical,
        ),
    ))
    .spacing(8.0)
    .padding(16.0)
    .into()
}
