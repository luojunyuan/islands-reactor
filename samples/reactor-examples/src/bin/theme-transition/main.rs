#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Theme transition", app)
}

fn app(_: &mut RenderCx) -> Element {
    border(
        vstack((
            text_block("Change the requested theme of this XAML Island."),
            button("System default").on_click(|| set_requested_theme(RequestedTheme::Default)),
            button("Light").on_click(|| set_requested_theme(RequestedTheme::Light)),
            button("Dark").on_click(|| set_requested_theme(RequestedTheme::Dark)),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
