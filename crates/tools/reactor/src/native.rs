pub(super) mod attach_event {
    /// Generates `attach_event` dispatch arms from control declarations.
    ///
    /// Events with `add_method` + `invoke` are auto-generated. Events with
    /// `manual = true` are skipped; they use hand-written attach_event arms in
    /// the backend.
    use proc_macro2::TokenStream;
    use quote::quote;

    use crate::helpers::*;
    use crate::metadata::MetadataResolver;
    use crate::schema::*;

    /// Returns `h` when the interface is the default for the handle, else casts
    /// through the correct generated bindings module.
    fn event_receiver(module: &str, iface: &proc_macro2::Ident, handle_name: &str) -> TokenStream {
        if iface == &format!("I{handle_name}") {
            quote! { h }
        } else {
            let module = ident(module);
            quote! { h.cast::<#module::#iface>().unwrap() }
        }
    }

    pub fn generate(controls: &[Control], resolver: &MetadataResolver) -> String {
        let mut all_arms = Vec::new();

        for ctrl in controls {
            all_arms.extend(generate_control_event_arms(ctrl, resolver));
        }

        format_generated(quote! {
            use super::*;
            use crate::bindings as Xaml;
            /// Try to handle an `attach_event` call via generated dispatch.
            /// Returns `Some(revokers)` if handled, `None` to fall through.
            pub(super) fn dispatch(
                handle: &Handle,
                event: Event,
                handler: &EventHandler,
            ) -> Option<Vec<windows_core::EventRevoker>> {
                let mut revokers = Vec::new();
                match (event, handle) {
                    #(#all_arms)*
                    _ => return None,
                }
                Some(revokers)
            }
        })
    }

    fn generate_control_event_arms(
        ctrl: &Control,
        resolver: &MetadataResolver,
    ) -> Vec<TokenStream> {
        if ctrl.event.is_empty() {
            return Vec::new();
        }
        let handle = ident(ctrl.handle());
        let handle_module = ctrl.module_alias().unwrap_or("Xaml");
        ctrl.event
            .iter()
            .filter_map(|e| gen_event_arm(&handle, ctrl.handle(), handle_module, e, resolver))
            .collect()
    }

    /// Generate a sender-getter pattern: cast sender to handle type, call getter,
    /// invoke handler.
    fn gen_sender_getter(
        receiver_module: &str,
        handle_module: &str,
        iface: &proc_macro2::Ident,
        add_ident: &proc_macro2::Ident,
        handle_name: &str,
        getter: &str,
        invoke_call: TokenStream,
        default: TokenStream,
    ) -> TokenStream {
        let getter_ident = ident(&bindgen_method_name(getter));
        let handle_ident = ident(handle_name);
        let handle_module = ident(handle_module);
        let receiver = event_receiver(receiver_module, iface, handle_name);
        quote! {
            let handler = handler.clone();
            revokers.push(
                #receiver
                    .#add_ident(move |sender, _args| {
                        let v = sender.as_ref()
                            .and_then(|s| s.cast::<#handle_module::#handle_ident>().ok())
                            .and_then(|s| s.#getter_ident().ok())
                            .unwrap_or(#default);
                        handler.#invoke_call;
                    })
                    .unwrap(),
            );
        }
    }

    /// Generate an args-getter pattern: read value from event args via getter,
    /// invoke handler.
    fn gen_args_getter(
        receiver_module: &str,
        iface: &proc_macro2::Ident,
        add_ident: &proc_macro2::Ident,
        handle_name: &str,
        getter: &str,
        invoke_call: TokenStream,
    ) -> TokenStream {
        let getter_ident = ident(&bindgen_method_name(getter));
        let receiver = event_receiver(receiver_module, iface, handle_name);
        quote! {
            let handler = handler.clone();
            revokers.push(
                #receiver
                    .#add_ident(move |_sender, args| {
                        if let Some(a) = args.as_ref()
                            && let Ok(v) = a.#getter_ident()
                        {
                            handler.#invoke_call;
                        }
                    })
                    .unwrap(),
            );
        }
    }

    fn require_property(e: &EventDecl, handle_name: &str, pattern: &str) -> String {
        let prop = e.property.as_deref().unwrap_or_else(|| {
            panic!(
                "event '{}.{}': {pattern} requires property",
                handle_name,
                e.event()
            )
        });
        format!("get_{prop}")
    }

    fn gen_event_arm(
        handle: &proc_macro2::Ident,
        handle_name: &str,
        handle_module: &str,
        e: &EventDecl,
        resolver: &MetadataResolver,
    ) -> Option<TokenStream> {
        if e.manual {
            return None;
        }

        let event_name = e.event();
        let add_method = e.add_method().unwrap_or_else(|| {
            panic!("event '{handle_name}.{event_name}' has neither manual nor add_method")
        });
        let invoke = e.invoke();

        let event = ident(&event_name);
        let iface_ref = resolver
            .resolve(handle_name, &add_method)
            .unwrap_or_else(|| {
                panic!("metadata: cannot resolve {handle_name}.{add_method} for event")
            });
        let receiver_module = iface_ref.module_alias();
        let iface = ident(iface_ref.short_name());
        let add_ident = ident(&bindgen_method_name(&add_method));

        let handler_body = match invoke {
            "invoke" => {
                let receiver = event_receiver(receiver_module, &iface, handle_name);
                quote! {
                    let handler = handler.clone();
                    revokers.push(
                        #receiver
                            .#add_ident(move |_sender, _args| {
                                handler.invoke();
                            })
                            .unwrap(),
                    );
                }
            }
            "invoke_bool_getter" => {
                let g = require_property(e, handle_name, invoke);
                gen_sender_getter(
                    receiver_module,
                    handle_module,
                    &iface,
                    &add_ident,
                    handle_name,
                    &g,
                    quote!(invoke_bool(v)),
                    quote!(false),
                )
            }
            "invoke_string_getter" => {
                let g = require_property(e, handle_name, invoke);
                gen_sender_getter(
                    receiver_module,
                    handle_module,
                    &iface,
                    &add_ident,
                    handle_name,
                    &g,
                    quote!(invoke_string(v)),
                    quote!(Default::default()),
                )
            }
            "invoke_i32_getter" => {
                let g = require_property(e, handle_name, invoke);
                gen_sender_getter(
                    receiver_module,
                    handle_module,
                    &iface,
                    &add_ident,
                    handle_name,
                    &g,
                    quote!(invoke_i32(v)),
                    quote!(-1),
                )
            }
            "invoke_f64_getter" => {
                let g = require_property(e, handle_name, invoke);
                gen_sender_getter(
                    receiver_module,
                    handle_module,
                    &iface,
                    &add_ident,
                    handle_name,
                    &g,
                    quote!(invoke_f64(v)),
                    quote!(-1.0),
                )
            }
            "invoke_i32_args" => {
                let g = require_property(e, handle_name, invoke);
                gen_args_getter(
                    receiver_module,
                    &iface,
                    &add_ident,
                    handle_name,
                    &g,
                    quote!(invoke_i32(v)),
                )
            }
            "invoke_f64_args" => {
                let g = require_property(e, handle_name, invoke);
                gen_args_getter(
                    receiver_module,
                    &iface,
                    &add_ident,
                    handle_name,
                    &g,
                    quote!(invoke_f64(v)),
                )
            }
            "invoke_bool_dual" => {
                let false_evt = e.false_event.as_deref().unwrap_or_else(|| {
                    panic!(
                        "event '{handle_name}.{event_name}': invoke_bool_dual requires false_event"
                    )
                });
                let add_false = format!("add_{false_evt}");
                let add_false_ident = ident(&bindgen_method_name(&add_false));
                let iface_false_ref =
                    resolver
                        .resolve(handle_name, &add_false)
                        .unwrap_or_else(|| {
                            panic!("metadata: cannot resolve {handle_name}.{add_false} for event")
                        });
                let receiver_false_module = iface_false_ref.module_alias();
                let iface_false = ident(iface_false_ref.short_name());
                let receiver = event_receiver(receiver_module, &iface, handle_name);
                let receiver_false =
                    event_receiver(receiver_false_module, &iface_false, handle_name);
                quote! {
                    let true_handler = handler.clone();
                    revokers.push(
                        #receiver
                            .#add_ident(move |_sender, _args| {
                                true_handler.invoke_bool(true);
                            })
                            .unwrap(),
                    );
                    let false_handler = handler.clone();
                    revokers.push(
                        #receiver_false
                            .#add_false_ident(move |_sender, _args| {
                                false_handler.invoke_bool(false);
                            })
                            .unwrap(),
                    );
                }
            }
            _ => panic!("event '{handle_name}.{event_name}': unknown invoke pattern '{invoke}'"),
        };

        Some(quote! {
            (Event::#event, Handle::#handle(h)) => {
                #handler_body
            }
        })
    }
}
pub(super) mod set_prop {
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
            use super::*;
            use crate::bindings as Xaml;
            /// Try to handle a `set_prop` call via generated dispatch.
            /// Returns `Ok(true)` if handled, `Ok(false)` to fall through.
            pub(super) fn dispatch(
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

    fn collect_prop_arms(
        ctrl: &Control,
        p: &PropDecl,
        resolver: &MetadataResolver,
    ) -> Vec<ArmDesc> {
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
                let m = ident(&bindgen_method_name(method));
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
                let m = ident(&bindgen_method_name(method));
                let arg = arg_tokens(value_variant);
                quote! { #cast.#m(Some(#arg))?; }
            }
            Body::CastNone {
                module,
                iface,
                method,
            } => {
                let cast = cast_tokens(module, iface, wildcard, handle_name);
                let m = ident(&bindgen_method_name(method));
                quote! { #cast.#m(None)?; }
            }
            Body::IReference {
                module,
                iface,
                method,
            } => {
                let cast = cast_tokens(module, iface, wildcard, handle_name);
                let m = ident(&bindgen_method_name(method));
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
                let m = ident(&bindgen_method_name(method));
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
                let method = ident(&bindgen_method_name(method));
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
}

pub(super) fn generate_dispatch(
    controls: &[super::schema::Control],
    resolver: &super::metadata::MetadataResolver,
) -> String {
    const HEADER: &str = "// This file is auto-generated by tool-reactor. Do not edit.\n\n";
    let handle_factory = generate_handle_factory(controls);
    let attach_event = attach_event::generate(controls, resolver);
    let set_prop = set_prop::generate(controls, resolver);
    let handle_factory = handle_factory.strip_prefix(HEADER).unwrap();
    let attach_event = attach_event.strip_prefix(HEADER).unwrap();
    let set_prop = set_prop.strip_prefix(HEADER).unwrap();

    format!(
        "{HEADER}{handle_factory}\nmod attach_event {{\n{attach_event}}}\n\nmod set_prop {{\n{set_prop}}}\n\nuse attach_event::dispatch as dispatch_attach_event;\nuse set_prop::dispatch as dispatch_set_prop;\n",
    )
}

fn generate_handle_factory(controls: &[super::schema::Control]) -> String {
    use proc_macro2::TokenStream;
    use quote::quote;

    use crate::helpers::{format_generated, ident};

    let mut arms = controls
        .iter()
        .filter(|control| !matches!(control.kind(), "SelectorBar" | "TitleBar"))
        .map(|control| {
            let kind = ident(control.kind());
            let handle = ident(control.handle());
            quote! {
                ControlKind::#kind => Some(Xaml::#handle::new().map(Handle::#kind)),
            }
        })
        .collect::<Vec<TokenStream>>();
    arms.extend(super::gen_ir::EXTRA_CONTROL_KINDS.iter().map(|name| {
        let kind = ident(name);
        quote! {
            ControlKind::#kind => Some(Xaml::#kind::new().map(Handle::#kind)),
        }
    }));

    format_generated(quote! {
        /// Creates native controls whose constructors are described by the schema.
        fn create_handle(kind: ControlKind) -> Option<windows_core::Result<Handle>> {
            match kind {
                #(#arms)*
                _ => None,
            }
        }
    })
}
