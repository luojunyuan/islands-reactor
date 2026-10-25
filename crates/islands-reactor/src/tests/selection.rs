use std::cell::Cell;
use std::rc::Rc;

use super::super::*;
use crate::test_support::{Op, noop_rerender, reconciler};

#[test]
fn templated_selection_uses_current_callback_and_index() {
    let selected = Rc::new(Cell::new(-1));
    let selected_callback = Rc::clone(&selected);
    let list: Element = list_view(vec!["a", "b", "c"], |item, _| TextBlock::new(*item))
        .selected_index(1)
        .on_selection_changed(move |index| selected_callback.set(index))
        .into();
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &list, None, noop_rerender())
        .unwrap();

    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetTemplatedSelectedIndex { id, index: 1 } if *id == root
    )));
    reconciler.backend.fire_templated_selection_changed(root, 2);
    assert_eq!(selected.get(), 2);
}

#[test]
fn selection_mode_updates_without_recreating_list() {
    let old: Element = list_view(vec!["a", "b"], |item, _| TextBlock::new(*item))
        .selection_mode(SelectionMode::Single)
        .into();
    let new: Element = list_view(vec!["a", "b"], |item, _| TextBlock::new(*item))
        .selection_mode(SelectionMode::Extended)
        .into();
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler.backend.ops.clear();

    reconciler.reconcile(Some(&old), &new, Some(root), noop_rerender());

    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetTemplatedSelectionMode { id, mode: SelectionMode::Extended } if *id == root
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
fn selected_index_updates_without_recreating_templated_list() {
    let old: Element = list_view(vec!["a", "b", "c"], |item, _| TextBlock::new(*item))
        .selected_index(1)
        .into();
    let new: Element = list_view(vec!["a", "b", "c"], |item, _| TextBlock::new(*item))
        .selected_index(2)
        .into();
    let mut reconciler = reconciler();
    let list = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler.backend.ops.clear();

    reconciler.reconcile(Some(&old), &new, Some(list), noop_rerender());

    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetTemplatedSelectedIndex { id, index: 2 } if *id == list
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
fn templated_selection_callback_replacement_keeps_the_list_control() {
    let old_calls = Rc::new(Cell::new(0));
    let old_callback = Rc::clone(&old_calls);
    let old: Element = list_view(vec!["a", "b"], |item, _| TextBlock::new(*item))
        .on_selection_changed(move |_| old_callback.set(old_callback.get() + 1))
        .into();
    let mut reconciler = reconciler();
    let list = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();

    let new_calls = Rc::new(Cell::new(0));
    let new_callback = Rc::clone(&new_calls);
    let new: Element = list_view(vec!["a", "b"], |item, _| TextBlock::new(*item))
        .on_selection_changed(move |_| new_callback.set(new_callback.get() + 1))
        .into();
    reconciler.reconcile(Some(&old), &new, Some(list), noop_rerender());
    reconciler.backend.fire_templated_selection_changed(list, 1);

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

#[test]
fn list_box_selection_handler_replacement_uses_the_current_callback() {
    let old_index = Rc::new(Cell::new(-1));
    let old_callback = Rc::clone(&old_index);
    let old: Element = ListBox::new()
        .items(["first", "second"])
        .on_selection_changed(move |index| old_callback.set(index))
        .into();
    let mut reconciler = reconciler();
    let list = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();

    let new_index = Rc::new(Cell::new(-1));
    let new_callback = Rc::clone(&new_index);
    let new: Element = ListBox::new()
        .items(["first", "second"])
        .on_selection_changed(move |index| new_callback.set(index))
        .into();
    reconciler.reconcile(Some(&old), &new, Some(list), noop_rerender());
    reconciler
        .backend
        .fire_i32(list, Event::SelectionChanged, 1);

    assert_eq!(old_index.get(), -1);
    assert_eq!(new_index.get(), 1);
    assert!(
        !reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Create { .. } | Op::Destroy { .. }))
    );
}
