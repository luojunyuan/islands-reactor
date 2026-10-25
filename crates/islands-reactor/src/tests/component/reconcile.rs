use std::cell::Cell;
use std::rc::Rc;

use super::*;

fn render_label(value: &String, _: &mut RenderCx) -> Element {
    TextBlock::new(value.clone()).into()
}

#[test]
fn component_props_update_the_existing_native_control() {
    let mut reconciler = reconciler();
    let old = component(render_label, String::from("before"));
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    let create_count = reconciler
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::Create { .. }))
        .count();

    let new = component(render_label, String::from("after"));
    let updated = reconciler
        .reconcile(Some(&old), &new, Some(root), noop_rerender())
        .unwrap();

    assert_eq!(updated, root);
    assert_eq!(
        reconciler
            .backend
            .ops
            .iter()
            .filter(|op| matches!(op, Op::Create { .. }))
            .count(),
        create_count
    );
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Text,
            value: PropValue::Str(value),
        } if *id == root && value == "after"
    )));
}

fn counted_renderer(count: Rc<Cell<usize>>) -> impl Fn(&u32, &mut RenderCx) -> Element {
    move |value, _| {
        count.set(count.get() + 1);
        TextBlock::new(value.to_string()).into()
    }
}

#[test]
fn memoized_component_skips_equal_props() {
    let old_count = Rc::new(Cell::new(0));
    let new_count = Rc::new(Cell::new(0));
    let old = memo(counted_renderer(Rc::clone(&old_count)), 7);
    let new = memo(counted_renderer(Rc::clone(&new_count)), 7);
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler.backend.ops.clear();

    reconciler.reconcile(Some(&old), &new, Some(root), noop_rerender());

    assert_eq!(old_count.get(), 1);
    assert_eq!(new_count.get(), 0);
    assert!(reconciler.backend.ops.is_empty());
}

fn child_renderer(renders: Rc<Cell<usize>>) -> impl Fn(&(), &mut RenderCx) -> Element {
    move |(), _| {
        renders.set(renders.get() + 1);
        TextBlock::new("stable child").into()
    }
}

fn parent_renderer(child_renders: Rc<Cell<usize>>) -> impl Fn(&u32, &mut RenderCx) -> Element {
    move |value, _| {
        let mut panel = StackPanel::vertical();
        panel.children = vec![
            TextBlock::new(value.to_string()).into(),
            component(child_renderer(Rc::clone(&child_renders)), ()),
        ];
        panel.into()
    }
}

#[test]
fn parent_component_update_reuses_an_unchanged_nested_component() {
    let child_renders = Rc::new(Cell::new(0));
    let old = component(parent_renderer(Rc::clone(&child_renders)), 1_u32);
    let new = component(parent_renderer(Rc::clone(&child_renders)), 2_u32);
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    let child_id = reconciler.backend.children_of(root)[1];
    reconciler.backend.ops.clear();

    reconciler.reconcile(Some(&old), &new, Some(root), noop_rerender());

    assert_eq!(child_renders.get(), 1);
    assert_eq!(reconciler.backend.children_of(root)[1], child_id);
    assert!(!reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp { id, prop: Prop::Text, .. } if *id == child_id
    )));
}
