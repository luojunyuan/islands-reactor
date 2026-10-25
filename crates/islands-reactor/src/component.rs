use super::*;

pub(crate) mod component_element {
    use std::any::{Any, TypeId};
    use std::rc::Rc;

    use super::*;

    pub(crate) trait ComponentObject: 'static {
        fn render(&self, cx: &mut RenderCx) -> Element;

        fn props_as_any(&self) -> &dyn Any;

        fn component_type_id(&self) -> TypeId;

        fn is_equivalent(&self, other: &dyn ComponentObject) -> bool;

        fn should_update(&self, other: &dyn ComponentObject) -> bool;

        fn invoke_appeared(&self, cx: &mut RenderCx);

        fn invoke_disappeared(&self, cx: &mut RenderCx);

        fn has_on_appeared(&self) -> bool;

        fn has_on_disappeared(&self) -> bool;
    }

    struct ComponentCell<C, P>
    where
        C: Component<P>,
        P: Clone + PartialEq + 'static,
    {
        component: C,
        props: P,
    }

    impl<C, P> ComponentObject for ComponentCell<C, P>
    where
        C: Component<P>,
        P: Clone + PartialEq + 'static,
    {
        fn render(&self, cx: &mut RenderCx) -> Element {
            self.component.render(&self.props, cx)
        }

        fn props_as_any(&self) -> &dyn Any {
            &self.props
        }

        fn component_type_id(&self) -> TypeId {
            TypeId::of::<C>()
        }

        fn is_equivalent(&self, other: &dyn ComponentObject) -> bool {
            if self.component_type_id() != other.component_type_id() {
                return false;
            }
            let Some(other_props) = other.props_as_any().downcast_ref::<P>() else {
                return false;
            };
            &self.props == other_props
        }

        fn should_update(&self, other: &dyn ComponentObject) -> bool {
            if self.component_type_id() != other.component_type_id() {
                return true;
            }
            let Some(other_props) = other.props_as_any().downcast_ref::<P>() else {
                return true;
            };

            self.component.should_update(&self.props, other_props)
        }

        fn invoke_appeared(&self, cx: &mut RenderCx) {
            self.component.on_appeared(&self.props, cx);
        }

        fn invoke_disappeared(&self, cx: &mut RenderCx) {
            self.component.on_disappeared(&self.props, cx);
        }

        fn has_on_appeared(&self) -> bool {
            self.component.has_on_appeared()
        }

        fn has_on_disappeared(&self) -> bool {
            self.component.has_on_disappeared()
        }
    }

    /// Erased [`Component`]+props pair carried inside [`Element::Component`].
    #[doc(hidden)]
    pub struct ComponentElement {
        pub key: Option<String>,
        pub memoised: bool,
        pub(crate) obj: Rc<dyn ComponentObject>,
    }

    impl Clone for ComponentElement {
        fn clone(&self) -> Self {
            Self {
                key: self.key.clone(),
                memoised: self.memoised,
                obj: Rc::clone(&self.obj),
            }
        }
    }

    impl std::fmt::Debug for ComponentElement {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ComponentElement")
                .field("key", &self.key)
                .field("memoised", &self.memoised)
                .field("component_type_id", &self.obj.component_type_id())
                .finish()
        }
    }

    impl PartialEq for ComponentElement {
        fn eq(&self, other: &Self) -> bool {
            if self.key != other.key || self.memoised != other.memoised {
                return false;
            }
            if self.memoised {
                self.obj.is_equivalent(&*other.obj)
            } else {
                !self.obj.should_update(&*other.obj)
            }
        }
    }

    /// Wrap a [`Component`] + props into an [`Element`] for embedding in a tree.
    pub fn component<C, P>(component: C, props: P) -> Element
    where
        C: Component<P>,
        P: Clone + PartialEq + 'static,
    {
        let cell = ComponentCell { component, props };
        Element::Component(ComponentElement {
            key: None,
            memoised: false,
            obj: Rc::new(cell),
        })
    }

    /// Like [`component()`](crate::component_element::component()), but treat the component as memoised: the host
    /// reuses the previous render output when props compare equal.
    pub fn memo<C, P>(component: C, props: P) -> Element
    where
        C: Component<P>,
        P: Clone + PartialEq + 'static,
    {
        let cell = ComponentCell { component, props };
        Element::Component(ComponentElement {
            key: None,
            memoised: true,
            obj: Rc::new(cell),
        })
    }
}
pub(crate) mod context {
    use std::any::{Any, TypeId};
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU32, Ordering};

    use std::collections::HashMap;

    use super::element::Element;

    /// Process-wide unique identifier for a [`Context`] of any value type.
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct ContextId(u32);

    static NEXT_CONTEXT_ID: AtomicU32 = AtomicU32::new(1);

    impl ContextId {
        pub fn new() -> Self {
            Self(NEXT_CONTEXT_ID.fetch_add(1, Ordering::Relaxed))
        }
    }

    impl Default for ContextId {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Typed lookup key + default for a value made available to descendants
    /// via [`ProviderElement`] and [`RenderCx::use_context()`](crate::RenderCx::use_context).
    pub struct Context<T> {
        pub default: T,
        pub id: ContextId,
    }

    impl<T> Context<T> {
        pub fn new(default: T) -> Self {
            Self {
                default,
                id: ContextId::new(),
            }
        }
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for Context<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Context")
                .field("id", &self.id)
                .field("default", &self.default)
                .finish()
        }
    }

    struct ContextEntry {
        value_type_id: TypeId,
        value: Rc<dyn Any>,
    }

    #[derive(Default)]
    pub struct ContextStack {
        entries: std::cell::RefCell<HashMap<ContextId, Vec<ContextEntry>>>,
        push_order: std::cell::RefCell<Vec<ContextId>>,
    }

    impl ContextStack {
        pub fn new() -> Self {
            Self::default()
        }

        #[cfg(test)]
        pub(crate) fn push<T: 'static>(&self, context_id: ContextId, value: T) -> ContextScope<'_> {
            self.push_raw_retain(context_id, TypeId::of::<T>(), Rc::new(value));
            ContextScope { stack: self }
        }

        pub(crate) fn push_raw_retain(
            &self,
            context_id: ContextId,
            value_type_id: TypeId,
            value: Rc<dyn Any>,
        ) {
            self.entries
                .borrow_mut()
                .entry(context_id)
                .or_default()
                .push(ContextEntry {
                    value_type_id,
                    value,
                });
            self.push_order.borrow_mut().push(context_id);
        }

        pub(crate) fn pop_raw(&self) {
            if let Some(id) = self.push_order.borrow_mut().pop() {
                let mut entries = self.entries.borrow_mut();
                if let Some(stack) = entries.get_mut(&id) {
                    stack.pop();
                    if stack.is_empty() {
                        entries.remove(&id);
                    }
                }
            }
        }

        pub fn get<T: 'static + Clone>(&self, context_id: ContextId) -> Option<T> {
            let entries = self.entries.borrow();
            let entry = entries.get(&context_id)?.last()?;
            assert!(
                entry.value_type_id == TypeId::of::<T>(),
                "context type mismatch for context {:?}: provided as {:?}, consumed as {:?}",
                context_id,
                entry.value_type_id,
                TypeId::of::<T>()
            );
            entry.value.downcast_ref::<T>().cloned()
        }

        pub fn depth(&self) -> usize {
            self.push_order.borrow().len()
        }

        #[cfg(test)]
        fn pop(&self) {
            self.pop_raw();
        }
    }

    #[cfg(test)]
    pub(crate) struct ContextScope<'a> {
        stack: &'a ContextStack,
    }

    #[cfg(test)]
    impl Drop for ContextScope<'_> {
        fn drop(&mut self) {
            self.stack.pop();
        }
    }

    #[derive(Clone)]
    pub struct ContextProvision {
        pub(crate) context_id: ContextId,
        pub(crate) value_type_id: TypeId,
        pub(crate) value: Rc<dyn Any>,
        eq_fn: fn(&dyn Any, &dyn Any) -> bool,
    }

    impl ContextProvision {
        pub fn new<T: Clone + PartialEq + 'static>(id: ContextId, value: T) -> Self {
            Self {
                context_id: id,
                value_type_id: TypeId::of::<T>(),
                value: Rc::new(value),
                eq_fn: |a, b| match (a.downcast_ref::<T>(), b.downcast_ref::<T>()) {
                    (Some(a), Some(b)) => a == b,
                    _ => false,
                },
            }
        }
    }

    impl std::fmt::Debug for ContextProvision {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ContextProvision")
                .field("context_id", &self.context_id)
                .field("value_type_id", &self.value_type_id)
                .finish()
        }
    }

    impl PartialEq for ContextProvision {
        fn eq(&self, other: &Self) -> bool {
            if self.context_id != other.context_id || self.value_type_id != other.value_type_id {
                return false;
            }
            if Rc::ptr_eq(&self.value, &other.value) {
                return true;
            }
            (self.eq_fn)(self.value.as_ref(), other.value.as_ref())
        }
    }

    #[derive(Clone, Debug)]
    pub struct ProviderElement {
        pub key: Option<String>,
        pub provisions: Vec<ContextProvision>,
        pub child: Box<Element>,
    }

    impl PartialEq for ProviderElement {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
                && self.provisions == other.provisions
                && self.child == other.child
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn context_ids_are_unique() {
            let a = ContextId::new();
            let b = ContextId::new();
            assert_ne!(a, b);
        }

        #[test]
        fn context_new_assigns_unique_id() {
            let a = Context::<i32>::new(0);
            let b = Context::<i32>::new(0);
            assert_ne!(a.id, b.id);
        }

        #[test]
        fn stack_get_returns_none_when_empty() {
            let s = ContextStack::new();
            let id = ContextId::new();
            assert!(s.get::<i32>(id).is_none());
        }

        #[test]
        fn stack_push_pop_shadows_lookup() {
            let s = ContextStack::new();
            let id = ContextId::new();
            assert!(s.get::<i32>(id).is_none());
            {
                let _g = s.push(id, 42_i32);
                assert_eq!(s.depth(), 1);
                assert_eq!(s.get::<i32>(id), Some(42));
            }
            assert_eq!(s.depth(), 0);
            assert!(s.get::<i32>(id).is_none());
        }

        #[test]
        fn nested_provides_return_innermost() {
            let s = ContextStack::new();
            let id = ContextId::new();
            let _a = s.push(id, "outer".to_string());
            {
                let _b = s.push(id, "inner".to_string());
                assert_eq!(s.get::<String>(id), Some("inner".into()));
            }
            assert_eq!(s.get::<String>(id), Some("outer".into()));
        }

        #[test]
        fn distinct_ids_dont_shadow() {
            let s = ContextStack::new();
            let id_a = ContextId::new();
            let id_b = ContextId::new();
            let _a = s.push(id_a, 1_i32);
            let _b = s.push(id_b, 2_i32);
            assert_eq!(s.get::<i32>(id_a), Some(1));
            assert_eq!(s.get::<i32>(id_b), Some(2));
        }

        #[test]
        #[should_panic(expected = "context type mismatch")]
        fn mismatched_types_panic() {
            let s = ContextStack::new();
            let id = ContextId::new();
            let _g = s.push(id, 1_i32);
            let _ = s.get::<String>(id);
        }
    }
}
pub(crate) mod custom {
    //! Extension hatch for downstream widgets that aren't part of the core
    //! [`Element`](crate::element::Element) enum. Leaf-only in step 1a.

    use std::any::{Any, TypeId};

    use crate::ir::{Backend, ControlId};

    /// Out-of-tree widget definition managed by the reconciler via
    /// [`Element::Custom`](crate::element::Element::Custom).
    pub trait CustomElement: 'static {
        /// Standard `Any` accessor; the implementor's body is almost always `self`.
        fn as_any(&self) -> &dyn Any;

        /// Stable type identity used by the reconciler to decide
        /// mount-vs-update; defaults to the underlying `Any::type_id`.
        fn type_id(&self) -> TypeId {
            self.as_any().type_id()
        }

        /// Human-readable name surfaced in diagnostics and `Element::kind_name`.
        fn kind_name(&self) -> &'static str;

        /// Optional key for keyed reconciliation inside multi-child containers.
        fn key(&self) -> Option<&str> {
            None
        }

        /// Structural equality against another element of the same `type_id`;
        /// returning `false` is always safe but skips an `update` short-circuit.
        fn eq_dyn(&self, other: &dyn CustomElement) -> bool;

        /// Boxed clone so [`Element`](crate::element::Element) stays `Clone`.
        fn clone_dyn(&self) -> Box<dyn CustomElement>;

        /// Create the underlying control via `backend` and return its id.
        fn mount(&self, backend: &mut dyn Backend) -> ControlId;

        /// Apply the diff from `prev` (same `type_id`) to the live control `id`.
        fn update(&self, prev: &dyn CustomElement, id: ControlId, backend: &mut dyn Backend);

        /// Hook fired just before [`Backend::destroy`]; defaults to no-op.
        fn before_destroy(&self, _id: ControlId, _backend: &mut dyn Backend) {}
    }

    /// Boxed [`CustomElement`] with `Clone` / `Debug` / `PartialEq` so it can
    /// live inside the `Element` enum.
    pub struct CustomElementHandle(pub Box<dyn CustomElement>);

    impl CustomElementHandle {
        pub fn new<C: CustomElement>(c: C) -> Self {
            Self(Box::new(c))
        }
        pub fn get(&self) -> &dyn CustomElement {
            &*self.0
        }
    }

    impl Clone for CustomElementHandle {
        fn clone(&self) -> Self {
            Self(self.0.clone_dyn())
        }
    }

    impl std::fmt::Debug for CustomElementHandle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("CustomElement")
                .field("kind", &self.0.kind_name())
                .field("type_id", &CustomElement::type_id(&*self.0))
                .field("key", &self.0.key())
                .finish()
        }
    }

    impl PartialEq for CustomElementHandle {
        fn eq(&self, other: &Self) -> bool {
            if CustomElement::type_id(&*self.0) != CustomElement::type_id(&*other.0) {
                return false;
            }
            if self.0.key() != other.0.key() {
                return false;
            }
            self.0.eq_dyn(&*other.0)
        }
    }
}
pub(crate) mod error_boundary {
    use std::any::Any;

    use super::*;

    crate::impl_rc_fn_wrapper! {
        /// Renders a fallback subtree given a panic message string.
        pub struct Fallback(dyn Fn(&str) -> Element);
    }

    impl Fallback {
        pub fn invoke(&self, message: &str) -> Element {
            (self.inner)(message)
        }
    }

    /// Subtree wrapper that catches panics from descendants and renders the
    /// `fallback` element instead. Carried by [`Element::ErrorBoundary`].
    #[doc(hidden)]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ErrorBoundaryElement {
        pub key: Option<String>,
        pub child: Box<Element>,
        pub fallback: Fallback,
    }

    /// Wrap `child` in an [`ErrorBoundaryElement`] using `fallback` for recovery.
    pub fn error_boundary<F>(child: impl Into<Element>, fallback: F) -> Element
    where
        F: Fn(&str) -> Element + 'static,
    {
        Element::ErrorBoundary(ErrorBoundaryElement {
            key: None,
            child: Box::new(child.into()),
            fallback: Fallback::new(fallback),
        })
    }

    pub(crate) fn panic_message(payload: Box<dyn Any + Send>) -> String {
        if let Some(s) = payload.downcast_ref::<&'static str>() {
            (*s).to_string()
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else {
            "<non-string panic>".to_string()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fallback_invoke_returns_element() {
            let fb = Fallback::new(|msg| TextBlock::new(format!("oops: {msg}")).into());
            let e = fb.invoke("boom");
            match e {
                Element::TextBlock(t) => assert_eq!(t.text, "oops: boom"),
                other => panic!("unexpected {other:?}"),
            }
        }

        #[test]
        fn fallback_clone_ptr_eq() {
            let fb1 = Fallback::new(|_| Element::Empty);
            let fb2 = fb1.clone();
            let fb3 = Fallback::new(|_| Element::Empty);
            assert_eq!(fb1, fb2);
            assert_ne!(fb1, fb3);
        }

        #[test]
        fn error_boundary_factory_produces_variant() {
            let eb = error_boundary(TextBlock::new("child"), |_| Element::Empty);
            assert!(matches!(eb, Element::ErrorBoundary(_)));
        }

        #[test]
        fn panic_message_handles_known_payload_shapes() {
            let e: Box<dyn Any + Send> = Box::new("literal");
            assert_eq!(panic_message(e), "literal");
            let e: Box<dyn Any + Send> = Box::new(String::from("owned"));
            assert_eq!(panic_message(e), "owned");
            let e: Box<dyn Any + Send> = Box::new(42_i32);
            assert_eq!(panic_message(e), "<non-string panic>");
        }
    }
}
mod rc_fn {
    /// Generate an `Rc`-pointer-equal newtype around `dyn Fn(...)`. Provides
    /// `new`, `Clone`, `Debug` (pointer-formatted), and `PartialEq`/`Eq`
    /// based on `Rc::ptr_eq`.
    #[macro_export]
    #[doc(hidden)]
    macro_rules! impl_rc_fn_wrapper {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident $(< $param:ident >)? (
            dyn Fn ( $($args:tt)* ) $(-> $ret:ty)?
        );
    ) => {
        $(#[$attr])*
        $vis struct $name $(< $param >)? {
            inner: ::std::rc::Rc<dyn Fn($($args)*) $(-> $ret)?>,
        }

        impl $(< $param >)? $name $(< $param >)? {
            pub fn new<__F>(f: __F) -> Self
            where
                __F: Fn($($args)*) $(-> $ret)? + 'static,
            {
                Self { inner: ::std::rc::Rc::new(f) }
            }
        }

        impl $(< $param >)? ::std::clone::Clone for $name $(< $param >)? {
            fn clone(&self) -> Self {
                Self { inner: ::std::rc::Rc::clone(&self.inner) }
            }
        }

        impl $(< $param >)? ::std::fmt::Debug for $name $(< $param >)? {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&::std::format_args!(
                        "{:p}",
                        ::std::rc::Rc::as_ptr(&self.inner)
                    ))
                    .finish()
            }
        }

        impl $(< $param >)? ::std::cmp::PartialEq for $name $(< $param >)? {
            fn eq(&self, other: &Self) -> bool {
                ::std::rc::Rc::ptr_eq(&self.inner, &other.inner)
            }
        }

        impl $(< $param >)? ::std::cmp::Eq for $name $(< $param >)? {}
    };
}
}
pub(crate) mod render_context {
    use std::any::Any;
    use std::cell::{Cell, Ref, RefCell};
    use std::fmt;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex, OnceLock};
    use std::thread::{self, ThreadId};

    use std::collections::HashSet;

    use super::*;

    /// Type-erased dependency tuple used to gate `use_memo` / `use_effect`
    /// re-runs; auto-implemented for any `T: PartialEq + Clone + 'static`.
    pub trait Deps: 'static {
        fn as_any(&self) -> &dyn Any;

        fn deps_eq(&self, other: &dyn Deps) -> bool;

        fn clone_boxed(&self) -> Box<dyn Deps>;
    }

    impl<T: PartialEq + Clone + 'static> Deps for T {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn deps_eq(&self, other: &dyn Deps) -> bool {
            match other.as_any().downcast_ref::<T>() {
                Some(o) => self == o,
                None => false,
            }
        }

        fn clone_boxed(&self) -> Box<dyn Deps> {
            Box::new(self.clone())
        }
    }

    type Cleanup = Box<dyn FnOnce()>;

    type PendingEffect = Box<dyn FnOnce() -> Option<Cleanup>>;

    pub(crate) enum HookSlot {
        State {
            cell: Rc<RefCell<Box<dyn Any>>>,
            type_name: &'static str,
        },
        Memo {
            value: Rc<RefCell<Box<dyn Any>>>,
            deps: Option<Box<dyn Deps>>,
        },
        Effect {
            deps: Option<Box<dyn Deps>>,
            pending: Option<PendingEffect>,
            cleanup: Option<Cleanup>,
            pending_cleanup: Option<Cleanup>,
        },
        /// State slot whose cell is `Arc<Mutex<…>>` so writes can come from
        /// any thread. Backs [`RenderCx::use_async_state`].
        AsyncState {
            cell: Arc<Mutex<Box<dyn Any + Send>>>,
            type_name: &'static str,
        },
    }

    impl fmt::Debug for HookSlot {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                HookSlot::State { type_name, .. } => write!(f, "State({type_name})"),
                HookSlot::Memo { .. } => f.write_str("Memo { .. }"),
                HookSlot::Effect {
                    pending, cleanup, ..
                } => f
                    .debug_struct("Effect")
                    .field("pending", &pending.is_some())
                    .field("has_cleanup", &cleanup.is_some())
                    .finish(),
                HookSlot::AsyncState { type_name, .. } => write!(f, "AsyncState({type_name})"),
            }
        }
    }

    crate::impl_rc_fn_wrapper! {
        /// Setter returned by `use_state`; replaces the slot value and
        /// requests a rerender.
        pub struct SetState<T>(dyn Fn(T));
    }

    impl<T> SetState<T> {
        pub fn call(&self, value: T) {
            (self.inner)(value);
        }
    }

    impl<T: 'static> From<SetState<T>> for super::callback::Callback<T> {
        fn from(s: SetState<T>) -> Self {
            Self::from_rc(s.inner)
        }
    }

    type ReducerClosure<T> = Box<dyn FnOnce(T) -> T>;

    crate::impl_rc_fn_wrapper! {
        /// Functional updater returned by `use_state`; mutates the slot value
        /// via `f(prev) -> next` and requests a rerender.
        pub struct Updater<T>(dyn Fn(ReducerClosure<T>));
    }

    impl<T: 'static> Updater<T> {
        pub fn call<F>(&self, reducer: F)
        where
            F: FnOnce(T) -> T + 'static,
        {
            (self.inner)(Box::new(reducer));
        }
    }

    crate::impl_rc_fn_wrapper! {
        /// Action dispatcher returned by `use_reducer`.
        pub struct Dispatch<A>(dyn Fn(A));
    }

    impl<A> Dispatch<A> {
        pub fn call(&self, action: A) {
            (self.inner)(action);
        }
    }

    impl<A: 'static> From<Dispatch<A>> for super::callback::Callback<A> {
        fn from(d: Dispatch<A>) -> Self {
            Self::from_rc(d.inner)
        }
    }

    /// Thread-safe setter returned by [`RenderCx::use_async_state`]. Calling
    /// `.call(value)` from any thread auto-marshals the state write + rerender
    /// request back onto the UI thread via the host's [`UiMarshaller`].
    pub struct AsyncSetState<T: Send + 'static> {
        cell: Arc<Mutex<Box<dyn Any + Send>>>,
        marshaller: UiMarshaller,
        /// Owning component's `state_dirty` flag (shared with `RenderCx`), set on
        /// a real change so the reconciler re-renders the component even when
        /// nested under a parent whose element tree is unchanged.
        dirty: Arc<AtomicBool>,
        type_name: &'static str,
        _marker: std::marker::PhantomData<fn(T)>,
    }

    impl<T: Send + 'static> Clone for AsyncSetState<T> {
        fn clone(&self) -> Self {
            Self {
                cell: Arc::clone(&self.cell),
                marshaller: self.marshaller.clone(),
                dirty: Arc::clone(&self.dirty),
                type_name: self.type_name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    impl<T: Send + 'static> fmt::Debug for AsyncSetState<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AsyncSetState")
                .field("type", &self.type_name)
                .finish_non_exhaustive()
        }
    }

    impl<T: Send + Clone + PartialEq + 'static> AsyncSetState<T> {
        /// Apply `value` to the state slot from any thread. The write is
        /// marshalled to the UI thread; no-op if value is unchanged.
        pub fn call(&self, value: T) {
            let cell = Arc::clone(&self.cell);
            let dirty = Arc::clone(&self.dirty);
            let type_name = self.type_name;
            self.marshaller.dispatch(move || {
                let mut slot = cell.lock().unwrap();
                let prev = slot.downcast_ref::<T>().unwrap_or_else(|| {
                    panic!(
                        "AsyncSetState slot type mismatch (expected `{type_name}`): \
                     hook order invariant violated"
                    )
                });
                if *prev == value {
                    return;
                }
                *slot = Box::new(value);
                drop(slot);
                // Mark the owning component dirty so the reconciler does not skip
                // it; a bare rerender request only re-renders the root.
                dirty.store(true, Ordering::Relaxed);
                request_ui_rerender_on_ui_thread();
            });
        }
    }

    /// `Rc<RefCell<T>>` wrapper returned by `use_ref`; identity-stable across
    /// renders so callers can pin mutable state outside the hooks vector.
    pub struct HookRef<T> {
        inner: Rc<RefCell<T>>,
    }

    impl<T> HookRef<T> {
        pub fn borrow(&self) -> Ref<'_, T> {
            self.inner.borrow()
        }

        pub fn borrow_mut(&self) -> std::cell::RefMut<'_, T> {
            self.inner.borrow_mut()
        }

        pub fn replace(&self, v: T) -> T {
            self.inner.replace(v)
        }

        pub fn set(&self, v: T) {
            *self.inner.borrow_mut() = v;
        }
    }

    impl<T: Clone> HookRef<T> {
        pub fn get_cloned(&self) -> T {
            self.inner.borrow().clone()
        }
    }

    impl<T> Clone for HookRef<T> {
        fn clone(&self) -> Self {
            Self {
                inner: Rc::clone(&self.inner),
            }
        }
    }

    impl<T: fmt::Debug> fmt::Debug for HookRef<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.inner.try_borrow() {
                Ok(b) => f.debug_tuple("HookRef").field(&*b).finish(),
                Err(_) => f.debug_tuple("HookRef").field(&"<borrowed>").finish(),
            }
        }
    }

    /// Stable `ContextId` used to track which components subscribed to
    /// [`RenderCx::use_inner_size`].
    pub(crate) fn inner_size_context_id() -> ContextId {
        static ID: OnceLock<ContextId> = OnceLock::new();
        *ID.get_or_init(ContextId::new)
    }

    /// Stable `ContextId` used to track which components subscribed to
    /// [`RenderCx::use_dpi`].
    pub(crate) fn dpi_context_id() -> ContextId {
        static ID: OnceLock<ContextId> = OnceLock::new();
        *ID.get_or_init(ContextId::new)
    }

    /// Per-component render context: hooks vector cursor, the rerender
    /// request closure, and ambient context-provider stack. Passed to every
    /// [`Component::render`] call.
    pub struct RenderCx {
        hooks: Rc<RefCell<Vec<HookSlot>>>,
        cursor: usize,
        request_rerender: Rc<dyn Fn()>,
        /// Shared flag set whenever a hook value changes, by both the synchronous
        /// setters (`SetState`/`Updater`/`Dispatch`) and the off-thread
        /// [`AsyncSetState`]. The reconciler checks this to force a re-render of
        /// the owning component even when the parent's element tree is
        /// structurally identical, so nested components are not skipped after a
        /// state update. It is an `Arc<AtomicBool>` rather than a `Cell` because
        /// [`AsyncSetState`] captures it into a `Send` closure that is marshalled
        /// onto the UI thread; the synchronous setters only ever touch it from the
        /// UI thread.
        state_dirty: Arc<AtomicBool>,
        ui_thread: Option<ThreadId>,
        context_stack: Option<Rc<ContextStack>>,
        read_contexts: RefCell<HashSet<ContextId>>,
        /// `Send + Sync` handle to the UI thread's render-aware dispatcher,
        /// populated by the host for [`Self::use_ui_marshaller`] /
        /// [`Self::use_async_state`].
        marshaller: Option<UiMarshaller>,
        inner_size: Rc<Cell<Size>>,
        /// Per-monitor DPI for the host window. Shared with the
        /// reconciler via [`Self::set_dpi_cell`]; updated by the host when the
        /// window moves across monitors.
        dpi: Rc<Cell<u32>>,
    }

    impl fmt::Debug for RenderCx {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderCx")
                .field("cursor", &self.cursor)
                .field("hook_count", &self.hooks.borrow().len())
                .field("ui_thread", &self.ui_thread)
                .field("has_marshaller", &self.marshaller.is_some())
                .finish()
        }
    }

    impl RenderCx {
        pub fn new(request_rerender: Rc<dyn Fn()>) -> Self {
            Self {
                hooks: Rc::new(RefCell::new(Vec::new())),
                cursor: 0,
                request_rerender,
                state_dirty: Arc::new(AtomicBool::new(false)),
                ui_thread: None,
                context_stack: None,
                read_contexts: RefCell::new(HashSet::default()),
                marshaller: None,
                inner_size: Rc::new(Cell::new(Size::default())),
                dpi: Rc::new(Cell::new(96)),
            }
        }

        pub fn take_read_contexts(&mut self) -> HashSet<ContextId> {
            std::mem::take(&mut *self.read_contexts.borrow_mut())
        }

        pub fn set_context_stack(&mut self, stack: Rc<ContextStack>) {
            self.context_stack = Some(stack);
        }

        pub fn use_context<T>(&self, context: &Context<T>) -> T
        where
            T: 'static + Clone,
        {
            self.read_contexts.borrow_mut().insert(context.id);

            if let Some(stack) = &self.context_stack
                && let Some(v) = stack.get::<T>(context.id)
            {
                return v;
            }
            context.default.clone()
        }

        pub fn for_test() -> Self {
            Self::new(Rc::new(|| {}))
        }

        pub fn begin_render(&mut self) {
            self.cursor = 0;
            self.ui_thread = Some(thread::current().id());
            self.read_contexts.borrow_mut().clear();
            self.state_dirty.store(false, Ordering::Relaxed);
        }

        pub fn hook_count(&self) -> usize {
            self.hooks.borrow().len()
        }

        pub fn set_request_rerender(&mut self, request_rerender: Rc<dyn Fn()>) {
            self.request_rerender = request_rerender;
        }

        /// Returns `true` if any state write (synchronous or off-thread async)
        /// has changed a hook value since the last render, and clears the flag.
        pub fn take_state_dirty(&self) -> bool {
            self.state_dirty.swap(false, Ordering::Relaxed)
        }

        /// Returns the current dirty state without clearing it.
        pub fn peek_state_dirty(&self) -> bool {
            self.state_dirty.load(Ordering::Relaxed)
        }

        /// Install (or replace) the [`UiMarshaller`] used by
        /// [`Self::use_ui_marshaller`] / [`Self::use_async_state`].
        pub fn set_marshaller(&mut self, marshaller: Option<UiMarshaller>) {
            self.marshaller = marshaller;
        }

        /// Returns the UI-thread marshaller installed by the host.
        pub fn use_ui_marshaller(&mut self) -> UiMarshaller {
            self.marshaller
                .clone()
                .expect("use_ui_marshaller requires a WinUI host")
        }

        /// Like [`Self::use_state`], but returns an [`AsyncSetState<T>`]
        /// whose `.call(value)` is safe to invoke from any thread; writes
        /// auto-marshal onto the UI thread via the host's [`UiMarshaller`].
        pub fn use_async_state<T>(&mut self, initial: T) -> (T, AsyncSetState<T>)
        where
            T: 'static + Send + Clone + PartialEq,
        {
            let slot_index = self.cursor;
            self.cursor += 1;

            self.debug_assert_on_ui_thread(slot_index);

            let (cell, type_name) = {
                let mut hooks = self.hooks.borrow_mut();
                if slot_index == hooks.len() {
                    let boxed: Box<dyn Any + Send> = Box::new(initial);
                    let cell = Arc::new(Mutex::new(boxed));
                    let type_name = std::any::type_name::<T>();
                    hooks.push(HookSlot::AsyncState {
                        cell: Arc::clone(&cell),
                        type_name,
                    });
                    (cell, type_name)
                } else {
                    match &hooks[slot_index] {
                        HookSlot::AsyncState { cell, type_name } => (Arc::clone(cell), *type_name),
                        _ => panic!(
                            "hook called in different order: slot {slot_index} held a \
                         non-AsyncState slot, now called as `use_async_state`"
                        ),
                    }
                }
            };

            let current = {
                let slot = cell.lock().unwrap();
                match slot.downcast_ref::<T>() {
                    Some(v) => v.clone(),
                    None => panic!(
                        "hook called in different order: slot {slot_index} was \
                     `{type_name}`, now called as `use_async_state::<{requested}>`",
                        requested = std::any::type_name::<T>()
                    ),
                }
            };

            let marshaller = self
                .marshaller
                .clone()
                .expect("use_async_state requires a WinUI host");

            let setter = AsyncSetState::<T> {
                cell,
                marshaller,
                dirty: Arc::clone(&self.state_dirty),
                type_name,
                _marker: std::marker::PhantomData,
            };

            (current, setter)
        }

        pub fn use_state<T>(&mut self, initial: T) -> (T, SetState<T>)
        where
            T: 'static + Clone + PartialEq,
        {
            let (current, cell, slot_index) = self.state_slot(initial);
            let setter = self.make_state_setter::<T>(cell, slot_index);
            (current, setter)
        }

        pub fn use_reducer<T>(&mut self, initial: T) -> (T, Updater<T>)
        where
            T: 'static + Clone + PartialEq,
        {
            let (current, cell, slot_index) = self.state_slot(initial);
            let updater = self.make_updater::<T>(cell, slot_index);
            (current, updater)
        }

        pub fn use_reducer_fn<S, A, R>(&mut self, reducer: R, initial: S) -> (S, Dispatch<A>)
        where
            S: 'static + Clone + PartialEq,
            A: 'static,
            R: Fn(S, A) -> S + 'static,
        {
            let (current, cell, slot_index) = self.state_slot(initial);
            let dispatch = self.make_dispatch::<S, A, R>(cell, slot_index, reducer);
            (current, dispatch)
        }

        pub fn use_ref<T: 'static>(&mut self, initial: T) -> HookRef<T> {
            let slot_index = self.cursor;
            self.cursor += 1;

            self.debug_assert_on_ui_thread(slot_index);

            let mut hooks = self.hooks.borrow_mut();
            if slot_index == hooks.len() {
                let h = HookRef {
                    inner: Rc::new(RefCell::new(initial)),
                };
                let boxed: Box<dyn Any> = Box::new(h.clone());
                hooks.push(HookSlot::State {
                    cell: Rc::new(RefCell::new(boxed)),
                    type_name: std::any::type_name::<HookRef<T>>(),
                });
                return h;
            }

            match &hooks[slot_index] {
                HookSlot::State { cell, type_name } => {
                    let slot = cell.borrow();
                    let h = slot.downcast_ref::<HookRef<T>>().unwrap_or_else(|| {
                        panic!(
                            "hook called in different order: slot {slot_index} was \
                         `{type_name}`, now called as `use_ref::<{requested}>`",
                            requested = std::any::type_name::<T>()
                        )
                    });
                    h.clone()
                }
                _ => panic!(
                    "hook called in different order: slot {slot_index} held a \
                 non-State slot, now called as `use_ref`"
                ),
            }
        }

        /// Read the host's effective [`ColorScheme`] (Light / Dark) for the
        /// current render. Re-rendering on theme switches is driven by the
        /// host via `FrameworkElement::ActualThemeChanged`.
        pub fn use_color_scheme(&self) -> super::theme::ColorScheme {
            super::theme::current_color_scheme()
        }

        pub fn use_memo<T, D, F>(&mut self, deps: D, factory: F) -> T
        where
            T: 'static + Clone,
            D: Deps,
            F: FnOnce() -> T,
        {
            let slot_index = self.cursor;
            self.cursor += 1;

            self.debug_assert_on_ui_thread(slot_index);

            let mut hooks = self.hooks.borrow_mut();
            if slot_index == hooks.len() {
                let value = factory();
                let value_cell: Rc<RefCell<Box<dyn Any>>> =
                    Rc::new(RefCell::new(Box::new(value.clone())));
                hooks.push(HookSlot::Memo {
                    value: value_cell,
                    deps: Some(deps.clone_boxed()),
                });
                return value;
            }

            match &mut hooks[slot_index] {
                HookSlot::Memo {
                    value,
                    deps: stored_deps,
                } => {
                    let deps_equal = match stored_deps {
                        Some(stored) => stored.deps_eq(&deps),
                        None => false,
                    };

                    if deps_equal {
                        let b = value.borrow();
                        b.downcast_ref::<T>()
                            .unwrap_or_else(|| {
                                panic!(
                                    "hook called in different order: slot \
                                 {slot_index} memo is not `{}`",
                                    std::any::type_name::<T>()
                                )
                            })
                            .clone()
                    } else {
                        let new_value = factory();
                        *value.borrow_mut() = Box::new(new_value.clone());
                        *stored_deps = Some(deps.clone_boxed());
                        new_value
                    }
                }
                _ => panic!(
                    "hook called in different order: slot {slot_index} held a \
                 non-Memo slot, now called as `use_memo`"
                ),
            }
        }

        pub fn use_callback<A, D, F>(&mut self, deps: D, f: F) -> Callback<A>
        where
            A: 'static,
            D: Deps,
            F: Fn(A) + 'static,
        {
            self.use_memo(deps, || Callback::new(f))
        }

        pub fn use_effect<D, F>(&mut self, deps: D, f: F)
        where
            D: Deps,
            F: FnOnce() + 'static,
        {
            self.use_effect_with_cleanup(deps, move || {
                f();
                None::<Cleanup>
            });
        }

        pub fn use_effect_with_cleanup<D, F, C>(&mut self, deps: D, f: F)
        where
            D: Deps,
            F: FnOnce() -> Option<C> + 'static,
            C: FnOnce() + 'static,
        {
            let slot_index = self.cursor;
            self.cursor += 1;

            self.debug_assert_on_ui_thread(slot_index);

            let mut hooks = self.hooks.borrow_mut();
            if slot_index == hooks.len() {
                let f_erased: PendingEffect = Box::new(move || f().map(|c| Box::new(c) as Cleanup));
                hooks.push(HookSlot::Effect {
                    deps: Some(deps.clone_boxed()),
                    pending: Some(f_erased),
                    cleanup: None,
                    pending_cleanup: None,
                });
                return;
            }

            match &mut hooks[slot_index] {
                HookSlot::Effect {
                    deps: stored_deps,
                    pending,
                    cleanup,
                    pending_cleanup,
                } => {
                    let deps_equal = match stored_deps {
                        Some(stored) => stored.deps_eq(&deps),
                        None => false,
                    };

                    if !deps_equal {
                        if let Some(prev) = cleanup.take() {
                            *pending_cleanup = Some(prev);
                        }
                        *stored_deps = Some(deps.clone_boxed());

                        let f_erased: PendingEffect =
                            Box::new(move || f().map(|c| Box::new(c) as Cleanup));
                        *pending = Some(f_erased);
                    }
                }
                _ => panic!(
                    "hook called in different order: slot {slot_index} held a \
                 non-Effect slot, now called as `use_effect`"
                ),
            }
        }

        pub fn flush_effects(&mut self) {
            let hooks = Rc::clone(&self.hooks);
            let mut hooks = hooks.borrow_mut();

            for slot in hooks.iter_mut() {
                if let HookSlot::Effect {
                    pending_cleanup, ..
                } = slot
                    && let Some(c) = pending_cleanup.take()
                {
                    c();
                }
            }

            for slot in hooks.iter_mut() {
                if let HookSlot::Effect {
                    pending, cleanup, ..
                } = slot
                    && let Some(effect) = pending.take()
                {
                    let new_cleanup = effect();
                    *cleanup = new_cleanup;
                }
            }
        }

        pub fn run_cleanups(&mut self) {
            let hooks = Rc::clone(&self.hooks);
            let mut hooks = hooks.borrow_mut();
            for slot in hooks.iter_mut() {
                if let HookSlot::Effect {
                    pending_cleanup,
                    cleanup,
                    ..
                } = slot
                {
                    if let Some(c) = pending_cleanup.take() {
                        c();
                    }
                    if let Some(c) = cleanup.take() {
                        c();
                    }
                }
            }
        }

        /// Returns the host window's client-area size in DIPs. Re-renders
        /// the component whenever the window is resized.
        pub fn use_inner_size(&self) -> Size {
            self.read_contexts
                .borrow_mut()
                .insert(inner_size_context_id());
            self.inner_size.get()
        }

        /// Returns the host window's per-monitor DPI. Re-renders the component
        /// whenever the window moves to a monitor with a different scale factor.
        pub fn use_dpi(&self) -> u32 {
            self.read_contexts.borrow_mut().insert(dpi_context_id());
            self.dpi.get()
        }

        pub(crate) fn set_inner_size_cell(&mut self, cell: Rc<Cell<Size>>) {
            self.inner_size = cell;
        }

        pub(crate) fn set_dpi_cell(&mut self, cell: Rc<Cell<u32>>) {
            self.dpi = cell;
        }

        fn state_slot<T>(&mut self, initial: T) -> (T, Rc<RefCell<Box<dyn Any>>>, usize)
        where
            T: 'static + Clone,
        {
            let slot_index = self.cursor;
            self.cursor += 1;

            self.debug_assert_on_ui_thread(slot_index);

            let (cell, stored_type_name) = {
                let mut hooks = self.hooks.borrow_mut();
                if slot_index == hooks.len() {
                    let boxed: Box<dyn Any> = Box::new(initial);
                    let cell = Rc::new(RefCell::new(boxed));
                    let type_name = std::any::type_name::<T>();
                    hooks.push(HookSlot::State {
                        cell: Rc::clone(&cell),
                        type_name,
                    });
                    (cell, type_name)
                } else {
                    match &hooks[slot_index] {
                        HookSlot::State { cell, type_name } => (Rc::clone(cell), *type_name),
                        _ => panic!(
                            "hook called in different order: slot {slot_index} held a \
                         non-State slot, now called as `use_state`/`use_reducer`"
                        ),
                    }
                }
            };

            let current = {
                let slot: Ref<Box<dyn Any>> = cell.borrow();
                match slot.downcast_ref::<T>() {
                    Some(v) => v.clone(),
                    None => panic!(
                        "hook called in different order: slot {slot_index} was \
                     `{stored_type_name}`, now called as `use_state::<{requested}>`",
                        requested = std::any::type_name::<T>()
                    ),
                }
            };

            (current, cell, slot_index)
        }

        fn make_state_setter<T>(
            &self,
            cell: Rc<RefCell<Box<dyn Any>>>,
            slot_index: usize,
        ) -> SetState<T>
        where
            T: 'static + Clone + PartialEq,
        {
            let request = Rc::clone(&self.request_rerender);
            let dirty = Arc::clone(&self.state_dirty);
            let ui_thread = self.ui_thread;
            SetState {
                inner: Rc::new(move |value: T| {
                    if let Some(tid) = ui_thread {
                        debug_assert_eq!(
                            tid,
                            thread::current().id(),
                            "SetState for slot {slot_index} called off the UI thread \
                         (expected {tid:?}, got {actual:?})",
                            actual = thread::current().id()
                        );
                    }
                    let mut slot = cell.borrow_mut();
                    let prev = slot.downcast_ref::<T>().unwrap();
                    if *prev == value {
                        return;
                    }
                    *slot = Box::new(value);
                    drop(slot);
                    dirty.store(true, Ordering::Relaxed);
                    request();
                }),
            }
        }

        fn make_updater<T>(&self, cell: Rc<RefCell<Box<dyn Any>>>, slot_index: usize) -> Updater<T>
        where
            T: 'static + Clone + PartialEq,
        {
            let request = Rc::clone(&self.request_rerender);
            let dirty = Arc::clone(&self.state_dirty);
            let ui_thread = self.ui_thread;
            Updater {
                inner: Rc::new(move |reducer: ReducerClosure<T>| {
                    if let Some(tid) = ui_thread {
                        debug_assert_eq!(
                            tid,
                            thread::current().id(),
                            "Updater for slot {slot_index} called off the UI thread"
                        );
                    }

                    let prev = {
                        let slot = cell.borrow();
                        slot.downcast_ref::<T>().unwrap().clone()
                    };
                    let next = reducer(prev.clone());
                    if prev == next {
                        return;
                    }
                    *cell.borrow_mut() = Box::new(next);
                    dirty.store(true, Ordering::Relaxed);
                    request();
                }),
            }
        }

        fn make_dispatch<S, A, R>(
            &self,
            cell: Rc<RefCell<Box<dyn Any>>>,
            slot_index: usize,
            reducer: R,
        ) -> Dispatch<A>
        where
            S: 'static + Clone + PartialEq,
            A: 'static,
            R: Fn(S, A) -> S + 'static,
        {
            let request = Rc::clone(&self.request_rerender);
            let dirty = Arc::clone(&self.state_dirty);
            let ui_thread = self.ui_thread;
            let reducer = Rc::new(reducer);
            Dispatch {
                inner: Rc::new(move |action: A| {
                    if let Some(tid) = ui_thread {
                        debug_assert_eq!(
                            tid,
                            thread::current().id(),
                            "Dispatch for slot {slot_index} called off the UI thread"
                        );
                    }
                    let prev = {
                        let slot = cell.borrow();
                        slot.downcast_ref::<S>().unwrap().clone()
                    };
                    let next = reducer(prev.clone(), action);
                    if prev == next {
                        return;
                    }
                    *cell.borrow_mut() = Box::new(next);
                    dirty.store(true, Ordering::Relaxed);
                    request();
                }),
            }
        }

        fn debug_assert_on_ui_thread(&self, slot_index: usize) {
            if let Some(tid) = self.ui_thread {
                debug_assert_eq!(
                    tid,
                    thread::current().id(),
                    "hook at slot {slot_index} called off the UI thread"
                );
            }
        }
    }
}

/// Stateless render unit parameterised by props `P`. Implementors return
/// an [`Element`] tree from `render`; the host re-invokes `render` when
/// `should_update` returns `true` for the latest props.
pub trait Component<P = ()>: 'static
where
    P: Clone + PartialEq + 'static,
{
    fn render(&self, props: &P, cx: &mut RenderCx) -> Element;

    fn should_update(&self, old_props: &P, new_props: &P) -> bool {
        old_props != new_props
    }

    fn has_on_appeared(&self) -> bool {
        false
    }

    fn has_on_disappeared(&self) -> bool {
        false
    }

    fn on_appeared(&self, _props: &P, _cx: &mut RenderCx) {}

    fn on_disappeared(&self, _props: &P, _cx: &mut RenderCx) {}
}

/// Blanket impl: any `Fn(&P, &mut RenderCx) -> Element` is a [`Component<P>`].
///
/// ```ignore
/// fn greeting(props: &GreetingProps, _cx: &mut RenderCx) -> Element {
///     text_block(format!("Hello, {}!", props.name)).into()
/// }
/// ```
///
/// For unit-props, use `fn(_: &(), cx: &mut RenderCx) -> Element`.
impl<F, P> Component<P> for F
where
    F: Fn(&P, &mut RenderCx) -> Element + 'static,
    P: Clone + PartialEq + 'static,
{
    fn render(&self, props: &P, cx: &mut RenderCx) -> Element {
        (self)(props, cx)
    }
}
