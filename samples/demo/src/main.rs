#![windows_subsystem = "windows"]

use islands_reactor::*;

struct Demo;

impl Component for Demo {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        border(
            vstack((
                text_block("Hello from islands-reactor")
                    .font_size(20.0)
                    .bold(),
                text_block("A minimal XAML Islands demo."),
                button("Click me"),
            ))
            .spacing(8.0),
        )
        .padding(16.0)
        .into()
    }
}

fn main() -> Result<()> {
    App::new().title("Islands Reactor Demo").run(|| Demo)
}
