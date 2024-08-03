use std::{cell:: RefCell, net, rc::Rc};
use rand::prelude::*;
type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}

impl Node {
    fn new(next: Vec<Link>, offset: u64, command: String)->Rc<RefCell<Node>>{
        Rc::new(RefCell::new(Node{next, offset, command}))
    }
}

#[derive(Clone)]
pub struct BestTransactionLog {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

impl BestTransactionLog {
    fn append(&mut self, offset: u64, value: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };
        let new = Node::new(vec![None; level], offset, value);
    }
    fn get_level(&self)->usize{
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n +=1;
        }
        n
    }

    pub fn find(&self, offset: u64)->Option<String>{
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level;
                let node = head.clone();
                let mut result = None;
                loop{
                    if node.borrow().next[start_level].is_some(){
                        break;
                    }
                    start_level -=1;
                }
                let mut n = node;
                for level in (0..=start_level).rev(){
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next) if next.borrow().offset <= offset => n = next.clone(),
                            _ => break
                        };
                    }
                    if n.borrow().offset == offset{
                        let tmp = n.borrow();
                        result = Some(tmp.command.clone());
                        break;
                    }
                }
                result
            }
            None => None
        }
    }
}
