#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Use effect", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_u32);
    cx.use_effect((count,), move || println!("Counter changed to {count}"));
    border(
        vstack((
            text_block(format!("Count: {count}")),
            button("Increment").on_click(move || set_count.call(count + 1)),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
