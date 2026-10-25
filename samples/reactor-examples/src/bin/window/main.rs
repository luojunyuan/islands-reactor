#![windows_subsystem = "windows"]

use islands_reactor::*;

fn main() -> Result<()> {
    App::new()
        .title("Window size observer")
        .inner_size(520.0, 360.0)
        .render(app)
}

fn app(cx: &mut RenderCx) -> Element {
    let size = cx.use_inner_size();
    border(
        vstack((
            text_block("Resize the window"),
            text_block(format!(
                "Client size: {:.0} x {:.0} DIP",
                size.width, size.height
            ))
            .font_size(20.0)
            .bold(),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
