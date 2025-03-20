use bon::Builder;


#[derive(Debug)]
enum PathSegment<'a> {
	Normal(&'a str),
	RegParam(&'a str)
}

impl std::fmt::Display for PathSegment<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Normal(s) => write!(f, "{s}"),
			Self::RegParam(p) => write!(f, "{{{p}}}"),
		}
	}
}

#[derive(Debug)]
pub struct Path<'a> {
	segments: Vec<PathSegment<'a>>
}

impl<'a> Path<'a> {
	pub const fn new() -> Self {
		Self { segments: Vec::new() }
	}
	pub fn with_capacity(cap: usize) -> Self {
		Self { segments: Vec::with_capacity(cap) }
	}

	pub fn normal(&mut self, s: &'a str) -> &mut Self {
		self.segments.push(PathSegment::Normal(s));
		self
	}

	pub fn regular_param(&mut self, name: &'a str) -> &mut Self {
		self.segments.push(PathSegment::RegParam(name));
		self
	}

	pub fn is_empty(&self) -> bool {
		self.segments.is_empty()
	}
}

impl std::fmt::Display for Path<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.is_empty() {
			return write!(f, "/");
		}
		for s in &self.segments {
			write!(f, "/{s}")?;
		}
		Ok(())
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
			$(path_inner!(x, $s);)+
			x
		}
		
	};
}

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