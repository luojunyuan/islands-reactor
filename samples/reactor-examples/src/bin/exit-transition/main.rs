#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;
use std::time::Duration;

fn main() -> Result<()> {
    run("Exit transition", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (visible, set_visible) = cx.use_state(true);
    let set_visible_for_button = set_visible.clone();
    let mut children = vec![
        button(if visible {
            "Remove item"
        } else {
            "Restore item"
        })
        .on_click(move || set_visible_for_button.call(!visible))
        .into(),
    ];
    if visible {
        children.push(
            border(text_block("This element has enter and exit transitions."))
                .padding(12.0)
                .transition(
                    Some(AnimationConfig::fade_in(Duration::from_millis(180))),
                    Some(AnimationConfig::fade_out(Duration::from_millis(180))),
                )
                .into(),
        );
    }
    border(vstack(children).spacing(10.0)).padding(16.0).into()
}
