use std::cell::Cell;
use std::rc::Rc;

use super::super::*;
use crate::test_support::{Op, noop_rerender, reconciler};
use crate::widget::Widget;

#[test]
fn content_dialog_maps_native_result_to_public_result() {
    let result = Rc::new(Cell::new(ContentDialogResult::None));
    let result_callback = Rc::clone(&result);
    let bindings = ContentDialog::new("Confirm")
        .primary_button_text("Yes")
        .secondary_button_text("No")
        .is_open(true)
        .on_closed(move |value| result_callback.set(value))
        .bindings();

    let Some(Binding::Event(Event::Closed, Some(EventHandler::I32(handler)))) = bindings
        .iter()
        .find(|binding| matches!(binding, Binding::Event(Event::Closed, _)))
    else {
        panic!("expected a closed event binding");
    };
    handler.invoke(2);

    assert_eq!(result.get(), ContentDialogResult::Secondary);
    assert!(bindings.contains(&Binding::Prop(Prop::IsOpen, PropValue::Bool(true))));
}

#[test]
fn content_dialog_result_mapping_handles_unknown_native_values() {
    assert_eq!(ContentDialogResult::from_i32(0), ContentDialogResult::None);
    assert_eq!(
        ContentDialogResult::from_i32(1),
        ContentDialogResult::Primary
    );
    assert_eq!(
        ContentDialogResult::from_i32(2),
        ContentDialogResult::Secondary
    );
    assert_eq!(ContentDialogResult::from_i32(-1), ContentDialogResult::None);
    assert_eq!(ContentDialogResult::from_i32(3), ContentDialogResult::None);
}

#[test]
fn replacing_a_dialog_closed_handler_uses_the_latest_callback() {
    let old_calls = Rc::new(Cell::new(0));
    let old_callback = Rc::clone(&old_calls);
    let old: Element = ContentDialog::new("Confirm")
        .is_open(true)
        .on_closed(move |_| old_callback.set(old_callback.get() + 1))
        .into();
    let mut reconciler = reconciler();
    let dialog = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();

    let new_calls = Rc::new(Cell::new(0));
    let new_callback = Rc::clone(&new_calls);
    let new: Element = ContentDialog::new("Confirm")
        .is_open(true)
        .on_closed(move |_| new_callback.set(new_callback.get() + 1))
        .into();
    reconciler.reconcile(Some(&old), &new, Some(dialog), noop_rerender());
    reconciler.backend.fire_i32(dialog, Event::Closed, 1);

    assert_eq!(old_calls.get(), 0);
    assert_eq!(new_calls.get(), 1);
    assert!(
        !reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Create { .. } | Op::Destroy { .. }))
    );
}
