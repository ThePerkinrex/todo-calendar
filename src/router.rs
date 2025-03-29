use std::{borrow::Cow, collections::HashMap, convert::Infallible};

use axum::{
    extract::Request,
    handler::Handler,
    response::IntoResponse,
    routing::{MethodRouter, Route},
};
use path::Path;
use tower::{Layer, Service};

pub mod path;

macro_rules! reimpl_router {
	(
        // Optional attributes, like #[doc(...)]
        $(#[$attr:meta])*
        // Visibility and function keyword.
        $vis:vis fn $name:ident $(<$($gen:tt),*>)?
        // Parameters: the first must be `self` and then zero or more other tokens.
        ( self $(+ {name: $path_name_var:ident, path: $path_var:ident})? $(, $($arg_name:ident : $arg_type:ty),*)? )
        // Return type must be `Self`
        -> Self
        // Optional where clause (one or more tokens)
        $(where $($where_clause:tt)+)?
    ) => {
        $(#[$attr])*
        $vis fn $name <IntoCow: Into<Cow<'a, str>>, $($($gen),*)?> ( self $(, $path_name_var: IntoCow, $path_var: Path<'a>)? $(, $($arg_name : $arg_type),*)? ) -> Self
            $(where $($where_clause)+)?
		{
			#[allow(unused_mut)]
			let Self { inner, mut paths } = self;
			let inner = inner.$name($(& $path_var.to_string(),)? $($($arg_name),*)?);
			$(paths.insert($path_name_var.into(), $path_var);)?
			Self { inner, paths }
		}
    };
}

pub struct Router<'a, S = ()> {
    inner: axum::Router<S>,
    paths: HashMap<Cow<'a, str>, Path<'a>>,
}

impl<'a, S> Router<'a, S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            inner: axum::Router::new(),
            paths: HashMap::new(),
        }
    }

    reimpl_router! {
        pub fn route(self + {name: name, path: path}, method_router: MethodRouter<S>) -> Self
    }
    // pub fn route(self, name: &'a str, path: Path<'a>, method_router: MethodRouter<S>) -> Self {
    //     let Self { inner, mut paths } = self;
    //     let inner = inner.route(&path.to_string(), method_router);
    //     paths.insert(name, path);
    //     Self { inner, paths }
    // }

    reimpl_router! {
        pub fn route_service<T>(self + {name: name, path: path}, service: T) -> Self
        where
            T: Service<Request, Error = Infallible> + Clone + Send + Sync + 'static,
            T::Response: IntoResponse,
            T::Future: Send + 'static,
    }

    pub fn nest<I: Into<Cow<'a, str>>>(self, name: I, path: Path<'a>, router: Self) -> Self {
        let Self { inner, mut paths } = self;
        let inner = inner.nest(&path.to_string(), router.inner);
        let name = name.into();
        paths.extend(
            router
                .paths
                .into_iter()
                .map(|(n, p)| (Cow::Owned(format!("{name} {n}")), path.clone().join(p))),
        );
        Self { inner, paths }
    }

    reimpl_router! {
        pub fn nest_service<T>(self + {name: name, path: path}, service: T) -> Self
        where
            T: Service<Request, Error = Infallible> + Clone + Send + Sync + 'static,
            T::Response: IntoResponse,
            T::Future: Send + 'static,
    }

    pub fn merge<R>(self, other: R) -> Self
    where
        R: Into<Self>,
    {
        let Self { inner, mut paths } = self;
        let other: Self = other.into();
        let inner = inner.merge(other.inner);
        paths.extend(other.paths);
        Self { inner, paths }
    }

    pub fn fallback<H, T>(mut self, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.inner = self.inner.fallback(handler);
        self
    }

    pub fn fallback_service<T>(mut self, service: T) -> Self
    where
        T: Service<Request, Error = Infallible> + Clone + Send + Sync + 'static,
        T::Response: IntoResponse,
        T::Future: Send + 'static,
    {
        self.inner = self.inner.fallback_service(service);
        self
    }

    pub fn with_state<S2>(self, state: S) -> Router<'a, S2> {
        Router {
            inner: self.inner.with_state(state),
            paths: self.paths,
        }
    }

    pub fn layer<L>(mut self, layer: L) -> Self
    where
        L: Layer<Route> + Clone + Send + Sync + 'static,
        L::Service: Service<Request> + Clone + Send + Sync + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
    {
        self.inner = self.inner.layer(layer);
        self
    }

    pub fn visit<V: PathVisitor<'a>>(&self, visitor: &mut V) {
        let mut sorted_paths = self.paths.iter().collect::<Vec<_>>();
        sorted_paths.sort_by_key(|(name, _)| *name);
        for (k, v) in sorted_paths {
            visitor.visit(k.as_ref(), v);
        }
    }
}

impl<S> Default for Router<'_, S>
where
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, S> From<Router<'a, S>> for axum::Router<S> {
    fn from(value: Router<'a, S>) -> Self {
        value.inner
    }
}

pub trait PathVisitor<'a> {
    fn visit(&mut self, name: &str, path: &Path<'a>);
}
