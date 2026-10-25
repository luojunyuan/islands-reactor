mod attachments;
mod component;
mod declarations;
mod dialogs;
mod events;
mod failures;
mod input;
mod reconcile;
mod selection;
mod virtualization;

#[track_caller]
fn assert_panics(expected: &str, f: impl FnOnce()) {
    let payload = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).unwrap_err();
    let message = payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&str>().copied())
        .unwrap();
    assert!(
        message.contains(expected),
        "panic {message:?} does not contain {expected:?}"
    );
}
