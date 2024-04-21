pub trait Comparator<T> {
    fn compare(&self, a: &T, b: &T) -> bool;
}
