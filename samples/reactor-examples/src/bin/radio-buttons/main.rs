#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Radio buttons", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(String::from("Email"));
    let selected_email = selected == "Email";
    let selected_sms = selected == "SMS";
    let selected_none = selected == "None";
    let set_email = set_selected.clone();
    let set_sms = set_selected.clone();
    let set_none = set_selected;
    border(
        vstack((
            text_block("Notification preference").font_size(18.0).bold(),
            RadioButton::new("Email")
                .group("notifications")
                .checked(selected_email)
                .on_checked(move || set_email.call(String::from("Email"))),
            RadioButton::new("SMS")
                .group("notifications")
                .checked(selected_sms)
                .on_checked(move || set_sms.call(String::from("SMS"))),
            RadioButton::new("None")
                .group("notifications")
                .checked(selected_none)
                .on_checked(move || set_none.call(String::from("None"))),
            text_block(format!("Selected: {selected}")),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
