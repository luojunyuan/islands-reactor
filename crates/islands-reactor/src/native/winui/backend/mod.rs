use std::cell::{Cell, RefCell};
use std::rc::Rc;

use rustc_hash::FxHashMap;
use windows_core::{IInspectable, Interface, Result};

use crate::bindings as Xaml;
use crate::ir::*;
use crate::*;

mod convert;
mod generated;

/// Backend that creates native `Windows.UI.Xaml` controls for XAML Islands.
pub struct WinUIBackend {
    controls: RefCell<FxHashMap<ControlId, Handle>>,
    event_revokers: RefCell<FxHashMap<(ControlId, Event), Vec<EventSubscription>>>,
    templated_selection_revokers: RefCell<FxHashMap<ControlId, EventSubscription>>,
    templated_realizers: RefCell<FxHashMap<ControlId, Rc<dyn Fn(usize)>>>,
    parent_children: RefCell<FxHashMap<ControlId, Vec<ControlId>>>,
    templated_rows: RefCell<FxHashMap<ControlId, Vec<Option<ControlId>>>>,
    templated_selected_indices: RefCell<FxHashMap<ControlId, i32>>,
    theme_brush_registry: RefCell<FxHashMap<ControlId, Vec<(Prop, crate::theme::ThemeRef)>>>,
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
            controls: RefCell::new(FxHashMap::default()),
            event_revokers: RefCell::new(FxHashMap::default()),
            templated_selection_revokers: RefCell::new(FxHashMap::default()),
            templated_realizers: RefCell::new(FxHashMap::default()),
            parent_children: RefCell::new(FxHashMap::default()),
            templated_rows: RefCell::new(FxHashMap::default()),
            templated_selected_indices: RefCell::new(FxHashMap::default()),
            theme_brush_registry: RefCell::new(FxHashMap::default()),
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

    fn set_prop_inner(&self, id: ControlId, prop: Prop, value: &PropValue) -> Result<()> {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return Ok(());
        };

        if generated::dispatch_set_prop(handle, prop, value)? {
            return Ok(());
        }

        match (prop, value, handle) {
            (Prop::Text, PropValue::Str(s), Handle::TextBlock(tb)) => tb.SetText(s)?,
            (Prop::Text, PropValue::Str(s), Handle::TextBox(tb)) => {
                if tb.Text().ok() != Some(s.clone()) {
                    tb.SetText(s)?;
                }
            }
            (Prop::Text, PropValue::Str(s), Handle::AutoSuggestBox(asb)) => {
                if asb.Text().ok().as_deref() != Some(s.as_str()) {
                    asb.SetText(s)?;
                }
            }
            (
                Prop::TextWrapping | Prop::TextWrappingWrap,
                PropValue::I32(v),
                Handle::TextBlock(tb),
            ) => {
                tb.SetTextWrapping(Xaml::TextWrapping(*v))?;
            }
            (
                Prop::TextWrapping | Prop::TextWrappingWrap,
                PropValue::I32(v),
                Handle::TextBox(tb),
            ) => {
                tb.SetTextWrapping(Xaml::TextWrapping(*v))?;
            }
            (Prop::IsTextSelectionEnabled, PropValue::Bool(v), Handle::TextBlock(tb)) => {
                tb.SetIsTextSelectionEnabled(*v)?;
            }
            (Prop::PlaceholderText, PropValue::Str(s), h) => {
                if let Ok(tb) = h.cast_inner::<Xaml::TextBox>() {
                    tb.cast::<Xaml::ITextBox2>()?.SetPlaceholderText(s)?;
                } else if let Ok(pb) = h.cast_inner::<Xaml::PasswordBox>() {
                    pb.cast::<Xaml::IPasswordBox2>()?.SetPlaceholderText(s)?;
                } else if let Ok(asb) = h.cast_inner::<Xaml::AutoSuggestBox>() {
                    asb.SetPlaceholderText(s)?;
                } else if let Ok(cdp) = h.cast_inner::<Xaml::CalendarDatePicker>() {
                    cdp.SetPlaceholderText(s)?;
                }
            }
            (Prop::Header, PropValue::Str(s), h) => {
                let content = string_reference(s);
                if let Ok(tb) = h.cast_inner::<Xaml::TextBox>() {
                    tb.cast::<Xaml::ITextBox2>()?.SetHeader(&content)?;
                } else if let Ok(pb) = h.cast_inner::<Xaml::PasswordBox>() {
                    pb.cast::<Xaml::IPasswordBox2>()?.SetHeader(&content)?;
                } else if let Ok(cb) = h.cast_inner::<Xaml::ComboBox>() {
                    cb.cast::<Xaml::IComboBox2>()?.SetHeader(&content)?;
                } else if let Ok(slider) = h.cast_inner::<Xaml::Slider>() {
                    slider.cast::<Xaml::ISlider2>()?.SetHeader(&content)?;
                } else if let Ok(tp) = h.cast_inner::<Xaml::TimePicker>() {
                    tp.SetHeader(&content)?;
                } else if let Ok(dp) = h.cast_inner::<Xaml::DatePicker>() {
                    dp.SetHeader(&content)?;
                } else if let Ok(cdp) = h.cast_inner::<Xaml::CalendarDatePicker>() {
                    cdp.SetHeader(&content)?;
                } else if let Ok(asb) = h.cast_inner::<Xaml::AutoSuggestBox>() {
                    asb.SetHeader(&content)?;
                } else if let Ok(reb) = h.cast_inner::<Xaml::RichEditBox>() {
                    reb.cast::<Xaml::IRichEditBox2>()?.SetHeader(&content)?;
                }
            }
            (Prop::Content, PropValue::Str(s), h) => {
                set_content_string(h, s)?;
            }
            (Prop::IsEnabled, PropValue::Bool(v), h) => {
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetIsEnabled(*v)?;
                }
            }
            (Prop::IsChecked, PropValue::Bool(v), h) => {
                if let Ok(toggle) = h.cast_inner::<Xaml::ToggleButton>() {
                    toggle.SetIsChecked(Some(*v))?;
                }
            }
            (Prop::IsOn, PropValue::Bool(v), Handle::ToggleSwitch(ts)) => {
                ts.SetIsOn(*v)?;
            }
            (Prop::OnContent, PropValue::Str(s), Handle::ToggleSwitch(ts)) => {
                ts.SetOnContent(&string_reference(s))?;
            }
            (Prop::OffContent, PropValue::Str(s), Handle::ToggleSwitch(ts)) => {
                ts.SetOffContent(&string_reference(s))?;
            }
            (Prop::Orientation, PropValue::I32(v), Handle::StackPanel(sp)) => {
                sp.SetOrientation(Xaml::Orientation(*v))?;
            }
            (Prop::Orientation, PropValue::I32(v), Handle::Slider(slider)) => {
                slider.SetOrientation(Xaml::Orientation(*v))?;
            }
            (Prop::Spacing, PropValue::F64(v), Handle::StackPanel(sp)) => {
                sp.cast::<Xaml::IStackPanel4>()?.SetSpacing(*v)?;
            }
            (Prop::RowSpacing, PropValue::F64(v), Handle::Grid(g)) => {
                g.cast::<Xaml::IGrid3>()?.SetRowSpacing(*v)?;
            }
            (Prop::ColumnSpacing, PropValue::F64(v), Handle::Grid(g)) => {
                g.cast::<Xaml::IGrid3>()?.SetColumnSpacing(*v)?;
            }
            (Prop::Margin, PropValue::Thickness(t), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetMargin(xaml_thickness(*t))?;
                }
            }
            (Prop::Padding, PropValue::Thickness(t), h) => {
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetPadding(xaml_thickness(*t))?;
                } else if let Ok(border) = h.cast_inner::<Xaml::Border>() {
                    border.SetPadding(xaml_thickness(*t))?;
                }
            }
            (Prop::Width, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetWidth(*v)?;
                }
            }
            (Prop::Height, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetHeight(*v)?;
                }
            }
            (Prop::MinWidth, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetMinWidth(*v)?;
                }
            }
            (Prop::MaxWidth, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetMaxWidth(*v)?;
                }
            }
            (Prop::MinHeight, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetMinHeight(*v)?;
                }
            }
            (Prop::MaxHeight, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetMaxHeight(*v)?;
                }
            }
            (Prop::HorizontalAlignment, PropValue::I32(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetHorizontalAlignment(Xaml::HorizontalAlignment(*v))?;
                }
            }
            (Prop::VerticalAlignment, PropValue::I32(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetVerticalAlignment(Xaml::VerticalAlignment(*v))?;
                }
            }
            (Prop::Opacity, PropValue::F64(v), h) => {
                if let Ok(ui) = h.cast_inner::<Xaml::UIElement>() {
                    ui.SetOpacity(*v)?;
                }
            }
            (Prop::FontSize, PropValue::F64(v), h) => {
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetFontSize(*v)?;
                } else if let Ok(tb) = h.cast_inner::<Xaml::TextBlock>() {
                    tb.SetFontSize(*v)?;
                } else if let Ok(rtb) = h.cast_inner::<Xaml::RichTextBlock>() {
                    rtb.SetFontSize(*v)?;
                }
            }
            (Prop::FontWeight, PropValue::U16(v), h) => {
                let weight = Xaml::FontWeight { weight: *v };
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetFontWeight(weight)?;
                } else if let Ok(tb) = h.cast_inner::<Xaml::TextBlock>() {
                    tb.SetFontWeight(weight)?;
                }
            }
            (Prop::FontFamily, PropValue::Str(s), h) => {
                let family = Xaml::FontFamily::CreateInstanceWithName(s)?;
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetFontFamily(&family)?;
                } else if let Ok(tb) = h.cast_inner::<Xaml::TextBlock>() {
                    tb.SetFontFamily(&family)?;
                } else if let Ok(rtb) = h.cast_inner::<Xaml::RichTextBlock>() {
                    rtb.SetFontFamily(&family)?;
                }
            }
            (Prop::Foreground, PropValue::Brush(b), h) => {
                let brush = xaml_brush(b)?;
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetForeground(&brush)?;
                } else if let Ok(tb) = h.cast_inner::<Xaml::TextBlock>() {
                    tb.SetForeground(&brush)?;
                } else if let Ok(rtb) = h.cast_inner::<Xaml::RichTextBlock>() {
                    rtb.SetForeground(&brush)?;
                }
            }
            (Prop::Background, PropValue::Brush(b), h) => {
                let brush = xaml_brush(b)?;
                if let Ok(border) = h.cast_inner::<Xaml::Border>() {
                    border.SetBackground(&brush)?;
                } else if let Ok(panel) = h.cast_inner::<Xaml::Panel>() {
                    panel.SetBackground(&brush)?;
                } else if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetBackground(&brush)?;
                }
            }
            (Prop::StyleVariant, PropValue::I32(v), Handle::Button(button)) => {
                apply_button_style_variant(button, *v)?;
            }
            (Prop::BorderBrush, PropValue::Brush(b), Handle::Border(border)) => {
                border.SetBorderBrush(&xaml_brush(b)?)?;
            }
            (Prop::BorderThickness, PropValue::Thickness(t), Handle::Border(border)) => {
                border.SetBorderThickness(xaml_thickness(*t))?;
            }
            (Prop::CornerRadius, PropValue::F64(v), Handle::Border(border)) => {
                border.SetCornerRadius(Xaml::CornerRadius {
                    top_left: *v,
                    top_right: *v,
                    bottom_right: *v,
                    bottom_left: *v,
                })?;
            }
            (Prop::GridRows, PropValue::GridLengths(rows), Handle::Grid(grid)) => {
                set_grid_rows(grid, rows)?;
            }
            (Prop::GridColumns, PropValue::GridLengths(cols), Handle::Grid(grid)) => {
                set_grid_columns(grid, cols)?;
            }
            (Prop::AttachedGridRow, PropValue::I32(v), h) => {
                let element = h.as_framework_element()?;
                Xaml::Grid::SetRow(&element, *v)?;
            }
            (Prop::AttachedGridColumn, PropValue::I32(v), h) => {
                let element = h.as_framework_element()?;
                Xaml::Grid::SetColumn(&element, *v)?;
            }
            (Prop::AttachedGridRowSpan, PropValue::I32(v), h) => {
                let element = h.as_framework_element()?;
                Xaml::Grid::SetRowSpan(&element, *v)?;
            }
            (Prop::AttachedGridColumnSpan, PropValue::I32(v), h) => {
                let element = h.as_framework_element()?;
                Xaml::Grid::SetColumnSpan(&element, *v)?;
            }
            (Prop::AttachedCanvasLeft, PropValue::F64(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::Canvas::SetLeft(&element, *v)?;
            }
            (Prop::AttachedCanvasTop, PropValue::F64(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::Canvas::SetTop(&element, *v)?;
            }
            (Prop::AttachedCanvasZIndex, PropValue::I32(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::Canvas::SetZIndex(&element, *v)?;
            }
            (Prop::AlignLeftWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignLeftWithPanel(&element, *v)?;
            }
            (Prop::AlignRightWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignRightWithPanel(&element, *v)?;
            }
            (Prop::AlignTopWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignTopWithPanel(&element, *v)?;
            }
            (Prop::AlignBottomWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignBottomWithPanel(&element, *v)?;
            }
            (Prop::AlignHCenterWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignHorizontalCenterWithPanel(&element, *v)?;
            }
            (Prop::AlignVCenterWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignVerticalCenterWithPanel(&element, *v)?;
            }
            (Prop::ImageSource, PropValue::Str(uri), Handle::Image(image)) => {
                set_image_uri(image, uri)?;
            }
            (Prop::Stretch, PropValue::I32(v), h) => {
                if let Ok(image) = h.cast_inner::<Xaml::Image>() {
                    image.SetStretch(Xaml::Stretch(*v))?;
                } else if let Ok(viewbox) = h.cast_inner::<Xaml::Viewbox>() {
                    viewbox.SetStretch(Xaml::Stretch(*v))?;
                }
            }
            (Prop::Value, PropValue::F64(v), h) => {
                if let Ok(range) = h.cast_inner::<Xaml::RangeBase>() {
                    range.SetValue(*v)?;
                } else if let Ok(rating) = h.cast_inner::<Xaml::RatingControl>() {
                    rating.SetValue(*v)?;
                } else if let Ok(cp) = h.cast_inner::<Xaml::ColorPicker>() {
                    let _ = cp.SetColor(Xaml::Color {
                        a: 255,
                        r: *v as u8,
                        g: *v as u8,
                        b: *v as u8,
                    });
                }
            }
            (Prop::Minimum, PropValue::F64(v), h) => {
                if let Ok(range) = h.cast_inner::<Xaml::RangeBase>() {
                    range.SetMinimum(*v)?;
                }
            }
            (Prop::Maximum, PropValue::F64(v), h) => {
                if let Ok(range) = h.cast_inner::<Xaml::RangeBase>() {
                    range.SetMaximum(*v)?;
                }
            }
            (Prop::Step, PropValue::F64(v), Handle::Slider(slider)) => {
                slider.SetStepFrequency(*v)?;
            }
            (Prop::IsActive, PropValue::Bool(v), Handle::ProgressRing(ring)) => {
                ring.SetIsActive(*v)?;
            }
            (Prop::IsIndeterminate, PropValue::Bool(v), h) => {
                if let Ok(bar) = h.cast_inner::<Xaml::ProgressBar>() {
                    bar.SetIsIndeterminate(*v)?;
                }
            }
            (Prop::SelectedIndex, PropValue::I32(v), h) => {
                if let Ok(selector) = h.cast_inner::<Xaml::Selector>() {
                    selector.SetSelectedIndex(*v)?;
                } else if let Ok(pivot) = h.cast_inner::<Xaml::Pivot>() {
                    pivot.SetSelectedIndex(*v)?;
                }
            }
            (Prop::SelectionMode, PropValue::I32(v), h) => {
                if let Ok(list) = h.cast_inner::<Xaml::ListView>() {
                    list.cast::<Xaml::IListViewBase>()?
                        .SetSelectionMode(Xaml::ListViewSelectionMode(*v))?;
                } else if let Ok(grid) = h.cast_inner::<Xaml::GridView>() {
                    grid.cast::<Xaml::IListViewBase>()?
                        .SetSelectionMode(Xaml::ListViewSelectionMode(*v))?;
                } else if let Ok(tree) = h.cast_inner::<Xaml::TreeView>() {
                    tree.SetSelectionMode(Xaml::TreeViewSelectionMode(*v))?;
                }
            }
            (Prop::Items, PropValue::StrList(items), h) => {
                if let Ok(asb) = h.cast_inner::<Xaml::AutoSuggestBox>() {
                    set_auto_suggest_items(&asb, items)?;
                } else {
                    set_string_items(h, items)?;
                }
            }
            (Prop::Items, PropValue::SelectorBarItems(items), Handle::SelectorBar(selector)) => {
                selector.set_items(items, None)?;
            }
            (Prop::AutoSuggestItems, PropValue::StrList(items), Handle::AutoSuggestBox(asb)) => {
                set_auto_suggest_items(asb, items)?;
            }
            (Prop::AutoSuggestPlaceholder, PropValue::Str(s), Handle::AutoSuggestBox(asb)) => {
                asb.SetPlaceholderText(s)?;
            }
            (Prop::IsPaneOpen, PropValue::Bool(v), h) => {
                if let Ok(split) = h.cast_inner::<Xaml::SplitView>() {
                    split.SetIsPaneOpen(*v)?;
                }
            }
            (Prop::IsPaneToggleButtonVisible, PropValue::Bool(v), Handle::TitleBar(tb)) => {
                tb.set_pane_toggle_button_visible(*v)?;
            }
            (Prop::IsBackButtonVisible, PropValue::Bool(v), Handle::TitleBar(tb)) => {
                tb.set_back_button_visible(*v)?;
            }
            (Prop::IsBackButtonEnabled, PropValue::Bool(v), Handle::TitleBar(tb)) => {
                tb.set_back_button_enabled(*v)?;
            }
            (Prop::Title, PropValue::Str(s), Handle::TitleBar(tb)) => {
                tb.set_title(s)?;
            }
            (Prop::Subtitle, PropValue::Str(s), Handle::TitleBar(tb)) => {
                tb.set_subtitle(s)?;
            }
            (Prop::Tall, PropValue::Bool(v), Handle::TitleBar(tb)) => {
                tb.set_tall(*v)?;
            }
            (Prop::Fill, PropValue::Brush(b), h) => {
                if let Ok(shape) = h.cast_inner::<Xaml::Shape>() {
                    shape.SetFill(&xaml_brush(b)?)?;
                }
            }
            (Prop::Stroke, PropValue::Brush(b), h) => {
                if let Ok(shape) = h.cast_inner::<Xaml::Shape>() {
                    shape.SetStroke(&xaml_brush(b)?)?;
                }
            }
            (Prop::StrokeThickness, PropValue::F64(v), h) => {
                if let Ok(shape) = h.cast_inner::<Xaml::Shape>() {
                    shape.SetStrokeThickness(*v)?;
                }
            }
            (Prop::LineEndpoints, PropValue::LineEndpoints(line), Handle::Line(l)) => {
                l.SetX1(line.x1)?;
                l.SetY1(line.y1)?;
                l.SetX2(line.x2)?;
                l.SetY2(line.y2)?;
            }
            (Prop::CornerRadius, PropValue::F64(v), Handle::Rectangle(r)) => {
                r.SetRadiusX(*v)?;
                r.SetRadiusY(*v)?;
            }
            (Prop::DisplayName, PropValue::Str(s), Handle::PersonPicture(p)) => {
                p.SetDisplayName(s)?;
            }
            (Prop::Initials, PropValue::Str(s), Handle::PersonPicture(p)) => {
                p.SetInitials(s)?;
            }
            (Prop::IsReadOnly, PropValue::Bool(v), Handle::RichEditBox(reb)) => {
                reb.SetIsReadOnly(*v)?;
            }
            (Prop::AcceptsReturn, PropValue::Bool(v), Handle::TextBox(tb)) => {
                tb.SetAcceptsReturn(*v)?;
            }
            (Prop::PasswordRevealMode, PropValue::I32(v), Handle::PasswordBox(pb)) => {
                pb.cast::<Xaml::IPasswordBox3>()?
                    .SetPasswordRevealMode(Xaml::PasswordRevealMode(*v))?;
            }
            (Prop::IsPasswordRevealButtonEnabled, PropValue::Bool(v), Handle::PasswordBox(pb)) => {
                pb.SetIsPasswordRevealButtonEnabled(*v)?;
            }
            (Prop::DayVisible, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                dp.SetDayVisible(*v)?;
            }
            (Prop::MonthVisible, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                dp.SetMonthVisible(*v)?;
            }
            (Prop::YearVisible, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                dp.SetYearVisible(*v)?;
            }
            (Prop::ClockIdentifier, PropValue::Str(s), Handle::TimePicker(tp)) => {
                tp.SetClockIdentifier(s)?;
            }
            (Prop::MinuteIncrement, PropValue::I32(v), Handle::TimePicker(tp)) => {
                tp.SetMinuteIncrement(*v)?;
            }
            (Prop::IsCalendarOpen, PropValue::Bool(v), Handle::CalendarDatePicker(cdp)) => {
                cdp.SetIsCalendarOpen(*v)?;
            }
            (Prop::IsTodayHighlighted, PropValue::Bool(v), h) => {
                if let Ok(cdp) = h.cast_inner::<Xaml::CalendarDatePicker>() {
                    cdp.SetIsTodayHighlighted(*v)?;
                } else if let Ok(cv) = h.cast_inner::<Xaml::CalendarView>() {
                    cv.SetIsTodayHighlighted(*v)?;
                }
            }
            (Prop::IsGroupLabelVisible, PropValue::Bool(v), Handle::CalendarView(cv)) => {
                cv.SetIsGroupLabelVisible(*v)?;
            }
            (Prop::Delay, PropValue::I32(v), Handle::RepeatButton(rb)) => {
                rb.SetDelay(*v)?;
            }
            (Prop::Interval, PropValue::I32(v), Handle::RepeatButton(rb)) => {
                rb.SetInterval(*v)?;
            }
            (Prop::OpenPaneLength, PropValue::F64(v), Handle::SplitView(sv)) => {
                sv.SetOpenPaneLength(*v)?;
            }
            (Prop::CompactPaneLength, PropValue::F64(v), Handle::SplitView(sv)) => {
                sv.SetCompactPaneLength(*v)?;
            }
            (Prop::DisplayMode, PropValue::I32(v), Handle::SplitView(sv)) => {
                sv.SetDisplayMode(Xaml::SplitViewDisplayMode(*v))?;
            }
            (Prop::HorizontalScrollBarVisibility, PropValue::I32(v), h) => {
                if let Ok(sv) = h.cast_inner::<Xaml::ScrollViewer>() {
                    sv.SetHorizontalScrollBarVisibility(Xaml::ScrollBarVisibility(*v))?;
                }
            }
            (Prop::VerticalScrollBarVisibility, PropValue::I32(v), h) => {
                if let Ok(sv) = h.cast_inner::<Xaml::ScrollViewer>() {
                    sv.SetVerticalScrollBarVisibility(Xaml::ScrollBarVisibility(*v))?;
                }
            }
            _ => {}
        }

        Ok(())
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
        let _ = self.set_prop_inner(id, prop, value);
    }

    fn append_child(&mut self, parent: ControlId, child: ControlId) {
        let index = self
            .parent_children
            .borrow()
            .get(&parent)
            .map_or(0, Vec::len);
        self.insert_child(parent, index, child);
    }

    fn remove_child(&mut self, parent: ControlId, index: usize) {
        let visual = self.visual_index(parent, index);
        if let Some(list) = self.parent_children.borrow_mut().get_mut(&parent)
            && index < list.len()
        {
            list.remove(index);
        }
        self.visual_remove_at(parent, visual);
    }

    fn replace_child(&mut self, parent: ControlId, index: usize, new: ControlId) {
        let visual = self.visual_index(parent, index);
        if let Some(list) = self.parent_children.borrow_mut().get_mut(&parent)
            && index < list.len()
        {
            list[index] = new;
        }
        self.visual_set_at(parent, visual, new);
    }

    fn move_child(&mut self, parent: ControlId, from: usize, to: usize) {
        let child = {
            let mut parents = self.parent_children.borrow_mut();
            let Some(list) = parents.get_mut(&parent) else {
                return;
            };
            if from >= list.len() || to >= list.len() {
                return;
            }
            let child = list.remove(from);
            list.insert(to, child);
            child
        };
        self.visual_remove_at(parent, from);
        self.visual_insert_at(parent, to, child);
    }

    fn insert_child(&mut self, parent: ControlId, index: usize, child: ControlId) {
        let visual = self.visual_index(parent, index);
        self.parent_children
            .borrow_mut()
            .entry(parent)
            .or_default()
            .insert(index, child);
        self.visual_insert_at(parent, visual, child);
    }

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
    }

    fn attach_event(&mut self, id: ControlId, event: Event, handler: EventHandler) {
        self.detach_event(id, event);
        let mut revokers = Vec::new();
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };

        if let Some(revokers) = generated::dispatch_attach_event(handle, event, &handler) {
            if !revokers.is_empty() {
                self.event_revokers
                    .borrow_mut()
                    .insert((id, event), revokers);
            }
            return;
        }

        match (event, handle) {
            (Event::Click, h) => {
                if let Ok(button) = h.cast_inner::<Xaml::ButtonBase>() {
                    revokers.push(
                        button
                            .Click(move |_, _| {
                                handler.invoke();
                            })
                            .unwrap(),
                    );
                }
            }
            (Event::Checked, h) => {
                if let Ok(toggle) = h.cast_inner::<Xaml::ToggleButton>() {
                    let h1 = handler.clone();
                    revokers.push(
                        toggle
                            .Checked(move |_, _| {
                                h1.invoke_bool(true);
                            })
                            .unwrap(),
                    );
                    revokers.push(
                        toggle
                            .Unchecked(move |_, _| {
                                handler.invoke_bool(false);
                            })
                            .unwrap(),
                    );
                }
            }
            (Event::Toggled, Handle::ToggleSwitch(ts)) => {
                let ts_for_cb = ts.clone();
                revokers.push(
                    ts.Toggled(move |_, _| {
                        handler.invoke_bool(ts_for_cb.IsOn().unwrap_or(false));
                    })
                    .unwrap(),
                );
            }
            (Event::TextChanged, Handle::TextBox(tb)) => {
                revokers.push(
                    tb.TextChanged(move |sender, _| {
                        let text = sender
                            .as_ref()
                            .and_then(|s| s.cast::<Xaml::TextBox>().ok())
                            .and_then(|t| t.Text().ok())
                            .unwrap_or_default();
                        handler.invoke_string(text);
                    })
                    .unwrap(),
                );
            }
            (Event::TextChanged, Handle::AutoSuggestBox(asb)) => {
                revokers.push(
                    asb.TextChanged(move |sender, args| {
                        let is_user_input = args
                            .as_ref()
                            .and_then(|a| a.Reason().ok())
                            .is_some_and(|reason| {
                                reason == Xaml::AutoSuggestionBoxTextChangeReason::UserInput
                            });
                        if is_user_input {
                            let text = sender
                                .as_ref()
                                .and_then(|t| t.Text().ok())
                                .unwrap_or_default();
                            handler.invoke_string(text);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::QuerySubmitted, Handle::AutoSuggestBox(asb)) => {
                revokers.push(
                    asb.cast::<Xaml::IAutoSuggestBox2>()
                        .unwrap()
                        .QuerySubmitted(move |_, args| {
                            let text = args
                                .as_ref()
                                .and_then(|a| a.QueryText().ok())
                                .unwrap_or_default();
                            handler.invoke_string(text);
                        })
                        .unwrap(),
                );
            }
            (Event::SuggestionChosen, Handle::AutoSuggestBox(asb)) => {
                revokers.push(
                    asb.SuggestionChosen(move |_, args| {
                        let text = args
                            .as_ref()
                            .and_then(|a| a.SelectedItem().ok())
                            .and_then(inspectable_to_string)
                            .unwrap_or_default();
                        handler.invoke_string(text);
                    })
                    .unwrap(),
                );
            }
            (Event::ColorChanged, Handle::ColorPicker(cp)) => {
                revokers.push(
                    cp.ColorChanged(move |_, args| {
                        let color =
                            args.as_ref()
                                .and_then(|a| a.NewColor().ok())
                                .unwrap_or(Xaml::Color {
                                    a: 255,
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                });
                        handler.invoke_color((color.a, color.r, color.g, color.b));
                    })
                    .unwrap(),
                );
            }
            (Event::SelectedDateChanged, Handle::DatePicker(dp)) => {
                revokers.push(
                    dp.DateChanged(move |_, args| {
                        if let Some(a) = args.as_ref()
                            && let Ok(dt) = a.NewDate()
                        {
                            handler.invoke_datetime(dt);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::SelectedTimeChanged, Handle::TimePicker(tp)) => {
                revokers.push(
                    tp.TimeChanged(move |_, args| {
                        if let Some(a) = args.as_ref()
                            && let Ok(ts) = a.NewTime()
                        {
                            handler.invoke_timespan(ts);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::SelectionChanged, h) => {
                if let Ok(selector) = h.cast_inner::<Xaml::Selector>() {
                    let selector_for_cb = selector.clone();
                    revokers.push(
                        selector
                            .SelectionChanged(move |_, _| {
                                handler.invoke_i32(selector_for_cb.SelectedIndex().unwrap_or(-1));
                            })
                            .unwrap(),
                    );
                } else if let Handle::SelectorBar(selector) = h {
                    let _ = selector.set_selection_changed_handler(Some(handler));
                }
            }
            (Event::ValueChanged, h) => {
                if let Ok(range) = h.cast_inner::<Xaml::RangeBase>() {
                    revokers.push(
                        range
                            .ValueChanged(move |_, args| {
                                let value = args
                                    .as_ref()
                                    .and_then(|a| a.NewValue().ok())
                                    .unwrap_or_default();
                                handler.invoke_f64(value);
                            })
                            .unwrap(),
                    );
                } else if let Ok(rating) = h.cast_inner::<Xaml::RatingControl>() {
                    revokers.push(
                        rating
                            .ValueChanged(move |sender, _| {
                                let value = sender
                                    .as_ref()
                                    .and_then(|r| r.Value().ok())
                                    .unwrap_or_default();
                                handler.invoke_f64(value);
                            })
                            .unwrap(),
                    );
                }
            }
            (Event::BackRequested, Handle::TitleBar(tb)) => {
                if let Ok(subscription) = tb.add_back_requested(handler) {
                    revokers.push(subscription);
                }
            }
            (Event::PaneToggleRequested, Handle::TitleBar(tb)) => {
                if let Ok(subscription) = tb.add_pane_toggle_requested(handler) {
                    revokers.push(subscription);
                }
            }
            (Event::PasswordChanged, Handle::PasswordBox(pb)) => {
                revokers.push(
                    pb.PasswordChanged(move |sender, _| {
                        let text = sender
                            .as_ref()
                            .and_then(|s| s.cast::<Xaml::PasswordBox>().ok())
                            .and_then(|p| p.Password().ok())
                            .unwrap_or_default();
                        handler.invoke_string(text);
                    })
                    .unwrap(),
                );
            }
            (Event::ItemInvoked, Handle::TreeView(tv)) => {
                revokers.push(
                    tv.ItemInvoked(move |_, args| {
                        let text = args
                            .as_ref()
                            .and_then(|a| a.InvokedItem().ok())
                            .and_then(|insp| {
                                insp.cast::<Xaml::ITreeViewNode>()
                                    .ok()
                                    .and_then(|node| node.Content().ok())
                            })
                            .and_then(inspectable_to_string)
                            .unwrap_or_default();
                        handler.invoke_string(text);
                    })
                    .unwrap(),
                );
            }
            _ => {}
        }

        if !revokers.is_empty() {
            self.event_revokers
                .borrow_mut()
                .insert((id, event), revokers);
        }
    }

    fn detach_event(&mut self, id: ControlId, event: Event) {
        self.event_revokers.borrow_mut().remove(&(id, event));
        if event == Event::SelectionChanged {
            if let Some(Handle::SelectorBar(selector)) = self.controls.borrow().get(&id) {
                let _ = selector.set_selection_changed_handler(None);
            }
        }
    }

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

    fn get_native_element(&self, id: ControlId) -> Option<IInspectable> {
        self.get_ui_element(id)
    }
}

enum Handle {
    AutoSuggestBox(Xaml::AutoSuggestBox),
    Border(Xaml::Border),
    Button(Xaml::Button),
    CalendarDatePicker(Xaml::CalendarDatePicker),
    CalendarView(Xaml::CalendarView),
    Canvas(Xaml::Canvas),
    CheckBox(Xaml::CheckBox),
    ColorPicker(Xaml::ColorPicker),
    ComboBox(Xaml::ComboBox),
    CommandBar(Xaml::CommandBar),
    ContentDialog(Xaml::ContentDialog),
    DatePicker(Xaml::DatePicker),
    DropDownButton(Xaml::DropDownButton),
    Grid(Xaml::Grid),
    HyperlinkButton(Xaml::HyperlinkButton),
    Image(Xaml::Image),
    ListBox(Xaml::ListBox),
    MenuBar(Xaml::MenuBar),
    PasswordBox(Xaml::PasswordBox),
    PersonPicture(Xaml::PersonPicture),
    Pivot(Xaml::Pivot),
    PivotItem(Xaml::PivotItem),
    ProgressBar(Xaml::ProgressBar),
    ProgressRing(Xaml::ProgressRing),
    RadioButton(Xaml::RadioButton),
    RatingControl(Xaml::RatingControl),
    Rectangle(Xaml::Rectangle),
    RelativePanel(Xaml::RelativePanel),
    RepeatButton(Xaml::RepeatButton),
    RichEditBox(Xaml::RichEditBox),
    RichTextBlock(Xaml::RichTextBlock),
    ScrollView(Xaml::ScrollViewer),
    ScrollViewer(Xaml::ScrollViewer),
    Slider(Xaml::Slider),
    SplitButton(Xaml::SplitButton),
    SplitView(Xaml::SplitView),
    StackPanel(Xaml::StackPanel),
    TextBlock(Xaml::TextBlock),
    TextBox(Xaml::TextBox),
    TitleBar(IslandTitleBar),
    TimePicker(Xaml::TimePicker),
    ToggleButton(Xaml::ToggleButton),
    ToggleSwitch(Xaml::ToggleSwitch),
    TreeView(Xaml::TreeView),
    Viewbox(Xaml::Viewbox),
    Ellipse(Xaml::Ellipse),
    Line(Xaml::Line),
    ListView(Xaml::ListView),
    GridView(Xaml::GridView),
    FlipView(Xaml::FlipView),
    SelectorBar(IslandSelectorBar),
}

impl Handle {
    fn new(kind: ControlKind) -> Result<Self> {
        Ok(match kind {
            ControlKind::AutoSuggestBox => Self::AutoSuggestBox(Xaml::AutoSuggestBox::new()?),
            ControlKind::Border => Self::Border(Xaml::Border::new()?),
            ControlKind::Button => Self::Button(Xaml::Button::new()?),
            ControlKind::CalendarDatePicker => {
                Self::CalendarDatePicker(Xaml::CalendarDatePicker::new()?)
            }
            ControlKind::CalendarView => Self::CalendarView(Xaml::CalendarView::new()?),
            ControlKind::Canvas => Self::Canvas(Xaml::Canvas::new()?),
            ControlKind::CheckBox => Self::CheckBox(Xaml::CheckBox::new()?),
            ControlKind::ColorPicker => Self::ColorPicker(Xaml::ColorPicker::new()?),
            ControlKind::ComboBox => Self::ComboBox(Xaml::ComboBox::new()?),
            ControlKind::CommandBar => Self::CommandBar(Xaml::CommandBar::new()?),
            ControlKind::ContentDialog => Self::ContentDialog(Xaml::ContentDialog::new()?),
            ControlKind::DatePicker => Self::DatePicker(Xaml::DatePicker::new()?),
            ControlKind::DropDownButton => Self::DropDownButton(Xaml::DropDownButton::new()?),
            ControlKind::Grid => Self::Grid(Xaml::Grid::new()?),
            ControlKind::HyperlinkButton => Self::HyperlinkButton(Xaml::HyperlinkButton::new()?),
            ControlKind::Image => Self::Image(Xaml::Image::new()?),
            ControlKind::ListBox => Self::ListBox(Xaml::ListBox::new()?),
            ControlKind::MenuBar => Self::MenuBar(Xaml::MenuBar::new()?),
            ControlKind::PasswordBox => Self::PasswordBox(Xaml::PasswordBox::new()?),
            ControlKind::PersonPicture => Self::PersonPicture(Xaml::PersonPicture::new()?),
            ControlKind::Pivot => Self::Pivot(Xaml::Pivot::new()?),
            ControlKind::PivotItem => Self::PivotItem(Xaml::PivotItem::new()?),
            ControlKind::ProgressBar => Self::ProgressBar(Xaml::ProgressBar::new()?),
            ControlKind::ProgressRing => Self::ProgressRing(Xaml::ProgressRing::new()?),
            ControlKind::RadioButton => Self::RadioButton(Xaml::RadioButton::new()?),
            ControlKind::RatingControl => Self::RatingControl(Xaml::RatingControl::new()?),
            ControlKind::Rectangle => Self::Rectangle(Xaml::Rectangle::new()?),
            ControlKind::RelativePanel => Self::RelativePanel(Xaml::RelativePanel::new()?),
            ControlKind::RepeatButton => Self::RepeatButton(Xaml::RepeatButton::new()?),
            ControlKind::RichEditBox => Self::RichEditBox(Xaml::RichEditBox::new()?),
            ControlKind::RichTextBlock => Self::RichTextBlock(Xaml::RichTextBlock::new()?),
            ControlKind::ScrollView => Self::ScrollView(Xaml::ScrollViewer::new()?),
            ControlKind::ScrollViewer => Self::ScrollViewer(Xaml::ScrollViewer::new()?),
            ControlKind::Slider => Self::Slider(Xaml::Slider::new()?),
            ControlKind::SplitButton => Self::SplitButton(Xaml::SplitButton::new()?),
            ControlKind::SplitView => Self::SplitView(Xaml::SplitView::new()?),
            ControlKind::StackPanel => Self::StackPanel(Xaml::StackPanel::new()?),
            ControlKind::TextBlock => Self::TextBlock(Xaml::TextBlock::new()?),
            ControlKind::TextBox => Self::TextBox(Xaml::TextBox::new()?),
            ControlKind::TitleBar => Self::TitleBar(IslandTitleBar::new()?),
            ControlKind::TimePicker => Self::TimePicker(Xaml::TimePicker::new()?),
            ControlKind::ToggleButton => Self::ToggleButton(Xaml::ToggleButton::new()?),
            ControlKind::ToggleSwitch => Self::ToggleSwitch(Xaml::ToggleSwitch::new()?),
            ControlKind::TreeView => Self::TreeView(Xaml::TreeView::new()?),
            ControlKind::Viewbox => Self::Viewbox(Xaml::Viewbox::new()?),
            ControlKind::Ellipse => Self::Ellipse(Xaml::Ellipse::new()?),
            ControlKind::Line => Self::Line(Xaml::Line::new()?),
            ControlKind::ListView => Self::ListView(Xaml::ListView::new()?),
            ControlKind::GridView => Self::GridView(Xaml::GridView::new()?),
            ControlKind::FlipView => Self::FlipView(Xaml::FlipView::new()?),
            ControlKind::SelectorBar => Self::SelectorBar(IslandSelectorBar::new()?),
        })
    }

    fn cast_inner<T: Interface>(&self) -> Result<T> {
        match self {
            Self::AutoSuggestBox(v) => v.cast(),
            Self::Border(v) => v.cast(),
            Self::Button(v) => v.cast(),
            Self::CalendarDatePicker(v) => v.cast(),
            Self::CalendarView(v) => v.cast(),
            Self::Canvas(v) => v.cast(),
            Self::CheckBox(v) => v.cast(),
            Self::ColorPicker(v) => v.cast(),
            Self::ComboBox(v) => v.cast(),
            Self::CommandBar(v) => v.cast(),
            Self::ContentDialog(v) => v.cast(),
            Self::DatePicker(v) => v.cast(),
            Self::DropDownButton(v) => v.cast(),
            Self::Grid(v) => v.cast(),
            Self::HyperlinkButton(v) => v.cast(),
            Self::Image(v) => v.cast(),
            Self::ListBox(v) => v.cast(),
            Self::MenuBar(v) => v.cast(),
            Self::PasswordBox(v) => v.cast(),
            Self::PersonPicture(v) => v.cast(),
            Self::Pivot(v) => v.cast(),
            Self::PivotItem(v) => v.cast(),
            Self::ProgressBar(v) => v.cast(),
            Self::ProgressRing(v) => v.cast(),
            Self::RadioButton(v) => v.cast(),
            Self::RatingControl(v) => v.cast(),
            Self::Rectangle(v) => v.cast(),
            Self::RelativePanel(v) => v.cast(),
            Self::RepeatButton(v) => v.cast(),
            Self::RichEditBox(v) => v.cast(),
            Self::RichTextBlock(v) => v.cast(),
            Self::ScrollView(v) => v.cast(),
            Self::ScrollViewer(v) => v.cast(),
            Self::Slider(v) => v.cast(),
            Self::SplitButton(v) => v.cast(),
            Self::SplitView(v) => v.cast(),
            Self::StackPanel(v) => v.cast(),
            Self::TextBlock(v) => v.cast(),
            Self::TextBox(v) => v.cast(),
            Self::TitleBar(v) => v.as_ui_element()?.cast(),
            Self::TimePicker(v) => v.cast(),
            Self::ToggleButton(v) => v.cast(),
            Self::ToggleSwitch(v) => v.cast(),
            Self::TreeView(v) => v.cast(),
            Self::Viewbox(v) => v.cast(),
            Self::Ellipse(v) => v.cast(),
            Self::Line(v) => v.cast(),
            Self::ListView(v) => v.cast(),
            Self::GridView(v) => v.cast(),
            Self::FlipView(v) => v.cast(),
            Self::SelectorBar(v) => v.as_ui_element()?.cast(),
        }
    }

    fn as_ui_element(&self) -> Result<Xaml::UIElement> {
        self.cast_inner()
    }

    fn as_framework_element(&self) -> Result<Xaml::FrameworkElement> {
        self.cast_inner()
    }
}

type EventSubscription = windows_core::EventRevoker;

#[derive(Clone)]
pub(crate) struct IslandSelectorBar {
    root: Xaml::StackPanel,
    items: Rc<RefCell<Vec<SelectorBarItemDef>>>,
    buttons: Rc<RefCell<Vec<Xaml::ToggleButton>>>,
    selected_index: Rc<Cell<Option<usize>>>,
    handler: Rc<RefCell<Option<EventHandler>>>,
    item_revokers: Rc<RefCell<Vec<EventSubscription>>>,
}

impl IslandSelectorBar {
    fn new() -> Result<Self> {
        let root = Xaml::StackPanel::new()?;
        root.SetOrientation(Xaml::Orientation::Horizontal)?;
        if let Ok(spacing) = root.cast::<Xaml::IStackPanel4>() {
            let _ = spacing.SetSpacing(4.0);
        }
        Ok(Self {
            root,
            items: Rc::default(),
            buttons: Rc::default(),
            selected_index: Rc::default(),
            handler: Rc::default(),
            item_revokers: Rc::default(),
        })
    }

    fn as_ui_element(&self) -> Result<Xaml::UIElement> {
        self.root.cast()
    }

    fn set_items(&self, items: &[SelectorBarItemDef], handler: Option<EventHandler>) -> Result<()> {
        if let Some(handler) = handler {
            self.handler.replace(Some(handler));
        }
        self.items.replace(items.to_vec());
        self.selected_index.set((!items.is_empty()).then_some(0));
        self.item_revokers.borrow_mut().clear();
        self.buttons.borrow_mut().clear();

        let panel = self.root.cast::<Xaml::Panel>()?;
        let children = panel.Children()?;
        children.Clear()?;

        let mut buttons = Vec::with_capacity(items.len());
        for (index, item) in items.iter().enumerate() {
            let button = self.build_button(item, index == 0)?;
            let ui: Xaml::UIElement = button.cast()?;
            children.Append(&ui)?;
            buttons.push(button);
        }
        self.buttons.replace(buttons);
        self.wire_selection_changed()
    }

    fn set_selection_changed_handler(&self, handler: Option<EventHandler>) -> Result<()> {
        self.handler.replace(handler);
        self.wire_selection_changed()
    }

    fn build_button(
        &self,
        item: &SelectorBarItemDef,
        selected: bool,
    ) -> Result<Xaml::ToggleButton> {
        let button = Xaml::ToggleButton::new()?;
        button.SetIsChecked(Some(selected))?;
        let _ = button
            .cast::<Xaml::IControl>()?
            .SetPadding(xaml_thickness(Thickness::xy(12.0, 6.0)));

        let content: IInspectable = if let Some(icon) = item.icon {
            let panel = Xaml::StackPanel::new()?;
            panel.SetOrientation(Xaml::Orientation::Horizontal)?;
            if let Ok(spacing) = panel.cast::<Xaml::IStackPanel4>() {
                let _ = spacing.SetSpacing(6.0);
            }
            let icon = Xaml::SymbolIcon::CreateInstanceWithSymbol(Xaml::Symbol(icon.0))?;
            append_to_panel(&panel, &icon)?;
            let text = Xaml::TextBlock::new()?;
            text.SetText(&item.text)?;
            append_to_panel(&panel, &text)?;
            panel.cast()?
        } else {
            string_reference(&item.text)
        };
        button
            .cast::<Xaml::ContentControl>()?
            .SetContent(&content)?;
        Ok(button)
    }

    fn wire_selection_changed(&self) -> Result<()> {
        self.item_revokers.borrow_mut().clear();
        let Some(handler) = self.handler.borrow().clone() else {
            return Ok(());
        };

        let mut revokers = Vec::new();
        let buttons = self.buttons.borrow().clone();
        let items = self.items.borrow().clone();
        for (index, button) in buttons.iter().cloned().enumerate() {
            let buttons_for_cb = buttons.clone();
            let selected_index = self.selected_index.clone();
            let selected_text = items
                .get(index)
                .map(|item| item.text.clone())
                .unwrap_or_default();
            let handler = handler.clone();
            let revoker = button.Checked(move |_, _| {
                selected_index.set(Some(index));
                for (other_index, other) in buttons_for_cb.iter().enumerate() {
                    if other_index != index {
                        let _ = other.SetIsChecked(Some(false));
                    }
                }
                handler.invoke_string(selected_text.clone());
            })?;
            revokers.push(revoker);

            let selected_index = self.selected_index.clone();
            let button = button.clone();
            let revoker = button.clone().Unchecked(move |_, _| {
                if selected_index.get() == Some(index) {
                    let _ = button.SetIsChecked(Some(true));
                }
            })?;
            revokers.push(revoker);
        }
        self.item_revokers.replace(revokers);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct TitleBarMetrics {
    pub height_dips: f64,
    pub drag_left_dips: f64,
    pub drag_width_dips: f64,
}

#[derive(Clone, Debug)]
pub(crate) struct TitleBarState {
    pub metrics: TitleBarMetrics,
}

fn titlebar_state_from_handle(handle: &Handle) -> Option<TitleBarState> {
    if let Handle::TitleBar(titlebar) = handle {
        Some(titlebar.state())
    } else {
        None
    }
}

#[derive(Clone)]
pub(crate) struct IslandTitleBar {
    root: Xaml::Grid,
    title: Xaml::TextBlock,
    subtitle: Xaml::TextBlock,
    back_button: Xaml::Button,
    pane_button: Xaml::Button,
    content_host: Xaml::Border,
    footer_host: Xaml::Border,
    _caption_button_revokers: Rc<Vec<EventSubscription>>,
    height_dips: Rc<Cell<f64>>,
    back_button_visible: Rc<Cell<bool>>,
    pane_button_visible: Rc<Cell<bool>>,
}

impl IslandTitleBar {
    const STANDARD_HEIGHT: f64 = 40.0;
    const TALL_HEIGHT: f64 = 48.0;
    const COMMAND_WIDTH: f64 = 40.0;
    const TITLE_AREA_WIDTH: f64 = 180.0;
    const CAPTION_BUTTON_WIDTH: f64 = 46.0;
    const CAPTION_BUTTON_HEIGHT: f64 = 32.0;

    fn new() -> Result<Self> {
        let root = Xaml::Grid::new()?;
        root.cast::<Xaml::IGrid3>()?.SetColumnSpacing(0.0)?;
        let root_fe: Xaml::IFrameworkElement = root.cast()?;
        root_fe.SetHorizontalAlignment(Xaml::HorizontalAlignment::Stretch)?;
        root_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Top)?;

        append_grid_column(&root, grid_length_auto())?;
        append_grid_column(&root, grid_length_pixel(Self::TITLE_AREA_WIDTH))?;
        append_grid_column(&root, grid_length_star())?;
        append_grid_column(&root, grid_length_auto())?;
        append_grid_column(&root, grid_length_pixel(Self::CAPTION_BUTTON_WIDTH * 3.0))?;

        let commands = Xaml::StackPanel::new()?;
        commands
            .cast::<Xaml::IStackPanel>()?
            .SetOrientation(Xaml::Orientation::Horizontal)?;
        let commands_fe: Xaml::IFrameworkElement = commands.cast()?;
        commands_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        place_in_grid(&commands, 0)?;

        let back_button = titlebar_glyph_button("\u{E72B}", Self::COMMAND_WIDTH, 40.0)?;
        set_element_visibility(&back_button, false)?;
        append_to_panel(&commands, &back_button)?;
        let pane_button = titlebar_glyph_button("\u{E700}", Self::COMMAND_WIDTH, 40.0)?;
        set_element_visibility(&pane_button, false)?;
        append_to_panel(&commands, &pane_button)?;
        append_to_panel(&root, &commands)?;

        let heading = Xaml::StackPanel::new()?;
        heading
            .cast::<Xaml::IStackPanel>()?
            .SetOrientation(Xaml::Orientation::Vertical)?;
        let heading_fe: Xaml::IFrameworkElement = heading.cast()?;
        heading_fe.SetWidth(Self::TITLE_AREA_WIDTH)?;
        heading_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;

        let title = Xaml::TextBlock::new()?;
        title.SetFontSize(14.0)?;
        let title_fe: Xaml::IFrameworkElement = title.cast()?;
        title_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        append_to_panel(&heading, &title)?;

        let subtitle = Xaml::TextBlock::new()?;
        subtitle.SetFontSize(11.0)?;
        set_element_visibility(&subtitle, false)?;
        let subtitle_fe: Xaml::IFrameworkElement = subtitle.cast()?;
        subtitle_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        append_to_panel(&heading, &subtitle)?;
        place_in_grid(&heading, 1)?;
        append_to_panel(&root, &heading)?;

        let content_host = Xaml::Border::new()?;
        let content_fe: Xaml::IFrameworkElement = content_host.cast()?;
        content_fe.SetHorizontalAlignment(Xaml::HorizontalAlignment::Stretch)?;
        content_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        place_in_grid(&content_host, 2)?;
        append_to_panel(&root, &content_host)?;

        let footer_host = Xaml::Border::new()?;
        let footer_fe: Xaml::IFrameworkElement = footer_host.cast()?;
        footer_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        place_in_grid(&footer_host, 3)?;
        append_to_panel(&root, &footer_host)?;

        let caption_buttons = Xaml::Grid::new()?;
        for _ in 0..3 {
            append_grid_column(
                &caption_buttons,
                grid_length_pixel(Self::CAPTION_BUTTON_WIDTH),
            )?;
        }
        let caption_fe: Xaml::IFrameworkElement = caption_buttons.cast()?;
        caption_fe.SetHeight(Self::CAPTION_BUTTON_HEIGHT)?;
        caption_fe.SetHorizontalAlignment(Xaml::HorizontalAlignment::Right)?;
        caption_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;
        place_in_grid(&caption_buttons, 4)?;

        let minimize = titlebar_glyph_button(
            "\u{E921}",
            Self::CAPTION_BUTTON_WIDTH,
            Self::CAPTION_BUTTON_HEIGHT,
        )?;
        place_in_grid(&minimize, 0)?;
        append_to_panel(&caption_buttons, &minimize)?;
        let maximize = titlebar_glyph_button(
            "\u{E922}",
            Self::CAPTION_BUTTON_WIDTH,
            Self::CAPTION_BUTTON_HEIGHT,
        )?;
        place_in_grid(&maximize, 1)?;
        append_to_panel(&caption_buttons, &maximize)?;
        let close = titlebar_glyph_button(
            "\u{E8BB}",
            Self::CAPTION_BUTTON_WIDTH,
            Self::CAPTION_BUTTON_HEIGHT,
        )?;
        place_in_grid(&close, 2)?;
        append_to_panel(&caption_buttons, &close)?;
        append_to_panel(&root, &caption_buttons)?;

        let caption_button_revokers = vec![
            minimize
                .cast::<Xaml::ButtonBase>()?
                .Click(|_, _| super::host::minimize_active_window())?,
            maximize
                .cast::<Xaml::ButtonBase>()?
                .Click(|_, _| super::host::toggle_active_window_maximize())?,
            close
                .cast::<Xaml::ButtonBase>()?
                .Click(|_, _| super::host::close_active_window())?,
        ];

        let titlebar = Self {
            root,
            title,
            subtitle,
            back_button,
            pane_button,
            content_host,
            footer_host,
            _caption_button_revokers: Rc::new(caption_button_revokers),
            height_dips: Rc::new(Cell::new(Self::STANDARD_HEIGHT)),
            back_button_visible: Rc::new(Cell::new(false)),
            pane_button_visible: Rc::new(Cell::new(false)),
        };
        titlebar.apply_height(Self::STANDARD_HEIGHT)?;
        Ok(titlebar)
    }

    fn as_ui_element(&self) -> Result<Xaml::UIElement> {
        self.root.cast()
    }

    fn state(&self) -> TitleBarState {
        TitleBarState {
            metrics: self.metrics(),
        }
    }

    fn metrics(&self) -> TitleBarMetrics {
        let command_width = Self::COMMAND_WIDTH;
        let left = (if self.back_button_visible.get() {
            command_width
        } else {
            0.0
        }) + (if self.pane_button_visible.get() {
            command_width
        } else {
            0.0
        });
        TitleBarMetrics {
            height_dips: self.height_dips.get(),
            drag_left_dips: left,
            drag_width_dips: Self::TITLE_AREA_WIDTH,
        }
    }

    fn set_title(&self, value: &str) -> Result<()> {
        self.title.SetText(value)
    }

    fn set_subtitle(&self, value: &str) -> Result<()> {
        self.subtitle.SetText(value)?;
        set_element_visibility(&self.subtitle, !value.is_empty())
    }

    fn set_tall(&self, value: bool) -> Result<()> {
        let height = if value {
            Self::TALL_HEIGHT
        } else {
            Self::STANDARD_HEIGHT
        };
        self.height_dips.set(height);
        self.apply_height(height)
    }

    fn set_back_button_visible(&self, value: bool) -> Result<()> {
        self.back_button_visible.set(value);
        set_element_visibility(&self.back_button, value)
    }

    fn set_back_button_enabled(&self, value: bool) -> Result<()> {
        self.back_button
            .cast::<Xaml::IControl>()?
            .SetIsEnabled(value)
    }

    fn set_pane_toggle_button_visible(&self, value: bool) -> Result<()> {
        self.pane_button_visible.set(value);
        set_element_visibility(&self.pane_button, value)
    }

    fn set_content(&self, content: Option<&Xaml::UIElement>) -> Result<()> {
        match content {
            Some(content) => self.content_host.SetChild(content),
            None => self.content_host.SetChild(Option::<&Xaml::UIElement>::None),
        }
    }

    fn set_footer(&self, content: Option<&Xaml::UIElement>) -> Result<()> {
        match content {
            Some(content) => self.footer_host.SetChild(content),
            None => self.footer_host.SetChild(Option::<&Xaml::UIElement>::None),
        }
    }

    fn add_back_requested(&self, handler: EventHandler) -> Result<EventSubscription> {
        self.back_button
            .cast::<Xaml::ButtonBase>()?
            .Click(move |_, _| handler.invoke())
    }

    fn add_pane_toggle_requested(&self, handler: EventHandler) -> Result<EventSubscription> {
        self.pane_button
            .cast::<Xaml::ButtonBase>()?
            .Click(move |_, _| handler.invoke())
    }

    fn apply_height(&self, height: f64) -> Result<()> {
        self.root
            .cast::<Xaml::IFrameworkElement>()?
            .SetHeight(height)
    }
}

fn set_element_visibility<T: Interface>(element: &T, visible: bool) -> Result<()> {
    element
        .cast::<Xaml::IUIElement>()?
        .SetVisibility(if visible {
            Xaml::Visibility::Visible
        } else {
            Xaml::Visibility::Collapsed
        })
}

fn grid_length_auto() -> Xaml::GridLength {
    Xaml::GridLength {
        value: 0.0,
        grid_unit_type: Xaml::GridUnitType::Auto,
    }
}

fn grid_length_pixel(value: f64) -> Xaml::GridLength {
    Xaml::GridLength {
        value,
        grid_unit_type: Xaml::GridUnitType::Pixel,
    }
}

fn grid_length_star() -> Xaml::GridLength {
    Xaml::GridLength {
        value: 1.0,
        grid_unit_type: Xaml::GridUnitType::Star,
    }
}

fn append_grid_column(grid: &Xaml::Grid, width: Xaml::GridLength) -> Result<()> {
    let column = Xaml::ColumnDefinition::new()?;
    column.SetWidth(width)?;
    grid.ColumnDefinitions()?.Append(&column)
}

fn place_in_grid<T: Interface>(element: &T, column: i32) -> Result<()> {
    let fe: Xaml::FrameworkElement = element.cast()?;
    Xaml::Grid::SetColumn(&fe, column)
}

fn append_to_panel<P: Interface, C: Interface>(parent: &P, child: &C) -> Result<()> {
    let panel: Xaml::Panel = parent.cast()?;
    let child: Xaml::UIElement = child.cast()?;
    panel.Children()?.Append(&child)
}

fn titlebar_glyph_button(glyph: &str, width: f64, height: f64) -> Result<Xaml::Button> {
    let button = Xaml::Button::new()?;
    let fe: Xaml::IFrameworkElement = button.cast()?;
    fe.SetWidth(width)?;
    fe.SetHeight(height)?;
    fe.SetHorizontalAlignment(Xaml::HorizontalAlignment::Left)?;
    fe.SetVerticalAlignment(Xaml::VerticalAlignment::Top)?;

    let control: Xaml::IControl = button.cast()?;
    control.SetPadding(xaml_thickness(Thickness::default()))?;
    let _ = apply_button_style_variant(&button, 2);

    let text = Xaml::TextBlock::new()?;
    text.SetText(glyph)?;
    text.SetFontSize(10.0)?;
    if let Ok(font) = Xaml::FontFamily::CreateInstanceWithName("Segoe MDL2 Assets") {
        let _ = text.SetFontFamily(&font);
    }
    let text_fe: Xaml::IFrameworkElement = text.cast()?;
    text_fe.SetHorizontalAlignment(Xaml::HorizontalAlignment::Center)?;
    text_fe.SetVerticalAlignment(Xaml::VerticalAlignment::Center)?;

    button.cast::<Xaml::ContentControl>()?.SetContent(&text)?;
    Ok(button)
}

/// Build and apply a XAML Style with {ThemeResource} setters to an element.
/// WinUI handles theme-reactive resolution natively.
fn apply_theme_resource_style(handle: &Handle, bindings: &[(Prop, crate::theme::ThemeRef)]) {
    let Some((target_type, fe)) = style_target_for_handle(handle) else {
        return;
    };

    let mut setters = String::new();
    for (prop, theme_ref) in bindings {
        let dp_name = match prop {
            Prop::Background => "Background",
            Prop::Foreground => "Foreground",
            Prop::BorderBrush => "BorderBrush",
            _ => continue,
        };
        let resource_key = theme_ref.resource_key();
        setters.push_str(&format!(
            "<Setter Property='{dp_name}' Value='{{ThemeResource {resource_key}}}'/>"
        ));
    }

    if setters.is_empty() {
        let _ = fe.SetStyle(None);
        return;
    }

    let xaml = format!(
        "<Style xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='{target_type}'>{setters}</Style>"
    );

    match Xaml::XamlReader::Load(&xaml) {
        Ok(obj) => {
            if let Ok(style) = obj.cast::<Xaml::Style>() {
                let _ = fe.SetStyle(None);
                let _ = fe.SetStyle(&style);
            }
        }
        Err(err) => {
            crate::diagnostics::emit(&format!(
                "islands_reactor: theme resource style failed: {err:?}\n"
            ));
        }
    }
}

fn style_target_for_handle(handle: &Handle) -> Option<(&'static str, Xaml::IFrameworkElement)> {
    match handle {
        Handle::Border(v) => v.cast().ok().map(|fe| ("Border", fe)),
        Handle::StackPanel(v) => v.cast().ok().map(|fe| ("StackPanel", fe)),
        Handle::Grid(v) => v.cast().ok().map(|fe| ("Grid", fe)),
        Handle::Canvas(v) => v.cast().ok().map(|fe| ("Canvas", fe)),
        Handle::Button(v) => v.cast().ok().map(|fe| ("Button", fe)),
        Handle::TextBlock(v) => v.cast().ok().map(|fe| ("TextBlock", fe)),
        _ => None,
    }
}

fn xaml_thickness(t: Thickness) -> Xaml::Thickness {
    Xaml::Thickness {
        left: t.left,
        top: t.top,
        right: t.right,
        bottom: t.bottom,
    }
}

fn xaml_color(c: Color) -> Xaml::Color {
    Xaml::Color {
        a: c.a,
        r: c.r,
        g: c.g,
        b: c.b,
    }
}

fn xaml_brush(brush: &Brush) -> Result<Xaml::Brush> {
    match brush {
        Brush::Solid(color) => {
            let b = Xaml::SolidColorBrush::new()?;
            b.SetColor(xaml_color(*color))?;
            b.cast()
        }
    }
}

fn apply_button_style_variant(button: &Xaml::Button, variant: i32) -> Result<()> {
    let fe: Xaml::IFrameworkElement = button.cast()?;
    let style_key = match variant {
        1 => Some("AccentButtonStyle"),
        2 => Some("SubtleButtonStyle"),
        3 => Some("TextBlockButtonStyle"),
        _ => None,
    };
    if let Some(key) = style_key {
        apply_resource_style(&fe, key)?;
    } else {
        fe.SetStyle(None)?;
    }
    Ok(())
}

fn apply_resource_style(fe: &Xaml::IFrameworkElement, resource_key: &str) -> Result<()> {
    let resources = Xaml::Application::Current().and_then(|app| app.Resources())?;
    let key = windows_reference::IReference::from(windows_core::HSTRING::from(resource_key));
    let map = resources
        .cast::<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>>(
        )?;
    let style = map.Lookup(&key)?.cast::<Xaml::Style>()?;
    fe.SetStyle(&style)
}

fn hstring(s: &str) -> windows_core::HSTRING {
    windows_core::HSTRING::from(s)
}

fn string_reference(s: &str) -> IInspectable {
    windows::Foundation::PropertyValue::CreateString(&windows_core::HSTRING::from(s))
        .expect("PropertyValue::CreateString should box string content")
}

fn inspectable_to_string(value: IInspectable) -> Option<String> {
    value
        .cast::<windows::Foundation::IPropertyValue>()
        .ok()
        .and_then(|v| v.GetString().ok())
        .map(|s| s.to_string_lossy())
}

fn set_content_string(handle: &Handle, s: &str) -> Result<()> {
    let tb = Xaml::TextBlock::new()?;
    tb.SetText(s)?;
    if let Ok(cc) = handle.cast_inner::<Xaml::ContentControl>() {
        cc.SetContent(&tb)?;
    } else if let Ok(ddb) = handle.cast_inner::<Xaml::DropDownButton>() {
        ddb.cast::<Xaml::ContentControl>()?
            .SetContent(&string_reference(s))?;
    } else if let Ok(split) = handle.cast_inner::<Xaml::SplitButton>() {
        split
            .cast::<Xaml::ContentControl>()?
            .SetContent(&string_reference(s))?;
    }
    Ok(())
}

fn panel_children_vec(parent: &Handle) -> Option<windows_collections::IVector<Xaml::UIElement>> {
    parent
        .cast_inner::<Xaml::Panel>()
        .ok()?
        .Children()
        .ok()?
        .cast()
        .ok()
}

fn content_control_for(parent: &Handle) -> Option<Xaml::ContentControl> {
    parent.cast_inner().ok()
}

fn items_vector(parent: &Handle) -> Option<windows_collections::IVector<IInspectable>> {
    parent
        .cast_inner::<Xaml::ItemsControl>()
        .ok()?
        .Items()
        .ok()?
        .cast()
        .ok()
}

fn set_string_items(handle: &Handle, items: &[String]) -> Result<()> {
    if let Some(vec) = items_vector(handle) {
        vec.Clear()?;
        for item in items {
            vec.Append(&string_reference(item))?;
        }
    }
    Ok(())
}

fn set_auto_suggest_items(asb: &Xaml::AutoSuggestBox, items: &[String]) -> Result<()> {
    let vec: Vec<Option<IInspectable>> = items
        .iter()
        .map(|item| Some(string_reference(item)))
        .collect();
    let vec: windows_collections::IVector<IInspectable> = vec.into();
    asb.cast::<Xaml::IItemsControl>()?.SetItemsSource(&vec)
}

fn set_grid_rows(grid: &Xaml::Grid, rows: &[GridLength]) -> Result<()> {
    let defs = grid.RowDefinitions()?;
    defs.Clear()?;
    for row in rows {
        let def = Xaml::RowDefinition::new()?;
        def.SetHeight(xaml_grid_length(*row))?;
        defs.Append(&def)?;
    }
    Ok(())
}

fn set_grid_columns(grid: &Xaml::Grid, cols: &[GridLength]) -> Result<()> {
    let defs = grid.ColumnDefinitions()?;
    defs.Clear()?;
    for col in cols {
        let def = Xaml::ColumnDefinition::new()?;
        def.SetWidth(xaml_grid_length(*col))?;
        defs.Append(&def)?;
    }
    Ok(())
}

fn xaml_grid_length(length: GridLength) -> Xaml::GridLength {
    match length {
        GridLength::Auto => Xaml::GridLength {
            value: 0.0,
            grid_unit_type: Xaml::GridUnitType::Auto,
        },
        GridLength::Pixel(v) => Xaml::GridLength {
            value: v,
            grid_unit_type: Xaml::GridUnitType::Pixel,
        },
        GridLength::Star(v) => Xaml::GridLength {
            value: v,
            grid_unit_type: Xaml::GridUnitType::Star,
        },
    }
}

fn set_image_uri(image: &Xaml::Image, uri: &str) -> Result<()> {
    if uri.is_empty() {
        return Ok(());
    }
    let uri = windows::Foundation::Uri::CreateUri(&hstring(uri))?.cast::<Xaml::Uri>()?;
    let bitmap = Xaml::BitmapImage::new()?;
    bitmap.cast::<Xaml::BitmapImage>()?.SetUriSource(&uri)?;
    image.SetSource(&bitmap.cast::<Xaml::ImageSource>()?)
}
