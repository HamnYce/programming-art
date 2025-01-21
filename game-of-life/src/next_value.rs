/// used to determine the next value based on neighboring values
pub trait NextValue<T> {
    fn next_value(&self, neighbors: T) -> T;
}
