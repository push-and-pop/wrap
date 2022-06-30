use crate::stack::Stack;
use std::ptr;
pub trait Recycle: Default {
    fn reset(&mut self) {}
}

#[derive(Debug)]
pub struct ObjectPool<T> {
    value: *mut Stack<T>,
}

impl<T: Recycle> ObjectPool<T> {
    pub fn new() -> Self {
        let ref mut stack = Stack::new();
        Self {
            value: stack as *mut Stack<T>,
        }
    }

    pub fn get(&self) -> T {
        unsafe {
            match (*self.value).pop() {
                Some(t) => t,
                None => T::default(),
            }
        }
    }

    pub fn put(&self, mut t: T) {
        t.reset();
        unsafe {
            (*self.value).push(t);
        }
    }
}
impl<T: Default> Recycle for Box<T> {}

mod test {
    #[test]
    fn test() {
        let a: i32 = i32::default();
    }
}
