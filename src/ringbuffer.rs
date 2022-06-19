use std::{ptr, vec::Vec};
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
    // func (llb *Buffer) Read(p []byte) (n int, err error) {
    // 	if len(p) == 0 {
    // 		return 0, nil
    // 	}

    // 	for b := llb.pop(); b != nil; b = llb.pop() {
    // 		m := copy(p[n:], b.buf)
    // 		n += m
    // 		if m < b.len() {
    // 			b.buf = b.buf[m:]
    // 			llb.pushFront(b)
    // 		} else {
    // 			bsPool.Put(b.buf)
    // 		}
    // 		if n == len(p) {
    // 			return
    // 		}
    // 	}
    // 	return
    // }

    /// Read reads data from the Buffer.
    pub fn read(&mut self, p: &mut Vec<u8>) -> Result<isize, &'static str> {
        if p.len() == 0 {
            return Ok(0);
        }
        let mut n = 0;
        unsafe {
            loop {
                let mut b = self.pop();
                if ptr::null_mut() == b {
                    break;
                }
                let m = (*b).buf.len();
                p.extend((*b).buf.iter());
                n += m;
                if m < (*b).len() {
                    (*b).buf.drain(m..);
                } else {
                }
                let (left, right) = p.split_at_mut(n);
            }
        }

        return Ok(n as isize);
    }

    /// pop returns and removes the head of l. If l is empty, it returns nil.
    pub fn pop(&mut self) -> *mut Node {
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
