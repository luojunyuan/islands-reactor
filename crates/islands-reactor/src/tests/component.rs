use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use super::super::*;
use crate::test_support::{Op, noop_rerender, reconciler};

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

#[test]
fn context_provider_value_reaches_nested_component() {
    let context = Rc::new(Context::new(String::from("default")));
    let child_context = Rc::clone(&context);
    let child = component(
        move |_: &(), cx: &mut RenderCx| {
            TextBlock::new(cx.use_context(child_context.as_ref())).into()
        },
        (),
    );
    let tree = child.provide(context.as_ref(), String::from("provided"));
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &tree, None, noop_rerender())
        .unwrap();

    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Text,
            value: PropValue::Str(value),
        } if *id == root && value == "provided"
    )));
}

#[test]
fn nested_context_providers_shadow_outer_values() {
    let context = Rc::new(Context::new(String::from("default")));
    let child_context = Rc::clone(&context);
    let child = component(
        move |_: &(), cx: &mut RenderCx| {
            TextBlock::new(cx.use_context(child_context.as_ref())).into()
        },
        (),
    );
    let inner = child.provide(context.as_ref(), String::from("inner"));
    let tree = Element::Provider(ProviderElement {
        key: None,
        provisions: vec![ContextProvision::new(context.id, String::from("outer"))],
        child: Box::new(inner),
    });
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &tree, None, noop_rerender())
        .unwrap();

    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Text,
            value: PropValue::Str(value),
        } if *id == root && value == "inner"
    )));
}

fn context_reader(
    context: Rc<Context<u32>>,
    renders: Rc<Cell<usize>>,
) -> impl Fn(&(), &mut RenderCx) -> Element {
    move |(), cx| {
        renders.set(renders.get() + 1);
        TextBlock::new(cx.use_context(context.as_ref()).to_string()).into()
    }
}

fn render_counter(renders: Rc<Cell<usize>>) -> impl Fn(&(), &mut RenderCx) -> Element {
    move |(), _| {
        renders.set(renders.get() + 1);
        TextBlock::new("other").into()
    }
}

fn panel(children: Vec<Element>) -> Element {
    let mut panel = StackPanel::vertical();
    panel.children = children;
    panel.into()
}

#[test]
fn provider_updates_rerender_subscribers_without_rerendering_other_components() {
    let context = Rc::new(Context::new(0));
    let subscriber_renders = Rc::new(Cell::new(0));
    let other_renders = Rc::new(Cell::new(0));
    let old_subscriber = component(
        context_reader(Rc::clone(&context), Rc::clone(&subscriber_renders)),
        (),
    )
    .provide(context.as_ref(), 1);
    let old = panel(vec![
        old_subscriber,
        component(render_counter(Rc::clone(&other_renders)), ()),
    ]);
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();

    let new_subscriber = component(
        context_reader(Rc::clone(&context), Rc::clone(&subscriber_renders)),
        (),
    )
    .provide(context.as_ref(), 2);
    let new = panel(vec![
        new_subscriber,
        component(render_counter(Rc::clone(&other_renders)), ()),
    ]);
    reconciler.reconcile(Some(&old), &new, Some(root), noop_rerender());

    assert_eq!(subscriber_renders.get(), 2);
    assert_eq!(other_renders.get(), 1);
    let subscriber = reconciler.backend.children_of(root)[0];
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Text,
            value: PropValue::Str(value),
        } if *id == subscriber && value == "2"
    )));
}

fn effect_renderer(
    events: Rc<RefCell<Vec<(&'static str, u32)>>>,
) -> impl Fn(&u32, &mut RenderCx) -> Element {
    move |value, cx| {
        let setup_events = Rc::clone(&events);
        let cleanup_events = Rc::clone(&events);
        let value = *value;
        cx.use_effect_with_cleanup(value, move || {
            setup_events.borrow_mut().push(("setup", value));
            Some(move || cleanup_events.borrow_mut().push(("cleanup", value)))
        });
        TextBlock::new(value.to_string()).into()
    }
}

#[test]
fn component_effect_cleanup_runs_before_replacement_and_on_unmount() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let old = component(effect_renderer(Rc::clone(&events)), 1_u32);
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();

    let updated = component(effect_renderer(Rc::clone(&events)), 2_u32);
    reconciler.reconcile(Some(&old), &updated, Some(root), noop_rerender());
    assert_eq!(
        events.borrow().as_slice(),
        [("setup", 1), ("cleanup", 1), ("setup", 2)]
    );

    let replacement: Element = Button::new("Done").into();
    reconciler.reconcile(Some(&updated), &replacement, Some(root), noop_rerender());
    assert_eq!(
        events.borrow().as_slice(),
        [("setup", 1), ("cleanup", 1), ("setup", 2), ("cleanup", 2)]
    );
}

fn state_renderer(
    setter: Rc<RefCell<Option<SetState<u32>>>>,
) -> impl Fn(&(), &mut RenderCx) -> Element {
    move |(), cx| {
        let (value, update) = cx.use_state(0_u32);
        *setter.borrow_mut() = Some(update);
        TextBlock::new(value.to_string()).into()
    }
}

#[test]
fn state_updates_force_component_render_with_unchanged_props() {
    let setter = Rc::new(RefCell::new(None));
    let old = component(state_renderer(Rc::clone(&setter)), ());
    let new = component(state_renderer(Rc::clone(&setter)), ());
    let mut reconciler = reconciler();
    let root = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    setter.borrow().as_ref().unwrap().call(7);

    reconciler.reconcile(Some(&old), &new, Some(root), noop_rerender());

    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Text,
            value: PropValue::Str(value),
        } if *id == root && value == "7"
    )));
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
