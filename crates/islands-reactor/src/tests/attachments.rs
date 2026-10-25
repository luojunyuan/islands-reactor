use super::super::*;
use crate::test_support::{Op, noop_rerender, reconciler};

#[test]
fn title_bar_content_reuses_and_clears_its_header_control() {
    let old: Element = TitleBar::new("Window")
        .content(TextBlock::new("Before"))
        .into();
    let mut reconciler = reconciler();
    let title_bar = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    let header = reconciler
        .backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::SetHeaderElement {
                id,
                header_id: Some(header),
            } if *id == title_bar => Some(*header),
            _ => None,
        })
        .unwrap();
    reconciler.backend.ops.clear();

    let updated: Element = TitleBar::new("Window")
        .content(TextBlock::new("After"))
        .into();
    reconciler.reconcile(Some(&old), &updated, Some(title_bar), noop_rerender());

    assert!(!reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetHeaderElement { id, header_id: Some(new_header) }
            if *id == title_bar && *new_header != header
    )));
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Text,
            value: PropValue::Str(value),
        } if *id == header && value == "After"
    )));

    let removed: Element = TitleBar::new("Window").into();
    reconciler.reconcile(Some(&updated), &removed, Some(title_bar), noop_rerender());
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetHeaderElement { id, header_id: None } if *id == title_bar
    )));
    assert!(
        reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Destroy { id } if *id == header))
    );
}

#[test]
fn tooltip_is_cleared_when_a_widget_drops_the_modifier() {
    let old: Element = Button::new("Save").tooltip("Save changes").into();
    let new: Element = Button::new("Save").into();
    let mut reconciler = reconciler();
    let button = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler.backend.ops.clear();

    reconciler.reconcile(Some(&old), &new, Some(button), noop_rerender());

    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetTooltip { id, tooltip: None } if *id == button
    )));
    assert!(
        !reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Destroy { .. }))
    );
}

#[test]
fn button_flyout_properties_update_and_clear_in_place() {
    let old: Element = Button::new("More").flyout("Old menu").into();
    let new: Element = Button::new("More").flyout("New menu").into();
    let removed: Element = Button::new("More").into();
    let mut reconciler = reconciler();
    let button = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();

    reconciler.reconcile(Some(&old), &new, Some(button), noop_rerender());
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::FlyoutContent,
            value: PropValue::Str(value),
        } if *id == button && value == "New menu"
    )));

    reconciler.reconcile(Some(&new), &removed, Some(button), noop_rerender());
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::FlyoutContent,
            value: PropValue::Unset,
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
