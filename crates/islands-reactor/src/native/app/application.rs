use std::cell::RefCell;
use std::panic::AssertUnwindSafe;

use windows::{
    Win32::{
        ro::{RO_INIT_SINGLETHREADED, RoInitialize},
        windef::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
        winuser::SetProcessDpiAwarenessContext,
    },
    core::{Error, HRESULT, Result},
};

use crate::native::winui::*;

const E_FAIL: HRESULT = HRESULT(0x80004005u32 as i32);

thread_local! {
    static HOST_SLOT: RefCell<Option<ReactorHost>> = const { RefCell::new(None) };
}

pub fn with_active_host<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&ReactorHost) -> R,
{
    HOST_SLOT.with(|slot| slot.borrow().as_ref().map(f))
}

fn clear_active_host() {
    HOST_SLOT.with(|slot| {
        *slot.borrow_mut() = None;
    });
}

pub struct App {
    title: Option<String>,
    inner_size: Option<crate::Size>,
    inner_constraints: InnerConstraints,
    eager_templated_realization: bool,
    presenter: PresenterKind,
    backdrop: Option<Backdrop>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            title: None,
            inner_size: None,
            inner_constraints: InnerConstraints::default(),
            eager_templated_realization: false,
            presenter: PresenterKind::Default,
            backdrop: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn inner_size(mut self, width: f64, height: f64) -> Self {
        self.inner_size = Some(crate::Size { width, height });
        self
    }

    pub fn inner_constraints(mut self, constraints: InnerConstraints) -> Self {
        self.inner_constraints = constraints;
        self
    }

    pub fn eager_templated_realization(mut self, on: bool) -> Self {
        self.eager_templated_realization = on;
        self
    }

    pub fn presenter(mut self, kind: PresenterKind) -> Self {
        self.presenter = kind;
        self
    }

    pub fn fullscreen(self, on: bool) -> Self {
        self.presenter(if on {
            PresenterKind::FullScreen
        } else {
            PresenterKind::Default
        })
    }

    pub fn backdrop(mut self, backdrop: Backdrop) -> Self {
        self.backdrop = Some(backdrop);
        self
    }

    pub fn run<F, C>(self, root_factory: F) -> Result<()>
    where
        F: FnOnce() -> C + Send + 'static,
        C: Component + 'static,
    {
        init_app_platform()?;
        let title = self.title.unwrap_or_else(|| "islands-reactor".to_string());
        let eager = self.eager_templated_realization;
        let size = self.inner_size;
        let constraints = self.inner_constraints;
        let presenter = self.presenter;
        let backdrop = self.backdrop;

        let result = run_callback("OnLaunched", move || {
            let root: Box<dyn Component> = Box::new(root_factory());
            let host = ReactorHost::new_with_window_options_and_backdrop(
                &title,
                size,
                constraints,
                root,
                backdrop,
                |recon| {
                    recon.eager_templated_realization = eager;
                },
            )?;
            host.set_presenter(presenter);
            host.activate()?;
            HOST_SLOT.with(|slot| {
                *slot.borrow_mut() = Some(host);
            });
            Ok(())
        });

        result?;
        let loop_result = message_loop();
        clear_active_host();
        loop_result
    }

    pub fn render<F>(self, f: F) -> Result<()>
    where
        F: Fn(&mut crate::render_context::RenderCx) -> crate::element::Element + Send + 'static,
    {
        self.run(move || RenderFn(f))
    }
}

struct RenderFn<F>(F);

impl<F> crate::component::Component for RenderFn<F>
where
    F: Fn(&mut crate::render_context::RenderCx) -> crate::element::Element + 'static,
{
    fn render(
        &self,
        _props: &(),
        cx: &mut crate::render_context::RenderCx,
    ) -> crate::element::Element {
        (self.0)(cx)
    }
}

fn run_callback<F>(label: &'static str, f: F) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    match std::panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(err),
        Err(payload) => {
            let msg = crate::diagnostics::format_panic_payload(&payload);
            Err(Error::new(E_FAIL, format!("{label} panicked: {msg}")))
        }
    }
}

fn init_app_platform() -> Result<()> {
    crate::diagnostics::install();
    unsafe {
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        RoInitialize(RO_INIT_SINGLETHREADED).ok()?;
    }
    Ok(())
}
