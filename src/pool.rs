use crossbeam::atomic::AtomicCell;
pub trait Recycle: Default {
    fn get() {}

    fn reset() {}
}

pub struct Pool<T> {
    value: Vec<AtomicCell<T>>,
}

mod test {
    #[test]
    fn test() {}
}
