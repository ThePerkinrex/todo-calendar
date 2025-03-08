pub struct SqlVar<T>(pub T)
where
    T: std::fmt::Display;

impl<T> std::fmt::Display for SqlVar<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}

pub struct SqlEquals<A, B>(pub A, pub B)
where
    A: std::fmt::Display,
    B: std::fmt::Display;

impl<A, B> std::fmt::Display for SqlEquals<A, B>
where
    A: std::fmt::Display,
    B: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.0, self.1)
    }
}

pub struct SqlAndJoin<F, I, T>(pub F)
where
    F: Fn() -> I,
    I: Iterator<Item = T>,
    T: std::fmt::Display;

impl<F, I, T> std::fmt::Display for SqlAndJoin<F, I, T>
where
    F: Fn() -> I,
    I: Iterator<Item = T>,
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_first = true;
        for item in self.0() {
            if is_first {
                is_first = !is_first;
            } else {
                write!(f, "AND ")?;
            }
            write!(f, "{} ", item)?;
        }
        Ok(())
    }
}
