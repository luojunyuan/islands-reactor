use std::cell::{Cell, RefCell};
use std::rc::Rc;

use std::collections::{HashMap, HashSet};
use windows_core::{IInspectable, Interface, Result};

use crate::bindings as Xaml;

pub(super) fn string_as_textblock(text: &str) -> Result<IInspectable> {
    let text_block = Xaml::TextBlock::new()?;
    text_block.SetText(text)?;
    text_block.cast()
}

/// Backend that creates native `Windows.UI.Xaml` controls for XAML Islands.
pub struct WinUIBackend {
    controls: RefCell<HashMap<ControlId, Handle>>,
    event_revokers: RefCell<HashMap<(ControlId, Event), Vec<EventSubscription>>>,
    live_regions: RefCell<HashSet<ControlId>>,
    templated_selection_revokers: RefCell<HashMap<ControlId, EventSubscription>>,
    templated_realizers: RefCell<HashMap<ControlId, Rc<dyn Fn(usize)>>>,
    parent_children: RefCell<HashMap<ControlId, Vec<ControlId>>>,
    templated_rows: RefCell<HashMap<ControlId, Vec<Option<ControlId>>>>,
    templated_selected_indices: RefCell<HashMap<ControlId, i32>>,
    theme_brush_registry: RefCell<HashMap<ControlId, Vec<(Prop, crate::theme::ThemeRef)>>>,
    next_id: RefCell<u32>,
}

impl Default for WinUIBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl WinUIBackend {
    pub fn new() -> Self {
        Self {
            controls: RefCell::new(HashMap::default()),
            event_revokers: RefCell::new(HashMap::default()),
            live_regions: RefCell::new(HashSet::default()),
            templated_selection_revokers: RefCell::new(HashMap::default()),
            templated_realizers: RefCell::new(HashMap::default()),
            parent_children: RefCell::new(HashMap::default()),
            templated_rows: RefCell::new(HashMap::default()),
            templated_selected_indices: RefCell::new(HashMap::default()),
            theme_brush_registry: RefCell::new(HashMap::default()),
            next_id: RefCell::new(0),
        }
    }

    pub(crate) fn root_titlebar_state(&self, root_id: Option<ControlId>) -> Option<TitleBarState> {
        let root_id = root_id?;
        let controls = self.controls.borrow();
        titlebar_state_from_handle(controls.get(&root_id)?).or_else(|| {
            self.parent_children
                .borrow()
                .get(&root_id)
                .into_iter()
                .flat_map(|children| children.iter())
                .find_map(|child_id| titlebar_state_from_handle(controls.get(child_id)?))
        })
    }

    pub fn get_ui_element(&self, id: ControlId) -> Option<IInspectable> {
        self.controls
            .borrow()
            .get(&id)
            .and_then(|h| h.as_ui_element().ok())
            .and_then(|u| u.cast().ok())
    }

    pub(crate) fn shutdown(&self) {
        self.event_revokers.borrow_mut().clear();
        self.live_regions.borrow_mut().clear();
        self.templated_selection_revokers.borrow_mut().clear();
        self.templated_realizers.borrow_mut().clear();
        self.templated_rows.borrow_mut().clear();
        self.templated_selected_indices.borrow_mut().clear();
        self.parent_children.borrow_mut().clear();
        self.theme_brush_registry.borrow_mut().clear();
        self.controls.borrow_mut().clear();
    }

    fn alloc_id(&self) -> ControlId {
        let mut counter = self.next_id.borrow_mut();
        *counter += 1;
        ControlId::new(*counter)
    }

    fn visual_index(&self, parent: ControlId, logical: usize) -> usize {
        self.parent_children
            .borrow()
            .get(&parent)
            .map_or(logical, |children| logical.min(children.len()))
    }

    fn apply_templated_selected_index(&self, id: ControlId, index: i32) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let selector: Xaml::ISelector = match handle {
            Handle::ListView(lv) => match lv.cast() {
                Ok(selector) => selector,
                Err(_) => return,
            },
            Handle::GridView(gv) => match gv.cast() {
                Ok(selector) => selector,
                Err(_) => return,
            },
            Handle::FlipView(fv) => match fv.cast() {
                Ok(selector) => selector,
                Err(_) => return,
            },
            _ => return,
        };
        let _ = selector.SetSelectedIndex(index);
    }

    fn notify_live_regions(&self, id: ControlId) {
        let live_regions = self.live_regions.borrow();
        if live_regions.is_empty() {
            return;
        }

        let parents = self.parent_children.borrow();
        let mut current = Some(id);
        let mut visited = HashSet::new();
        let mut regions = Vec::new();
        while let Some(current_id) = current {
            if !visited.insert(current_id) {
                break;
            }
            if live_regions.contains(&current_id) {
                regions.push(current_id);
            }
            current = parents
                .iter()
                .find_map(|(parent, children)| children.contains(&current_id).then_some(*parent));
        }
        drop(parents);
        drop(live_regions);

        if regions.is_empty()
            || !Xaml::AutomationPeer::ListenerExists(Xaml::AutomationEvents::LiveRegionChanged)
                .unwrap_or(false)
        {
            return;
        }
        for region in regions {
            let Some(element) = self
                .get_ui_element(region)
                .and_then(|element| element.cast::<Xaml::UIElement>().ok())
            else {
                continue;
            };
            match Xaml::FrameworkElementAutomationPeer::CreatePeerForElement(&element) {
                Ok(peer) => {
                    if let Err(error) =
                        peer.RaiseAutomationEvent(Xaml::AutomationEvents::LiveRegionChanged)
                    {
                        crate::diagnostics::emit(&format!(
                            "islands_reactor: failed to raise a live-region event: {error}\n"
                        ));
                    }
                }
                Err(error) if error.code().is_ok() => {}
                Err(error) => crate::diagnostics::emit(&format!(
                    "islands_reactor: failed to create an automation peer: {error}\n"
                )),
            }
        }
    }

    fn visual_insert_at(&self, parent: ControlId, index: usize, child: ControlId) {
        let map = self.controls.borrow();
        let Some(parent_h) = map.get(&parent) else {
            return;
        };
        let Some(child_h) = map.get(&child) else {
            return;
        };
        let Ok(child_ui) = child_h.as_ui_element() else {
            return;
        };

        match parent_h {
            Handle::StackPanel(_)
            | Handle::Grid(_)
            | Handle::Canvas(_)
            | Handle::RelativePanel(_) => {
                if let Some(children) = panel_children_vec(parent_h) {
                    let size = children.Size().unwrap_or(0);
                    if index as u32 >= size {
                        let _ = children.Append(&child_ui);
                    } else {
                        let _ = children.InsertAt(index as u32, &child_ui);
                    }
                }
            }
            Handle::Border(border) => {
                let _ = border.SetChild(&child_ui);
            }
            Handle::Viewbox(v) => {
                let _ = v.SetChild(&child_ui);
            }
            Handle::ScrollView(_) | Handle::ScrollViewer(_) | Handle::PivotItem(_) => {
                if let Some(cc) = content_control_for(parent_h) {
                    let _ = cc.SetContent(&child_ui);
                }
            }
            Handle::SplitView(sv) => {
                let _ = sv.SetContent(&child_ui);
            }
            Handle::Pivot(_) => {
                if let Some(items) = items_vector(parent_h) {
                    let Ok(insp) = child_ui.cast::<IInspectable>() else {
                        return;
                    };
                    let size = items.Size().unwrap_or(0);
                    if index as u32 >= size {
                        let _ = items.Append(&insp);
                    } else {
                        let _ = items.InsertAt(index as u32, &insp);
                    }
                }
            }
            Handle::ListView(_) | Handle::GridView(_) | Handle::FlipView(_) => {
                if let Some(items) = items_vector(parent_h) {
                    let Ok(insp) = child_ui.cast::<IInspectable>() else {
                        return;
                    };
                    let size = items.Size().unwrap_or(0);
                    if index as u32 >= size {
                        let _ = items.Append(&insp);
                    } else {
                        let _ = items.InsertAt(index as u32, &insp);
                    }
                }
            }
            _ => {}
        }
    }

    fn visual_remove_at(&self, parent: ControlId, index: usize) {
        let map = self.controls.borrow();
        let Some(parent_h) = map.get(&parent) else {
            return;
        };
        match parent_h {
            Handle::StackPanel(_)
            | Handle::Grid(_)
            | Handle::Canvas(_)
            | Handle::RelativePanel(_) => {
                if let Some(children) = panel_children_vec(parent_h)
                    && (index as u32) < children.Size().unwrap_or(0)
                {
                    let _ = children.RemoveAt(index as u32);
                }
            }
            Handle::Border(border) => {
                let _ = border.SetChild(None);
            }
            Handle::Viewbox(v) => {
                let _ = v.SetChild(None);
            }
            Handle::ScrollView(_) | Handle::ScrollViewer(_) | Handle::PivotItem(_) => {
                if let Some(cc) = content_control_for(parent_h) {
                    let _ = cc.SetContent(None);
                }
            }
            Handle::SplitView(sv) => {
                let _ = sv.SetContent(None);
            }
            Handle::Pivot(_) | Handle::ListView(_) | Handle::GridView(_) | Handle::FlipView(_) => {
                if let Some(items) = items_vector(parent_h)
                    && (index as u32) < items.Size().unwrap_or(0)
                {
                    let _ = items.RemoveAt(index as u32);
                }
            }
            _ => {}
        }
    }

    fn visual_set_at(&self, parent: ControlId, index: usize, child: ControlId) {
        self.visual_remove_at(parent, index);
        self.visual_insert_at(parent, index, child);
    }
}

impl Backend for WinUIBackend {
    fn create(&mut self, kind: ControlKind) -> ControlId {
        let id = self.alloc_id();
        let handle = Handle::new(kind).unwrap_or_else(|err| {
            panic!("islands-reactor: unsupported or failed native control {kind:?}: {err:?}")
        });
        self.controls.borrow_mut().insert(id, handle);
        id
    }

    fn set_prop(&mut self, id: ControlId, prop: Prop, value: &PropValue) {
        if self.set_prop_inner(id, prop, value).is_ok() && prop == Prop::Text {
            self.notify_live_regions(id);
        }
    }

    islands_relation_backend_methods!();

    fn destroy(&mut self, id: ControlId) {
        self.controls.borrow_mut().remove(&id);
        self.event_revokers
            .borrow_mut()
            .retain(|(cid, _), _| *cid != id);
        self.templated_selection_revokers.borrow_mut().remove(&id);
        self.templated_realizers.borrow_mut().remove(&id);
        self.parent_children.borrow_mut().remove(&id);
        self.templated_rows.borrow_mut().remove(&id);
        self.templated_selected_indices.borrow_mut().remove(&id);
        self.theme_brush_registry.borrow_mut().remove(&id);
        self.live_regions.borrow_mut().remove(&id);
    }

    islands_event_backend_methods!();

    fn set_templated_item_count(&mut self, id: ControlId, count: usize) {
        self.templated_rows
            .borrow_mut()
            .entry(id)
            .or_default()
            .resize(count, None);
        if let Some(realize) = self.templated_realizers.borrow().get(&id).cloned() {
            for row in 0..count {
                realize(row);
            }
        }
    }

    fn set_templated_row_content(
        &mut self,
        list_id: ControlId,
        row_idx: usize,
        content: Option<ControlId>,
    ) {
        let mut rows = self.templated_rows.borrow_mut();
        let list = rows.entry(list_id).or_default();
        if row_idx >= list.len() {
            list.resize(row_idx + 1, None);
        }
        let previous = list[row_idx];
        list[row_idx] = content;
        drop(rows);

        match (previous, content) {
            (None, None) => {}
            (Some(_), None) => self.visual_remove_at(list_id, row_idx),
            (None, Some(content_id)) => self.visual_insert_at(list_id, row_idx, content_id),
            (Some(previous_id), Some(content_id)) if previous_id == content_id => {}
            (Some(_), Some(content_id)) => self.visual_set_at(list_id, row_idx, content_id),
        }

        if let Some(index) = self
            .templated_selected_indices
            .borrow()
            .get(&list_id)
            .copied()
        {
            self.apply_templated_selected_index(list_id, index);
        }
    }

    fn set_templated_selected_index(&mut self, id: ControlId, index: i32) {
        self.templated_selected_indices
            .borrow_mut()
            .insert(id, index);
        self.apply_templated_selected_index(id, index);
    }

    fn set_templated_selection_mode(
        &mut self,
        id: ControlId,
        mode: crate::templated_list::SelectionMode,
    ) {
        let native = match mode {
            crate::templated_list::SelectionMode::None => 0,
            crate::templated_list::SelectionMode::Single => 1,
            crate::templated_list::SelectionMode::Multiple => 2,
            crate::templated_list::SelectionMode::Extended => 3,
        };
        let _ = self.set_prop_inner(id, Prop::SelectionMode, &PropValue::I32(native));
    }

    fn attach_templated_selection_changed(&mut self, id: ControlId, handler: Callback<i32>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        if let Ok(selector) = handle.cast_inner::<Xaml::Selector>() {
            let selector_for_cb = selector.clone();
            if let Ok(subscription) = selector.SelectionChanged(move |_, _| {
                handler.invoke(selector_for_cb.SelectedIndex().unwrap_or(-1));
            }) {
                self.templated_selection_revokers
                    .borrow_mut()
                    .insert(id, subscription);
            }
        }
    }

    fn attach_templated_realization(
        &mut self,
        id: ControlId,
        realize: Rc<dyn Fn(usize)>,
        _recycle: Rc<dyn Fn(usize)>,
    ) {
        self.templated_realizers.borrow_mut().insert(id, realize);
    }

    fn set_header_element(&mut self, id: ControlId, header_id: Option<ControlId>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let content = header_id
            .and_then(|hid| map.get(&hid))
            .and_then(|h| h.as_ui_element().ok());
        match handle {
            Handle::TitleBar(tb) => {
                let _ = tb.set_content(content.as_ref());
            }
            _ => {}
        };
    }

    fn set_pane_element(&mut self, id: ControlId, pane_id: Option<ControlId>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let content = pane_id
            .and_then(|pid| map.get(&pid))
            .and_then(|h| h.as_ui_element().ok());
        match handle {
            Handle::SplitView(split) => match content {
                Some(ui) => {
                    let _ = split.SetPane(&ui);
                }
                None => {
                    let _ = split.SetPane(None);
                }
            },
            Handle::TitleBar(tb) => {
                let _ = tb.set_footer(content.as_ref());
            }
            _ => {}
        };
    }

    fn set_theme_bindings(
        &mut self,
        id: ControlId,
        _kind: ControlKind,
        bindings: &[(Prop, crate::theme::ThemeRef)],
    ) {
        if bindings.is_empty() {
            self.theme_brush_registry.borrow_mut().remove(&id);
            let map = self.controls.borrow();
            if let Some(handle) = map.get(&id)
                && let Some((_, fe)) = style_target_for_handle(handle)
            {
                let _ = fe.SetStyle(None);
            }
            return;
        }

        self.theme_brush_registry
            .borrow_mut()
            .insert(id, bindings.to_vec());

        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        apply_theme_resource_style(handle, bindings);
    }

    fn on_theme_changed(&mut self) {
        let controls = self.controls.borrow();
        let registry = self.theme_brush_registry.borrow();
        for (id, bindings) in registry.iter() {
            let Some(handle) = controls.get(id) else {
                continue;
            };
            apply_theme_resource_style(handle, bindings);
        }
    }

    fn set_implicit_transitions(
        &mut self,
        _id: ControlId,
        _transitions: Option<crate::animation::ImplicitTransitions>,
    ) {
    }

    fn run_property_animation(
        &mut self,
        _id: ControlId,
        _config: Option<crate::animation::AnimationConfig>,
    ) {
    }

    fn set_rich_text_paragraphs(
        &mut self,
        id: ControlId,
        paragraphs: &[crate::rich_text::RichTextParagraph],
    ) {
        let map = self.controls.borrow();
        let Some(Handle::RichTextBlock(rtb)) = map.get(&id) else {
            return;
        };
        let Ok(blocks) = rtb.Blocks() else {
            return;
        };
        let _ = blocks.Clear();
        for para_def in paragraphs {
            let Ok(para) = Xaml::Paragraph::new() else {
                continue;
            };
            let Ok(inlines) = para.Inlines() else {
                continue;
            };
            for inline in &para_def.inlines {
                let text = match inline {
                    crate::rich_text::RichTextInline::Run(run) => run.text.as_str(),
                    crate::rich_text::RichTextInline::LineBreak => "\n",
                    crate::rich_text::RichTextInline::Hyperlink(link) => link.text.as_str(),
                };
                if let Ok(run) = Xaml::Run::new() {
                    let _ = run.SetText(text);
                    let _ = run.cast::<Xaml::Inline>().and_then(|r| inlines.Append(&r));
                }
            }
            let _ = para.cast::<Xaml::Block>().and_then(|p| blocks.Append(&p));
        }
    }

    fn set_tooltip(&mut self, id: ControlId, tooltip: Option<&crate::tooltip::Tooltip>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let Ok(dep) = handle
            .as_framework_element()
            .and_then(|fe| fe.cast::<Xaml::DependencyObject>())
        else {
            return;
        };
        let inspectable = tooltip.and_then(|t| match &t.content {
            crate::tooltip::TooltipContent::Text(s) => Some(string_reference(s)),
            crate::tooltip::TooltipContent::Rich(_) => None,
        });
        let _ = Xaml::ToolTipService::SetToolTip(&dep, inspectable.as_ref());
    }

    fn set_accessibility(&mut self, id: ControlId, accessibility: &crate::AccessibilityModifiers) {
        match accessibility.live_setting {
            Some(crate::LiveSetting::Polite | crate::LiveSetting::Assertive) => {
                self.live_regions.borrow_mut().insert(id);
            }
            Some(crate::LiveSetting::Off) | None => {
                self.live_regions.borrow_mut().remove(&id);
            }
        }
        let element = self
            .controls
            .borrow()
            .get(&id)
            .and_then(|handle| handle.as_framework_element().ok())
            .and_then(|element| element.cast::<Xaml::DependencyObject>().ok());
        if let Some(element) = element
            && let Err(error) = automation::apply(&element, accessibility)
        {
            crate::diagnostics::emit(&format!(
                "islands_reactor: failed to set UI Automation properties: {error}\n"
            ));
        }
    }

    fn get_native_element(&self, id: ControlId) -> Option<IInspectable> {
        self.get_ui_element(id)
    }
}
