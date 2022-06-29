#[macro_use]
mod utils;
mod error;
mod pool;
mod queue;
mod ringbuffer;
mod stack;

pub use queue::Queue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
