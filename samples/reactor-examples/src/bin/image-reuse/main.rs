#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Image reuse", app)
}

fn app(_: &mut RenderCx) -> Element {
    let image_path = format!(
        "file:///{}/assets/image-sources.png",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/")
    );
    let image = || {
        Image::new_with_uri(image_path.clone())
            .width(160.0)
            .height(90.0)
    };
    border(
        vstack((
            text_block("The same image source is reused in several controls."),
            hstack((image(), image(), image())).spacing(8.0),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
