use std::cmp;

use rand::seq::index;

type Node = Option<u64>;
pub struct DynamicArray {
    buf: Box<[Node]>,
    cap: usize,
    pub length: usize,
}

impl DynamicArray {
    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        let mut new_cap = old_cap + (old_cap >> 1);

        new_cap = cmp::max(new_cap, min_cap);
        new_cap = cmp::min(new_cap, usize::max_value());
        let current = self.buf.clone();
        self.cap = new_cap;

        self.buf = vec![None; new_cap].into_boxed_slice();
        self.buf[..current.len()].clone_from_slice(&current);
    }

    pub fn at(&mut self, index: usize) -> Option<u64> {
        if self.length > index {
            self.buf[index]
        } else {
            None
        }
    }
}

pub struct ListIterator {
    current: usize,
    data: Box<[Node]>,
}
impl Iterator for ListIterator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            self.current += 1;
            item
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            if self.current == 0 {
                self.current = self.data.len() - 1;
            } else {
                self.current -= 1;
            }
            item
        } else {
            None
        }
    }
}
