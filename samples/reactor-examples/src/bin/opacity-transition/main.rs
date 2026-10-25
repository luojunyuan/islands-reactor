#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;
use std::time::Duration;

fn main() -> Result<()> {
    run("Opacity transition", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (visible, set_visible) = cx.use_state(true);
    let set_visible_for_button = set_visible.clone();
    let sample = if visible {
        text_block("Opacity transitions are applied by the Island backend.")
            .with_opacity_transition(Duration::from_millis(300))
    } else {
        text_block("Opacity transitions are applied by the Island backend.")
            .opacity(0.0)
            .with_opacity_transition(Duration::from_millis(300))
    };
    border(
        vstack((
            sample,
            button(if visible { "Fade out" } else { "Fade in" })
                .on_click(move || set_visible_for_button.call(!visible)),
        ))
        .spacing(10.0),
    )
    .padding(16.0)
    .into()
}
