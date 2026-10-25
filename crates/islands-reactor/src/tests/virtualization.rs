use super::super::*;
use crate::test_support::{Op, noop_rerender, reconciler};
use std::cell::Cell;
use std::rc::Rc;

fn virtual_texts(items: Vec<&'static str>) -> Element {
    list_view(items, |item, _| TextBlock::new(*item))
        .with_key_selector(|item| String::from(*item))
        .into()
}

#[test]
fn only_realized_rows_create_controls_and_recycling_destroys_them() {
    let list = virtual_texts(vec!["first", "second"]);
    let mut reconciler = reconciler();
    let list_id = reconciler
        .reconcile(None, &list, None, noop_rerender())
        .unwrap();
    assert!(!reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::Create {
            kind: ControlKind::TextBlock,
            ..
        }
    )));

    reconciler.backend.simulate_prepare_row(list_id, 1);
    reconciler.drain_realizations();
    let row_id = reconciler
        .backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::MountRowContent {
                list_id: owner,
                row_idx: 1,
                content,
            } if *owner == list_id => Some(*content),
            _ => None,
        })
        .unwrap();

    reconciler.backend.simulate_clear_row(list_id, 1);
    reconciler.drain_realizations();
    assert!(
        reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Destroy { id } if *id == row_id))
    );
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::ClearRowContent { list_id: owner, row_idx: 1 } if *owner == list_id
    )));
}

#[test]
fn invalid_realization_indices_are_ignored() {
    let list = virtual_texts(vec!["one"]);
    let mut reconciler = reconciler();
    let list_id = reconciler
        .reconcile(None, &list, None, noop_rerender())
        .unwrap();

    reconciler.backend.simulate_prepare_row(list_id, 5);
    reconciler.drain_realizations();

    assert!(
        !reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::MountRowContent { .. }))
    );
}

#[test]
fn eager_realization_mounts_each_row_once() {
    let list = virtual_texts(vec!["one", "two", "three"]);
    let mut reconciler = reconciler();
    reconciler.eager_templated_realization = true;
    reconciler.reconcile(None, &list, None, noop_rerender());

    reconciler.drain_realizations();

    assert_eq!(
        reconciler
            .backend
            .ops
            .iter()
            .filter(|op| matches!(op, Op::MountRowContent { .. }))
            .count(),
        3
    );
}

#[test]
fn shrinking_a_virtual_list_clears_and_destroys_removed_realized_rows() {
    let old = virtual_texts(vec!["first", "second"]);
    let new = virtual_texts(vec!["first"]);
    let mut reconciler = reconciler();
    let list = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler.backend.simulate_prepare_row(list, 1);
    reconciler.drain_realizations();
    let row = reconciler.backend.row_contents_of(list)[&1];
    reconciler.backend.ops.clear();

    reconciler.reconcile(Some(&old), &new, Some(list), noop_rerender());

    assert!(reconciler.backend.ops.iter().any(
        |op| matches!(op, Op::SetTemplatedItemCount { list_id, count: 1 } if *list_id == list)
    ));
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::ClearRowContent { list_id, row_idx: 1 } if *list_id == list
    )));
    assert!(
        reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Destroy { id } if *id == row))
    );
    assert!(reconciler.backend.row_contents_of(list).is_empty());
}

#[test]
fn queued_realization_is_ignored_after_its_item_is_removed() {
    let old = virtual_texts(vec!["first", "second"]);
    let new = virtual_texts(vec!["first"]);
    let mut reconciler = reconciler();
    let list = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler.backend.simulate_prepare_row(list, 1);

    reconciler.reconcile(Some(&old), &new, Some(list), noop_rerender());
    reconciler.drain_realizations();

    assert!(!reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::MountRowContent { list_id, row_idx: 1, .. } if *list_id == list
    )));
}

#[test]
fn updating_a_realized_item_reuses_its_native_row() {
    let old = virtual_texts(vec!["before"]);
    let new = virtual_texts(vec!["after"]);
    let mut reconciler = reconciler();
    let list = reconciler
        .reconcile(None, &old, None, noop_rerender())
        .unwrap();
    reconciler.backend.simulate_prepare_row(list, 0);
    reconciler.drain_realizations();
    let row = reconciler.backend.row_contents_of(list)[&0];
    reconciler.backend.ops.clear();

    reconciler.reconcile(Some(&old), &new, Some(list), noop_rerender());

    assert_eq!(reconciler.backend.row_contents_of(list)[&0], row);
    assert!(reconciler.backend.ops.iter().any(|op| matches!(
        op,
        Op::SetProp {
            id,
            prop: Prop::Text,
            value: PropValue::Str(value),
        } if *id == row && value == "after"
    )));
    assert!(
        !reconciler
            .backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Create { .. } | Op::Destroy { .. }))
    );
}

struct VirtualRowRenderer {
    mounted: Rc<Cell<usize>>,
    unmounted: Rc<Cell<usize>>,
}

impl Component<u32> for VirtualRowRenderer {
    fn render(&self, value: &u32, cx: &mut RenderCx) -> Element {
        let mounted = Rc::clone(&self.mounted);
        let unmounted = Rc::clone(&self.unmounted);
        cx.use_effect_with_cleanup((), move || {
            mounted.set(mounted.get() + 1);
            Some(move || unmounted.set(unmounted.get() + 1))
        });
        TextBlock::new(value.to_string()).into()
    }
}

fn component_rows(
    items: Vec<u32>,
    mounted: Rc<Cell<usize>>,
    unmounted: Rc<Cell<usize>>,
) -> Element {
    list_view(items, move |item, _| {
        component(
            VirtualRowRenderer {
                mounted: Rc::clone(&mounted),
                unmounted: Rc::clone(&unmounted),
            },
            *item,
        )
    })
    .into()
}

#[test]
fn virtual_component_lifetime_tracks_row_realization() {
    let mounted = Rc::new(Cell::new(0));
    let unmounted = Rc::new(Cell::new(0));
    let list = component_rows(vec![10, 20], Rc::clone(&mounted), Rc::clone(&unmounted));
    let mut reconciler = reconciler();
    let list_id = reconciler
        .reconcile(None, &list, None, noop_rerender())
        .unwrap();

    assert_eq!(mounted.get(), 0);
    reconciler.backend.simulate_prepare_row(list_id, 1);
    reconciler.drain_realizations();
    assert_eq!(mounted.get(), 1);
    assert_eq!(unmounted.get(), 0);

    reconciler.backend.simulate_clear_row(list_id, 1);
    reconciler.drain_realizations();
    assert_eq!(unmounted.get(), 1);
}
