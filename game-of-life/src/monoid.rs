pub trait Monoid<T> {
    fn append(self, other: T) -> T;
}
