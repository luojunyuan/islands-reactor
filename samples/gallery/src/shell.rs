use crate::controls::{CardItem, card_grid};
use crate::registry::{self, CATEGORIES, ControlInfo};
use crate::router;
use islands_reactor::*;

pub fn gallery_shell(cx: &mut RenderCx) -> Element {
    let (nav, set_nav) = cx.use_state((String::from("home"), Vec::<String>::new()));
    let (is_pane_open, set_pane_open) = cx.use_state(true);
    let (search_text, set_search_text) = cx.use_state(String::new());

    let selected_tag = nav.0;
    let history = nav.1;
    let is_dark = matches!(cx.use_color_scheme(), ColorScheme::Dark);

    let category_tags: Vec<String> = CATEGORIES
        .iter()
        .map(|category| registry::category_tag(category))
        .collect();

    let suggestions: Vec<String> = if search_text.is_empty() {
        Vec::new()
    } else {
        registry::search(&search_text)
            .iter()
            .map(|control| control.title.to_string())
            .collect()
    };

    let content: Element = if selected_tag == "home" {
        crate::pages::home::home_page({
            let (set_nav, tag, hist) = (set_nav.clone(), selected_tag.clone(), history.clone());
            move |key| {
                let mut next_history = hist.clone();
                next_history.push(tag.clone());
                set_nav.call((key, next_history));
            }
        })
    } else if selected_tag.eq_ignore_ascii_case("settings") {
        component(crate::pages::settings::settings_page, ())
    } else if category_tags.contains(&selected_tag) {
        let category = CATEGORIES
            .iter()
            .find(|category| registry::category_tag(category) == selected_tag)
            .unwrap_or(&"");
        let controls = registry::controls_in_category(category);
        render_category_page(category, controls, {
            let (set_nav, tag, hist) = (set_nav.clone(), selected_tag.clone(), history.clone());
            move |key| {
                let mut next_history = hist.clone();
                next_history.push(tag.clone());
                set_nav.call((key, next_history));
            }
        })
    } else {
        router::route(&selected_tag)
    };

    let search_box: Element = auto_suggest_box(&*search_text)
        .placeholder_text("Search controls and samples...")
        .items(suggestions)
        .on_text_changed(move |text| set_search_text.call(text))
        .on_query_submitted({
            let (set_nav, tag, hist) = (set_nav.clone(), selected_tag.clone(), history.clone());
            move |query: String| {
                if let Some(info) = registry::ALL_CONTROLS
                    .iter()
                    .find(|control| control.title.eq_ignore_ascii_case(&query))
                {
                    let mut next_history = hist.clone();
                    next_history.push(tag.clone());
                    set_nav.call((info.tag.to_string(), next_history));
                }
            }
        })
        .on_suggestion_chosen({
            let (set_nav, tag, hist) = (set_nav.clone(), selected_tag.clone(), history.clone());
            move |chosen: String| {
                if let Some(info) = registry::ALL_CONTROLS
                    .iter()
                    .find(|control| control.title.eq_ignore_ascii_case(&chosen))
                {
                    let mut next_history = hist.clone();
                    next_history.push(tag.clone());
                    set_nav.call((info.tag.to_string(), next_history));
                }
            }
        })
        .width(320.0)
        .into();

    let theme_button = {
        let glyph = if is_dark { "\u{E706}" } else { "\u{E708}" };
        button(glyph)
            .on_click(move || {
                set_requested_theme(if is_dark {
                    RequestedTheme::Light
                } else {
                    RequestedTheme::Dark
                });
            })
            .font_family("Segoe MDL2 Assets")
            .font_size(14.0)
            .width(40.0)
            .height(36.0)
            .padding(0.0)
    };

    let title_bar = TitleBar::new("Islands Reactor Gallery")
        .subtitle("XAML Islands controls")
        .back_button_visible(!history.is_empty())
        .back_button_enabled(!history.is_empty())
        .pane_toggle_button_visible(true)
        .on_back_requested({
            let (set_nav, hist) = (set_nav.clone(), history.clone());
            move || {
                let mut next_history = hist.clone();
                if let Some(previous) = next_history.pop() {
                    set_nav.call((previous, next_history));
                }
            }
        })
        .on_pane_toggle_requested(move || set_pane_open.call(!is_pane_open))
        .content(search_box)
        .footer(theme_button)
        .tall(true);

    let mut nav_buttons = vec![navigation_button(
        "Home",
        "home",
        &selected_tag,
        &history,
        set_nav.clone(),
    )];
    for category in CATEGORIES {
        nav_buttons.push(navigation_button(
            category,
            registry::category_tag(category),
            &selected_tag,
            &history,
            set_nav.clone(),
        ));
    }
    nav_buttons.push(navigation_button(
        "Settings",
        "settings",
        &selected_tag,
        &history,
        set_nav,
    ));

    let content_area: Element = if is_pane_open {
        let navigation = scroll_view(vstack(nav_buttons).spacing(4.0))
            .vertical_scroll_bar_visibility(ScrollingScrollBarVisibility::Auto)
            .grid_column(0);
        grid((navigation, content.grid_column(1)))
            .columns([GridLength::Pixel(240.0), GridLength::Star(1.0)])
            .into()
    } else {
        content
    };

    grid((title_bar.grid_row(0), content_area.grid_row(1)))
        .rows([GridLength::Auto, GridLength::Star(1.0)])
        .columns([GridLength::Star(1.0)])
        .into()
}

fn navigation_button(
    label: &str,
    target_tag: impl Into<String>,
    selected_tag: &str,
    history: &[String],
    set_nav: SetState<(String, Vec<String>)>,
) -> Element {
    let target_tag = target_tag.into();
    let is_selected = target_tag == selected_tag;
    let current_tag = selected_tag.to_string();
    let previous_history = history.to_vec();
    let button = button(label).on_click(move || {
        let mut next_history = previous_history.clone();
        if target_tag != current_tag {
            next_history.push(current_tag.clone());
            set_nav.call((target_tag.clone(), next_history));
        }
    });

    if is_selected {
        button.accent().into()
    } else {
        button.subtle().into()
    }
}

fn render_category_page(
    category: &'static str,
    controls: Vec<&'static ControlInfo>,
    on_navigate: impl Fn(String) + 'static,
) -> Element {
    let count = controls.len();
    let items: Vec<CardItem> = controls
        .iter()
        .map(|control| CardItem {
            title: control.title.to_string(),
            subtitle: control.description.to_string(),
            image_file: control.image.to_string(),
            key: control.tag.to_string(),
        })
        .collect();

    let root = grid((
        vstack((
            text_block(category).font_size(28.0).bold(),
            text_block(format!("{count} controls")).opacity(0.6),
        ))
        .spacing(4.0)
        .grid_row(0),
        card_grid(&items, on_navigate).grid_row(1),
    ))
    .rows([GridLength::Auto, GridLength::Star(1.0)])
    .columns([GridLength::Star(1.0)])
    .row_spacing(24.0);

    border(root)
        .padding(Thickness {
            left: 36.0,
            top: 40.0,
            right: 36.0,
            bottom: 36.0,
        })
        .into()
}
