#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("App bar icons", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (status, set_status) = cx.use_state(String::from("Ready"));
    let set_status_for_command = set_status.clone();
    border(
        vstack((
            command_bar(vec![
                app_bar_button_icon("New", SymbolGlyph::Add),
                app_bar_button_icon("Save", SymbolGlyph::Save),
                app_bar_button_icon("Delete", SymbolGlyph::Delete),
            ])
            .on_click(move |label| set_status_for_command.call(format!("Selected: {label}"))),
            text_block(status),
        ))
        .spacing(12.0),
    )
    .padding(16.0)
    .into()
}
