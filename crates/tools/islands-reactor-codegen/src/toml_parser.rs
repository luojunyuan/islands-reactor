/// V2 TOML schema: metadata-driven, concise format.
///
/// Format:
/// ```toml
/// ["Windows.UI.Xaml.Controls.Button"]
/// Content = { type = "Str" }
/// IsEnabled = { default = "true" }
/// Click = {}
/// ```
///
/// Each section key is a full WinUI type name.
/// Each member key is a WinUI property or event name (from metadata).
/// The tool looks up `put_{Name}` or `add_{Name}` in metadata to determine
/// whether it's a property or event, and infers method, value type, invoke
/// pattern, etc.
///
/// Only overrides need to be specified — everything else comes from metadata.
use serde::Deserialize;

use crate::helpers::to_snake_case;
use crate::metadata::MetadataResolver;
use crate::schema::{Control, EnumMapSetter, EventDecl, PropDecl};

/// Reserved key for control-level options (widget name, kind, etc.)
const CONTROL_KEY: &str = "_control";

/// Control-level overrides.
#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
struct ControlMeta {
    /// Rust widget struct name override (e.g. "DatePickerWidget").
    /// Default: short class name (e.g. "DatePicker").
    #[serde(default)]
    widget: Option<String>,
    /// Handle/Kind override. Default: short class name.
    #[serde(default)]
    handle: Option<String>,
    /// Namespace override. Default: extracted from full type name.
    #[serde(default)]
    namespace: Option<String>,
}

/// Override hints for a property or event member.
#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
struct MemberOverride {
    // ── Shared ──
    /// Explicit PropValue variant name (for properties) or
    /// EventHandler variant name (for events: Unit/Bool/Str/F64/I32/Color/DateTime/TimeSpan).
    #[serde(default, rename = "type")]
    value: Option<String>,
    /// Explicit Rust field name (overrides snake_case of key).
    #[serde(default)]
    field: Option<String>,

    // ── Event-specific ──
    /// Skip codegen; hand-written attach_event in backend.
    #[serde(default)]
    manual: Option<bool>,
    /// Property name on sender/args (e.g. "IsOn"); codegen prepends "get_".
    #[serde(default)]
    property: Option<String>,
    /// Complement event for bool-dual patterns (e.g. "Unchecked").
    #[serde(default)]
    false_event: Option<String>,
    /// Generate core bindings only; backend set/event dispatch is hand-written.
    #[serde(default)]
    custom: Option<bool>,
    /// Explicit WinUI enum type for enum-backed I32 properties.
    #[serde(default)]
    winui_type: Option<String>,
    /// Explicit binding module for `winui_type`, e.g. `Xaml` or `Iuxc`.
    #[serde(default)]
    enum_module: Option<String>,
}

/// Parse the v2 TOML and resolve against metadata to produce `Vec<Control>`.
pub fn parse(toml_content: &str, resolver: &MetadataResolver) -> Vec<Control> {
    let table: toml::Table =
        toml::from_str(toml_content).unwrap_or_else(|e| panic!("Failed to parse TOML: {e}"));

    let line_index = LineIndex::new(toml_content);
    let mut controls = Vec::new();

    for (type_name, section) in &table {
        let section = section
            .as_table()
            .unwrap_or_else(|| panic!("expected table for '{type_name}'"));

        let (namespace, short_name) = split_type_name(type_name);

        let meta: ControlMeta = section
            .get(CONTROL_KEY)
            .map(|v| {
                v.clone().try_into().unwrap_or_else(|e| {
                    let line = line_index.find(type_name, CONTROL_KEY);
                    panic!("{line}failed to parse {CONTROL_KEY} for '{type_name}': {e}")
                })
            })
            .unwrap_or_default();

        let widget_name = meta.widget.as_deref().unwrap_or(short_name);
        let handle = meta.handle.as_deref().unwrap_or(short_name);
        let ns = meta.namespace.as_deref().unwrap_or(namespace);

        let mut props = Vec::new();
        let mut events = Vec::new();

        for (member_name, member_value) in section {
            if member_name == CONTROL_KEY {
                continue;
            }
            let line = line_index.find(type_name, member_name);
            assert!(
                !member_name.starts_with('_'),
                "{line}reserved key '{member_name}' in '{type_name}' — only '{CONTROL_KEY}' is supported"
            );

            let overrides: MemberOverride = member_value.clone().try_into().unwrap_or_else(|e| {
                panic!("{line}failed to parse member '{member_name}' in '{type_name}': {e}")
            });

            let is_custom_prop = overrides.custom.unwrap_or(false) && overrides.manual.is_none();
            let is_prop = is_custom_prop
                || resolver.has_method(handle, &format!("put_{member_name}"))
                || resolver.has_method(handle, &format!("get_{member_name}"));
            let is_event =
                !is_custom_prop && resolver.has_method(handle, &format!("add_{member_name}"));

            assert!(
                !(is_prop && is_event),
                "{line}'{member_name}' on '{type_name}' is both a property and an event in metadata — add explicit kind"
            );
            if !is_prop && !is_event {
                if overrides.manual.is_some() {
                    let event = build_event(member_name, handle, &overrides, resolver);
                    events.push(event);
                    continue;
                }
                panic!(
                    "{line}'{member_name}' on '{type_name}' not found in metadata (no put_{member_name} or add_{member_name})"
                );
            }

            if is_prop {
                let prop = build_prop(member_name, handle, &overrides, resolver);
                props.push(prop);
            } else {
                let event = build_event(member_name, handle, &overrides, resolver);
                events.push(event);
            }
        }

        // Build the Control, preserving existing naming conventions.
        let control = Control {
            name: widget_name.to_string(),
            kind: if widget_name != short_name {
                Some(short_name.to_string())
            } else {
                None
            },
            handle: if handle != short_name {
                Some(handle.to_string())
            } else {
                None
            },
            namespace: if ns != "Windows.UI.Xaml.Controls" {
                Some(ns.to_string())
            } else {
                None
            },
            prop: props,
            event: events,
        };

        controls.push(control);
    }

    controls
}

/// Build a PropDecl from metadata + overrides.
fn build_prop(
    member_name: &str,
    handle: &str,
    overrides: &MemberOverride,
    resolver: &MetadataResolver,
) -> PropDecl {
    let field = overrides
        .field
        .clone()
        .unwrap_or_else(|| to_snake_case(member_name));

    let has_explicit_type = overrides.value.is_some();

    let method_name = format!("put_{member_name}");
    let has_method = resolver.has_method(handle, &method_name);
    let custom = overrides.custom.unwrap_or(false);

    let (value, inferred_copy) = if let Some(v) = &overrides.value {
        // Explicit value — check metadata method for copy-ness, falling
        // back to value-name analysis when no method exists.
        let copy = if has_method {
            resolver.is_method_copy(handle, &method_name)
        } else {
            resolver.is_copy_value_name(v)
        };
        (Some(v.clone()), Some(copy))
    } else if has_method && !custom {
        match resolver.infer_value_type(handle, &method_name) {
            Some((name, copy)) => (Some(name), Some(copy)),
            None => (None, None),
        }
    } else {
        (None, None)
    };

    // Check if metadata says this is an enum type (for auto-inference).
    let is_metadata_enum = overrides.winui_type.is_some()
        || (has_method && !custom && resolver.enum_info(handle, &method_name).is_some());

    let (method, method_optional, method_ireference, method_textblock) = if is_metadata_enum {
        (None, None, None, None)
    } else if has_method && !custom {
        classify_setter(handle, &method_name, has_explicit_type, resolver)
    } else {
        (None, None, None, None)
    };

    // Infer enum_map from metadata when the parameter is an enum type.
    let method_enum_map = if let Some(winui_type) = &overrides.winui_type {
        Some(EnumMapSetter {
            method: Some(method_name.clone()),
            module: overrides.enum_module.clone(),
            winui_type: winui_type.clone(),
        })
    } else if has_method && !custom {
        if let Some((enum_name, module, _variants)) = resolver.enum_info(handle, &method_name) {
            Some(EnumMapSetter {
                method: Some(method_name.clone()),
                module: Some(module.to_string()),
                winui_type: enum_name.to_string(),
            })
        } else {
            None
        }
    } else {
        None
    };

    // Infer whether the PropValue variant is Copy.
    // enum_map always wraps Copy types; metadata inference derives
    // copy-ness from the Type variant (ValueName → Copy,
    // String/ClassName → non-Copy); explicit overrides without metadata
    // default to non-Copy (safe — .clone() always works).
    let copy_value = method_enum_map.is_some() || inferred_copy.unwrap_or(false);

    // Enum-map properties are transported as I32 — override any metadata-inferred value.
    let (value, enum_as_i32) = if method_enum_map.is_some() {
        (Some("I32".to_string()), true)
    } else {
        (value, false)
    };

    PropDecl {
        field,
        meta_name: member_name.to_string(),
        value,
        method,
        method_optional,
        method_ireference,
        method_textblock,
        method_enum_map,
        copy_value,
        enum_as_i32,
        custom,
    }
}

/// Classify a property setter method into the correct setter kind.
///
/// When the metadata param type is `Object` (IInspectable), an explicit `type`
/// in the TOML selects IReference wrapping (boxing the value); otherwise the
/// default TextBlock wrapping is used.
fn classify_setter(
    handle: &str,
    method_name: &str,
    has_explicit_type: bool,
    resolver: &MetadataResolver,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    use crate::metadata::ParamClass;

    let param_class = resolver.classify_param(handle, method_name);

    match param_class {
        Some(ParamClass::NullableBool) => (None, Some(method_name.to_string()), None, None),
        Some(ParamClass::IInspectable) => {
            if has_explicit_type {
                (None, None, Some(method_name.to_string()), None)
            } else {
                (None, None, None, Some(method_name.to_string()))
            }
        }
        _ => (Some(method_name.to_string()), None, None, None),
    }
}

/// Build an EventDecl from metadata + overrides.
fn build_event(
    member_name: &str,
    handle: &str,
    overrides: &MemberOverride,
    resolver: &MetadataResolver,
) -> EventDecl {
    let field = overrides
        .field
        .clone()
        .unwrap_or_else(|| format!("on_{}", to_snake_case(member_name)));

    // Does the property live on the sender (control) or on the event args?
    let property_on_sender = overrides.property.as_ref().is_some_and(|prop| {
        let put_method = format!("put_{prop}");
        resolver.has_method(handle, &put_method)
    });

    // Infer value (type) from property metadata when not explicit.
    let value = overrides.value.clone().or_else(|| {
        let prop = overrides.property.as_deref()?;
        if property_on_sender {
            let put_method = format!("put_{prop}");
            resolver
                .infer_value_type(handle, &put_method)
                .map(|(v, _)| v)
        } else {
            let add_event = format!("add_{member_name}");
            resolver.infer_event_args_type(handle, &add_event, prop)
        }
    });

    // Infer invoke pattern.
    let invoke = if overrides.false_event.is_some() {
        Some("invoke_bool_dual".to_string())
    } else if overrides.property.is_some() {
        value.as_deref().and_then(|v| {
            let suffix = if property_on_sender {
                "_getter"
            } else {
                "_args"
            };
            Some(match v {
                "Bool" => format!("invoke_bool{suffix}"),
                "Str" => format!("invoke_string{suffix}"),
                "F64" => format!("invoke_f64{suffix}"),
                "I32" => format!("invoke_i32{suffix}"),
                _ => return None,
            })
        })
    } else {
        None
    };

    let manual = overrides.manual.unwrap_or(false);
    let add_method = if manual {
        None
    } else {
        Some(format!("add_{member_name}"))
    };

    EventDecl {
        field,
        meta_name: member_name.to_string(),
        event: None,
        value,
        manual,
        add_method,
        invoke,
        property: overrides.property.clone(),
        false_event: overrides.false_event.clone(),
    }
}

/// Split "Windows.UI.Xaml.Controls.Button" into ("Windows.UI.Xaml.Controls", "Button").
fn split_type_name(full_name: &str) -> (&str, &str) {
    full_name.rsplit_once('.').unwrap_or_else(|| {
        panic!(
            "type name '{full_name}' must contain a dot (e.g. 'Windows.UI.Xaml.Controls.Button')"
        )
    })
}

/// Convert PascalCase to snake_case.
/// Maps TOML keys to source line numbers for clickable error messages.
struct LineIndex {
    /// (section_header, member_key) → 1-based line number
    entries: std::collections::HashMap<(String, String), usize>,
}

impl LineIndex {
    fn new(source: &str) -> Self {
        let mut entries = std::collections::HashMap::new();
        let mut current_section = String::new();
        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if let Some(inner) = trimmed.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                current_section = inner.trim_matches('"').to_string();
            } else if let Some(key) = trimmed.split('=').next() {
                let key = key.trim();
                if !key.is_empty() && !current_section.is_empty() {
                    entries.insert((current_section.clone(), key.to_string()), i + 1);
                }
            }
        }
        Self { entries }
    }

    /// Returns a prefix like "reactor_widgets.toml:42: " or "" if not found.
    fn find(&self, section: &str, key: &str) -> String {
        self.entries
            .get(&(section.to_string(), key.to_string()))
            .map(|line| format!("reactor_widgets.toml:{line}: "))
            .unwrap_or_default()
    }
}
