use super::*;

pub use crate::bindings_iuxc::ScrollingScrollBarVisibility;

#[derive(Clone, Debug, PartialEq)]
pub struct ScrollView {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub child: Box<Element>,
    pub horizontal_scroll_bar_visibility: ScrollingScrollBarVisibility,
    pub vertical_scroll_bar_visibility: ScrollingScrollBarVisibility,
}

impl Default for ScrollView {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            child: Box::new(Element::Empty),
            horizontal_scroll_bar_visibility: ScrollingScrollBarVisibility::Hidden,
            vertical_scroll_bar_visibility: ScrollingScrollBarVisibility::Auto,
        }
    }
}

impl ScrollView {
    pub fn new(child: impl Into<Element>) -> Self {
        Self {
            child: Box::new(child.into()),
            ..Default::default()
        }
    }

    pub fn horizontal_scroll_bar_visibility(mut self, v: ScrollingScrollBarVisibility) -> Self {
        self.horizontal_scroll_bar_visibility = v;
        self
    }

    pub fn vertical_scroll_bar_visibility(mut self, v: ScrollingScrollBarVisibility) -> Self {
        self.vertical_scroll_bar_visibility = v;
        self
    }
}

impl Widget for ScrollView {
    widget_header!(ControlKind::ScrollView);

    fn bindings(&self) -> PropBindings {
        crate::generated::scroll_view_bindings(self)
    }

    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.child)
    }
}

pub fn scroll_view(child: impl Into<Element>) -> ScrollView {
    ScrollView::new(child)
}
