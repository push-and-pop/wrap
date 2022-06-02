use std::{borrow::BorrowMut, error::Error, ops::Deref};
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    buf: Vec<u8>,
    next: Option<Box<Node>>,
}

impl Node {
    #[inline]
    pub fn len(&self) -> usize {
        return self.buf.len();
    }
}

struct Buffer {
    bs: Vec<Vec<u8>>,
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
    size: isize,
    bytes: isize,
}

impl Buffer {
    // Read reads data from the Buffer.
    pub fn read(&self, p: Vec<u8>) -> Result<isize, &'static str> {
        if p.len() == 0 {
            return Ok(0);
        }
        return Ok(0);
    }

    // pop returns and removes the head of l. If l is empty, it returns nil.
    pub fn pop(mut self) -> Option<Box<Node>> {
        match self.head {
            None => None,
            Some(_) => {
                let mut b = self.head;
                //self.tail.insert(value)
                //if let Some(tail) = b {};
                self.tail.replace(b?.as_mut().next.take()?);
                if self.tail == None {}
                return b;
            }
        }
    }
}
