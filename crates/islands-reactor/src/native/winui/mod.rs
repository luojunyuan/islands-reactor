pub(crate) mod automation;
pub mod dispatcher;
pub(crate) mod hooks;
pub mod hosting;
pub(crate) mod template_cache;
pub(crate) mod xaml_interop;

include!("objects.rs");
include!("../generated.rs");
include!("dependency.rs");
include!("events.rs");
include!("relations.rs");
include!("adapter.rs");

pub use dispatcher::*;
pub use hooks::{DispatcherTimer, Rendering, on_rendering};
pub use hosting::*;
pub use template_cache::*;

pub(crate) use crate::*;
