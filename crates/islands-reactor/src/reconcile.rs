use std::cell::{Cell, RefCell};
use std::rc::Rc;

use rustc_hash::FxHashMap;

pub(crate) use super::*;

mod child {
    use rustc_hash::{FxHashMap, FxHashSet};
    use std::borrow::Cow;

    use super::*;

    fn update_child_tracked<B: Backend + 'static>(
        reconciler: &mut Reconciler<B>,
        parent: ControlId,
        index: usize,
        old: &Element,
        new: &Element,
    ) {
        let Some(old_id) = reconciler.child_at(parent, index) else {
            return;
        };
        match reconciler.update(old, new, old_id) {
            Some(nid) if nid != old_id => {
                reconciler.replace_child_tracked(parent, index, nid);
            }
            Some(_) => {}
            None => {
                reconciler.remove_child_tracked(parent, index);
            }
        }
    }

    pub(crate) fn reconcile<B: Backend + 'static>(
        reconciler: &mut Reconciler<B>,
        parent: ControlId,
        old_live: LiveChildrenRef<'_>,
        new_live: LiveChildrenRef<'_>,
    ) {
        let has_keys = old_live.any_has_key() || new_live.any_has_key();

        if has_keys {
            reconcile_keyed_live(reconciler, parent, &old_live, &new_live);
        } else {
            reconcile_positional_live(reconciler, parent, &old_live, &new_live);
        }
    }

    pub(crate) fn reconcile_positional<B: Backend + 'static>(
        reconciler: &mut Reconciler<B>,
        parent: ControlId,
        old_live: LiveChildrenRef<'_>,
        new_live: LiveChildrenRef<'_>,
    ) {
        reconcile_positional_live(reconciler, parent, &old_live, &new_live);
    }

    fn reconcile_positional_live<B: Backend + 'static>(
        reconciler: &mut Reconciler<B>,
        parent: ControlId,
        old_live: &LiveChildrenRef<'_>,
        new_live: &LiveChildrenRef<'_>,
    ) {
        let old_len = old_live.len();
        let new_len = new_live.len();
        let common = old_len.min(new_len);

        for i in 0..common {
            if reconciler.child_at(parent, i).is_none() {
                debug_assert!(
                    false,
                    "audit §7.1.10: positional reconcile out of mirror bounds at \
                 parent={parent:?} i={i} common={common} \
                 (mirror_len={mirror_len}, old_live.len={old_len}, \
                  new_live.len={new_len}). Likely a child-tracking miss \
                 in a recent edit to the reconciler.",
                    mirror_len = reconciler
                        .children_mirror
                        .get(&parent)
                        .map_or(0, |v| v.len()),
                    old_len = old_len,
                    new_len = new_len,
                );
                break;
            }

            let old_el = old_live.get(i).unwrap();
            let new_el = new_live.get(i).unwrap();

            if !reconciler.force_component_rerender && can_skip_update(old_el, new_el) {
                // Even when elements are structurally identical, a child component
                // whose state was mutated via SetState must still be re-rendered.
                let child_id = reconciler.child_at(parent, i);
                let state_dirty =
                    child_id.is_some_and(|cid| reconciler.is_component_state_dirty(cid));
                if !state_dirty {
                    reconciler.debug_elements_skipped += 1;
                    continue;
                }
            }

            if old_el.kind_matches(new_el) {
                update_child_tracked(reconciler, parent, i, old_el, new_el);
            } else {
                if let Some(old_id) = reconciler.child_at(parent, i) {
                    reconciler.unmount(old_id);
                }
                match reconciler.mount(new_el) {
                    Some(new_id) => reconciler.replace_child_tracked(parent, i, new_id),
                    None => reconciler.remove_child_tracked(parent, i),
                }
            }
        }

        if old_len > new_len {
            for idx in (common..old_len).rev() {
                if let Some(id) = reconciler.child_at(parent, idx) {
                    reconciler.unmount(id);
                }
                reconciler.remove_child_tracked(parent, idx);
            }
        }

        if new_len > old_len {
            for i in common..new_len {
                let el = new_live.get(i).unwrap();
                if let Some(child_id) = reconciler.mount(el) {
                    reconciler.append_child_tracked(parent, child_id);
                }
            }
        }
    }

    fn effective_key(el: &Element, positional_index: usize) -> Cow<'_, str> {
        if let Some(k) = el.key() {
            return Cow::Borrowed(k);
        }
        Cow::Owned(format!("__pos_{positional_index}_{}", el.kind_name()))
    }

    fn key_match(a: &Element, b: &Element) -> bool {
        if !a.can_update(b) {
            return false;
        }

        a.key() == b.key()
    }

    fn reconcile_keyed_live<B: Backend + 'static>(
        reconciler: &mut Reconciler<B>,
        parent: ControlId,
        old: &LiveChildrenRef<'_>,
        new: &LiveChildrenRef<'_>,
    ) {
        let old_len = old.len();
        let new_len = new.len();

        let mut prefix = 0;
        while prefix < old_len
            && prefix < new_len
            && key_match(old.get(prefix).unwrap(), new.get(prefix).unwrap())
        {
            let old_el = old.get(prefix).unwrap();
            let new_el = new.get(prefix).unwrap();
            if reconciler.child_at(parent, prefix).is_some() {
                if !reconciler.force_component_rerender && can_skip_update(old_el, new_el) {
                    let child_id = reconciler.child_at(parent, prefix);
                    let state_dirty =
                        child_id.is_some_and(|cid| reconciler.is_component_state_dirty(cid));
                    if state_dirty {
                        update_child_tracked(reconciler, parent, prefix, old_el, new_el);
                    } else {
                        reconciler.debug_elements_skipped += 1;
                    }
                } else {
                    update_child_tracked(reconciler, parent, prefix, old_el, new_el);
                }
            }
            prefix += 1;
        }

        debug_assert!(prefix <= old_len.min(new_len));
        let old_remaining = old_len.saturating_sub(prefix);
        let new_remaining = new_len.saturating_sub(prefix);

        let mut suffix = 0;
        while suffix < old_remaining
            && suffix < new_remaining
            && key_match(
                old.get(old_len - 1 - suffix).unwrap(),
                new.get(new_len - 1 - suffix).unwrap(),
            )
        {
            let old_idx = old_len - 1 - suffix;
            let panel_idx = reconciler
                .children_mirror
                .get(&parent)
                .map_or(0, |v| v.len().saturating_sub(1).saturating_sub(suffix));
            let old_el = old.get(old_idx).unwrap();
            let new_el = new.get(new_len - 1 - suffix).unwrap();
            if reconciler.child_at(parent, panel_idx).is_some() {
                if !reconciler.force_component_rerender && can_skip_update(old_el, new_el) {
                    let child_id = reconciler.child_at(parent, panel_idx);
                    let state_dirty =
                        child_id.is_some_and(|cid| reconciler.is_component_state_dirty(cid));
                    if state_dirty {
                        update_child_tracked(reconciler, parent, panel_idx, old_el, new_el);
                    } else {
                        reconciler.debug_elements_skipped += 1;
                    }
                } else {
                    update_child_tracked(reconciler, parent, panel_idx, old_el, new_el);
                }
            }
            suffix += 1;
        }

        let old_start = prefix;
        let old_end = old_len - suffix;
        let new_start = prefix;
        let new_end = new_len - suffix;
        let old_mid_len = old_end - old_start;
        let new_mid_len = new_end - new_start;

        if old_mid_len == 0 && new_mid_len == 0 {
            return;
        }

        if old_mid_len == 0 {
            for i in 0..new_mid_len {
                let new_el = new.get(new_start + i).unwrap();
                if let Some(ctrl) = reconciler.mount(new_el) {
                    reconciler.insert_child_tracked(parent, prefix + i, ctrl);
                }
            }
            return;
        }

        if new_mid_len == 0 {
            for i in (0..old_mid_len).rev() {
                let panel_idx = prefix + i;
                if let Some(id) = reconciler.child_at(parent, panel_idx) {
                    reconciler.unmount(id);
                }
                reconciler.remove_child_tracked(parent, panel_idx);
            }
            return;
        }

        reconcile_keyed_middle(
            reconciler,
            parent,
            old,
            new,
            old_start,
            old_mid_len,
            new_start,
            new_mid_len,
            prefix,
        );
    }

    #[allow(clippy::too_many_arguments, clippy::needless_range_loop)]
    fn reconcile_keyed_middle<B: Backend + 'static>(
        reconciler: &mut Reconciler<B>,
        parent: ControlId,
        old: &LiveChildrenRef<'_>,
        new: &LiveChildrenRef<'_>,
        old_start: usize,
        old_mid_len: usize,
        new_start: usize,
        new_mid_len: usize,
        prefix: usize,
    ) {
        let mut old_key_map: FxHashMap<Cow<'_, str>, usize> = FxHashMap::default();
        old_key_map.reserve(old_mid_len);
        for i in 0..old_mid_len {
            let el = old.get(old_start + i).unwrap();
            let key = effective_key(el, old_start + i);
            old_key_map.insert(key, i);
        }

        let mut new_to_old: Vec<i32> = vec![-1; new_mid_len];
        let mut matched: Vec<bool> = vec![false; old_mid_len];
        for i in 0..new_mid_len {
            let el = new.get(new_start + i).unwrap();
            let key = effective_key(el, new_start + i);
            if let Some(&old_rel) = old_key_map.get(key.as_ref()) {
                let old_el = old.get(old_start + old_rel).unwrap();

                if old_el.can_update(el) {
                    new_to_old[i] = old_rel as i32;
                    matched[old_rel] = true;
                }
            }
        }

        let lis: FxHashSet<usize> = compute_lis(&new_to_old);

        for i in (0..old_mid_len).rev() {
            if !matched[i] {
                let panel_idx = prefix + i;
                if let Some(id) = reconciler.child_at(parent, panel_idx) {
                    reconciler.unmount(id);
                }
                reconciler.remove_child_tracked(parent, panel_idx);
            }
        }

        let mut key_to_panel: FxHashMap<Cow<'_, str>, usize> = FxHashMap::default();
        {
            let mut panel_idx = prefix;
            for i in 0..old_mid_len {
                if matched[i] {
                    let key = effective_key(old.get(old_start + i).unwrap(), old_start + i);
                    key_to_panel.insert(key, panel_idx);
                    panel_idx += 1;
                }
            }
        }

        for i in 0..new_mid_len {
            let target_panel_idx = prefix + i;
            let new_el = new.get(new_start + i).unwrap();

            if new_to_old[i] == -1 {
                if let Some(ctrl) = reconciler.mount(new_el) {
                    reconciler.insert_child_tracked(parent, target_panel_idx, ctrl);

                    for v in key_to_panel.values_mut() {
                        if *v >= target_panel_idx {
                            *v += 1;
                        }
                    }
                }
            } else if lis.contains(&i) {
                let old_rel = new_to_old[i] as usize;
                let old_el = old.get(old_start + old_rel).unwrap();
                // LIS items don't need to move, but earlier non-LIS moves in
                // this same loop may have shifted them to a different actual
                // panel slot. Use `key_to_panel` for the LIS element's current
                // position so we update the right control. (Using
                // `target_panel_idx` would update whichever control happens to
                // have been displaced into that slot — see the keyed-shuffle
                // regression test in this module.)
                let key = effective_key(old_el, old_start + old_rel);
                let actual_pos = *key_to_panel.get(key.as_ref()).unwrap();
                if reconciler.child_at(parent, actual_pos).is_some() {
                    if can_skip_update(old_el, new_el) {
                        let child_id = reconciler.child_at(parent, actual_pos);
                        let state_dirty =
                            child_id.is_some_and(|cid| reconciler.is_component_state_dirty(cid));
                        if state_dirty {
                            update_child_tracked(reconciler, parent, actual_pos, old_el, new_el);
                        } else {
                            reconciler.debug_elements_skipped += 1;
                        }
                    } else {
                        update_child_tracked(reconciler, parent, actual_pos, old_el, new_el);
                    }
                }
            } else {
                let old_rel = new_to_old[i] as usize;
                let old_el = old.get(old_start + old_rel).unwrap();
                let key = effective_key(old_el, old_start + old_rel);
                let current_pos = *key_to_panel.get(key.as_ref()).unwrap();
                if current_pos != target_panel_idx {
                    reconciler.move_child_tracked(parent, current_pos, target_panel_idx);

                    for v in key_to_panel.values_mut() {
                        if *v == current_pos {
                            *v = target_panel_idx;
                        } else if current_pos < target_panel_idx
                            && *v > current_pos
                            && *v <= target_panel_idx
                        {
                            *v -= 1;
                        } else if current_pos > target_panel_idx
                            && *v >= target_panel_idx
                            && *v < current_pos
                        {
                            *v += 1;
                        }
                    }
                }

                if reconciler.child_at(parent, target_panel_idx).is_some() {
                    if can_skip_update(old_el, new_el) {
                        let child_id = reconciler.child_at(parent, target_panel_idx);
                        let state_dirty =
                            child_id.is_some_and(|cid| reconciler.is_component_state_dirty(cid));
                        if state_dirty {
                            update_child_tracked(
                                reconciler,
                                parent,
                                target_panel_idx,
                                old_el,
                                new_el,
                            );
                        } else {
                            reconciler.debug_elements_skipped += 1;
                        }
                    } else {
                        update_child_tracked(reconciler, parent, target_panel_idx, old_el, new_el);
                    }
                }
            }
        }
    }

    pub(crate) fn compute_lis(arr: &[i32]) -> FxHashSet<usize> {
        let n = arr.len();
        if n == 0 {
            return FxHashSet::default();
        }

        let mut tails: Vec<i32> = Vec::new();
        let mut tail_idx: Vec<usize> = Vec::new();
        let mut pred: Vec<i32> = vec![-1; n];

        for (i, &val) in arr.iter().enumerate() {
            if val == -1 {
                continue;
            }

            let mut lo = 0_usize;
            let mut hi = tails.len();
            while lo < hi {
                let mid = (lo + hi) / 2;
                if tails[mid] < val {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }

            if lo == tails.len() {
                tails.push(val);
                tail_idx.push(i);
            } else {
                tails[lo] = val;
                tail_idx[lo] = i;
            }

            if lo > 0 {
                pred[i] = tail_idx[lo - 1] as i32;
            }
        }

        let mut result: FxHashSet<usize> = FxHashSet::default();
        if tail_idx.is_empty() {
            return result;
        }
        let mut idx = *tail_idx.last().unwrap() as i32;
        while idx != -1 {
            result.insert(idx as usize);
            idx = pred[idx as usize];
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn lis_empty() {
            let s = compute_lis(&[]);
            assert!(s.is_empty());
        }

        #[test]
        fn lis_single() {
            let s = compute_lis(&[5]);
            assert_eq!(s.len(), 1);
            assert!(s.contains(&0));
        }

        #[test]
        fn lis_sorted() {
            let s = compute_lis(&[1, 2, 3, 4, 5]);
            assert_eq!(s.len(), 5);
        }

        #[test]
        fn lis_reversed() {
            let s = compute_lis(&[5, 4, 3, 2, 1]);
            assert_eq!(s.len(), 1);
        }

        #[test]
        fn lis_with_unmapped() {
            let s = compute_lis(&[-1, 2, -1, 4, -1]);
            assert_eq!(s.len(), 2);
        }

        #[test]
        fn lis_all_unmapped() {
            let s = compute_lis(&[-1, -1, -1]);
            assert!(s.is_empty());
        }

        #[test]
        fn lis_mixed() {
            let s = compute_lis(&[3, 1, 2, 4]);
            assert_eq!(s.len(), 3);
        }

        #[test]
        fn lis_returned_indices_are_strictly_increasing_values() {
            let arr = [5, 1, 4, 2, 3];
            let s = compute_lis(&arr);
            let mut ordered: Vec<usize> = s.into_iter().collect();
            ordered.sort();
            for w in ordered.windows(2) {
                assert!(arr[w[1]] > arr[w[0]]);
            }
        }

        #[test]
        fn lis_realistic_reconcile_shift_one() {
            let s = compute_lis(&[2, 0, 1, 3]);
            assert_eq!(s.len(), 3);
        }

        /// Regression: in a keyed reorder of mixed-kind children, the LIS
        /// branch of `reconcile_keyed_middle` previously called
        /// `update_child_tracked(parent, target_panel_idx, ..)` even though an
        /// earlier non-LIS move had already shifted the LIS element to a
        /// different actual slot. The reconciler then updated the wrong
        /// control: e.g. it pushed a `Click` binding (from a Button element)
        /// onto a TextBlock control, which crashed the WinUI backend with
        /// `STATUS_STACK_BUFFER_OVERRUN` (see `winui/backend.rs::attach_event`).
        ///
        /// Scenario: a vertical StackPanel with five keyed children
        /// `[empty(TextBlock), X(Button), A(Button), B(Button), C(Button)]`
        /// reordered to `[X, A, empty, B, C]`. Empty moves 0 -> 2, X moves
        /// 1 -> 0, A moves 2 -> 1, B/C stay (LIS includes new indices 0, 1,
        /// 3, 4 with old positions 1, 2, 3, 4). With the buggy left-to-right
        /// pass, the LIS update for new[0]=X runs at `target_panel_idx=0`
        /// while slot 0 still holds the empty TextBlock, so Click bindings
        /// get attached to the TextBlock — that's the crash.
        #[test]
        fn keyed_reorder_with_mixed_kinds_keeps_controls_aligned() {
            use crate::element::{Button, Element, StackPanel, TextBlock};
            use crate::ir::{ControlKind, Op, PropValue, RecordingBackend};
            use crate::reconcile::Reconciler;
            use std::rc::Rc;

            fn tile(key: &'static str) -> Element {
                let mut b = Button::new(key);
                b.key = Some(key.to_string());
                b.on_click = Some(crate::callback::Callback::new(|()| {}));
                Element::Button(b)
            }
            fn empty() -> Element {
                let mut t = TextBlock::new("");
                t.key = Some("empty".to_string());
                Element::TextBlock(t)
            }
            fn stack(children: Vec<Element>) -> Element {
                let mut s = StackPanel::vertical();
                s.children = children;
                Element::StackPanel(s)
            }

            let old = stack(vec![
                empty(),
                tile("tile-X"),
                tile("tile-A"),
                tile("tile-B"),
                tile("tile-C"),
            ]);
            let new = stack(vec![
                tile("tile-X"),
                tile("tile-A"),
                empty(),
                tile("tile-B"),
                tile("tile-C"),
            ]);

            let mut reconciler = Reconciler::new(RecordingBackend::new());
            let noop: Rc<dyn Fn()> = Rc::new(|| {});

            let root = reconciler
                .reconcile(None, &old, None, Rc::clone(&noop))
                .unwrap();

            // Snapshot (id -> kind) from the mount-phase ops.
            let mut kind_of: rustc_hash::FxHashMap<crate::ir::ControlId, ControlKind> =
                rustc_hash::FxHashMap::default();
            for op in &reconciler.backend.ops {
                if let Op::Create { id, kind } = op {
                    kind_of.insert(*id, *kind);
                }
            }
            // Map mount-order button ids to their key labels for the
            // post-reorder label check.
            let mount_button_ids: Vec<crate::ir::ControlId> = reconciler
                .backend
                .ops
                .iter()
                .filter_map(|op| match op {
                    Op::Create {
                        id,
                        kind: ControlKind::Button,
                    } => Some(*id),
                    _ => None,
                })
                .collect();
            let mount_button_labels = ["tile-X", "tile-A", "tile-B", "tile-C"];
            assert_eq!(mount_button_ids.len(), mount_button_labels.len());

            reconciler.backend.clear_ops();
            reconciler
                .reconcile(Some(&old), &new, Some(root), Rc::clone(&noop))
                .unwrap();

            // Pick up any new ids minted by the update (shouldn't be any).
            for op in &reconciler.backend.ops {
                if let Op::Create { id, kind } = op {
                    kind_of.insert(*id, *kind);
                }
            }

            // 1. No `Click` attach should land on a non-Button control —
            //    this is the assertion that fails (and would panic in WinUI)
            //    with the buggy reconciler.
            for op in &reconciler.backend.ops {
                if let Op::AttachEvent {
                    id,
                    event: crate::ir::Event::Click,
                    ..
                } = op
                {
                    let kind = kind_of.get(id).copied();
                    assert_eq!(
                        kind,
                        Some(ControlKind::Button),
                        "Click attach landed on non-Button control {id} (kind={kind:?}). \
                     The LIS branch of reconcile_keyed_middle is desynced from \
                     key_to_panel — see this test's doc comment."
                    );
                }
            }

            // 2. After reconciliation, every button control still carries
            //    the label that matches its original key.
            let mut latest_label: rustc_hash::FxHashMap<crate::ir::ControlId, String> =
                rustc_hash::FxHashMap::default();
            for op in &reconciler.backend.ops {
                if let Op::SetProp {
                    id,
                    prop: crate::ir::Prop::Content,
                    value: PropValue::Str(s),
                } = op
                {
                    latest_label.insert(*id, s.clone());
                }
            }
            for (id, want) in mount_button_ids.iter().zip(mount_button_labels.iter()) {
                if let Some(got) = latest_label.get(id) {
                    assert_eq!(
                        got, want,
                        "Button control {id} should still carry label {want} after the \
                     reorder, but the latest ButtonContent set was {got:?}. The LIS \
                     branch of reconcile_keyed_middle is pushing the wrong element's \
                     bindings onto displaced controls."
                    );
                }
            }
        }
    }
}
mod diff_helpers {
    use super::*;

    impl<B: Backend> Reconciler<B> {
        pub(crate) fn diff_opt_f64(
            &mut self,
            id: ControlId,
            prop: Prop,
            old: Option<f64>,
            new: Option<f64>,
        ) {
            if old == new {
                return;
            }
            match new {
                Some(v) => self.backend.set_prop(id, prop, &PropValue::F64(v)),
                None => self.backend.set_prop(id, prop, &PropValue::Unset),
            }
        }

        pub(crate) fn diff_opt_copy<T: Copy + PartialEq>(
            &mut self,
            id: ControlId,
            prop: Prop,
            old: Option<T>,
            new: Option<T>,
            wrap: fn(T) -> PropValue,
        ) {
            if old == new {
                return;
            }
            match new {
                Some(v) => self.backend.set_prop(id, prop, &wrap(v)),
                None => self.backend.set_prop(id, prop, &PropValue::Unset),
            }
        }

        pub(crate) fn diff_opt_clone<T: Clone + PartialEq>(
            &mut self,
            id: ControlId,
            prop: Prop,
            old: &Option<T>,
            new: &Option<T>,
            wrap: fn(T) -> PropValue,
        ) {
            if old == new {
                return;
            }
            match new {
                Some(v) => self.backend.set_prop(id, prop, &wrap(v.clone())),
                None => self.backend.set_prop(id, prop, &PropValue::Unset),
            }
        }

        pub(crate) fn apply_props(&mut self, id: ControlId, bindings: &[Binding]) {
            for b in bindings {
                match b {
                    Binding::Prop(p, v) => self.backend.set_prop(id, *p, v),
                    Binding::Event(e, Some(h)) => self.backend.attach_event(id, *e, h.clone()),
                    Binding::Event(_, None) => {}
                }
            }
        }

        pub(crate) fn diff_props(&mut self, id: ControlId, old: &[Binding], new: &[Binding]) {
            for b in new {
                match b {
                    Binding::Prop(p, v) => match find_prop(old, *p) {
                        Some(ov) if ov == v => {}
                        _ => self.backend.set_prop(id, *p, v),
                    },
                    Binding::Event(e, new_h) => {
                        let old_inner: Option<&_> = find_event(old, *e).and_then(|o| o.as_ref());
                        if old_inner == new_h.as_ref() {
                            continue;
                        }
                        match new_h {
                            Some(h) => self.backend.attach_event(id, *e, h.clone()),
                            None => self.backend.detach_event(id, *e),
                        }
                    }
                }
            }
            for b in old {
                match b {
                    Binding::Prop(p, _) => {
                        if find_prop(new, *p).is_none() {
                            self.backend.set_prop(id, *p, &PropValue::Unset);
                        }
                    }
                    Binding::Event(e, old_h) => {
                        if find_event(new, *e).is_none() && old_h.is_some() {
                            self.backend.detach_event(id, *e);
                        }
                    }
                }
            }
        }
    }
}
mod templated {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    pub(crate) struct TemplatedListState {
        pub(crate) element: TemplatedListElement,
        pub(crate) rows: Vec<Option<RealizedRow>>,
    }

    pub(crate) struct RealizedRow {
        pub(crate) rendered: Element,
        pub(crate) content_id: ControlId,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum RealizationRequest {
        Realize { list_id: ControlId, row_idx: usize },
        Recycle { list_id: ControlId, row_idx: usize },
    }

    pub type RealizationQueue = Rc<RefCell<Vec<RealizationRequest>>>;

    pub fn new_realization_queue() -> RealizationQueue {
        Rc::new(RefCell::new(Vec::new()))
    }

    impl<B: Backend + 'static> Reconciler<B> {
        pub(crate) fn mount_templated_list(&mut self, tl: &TemplatedListElement) -> ControlId {
            let kind = match tl.kind {
                TemplatedKind::ListView => ControlKind::ListView,
                TemplatedKind::GridView => ControlKind::GridView,
                TemplatedKind::FlipView => ControlKind::FlipView,
            };
            let id = self.acquire_control(kind);
            self.apply_modifiers(id, &tl.modifiers);

            let count = tl.item_count();
            let mut rows: Vec<Option<RealizedRow>> = Vec::with_capacity(count);
            rows.resize_with(count, || None);
            self.templated_lists.insert(
                id,
                TemplatedListState {
                    element: tl.clone(),
                    rows,
                },
            );

            if tl.has_selection_handler() {
                let trampoline = self.selection_callbacks.entry(id).or_default().clone();
                *trampoline.borrow_mut() = tl.raw_selection_callback();
                let trampoline_c = Rc::clone(&trampoline);
                let cb = crate::callback::Callback::new(move |idx: i32| {
                    if let Some(inner) = trampoline_c.borrow().as_ref() {
                        inner.invoke(idx);
                    }
                });
                self.backend.attach_templated_selection_changed(id, cb);
            }

            let queue_r = Rc::clone(&self.realization_queue);
            let queue_c = Rc::clone(&self.realization_queue);
            let list_id = id;
            let realize: Rc<dyn Fn(usize)> = Rc::new(move |row_idx: usize| {
                queue_r
                    .borrow_mut()
                    .push(RealizationRequest::Realize { list_id, row_idx });
            });
            let recycle: Rc<dyn Fn(usize)> = Rc::new(move |row_idx: usize| {
                queue_c
                    .borrow_mut()
                    .push(RealizationRequest::Recycle { list_id, row_idx });
            });
            self.backend
                .attach_templated_realization(id, realize, recycle);

            self.backend.set_templated_item_count(id, count);
            self.backend
                .set_templated_selection_mode(id, tl.selection_mode);
            let sel = tl.selected_index();
            if sel >= 0 {
                self.backend.set_templated_selected_index(id, sel);
            }

            if self.eager_templated_realization {
                let mut q = self.realization_queue.borrow_mut();
                for row_idx in 0..count {
                    q.push(RealizationRequest::Realize {
                        list_id: id,
                        row_idx,
                    });
                }
            }

            id
        }

        pub(crate) fn update_templated_list(
            &mut self,
            old: &TemplatedListElement,
            new: &TemplatedListElement,
            id: ControlId,
        ) {
            self.diff_modifiers(id, &old.modifiers, &new.modifiers);

            if let Some(state) = self.templated_lists.get_mut(&id) {
                state.element = new.clone();
            }

            if let Some(cell) = self.selection_callbacks.get(&id) {
                *cell.borrow_mut() = new.raw_selection_callback();
            }

            if old.selected_index() != new.selected_index() {
                self.backend
                    .set_templated_selected_index(id, new.selected_index());
            }

            if old.selection_mode != new.selection_mode {
                self.backend
                    .set_templated_selection_mode(id, new.selection_mode);
            }

            let old_count = old.item_count();
            let new_count = new.item_count();
            if old_count != new_count {
                let to_unmount: Vec<(usize, ControlId)> = {
                    let state = self.templated_lists.get_mut(&id).unwrap();
                    let mut out = Vec::new();
                    if new_count < state.rows.len() {
                        for (i, row) in state.rows.drain(new_count..).enumerate() {
                            if let Some(row) = row {
                                out.push((new_count + i, row.content_id));
                            }
                        }
                    }
                    state.rows.resize_with(new_count, || None);
                    out
                };
                for (row_idx, cid) in to_unmount {
                    self.backend.set_templated_row_content(id, row_idx, None);
                    self.unmount(cid);
                }
                self.backend.set_templated_item_count(id, new_count);
            }

            if !old.same_items_as(new) {
                self.refresh_realized_rows(id, new);
            }
        }

        fn refresh_realized_rows(&mut self, id: ControlId, new: &TemplatedListElement) {
            let realized_indices: Vec<usize> = {
                let state = self.templated_lists.get(&id).unwrap();
                state
                    .rows
                    .iter()
                    .enumerate()
                    .filter_map(|(i, r)| r.as_ref().map(|_| i))
                    .collect()
            };

            for row_idx in realized_indices {
                if row_idx >= new.item_count() {
                    continue;
                }
                let (old_el, content_id) = {
                    let state = self.templated_lists.get(&id).unwrap();
                    let row = state.rows[row_idx].as_ref().unwrap();
                    (row.rendered.clone(), row.content_id)
                };
                let new_el = new.build_item_view(row_idx);

                if can_skip_update(&old_el, &new_el) {
                    self.debug_elements_skipped += 1;
                    if let Some(state) = self.templated_lists.get_mut(&id)
                        && let Some(Some(row)) = state.rows.get_mut(row_idx)
                    {
                        row.rendered = new_el;
                    }
                    continue;
                }

                let new_id = self.update(&old_el, &new_el, content_id);
                if let Some(nid) = new_id {
                    if let Some(state) = self.templated_lists.get_mut(&id)
                        && let Some(slot) = state.rows.get_mut(row_idx)
                    {
                        *slot = Some(RealizedRow {
                            rendered: new_el,
                            content_id: nid,
                        });
                    }
                    if nid != content_id {
                        self.backend
                            .set_templated_row_content(id, row_idx, Some(nid));
                    }
                } else {
                    if let Some(state) = self.templated_lists.get_mut(&id)
                        && let Some(slot) = state.rows.get_mut(row_idx)
                    {
                        *slot = None;
                    }
                    self.backend.set_templated_row_content(id, row_idx, None);
                }
            }
        }

        pub fn drain_realizations(&mut self) {
            let drained = {
                let mut q = self.realization_queue.borrow_mut();
                std::mem::take(&mut *q)
            };
            for req in drained {
                match req {
                    RealizationRequest::Realize { list_id, row_idx } => {
                        self.realize_row_inner(list_id, row_idx);
                    }
                    RealizationRequest::Recycle { list_id, row_idx } => {
                        self.recycle_row_inner(list_id, row_idx);
                    }
                }
            }
        }

        fn realize_row_inner(&mut self, list_id: ControlId, row_idx: usize) {
            let (rendered, count) = {
                let Some(state) = self.templated_lists.get(&list_id) else {
                    return;
                };
                if row_idx >= state.element.item_count() {
                    return;
                }
                (
                    state.element.build_item_view(row_idx),
                    state.element.item_count(),
                )
            };

            if let Some(existing) = self.clear_row_realized_state(list_id, row_idx) {
                self.unmount(existing);
            }

            let Some(content_id) = self.mount(&rendered) else {
                return;
            };

            self.backend
                .set_templated_row_content(list_id, row_idx, Some(content_id));
            if let Some(state) = self.templated_lists.get_mut(&list_id) {
                if state.rows.len() < count {
                    state.rows.resize_with(count, || None);
                }
                state.rows[row_idx] = Some(RealizedRow {
                    rendered,
                    content_id,
                });
            }

            self.dispatch_appeared(content_id);
        }

        fn recycle_row_inner(&mut self, list_id: ControlId, row_idx: usize) {
            let Some(existing) = self.clear_row_realized_state(list_id, row_idx) else {
                return;
            };

            self.dispatch_disappeared(existing);
            self.backend
                .set_templated_row_content(list_id, row_idx, None);
            if self.defer_templated_unmounts {
                self.deferred_unmounts.push(existing);
            } else {
                self.unmount(existing);
            }
        }

        fn clear_row_realized_state(
            &mut self,
            list_id: ControlId,
            row_idx: usize,
        ) -> Option<ControlId> {
            let state = self.templated_lists.get_mut(&list_id)?;
            let row = state.rows.get_mut(row_idx)?;
            row.take().map(|r| r.content_id)
        }

        fn dispatch_appeared(&mut self, id: ControlId) {
            if self.appeared_listener_count == 0 {
                return;
            }
            let subtree = self.collect_subtree_ids(id);
            for node in subtree {
                if let Some(inst) = self.component_instances.get_mut(&node) {
                    if !inst.last_obj.has_on_appeared() {
                        continue;
                    }
                    inst.render_cx
                        .set_context_stack(Rc::clone(&self.context_stack));
                    inst.last_obj.invoke_appeared(&mut inst.render_cx);
                }
            }
        }

        fn dispatch_disappeared(&mut self, id: ControlId) {
            if self.disappeared_listener_count == 0 {
                return;
            }
            let subtree = self.collect_subtree_ids(id);
            for node in subtree {
                if let Some(inst) = self.component_instances.get_mut(&node) {
                    if !inst.last_obj.has_on_disappeared() {
                        continue;
                    }
                    inst.render_cx
                        .set_context_stack(Rc::clone(&self.context_stack));
                    inst.last_obj.invoke_disappeared(&mut inst.render_cx);
                }
            }
        }

        fn collect_subtree_ids(&self, id: ControlId) -> Vec<ControlId> {
            let mut out = Vec::new();
            let mut stack = vec![id];
            while let Some(node) = stack.pop() {
                out.push(node);
                if let Some(children) = self.children_mirror.get(&node) {
                    for child in children.iter().rev() {
                        stack.push(*child);
                    }
                }
            }
            out
        }
    }
}
mod widget_dispatch {
    use super::*;

    impl<B: Backend + 'static> Reconciler<B> {
        pub(crate) fn mount_widget(&mut self, w: &dyn Widget) -> ControlId {
            let id = self.acquire_control(w.kind());
            self.apply_props(id, &w.bindings());
            self.apply_modifiers(id, w.modifiers());
            self.apply_attached(id, w.attached());
            self.mount_widget_children(id, w.children());
            if let Some(hdr) = w.header_element()
                && let Some(hdr_id) = self.mount(hdr)
            {
                self.backend.set_header_element(id, Some(hdr_id));
                self.header_elements.insert(id, hdr_id);
            }
            if let Some(pane) = w.pane_element()
                && let Some(pane_id) = self.mount(pane)
            {
                self.backend.set_pane_element(id, Some(pane_id));
                self.pane_elements.insert(id, pane_id);
            }
            if let Some(cb) = w.on_mounted_callback() {
                cb.invoke(self.backend.get_native_element(id));
            }
            if let Some(cb) = w.on_unmounted_callback() {
                self.unmount_callbacks.insert(id, cb.clone());
            }
            id
        }

        pub(crate) fn update_widget(&mut self, old: &dyn Widget, new: &dyn Widget, id: ControlId) {
            self.diff_props(id, &old.bindings(), &new.bindings());
            self.diff_modifiers(id, old.modifiers(), new.modifiers());
            self.diff_attached(id, old.attached(), new.attached());
            self.update_widget_children(id, old.children(), new.children());
            self.update_header_element(id, old.header_element(), new.header_element());
            self.update_pane_element(id, old.pane_element(), new.pane_element());
            if let Some(cb) = new.on_unmounted_callback() {
                self.unmount_callbacks.insert(id, cb.clone());
            } else {
                self.unmount_callbacks.remove(&id);
            }
        }

        fn mount_widget_children(&mut self, id: ControlId, children: Children<'_>) {
            match children {
                Children::None => {}
                Children::PositionalSingle(child) => {
                    if let Some(child_id) = self.mount(child) {
                        self.append_child_tracked(id, child_id);
                    }
                }
                Children::Keyed(list) => {
                    for child in crate::reconcile::collect_live(list) {
                        if let Some(child_id) = self.mount(child) {
                            self.append_child_tracked(id, child_id);
                        }
                    }
                }
                Children::PivotItems(items) => {
                    for item in items {
                        self.mount_pivot_item(id, item);
                    }
                }
            }
        }

        fn update_widget_children(&mut self, id: ControlId, old: Children<'_>, new: Children<'_>) {
            match (old, new) {
                (Children::None, Children::None) => {}
                (Children::PositionalSingle(o), Children::PositionalSingle(n)) => {
                    let oc = std::slice::from_ref(o);
                    let nc = std::slice::from_ref(n);
                    self.reconcile_children_positional(id, oc, nc);
                }
                (Children::Keyed(o), Children::Keyed(n)) => {
                    self.reconcile_children(id, o, n);
                }
                (Children::PivotItems(o), Children::PivotItems(n)) => {
                    self.reconcile_pivot_items(id, o, n);
                }
                _ => {
                    debug_assert!(
                        false,
                        "update_widget_children: child-layout shape changed across update"
                    );
                }
            }
        }

        fn update_header_element(
            &mut self,
            id: ControlId,
            old: Option<&Element>,
            new: Option<&Element>,
        ) {
            match (old, new) {
                (None, None) => {}
                (None, Some(hdr)) => {
                    if let Some(hdr_id) = self.mount(hdr) {
                        self.backend.set_header_element(id, Some(hdr_id));
                        self.header_elements.insert(id, hdr_id);
                    }
                }
                (Some(_), None) => {
                    if let Some(hdr_id) = self.header_elements.remove(&id) {
                        self.backend
                            .set_header_element(id, Option::<ControlId>::None);
                        self.unmount(hdr_id);
                    }
                }
                (Some(old_el), Some(new_el)) => {
                    // Reconcile in-place when possible to preserve focus/state.
                    if let Some(hdr_id) = self.header_elements.get(&id).copied() {
                        if old_el.kind_matches(new_el) {
                            let new_id = self.update(old_el, new_el, hdr_id);
                            match new_id {
                                Some(nid) if nid != hdr_id => {
                                    self.backend.set_header_element(id, Some(nid));
                                    self.header_elements.insert(id, nid);
                                }
                                None => {
                                    self.backend
                                        .set_header_element(id, Option::<ControlId>::None);
                                    self.header_elements.remove(&id);
                                }
                                _ => {}
                            }
                            return;
                        }
                        self.header_elements.remove(&id);
                        self.unmount(hdr_id);
                    }
                    if let Some(hdr_id) = self.mount(new_el) {
                        self.backend.set_header_element(id, Some(hdr_id));
                        self.header_elements.insert(id, hdr_id);
                    }
                }
            }
        }

        fn update_pane_element(
            &mut self,
            id: ControlId,
            old: Option<&Element>,
            new: Option<&Element>,
        ) {
            match (old, new) {
                (None, None) => {}
                (None, Some(pane)) => {
                    if let Some(pane_id) = self.mount(pane) {
                        self.backend.set_pane_element(id, Some(pane_id));
                        self.pane_elements.insert(id, pane_id);
                    }
                }
                (Some(_), None) => {
                    if let Some(pane_id) = self.pane_elements.remove(&id) {
                        self.backend.set_pane_element(id, Option::<ControlId>::None);
                        self.unmount(pane_id);
                    }
                }
                (Some(old_el), Some(new_el)) => {
                    // Reconcile in-place when possible to preserve focus/state.
                    if let Some(pane_id) = self.pane_elements.get(&id).copied() {
                        if old_el.kind_matches(new_el) {
                            let new_id = self.update(old_el, new_el, pane_id);
                            match new_id {
                                Some(nid) if nid != pane_id => {
                                    self.backend.set_pane_element(id, Some(nid));
                                    self.pane_elements.insert(id, nid);
                                }
                                None => {
                                    self.backend.set_pane_element(id, Option::<ControlId>::None);
                                    self.pane_elements.remove(&id);
                                }
                                _ => {}
                            }
                            return;
                        }
                        self.pane_elements.remove(&id);
                        self.unmount(pane_id);
                    }
                    if let Some(pane_id) = self.mount(new_el) {
                        self.backend.set_pane_element(id, Some(pane_id));
                        self.pane_elements.insert(id, pane_id);
                    }
                }
            }
        }

        fn mount_pivot_item(&mut self, parent: ControlId, item: &PivotItem) {
            let item_id = self.acquire_control(crate::ir::ControlKind::PivotItem);
            self.backend.set_prop(
                item_id,
                Prop::ItemHeader,
                &PropValue::Str(item.header.clone()),
            );
            if let Some(content_id) = self.mount(&item.content) {
                self.append_child_tracked(item_id, content_id);
            }
            self.append_child_tracked(parent, item_id);
        }

        fn reconcile_pivot_items(
            &mut self,
            parent: ControlId,
            old: &[PivotItem],
            new: &[PivotItem],
        ) {
            let common = old.len().min(new.len());

            for i in 0..common {
                let Some(item_id) = self.child_at(parent, i) else {
                    continue;
                };
                let o = &old[i];
                let n = &new[i];
                if o.header != n.header {
                    self.backend.set_prop(
                        item_id,
                        Prop::ItemHeader,
                        &PropValue::Str(n.header.clone()),
                    );
                }
                let oc = std::slice::from_ref(&o.content);
                let nc = std::slice::from_ref(&n.content);
                self.reconcile_children_positional(item_id, oc, nc);
            }

            if new.len() > old.len() {
                for n in &new[old.len()..] {
                    self.mount_pivot_item(parent, n);
                }
            }

            while self.child_at(parent, new.len()).is_some() {
                let extra_id = self.child_at(parent, new.len()).unwrap();
                self.remove_child_tracked(parent, new.len());
                self.unmount(extra_id);
            }
        }

        pub(crate) fn apply_attached(&mut self, id: ControlId, attached: Option<&AttachedProps>) {
            let Some(att) = attached else { return };
            // GridPlacement is now on Modifiers::grid — handled by apply_modifiers.
            if let Some(p) = att.get::<crate::widgets::CanvasPosition>() {
                self.apply_canvas_position(id, *p);
            }
            if let Some(p) = att.get::<crate::widgets::RelativePanelAlignment>() {
                self.apply_relative_panel_alignment(id, *p);
            }
        }

        pub(crate) fn apply_grid_placement(&mut self, id: ControlId, p: GridPlacement) {
            if p.row != 0 {
                self.backend
                    .set_prop(id, Prop::AttachedGridRow, &PropValue::I32(p.row));
            }
            if p.column != 0 {
                self.backend
                    .set_prop(id, Prop::AttachedGridColumn, &PropValue::I32(p.column));
            }
            if p.row_span > 1 {
                self.backend
                    .set_prop(id, Prop::AttachedGridRowSpan, &PropValue::I32(p.row_span));
            }
            if p.column_span > 1 {
                self.backend.set_prop(
                    id,
                    Prop::AttachedGridColumnSpan,
                    &PropValue::I32(p.column_span),
                );
            }
        }

        /// Unconditionally emits all four grid attached props — used in the diff
        /// path to clear stale values when placement changes or is removed.
        pub(crate) fn apply_grid_placement_full(&mut self, id: ControlId, p: GridPlacement) {
            self.backend
                .set_prop(id, Prop::AttachedGridRow, &PropValue::I32(p.row));
            self.backend
                .set_prop(id, Prop::AttachedGridColumn, &PropValue::I32(p.column));
            self.backend
                .set_prop(id, Prop::AttachedGridRowSpan, &PropValue::I32(p.row_span));
            self.backend.set_prop(
                id,
                Prop::AttachedGridColumnSpan,
                &PropValue::I32(p.column_span),
            );
        }

        pub(crate) fn apply_canvas_position(
            &mut self,
            id: ControlId,
            p: crate::widgets::CanvasPosition,
        ) {
            // Canvas defaults are 0.0 — only emit when non-zero on mount;
            // the diff path always emits to overwrite the previous value.
            if p.left != 0.0 {
                self.backend
                    .set_prop(id, Prop::AttachedCanvasLeft, &PropValue::F64(p.left));
            }
            if p.top != 0.0 {
                self.backend
                    .set_prop(id, Prop::AttachedCanvasTop, &PropValue::F64(p.top));
            }
            if p.z_index != 0 {
                self.backend
                    .set_prop(id, Prop::AttachedCanvasZIndex, &PropValue::I32(p.z_index));
            }
        }

        pub(crate) fn diff_attached(
            &mut self,
            id: ControlId,
            old: Option<&AttachedProps>,
            new: Option<&AttachedProps>,
        ) {
            // GridPlacement is now on Modifiers::grid — handled by diff_modifiers.

            let old_canvas = old
                .and_then(|a| a.get::<crate::widgets::CanvasPosition>())
                .copied();
            let new_canvas = new
                .and_then(|a| a.get::<crate::widgets::CanvasPosition>())
                .copied();
            if old_canvas != new_canvas {
                let p = new_canvas.unwrap_or_default();
                self.backend
                    .set_prop(id, Prop::AttachedCanvasLeft, &PropValue::F64(p.left));
                self.backend
                    .set_prop(id, Prop::AttachedCanvasTop, &PropValue::F64(p.top));
                self.backend
                    .set_prop(id, Prop::AttachedCanvasZIndex, &PropValue::I32(p.z_index));
            }

            let old_rp = old
                .and_then(|a| a.get::<crate::widgets::RelativePanelAlignment>())
                .copied();
            let new_rp = new
                .and_then(|a| a.get::<crate::widgets::RelativePanelAlignment>())
                .copied();
            if old_rp != new_rp {
                let p = new_rp.unwrap_or_default();
                self.apply_relative_panel_alignment_full(id, p);
            }
        }

        pub(crate) fn apply_relative_panel_alignment(
            &mut self,
            id: ControlId,
            p: crate::widgets::RelativePanelAlignment,
        ) {
            if p.align_left_with_panel {
                self.backend
                    .set_prop(id, Prop::AlignLeftWithPanel, &PropValue::Bool(true));
            }
            if p.align_right_with_panel {
                self.backend
                    .set_prop(id, Prop::AlignRightWithPanel, &PropValue::Bool(true));
            }
            if p.align_top_with_panel {
                self.backend
                    .set_prop(id, Prop::AlignTopWithPanel, &PropValue::Bool(true));
            }
            if p.align_bottom_with_panel {
                self.backend
                    .set_prop(id, Prop::AlignBottomWithPanel, &PropValue::Bool(true));
            }
            if p.align_h_center_with_panel {
                self.backend
                    .set_prop(id, Prop::AlignHCenterWithPanel, &PropValue::Bool(true));
            }
            if p.align_v_center_with_panel {
                self.backend
                    .set_prop(id, Prop::AlignVCenterWithPanel, &PropValue::Bool(true));
            }
        }

        fn apply_relative_panel_alignment_full(
            &mut self,
            id: ControlId,
            p: crate::widgets::RelativePanelAlignment,
        ) {
            self.backend.set_prop(
                id,
                Prop::AlignLeftWithPanel,
                &PropValue::Bool(p.align_left_with_panel),
            );
            self.backend.set_prop(
                id,
                Prop::AlignRightWithPanel,
                &PropValue::Bool(p.align_right_with_panel),
            );
            self.backend.set_prop(
                id,
                Prop::AlignTopWithPanel,
                &PropValue::Bool(p.align_top_with_panel),
            );
            self.backend.set_prop(
                id,
                Prop::AlignBottomWithPanel,
                &PropValue::Bool(p.align_bottom_with_panel),
            );
            self.backend.set_prop(
                id,
                Prop::AlignHCenterWithPanel,
                &PropValue::Bool(p.align_h_center_with_panel),
            );
            self.backend.set_prop(
                id,
                Prop::AlignVCenterWithPanel,
                &PropValue::Bool(p.align_v_center_with_panel),
            );
        }
    }
}
mod wrappers {
    use std::panic::AssertUnwindSafe;
    use std::rc::Rc;

    use super::*;

    impl<B: Backend + 'static> Reconciler<B> {
        pub(crate) fn mount_component(&mut self, ce: &ComponentElement) -> Option<ControlId> {
            let mut cx = RenderCx::new(Rc::clone(&self.request_rerender));
            cx.set_context_stack(self.context_stack_handle());
            cx.set_marshaller(self.marshaller.clone());
            cx.set_inner_size_cell(Rc::clone(&self.inner_size));
            cx.set_dpi_cell(Rc::clone(&self.dpi));
            cx.begin_render();
            let rendered = ce.obj.render(&mut cx);
            cx.flush_effects();
            let read_contexts = cx.take_read_contexts();

            let rendered_id = self.mount(&rendered)?;
            self.register_component_instance(
                rendered_id,
                ComponentInstance {
                    render_cx: cx,
                    last_rendered: rendered,
                    last_obj: Rc::clone(&ce.obj),
                    read_contexts,
                },
            );
            Some(rendered_id)
        }

        pub(crate) fn update_component(
            &mut self,
            old: &ComponentElement,
            new: &ComponentElement,
            id: ControlId,
        ) -> Option<ControlId> {
            if old.obj.component_type_id() != new.obj.component_type_id() {
                self.unmount(id);
                return self.mount_component(new);
            }

            let forced_by_context = self.forced_components.contains(&id);
            let state_dirty = self
                .component_instances
                .get(&id)
                .is_some_and(|inst| inst.render_cx.take_state_dirty());
            let needs_update = if forced_by_context || state_dirty {
                true
            } else if new.memoised {
                !old.obj.is_equivalent(&*new.obj)
            } else {
                old.obj.should_update(&*new.obj)
            };

            if !needs_update {
                if let Some(inst) = self.component_instances.get_mut(&id) {
                    inst.last_obj = Rc::clone(&new.obj);
                }
                return Some(id);
            }

            let Some(mut inst) = self.take_component_instance(id) else {
                return self.mount_component(new);
            };

            inst.render_cx
                .set_context_stack(self.context_stack_handle());
            inst.render_cx
                .set_inner_size_cell(Rc::clone(&self.inner_size));
            inst.render_cx.set_dpi_cell(Rc::clone(&self.dpi));
            inst.render_cx.begin_render();
            let rendered = new.obj.render(&mut inst.render_cx);
            inst.render_cx.flush_effects();
            inst.read_contexts = inst.render_cx.take_read_contexts();

            let new_id = self.update(&inst.last_rendered, &rendered, id);

            inst.last_rendered = rendered;
            inst.last_obj = Rc::clone(&new.obj);

            if let Some(nid) = new_id {
                self.register_component_instance(nid, inst);
                Some(nid)
            } else {
                inst.render_cx.run_cleanups();
                None
            }
        }

        pub(crate) fn mount_error_boundary(
            &mut self,
            eb: &ErrorBoundaryElement,
        ) -> Option<ControlId> {
            let _expect = crate::diagnostics::ExpectPanicGuard::new();
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| self.mount(&eb.child)));
            match result {
                Ok(id) => id,
                Err(payload) => {
                    let msg = panic_message(payload);
                    let fallback = eb.fallback.invoke(&msg);
                    let id = self.mount(&fallback);
                    if let Some(id) = id {
                        self.error_boundary_fallbacks.insert(id);
                    }
                    id
                }
            }
        }

        pub(crate) fn update_error_boundary(
            &mut self,
            old: &ErrorBoundaryElement,
            new: &ErrorBoundaryElement,
            id: ControlId,
        ) -> Option<ControlId> {
            if self.error_boundary_fallbacks.remove(&id) {
                self.unmount(id);
                let _expect = crate::diagnostics::ExpectPanicGuard::new();
                let result = std::panic::catch_unwind(AssertUnwindSafe(|| self.mount(&new.child)));
                return match result {
                    Ok(nid) => nid,
                    Err(payload) => {
                        let msg = panic_message(payload);
                        let fallback = new.fallback.invoke(&msg);
                        let nid = self.mount(&fallback);
                        if let Some(nid) = nid {
                            self.error_boundary_fallbacks.insert(nid);
                        }
                        nid
                    }
                };
            }

            let _expect = crate::diagnostics::ExpectPanicGuard::new();
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                self.update(&old.child, &new.child, id)
            }));
            match result {
                Ok(nid) => nid,
                Err(payload) => {
                    let msg = panic_message(payload);
                    let fallback = new.fallback.invoke(&msg);
                    self.unmount(id);
                    let nid = self.mount(&fallback);
                    if let Some(nid) = nid {
                        self.error_boundary_fallbacks.insert(nid);
                    }
                    nid
                }
            }
        }

        pub(crate) fn mount_provider(&mut self, p: &ProviderElement) -> Option<ControlId> {
            let pushed = self.push_provisions(&p.provisions);
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| self.mount(&p.child)));
            self.pop_provisions(pushed);
            match result {
                Ok(id) => id,
                Err(payload) => std::panic::resume_unwind(payload),
            }
        }

        pub(crate) fn update_provider(
            &mut self,
            old: &ProviderElement,
            new: &ProviderElement,
            id: ControlId,
        ) -> Option<ControlId> {
            let mut changed_ids: rustc_hash::FxHashSet<crate::context::ContextId> =
                rustc_hash::FxHashSet::default();

            let mut old_by_id: rustc_hash::FxHashMap<
                crate::context::ContextId,
                &crate::context::ContextProvision,
            > = rustc_hash::FxHashMap::default();
            for p in &old.provisions {
                old_by_id.insert(p.context_id, p);
            }
            for new_p in &new.provisions {
                match old_by_id.remove(&new_p.context_id) {
                    None => {
                        changed_ids.insert(new_p.context_id);
                    }
                    Some(old_p) => {
                        if old_p != new_p {
                            changed_ids.insert(new_p.context_id);
                        }
                    }
                }
            }

            for old_id in old_by_id.into_keys() {
                changed_ids.insert(old_id);
            }

            let prev_flag = self.force_component_rerender;
            let saved_forced = self.forced_components.clone();
            if !changed_ids.is_empty() {
                let affected = self.collect_affected_components(id, &changed_ids);
                if !affected.is_empty() {
                    self.force_component_rerender = true;
                    self.forced_components.extend(affected);
                }
            }

            let pushed = self.push_provisions(&new.provisions);
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                self.update(&old.child, &new.child, id)
            }));
            self.pop_provisions(pushed);

            self.forced_components = saved_forced;
            self.force_component_rerender = prev_flag;

            match result {
                Ok(nid) => nid,
                Err(payload) => std::panic::resume_unwind(payload),
            }
        }

        fn push_provisions(&self, provisions: &[crate::context::ContextProvision]) -> usize {
            for p in provisions {
                self.context_stack.push_raw_retain(
                    p.context_id,
                    p.value_type_id,
                    Rc::clone(&p.value),
                );
            }
            provisions.len()
        }

        fn pop_provisions(&self, count: usize) {
            for _ in 0..count {
                self.context_stack.pop_raw();
            }
        }

        pub(crate) fn mount_custom(&mut self, c: &crate::custom::CustomElementHandle) -> ControlId {
            self.debug_ui_elements_created += 1;
            let id = c.0.mount(&mut self.backend);
            self.custom_handles.insert(id, c.0.clone_dyn());
            id
        }

        pub(crate) fn update_custom(
            &mut self,
            old: &crate::custom::CustomElementHandle,
            new: &crate::custom::CustomElementHandle,
            id: ControlId,
        ) -> ControlId {
            debug_assert_eq!(
                crate::custom::CustomElement::type_id(&*old.0),
                crate::custom::CustomElement::type_id(&*new.0),
                "update_custom invoked across different CustomElement types — \
             reconciler should have unmount+remount via kind_matches",
            );
            if !old.0.eq_dyn(&*new.0) {
                new.0.update(&*old.0, id, &mut self.backend);
                self.custom_handles.insert(id, new.0.clone_dyn());
            }
            id
        }
    }
}

pub(crate) use self::templated::TemplatedListState;
pub use self::templated::{RealizationQueue, new_realization_queue};

/// Diff/apply engine that drives a [`Backend`] from successive
/// [`Element`] trees. Owns the bookkeeping needed to reuse controls
/// across renders (children mirror, component instances, custom
/// handles, templated-list state, …).
#[doc(hidden)]
pub struct Reconciler<B: Backend> {
    pub backend: B,
    pub debug_elements_skipped: u64,
    pub debug_elements_diffed: u64,
    pub debug_ui_elements_created: u64,
    pub(crate) children_mirror: FxHashMap<ControlId, Vec<ControlId>>,
    pub(crate) id_kinds: FxHashMap<ControlId, ControlKind>,
    pub(crate) context_stack: Rc<ContextStack>,
    pub(crate) component_instances: FxHashMap<ControlId, ComponentInstance>,
    pub(crate) appeared_listener_count: usize,
    pub(crate) disappeared_listener_count: usize,
    pub(crate) force_component_rerender: bool,
    pub(crate) forced_components: rustc_hash::FxHashSet<ControlId>,
    pub(crate) error_boundary_fallbacks: rustc_hash::FxHashSet<ControlId>,
    pub(crate) templated_lists: FxHashMap<ControlId, TemplatedListState>,
    pub(crate) custom_handles: FxHashMap<ControlId, Box<dyn crate::custom::CustomElement>>,
    pub eager_templated_realization: bool,
    pub defer_templated_unmounts: bool,
    pub(crate) deferred_unmounts: Vec<ControlId>,
    pub realization_queue: RealizationQueue,
    pub(crate) selection_callbacks: FxHashMap<ControlId, Rc<RefCell<Option<Callback<i32>>>>>,
    /// Pre-unmount callbacks keyed by control id, invoked with the native
    /// element (or `None`) just before the control is destroyed (see
    /// [`Widget::on_unmounted_callback`]).
    pub(crate) unmount_callbacks:
        FxHashMap<ControlId, Callback<Option<windows_core::IInspectable>>>,
    /// Tracks header element control IDs for widgets that use header_element().
    pub(crate) header_elements: FxHashMap<ControlId, ControlId>,
    /// Tracks pane element control IDs for widgets that use pane_element().
    pub(crate) pane_elements: FxHashMap<ControlId, ControlId>,
    /// UI marshaller propagated into every nested component's
    /// [`RenderCx`] for [`RenderCx::use_async_state`].
    pub(crate) marshaller: Option<UiMarshaller>,
    pub(crate) inner_size: Rc<Cell<Size>>,
    pub(crate) dpi: Rc<Cell<u32>>,
    /// Rerender hook propagated from the render host; child components
    /// clone this into their `RenderCx` so `SetState` triggers re-render.
    pub(crate) request_rerender: Rc<dyn Fn()>,
}

pub(crate) struct ComponentInstance {
    pub(crate) render_cx: RenderCx,
    pub(crate) last_rendered: Element,
    pub(crate) last_obj: Rc<dyn crate::component_element::ComponentObject>,
    pub(crate) read_contexts: rustc_hash::FxHashSet<ContextId>,
}

impl<B: Backend + 'static> Reconciler<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            debug_elements_skipped: 0,
            debug_elements_diffed: 0,
            debug_ui_elements_created: 0,
            children_mirror: FxHashMap::default(),
            id_kinds: FxHashMap::default(),
            context_stack: Rc::new(ContextStack::new()),
            component_instances: FxHashMap::default(),
            appeared_listener_count: 0,
            disappeared_listener_count: 0,
            force_component_rerender: false,
            forced_components: rustc_hash::FxHashSet::default(),
            error_boundary_fallbacks: rustc_hash::FxHashSet::default(),
            templated_lists: FxHashMap::default(),
            custom_handles: FxHashMap::default(),
            realization_queue: new_realization_queue(),
            selection_callbacks: FxHashMap::default(),
            unmount_callbacks: FxHashMap::default(),
            header_elements: FxHashMap::default(),
            pane_elements: FxHashMap::default(),
            eager_templated_realization: false,
            defer_templated_unmounts: false,
            deferred_unmounts: Vec::new(),
            marshaller: None,
            inner_size: Rc::new(Cell::new(Size::default())),
            dpi: Rc::new(Cell::new(96_u32)),
            request_rerender: Rc::new(|| {}),
        }
    }

    /// Install the [`UiMarshaller`] propagated into nested
    /// [`RenderCx`] instances.
    pub(crate) fn set_marshaller(&mut self, marshaller: Option<UiMarshaller>) {
        self.marshaller = marshaller;
    }

    pub fn flush_deferred_unmounts(&mut self) {
        let drained = std::mem::take(&mut self.deferred_unmounts);
        for cid in drained {
            self.unmount(cid);
        }
    }

    pub(crate) fn context_stack_handle(&self) -> Rc<ContextStack> {
        Rc::clone(&self.context_stack)
    }

    /// Returns `true` if the component instance at `id` has had state
    /// modified since its last render (i.e. a `SetState`/`Updater`/`Dispatch`
    /// fired). Used by child reconciliation to bypass `can_skip_update`.
    /// Does NOT clear the flag — `update_component` consumes it.
    pub(crate) fn is_component_state_dirty(&self, id: ControlId) -> bool {
        self.component_instances
            .get(&id)
            .is_some_and(|inst| inst.render_cx.peek_state_dirty())
    }

    pub fn reset_stats(&mut self) {
        self.debug_elements_skipped = 0;
        self.debug_elements_diffed = 0;
        self.debug_ui_elements_created = 0;
    }

    #[doc(hidden)]
    pub fn debug_forced_components_len(&self) -> usize {
        self.forced_components.len()
    }

    pub(crate) fn acquire_control(&mut self, kind: ControlKind) -> ControlId {
        self.debug_ui_elements_created += 1;
        let id = self.backend.create(kind);

        if let Some(stale) = self.component_instances.remove(&id) {
            self.unregister_component_listeners(&stale);
        }
        self.templated_lists.remove(&id);
        self.selection_callbacks.remove(&id);
        self.error_boundary_fallbacks.remove(&id);
        self.children_mirror.remove(&id);
        self.custom_handles.remove(&id);
        self.id_kinds.insert(id, kind);
        id
    }

    pub(crate) fn register_component_instance(
        &mut self,
        id: ControlId,
        inst: ComponentInstance,
    ) -> Option<ComponentInstance> {
        if inst.last_obj.has_on_appeared() {
            self.appeared_listener_count += 1;
        }
        if inst.last_obj.has_on_disappeared() {
            self.disappeared_listener_count += 1;
        }
        let displaced = self.component_instances.insert(id, inst);
        if let Some(prev) = &displaced {
            self.unregister_component_listeners(prev);
        }
        displaced
    }

    pub(crate) fn take_component_instance(&mut self, id: ControlId) -> Option<ComponentInstance> {
        let inst = self.component_instances.remove(&id)?;
        self.unregister_component_listeners(&inst);
        Some(inst)
    }

    fn unregister_component_listeners(&mut self, inst: &ComponentInstance) {
        if inst.last_obj.has_on_appeared() {
            debug_assert!(
                self.appeared_listener_count > 0,
                "appeared_listener_count underflow: register/take are mismatched"
            );
            self.appeared_listener_count -= 1;
        }
        if inst.last_obj.has_on_disappeared() {
            debug_assert!(
                self.disappeared_listener_count > 0,
                "disappeared_listener_count underflow: register/take are mismatched"
            );
            self.disappeared_listener_count -= 1;
        }
    }

    #[doc(hidden)]
    pub fn debug_appeared_listener_count(&self) -> usize {
        self.appeared_listener_count
    }

    #[doc(hidden)]
    pub fn debug_disappeared_listener_count(&self) -> usize {
        self.disappeared_listener_count
    }

    pub fn reconcile(
        &mut self,
        old: Option<&Element>,
        new: &Element,
        existing: Option<ControlId>,
        request_rerender: Rc<dyn Fn()>,
    ) -> Option<ControlId> {
        self.request_rerender = request_rerender;
        match (existing, old) {
            (None, _) => self.mount(new),
            (Some(id), Some(old_el)) => self.update(old_el, new, id),
            (Some(_id), None) => self.mount(new),
        }
    }

    pub(crate) fn mount(&mut self, el: &Element) -> Option<ControlId> {
        match el {
            Element::Component(ce) => return self.mount_component(ce),
            Element::ErrorBoundary(eb) => return self.mount_error_boundary(eb),
            Element::Provider(pe) => return self.mount_provider(pe),
            Element::TemplatedList(tl) => return Some(self.mount_templated_list(tl)),
            Element::Custom(c) => return Some(self.mount_custom(c)),
            Element::Group(_) => {
                panic!(
                    "Element::Group can only appear inside a multi-child container's child list. \
                     A Group at a single-child position (e.g. as a Component's render output, \
                     or as the sole child of Border / ScrollViewer) is not supported. \
                     Wrap the Group in a StackPanel, return its single non-empty child, or place it \
                     directly inside a StackPanel/Grid that owns it as one of many children."
                );
            }
            Element::Empty => return None,
            _ => {}
        }
        let widget = el.as_widget().unwrap();
        let id = self.mount_widget(widget);
        if let Element::RichTextBlock(rt) = el
            && !rt.paragraphs.is_empty()
        {
            self.backend.set_rich_text_paragraphs(id, &rt.paragraphs);
        }
        Some(id)
    }

    pub(crate) fn update(
        &mut self,
        old: &Element,
        new: &Element,
        id: ControlId,
    ) -> Option<ControlId> {
        if !self.force_component_rerender && can_skip_update(old, new) {
            // A component with dirty internal state must still be re-rendered
            // even when its element is structurally identical.
            if !self.is_component_state_dirty(id) {
                self.debug_elements_skipped += 1;
                return Some(id);
            }
        }
        self.debug_elements_diffed += 1;

        if !old.kind_matches(new) {
            self.unmount(id);
            return self.mount(new);
        }

        match (old, new) {
            (Element::Component(o), Element::Component(n)) => {
                return self.update_component(o, n, id);
            }
            (Element::ErrorBoundary(o), Element::ErrorBoundary(n)) => {
                return self.update_error_boundary(o, n, id);
            }
            (Element::Provider(o), Element::Provider(n)) => return self.update_provider(o, n, id),
            (Element::TemplatedList(o), Element::TemplatedList(n)) => {
                self.update_templated_list(o, n, id);
                return Some(id);
            }
            (Element::Custom(o), Element::Custom(n)) => {
                return Some(self.update_custom(o, n, id));
            }
            (Element::Group(_), Element::Group(_)) => {
                panic!(
                    "Element::Group reached update() with a ControlId. \
                     Group is a fragment and cannot own a control. \
                     This usually means a Group was placed at a single-child \
                     position; see Element::Group docs."
                );
            }
            (Element::Empty, Element::Empty) => return None,
            _ => {}
        }

        let (Some(ow), Some(nw)) = (old.as_widget(), new.as_widget()) else {
            unreachable!("kind_matches guarantees same variant; non-widget variants handled above");
        };
        self.update_widget(ow, nw, id);
        if let (Element::RichTextBlock(o), Element::RichTextBlock(n)) = (old, new)
            && o.paragraphs != n.paragraphs
        {
            self.backend.set_rich_text_paragraphs(id, &n.paragraphs);
        }
        Some(id)
    }

    pub(crate) fn unmount(&mut self, id: ControlId) {
        let mut to_release = Vec::new();
        self.collect_subtree(id, &mut to_release);
        for node in to_release.into_iter().rev() {
            if let Some(mut inst) = self.take_component_instance(node) {
                inst.render_cx.run_cleanups();
            }

            if let Some(state) = self.templated_lists.remove(&node) {
                for row in state.rows.into_iter().flatten() {
                    self.unmount(row.content_id);
                }
            }

            // Unmount header/pane element subtrees that are tracked outside
            // children_mirror (mounted via Widget::header_element / pane_element).
            if let Some(hdr_id) = self.header_elements.remove(&node) {
                self.unmount(hdr_id);
            }
            if let Some(pane_id) = self.pane_elements.remove(&node) {
                self.unmount(pane_id);
            }

            self.selection_callbacks.remove(&node);

            // Let the control tear down resources bound to it (e.g. join a
            // render thread) while the native control still exists, before it
            // is destroyed. The callback always runs when registered; the
            // native element is passed when available, else `None`.
            if let Some(cb) = self.unmount_callbacks.remove(&node) {
                cb.invoke(self.backend.get_native_element(node));
            }

            self.error_boundary_fallbacks.remove(&node);

            if let Some(handle) = self.custom_handles.remove(&node) {
                handle.before_destroy(node, &mut self.backend);
            }

            self.children_mirror.remove(&node);
            self.id_kinds.remove(&node);
            self.backend.destroy(node);
        }
    }

    fn collect_subtree(&self, id: ControlId, out: &mut Vec<ControlId>) {
        let mut stack = vec![id];
        while let Some(node) = stack.pop() {
            out.push(node);
            if let Some(children) = self.children_mirror.get(&node) {
                for child in children.iter().rev() {
                    stack.push(*child);
                }
            }
        }
    }

    pub(crate) fn append_child_tracked(&mut self, parent: ControlId, child: ControlId) {
        self.children_mirror.entry(parent).or_default().push(child);
        self.backend.append_child(parent, child);
    }

    pub(crate) fn remove_child_tracked(&mut self, parent: ControlId, index: usize) {
        if let Some(list) = self.children_mirror.get_mut(&parent)
            && index < list.len()
        {
            list.remove(index);
        }
        self.backend.remove_child(parent, index);
    }

    pub(crate) fn replace_child_tracked(
        &mut self,
        parent: ControlId,
        index: usize,
        new: ControlId,
    ) {
        if let Some(list) = self.children_mirror.get_mut(&parent)
            && index < list.len()
        {
            list[index] = new;
        }
        self.backend.replace_child(parent, index, new);
    }

    pub(crate) fn move_child_tracked(&mut self, parent: ControlId, from: usize, to: usize) {
        if from == to {
            return;
        }
        if let Some(list) = self.children_mirror.get_mut(&parent)
            && from < list.len()
            && to < list.len()
        {
            let item = list.remove(from);
            list.insert(to, item);
        }
        self.backend.move_child(parent, from, to);
    }

    pub(crate) fn insert_child_tracked(
        &mut self,
        parent: ControlId,
        index: usize,
        child: ControlId,
    ) {
        let list = self.children_mirror.entry(parent).or_default();
        let clamped = index.min(list.len());
        list.insert(clamped, child);
        self.backend.insert_child(parent, clamped, child);
    }

    pub(crate) fn child_at(&self, parent: ControlId, i: usize) -> Option<ControlId> {
        self.children_mirror
            .get(&parent)
            .and_then(|v| v.get(i).copied())
    }

    pub(crate) fn apply_modifiers(&mut self, id: ControlId, mods: &Modifiers) {
        if mods.is_empty() {
            return;
        }
        if let Some(v) = mods.margin {
            self.backend
                .set_prop(id, Prop::Margin, &PropValue::Thickness(v));
        }
        if let Some(v) = mods.padding {
            self.backend
                .set_prop(id, Prop::Padding, &PropValue::Thickness(v));
        }
        if let Some(v) = mods.width {
            self.backend.set_prop(id, Prop::Width, &PropValue::F64(v));
        }
        if let Some(v) = mods.height {
            self.backend.set_prop(id, Prop::Height, &PropValue::F64(v));
        }
        if let Some(v) = mods.min_width {
            self.backend
                .set_prop(id, Prop::MinWidth, &PropValue::F64(v));
        }
        if let Some(v) = mods.max_width {
            self.backend
                .set_prop(id, Prop::MaxWidth, &PropValue::F64(v));
        }
        if let Some(v) = mods.min_height {
            self.backend
                .set_prop(id, Prop::MinHeight, &PropValue::F64(v));
        }
        if let Some(v) = mods.max_height {
            self.backend
                .set_prop(id, Prop::MaxHeight, &PropValue::F64(v));
        }
        if let Some(v) = mods.horizontal_alignment {
            self.backend
                .set_prop(id, Prop::HorizontalAlignment, &PropValue::I32(v.0));
        }
        if let Some(v) = mods.vertical_alignment {
            self.backend
                .set_prop(id, Prop::VerticalAlignment, &PropValue::I32(v.0));
        }
        if let Some(v) = mods.opacity {
            self.backend.set_prop(id, Prop::Opacity, &PropValue::F64(v));
        }
        if let Some(v) = &mods.background {
            self.backend
                .set_prop(id, Prop::Background, &PropValue::Brush(v.clone()));
        }
        if let Some(v) = &mods.foreground {
            self.backend
                .set_prop(id, Prop::Foreground, &PropValue::Brush(v.clone()));
        }
        if let Some(v) = &mods.font_family {
            self.backend
                .set_prop(id, Prop::FontFamily, &PropValue::Str(v.clone()));
        }
        if let Some(v) = mods.font_size {
            self.backend
                .set_prop(id, Prop::FontSize, &PropValue::F64(v));
        }

        self.apply_theme_bindings_for(id, mods);
        self.apply_animations_for(id, mods);
        self.apply_accessibility_for(id, mods);
        self.apply_keyboard_accelerators_for(id, mods);
        self.apply_tooltip_for(id, mods);
        self.apply_pointer_handlers_for(id, mods);

        if let Some(p) = mods.grid {
            self.apply_grid_placement(id, p);
        }

        if let Some(res) = &mods.resources
            && !res.is_empty()
        {
            self.backend
                .set_prop(id, Prop::Resources, &PropValue::Resources((**res).clone()));
        }
    }

    pub(crate) fn apply_tooltip_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(tt) = mods.tooltip.as_deref() else {
            return;
        };
        self.backend.set_tooltip(id, Some(tt));
    }

    pub(crate) fn apply_pointer_handlers_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(ph) = mods.pointer_handlers.as_deref() else {
            return;
        };
        if ph.is_empty() {
            return;
        }
        self.backend.set_pointer_handlers(id, Some(ph));
    }

    pub(crate) fn apply_accessibility_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(acc) = mods.accessibility.as_deref() else {
            return;
        };
        if acc.is_empty() {
            return;
        }
        self.backend.set_accessibility(id, acc);
    }

    pub(crate) fn apply_keyboard_accelerators_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(list) = mods.keyboard_accelerators.as_deref() else {
            return;
        };
        if list.is_empty() {
            return;
        }
        self.backend.set_keyboard_accelerators(id, list);
    }

    pub(crate) fn apply_animations_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(anim) = mods.animations.as_deref() else {
            return;
        };
        if anim.is_empty() {
            return;
        }
        if let Some(it) = anim.implicit_transitions
            && !it.is_empty()
        {
            self.backend.set_implicit_transitions(id, Some(it));
        }
        if let Some(la) = anim.layout_animation {
            self.backend.set_layout_animation(id, Some(la));
        }

        if let Some(p) = anim.property_animation {
            self.backend.run_property_animation(id, Some(p));
        } else if let Some(enter) = anim.enter_transition {
            self.backend.run_property_animation(id, Some(enter));
        }
    }

    pub(crate) fn diff_animations_for(
        &mut self,
        id: ControlId,
        old: Option<&AnimationModifiers>,
        new: Option<&AnimationModifiers>,
    ) {
        let old_it = old.and_then(|a| a.implicit_transitions);
        let new_it = new.and_then(|a| a.implicit_transitions);
        if old_it != new_it {
            self.backend
                .set_implicit_transitions(id, new_it.filter(|t| !t.is_empty()));
        }
        let old_la = old.and_then(|a| a.layout_animation);
        let new_la = new.and_then(|a| a.layout_animation);
        if old_la != new_la {
            self.backend.set_layout_animation(id, new_la);
        }

        let old_pa = old.and_then(|a| a.property_animation);
        let new_pa = new.and_then(|a| a.property_animation);
        if old_pa != new_pa {
            self.backend.run_property_animation(id, new_pa);
        }
    }

    pub(crate) fn apply_theme_bindings_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(map) = mods.theme_bindings.as_deref() else {
            return;
        };
        if map.is_empty() {
            return;
        }
        let kind = match self.id_kinds.get(&id) {
            Some(k) => *k,
            None => return,
        };
        let bindings: Vec<(Prop, ThemeRef)> = map.iter().map(|(p, t)| (*p, t.clone())).collect();
        self.backend.set_theme_bindings(id, kind, &bindings);
    }

    pub(crate) fn diff_modifiers(&mut self, id: ControlId, old: &Modifiers, new: &Modifiers) {
        self.diff_opt_copy(
            id,
            Prop::Margin,
            old.margin,
            new.margin,
            PropValue::Thickness,
        );
        self.diff_opt_copy(
            id,
            Prop::Padding,
            old.padding,
            new.padding,
            PropValue::Thickness,
        );
        self.diff_opt_f64(id, Prop::Width, old.width, new.width);
        self.diff_opt_f64(id, Prop::Height, old.height, new.height);
        self.diff_opt_f64(id, Prop::MinWidth, old.min_width, new.min_width);
        self.diff_opt_f64(id, Prop::MaxWidth, old.max_width, new.max_width);
        self.diff_opt_f64(id, Prop::MinHeight, old.min_height, new.min_height);
        self.diff_opt_f64(id, Prop::MaxHeight, old.max_height, new.max_height);
        self.diff_opt_copy(
            id,
            Prop::HorizontalAlignment,
            old.horizontal_alignment,
            new.horizontal_alignment,
            |v: HorizontalAlignment| PropValue::I32(v.0),
        );
        self.diff_opt_copy(
            id,
            Prop::VerticalAlignment,
            old.vertical_alignment,
            new.vertical_alignment,
            |v: VerticalAlignment| PropValue::I32(v.0),
        );
        self.diff_opt_f64(id, Prop::Opacity, old.opacity, new.opacity);
        self.diff_opt_clone(
            id,
            Prop::Background,
            &old.background,
            &new.background,
            PropValue::Brush,
        );
        self.diff_opt_clone(
            id,
            Prop::Foreground,
            &old.foreground,
            &new.foreground,
            PropValue::Brush,
        );
        self.diff_opt_clone(
            id,
            Prop::FontFamily,
            &old.font_family,
            &new.font_family,
            PropValue::Str,
        );
        self.diff_opt_f64(id, Prop::FontSize, old.font_size, new.font_size);

        if old.theme_bindings != new.theme_bindings {
            let kind = self.id_kinds.get(&id).copied();
            if let Some(kind) = kind {
                let bindings: Vec<(Prop, ThemeRef)> = new
                    .theme_bindings
                    .as_deref()
                    .map(|m| m.iter().map(|(p, t)| (*p, t.clone())).collect())
                    .unwrap_or_default();
                self.backend.set_theme_bindings(id, kind, &bindings);
            }
        }

        let old_anim = old.animations.as_deref();
        let new_anim = new.animations.as_deref();
        if old_anim != new_anim {
            self.diff_animations_for(id, old_anim, new_anim);
        }

        let old_acc = old.accessibility.as_deref();
        let new_acc = new.accessibility.as_deref();
        if old_acc != new_acc {
            static EMPTY: AccessibilityModifiers = AccessibilityModifiers {
                automation_name: None,
                automation_id: None,
                help_text: None,
                live_setting: None,
                heading_level: None,
            };
            let new_acc = new_acc.unwrap_or(&EMPTY);
            self.backend.set_accessibility(id, new_acc);
        }

        let old_ka = old.keyboard_accelerators.as_deref();
        let new_ka = new.keyboard_accelerators.as_deref();
        if old_ka != new_ka {
            let new_ka = new_ka.map_or(&[][..], |v| v.as_slice());
            self.backend.set_keyboard_accelerators(id, new_ka);
        }

        // Tooltip is a `ToolTipService` attached property and survives
        // re-renders; emit a `None` clear on Some→None transitions.
        let old_tt = old.tooltip.as_deref();
        let new_tt = new.tooltip.as_deref();
        if old_tt != new_tt {
            self.backend.set_tooltip(id, new_tt);
        }

        // Pointer / tap handlers: clear on Some→None so we don't leak
        // previously-attached event tokens.
        let old_ph = old.pointer_handlers.as_deref();
        let new_ph = new.pointer_handlers.as_deref();
        if old_ph != new_ph {
            let new_ph = new_ph.filter(|p| !p.is_empty());
            self.backend.set_pointer_handlers(id, new_ph);
        }

        // Grid placement — fast path (avoids AttachedProps dynamic dispatch).
        // Always emit all four props on change so stale values are cleared.
        if old.grid != new.grid {
            self.apply_grid_placement_full(id, new.grid.unwrap_or_default());
        }

        if old.resources != new.resources
            && let Some(res) = &new.resources
            && !res.is_empty()
        {
            self.backend
                .set_prop(id, Prop::Resources, &PropValue::Resources((**res).clone()));
        }
    }

    pub(crate) fn collect_affected_components(
        &self,
        root_id: ControlId,
        changed: &rustc_hash::FxHashSet<ContextId>,
    ) -> Vec<ControlId> {
        let mut result = Vec::new();
        let mut stack = vec![root_id];
        while let Some(id) = stack.pop() {
            if let Some(inst) = self.component_instances.get(&id)
                && inst.read_contexts.iter().any(|c| changed.contains(c))
            {
                result.push(id);
            }
            if let Some(kids) = self.children_mirror.get(&id) {
                for k in kids {
                    stack.push(*k);
                }
            }
        }
        result
    }

    pub fn notify_theme_changed(&mut self) {
        self.backend.on_theme_changed();
    }

    pub(crate) fn reconcile_children_positional(
        &mut self,
        parent: ControlId,
        old: &[Element],
        new: &[Element],
    ) {
        let old_live = LiveChildren::from_slice(old);
        let new_live = LiveChildren::from_slice(new);
        child::reconcile_positional(self, parent, old_live.as_ref(), new_live.as_ref());
    }

    pub(crate) fn reconcile_children(
        &mut self,
        parent: ControlId,
        old: &[Element],
        new: &[Element],
    ) {
        let old_live = LiveChildren::from_slice(old);
        let new_live = LiveChildren::from_slice(new);
        child::reconcile(self, parent, old_live.as_ref(), new_live.as_ref());
    }

    pub(crate) fn set_inner_size_cell(&mut self, cell: Rc<Cell<Size>>) {
        self.inner_size = cell;
    }

    pub(crate) fn set_dpi_cell(&mut self, cell: Rc<Cell<u32>>) {
        self.dpi = cell;
    }

    pub(crate) fn force_context_subscribers(
        &mut self,
        root_id: ControlId,
        context_ids: &rustc_hash::FxHashSet<ContextId>,
    ) {
        let affected = self.collect_affected_components(root_id, context_ids);
        if !affected.is_empty() {
            self.force_component_rerender = true;
            self.forced_components.extend(affected);
        }
    }

    pub(crate) fn clear_forced_component_rerender(&mut self) {
        self.force_component_rerender = false;
        self.forced_components.clear();
    }
}

/// A cow-like container that avoids allocating a `Vec<&Element>` when the
/// input slice has no `Empty` or `Group` elements (the common fast-path for
/// grids and most stack panels).
enum LiveChildren<'a> {
    /// All elements are live — borrow the original slice directly.
    Flat(&'a [Element]),
    /// Contains Empty/Group elements that need filtering.
    Filtered(Vec<&'a Element>),
}

impl<'a> LiveChildren<'a> {
    fn from_slice(slice: &'a [Element]) -> Self {
        let needs_filter = slice
            .iter()
            .any(|e| matches!(e, Element::Empty | Element::Group(_)));
        if needs_filter {
            LiveChildren::Filtered(collect_live(slice))
        } else {
            LiveChildren::Flat(slice)
        }
    }

    fn as_ref(&self) -> LiveChildrenRef<'_> {
        match self {
            LiveChildren::Flat(s) => LiveChildrenRef::Flat(s),
            LiveChildren::Filtered(v) => LiveChildrenRef::Filtered(v),
        }
    }
}

/// Borrowable view into either flat or filtered children, providing
/// slice-like access via `len`, `get`, and `iter`.
pub(crate) enum LiveChildrenRef<'a> {
    Flat(&'a [Element]),
    Filtered(&'a [&'a Element]),
}

impl<'a> LiveChildrenRef<'a> {
    pub(crate) fn len(&self) -> usize {
        match self {
            LiveChildrenRef::Flat(s) => s.len(),
            LiveChildrenRef::Filtered(v) => v.len(),
        }
    }

    pub(crate) fn get(&self, i: usize) -> Option<&'a Element> {
        match self {
            LiveChildrenRef::Flat(s) => s.get(i),
            LiveChildrenRef::Filtered(v) => v.get(i).copied(),
        }
    }

    pub(crate) fn any_has_key(&self) -> bool {
        match self {
            LiveChildrenRef::Flat(s) => s.iter().any(|e| e.key().is_some()),
            LiveChildrenRef::Filtered(v) => v.iter().any(|e| e.key().is_some()),
        }
    }
}

/// Flatten a child slice for reconciliation: drops `Element::Empty` and
/// recursively splices `Element::Group` children into the output.
pub(crate) fn collect_live(slice: &[Element]) -> Vec<&Element> {
    let mut out = Vec::with_capacity(slice.len());
    for el in slice {
        push_live(el, &mut out);
    }
    out
}

fn push_live<'a>(el: &'a Element, out: &mut Vec<&'a Element>) {
    match el {
        Element::Empty => {}
        Element::Group(g) => {
            for child in &g.children {
                push_live(child, out);
            }
        }
        other => out.push(other),
    }
}

#[cfg(test)]
mod lifecycle_tests {
    use super::*;
    use crate::callback::Callback;
    use crate::ir::{
        Backend, ControlKind, Event, EventHandler, Op, Prop, PropValue, RecordingBackend,
    };
    use std::cell::RefCell;

    /// Each entry records a callback invocation as `(tag, native_present)`,
    /// where `native_present` is whether a native element was passed.
    type Log = Rc<RefCell<Vec<(&'static str, bool)>>>;

    /// Minimal in-crate [`Widget`] used to exercise the general mount/unmount
    /// callback mechanism independently of any concrete widget's None policy.
    /// It forwards the raw `Option<IInspectable>` straight into the log.
    struct ProbeWidget {
        modifiers: Modifiers,
        mounted: Option<Callback<Option<windows_core::IInspectable>>>,
        unmounted: Option<Callback<Option<windows_core::IInspectable>>>,
    }

    impl ProbeWidget {
        fn new(log: &Log, with_mount: bool, with_unmount: bool) -> Self {
            let mk = |tag: &'static str| {
                let log = log.clone();
                Callback::new(move |native: Option<windows_core::IInspectable>| {
                    log.borrow_mut().push((tag, native.is_some()));
                })
            };
            Self {
                modifiers: Modifiers::default(),
                mounted: with_mount.then(|| mk("mount")),
                unmounted: with_unmount.then(|| mk("unmount")),
            }
        }
    }

    impl Widget for ProbeWidget {
        fn kind(&self) -> ControlKind {
            ControlKind::Border
        }
        fn key(&self) -> Option<&str> {
            None
        }
        fn modifiers(&self) -> &Modifiers {
            &self.modifiers
        }
        fn bindings(&self) -> PropBindings {
            Vec::new()
        }
        fn on_mounted_callback(&self) -> Option<&Callback<Option<windows_core::IInspectable>>> {
            self.mounted.as_ref()
        }
        fn on_unmounted_callback(&self) -> Option<&Callback<Option<windows_core::IInspectable>>> {
            self.unmounted.as_ref()
        }
    }

    /// A backend that exposes no native elements (its `get_native_element`
    /// uses the trait default, returning `None`). Models a real backend — or
    /// a control kind — without a native handle, so the general "fire the
    /// callback with `None`" path can be exercised. Only the operations the
    /// lifecycle tests rely on are recorded.
    #[derive(Default)]
    struct NoNativeBackend {
        next_id: u32,
        destroyed: Vec<ControlId>,
    }

    impl Backend for NoNativeBackend {
        fn create(&mut self, _kind: ControlKind) -> ControlId {
            self.next_id += 1;
            ControlId::new(self.next_id)
        }
        fn set_prop(&mut self, _id: ControlId, _prop: Prop, _value: &PropValue) {}
        fn append_child(&mut self, _parent: ControlId, _child: ControlId) {}
        fn remove_child(&mut self, _parent: ControlId, _index: usize) {}
        fn replace_child(&mut self, _parent: ControlId, _index: usize, _new: ControlId) {}
        fn move_child(&mut self, _parent: ControlId, _from: usize, _to: usize) {}
        fn insert_child(&mut self, _parent: ControlId, _index: usize, _child: ControlId) {}
        fn destroy(&mut self, id: ControlId) {
            self.destroyed.push(id);
        }
        fn attach_event(&mut self, _id: ControlId, _event: Event, _handler: EventHandler) {}
    }

    fn reconciler() -> Reconciler<RecordingBackend> {
        Reconciler::new(RecordingBackend::new())
    }

    fn destroyed(r: &Reconciler<RecordingBackend>, id: ControlId) -> bool {
        r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Destroy { id: did } if *did == id))
    }

    #[test]
    fn mount_callback_fires_once_with_native_when_available() {
        let log: Log = Rc::default();
        let mut r = reconciler();
        let probe = ProbeWidget::new(&log, true, false);
        r.mount_widget(&probe);
        assert_eq!(*log.borrow(), vec![("mount", true)]);
    }

    #[test]
    fn mount_callback_fires_once_with_none_when_no_native() {
        // The callback still fires even when the backend exposes no native
        // element — it simply receives `None`.
        let log: Log = Rc::default();
        let mut r = Reconciler::new(NoNativeBackend::default());
        let probe = ProbeWidget::new(&log, true, false);
        r.mount_widget(&probe);
        assert_eq!(*log.borrow(), vec![("mount", false)]);
    }

    #[test]
    fn unmount_callback_fires_before_destroy_with_native() {
        // The backend drops a control's native element in `destroy`, so the
        // callback receiving a `Some` native element proves it ran *before*
        // the destroy for that node.
        let log: Log = Rc::default();
        let mut r = reconciler();
        let probe = ProbeWidget::new(&log, false, true);
        let id = r.mount_widget(&probe);
        assert!(log.borrow().is_empty(), "must not fire before unmount");

        r.unmount(id);
        assert_eq!(*log.borrow(), vec![("unmount", true)]);
        assert!(destroyed(&r, id), "control was destroyed");
    }

    #[test]
    fn unmount_callback_fires_with_none_when_no_native() {
        // Teardown must run regardless of native-element presence; here it
        // receives `None`.
        let log: Log = Rc::default();
        let mut r = Reconciler::new(NoNativeBackend::default());
        let probe = ProbeWidget::new(&log, false, true);
        let id = r.mount_widget(&probe);
        r.unmount(id);
        assert_eq!(*log.borrow(), vec![("unmount", false)]);
        assert!(r.backend.destroyed.contains(&id), "control was destroyed");
    }

    #[test]
    fn mount_and_unmount_fire_across_lifecycle() {
        let log: Log = Rc::default();
        let mut r = reconciler();
        let probe = ProbeWidget::new(&log, true, true);
        let id = r.mount_widget(&probe);
        assert_eq!(*log.borrow(), vec![("mount", true)]);
        r.unmount(id);
        assert_eq!(*log.borrow(), vec![("mount", true), ("unmount", true)]);
    }

    #[test]
    fn update_replaces_then_removes_unmount_callback() {
        let log: Log = Rc::default();
        let mut r = reconciler();

        // Mount with an unmount callback registered.
        let first = ProbeWidget::new(&log, false, true);
        let id = r.mount_widget(&first);

        // Update with a fresh unmount callback — replaces the registered one.
        let second_log: Log = Rc::default();
        let second = ProbeWidget::new(&second_log, false, true);
        r.update_widget(&first, &second, id);

        // Update again without an unmount callback — clears the registration.
        let third = ProbeWidget::new(&log, false, false);
        r.update_widget(&second, &third, id);

        r.unmount(id);
        assert!(
            log.borrow().is_empty(),
            "neither the original nor a cleared callback fires"
        );
        assert!(
            second_log.borrow().is_empty(),
            "the replaced callback was cleared before teardown"
        );
    }
}
