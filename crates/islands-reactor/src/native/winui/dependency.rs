impl WinUIBackend {
    fn set_prop_inner(&self, id: ControlId, prop: Prop, value: &PropValue) -> Result<()> {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return Ok(());
        };

        if dispatch_set_prop(handle, prop, value)? {
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

}
