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

mod accessibility;
mod animation;
mod backend;
mod callback;
mod component;
mod component_element;
mod context;
mod custom;
mod dispatcher;
mod element;
mod error_boundary;
mod generated;
mod geometry;
mod into_elements;
mod keyboard;
mod modifiers;
mod pointer;
mod prop_binding;
mod rc_fn;
mod reconciler;
mod render_context;
mod render_host;
mod resource;
mod rich_text;
mod templated_list;
mod theme;
mod tooltip;
mod widget;
mod widgets;
mod window;

pub mod declaration;
pub mod dsl;

#[allow(unused_imports)]
pub(crate) use accessibility::*;
#[allow(unused_imports)]
pub(crate) use animation::*;
#[allow(unused_imports)]
pub(crate) use backend::*;
#[allow(unused_imports)]
pub(crate) use callback::*;
#[allow(unused_imports)]
pub(crate) use component::*;
#[allow(unused_imports)]
pub(crate) use component_element::*;
#[allow(unused_imports)]
pub(crate) use context::*;
#[allow(unused_imports)]
pub(crate) use custom::*;
#[allow(unused_imports)]
pub(crate) use dispatcher::*;
#[allow(unused_imports)]
pub(crate) use element::*;
#[allow(unused_imports)]
pub(crate) use error_boundary::*;
#[allow(unused_imports)]
pub(crate) use generated::*;
#[allow(unused_imports)]
pub(crate) use geometry::*;
#[allow(unused_imports)]
pub(crate) use into_elements::*;
#[allow(unused_imports)]
pub(crate) use keyboard::*;
#[allow(unused_imports)]
pub(crate) use modifiers::*;
#[allow(unused_imports)]
pub(crate) use pointer::*;
#[allow(unused_imports)]
pub(crate) use prop_binding::*;
#[allow(unused_imports)]
pub(crate) use rc_fn::*;
#[allow(unused_imports)]
pub(crate) use reconciler::*;
#[allow(unused_imports)]
pub(crate) use render_context::*;
#[allow(unused_imports)]
pub(crate) use render_host::*;
#[allow(unused_imports)]
pub(crate) use resource::*;
#[allow(unused_imports)]
pub(crate) use rich_text::*;
#[allow(unused_imports)]
pub(crate) use templated_list::*;
#[allow(unused_imports)]
pub(crate) use theme::*;
#[allow(unused_imports)]
pub(crate) use tooltip::*;
#[allow(unused_imports)]
pub(crate) use widget::*;
#[allow(unused_imports)]
pub(crate) use widgets::*;
#[allow(unused_imports)]
pub(crate) use window::*;

pub use windows_core::{Error, Interface, Result};
pub use windows_time::{DateTime, TimeSpan};

pub use animation::*;
pub use backend::{Backend, ControlId};
pub use callback::*;
pub use component::*;
pub use component_element::*;
pub use context::Context;
pub use custom::*;
pub use declaration::*;
pub use dispatcher::*;
pub use dsl::*;
pub use element::*;
pub use element::{
    AccessibilityModifiers, AttachedProps, Brush, Color, CommandBarDefaultLabelPosition,
    FlyoutPlacementMode, GridLength, GridPlacement, HeadingLevel, HorizontalAlignment,
    KeyModifiers, KeyboardAccelerator, KeyboardKey, LiveSetting, Modifiers, Orientation,
    PasswordRevealMode, PointerEventInfo, PointerHandlers, ScrollBarVisibility,
    ScrollingScrollBarVisibility, Stretch, Symbol, TextWrapping, Thickness, Tooltip,
    TooltipContent, TooltipPlacement, TreeViewSelectionMode, VerticalAlignment,
};
pub use error_boundary::*;
pub use into_elements::IntoElements;
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
pub(crate) use native::{bindings, bindings_iuxc, diagnostics};
pub use render_context::*;
pub use render_host::*;
pub use resource::*;
pub use templated_list::{SelectionMode, flip_view, grid_view, list_view, virtual_list};
pub use theme::*;
pub use window::*;
