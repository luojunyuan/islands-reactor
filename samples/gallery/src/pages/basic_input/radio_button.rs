use crate::controls::*;
use islands_reactor::*;

pub fn radio_button_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(String::from("Option A"));

    page_content(
        "RadioButton",
        "Select one option from a group.",
        vec![
            sample_card(
                "Radio button group",
                vstack((
                    RadioButton::new("Option A")
                        .group("sample-options")
                        .checked(selected == "Option A")
                        .on_checked({
                            let set_selected = set_selected.clone();
                            move || set_selected.call(String::from("Option A"))
                        }),
                    RadioButton::new("Option B")
                        .group("sample-options")
                        .checked(selected == "Option B")
                        .on_checked({
                            let set_selected = set_selected.clone();
                            move || set_selected.call(String::from("Option B"))
                        }),
                    RadioButton::new("Option C")
                        .group("sample-options")
                        .checked(selected == "Option C")
                        .on_checked(move || set_selected.call(String::from("Option C"))),
                    text_block(format!("Selected: {selected}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"RadioButton::new("Option A").group("options").checked(is_selected)"#,
            ),
            sample_card(
                "Disabled option",
                RadioButton::new("Unavailable option")
                    .group("disabled-options")
                    .enabled(false),
                r#"RadioButton::new("Unavailable option").enabled(false)"#,
            ),
        ],
    )
}
