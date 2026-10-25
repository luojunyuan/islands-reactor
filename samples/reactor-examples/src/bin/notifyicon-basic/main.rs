#![windows_subsystem = "windows"]

use islands_reactor::*;

fn main() -> Result<()> {
    let icon_path = format!(
        "{}/assets/notifyicon.ico",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/")
    );
    App::new()
        .title("Notification icon")
        .notify_icon(
            NotifyIcon::new(icon_path)
                .tooltip("Islands Reactor notification icon")
                .on_event(|event| println!("Notification icon event: {event:?}")),
        )
        .render(|_| {
            border(
                vstack((
                    text_block("The notification icon is active."),
                    text_block("Use the system tray to inspect its tooltip."),
                ))
                .spacing(8.0),
            )
            .padding(16.0)
            .into()
        })
}
