use std::time::Duration;

use crate::bindings as Xaml;

/// RAII timer wrapper; stops and unhooks on drop.
pub struct DispatcherTimer {
    timer: Xaml::DispatcherTimer,
    _tick_revoker: windows_core::EventRevoker,
}

impl DispatcherTimer {
    pub fn new<F>(interval: Duration, f: F) -> windows_core::Result<Self>
    where
        F: Fn() + 'static,
    {
        Self::build(interval, true, f)
    }

    pub fn new_one_shot<F>(after: Duration, f: F) -> windows_core::Result<Self>
    where
        F: Fn() + 'static,
    {
        Self::build(after, false, f)
    }

    fn build<F>(interval: Duration, repeating: bool, f: F) -> windows_core::Result<Self>
    where
        F: Fn() + 'static,
    {
        let timer = Xaml::DispatcherTimer::new()?;
        timer.put_Interval(duration_to_timespan(interval))?;

        let timer_for_cb = timer.clone();
        let callback = UiThreadCallback(f);
        let tick_revoker = timer.add_Tick(move |_, _| {
            if !repeating {
                let _ = timer_for_cb.Stop();
            }
            callback.call();
        })?;
        timer.Start()?;
        Ok(Self {
            timer,
            _tick_revoker: tick_revoker,
        })
    }

    pub fn stop(&self) -> windows_core::Result<()> {
        self.timer.Stop()
    }

    pub fn start(&self) -> windows_core::Result<()> {
        self.timer.Start()
    }
}

impl Drop for DispatcherTimer {
    fn drop(&mut self) {
        let _ = self.timer.Stop();
    }
}

/// RAII handle for a `CompositionTarget::Rendering` subscription; detaches on drop.
pub struct Rendering {
    _revoker: windows_core::EventRevoker,
}

/// Subscribe `f` to `CompositionTarget::Rendering` for the current thread.
pub fn on_rendering<F>(f: F) -> windows_core::Result<Rendering>
where
    F: Fn() + 'static,
{
    let callback = UiThreadCallback(f);
    let revoker = Xaml::CompositionTarget::add_Rendering(move |_, _| {
        callback.call();
    })?;
    Ok(Rendering { _revoker: revoker })
}

fn duration_to_timespan(d: Duration) -> windows_time::TimeSpan {
    windows_time::TimeSpan::try_from(d).unwrap_or(windows_time::TimeSpan::MAX)
}

struct UiThreadCallback<F>(F);

unsafe impl<F> Send for UiThreadCallback<F> {}

impl<F: Fn()> UiThreadCallback<F> {
    fn call(&self) {
        (self.0)();
    }
}
