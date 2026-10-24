use super::*;

/// Definition of a single item in a [`SelectorBar`].
#[derive(Clone, Debug, PartialEq)]
pub struct SelectorBarItemDef {
    /// Display text.
    pub text: String,
    /// Optional symbol icon.
    pub icon: Option<SymbolGlyph>,
}

impl SelectorBarItemDef {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, icon: SymbolGlyph) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Builder for a [`SelectorBarItemDef`].
pub fn selector_bar_item(text: impl Into<String>) -> SelectorBarItemDef {
    SelectorBarItemDef::new(text)
}

/// A horizontal tab-like selector backed by system XAML toggle buttons.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct SelectorBar {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub items: Vec<SelectorBarItemDef>,
    pub on_selection_changed: Option<Callback<String>>,
}

impl SelectorBar {
    pub fn new(items: Vec<SelectorBarItemDef>) -> Self {
        Self {
            items,
            ..Default::default()
        }
    }

    pub fn on_selection_changed(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_selection_changed = Some(f.into_callback());
        self
    }
}

impl Widget for SelectorBar {
    widget_header!(ControlKind::SelectorBar);
    fn bindings(&self) -> PropBindings {
        let mut out = crate::generated::selector_bar_bindings(self);
        out.push(Binding::Prop(
            Prop::Items,
            PropValue::SelectorBarItems(self.items.clone()),
        ));
        out
    }
}

pub fn selector_bar(items: Vec<SelectorBarItemDef>) -> SelectorBar {
    SelectorBar::new(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_include_upstream_selector_bar_surface() {
        let selector = selector_bar(vec![
            selector_bar_item("Recent"),
            selector_bar_item("Shared"),
        ])
        .on_selection_changed(|_: String| {});

        let bindings = selector.bindings();

        assert!(bindings.iter().any(|binding| matches!(
            binding,
            Binding::Event(Event::SelectionChanged, Some(EventHandler::Str(_)))
        )));
        let items = bindings.iter().find_map(|binding| match binding {
            Binding::Prop(Prop::Items, PropValue::SelectorBarItems(items)) => Some(items),
            _ => None,
        });
        let items = items.expect("SelectorBar should emit Items");
        assert_eq!(items[0].text, "Recent");
        assert_eq!(items[1].text, "Shared");
    }
}
