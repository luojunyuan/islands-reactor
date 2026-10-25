#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Text box border", app)
}

fn app(_: &mut RenderCx) -> Element {
    let field = |label, hint| {
        vstack((
            text_block(label).semibold(),
            border(text_box("").placeholder_text(hint))
                .border_brush(ThemeRef::Accent)
                .border_thickness(Thickness::uniform(1.0))
                .padding(4.0),
        ))
        .spacing(4.0)
    };
    border(
        vstack((
            field(
                "Focused style",
                "Border is supplied by the containing layout",
            ),
            field("Second field", "Enter text here"),
        ))
        .spacing(12.0),
    )
    .padding(16.0)
    .into()
}
