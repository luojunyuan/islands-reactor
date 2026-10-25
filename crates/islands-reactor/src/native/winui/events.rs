macro_rules! islands_event_backend_methods {
    () => {
    fn attach_event(&mut self, id: ControlId, event: Event, handler: EventHandler) {
        self.detach_event(id, event);
        let mut revokers = Vec::new();
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };

        if let Some(revokers) = dispatch_attach_event(handle, event, &handler) {
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
    };
}
