use std::cell::Cell;
use std::rc::Rc;

use super::*;

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
