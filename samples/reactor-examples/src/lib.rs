use islands_reactor::{App, Element, RenderCx, Result};

pub fn run(title: &'static str, render: fn(&mut RenderCx) -> Element) -> Result<()> {
    App::new().title(title).render(move |cx| render(cx))
}
