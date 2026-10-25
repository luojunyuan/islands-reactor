use std::rc::Rc;

use crate::{Reconciler, test_support::RecordingBackend};

pub(crate) fn reconciler() -> Reconciler<RecordingBackend> {
    Reconciler::new(RecordingBackend::new())
}

pub(crate) fn noop_rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}
