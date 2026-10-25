#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;
use std::time::Duration;

fn main() -> Result<()> {
    run("Scale transition", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (large, set_large) = cx.use_state(false);
    let set_large_for_button = set_large.clone();
    let size = if large { 30.0 } else { 18.0 };
    border(
        vstack((
            text_block("Scale changes follow the implicit transition.")
                .font_size(size)
                .with_scale_transition(Duration::from_millis(250)),
            button(if large { "Smaller" } else { "Larger" })
                .on_click(move || set_large_for_button.call(!large)),
        ))
        .spacing(10.0),
    )
    .padding(16.0)
    .into()
}
