use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

mod gen_bindings {
    /// Generates `bindings()` helper functions from control declarations.
    use proc_macro2::TokenStream;
    use quote::quote;

    use crate::helpers::*;
    use crate::schema::*;

    pub fn generate(controls: &[Control]) -> String {
        let fns: Vec<TokenStream> = controls.iter().map(generate_one).collect();

        format_generated(quote! {
            use super::*;

            #(#fns)*
        })
    }

    fn generate_one(ctrl: &Control) -> TokenStream {
        let fn_name = ident(&format!("{}_bindings", to_snake_case(&ctrl.name)));
        let widget_type = ident(&ctrl.name);

        let has_trait_props = ctrl.prop.iter().any(|p| p.uses_trait_binding());

        let prop_stmts: Vec<TokenStream> = ctrl.prop.iter().map(gen_prop_binding).collect();
        let event_stmts: Vec<TokenStream> = ctrl.event.iter().map(gen_event_binding).collect();
        let cap = ctrl.prop.len() + ctrl.event.len();

        if cap == 0 {
            quote! {
                pub(crate) fn #fn_name(_w: &#widget_type) -> PropBindings {
                    Vec::new()
                }
            }
        } else if !has_trait_props {
            // All items are unconditional — use vec![...] directly
            let prop_items: Vec<TokenStream> = ctrl.prop.iter().map(gen_prop_item).collect();
            let event_items: Vec<TokenStream> = ctrl.event.iter().map(gen_event_item).collect();
            quote! {
                pub(crate) fn #fn_name(w: &#widget_type) -> PropBindings {
                    vec![#(#event_items,)* #(#prop_items,)*]
                }
            }
        } else {
            quote! {
                pub(crate) fn #fn_name(w: &#widget_type) -> PropBindings {
                    let mut out = Vec::with_capacity(#cap);
                    #(#event_stmts)*
                    #(#prop_stmts)*
                    out
                }
            }
        }
    }

    fn gen_prop_binding(p: &PropDecl) -> TokenStream {
        let prop = ident(p.prop());
        let field = ident(&p.field);

        if p.uses_trait_binding() {
            // Textblock/IReference setter — use ToPropStr trait so the same
            // generated code works whether the struct field is String or Option<String>.
            quote! {
                if let Some(v) = w.#field.to_prop_str() {
                    out.push(Binding::Prop(Prop::#prop, v));
                }
            }
        } else {
            // Always emit — field is T, never skipped.
            let value_expr = always_value_expr(p.value(), &field, p.copy_value, p.is_enum_as_i32());
            quote! {
                out.push(Binding::Prop(Prop::#prop, #value_expr));
            }
        }
    }

    /// Generate a `Binding::Prop(...)` expression for use inside `vec![...]`.
    fn gen_prop_item(p: &PropDecl) -> TokenStream {
        let prop = ident(p.prop());
        let field = ident(&p.field);
        let value_expr = always_value_expr(p.value(), &field, p.copy_value, p.is_enum_as_i32());
        quote! { Binding::Prop(Prop::#prop, #value_expr) }
    }

    /// Generate a `Binding::Event(...)` expression for use inside `vec![...]`.
    fn gen_event_item(e: &EventDecl) -> TokenStream {
        let event = ident(&e.event());
        let handler_variant = ident(e.value());
        let field = ident(&e.field);
        quote! {
            Binding::Event(
                Event::#event,
                w.#field.as_ref().map(|cb| EventHandler::#handler_variant(cb.clone())),
            )
        }
    }

    fn gen_event_binding(e: &EventDecl) -> TokenStream {
        let event = ident(&e.event());
        let handler_variant = ident(e.value());
        let field = ident(&e.field);

        quote! {
            out.push(Binding::Event(
                Event::#event,
                w.#field.as_ref().map(|cb| EventHandler::#handler_variant(cb.clone())),
            ));
        }
    }

    /// Build the `PropValue::Variant(w.field)` expression for an always-emit prop.
    fn always_value_expr(
        value_variant: &str,
        field: &proc_macro2::Ident,
        copy: bool,
        enum_as_i32: bool,
    ) -> TokenStream {
        // Enum properties use `.0` to access the inner i32 of the WinRT struct type.
        if enum_as_i32 {
            return quote! { PropValue::I32(w.#field.0) };
        }
        let variant = ident(value_variant);
        if copy {
            quote! { PropValue::#variant(w.#field) }
        } else {
            quote! { PropValue::#variant(w.#field.clone()) }
        }
    }
}
mod helpers {
    /// Shared helpers for the code generators.
    use proc_macro2::{Ident, Span, TokenStream};

    /// Create an `Ident` from a string.
    pub fn ident(s: &str) -> Ident {
        Ident::new(s, Span::call_site())
    }

    pub fn bindgen_method_name(method: &str) -> String {
        if let Some(property) = method.strip_prefix("put_") {
            format!("Set{property}")
        } else if let Some(property) = method.strip_prefix("get_") {
            property.to_string()
        } else if let Some(event) = method.strip_prefix("add_") {
            event.to_string()
        } else if let Some(event) = method.strip_prefix("remove_") {
            format!("Remove{event}")
        } else {
            method.to_string()
        }
    }

    /// Convert `PascalCase` to `snake_case`.
    pub fn to_snake_case(s: &str) -> String {
        let mut out = String::with_capacity(s.len() + 4);
        for (i, ch) in s.chars().enumerate() {
            if ch.is_uppercase() && i > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        }
        out
    }

    /// Format a `TokenStream` into a string with the auto-generated header.
    pub fn format_generated(tokens: TokenStream) -> String {
        let code = tokens.to_string();
        format!("// This file is auto-generated by tool-reactor. Do not edit.\n\n{code}\n")
    }
}
mod metadata {
    /// Reads WinUI `.winmd` files and resolves `put_*` method → interface mappings.
    ///
    /// Given a WinUI class name (e.g. `"TextBlock"`) and a method (e.g. `"put_Text"`),
    /// the resolver finds which exclusive interface owns that method (e.g. `"ITextBlock"`).
    use std::collections::HashMap;
    use std::path::Path;

    use windows_metadata::Type;
    use windows_metadata::reader::{File, Index, TypeDef, TypeDefOrRef};

    use crate::schema::module_alias_for_namespace;

    /// Resolved interface location: namespace + name.
    #[derive(Clone, Debug)]
    pub struct InterfaceRef {
        pub namespace: String,
        pub name: String,
    }

    impl InterfaceRef {
        /// The short name (e.g. `"ITextBlock"`).
        pub fn short_name(&self) -> &str {
            &self.name
        }

        pub fn module_alias(&self) -> &'static str {
            module_alias_for_namespace(&self.namespace).unwrap_or_else(|| {
                panic!(
                    "metadata: no backend module alias for {}.{}",
                    self.namespace, self.name
                )
            })
        }
    }

    /// Resolved method location: owning interface + parameter types.
    #[derive(Clone, Debug)]
    pub struct MethodRef {
        pub interface: InterfaceRef,
        /// Parameter types from the method signature (excludes `this`/return).
        pub param_types: Vec<Type>,
        /// Return type of the method.
        pub return_type: Type,
    }

    /// Classification of a metadata parameter type for setter pattern inference.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ParamClass {
        /// Primitive type (String, Bool, F64, etc.) → direct `put_X(value)`.
        Primitive,
        /// IInspectable → needs wrapping (textblock by default, or IReference when
        /// the TOML specifies an explicit type).
        IInspectable,
        /// IReference<bool> → `put_X(Some(value))`.
        NullableBool,
        /// Enum, struct, or other complex type → needs explicit TOML config.
        Complex,
    }

    /// Pre-built lookup: `(class_short_name, method_name) → MethodRef`.
    pub struct MetadataResolver {
        lookup: HashMap<(String, String), MethodRef>,
        /// Value-type structs that wrap a single primitive field.
        /// Maps `(namespace, name)` → the unwrapped inner `Type`.
        single_field_types: HashMap<(String, String), Type>,
        /// Enum types: maps `(namespace, name)` → list of variant names.
        enum_variants: HashMap<(String, String), Vec<String>>,
        /// Non-generic delegate → args class short name, resolved from the
        /// delegate's `Invoke` method signature.
        delegate_args: HashMap<String, String>,
    }

    impl MetadataResolver {
        /// Load all `.winmd` files from `winmd_dir` and build the resolver.
        pub fn load(winmd_dir: &Path) -> Self {
            let files: Vec<File> = std::fs::read_dir(winmd_dir)
                .expect("cannot read winmd directory")
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("winmd"))
                })
                .filter_map(|e| File::read(e.path()))
                .collect();

            assert!(
                !files.is_empty(),
                "no .winmd files found in {}",
                winmd_dir.display()
            );

            let index = Index::new(files);
            let mut lookup = HashMap::new();

            // Walk all types in the index, collecting method→interface for XAML classes.
            for (namespace, name, typedef) in index.iter() {
                if namespace.starts_with("Windows.UI.Xaml") {
                    Self::collect_methods_for_class(&index, name, &typedef, &mut lookup);
                }
            }

            // Validate all entries — remove any with namespaces that don't exist in the index.
            lookup.retain(|_, mref| {
                index
                    .get(&mref.interface.namespace, &mref.interface.name)
                    .next()
                    .is_some()
            });

            // Build single-field struct map: value types with exactly one field
            // get mapped to their inner primitive type (e.g. FontWeight → U16).
            let mut single_field_types = HashMap::new();
            let mut enum_variants = HashMap::new();
            for (namespace, name, typedef) in index.iter() {
                let fields: Vec<_> = typedef.fields().collect();
                // Enums have a `value__` field plus named variant fields.
                let variant_names: Vec<_> = fields
                    .iter()
                    .filter(|f| f.name() != "value__")
                    .map(|f| f.name().to_string())
                    .collect();
                let has_value_field = fields.iter().any(|f| f.name() == "value__");

                if has_value_field && !variant_names.is_empty() {
                    // This is an enum.
                    enum_variants.insert((namespace.to_string(), name.to_string()), variant_names);
                } else if fields.len() == 1 {
                    let inner_ty = fields[0].ty();
                    if Self::primitive_value_for_type(&inner_ty).is_some() {
                        single_field_types
                            .insert((namespace.to_string(), name.to_string()), inner_ty);
                    }
                }
            }

            // Build delegate→args map: for non-generic delegates used by add_*
            // methods, resolve the Invoke method's second parameter to find the
            // event args class.
            let mut delegate_args = HashMap::new();
            for ((_, method_name), mref) in &lookup {
                if !method_name.starts_with("add_") {
                    continue;
                }
                let Some(Type::ClassName(tn)) = mref.param_types.first() else {
                    continue;
                };
                if !tn.generics.is_empty() || delegate_args.contains_key(&tn.name) {
                    continue;
                }
                // Look up the delegate typedef and find its Invoke method.
                let Some(delegate_def) = index.get(&tn.namespace, &tn.name).next() else {
                    continue;
                };
                for method in delegate_def.methods() {
                    if method.name() == "Invoke" {
                        let sig = method.signature(&[]);
                        // Second parameter is the args type (first is sender).
                        if let Some(args_type) = sig.types.get(1) {
                            let args_name = match args_type {
                                Type::ClassName(args_tn) => Some(args_tn.name.clone()),
                                Type::ValueName(args_tn) => Some(args_tn.name.clone()),
                                _ => None,
                            };
                            if let Some(name) = args_name {
                                delegate_args.insert(tn.name.clone(), name);
                            }
                        }
                        break;
                    }
                }
            }

            MetadataResolver {
                lookup,
                single_field_types,
                enum_variants,
                delegate_args,
            }
        }

        /// Walk a class's interface hierarchy and record every `put_*` / `get_*` /
        /// `add_*` / `remove_*` method with its owning interface and parameter types.
        fn collect_methods_for_class(
            index: &Index,
            class_name: &str,
            typedef: &TypeDef<'_>,
            lookup: &mut HashMap<(String, String), MethodRef>,
        ) {
            // Walk implemented interfaces.
            for iface_impl in typedef.interface_impls() {
                let iface_type = iface_impl.interface(&[]);
                let (iface_ns, iface_name) = match &iface_type {
                    Type::ClassName(tn) | Type::ValueName(tn) => {
                        (tn.namespace.as_str(), tn.name.as_str())
                    }
                    _ => continue,
                };

                // Look up the interface TypeDef to get its methods.
                let Some(iface_def) = index.get(iface_ns, iface_name).next() else {
                    continue;
                };

                for method in iface_def.methods() {
                    let method_name = method.name();
                    if method_name.starts_with("put_")
                        || method_name.starts_with("get_")
                        || method_name.starts_with("add_")
                        || method_name.starts_with("remove_")
                    {
                        let key = (class_name.to_string(), method_name.to_string());
                        let sig = method.signature(&[]);
                        let method_ref = MethodRef {
                            interface: InterfaceRef {
                                namespace: iface_ns.to_string(),
                                name: iface_name.to_string(),
                            },
                            param_types: sig.types,
                            return_type: sig.return_type,
                        };
                        lookup.entry(key).or_insert(method_ref);
                    }
                }
            }

            // Walk base class chain to inherit methods.
            let Some(extends) = typedef.extends() else {
                return;
            };
            let (base_ns, base_name) = match extends {
                TypeDefOrRef::TypeDef(td) => (td.namespace().to_string(), td.name().to_string()),
                TypeDefOrRef::TypeRef(tr) => (tr.namespace().to_string(), tr.name().to_string()),
                _ => return,
            };
            // Stop at System.Object / DependencyObject.
            if base_name == "Object" || base_name == "DependencyObject" {
                return;
            }

            // Resolve the base class and recurse.
            if let Some(base_def) = index.get(&base_ns, &base_name).next() {
                Self::collect_methods_for_class(index, &base_name, &base_def, lookup);

                // Propagate base class methods to the derived class.
                let base_methods: Vec<(String, MethodRef)> = lookup
                    .iter()
                    .filter(|((cn, _), _)| cn == &base_name)
                    .map(|((_, mn), mref)| (mn.clone(), mref.clone()))
                    .collect();
                for (method_name, method_ref) in base_methods {
                    let key = (class_name.to_string(), method_name);
                    lookup.entry(key).or_insert(method_ref);
                }
            }
        }

        /// Resolve a method on a class to its interface.
        ///
        /// Returns the InterfaceRef if found.
        pub fn resolve(&self, class_name: &str, method_name: &str) -> Option<&InterfaceRef> {
            self.lookup
                .get(&(class_name.to_string(), method_name.to_string()))
                .map(|m| &m.interface)
        }

        /// Check if a method exists for a class in metadata.
        pub fn has_method(&self, class_name: &str, method_name: &str) -> bool {
            self.lookup
                .contains_key(&(class_name.to_string(), method_name.to_string()))
        }

        /// If a method's parameter is an enum, return `(short_name, module_alias, [variant_names])`.
        /// Used to auto-generate `enum_map` from metadata without explicit TOML overrides.
        pub fn enum_info(
            &self,
            class_name: &str,
            method_name: &str,
        ) -> Option<(&str, &'static str, &[String])> {
            let mref = self
                .lookup
                .get(&(class_name.to_string(), method_name.to_string()))?;
            let param = mref.param_types.first()?;
            if let Type::ValueName(tn) = param {
                let key = (tn.namespace.clone(), tn.name.clone());
                let variants = self.enum_variants.get(&key)?;
                let module = module_alias_for_namespace(&tn.namespace)?;
                Some((&tn.name, module, variants))
            } else {
                None
            }
        }

        /// Infer the `PropValue` variant name and Copy-ness from a method's parameter
        /// type in metadata.
        ///
        /// Returns `None` for complex types (enums, generics) that need
        /// explicit TOML declaration. The bool indicates whether the type is Copy.
        pub fn infer_value_type(
            &self,
            class_name: &str,
            method_name: &str,
        ) -> Option<(String, bool)> {
            let mref = self
                .lookup
                .get(&(class_name.to_string(), method_name.to_string()))?;
            let param = mref.param_types.first()?;
            let name = self.value_for_type(param)?;
            // Copy-ness follows the same unwrapping as value_for_type:
            // IReference<T> unwraps to T, single-field wrappers unwrap to inner.
            let copy = self.is_unwrapped_copy(param);
            Some((name, copy))
        }

        /// Infer the value type of a property on an event's args class.
        ///
        /// Given `(class, add_event, property)`, resolves the delegate parameter of
        /// `add_{event}` to find the args class, then looks up `get_{property}` on
        /// that class and returns the value type from its return type.
        pub fn infer_event_args_type(
            &self,
            class_name: &str,
            add_event: &str,
            property: &str,
        ) -> Option<String> {
            // Get the delegate type from the add method's first param.
            let add_ref = self
                .lookup
                .get(&(class_name.to_string(), add_event.to_string()))?;
            let delegate_type = add_ref.param_types.first()?;
            // Extract the args class name from the delegate type.
            let args_class = match delegate_type {
                // TypedEventHandler<TSender, TArgs> — extract TArgs from generics.
                Type::ClassName(tn) if tn.generics.len() == 2 => match &tn.generics[1] {
                    Type::ClassName(args_tn) => args_tn.name.clone(),
                    Type::ValueName(args_tn) => args_tn.name.clone(),
                    _ => return None,
                },
                // Non-generic delegate — look up args class from Invoke signature.
                Type::ClassName(tn) => self.delegate_args.get(&tn.name)?.clone(),
                _ => return None,
            };
            // Look up get_{property} on the args class.
            let getter = format!("get_{property}");
            let getter_ref = self.lookup.get(&(args_class, getter))?;
            self.value_for_type(&getter_ref.return_type)
        }

        /// Returns true if a metadata `Type` is Copy (primitive or value type).
        ///
        /// Follows the same pattern as `is_copyable` in windows-bindgen:
        /// primitives and ValueName types (enums, blittable structs) are Copy;
        /// String, ClassName, Object, and generics are not.
        fn is_copy(ty: &Type) -> bool {
            match ty {
                Type::String | Type::Object | Type::ClassName(_) => false,
                Type::Generic(..) | Type::Array(_) => false,
                // Primitives, ValueName (enums + structs), etc. → Copy
                _ => true,
            }
        }

        /// Check copy-ness after applying the same unwrapping as `value_for_type`:
        /// IReference<T> → check T, single-field wrappers → check inner type.
        fn is_unwrapped_copy(&self, ty: &Type) -> bool {
            match ty {
                // IReference<T> → the PropValue wraps T, not IReference
                Type::ClassName(tn)
                    if tn.namespace == "Windows.Foundation" && tn.name == "IReference`1" =>
                {
                    tn.generics.first().is_some_and(Self::is_copy)
                }
                Type::ValueName(tn) => {
                    let key = (tn.namespace.clone(), tn.name.clone());
                    if let Some(inner) = self.single_field_types.get(&key) {
                        // Single-field wrapper → unwraps to inner primitive
                        Self::is_copy(inner)
                    } else {
                        // Multi-field value type or enum → Copy
                        true
                    }
                }
                _ => Self::is_copy(ty),
            }
        }

        /// Check copy-ness for a method's parameter type, applying IReference
        /// unwrapping.
        pub fn is_method_copy(&self, class_name: &str, method_name: &str) -> bool {
            let Some(mref) = self
                .lookup
                .get(&(class_name.to_string(), method_name.to_string()))
            else {
                return false;
            };
            let Some(param) = mref.param_types.first() else {
                return false;
            };
            self.is_unwrapped_copy(param)
        }

        /// Check if a PropValue variant name wraps a Copy type.
        ///
        /// Used as a fallback when no metadata method exists (setter_fn properties).
        /// Reverse-maps the name through `primitive_value_for_type` to find the
        /// underlying `Type` and checks `is_copy` on it. Unknown names default
        /// to non-Copy (safe — `.clone()` always works on Clone types).
        pub fn is_copy_value_name(&self, value_name: &str) -> bool {
            // Check every primitive Type — if primitive_value_for_type produces
            // this name, use is_copy on that Type.
            if [
                Type::String,
                Type::Bool,
                Type::F64,
                Type::I32,
                Type::U16,
                Type::U32,
                Type::U8,
                Type::Object,
            ]
            .iter()
            .any(|ty| {
                Self::primitive_value_for_type(ty).as_deref() == Some(value_name)
                    && Self::is_copy(ty)
            }) {
                return true;
            }
            // Check metadata enums and value types by short name — enums and
            // value-type structs are always Copy in WinUI.
            self.enum_variants
                .keys()
                .any(|(_, name)| name == value_name)
                || self
                    .single_field_types
                    .keys()
                    .any(|(_, name)| name == value_name)
        }

        /// Map a metadata Type to a PropValue variant name.
        /// Handles primitives, IReference<bool>, single-field wrapper structs
        /// (e.g. FontWeight{Weight: u16} → U16), and multi-field value-type
        /// structs by using the struct's short name (e.g. Thickness → "Thickness").
        pub fn value_for_type(&self, ty: &Type) -> Option<String> {
            // Try primitives and well-known types first.
            if let Some(v) = Self::primitive_value_for_type(ty) {
                return Some(v);
            }
            match ty {
                Type::ValueName(tn) => {
                    let key = (tn.namespace.clone(), tn.name.clone());
                    // Single-field wrapper structs → unwrap to inner primitive
                    if let Some(inner) = self.single_field_types.get(&key) {
                        return Self::primitive_value_for_type(inner);
                    }
                    // Multi-field value types → use the struct's short name as
                    // the PropValue variant (e.g. Thickness → "Thickness").
                    // If there's no matching PropValue variant the generated code
                    // won't compile, signalling that an explicit override is needed.
                    Some(tn.name.clone())
                }
                _ => None,
            }
        }

        /// Map primitive and well-known Type variants to PropValue names.
        /// Does not require resolver state — used during construction.
        fn primitive_value_for_type(ty: &Type) -> Option<String> {
            match ty {
                Type::String => Some("Str".to_string()),
                Type::Bool => Some("Bool".to_string()),
                Type::F64 => Some("F64".to_string()),
                Type::I32 => Some("I32".to_string()),
                Type::U16 => Some("U16".to_string()),
                Type::U32 => Some("U32".to_string()),
                Type::Object => Some("Str".to_string()),
                Type::ClassName(tn)
                    if tn.namespace == "Windows.Foundation"
                        && tn.name == "IReference`1"
                        && tn.generics.first() == Some(&Type::Bool) =>
                {
                    Some("Bool".to_string())
                }
                _ => None,
            }
        }

        /// Classify the parameter type for setter pattern inference.
        pub fn classify_param(&self, class_name: &str, method_name: &str) -> Option<ParamClass> {
            let mref = self
                .lookup
                .get(&(class_name.to_string(), method_name.to_string()))?;
            let param = mref.param_types.first()?;
            Some(Self::classify_type(param))
        }

        /// Classify a Type into a setter pattern category.
        fn classify_type(ty: &Type) -> ParamClass {
            match ty {
                Type::String | Type::Bool | Type::F64 | Type::I32 | Type::U16 | Type::U32 => {
                    ParamClass::Primitive
                }
                Type::Object => ParamClass::IInspectable,
                Type::ClassName(tn)
                    if tn.namespace == "Windows.Foundation"
                        && tn.name == "IReference`1"
                        && tn.generics.first() == Some(&Type::Bool) =>
                {
                    ParamClass::NullableBool
                }
                _ => ParamClass::Complex,
            }
        }
    }
}
mod native {
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
        fn event_receiver(
            module: &str,
            iface: &proc_macro2::Ident,
            handle_name: &str,
        ) -> TokenStream {
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
                use super::super::*;
                use crate::bindings as Xaml;
                /// Try to handle an `attach_event` call via generated dispatch.
                /// Returns `Some(revokers)` if handled, `None` to fall through.
                pub(crate) fn dispatch(
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
                panic!("event '{handle_name}.{event_name}': invoke_bool_dual requires false_event")
            });
                    let add_false = format!("add_{false_evt}");
                    let add_false_ident = ident(&bindgen_method_name(&add_false));
                    let iface_false_ref =
                        resolver
                            .resolve(handle_name, &add_false)
                            .unwrap_or_else(|| {
                                panic!(
                                    "metadata: cannot resolve {handle_name}.{add_false} for event"
                                )
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
                _ => {
                    panic!("event '{handle_name}.{event_name}': unknown invoke pattern '{invoke}'")
                }
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
                use super::super::*;
                use super::super::convert::string_as_textblock;
                use crate::bindings as Xaml;
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

        fn resolve_iface(
            resolver: &MetadataResolver,
            handle: &str,
            method: &str,
        ) -> (String, String) {
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

    pub(super) const BACKEND_GENERATED_MODULE: &str = "// This file is auto-generated by tool-reactor. Do not edit.\n\
\n\
mod attach_event;\n\
mod set_prop;\n\
pub(super) use attach_event::dispatch as dispatch_attach_event;\n\
pub(super) use set_prop::dispatch as dispatch_set_prop;\n";
}
mod schema {
    /// TOML schema types for reactor widget declarations.
    ///
    /// These types define the internal representation that the codegen
    /// generators consume. They are constructed by the TOML parser in
    /// `toml_parser.rs`, not deserialized directly.
    use serde::Deserialize;

    use crate::metadata::MetadataResolver;

    /// One widget / control declaration.
    pub struct Control {
        /// Widget struct name (e.g. `"TextBlock"`).
        pub name: String,
        /// `ControlKind` variant. Defaults to `name` with `Widget` suffix stripped.
        pub kind: Option<String>,
        /// `Handle` variant / WinUI class name (e.g. `"TextBlock"`).
        /// Defaults to the resolved `kind`.
        /// The metadata resolver looks up `{namespace}.{handle}`.
        pub handle: Option<String>,
        /// WinRT namespace override. Defaults to `"Windows.UI.Xaml.Controls"`.
        pub namespace: Option<String>,
        /// Property bindings.
        pub prop: Vec<PropDecl>,
        /// Event bindings.
        pub event: Vec<EventDecl>,
    }

    impl Control {
        /// Resolved `kind` — defaults to `name` with trailing "Widget" stripped.
        pub fn kind(&self) -> &str {
            self.kind
                .as_deref()
                .unwrap_or_else(|| self.name.strip_suffix("Widget").unwrap_or(&self.name))
        }

        /// Resolved `handle` — defaults to `kind()`.
        pub fn handle(&self) -> &str {
            self.handle.as_deref().unwrap_or_else(|| self.kind())
        }

        /// Resolved WinRT namespace for this control.
        pub fn namespace(&self) -> &str {
            self.namespace
                .as_deref()
                .unwrap_or("Windows.UI.Xaml.Controls")
        }

        /// Rust binding module used by the backend for this control's native type.
        pub fn module_alias(&self) -> Option<&'static str> {
            module_alias_for_namespace(self.namespace())
        }
    }

    /// A property binding declaration.
    pub struct PropDecl {
        /// Widget struct field name (e.g. `"content"`, `"font_size"`).
        pub field: String,
        /// WinUI metadata property name (the TOML key, e.g. `"PlaceholderText"`).
        /// Also used as the `Prop` enum variant name.
        pub meta_name: String,
        /// `PropValue` variant name (e.g. `"Str"`, `"F64"`, `"Bool"`).
        /// If omitted, inferred from metadata parameter type.
        pub value: Option<String>,

        // ── Setter (pick one) ─────────────────────────────────────────
        /// COM method name (e.g. `"put_Text"`).  The codegen tool resolves
        /// the owning interface from winmd metadata automatically.
        /// Mutually exclusive with `method_textblock`.
        pub method: Option<String>,

        /// Like `method` but wraps the value in `Some()` before calling.
        /// Used for nullable COM properties like `put_IsChecked(Option<bool>)`.
        /// Unset calls with `None`.
        pub method_optional: Option<String>,

        /// Like `method` but wraps the value in `IReference::from()` before calling.
        /// Used for IInspectable-typed properties that accept a boxed string.
        /// Generates:
        ///   set: `IReference::from(v.as_str())` → `h.cast::<I>()?.put_Foo(&insp)?`
        ///   unset: `h.cast::<I>()?.put_Foo(None)?`
        pub method_ireference: Option<String>,

        /// Like `method` but wraps a Str value in a TextBlock first.
        /// Used for `put_Header` and similar IInspectable-typed properties
        /// where the reactor passes a string.  Generates:
        ///   set: `string_as_textblock(v)` → `h.cast::<I>()?.put_Header(&tb)?`
        ///   unset: `h.cast::<I>()?.put_Header(None)?`
        pub method_textblock: Option<String>,

        /// Multi-variant enum mapping setter. Maps each Rust enum variant to a
        /// WinUI enum variant and calls one `put_*` method.
        pub method_enum_map: Option<EnumMapSetter>,

        /// Whether the PropValue variant wraps a Copy type (no `.clone()` needed).
        /// Inferred automatically: primitives, enum_maps, and metadata
        /// value-type structs are Copy; String/Vec wrappers are not.
        pub copy_value: bool,

        /// Whether this property transports a `#[repr(i32)]` enum as `I32`.
        /// When true, generated bindings emit `PropValue::I32(w.field as i32)`.
        pub enum_as_i32: bool,

        /// Skip backend dispatch generation while still generating core bindings.
        pub custom: bool,
    }

    /// Enum-typed setter: maps a `#[repr(i32)]` reactor enum to a WinRT enum.
    /// The `I32` transport means the backend constructs `Xaml::WinUIType(v)` directly.
    #[derive(Deserialize, Clone, Debug)]
    pub struct EnumMapSetter {
        /// COM method name (e.g. `"put_Severity"`).
        /// Defaults to `put_{meta_name}`.
        #[serde(default)]
        pub method: Option<String>,
        /// Binding module for the WinUI enum type. Inferred from metadata when possible.
        #[serde(default)]
        pub module: Option<String>,
        /// WinUI enum type relative to `Xaml::` (e.g. `"TreeViewSelectionMode"`).
        pub winui_type: String,
    }

    impl EnumMapSetter {
        pub fn method(&self) -> &str {
            self.method.as_deref().expect("method must be resolved")
        }
    }

    /// Resolved setter kind — computed from the TOML fields.
    pub enum SetterKind<'a> {
        /// Auto-resolved via metadata: `h.cast::<Xaml::{interface}>()?.{method}({arg})?`
        Method {
            method: &'a str,
        },
        /// Like Method but wraps value in Some(): `h.cast::<I>()?.method(Some(arg))?`
        MethodOptional {
            method: &'a str,
        },
        /// Wraps string in IReference then calls method.
        MethodIReference {
            method: &'a str,
        },
        /// Wrap string in TextBlock then call method with IInspectable.
        MethodTextblock {
            method: &'a str,
        },
        /// Map multi-variant Rust enum to WinUI enum.
        MethodEnumMap {
            setter: &'a EnumMapSetter,
        },
        Custom,
    }

    impl PropDecl {
        /// The `Prop` enum variant name — always the metadata name (TOML key).
        pub fn prop(&self) -> &str {
            &self.meta_name
        }

        /// Get the resolved value variant name.
        /// Panics if not set (must call `resolve_defaults` first or set explicitly).
        pub fn value(&self) -> &str {
            self.value
            .as_deref()
            .unwrap_or_else(|| panic!("prop '{}' has no value — set it explicitly or ensure resolve_defaults() was called", self.prop()))
        }

        /// True if this property carries a `#[repr(i32)]` enum transported as `I32`.
        /// The generated bindings must emit `PropValue::I32(w.field as i32)`.
        pub fn is_enum_as_i32(&self) -> bool {
            self.enum_as_i32
        }

        /// Whether this property uses the `ToPropStr` trait for binding generation.
        ///
        /// True for Textblock and IReference setters — the trait lets the same
        /// generated code work whether the widget struct field is `String` or
        /// `Option<String>`.
        pub fn uses_trait_binding(&self) -> bool {
            self.method_textblock.is_some() || self.method_ireference.is_some()
        }

        /// Fill in default values that can be inferred.
        ///
        /// * `method` defaults to `put_{meta_name}` when no setter kind is specified.
        /// * `value` defaults to the metadata-inferred type when available.
        /// * Setter pattern auto-detected from metadata param type:
        ///   - IInspectable + explicit `value` → `method_ireference` (IReference boxing)
        ///   - IInspectable (no explicit value) → `method_textblock` (TextBlock wrapping)
        ///   - IReference<bool> → `method_optional`
        ///
        /// Must be called after construction, before any codegen.
        pub fn resolve_defaults(&mut self, resolver: Option<&MetadataResolver>, handle: &str) {
            use crate::metadata::ParamClass;

            if self.custom {
                assert!(
                    self.value.is_some(),
                    "custom prop '{}' must set an explicit type",
                    self.prop()
                );
                return;
            }

            // Infer method when no setter is specified.
            // Use put_{meta_name} — the metadata name is authoritative.
            let has_setter = self.method.is_some()
                || self.method_optional.is_some()
                || self.method_ireference.is_some()
                || self.method_textblock.is_some()
                || self.method_enum_map.is_some();
            if !has_setter {
                let method_name = format!("put_{}", self.meta_name);
                // Auto-detect setter pattern from metadata param type.
                if let Some(resolver) = resolver {
                    if let Some(param_class) = resolver.classify_param(handle, &method_name) {
                        match param_class {
                            ParamClass::IInspectable => {
                                if self.value.is_some() {
                                    self.method_ireference = Some(method_name);
                                } else {
                                    self.method_textblock = Some(method_name);
                                }
                            }
                            ParamClass::NullableBool => {
                                self.method_optional = Some(method_name);
                            }
                            ParamClass::Primitive | ParamClass::Complex => {
                                self.method = Some(method_name);
                            }
                        }
                    } else {
                        self.method = Some(method_name);
                    }
                } else {
                    self.method = Some(method_name);
                }
            }

            // Resolve default method for enum_map when omitted.
            // Use put_{meta_name} — the metadata name is authoritative.
            if self
                .method_enum_map
                .as_ref()
                .is_some_and(|s| s.method.is_none())
            {
                let method = format!("put_{}", self.meta_name);
                self.method_enum_map.as_mut().unwrap().method = Some(method);
            }

            // Enum-map properties are transported as I32.
            if self.method_enum_map.is_some() && self.value.is_none() {
                self.value = Some("I32".to_string());
            }

            // Infer value from metadata when omitted.
            if self.value.is_none() {
                let method_name = self
                    .method
                    .as_deref()
                    .or(self.method_optional.as_deref())
                    .or(self.method_ireference.as_deref())
                    .or(self.method_textblock.as_deref())
                    .or(self
                        .method_enum_map
                        .as_ref()
                        .and_then(|s| s.method.as_deref()));
                if let (Some(resolver), Some(method)) = (resolver, method_name)
                    && let Some((value, _copy)) = resolver.infer_value_type(handle, method)
                {
                    self.value = Some(value);
                }
            }
        }

        /// Get the resolved setter kind, validating mutual exclusion.
        pub fn setter(&self) -> SetterKind<'_> {
            let prop = self.prop();
            let count = self.method.is_some() as u8
                + self.method_optional.is_some() as u8
                + self.method_ireference.is_some() as u8
                + self.method_textblock.is_some() as u8
                + self.method_enum_map.is_some() as u8;
            if self.custom {
                return SetterKind::Custom;
            }
            assert!(
                count <= 1,
                "prop '{prop}' has multiple setter types — pick exactly one",
            );
            assert!(
                count >= 1,
                "prop '{prop}' has no setter — set one of method/method_optional/method_ireference/method_textblock/method_enum_map",
            );
            if let Some(m) = &self.method {
                SetterKind::Method { method: m }
            } else if let Some(m) = &self.method_optional {
                SetterKind::MethodOptional { method: m }
            } else if let Some(m) = &self.method_ireference {
                SetterKind::MethodIReference { method: m }
            } else if let Some(m) = &self.method_textblock {
                SetterKind::MethodTextblock { method: m }
            } else if let Some(s) = &self.method_enum_map {
                SetterKind::MethodEnumMap { setter: s }
            } else {
                SetterKind::Custom
            }
        }
    }

    /// Derive the argument expression from the `PropValue` variant name.
    ///
    /// This replaces the old `ArgKind` enum — the mapping is deterministic:
    /// - `Str` → `v.as_str()` (HSTRING params take `&str`)
    /// - `Bool`, `F64`, `I32`, `U16`, `U32` → `*v` (Copy types)
    /// - everything else → `v.clone()` (owned reference types)
    pub fn arg_expr_for_value(value_variant: &str) -> &'static str {
        match value_variant {
            "Str" => "str_ref",
            "Bool" | "F64" | "I32" | "U16" | "U32" | "U8" => "copy",
            _ => "clone",
        }
    }

    /// An event binding declaration.
    pub struct EventDecl {
        /// Widget struct field name (e.g. `"on_changed"`).
        pub field: String,
        /// WinUI metadata event name (the TOML key, e.g. `"PasswordChanged"`).
        /// Used as the default for `event()` (Event enum variant).
        pub meta_name: String,
        /// `Event` enum variant name override. If omitted, defaults to `meta_name`.
        pub event: Option<String>,
        /// `EventHandler` variant name (payload type: `"Unit"`, `"Bool"`, `"Str"`,
        /// `"F64"`, `"I32"`, `"Color"`, `"DateTime"`, `"TimeSpan"`).
        /// If omitted, inferred from invoke pattern.
        pub value: Option<String>,

        // ── Attachment (pick one) ─────────────────────────────
        /// If true, codegen skips this event; attachment is handled by
        /// hand-written code in the backend's `attach_event` fallback match.
        pub manual: bool,

        /// COM add method name for auto-generated event attachment.
        /// If omitted and not `manual`, inferred as `"add_{event}"`.
        pub add_method: Option<String>,

        /// How the handler is invoked. Required when `add_method` is set.
        pub invoke: Option<String>,

        /// Property name on sender/args for extracting values (e.g. `"IsOn"`).
        /// Code generation prepends `get_` to form the method name.
        pub property: Option<String>,

        /// Complement event that fires `false` for bool-dual patterns (e.g. `"Unchecked"`).
        /// Code generation prepends `add_` to form the method name.
        pub false_event: Option<String>,
    }

    impl EventDecl {
        /// Resolve the event name. Defaults to metadata name (TOML key).
        pub fn event(&self) -> String {
            if let Some(e) = &self.event {
                return e.clone();
            }
            self.meta_name.clone()
        }

        /// Resolve the EventHandler variant name (payload type).
        /// Inferred from invoke pattern when not explicit.
        pub fn value(&self) -> &str {
            if let Some(v) = &self.value {
                return v;
            }
            // Infer from invoke pattern
            match self.invoke() {
                "invoke" => "Unit",
                "invoke_bool_getter" | "invoke_bool_dual" => "Bool",
                "invoke_string_getter" => "Str",
                "invoke_f64_getter" | "invoke_f64_args" => "F64",
                "invoke_i32_getter" | "invoke_i32_args" => "I32",
                _ => "Unit",
            }
        }

        /// Resolve the add_method, inferring from event if not explicit.
        pub fn add_method(&self) -> Option<String> {
            if let Some(m) = &self.add_method {
                return Some(m.clone());
            }
            if self.manual {
                return None;
            }
            // Infer: "Click" → "add_Click"
            Some(format!("add_{}", self.event()))
        }

        /// Resolve the invoke pattern.
        ///
        /// Fully resolved at parse time by `build_event`. Falls back to `"invoke"` (unit).
        pub fn invoke(&self) -> &str {
            self.invoke.as_deref().unwrap_or("invoke")
        }
    }

    pub fn module_alias_for_namespace(namespace: &str) -> Option<&'static str> {
        if namespace.starts_with("Windows.UI.Xaml") {
            Some("Xaml")
        } else {
            None
        }
    }

    /// Validate TOML declarations against metadata. Returns a list of warnings.
    /// Catches mismatches like a method name that doesn't resolve, or an event
    /// add_method that doesn't exist in metadata.
    pub fn validate(controls: &[Control], resolver: &MetadataResolver) -> Vec<String> {
        let mut warnings = Vec::new();
        for ctrl in controls {
            let handle = ctrl.handle();
            for p in &ctrl.prop {
                if p.custom {
                    continue;
                }
                let method_name = p
                    .method
                    .as_deref()
                    .or(p.method_optional.as_deref())
                    .or(p.method_ireference.as_deref())
                    .or(p.method_textblock.as_deref())
                    .or(p.method_enum_map.as_ref().and_then(|s| s.method.as_deref()));
                if let Some(method) = method_name
                    && resolver.resolve(handle, method).is_none()
                {
                    warnings.push(format!(
                        "  ⚠ {handle}.{} → {method} not found in metadata",
                        p.field,
                    ));
                }
            }
            for e in &ctrl.event {
                // Skip custom attach — they have hand-written code.
                if e.manual {
                    continue;
                }
                if let Some(add_method) = e.add_method()
                    && resolver.resolve(handle, &add_method).is_none()
                {
                    warnings.push(format!(
                        "  ⚠ {handle}.{} → {add_method} not found in metadata",
                        e.field,
                    ));
                }
            }
        }
        warnings
    }
}
mod toml_parser {
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
        /// Explicit binding module for `winui_type`, e.g. `Xaml`.
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

                let overrides: MemberOverride =
                    member_value.clone().try_into().unwrap_or_else(|e| {
                        panic!("{line}failed to parse member '{member_name}' in '{type_name}': {e}")
                    });

                let is_custom_prop =
                    overrides.custom.unwrap_or(false) && overrides.manual.is_none();
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

        /// Returns a prefix like "schema.toml:42: " or "" if not found.
        fn find(&self, section: &str, key: &str) -> String {
            self.entries
                .get(&(section.to_string(), key.to_string()))
                .map(|line| format!("schema.toml:{line}: "))
                .unwrap_or_default()
        }
    }
}

const SCHEMA: &str = include_str!("schema.toml");

const XAML_MINIMAL_FILTERS: &str = include_str!("bindings.txt");
const WIDGET_DECLARATIONS: &str = include_str!("declarations.rs");
const XAML_BINDINGS_OUTPUT: &str = "crates/islands-reactor/src/native/bindings.rs";
const GENERATED_DECLARATIONS_OUTPUT: &str = "crates/islands-reactor/src/generated_declarations.rs";
const GENERATED_SET_PROP_OUTPUT: &str =
    "crates/islands-reactor/src/native/winui/backend/generated/set_prop.rs";
const GENERATED_ATTACH_EVENT_OUTPUT: &str =
    "crates/islands-reactor/src/native/winui/backend/generated/attach_event.rs";
const GENERATED_BACKEND_MODULE_OUTPUT: &str =
    "crates/islands-reactor/src/native/winui/backend/generated.rs";

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("generate-bindings") if args.next().is_none() => {
            if let Err(err) = generate_bindings() {
                eprintln!("tool-reactor: {err}");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("usage: cargo run -p tool-reactor -- generate-bindings");
            std::process::exit(2);
        }
    }
}

fn generate_bindings() -> Result<(), String> {
    let root = workspace_root()?;
    generate_xaml_bindings(&root)?;
    generate_reactor_code(&root)?;
    Ok(())
}

fn generate_xaml_bindings(root: &Path) -> Result<(), String> {
    let out = root.join(XAML_BINDINGS_OUTPUT);
    let temp = root.join("target").join("tool-reactor").join("bindings.rs");
    if let Some(parent) = temp.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    windows_bindgen::builder()
        .input_default()
        .output(&temp)
        .minimal()
        .flat()
        .filters(XAML_MINIMAL_FILTERS.lines())
        .write();
    patch_xaml_bindings(&temp)?;
    mark_generated(&temp)?;
    copy_if_changed(&temp, &out)
}

fn generate_reactor_code(root: &Path) -> Result<(), String> {
    let metadata_dir = stage_reactor_metadata(root)?;
    let resolver = metadata::MetadataResolver::load(&metadata_dir);
    let mut controls = toml_parser::parse(SCHEMA, &resolver);

    for ctrl in &mut controls {
        let handle = ctrl.handle().to_string();
        for prop in &mut ctrl.prop {
            prop.resolve_defaults(Some(&resolver), &handle);
        }
    }

    for warning in schema::validate(&controls, &resolver) {
        eprintln!("{warning}");
    }

    let bindings = gen_bindings::generate(&controls);
    let binding_body = bindings
        .strip_prefix("// This file is auto-generated by tool-reactor. Do not edit.\n\n")
        .ok_or_else(|| "generated widget bindings are missing their header".to_string())?;
    let declarations = format!(
        "// This file is auto-generated by tool-reactor. Do not edit.\n\n{}\n\n{}",
        WIDGET_DECLARATIONS.trim(),
        binding_body.trim(),
    );
    write_generated_rs(&root.join(GENERATED_DECLARATIONS_OUTPUT), &declarations)?;
    write_generated_rs(
        &root.join(GENERATED_SET_PROP_OUTPUT),
        &native::set_prop::generate(&controls, &resolver),
    )?;
    write_generated_rs(
        &root.join(GENERATED_ATTACH_EVENT_OUTPUT),
        &native::attach_event::generate(&controls, &resolver),
    )?;
    write_generated_rs(
        &root.join(GENERATED_BACKEND_MODULE_OUTPUT),
        native::BACKEND_GENERATED_MODULE,
    )
}

fn stage_reactor_metadata(root: &Path) -> Result<PathBuf, String> {
    let work = root
        .join("target")
        .join("tool-reactor")
        .join("reactor-metadata");
    recreate_dir(&work)?;

    let windows_winmd = locate_windows_bindgen_default_winmd(root)?;
    copy_file(&windows_winmd, &work.join("Windows.winmd"))?;
    Ok(work)
}

fn locate_windows_bindgen_default_winmd(root: &Path) -> Result<PathBuf, String> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(root.join("Cargo.toml"))
        .exec()
        .map_err(|err| format!("cargo metadata failed: {err}"))?;

    let package = metadata
        .packages
        .iter()
        .find(|package| package.name.as_str() == "windows-bindgen")
        .ok_or_else(|| "cargo metadata did not include windows-bindgen".to_string())?;
    let manifest = package.manifest_path.as_std_path();
    let winmd = manifest
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| {
            format!(
                "windows-bindgen manifest has no libraries directory: {}",
                manifest.display()
            )
        })?
        .join("default")
        .join("Windows.winmd");
    require_file(&winmd, "windows-bindgen default Windows.winmd")?;
    Ok(winmd)
}

fn write_generated_rs(path: &Path, code: &str) -> Result<(), String> {
    write_if_changed(path, &rustfmt(code))
}

fn write_if_changed(path: &Path, content: &str) -> Result<(), String> {
    if let Ok(existing) = fs::read_to_string(path)
        && existing == content
    {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(path, content).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn rustfmt(code: &str) -> String {
    let Ok(mut child) = Command::new("rustfmt")
        .arg("--edition=2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    else {
        return code.to_string();
    };
    if let Some(stdin) = child.stdin.as_mut() {
        let _ = stdin.write_all(code.as_bytes());
    }
    match child.wait_with_output() {
        Ok(output) if output.status.success() => {
            String::from_utf8(output.stdout).unwrap_or_else(|_| code.to_string())
        }
        _ => code.to_string(),
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("could not resolve workspace root from {}", dir.display()))
}

fn recreate_dir(path: &Path) -> Result<(), String> {
    if path.exists() {
        fs::remove_dir_all(path)
            .map_err(|err| format!("failed to remove {}: {err}", path.display()))?;
    }
    fs::create_dir_all(path).map_err(|err| format!("failed to create {}: {err}", path.display()))
}

fn copy_file(src: &Path, dst: &Path) -> Result<(), String> {
    require_file(src, "source file")?;
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::copy(src, dst).map(|_| ()).map_err(|err| {
        format!(
            "failed to copy {} to {}: {err}",
            src.display(),
            dst.display()
        )
    })
}

fn copy_if_changed(src: &Path, dst: &Path) -> Result<(), String> {
    require_file(src, "generated bindings")?;
    if let (Ok(existing), Ok(generated)) = (fs::read(dst), fs::read(src))
        && existing == generated
    {
        return Ok(());
    }
    copy_file(src, dst)
}

fn mark_generated(path: &Path) -> Result<(), String> {
    const HEADER: &str = "// This file is auto-generated by tool-reactor. Do not edit.\n";
    let mut code = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    if !code.starts_with(HEADER) {
        code.insert_str(0, HEADER);
        fs::write(path, code)
            .map_err(|err| format!("failed to write {}: {err}", path.display()))?;
    }
    Ok(())
}

fn patch_xaml_bindings(path: &Path) -> Result<(), String> {
    let mut code = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    patch_windows_core_api(&mut code);
    insert_selection_changed_event_handler_ctor(&mut code)?;
    fs::write(path, code).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn patch_windows_core_api(code: &mut String) {
    for name in ["Type", "CopyType", "CloneType"] {
        *code = code.replace(
            &format!("windows_core::{name}"),
            &format!("windows_core::imp::{name}"),
        );
    }
    *code = code
        .replace("windows_core::imp::Box::new", "std::boxed::Box::new")
        .replace("windows_core::Box::new", "std::boxed::Box::new");
}

fn insert_selection_changed_event_handler_ctor(code: &mut String) -> Result<(), String> {
    if code.contains("impl SelectionChangedEventHandler {\n    pub fn new<") {
        return Ok(());
    }

    let ctor = r#"struct SelectionChangedEventHandlerResultBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> SelectionChangedEventHandlerResultBox<F>
{
    const VTABLE: SelectionChangedEventHandler_Vtbl = SelectionChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<SelectionChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            )
            .into()
        }
    }
}
impl SelectionChangedEventHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SelectionChangedEventArgs>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    {
        let com = windows_core::imp::DelegateBox::<SelectionChangedEventHandler, _>::new(
            &SelectionChangedEventHandlerResultBox::<F>::VTABLE,
            handler,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
"#;

    for needle in [
        "#[repr(transparent)]\r\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\r\npub struct SelectionMode(pub i32);",
        "#[repr(transparent)]\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\npub struct SelectionMode(pub i32);",
        "#[repr(transparent)]\r\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\r\npub struct ListViewSelectionMode(pub i32);",
        "#[repr(transparent)]\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\npub struct ListViewSelectionMode(pub i32);",
    ] {
        if let Some(index) = code.find(needle) {
            code.insert_str(index, ctor);
            return Ok(());
        }
    }

    Err("failed to patch SelectionChangedEventHandler constructor".to_string())
}

fn require_file(path: &Path, label: &str) -> Result<(), String> {
    if path.is_file() {
        Ok(())
    } else {
        Err(format!("{label} not found: {}", path.display()))
    }
}
