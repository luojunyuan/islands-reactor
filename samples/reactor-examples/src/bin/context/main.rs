#![windows_subsystem = "windows"]

use std::sync::LazyLock;

use islands_reactor::*;
use islands_reactor_examples::run;

static COLOR_CONTEXT: LazyLock<Context<String>> =
    LazyLock::new(|| Context::new(String::from("light")));

fn main() -> Result<()> {
    run("Context", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (color, set_color) = cx.use_state(String::from("light"));
    let next = if color == "light" { "dark" } else { "light" };
    let set_color_for_button = set_color.clone();
    vstack((
        button(format!("Provide {next} context"))
            .on_click(move || set_color_for_button.call(next.to_owned())),
        component(ContextLabel, ()),
    ))
    .spacing(8.0)
    .padding(16.0)
    .provide(&COLOR_CONTEXT, color)
}

struct ContextLabel;

impl Component for ContextLabel {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let color = cx.use_context(&COLOR_CONTEXT);
        text_block(format!("Inherited color: {color}"))
            .font_size(20.0)
            .into()
    }
}
