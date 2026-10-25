/// Generates `set_prop` dispatch arms from control declarations.
///
/// Arms that share the same body across multiple Handle variants are merged
/// into OR-pattern arms listing specific handles. This ensures hand-written
/// dispatch arms in mod.rs remain reachable for handles not listed here.
use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::helpers::*;
use crate::metadata::MetadataResolver;
use crate::schema::*;
// ── Intermediate representation ──────────────────────────────────────

/// A match arm descriptor, collected before rendering.
struct ArmDesc {
    /// `Prop` enum variant name (e.g. `"IsEnabled"`).
    prop: String,
    /// `PropValue` variant name (e.g. `"Bool"`) or `"Unset"`.
    value_pat: String,
    /// Handle variant name (e.g. `"Button"`).
    handle: String,
    /// Grouping key — arms with the same key produce identical bodies.
    body_key: String,
    /// How to render the arm body.
    body: Body,
}

/// Describes the body of a set_prop match arm.
enum Body {
    /// `{cast}.{method}({arg})?`
    Cast {
        module: String,
        iface: String,
        method: String,
        value_variant: String,
    },
    /// `{cast}.{method}(Some({arg}))?`
    CastOptional {
        module: String,
        iface: String,
        method: String,
        value_variant: String,
    },
    /// `{cast}.{method}(None)?`
    CastNone {
        module: String,
        iface: String,
        method: String,
    },
    /// `let insp = IReference::from(v.as_str()); {cast}.{method}(&insp)?`
    IReference {
        module: String,
        iface: String,
        method: String,
    },
    /// `let tb = string_as_textblock(v.as_str())?; {cast}.{method}(&tb)?`
    Textblock {
        module: String,
        iface: String,
        method: String,
    },
    /// `{cast}.{method}(Xaml::{winui_type}(*v))?`
    EnumMap {
        module: String,
        iface: String,
        method: String,
        enum_module: String,
        winui_type: String,
    },
}

impl Body {
    /// Grouping key — two arms with the same key have identical bodies.
    fn key(&self) -> String {
        match self {
            Self::Cast {
                module,
                iface,
                method,
                value_variant,
            } => format!(
                "cast.{module}.{iface}.{method}.{}",
                arg_expr_for_value(value_variant)
            ),
            Self::CastOptional {
                module,
                iface,
                method,
                value_variant,
            } => format!(
                "opt.{module}.{iface}.{method}.{}",
                arg_expr_for_value(value_variant)
            ),
            Self::CastNone {
                module,
                iface,
                method,
            } => format!("none.{module}.{iface}.{method}"),
            Self::IReference {
                module,
                iface,
                method,
            } => format!("iref.{module}.{iface}.{method}"),
            Self::Textblock {
                module,
                iface,
                method,
            } => format!("tb.{module}.{iface}.{method}"),
            Self::EnumMap {
                module,
                iface,
                method,
                enum_module,
                winui_type,
            } => {
                format!("enum.{module}.{iface}.{method}.{enum_module}.{winui_type}")
            }
        }
    }
}

// ── Public entry point ───────────────────────────────────────────────

pub fn generate(controls: &[Control], resolver: &MetadataResolver) -> String {
    // Phase 1: collect all arm descriptors.
    let mut all_descs: Vec<ArmDesc> = Vec::new();
    for ctrl in controls {
        for p in &ctrl.prop {
            all_descs.extend(collect_prop_arms(ctrl, p, resolver));
        }
    }

    // Phase 2: group by (prop, value_pat), then sub-group by body_key.
    let mut groups: BTreeMap<(String, String), Vec<ArmDesc>> = BTreeMap::new();
    for desc in all_descs {
        groups
            .entry((desc.prop.clone(), desc.value_pat.clone()))
            .or_default()
            .push(desc);
    }

    // Phase 3: merge duplicate sub-groups into OR-pattern arms.
    let mut all_arms = Vec::new();

    for descs in groups.values() {
        let mut by_body: BTreeMap<&str, Vec<&ArmDesc>> = BTreeMap::new();
        for d in descs {
            by_body.entry(&d.body_key).or_default().push(d);
        }

        for sub in by_body.values() {
            if sub.len() > 1 {
                all_arms.push(render_or_pattern(sub));
            } else {
                all_arms.push(render_arm(sub[0]));
            }
        }
    }

    format_generated(quote! {
        use super::super::*;
        use super::super::convert::string_as_textblock;
        use crate::bindings as Xaml;
        use crate::bindings_iuxc as Iuxc;
        /// Try to handle a `set_prop` call via generated dispatch.
        /// Returns `Ok(true)` if handled, `Ok(false)` to fall through.
        pub(crate) fn dispatch(
            handle: &Handle,
            prop: Prop,
            value: &PropValue,
        ) -> windows_core::Result<bool> {
            match (prop, value, handle) {
                #(#all_arms)*
                _ => return Ok(false),
            }
            Ok(true)
        }
    })
}

// ── Arm collection ───────────────────────────────────────────────────

fn collect_prop_arms(ctrl: &Control, p: &PropDecl, resolver: &MetadataResolver) -> Vec<ArmDesc> {
    let prop = p.prop();
    let handle = ctrl.handle();
    let mut descs = Vec::new();

    match p.setter() {
        SetterKind::Custom => return descs,

        SetterKind::Method { method } => {
            let (module, iface) = resolve_iface(resolver, handle, method);
            descs.push(make_arm(
                prop,
                p.value(),
                handle,
                Body::Cast {
                    module,
                    iface,
                    method: method.to_string(),
                    value_variant: p.value().to_string(),
                },
            ));
        }

        SetterKind::MethodOptional { method } => {
            let (module, iface) = resolve_iface(resolver, handle, method);
            descs.push(make_arm(
                prop,
                p.value(),
                handle,
                Body::CastOptional {
                    module: module.clone(),
                    iface: iface.clone(),
                    method: method.to_string(),
                    value_variant: p.value().to_string(),
                },
            ));
            descs.push(make_arm(
                prop,
                "Unset",
                handle,
                Body::CastNone {
                    module,
                    iface,
                    method: method.to_string(),
                },
            ));
            return descs;
        }

        SetterKind::MethodIReference { method } => {
            let (module, iface) = resolve_iface(resolver, handle, method);
            descs.push(make_arm(
                prop,
                "Str",
                handle,
                Body::IReference {
                    module: module.clone(),
                    iface: iface.clone(),
                    method: method.to_string(),
                },
            ));
            descs.push(make_arm(
                prop,
                "Unset",
                handle,
                Body::CastNone {
                    module,
                    iface,
                    method: method.to_string(),
                },
            ));
            return descs;
        }

        SetterKind::MethodTextblock { method } => {
            let (module, iface) = resolve_iface(resolver, handle, method);
            descs.push(make_arm(
                prop,
                "Str",
                handle,
                Body::Textblock {
                    module: module.clone(),
                    iface: iface.clone(),
                    method: method.to_string(),
                },
            ));
            descs.push(make_arm(
                prop,
                "Unset",
                handle,
                Body::CastNone {
                    module,
                    iface,
                    method: method.to_string(),
                },
            ));
            return descs;
        }

        SetterKind::MethodEnumMap { setter } => {
            let (module, iface) = resolve_iface(resolver, handle, setter.method());
            let enum_module = setter.module.clone().unwrap_or_else(|| module.clone());
            descs.push(make_arm(
                prop,
                "I32",
                handle,
                Body::EnumMap {
                    module,
                    iface,
                    method: setter.method().to_string(),
                    enum_module,
                    winui_type: setter.winui_type.clone(),
                },
            ));
            return descs;
        }
    }

    descs
}

fn make_arm(prop: &str, value_pat: &str, handle: &str, body: Body) -> ArmDesc {
    ArmDesc {
        prop: prop.to_string(),
        value_pat: value_pat.to_string(),
        handle: handle.to_string(),
        body_key: body.key(),
        body,
    }
}

fn resolve_iface(resolver: &MetadataResolver, handle: &str, method: &str) -> (String, String) {
    let iface = resolver.resolve(handle, method).unwrap_or_else(|| {
        panic!(
            "metadata: cannot resolve {handle}.{method} — \
                 mark as setter_fn or check method name"
        )
    });
    (
        iface.module_alias().to_string(),
        iface.short_name().to_string(),
    )
}

// ── Rendering ────────────────────────────────────────────────────────

/// Render a single arm with a specific `Handle::X(h)` pattern.
fn render_arm(desc: &ArmDesc) -> TokenStream {
    let prop_id = ident(&desc.prop);
    let handle_name = Some(desc.handle.as_str());
    let body = render_body(&desc.body, false, handle_name);

    if desc.value_pat == "Unset" {
        let h = ident(&desc.handle);
        quote! {
            (Prop::#prop_id, PropValue::Unset, Handle::#h(h)) => { #body }
        }
    } else {
        let v = ident(&desc.value_pat);
        let h = ident(&desc.handle);
        quote! {
            (Prop::#prop_id, PropValue::#v(v), Handle::#h(h)) => { #body }
        }
    }
}

/// Render merged arms as an OR-pattern over specific handles.
/// Uses `handle.cast_inner()` since individual inner types are not bound.
fn render_or_pattern(descs: &[&ArmDesc]) -> TokenStream {
    let prop_id = ident(&descs[0].prop);
    let body = render_body(&descs[0].body, true, None);

    let mut handles: Vec<&str> = descs.iter().map(|d| d.handle.as_str()).collect();
    handles.sort();
    let handle_pats: Vec<TokenStream> = handles
        .iter()
        .map(|h| {
            let hi = ident(h);
            quote! { Handle::#hi(_) }
        })
        .collect();

    if descs[0].value_pat == "Unset" {
        quote! {
            (Prop::#prop_id, PropValue::Unset, #(#handle_pats)|*) => { #body }
        }
    } else {
        let vp = ident(&descs[0].value_pat);
        quote! {
            (Prop::#prop_id, PropValue::#vp(v), #(#handle_pats)|*) => { #body }
        }
    }
}

/// Generate the cast expression: specific `h.cast` or merged `handle.cast_inner`.
/// When the interface is the default (deref target) for the handle, no cast is needed.
fn cast_tokens(
    module: &str,
    iface: &str,
    wildcard: bool,
    handle_name: Option<&str>,
) -> TokenStream {
    // Default interface for class `Foo` is `IFoo` — Deref handles it.
    if !wildcard && handle_name.is_some_and(|h| iface == format!("I{h}")) {
        return quote! { h };
    }
    let m = ident(module);
    let i = ident(iface);
    if wildcard {
        quote! { handle.cast_inner::<#m::#i>()? }
    } else {
        quote! { h.cast::<#m::#i>()? }
    }
}

/// Render the body of a match arm.
fn render_body(body: &Body, wildcard: bool, handle_name: Option<&str>) -> TokenStream {
    match body {
        Body::Cast {
            module,
            iface,
            method,
            value_variant,
        } => {
            let cast = cast_tokens(module, iface, wildcard, handle_name);
            let m = ident(method);
            let arg = arg_tokens(value_variant);
            quote! { #cast.#m(#arg)?; }
        }
        Body::CastOptional {
            module,
            iface,
            method,
            value_variant,
        } => {
            let cast = cast_tokens(module, iface, wildcard, handle_name);
            let m = ident(method);
            let arg = arg_tokens(value_variant);
            quote! { #cast.#m(Some(#arg))?; }
        }
        Body::CastNone {
            module,
            iface,
            method,
        } => {
            let cast = cast_tokens(module, iface, wildcard, handle_name);
            let m = ident(method);
            quote! { #cast.#m(None)?; }
        }
        Body::IReference {
            module,
            iface,
            method,
        } => {
            let cast = cast_tokens(module, iface, wildcard, handle_name);
            let m = ident(method);
            quote! {
                let insp = windows_reference::IReference::from(v.as_str());
                #cast.#m(&insp)?;
            }
        }
        Body::Textblock {
            module,
            iface,
            method,
        } => {
            let cast = cast_tokens(module, iface, wildcard, handle_name);
            let m = ident(method);
            quote! {
                let tb = string_as_textblock(v.as_str())?;
                #cast.#m(&tb)?;
            }
        }
        Body::EnumMap {
            module,
            iface,
            method,
            enum_module,
            winui_type,
        } => {
            let cast = cast_tokens(module, iface, wildcard, handle_name);
            let module = ident(enum_module);
            let method = ident(method);
            let wt = ident(winui_type);
            quote! {
                #cast.#method(#module::#wt(*v))?;
            }
        }
    }
}

fn arg_tokens(value_variant: &str) -> TokenStream {
    match arg_expr_for_value(value_variant) {
        "str_ref" => quote! { v.as_str() },
        "copy" => quote! { *v },
        _ => quote! { v.clone() },
    }
}
