/// used to initialise a random number at the start of life
pub trait RandomInit<T> {
    fn random() -> T;
}
