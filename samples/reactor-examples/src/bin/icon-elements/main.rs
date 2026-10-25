#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Icon elements", app)
}

fn app(_: &mut RenderCx) -> Element {
    border(
        vstack((
            text_block("Symbol glyphs in command bar items"),
            command_bar(vec![
                app_bar_button_icon("Add", SymbolGlyph::Add),
                app_bar_button_icon("Edit", SymbolGlyph::Edit),
                app_bar_button_icon("Delete", SymbolGlyph::Delete),
            ]),
        ))
        .spacing(8.0),
    )
    .padding(12.0)
    .into()
}
