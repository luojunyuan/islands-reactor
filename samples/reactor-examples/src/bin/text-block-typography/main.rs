#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Text block typography", app)
}

fn app(_: &mut RenderCx) -> Element {
    border(
        vstack((
            text_block("Display").font_size(28.0).semibold(),
            text_block("The quick brown fox").font_size(20.0),
            text_block("Monospaced sample").font_family("Consolas"),
            text_block("Wrapped typography text that takes multiple lines.")
                .font_size(14.0)
                .wrap(),
        ))
        .spacing(12.0),
    )
    .padding(16.0)
    .width(360.0)
    .into()
}
