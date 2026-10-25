#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms,
    clippy::useless_transmute,
    clippy::missing_transmute_annotations,
    clippy::missing_safety_doc,
    clippy::too_many_arguments
)]
mod native;

mod application;
mod component;
pub mod declaration;
pub mod dsl;
mod generated_declarations;
mod ir;
mod reconcile;
mod window;

#[allow(unused_imports)]
pub(crate) use application::{dispatcher, render_host};
#[allow(unused_imports)]
pub(crate) use application::{dispatcher::*, render_host::*};
#[allow(unused_imports)]
pub(crate) use component::*;
#[allow(unused_imports)]
pub(crate) use component::{component_element, context, custom, error_boundary, render_context};
#[allow(unused_imports)]
pub(crate) use component::{
    component_element::*, context::*, custom::*, error_boundary::*, render_context::*,
};
#[allow(unused_imports)]
pub(crate) use declaration::visual_modifiers as modifiers;
#[allow(unused_imports)]
pub(crate) use declaration::*;
#[allow(unused_imports)]
pub(crate) use generated_declarations::*;
#[allow(unused_imports)]
pub(crate) use ir::*;
#[allow(unused_imports)]
pub(crate) use native::{bindings, diagnostics};
#[allow(unused_imports)]
pub(crate) use reconcile::*;
#[allow(unused_imports)]
pub(crate) use window::*;

pub use windows_core::{Error, Interface, Result};
pub use windows_time::{DateTime, TimeSpan};

pub use application::dispatcher::*;
pub use application::render_host::*;
pub use component::component_element::*;
pub use component::context::Context;
pub use component::custom::*;
pub use component::error_boundary::*;
pub use component::render_context::*;
pub use component::*;
pub use declaration::accessibility::*;
pub use declaration::animation::*;
pub use declaration::callback::*;
pub use declaration::element::*;
pub use declaration::geometry::*;
pub use declaration::keyboard::*;
pub use declaration::pointer::*;
pub use declaration::prop_binding::*;
pub use declaration::resource::*;
pub use declaration::rich_text::{
    RichText, RichTextBlock, RichTextHyperlink, RichTextInline, RichTextLineBreak,
    RichTextParagraph, RichTextRun,
};
pub use declaration::templated_list::{
    SelectionMode, flip_view, grid_view, list_view, virtual_list,
};
pub use declaration::theme::*;
pub use declaration::tooltip::*;
pub use declaration::visual_modifiers::*;
pub use dsl::*;
pub use ir::{Backend, ControlId};
pub use native::app;
pub use native::app::*;
pub use native::bootstrap;
pub use native::bootstrap::*;
pub use native::winui;
pub use native::winui::dispatcher::WinUIDispatcher;
pub use native::winui::host::{
    Backdrop, PresenterKind, ReactorHost, RequestedTheme, set_backdrop, set_requested_theme,
};
pub use native::winui::*;
pub use native::winui::{DispatcherTimer, Rendering, on_rendering};
pub use window::*;

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
