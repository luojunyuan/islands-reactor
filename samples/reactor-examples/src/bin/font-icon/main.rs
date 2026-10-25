#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Font icons", app)
}

fn app(_: &mut RenderCx) -> Element {
    let symbols = [
        ("Home", SymbolGlyph::Home),
        ("Mail", SymbolGlyph::Mail),
        ("Find", SymbolGlyph::Find),
        ("Setting", SymbolGlyph::Setting),
        ("Favorite", SymbolGlyph::Favorite),
    ];
    vstack(
        symbols
            .into_iter()
            .map(|(label, symbol)| button(label).icon(symbol).into())
            .collect::<Vec<_>>(),
    )
    .spacing(6.0)
    .into()
}
