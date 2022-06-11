use std::ptr;
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    buf: Vec<u8>,
    next: *mut Node,
}

impl Node {
    #[inline]
    pub fn len(&self) -> usize {
        return self.buf.len();
    }
}

struct Buffer {
    bs: Vec<Vec<u8>>,
    head: *mut Node,
    tail: *mut Node,
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
    pub fn pop(mut self) -> *mut Node {
        if ptr::null_mut() == self.head {
            return ptr::null_mut();
        }
        let mut b = self.head;
        unsafe {
            self.head = (*b).next;
            if ptr::null_mut() == self.head {
                self.tail = ptr::null_mut();
            }
            (*b).next = ptr::null_mut();
            self.size -= 1;
            self.bytes -= (*b).len() as isize;
        }
        return b;
    }
}
// func (llb *Buffer) pop() *node {
// 	if llb.head == nil {
// 		return nil
// 	}
// 	b := llb.head
// 	llb.head = b.next
// 	if llb.head == nil {
// 		llb.tail = nil
// 	}
// 	b.next = nil
// 	llb.size--
// 	llb.bytes -= b.len()
// 	return b
// }
