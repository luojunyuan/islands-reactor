#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Component input", app)
}

fn app(_: &mut RenderCx) -> Element {
    vstack((
        component(Greeting, GreetingInput { name: "Ada".into() }),
        component(
            Greeting,
            GreetingInput {
                name: "Grace".into(),
            },
        ),
    ))
    .spacing(8.0)
    .into()
}

#[derive(Clone, PartialEq)]
struct GreetingInput {
    name: String,
}

struct Greeting;

impl Component<GreetingInput> for Greeting {
    fn render(&self, input: &GreetingInput, _: &mut RenderCx) -> Element {
        border(text_block(format!("Hello, {}!", input.name)))
            .padding(10.0)
            .background(ThemeRef::CardBackground)
            .into()
    }
}
