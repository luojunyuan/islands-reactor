use std::cell::Cell;
use std::rc::Rc;

use super::super::*;
use crate::test_support::{noop_rerender, reconciler};

#[test]
fn click_and_text_input_events_reach_current_callbacks() {
    let clicked = Rc::new(Cell::new(0));
    let clicked_callback = Rc::clone(&clicked);
    let button: Element = Button::new("Run")
        .on_click(move || clicked_callback.set(clicked_callback.get() + 1))
        .into();
    let mut reconciler = reconciler();
    let button_id = reconciler
        .reconcile(None, &button, None, noop_rerender())
        .unwrap();
    reconciler.backend.fire(button_id, Event::Click);
    assert_eq!(clicked.get(), 1);

    let typed = Rc::new(std::cell::RefCell::new(String::new()));
    let typed_callback = Rc::clone(&typed);
    let old: Element = TextBox::new("")
        .on_text_changed(move |value| *typed_callback.borrow_mut() = value)
        .into();
    let text_id = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler
        .backend
        .fire_string(text_id, Event::TextChanged, String::from("Island"));
    assert_eq!(&*typed.borrow(), "Island");
}

#[test]
fn replacing_an_event_handler_detaches_the_old_callback() {
    let old_calls = Rc::new(Cell::new(0));
    let old_callback = Rc::clone(&old_calls);
    let old: Element = Button::new("Run")
        .on_click(move || old_callback.set(old_callback.get() + 1))
        .into();
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();

    let new_calls = Rc::new(Cell::new(0));
    let new_callback = Rc::clone(&new_calls);
    let new: Element = Button::new("Run")
        .on_click(move || new_callback.set(new_callback.get() + 1))
        .into();
    reconciler.reconcile(Some(&old), &new, Some(root), noop_rerender());
    reconciler.backend.fire(root, Event::Click);

    assert_eq!(old_calls.get(), 0);
    assert_eq!(new_calls.get(), 1);
}

#[test]
fn event_handler_payload_kinds_are_checked() {
    let handler = EventHandler::Bool(Callback::new(|_| {}));
    crate::tests::assert_panics("invoke_string", || {
        handler.invoke_string(String::from("bad payload"));
    });
}

#[test]
fn removing_an_event_handler_detaches_the_native_subscription() {
    let old: Element = Button::new("Run").on_click(|| {}).into();
    let new: Element = Button::new("Run").into();
    let mut reconciler = reconciler();
    let button = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler.backend.ops.clear();

    reconciler.reconcile(Some(&old), &new, Some(button), noop_rerender());

    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::DetachEvent {
            id,
            event: Event::Click,
        } if *id == button
    )));
    assert!(
        !reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Create { .. } | Op::Destroy { .. }))
    );
}

#[test]
fn typed_date_time_numeric_and_boolean_events_preserve_their_payloads() {
    let selected_date = Rc::new(Cell::new(DateTime::default()));
    let date_callback = Rc::clone(&selected_date);
    let selected_time = Rc::new(Cell::new(TimeSpan::ZERO));
    let time_callback = Rc::clone(&selected_time);
    let slider_value = Rc::new(Cell::new(0.0));
    let slider_callback = Rc::clone(&slider_value);
    let checked = Rc::new(Cell::new(false));
    let checked_callback = Rc::clone(&checked);

    let mut children = StackPanel::vertical();
    children.children = vec![
        DatePicker::new()
            .on_selected_date_changed(move |value| date_callback.set(value))
            .into(),
        TimePicker::new()
            .on_selected_time_changed(move |value| time_callback.set(value))
            .into(),
        Slider::new(0.0)
            .on_value_changed(move |value| slider_callback.set(value))
            .into(),
        CheckBox::new(false)
            .on_checked(move |value| checked_callback.set(value))
            .into(),
    ];
    let tree: Element = children.into();
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &tree, None, noop_rerender())
        .unwrap();
    let controls = reconciler.backend.children_of(root).to_vec();
    let date = DateTime::from_unix_secs(1_750_000_000);
    let time = TimeSpan::from_minutes(90);

    reconciler
        .backend
        .fire_datetime(controls[0], Event::SelectedDateChanged, date);
    reconciler
        .backend
        .fire_timespan(controls[1], Event::SelectedTimeChanged, time);
    reconciler
        .backend
        .fire_f64(controls[2], Event::ValueChanged, 42.5);
    reconciler
        .backend
        .fire_bool(controls[3], Event::Checked, true);

    assert_eq!(selected_date.get(), date);
    assert_eq!(selected_time.get(), time);
    assert_eq!(slider_value.get(), 42.5);
    assert!(checked.get());
}
