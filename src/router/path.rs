use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum PathSegment<'a> {
    Normal(Cow<'a, str>),
    RegParam(Cow<'a, str>),
}

impl std::fmt::Display for PathSegment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal(s) => write!(f, "{s}"),
            Self::RegParam(p) => write!(f, "{{{p}}}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Path<'a> {
    segments: Vec<PathSegment<'a>>,
}

impl<'a> Path<'a> {
    pub const fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            segments: Vec::with_capacity(cap),
        }
    }

    pub fn normal<T: Into<Cow<'a, str>>>(&mut self, s: T) -> &mut Self {
        self.segments.push(PathSegment::Normal(s.into()));
        self
    }

    pub fn regular_param<T: Into<Cow<'a, str>>>(&mut self, name: T) -> &mut Self {
        self.segments.push(PathSegment::RegParam(name.into()));
        self
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn join(mut self, other: Self) -> Self {
        self.segments.extend_from_slice(&other.segments);
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = &PathSegment<'a>> {
        self.segments.iter()
    }
}

impl Default for Path<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // if self.is_empty() {
        // 	return write!(f, "/");
        // }
        for s in &self.segments {
            write!(f, "/{s}")?;
        }
        write!(f, "/")
    }
}

#[macro_export]
macro_rules! path {
	( / ) => {
		$crate::router::path::Path::new()
	};
	( $( / $s:tt )+) => {
		{
			let mut x = $crate::router::path::Path::new();
			$($crate::path_inner!(x, $s);)+
			x
		}

	};
}

#[macro_export]
macro_rules! path_inner {
    ($x:ident, $s:ident) => {
        $x.normal(stringify!($s))
    };
    ($x:ident, { $s:ident } ) => {
        $x.regular_param(stringify!($s))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn use_macro() {
        assert_eq!(path!(/).to_string(), "/");
        assert_eq!(path!(/ hello ).to_string(), "/hello");
        assert_eq!(path!(/ {yes} ).to_string(), "/{yes}");
        assert_eq!(path!(/ hello / {yes} ).to_string(), "/hello/{yes}");
    }
}
