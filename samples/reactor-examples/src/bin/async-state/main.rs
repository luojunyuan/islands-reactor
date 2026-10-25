#![windows_subsystem = "windows"]

use std::{thread, time::Duration};

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Async state", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (status, set_status) = cx.use_async_state(String::from("Idle"));
    let set_status_for_work = set_status.clone();
    border(
        vstack((
            text_block(status),
            button("Run background work").on_click(move || {
                let set_status = set_status_for_work.clone();
                thread::spawn(move || {
                    set_status.call(String::from("Working"));
                    thread::sleep(Duration::from_millis(700));
                    set_status.call(String::from("Completed on a worker thread"));
                });
            }),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
