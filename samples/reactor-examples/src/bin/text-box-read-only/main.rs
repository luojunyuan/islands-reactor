#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Read-only text", app)
}

fn app(_: &mut RenderCx) -> Element {
    let hash = "d41d8cd98f00b204e9800998ecf8427e";
    border(
        vstack((
            text_block("Read-only rich text control"),
            rich_edit_box(hash)
                .header("Select and copy, but do not edit")
                .read_only()
                .height(90.0),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
