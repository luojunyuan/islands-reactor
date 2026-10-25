use super::super::*;
use crate::test_support::{Op, noop_rerender, reconciler};

fn keyed_text(key: &str, value: &str) -> Element {
    TextBlock::new(value).with_key(key).into()
}

fn stack(children: Vec<Element>) -> Element {
    let mut panel = StackPanel::vertical();
    panel.children = children;
    panel.into()
}

#[test]
fn property_changes_reuse_the_existing_control() {
    let old = TextBlock::new("old").into();
    let new = TextBlock::new("new").into();
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler.backend.ops.clear();

    let updated = reconciler
        .reconcile(Some(&old), &new, Some(root), noop_rerender())
        .unwrap();

    assert_eq!(updated, root);
    assert!(
        !reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Create { .. } | Op::Destroy { .. }))
    );
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Text,
            value: PropValue::Str(value),
        } if *id == root && value == "new"
    )));
}

#[test]
fn keyed_reorder_preserves_controls_and_declared_order() {
    let old = stack(vec![keyed_text("first", "A"), keyed_text("second", "B")]);
    let new = stack(vec![keyed_text("second", "B"), keyed_text("first", "A")]);
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    let before = reconciler.backend.children_of(root).to_vec();

    reconciler.reconcile(Some(&old), &new, Some(root), noop_rerender());

    assert_eq!(reconciler.backend.children_of(root), [before[1], before[0]]);
    assert!(
        !reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Create { .. } | Op::Destroy { .. }))
    );
}

#[test]
fn removed_child_is_destroyed_once() {
    let old = stack(vec![keyed_text("keep", "A"), keyed_text("remove", "B")]);
    let new = stack(vec![keyed_text("keep", "A")]);
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    let removed = reconciler.backend.children_of(root)[1];
    reconciler.backend.ops.clear();

    reconciler.reconcile(Some(&old), &new, Some(root), noop_rerender());

    assert_eq!(reconciler.backend.children_of(root).len(), 1);
    assert_eq!(
        reconciler
            .backend
            .ops
            .iter()
            .filter(|op| matches!(op, Op::Destroy { id } if *id == removed))
            .count(),
        1
    );
}

#[test]
fn keyed_insert_and_remove_preserve_retained_sibling_identity() {
    let old = stack(vec![keyed_text("remove", "A"), keyed_text("keep", "B")]);
    let new = stack(vec![keyed_text("keep", "B"), keyed_text("insert", "C")]);
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    let before = reconciler.backend.children_of(root).to_vec();
    reconciler.backend.ops.clear();

    reconciler.reconcile(Some(&old), &new, Some(root), noop_rerender());

    let after = reconciler.backend.children_of(root);
    assert_eq!(after.len(), 2);
    assert_eq!(after[0], before[1]);
    assert_ne!(after[1], before[0]);
    assert!(
        reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Destroy { id } if *id == before[0]))
    );
    assert!(
        reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Create { .. }))
    );
}

#[test]
fn positional_children_reconcile_by_index() {
    let old = stack(vec![TextBlock::new("A").into(), Button::new("Open").into()]);
    let new = stack(vec![
        TextBlock::new("Updated").into(),
        Button::new("Close").into(),
    ]);
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    let before = reconciler.backend.children_of(root).to_vec();

    reconciler.reconcile(Some(&old), &new, Some(root), noop_rerender());

    assert_eq!(reconciler.backend.children_of(root), before);
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Text,
            value: PropValue::Str(value),
        } if *id == before[0] && value == "Updated"
    )));
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Content,
            value: PropValue::Str(value),
        } if *id == before[1] && value == "Close"
    )));
}

#[test]
fn unchanged_tree_emits_no_native_operations() {
    let old: Element = Button::new("Stable").into();
    let new = old.clone();
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler.backend.ops.clear();

    let updated = reconciler
        .reconcile(Some(&old), &new, Some(root), noop_rerender())
        .unwrap();

    assert_eq!(updated, root);
    assert!(reconciler.backend.ops.is_empty());
}

#[test]
fn changing_the_root_widget_replaces_its_native_control() {
    let old: Element = TextBlock::new("Text").into();
    let new: Element = Button::new("Action").into();
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();

    let replacement = reconciler
        .reconcile(Some(&old), &new, Some(root), noop_rerender())
        .unwrap();

    assert_ne!(replacement, root);
    assert!(
        reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Destroy { id } if *id == root))
    );
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::Create {
            id,
            kind: ControlKind::Button,
        } if *id == replacement
    )));
}
