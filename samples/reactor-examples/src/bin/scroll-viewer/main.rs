#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Scroll viewer", app)
}

fn app(_: &mut RenderCx) -> Element {
    border(scroll_viewer(
        vstack(
            (1..=24)
                .map(|row| text_block(format!("Scrollable row {row}")).into())
                .collect::<Vec<_>>(),
        )
        .spacing(8.0),
    ))
    .padding(12.0)
    .height(300.0)
    .into()
}
