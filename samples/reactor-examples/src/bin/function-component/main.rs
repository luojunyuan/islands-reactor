#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Function component", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(1_u32);
    let set_count_for_button = set_count;
    vstack((
        heading("Reusable function components"),
        metric("Items", count),
        button("Add item").on_click(move || set_count_for_button.call(count + 1)),
    ))
    .spacing(10.0)
    .into()
}

fn heading(value: &str) -> Element {
    text_block(value).font_size(20.0).bold().into()
}

fn metric(label: &str, value: u32) -> Element {
    hstack((
        text_block(format!("{label}:")),
        text_block(value.to_string()),
    ))
    .spacing(6.0)
    .into()
}
