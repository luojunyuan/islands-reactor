#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Counter", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);
    let set_increment = set_count.clone();
    let set_decrement = set_count;
    border(
        vstack((
            text_block(format!("Count: {count}")).font_size(24.0).bold(),
            hstack((
                button("-1").on_click(move || set_decrement.call(count - 1)),
                button("+1")
                    .accent()
                    .on_click(move || set_increment.call(count + 1)),
            ))
            .spacing(8.0),
        ))
        .spacing(10.0),
    )
    .padding(16.0)
    .into()
}
