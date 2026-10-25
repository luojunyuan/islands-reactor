#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Card", app)
}

fn app(_: &mut RenderCx) -> Element {
    border(
        vstack((
            text_block("Project status").font_size(18.0).semibold(),
            text_block("The shared Border widget can act as a compact card surface."),
            button("Open project").accent(),
        ))
        .spacing(10.0),
    )
    .padding(18.0)
    .corner_radius(6.0)
    .border_brush(ThemeRef::Accent)
    .border_thickness(Thickness::uniform(1.0))
    .into()
}
