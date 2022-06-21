use core::mem::ManuallyDrop;
use core::sync::atomic::Ordering;
use crossbeam_epoch::{Atomic, Owned};
pub struct Stack<T> {
    top: Atomic<Node<T>>,
}

pub struct Node<T> {
    value: T,
    next: Atomic<Node<T>>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            top: Atomic::null(),
        }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack::default()
    }

    pub fn push(&self, t: T) {
        let mut node = Owned::new(Node {
            value: t,
            next: Atomic::null(),
        });

        let guard = crossbeam_epoch::pin();

        loop {
            let top = self.top.load(Ordering::Relaxed, &guard);
            node.next.store(top, Ordering::Relaxed);

            match self
                .top
                .compare_exchange(top, node, Ordering::Release, Ordering::Relaxed, &guard)
            {
                Ok(_) => break,
                Err(e) => node = e.new,
            }
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn test() {}
}
