#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Form", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (name, set_name) = cx.use_state(String::new());
    let (email, set_email) = cx.use_state(String::new());
    let (subscribed, set_subscribed) = cx.use_state(false);
    let (submitted, set_submitted) = cx.use_state(false);
    let set_submitted_on_click = set_submitted;
    border(
        vstack((
            text_block("Create an account").font_size(20.0).bold(),
            text_box(name.clone())
                .header("Name")
                .placeholder_text("Your name")
                .on_text_changed(set_name),
            text_box(email.clone())
                .header("Email")
                .placeholder_text("name@example.com")
                .on_text_changed(set_email),
            check_box(subscribed)
                .content("Send me product updates")
                .on_checked(set_subscribed),
            button("Submit")
                .accent()
                .on_click(move || set_submitted_on_click.call(true)),
            text_block(if submitted {
                format!("Submitted for {name} ({email})")
            } else {
                String::from("Complete the fields and submit.")
            }),
        ))
        .spacing(8.0),
    )
    .padding(16.0)
    .into()
}
