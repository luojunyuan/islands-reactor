#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Text block alignment", app)
}

fn app(_: &mut RenderCx) -> Element {
    let sample = |label, alignment| {
        border(text_block(label).horizontal_alignment(alignment))
            .border_brush(ThemeRef::SecondaryText)
            .border_thickness(Thickness::uniform(1.0))
            .padding(8.0)
    };
    vstack((
        text_block("Control alignment within a stretched border"),
        sample("Left aligned control", HorizontalAlignment::Left),
        sample("Centered control", HorizontalAlignment::Center),
        sample("Right aligned control", HorizontalAlignment::Right),
    ))
    .spacing(8.0)
    .padding(16.0)
    .into()
}
