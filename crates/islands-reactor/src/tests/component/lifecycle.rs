use std::cell::RefCell;
use std::rc::Rc;

use super::*;

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
