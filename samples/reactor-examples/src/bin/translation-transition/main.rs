#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;
use std::time::Duration;

fn main() -> Result<()> {
    run("Translation transition", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (moved, set_moved) = cx.use_state(false);
    let set_moved_for_button = set_moved.clone();
    let alignment = if moved {
        HorizontalAlignment::Right
    } else {
        HorizontalAlignment::Left
    };
    border(
        vstack((
            border(text_block("Move this item between the sides").padding(8.0))
                .horizontal_alignment(alignment)
                .with_translation_transition(Duration::from_millis(300)),
            button("Move").on_click(move || set_moved_for_button.call(!moved)),
        ))
        .spacing(10.0)
        .width(360.0),
    )
    .padding(16.0)
    .into()
}
