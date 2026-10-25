#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Keyed list reorder", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (items, set_items) = cx.use_state(vec![
        String::from("Alpha"),
        String::from("Bravo"),
        String::from("Charlie"),
        String::from("Delta"),
    ]);
    let rows = items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let key = item.clone();
            let up_items = items.clone();
            let up_setter = set_items.clone();
            let down_items = items.clone();
            let down_setter = set_items.clone();
            hstack((
                text_block(format!("{}  {item}", index + 1)).width(150.0),
                button("Up").enabled(index > 0).on_click(move || {
                    let mut reordered = up_items.clone();
                    reordered.swap(index, index.saturating_sub(1));
                    up_setter.call(reordered);
                }),
                button("Down")
                    .enabled(index + 1 < items.len())
                    .on_click(move || {
                        let mut reordered = down_items.clone();
                        let last = reordered.len() - 1;
                        reordered.swap(index, (index + 1).min(last));
                        down_setter.call(reordered);
                    }),
            ))
            .spacing(6.0)
            .with_key(key)
            .into()
        })
        .collect::<Vec<_>>();
    border(vstack(rows).spacing(8.0)).padding(16.0).into()
}
