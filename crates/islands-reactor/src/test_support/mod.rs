use std::rc::Rc;

use crate::Reconciler;
pub(crate) use crate::ir::{Op, RecordingBackend};

pub(crate) fn reconciler() -> Reconciler<RecordingBackend> {
    Reconciler::new(RecordingBackend::new())
}

pub(crate) fn noop_rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}
