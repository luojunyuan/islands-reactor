#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Responsive navigation", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let size = cx.use_inner_size();
    let compact = size.width < 640.0;
    let (selected, set_selected) = cx.use_state(String::from("Overview"));
    let (pane_open, set_pane_open) = cx.use_state(!compact);
    let set_overview = set_selected.clone();
    let set_activity = set_selected.clone();
    let set_settings = set_selected.clone();
    let set_pane_for_overview = set_pane_open.clone();
    let set_pane_for_activity = set_pane_open.clone();
    let set_pane_for_settings = set_pane_open.clone();
    let pane = vstack((
        button("Overview").on_click(move || {
            set_overview.call(String::from("Overview"));
            set_pane_for_overview.call(false);
        }),
        button("Activity").on_click(move || {
            set_activity.call(String::from("Activity"));
            set_pane_for_activity.call(false);
        }),
        button("Settings").on_click(move || {
            set_settings.call(String::from("Settings"));
            set_pane_for_settings.call(false);
        }),
    ))
    .spacing(6.0);
    let toggle_pane = set_pane_open;
    let content = vstack((
        text_block(format!("{selected} page"))
            .font_size(24.0)
            .bold(),
        text_block(format!("Window width: {:.0} DIP", size.width)),
        button(if pane_open { "Close menu" } else { "Open menu" })
            .on_click(move || toggle_pane.call(!pane_open)),
    ))
    .spacing(10.0)
    .padding(16.0);
    split_view(content)
        .pane(pane)
        .display_mode(if compact {
            SplitViewDisplayMode::Overlay
        } else {
            SplitViewDisplayMode::Inline
        })
        .is_pane_open(pane_open)
        .open_pane_length(220.0)
        .into()
}
