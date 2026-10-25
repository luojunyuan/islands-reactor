#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Button icons", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_u32);
    let set_add = set_count.clone();
    let set_save = set_count.clone();
    let set_delete = set_count;
    border(
        vstack((
            button("Add item")
                .icon(SymbolGlyph::Add)
                .on_click(move || set_add.call(count + 1)),
            button("Save")
                .icon(SymbolGlyph::Save)
                .accent()
                .on_click(move || set_save.call(count + 1)),
            button("Delete")
                .icon(SymbolGlyph::Delete)
                .on_click(move || set_delete.call(count.saturating_sub(1))),
            text_block(format!("Items: {count}")),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
