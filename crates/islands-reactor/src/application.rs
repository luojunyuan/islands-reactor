use super::*;

pub(crate) mod dispatcher {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum DispatchPriority {
        Normal,
        Low,
    }

    /// Schedules deferred work onto a render-aware queue.
    pub trait Dispatcher {
        fn enqueue(&self, priority: DispatchPriority, f: Box<dyn FnOnce()>) -> bool;
    }

    /// Thread-safe variant of [`Dispatcher`] accepting `Send` closures.
    /// Implementations marshal the closure onto the UI thread.
    pub trait SendDispatcher: Send + Sync + 'static {
        fn enqueue_send(
            &self,
            priority: DispatchPriority,
            f: Box<dyn FnOnce() + Send + 'static>,
        ) -> bool;
    }

    thread_local! {
        // UI thread's rerender hook, installed by `RenderHost::set_marshaller`.
        // Single-host-per-thread; replace with a per-host registry if
        // multi-host-per-thread is added.
        static UI_RERENDER: RefCell<Option<Rc<dyn Fn()>>> = const { RefCell::new(None) };
    }

    /// Install (or clear) the UI thread's rerender hook.
    #[doc(hidden)]
    pub fn set_ui_rerender(rerender: Option<Rc<dyn Fn()>>) {
        UI_RERENDER.with(|r| {
            *r.borrow_mut() = rerender;
        });
    }

    /// Request a rerender on the UI thread the marshaller targets.
    #[doc(hidden)]
    pub fn request_ui_rerender_on_ui_thread() {
        UI_RERENDER.with(|r| {
            if let Some(rr) = r.borrow().as_ref() {
                rr();
            }
        });
    }

    /// RAII guard around [`set_ui_rerender`]; clears the thread-local on drop.
    #[doc(hidden)]
    #[must_use = "the guard restores UI_RERENDER on drop; binding it to `_` drops it immediately"]
    pub struct UiRerenderGuard {
        _not_send: std::marker::PhantomData<*const ()>,
    }

    impl UiRerenderGuard {
        pub fn install(rerender: Rc<dyn Fn()>) -> Self {
            set_ui_rerender(Some(rerender));
            Self {
                _not_send: std::marker::PhantomData,
            }
        }
    }

    impl Drop for UiRerenderGuard {
        fn drop(&mut self) {
            set_ui_rerender(None);
        }
    }

    /// Thread-safe, clonable handle to the UI thread's render-aware
    /// dispatcher. Used by `AsyncSetState` to marshal state writes back
    /// onto the UI thread.
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct UiMarshaller {
        inner: Arc<dyn SendDispatcher>,
    }

    impl UiMarshaller {
        pub fn new(inner: Arc<dyn SendDispatcher>) -> Self {
            Self { inner }
        }

        /// Schedule `f` to run on the UI thread at normal priority.
        pub fn dispatch<F>(&self, f: F) -> bool
        where
            F: FnOnce() + Send + 'static,
        {
            self.inner
                .enqueue_send(DispatchPriority::Normal, Box::new(f))
        }

        /// Schedule `f` to run on the UI thread at low priority.
        pub fn dispatch_low<F>(&self, f: F) -> bool
        where
            F: FnOnce() + Send + 'static,
        {
            self.inner.enqueue_send(DispatchPriority::Low, Box::new(f))
        }
    }

    impl std::fmt::Debug for UiMarshaller {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("UiMarshaller").finish_non_exhaustive()
        }
    }

    /// In-memory [`SendDispatcher`] used by tests. Closures are queued in a
    /// mutex-guarded `VecDeque`; call [`Self::drain`] to run them.
    #[doc(hidden)]
    #[derive(Default)]
    pub struct ChannelDispatcher {
        inner: Arc<ChannelDispatcherInner>,
    }

    #[derive(Default)]
    struct ChannelDispatcherInner {
        normal: Mutex<VecDeque<Box<dyn FnOnce() + Send>>>,
        low: Mutex<VecDeque<Box<dyn FnOnce() + Send>>>,
    }

    impl ChannelDispatcher {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn pending(&self) -> usize {
            self.inner.normal.lock().unwrap().len() + self.inner.low.lock().unwrap().len()
        }

        pub fn drain(&self) -> usize {
            let mut fired = 0;
            loop {
                let item = {
                    let mut n = self.inner.normal.lock().unwrap();
                    if let Some(f) = n.pop_front() {
                        Some(f)
                    } else {
                        drop(n);
                        self.inner.low.lock().unwrap().pop_front()
                    }
                };
                match item {
                    Some(f) => {
                        f();
                        fired += 1;
                    }
                    None => break,
                }
            }
            fired
        }

        pub fn handle(&self) -> Arc<dyn SendDispatcher> {
            Arc::clone(&self.inner) as Arc<dyn SendDispatcher>
        }

        pub fn marshaller(&self) -> UiMarshaller {
            UiMarshaller::new(self.handle())
        }
    }

    impl SendDispatcher for ChannelDispatcherInner {
        fn enqueue_send(
            &self,
            priority: DispatchPriority,
            f: Box<dyn FnOnce() + Send + 'static>,
        ) -> bool {
            match priority {
                DispatchPriority::Normal => self.normal.lock().unwrap().push_back(f),
                DispatchPriority::Low => self.low.lock().unwrap().push_back(f),
            }
            true
        }
    }

    /// In-process [`Dispatcher`] that buffers work until [`drain`](Self::drain)
    /// is called. Used by tests and by `Application::run_once`.
    #[doc(hidden)]
    #[derive(Default)]
    pub struct RunOnDemandDispatcher {
        inner: Rc<DispatcherQueue>,
    }

    #[derive(Default)]
    struct DispatcherQueue {
        normal: RefCell<VecDeque<Box<dyn FnOnce()>>>,
        low: RefCell<VecDeque<Box<dyn FnOnce()>>>,
    }

    impl RunOnDemandDispatcher {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn pending(&self) -> usize {
            self.inner.normal.borrow().len() + self.inner.low.borrow().len()
        }

        pub fn drain(&self) -> usize {
            let mut fired = 0;
            loop {
                let item = {
                    let mut n = self.inner.normal.borrow_mut();
                    if let Some(f) = n.pop_front() {
                        Some(f)
                    } else {
                        drop(n);
                        self.inner.low.borrow_mut().pop_front()
                    }
                };
                match item {
                    Some(f) => {
                        f();
                        fired += 1;
                    }
                    None => break,
                }
            }
            fired
        }
    }

    impl Dispatcher for RunOnDemandDispatcher {
        fn enqueue(&self, priority: DispatchPriority, f: Box<dyn FnOnce()>) -> bool {
            match priority {
                DispatchPriority::Normal => self.inner.normal.borrow_mut().push_back(f),
                DispatchPriority::Low => self.inner.low.borrow_mut().push_back(f),
            }
            true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cell::RefCell;
        use std::rc::Rc;

        fn job(log: Rc<RefCell<Vec<&'static str>>>, tag: &'static str) -> Box<dyn FnOnce()> {
            Box::new(move || log.borrow_mut().push(tag))
        }

        // FIFO/priority tests for `RunOnDemandDispatcher` live in
        // `crates/tests/reactor/tests/dispatcher.rs`; this one stays here
        // because it pokes the private `inner` field directly.

        #[test]
        fn run_on_demand_dispatcher_promotes_normal_followup_over_remaining_low() {
            let dispatcher = RunOnDemandDispatcher::new();
            let log: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));

            let log_for_lo1 = Rc::clone(&log);
            let dispatcher_clone = dispatcher.inner.clone();
            let lo1: Box<dyn FnOnce()> = Box::new(move || {
                log_for_lo1.borrow_mut().push("lo1");

                let log2 = Rc::clone(&log_for_lo1);
                dispatcher_clone
                    .normal
                    .borrow_mut()
                    .push_back(Box::new(move || log2.borrow_mut().push("n_followup")));
            });

            dispatcher.enqueue(DispatchPriority::Low, lo1);
            dispatcher.enqueue(DispatchPriority::Low, job(Rc::clone(&log), "lo2"));

            dispatcher.drain();
            assert_eq!(*log.borrow(), vec!["lo1", "n_followup", "lo2"]);
        }
    }
}
pub(crate) mod render_host {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::time::Instant;

    use super::dispatcher::set_ui_rerender;
    use super::*;

    /// Per-render telemetry exposed by [`RenderHost::stats`].
    #[doc(hidden)]
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RenderStats {
        pub fps: f64,
        pub renders_in_window: u32,
        pub total_renders: u64,
        pub avg_tree_build_ms: f64,
        pub avg_reconcile_ms: f64,
        pub avg_effects_ms: f64,
        pub last_diffed: u64,
        pub last_skipped: u64,
        pub last_created: u64,
    }

    struct StatsAccumulator {
        window_start: Cell<Instant>,
        tree_build_sum_ms: Cell<f64>,
        reconcile_sum_ms: Cell<f64>,
        effects_sum_ms: Cell<f64>,
        renders_in_window: Cell<u32>,
        total_renders: Cell<u64>,
    }

    impl StatsAccumulator {
        fn new() -> Self {
            Self {
                window_start: Cell::new(Instant::now()),
                tree_build_sum_ms: Cell::new(0.0),
                reconcile_sum_ms: Cell::new(0.0),
                effects_sum_ms: Cell::new(0.0),
                renders_in_window: Cell::new(0),
                total_renders: Cell::new(0),
            }
        }

        #[allow(clippy::too_many_arguments)]
        fn record(
            &self,
            prev: RenderStats,
            tree_build_ms: f64,
            reconcile_ms: f64,
            effects_ms: f64,
            last_diffed: u64,
            last_skipped: u64,
            last_created: u64,
        ) -> RenderStats {
            self.tree_build_sum_ms
                .set(self.tree_build_sum_ms.get() + tree_build_ms);
            self.reconcile_sum_ms
                .set(self.reconcile_sum_ms.get() + reconcile_ms);
            self.effects_sum_ms
                .set(self.effects_sum_ms.get() + effects_ms);
            self.renders_in_window.set(self.renders_in_window.get() + 1);
            self.total_renders.set(self.total_renders.get() + 1);

            let window = self.window_start.get().elapsed();
            if window.as_secs_f64() >= 1.0 {
                let renders = self.renders_in_window.get();
                let fps = renders as f64 / window.as_secs_f64().max(f64::EPSILON);
                let avg = |sum: f64| {
                    if renders > 0 {
                        sum / renders as f64
                    } else {
                        0.0
                    }
                };

                let snapshot = RenderStats {
                    fps,
                    renders_in_window: renders,
                    total_renders: self.total_renders.get(),
                    avg_tree_build_ms: avg(self.tree_build_sum_ms.get()),
                    avg_reconcile_ms: avg(self.reconcile_sum_ms.get()),
                    avg_effects_ms: avg(self.effects_sum_ms.get()),
                    last_diffed,
                    last_skipped,
                    last_created,
                };

                self.tree_build_sum_ms.set(0.0);
                self.reconcile_sum_ms.set(0.0);
                self.effects_sum_ms.set(0.0);
                self.renders_in_window.set(0);
                self.window_start.set(Instant::now());

                snapshot
            } else {
                RenderStats {
                    last_diffed,
                    last_skipped,
                    last_created,
                    total_renders: self.total_renders.get(),
                    ..prev
                }
            }
        }
    }

    /// Owns a [`Reconciler`] and a root [`Component`], coalescing render
    /// requests onto the supplied [`Dispatcher`] (typically the UI thread).
    #[doc(hidden)]
    pub struct RenderHost<B: Backend, D: Dispatcher> {
        inner: Rc<RenderHostInner<B, D>>,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum RenderState {
        Idle,
        Pending,
        Rendering,
        RenderingDirty,
    }

    struct RenderHostInner<B: Backend, D: Dispatcher> {
        reconciler: RefCell<Reconciler<B>>,
        root: RefCell<Box<dyn Component>>,
        render_cx: RefCell<RenderCx>,
        root_id: Cell<Option<ControlId>>,
        last_tree: RefCell<Option<Element>>,
        render_state: Cell<RenderState>,
        dispatcher: D,
        /// Optional `Send + Sync` marshaller used to support
        /// [`RenderCx::use_async_state`]. When `None`, async setters
        /// will panic if any component asks for one.
        marshaller: RefCell<Option<UiMarshaller>>,
        render_count: Cell<u32>,
        stats: Cell<RenderStats>,
        post_render: RefCell<Option<Box<dyn Fn(Option<ControlId>)>>>,
        render_complete: RefCell<Option<Box<dyn Fn(f64, f64, f64)>>>,
        stats_accum: StatsAccumulator,
        inner_size: Rc<Cell<Size>>,
        dpi: Rc<Cell<u32>>,
    }

    impl<B: Backend + 'static, D: Dispatcher + 'static> RenderHost<B, D> {
        pub fn new(backend: B, root: Box<dyn Component>, dispatcher: D) -> Self {
            let mut reconciler = Reconciler::new(backend);
            let mut render_cx = RenderCx::new(Rc::new(|| {}));

            let inner_size = Rc::new(Cell::new(Size::default()));
            let dpi = Rc::new(Cell::new(96_u32));
            render_cx.set_inner_size_cell(Rc::clone(&inner_size));
            render_cx.set_dpi_cell(Rc::clone(&dpi));
            reconciler.set_inner_size_cell(Rc::clone(&inner_size));
            reconciler.set_dpi_cell(Rc::clone(&dpi));

            Self {
                inner: Rc::new(RenderHostInner {
                    reconciler: RefCell::new(reconciler),
                    root: RefCell::new(root),
                    render_cx: RefCell::new(render_cx),
                    root_id: Cell::new(None),
                    last_tree: RefCell::new(None),
                    render_state: Cell::new(RenderState::Idle),
                    dispatcher,
                    marshaller: RefCell::new(None),
                    render_count: Cell::new(0),
                    stats: Cell::new(RenderStats::default()),
                    post_render: RefCell::new(None),
                    render_complete: RefCell::new(None),
                    stats_accum: StatsAccumulator::new(),
                    inner_size,
                    dpi,
                }),
            }
        }

        pub fn stats(&self) -> RenderStats {
            self.inner.stats.get()
        }

        /// Install (or replace) the [`UiMarshaller`] used to support
        /// off-UI-thread state writes via [`RenderCx::use_async_state`], and
        /// publish the host's rerender hook to the UI thread's `UI_RERENDER`
        /// slot. Passing `None` clears both.
        pub fn set_marshaller(&self, marshaller: Option<UiMarshaller>) {
            self.inner.marshaller.borrow_mut().clone_from(&marshaller);
            self.inner
                .render_cx
                .borrow_mut()
                .set_marshaller(marshaller.clone());
            self.inner
                .reconciler
                .borrow_mut()
                .set_marshaller(marshaller.clone());

            if marshaller.is_some() {
                let weak = Rc::downgrade(&self.inner);
                let rerender: Rc<dyn Fn()> = Rc::new(move || {
                    if let Some(strong) = weak.upgrade() {
                        request_render(&strong);
                    }
                });
                set_ui_rerender(Some(rerender));
            } else {
                set_ui_rerender(None);
            }
        }

        pub fn kick(&self) {
            let weak = Rc::downgrade(&self.inner);
            let request = Rc::new(move || {
                if let Some(strong) = weak.upgrade() {
                    request_render(&strong);
                }
            });
            self.inner
                .render_cx
                .borrow_mut()
                .set_request_rerender(request);
            request_render(&self.inner);
        }

        pub fn request_render(&self) {
            request_render(&self.inner);
        }

        pub fn render_count(&self) -> u32 {
            self.inner.render_count.get()
        }

        pub fn is_render_pending(&self) -> bool {
            self.inner.render_state.get() != RenderState::Idle
        }

        pub fn is_rendering(&self) -> bool {
            matches!(
                self.inner.render_state.get(),
                RenderState::Rendering | RenderState::RenderingDirty
            )
        }

        pub fn needs_rerender(&self) -> bool {
            matches!(
                self.inner.render_state.get(),
                RenderState::Pending | RenderState::RenderingDirty
            )
        }

        pub fn is_idle(&self) -> bool {
            self.inner.render_state.get() == RenderState::Idle
        }

        pub fn root_id(&self) -> Option<ControlId> {
            self.inner.root_id.get()
        }

        pub fn with_reconciler<R>(&self, f: impl FnOnce(&Reconciler<B>) -> R) -> R {
            f(&self.inner.reconciler.borrow())
        }

        pub fn with_reconciler_mut<R>(&self, f: impl FnOnce(&mut Reconciler<B>) -> R) -> R {
            f(&mut self.inner.reconciler.borrow_mut())
        }

        pub fn clone_inner(&self) -> Self {
            Self {
                inner: Rc::clone(&self.inner),
            }
        }

        pub fn set_post_render<F>(&self, f: F)
        where
            F: Fn(Option<ControlId>) + 'static,
        {
            *self.inner.post_render.borrow_mut() = Some(Box::new(f));
        }

        pub fn set_render_complete<F>(&self, f: F)
        where
            F: Fn(f64, f64, f64) + 'static,
        {
            *self.inner.render_complete.borrow_mut() = Some(Box::new(f));
        }

        pub(crate) fn clear_callbacks(&self) {
            *self.inner.post_render.borrow_mut() = None;
            *self.inner.render_complete.borrow_mut() = None;
        }

        pub fn inner_size(&self) -> Size {
            self.inner.inner_size.get()
        }

        pub fn set_inner_size(&self, size: Size) {
            if self.inner.inner_size.get() == size {
                return;
            }
            self.inner.inner_size.set(size);
            if let Some(root_id) = self.inner.root_id.get() {
                let mut changed = rustc_hash::FxHashSet::default();
                changed.insert(inner_size_context_id());
                self.inner
                    .reconciler
                    .borrow_mut()
                    .force_context_subscribers(root_id, &changed);
            }
            self.request_render();
        }

        pub fn dpi(&self) -> u32 {
            self.inner.dpi.get()
        }

        pub fn set_dpi(&self, dpi: u32) {
            if self.inner.dpi.get() == dpi {
                return;
            }
            self.inner.dpi.set(dpi);
            if let Some(root_id) = self.inner.root_id.get() {
                let mut changed = rustc_hash::FxHashSet::default();
                changed.insert(dpi_context_id());
                self.inner
                    .reconciler
                    .borrow_mut()
                    .force_context_subscribers(root_id, &changed);
            }
            self.request_render();
        }
    }

    fn request_render<B: Backend + 'static, D: Dispatcher + 'static>(
        inner: &Rc<RenderHostInner<B, D>>,
    ) {
        match inner.render_state.get() {
            RenderState::Idle => {
                inner.render_state.set(RenderState::Pending);
                let inner_c = Rc::clone(inner);
                if !inner.dispatcher.enqueue(
                    DispatchPriority::Normal,
                    Box::new(move || render_loop(&inner_c)),
                ) {
                    inner.render_state.set(RenderState::Idle);
                }
            }
            RenderState::Pending | RenderState::RenderingDirty => {}
            RenderState::Rendering => {
                inner.render_state.set(RenderState::RenderingDirty);
            }
        }
    }

    fn render_loop<B: Backend + 'static, D: Dispatcher + 'static>(
        inner: &Rc<RenderHostInner<B, D>>,
    ) {
        inner.render_state.set(RenderState::Rendering);
        render_once(inner);

        if inner.render_state.get() == RenderState::RenderingDirty {
            inner.render_state.set(RenderState::Pending);
            let inner_c = Rc::clone(inner);
            if !inner.dispatcher.enqueue(
                DispatchPriority::Low,
                Box::new(move || render_loop(&inner_c)),
            ) {
                inner.render_state.set(RenderState::Idle);
            }
        } else {
            inner.render_state.set(RenderState::Idle);
        }
    }

    fn render_once<B: Backend + 'static, D: Dispatcher + 'static>(
        inner: &Rc<RenderHostInner<B, D>>,
    ) {
        let tree_build_started = Instant::now();
        let new_tree = {
            let mut cx = inner.render_cx.borrow_mut();
            cx.begin_render();
            let root = inner.root.borrow();
            root.render(&(), &mut cx)
        };
        let tree_build_ms = tree_build_started.elapsed().as_secs_f64() * 1000.0;

        let reconcile_started = Instant::now();
        // Take (not clone) the old tree to avoid deep-cloning every frame.
        let old_tree_owned = inner.last_tree.borrow_mut().take();
        let (new_root_id, last_diffed, last_skipped, last_created) = {
            let mut reconciler = inner.reconciler.borrow_mut();
            reconciler.reset_stats();

            let existing = inner.root_id.get();
            let weak = Rc::downgrade(inner);
            let request_rerender: Rc<dyn Fn()> = Rc::new(move || {
                if let Some(strong) = weak.upgrade() {
                    request_render(&strong);
                }
            });
            let id = reconciler.reconcile(
                old_tree_owned.as_ref(),
                &new_tree,
                existing,
                request_rerender,
            );
            reconciler.drain_realizations();
            reconciler.clear_forced_component_rerender();
            (
                id,
                reconciler.debug_elements_diffed,
                reconciler.debug_elements_skipped,
                reconciler.debug_ui_elements_created,
            )
        };
        let reconcile_ms = reconcile_started.elapsed().as_secs_f64() * 1000.0;
        inner.root_id.set(new_root_id);

        *inner.last_tree.borrow_mut() = Some(new_tree);

        let effects_started = Instant::now();
        inner.render_cx.borrow_mut().flush_effects();
        let effects_ms = effects_started.elapsed().as_secs_f64() * 1000.0;

        inner.render_count.set(inner.render_count.get() + 1);

        let cb_taken = inner.post_render.borrow_mut().take();
        if let Some(cb) = cb_taken {
            cb(inner.root_id.get());

            let mut slot = inner.post_render.borrow_mut();
            if slot.is_none() {
                *slot = Some(cb);
            }
        }

        let prev_stats = inner.stats.get();
        let new_stats = inner.stats_accum.record(
            prev_stats,
            tree_build_ms,
            reconcile_ms,
            effects_ms,
            last_diffed,
            last_skipped,
            last_created,
        );
        inner.stats.set(new_stats);

        let rc_taken = inner.render_complete.borrow_mut().take();
        if let Some(cb) = rc_taken {
            cb(tree_build_ms, reconcile_ms, effects_ms);
            let mut slot = inner.render_complete.borrow_mut();
            if slot.is_none() {
                *slot = Some(cb);
            }
        }
    }
}
