use std::{cell::RefCell, rc::Rc};

type SingleLink = Option<Rc<RefCell<Node>>>;
#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

pub struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub lenght: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            lenght: 0,
        }
    }
    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.lenght += 1;
        self.tail = Some(new);
    }
    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.lenght -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("paso algo raro")
                .into_inner()
                .value
        })
    }
}
