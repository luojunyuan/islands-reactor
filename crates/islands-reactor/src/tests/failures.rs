use super::super::*;
use crate::test_support::{Op, noop_rerender, reconciler};

#[test]
fn error_boundary_renders_fallback_after_component_panics() {
    let child = component(
        |_: &(), _: &mut RenderCx| -> Element { panic!("render failed") },
        (),
    );
    let tree = error_boundary(child, |message| {
        TextBlock::new(format!("recovered: {message}")).into()
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
        } if *id == root && value == "recovered: render failed"
    )));
}

#[test]
fn panic_without_an_error_boundary_is_propagated() {
    let child = component(
        |_: &(), _: &mut RenderCx| -> Element { panic!("unhandled") },
        (),
    );
    let mut reconciler = reconciler();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        reconciler.reconcile(None, &child, None, noop_rerender());
    }));

    assert!(result.is_err());
}
