use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressRing {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub is_active: bool,
}
impl Default for ProgressRing {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            is_active: true,
        }
    }
}
impl ProgressRing {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn indeterminate() -> Self {
        Self::default()
    }
    pub fn active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }
}

impl Widget for ProgressRing {
    widget_header!(ControlKind::ProgressRing);
    fn bindings(&self) -> PropBindings {
        crate::generated::progress_ring_bindings(self)
    }
}
