use core::mem::ManuallyDrop;
use core::ptr;
use core::sync::atomic::Ordering;
use crossbeam_epoch::{Atomic, Owned};

#[derive(Debug)]
pub struct Stack<T> {
    top: Atomic<Node<T>>,
}

#[derive(Debug)]
pub struct Node<T> {
    value: ManuallyDrop<T>,
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
            value: ManuallyDrop::new(t),
            next: Atomic::null(),
        });

        let guard = crossbeam_epoch::pin();

        loop {
            let top = self.top.load(Ordering::SeqCst, &guard);
            node.next.store(top, Ordering::SeqCst);

            match self
                .top
                .compare_exchange(top, node, Ordering::SeqCst, Ordering::SeqCst, &guard)
            {
                Ok(_) => break,
                Err(e) => node = e.new,
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        let guard = crossbeam_epoch::pin();
        loop {
            let head = self.top.load(Ordering::SeqCst, &guard);

            match unsafe { head.as_ref() } {
                Some(h) => {
                    let next = h.next.load(Ordering::SeqCst, &guard);

                    if self
                        .top
                        .compare_exchange(head, next, Ordering::SeqCst, Ordering::SeqCst, &guard)
                        .is_ok()
                    {
                        unsafe {
                            guard.defer_destroy(head);
                            return Some(ManuallyDrop::into_inner(ptr::read(&(*h).value)));
                        }
                    }
                }
                None => return None,
            }
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

mod test {
    use super::*;
    use crossbeam_utils::thread::scope;
    use std::thread;
    #[test]
    fn test() {
        let stack = Stack::new();
        for i in 0..100_000 {
            stack.push(i);
        }
        scope(|scope| {
            for i in 0..10 {
                scope.spawn(|_| {
                    for i in 0..10000 {
                        let a: i32 = stack.pop().unwrap().into();
                        println!("{:#?}", a);
                    }
                });
            }
        })
        .unwrap();
        stack.push(100);
        stack.push(101);
        assert_eq!(stack.pop().unwrap(), 101);
        assert_eq!(stack.pop().unwrap(), 100);
    }
}
