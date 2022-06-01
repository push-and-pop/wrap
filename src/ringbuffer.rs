use std::{error::Error, ops::Deref};
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node<'a> {
    buf: Vec<u8>,
    next: Option<&'a mut Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    #[inline]
    pub fn len(&self) -> usize {
        return self.buf.len();
    }
}

struct Buffer<'a> {
    bs: Vec<Vec<u8>>,
    head: Option<Box<Node<'a>>>,
    tail: Option<&'a mut Box<Node<'a>>>,
    size: isize,
    bytes: isize,
}

impl<'a> Buffer<'a> {
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
                //if let Some(tail) = b {};
                self.tail = b.as_deref_mut().unwrap().next;
                if self.tail == None {}
                return b;
            }
        }
    }
}
