#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Controlled input", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (name, set_name) = cx.use_state(String::from("Ada"));
    let (volume, set_volume) = cx.use_state(35.0_f64);
    border(
        vstack((
            text_box(name.clone())
                .header("Name")
                .on_text_changed(set_name.clone()),
            text_block(format!("Hello, {name}")),
            Slider::new(volume)
                .range(0.0, 100.0)
                .header("Volume")
                .on_value_changed(set_volume.clone()),
            text_block(format!("Volume: {volume:.0}%")),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
