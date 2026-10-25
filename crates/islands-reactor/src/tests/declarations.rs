use super::super::*;
use crate::test_support::{Op, noop_rerender, reconciler};
use crate::widget::Widget;

#[test]
fn button_declaration_keeps_content_and_style_bindings() {
    let bindings = Button::new("Save").accent().bindings();

    assert!(bindings.contains(&Binding::Prop(
        Prop::Content,
        PropValue::Str(String::from("Save"))
    )));
    assert!(bindings.contains(&Binding::Prop(Prop::StyleVariant, PropValue::I32(1))));
}

#[test]
fn shared_modifiers_keep_layout_and_key_information() {
    let button = Button::new("Open")
        .with_key("open")
        .margin(Thickness::uniform(4.0))
        .width(120.0)
        .grid_row(2)
        .grid_column(1);

    assert_eq!(button.key.as_deref(), Some("open"));
    assert_eq!(button.modifiers.margin, Some(Thickness::uniform(4.0)));
    assert_eq!(button.modifiers.width, Some(120.0));
    assert_eq!(button.modifiers.grid.unwrap().row, 2);
    assert_eq!(button.modifiers.grid.unwrap().column, 1);
}

#[test]
fn list_declaration_preserves_keys_selection_and_item_views() {
    let element = list_view(vec!["one", "two"], |item, _| TextBlock::new(*item))
        .with_key_selector(|item| String::from(*item))
        .selected_index(1)
        .selection_mode(SelectionMode::Extended)
        .build();

    let Element::TemplatedList(list) = element else {
        panic!("expected a templated list element");
    };
    assert_eq!(list.item_count(), 2);
    assert_eq!(list.collect_keys(), ["one", "two"]);
    assert_eq!(list.selected_index(), 1);
    assert_eq!(list.selection_mode, SelectionMode::Extended);
    assert!(matches!(list.build_item_view(1), Element::TextBlock(_)));
}

#[test]
fn shared_visual_properties_update_and_clear_without_recreating_controls() {
    let old: Element = Button::new("Action")
        .width(100.0)
        .height(40.0)
        .background(Color::rgb(10, 20, 30))
        .into();
    let new: Element = Button::new("Action")
        .width(120.0)
        .background(Color::rgb(30, 20, 10))
        .into();
    let mut reconciler = reconciler();
    let button = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler.backend.ops.clear();

    reconciler.reconcile(Some(&old), &new, Some(button), noop_rerender());

    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Width,
            value: PropValue::F64(120.0),
        } if *id == button
    )));
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Height,
            value: PropValue::Unset,
        } if *id == button
    )));
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Background,
            value: PropValue::Brush(Brush::Solid(color)),
        } if *id == button && *color == Color::rgb(30, 20, 10)
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
fn button_style_binding_is_removed_when_returning_to_default() {
    let old = Button::new("Save").accent().bindings();
    let new = Button::new("Save").bindings();
    assert!(old.contains(&Binding::Prop(Prop::StyleVariant, PropValue::I32(1))));
    assert!(
        !new.iter()
            .any(|binding| matches!(binding, Binding::Prop(Prop::StyleVariant, _)))
    );
}
