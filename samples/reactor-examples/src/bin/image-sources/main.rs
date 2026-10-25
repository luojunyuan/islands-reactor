#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Image source", app)
}

fn app(_: &mut RenderCx) -> Element {
    let image_path = format!(
        "file:///{}/assets/image-sources.png",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/")
    );
    border(
        vstack((
            text_block("Load a bundled PNG through a file URI."),
            Image::new_with_uri(image_path).width(320.0).height(180.0),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
