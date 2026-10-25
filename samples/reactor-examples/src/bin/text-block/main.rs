#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Text block", app)
}

fn app(_: &mut RenderCx) -> Element {
    vstack((
        text_block("Plain text"),
        text_block("Large text").font_size(20.0),
        text_block("Bold and large text").font_size(28.0).bold(),
        text_block("Selectable text for copying").selectable(),
        text_block("Wrapped text demonstrates how longer content flows across lines.").wrap(),
    ))
    .spacing(8.0)
    .padding(16.0)
    .into()
}
