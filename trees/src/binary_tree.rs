use std::mem;


#[derive(Clone, Debug)]
pub struct IoTDevice{
    pub numerical_id: u64,
    pub addres: String,
}

type Tree = Option<Box<Node>>;

struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree,
}
impl Node {
    fn new(dev: IoTDevice) -> Option<Box<Node>>{
        Some(Box::new(Node{dev, left: None, right: None}))
    }
}
pub struct BinarySearchTree {
    root: Tree,
    pub length: u64,
}

impl BinarySearchTree {
    pub fn add(&mut self, device: IoTDevice){
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }

    fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                if n.dev.numerical_id <= device.numerical_id {
                    n.left = self.add_rec(n.left, device);
                    Some(n)
                } else {
                    n.right = self.add_rec(n.right, device);
                    Some(n)
                }
            }
            _ => Node::new(device),
        }
    }

    pub fn find(&mut self, search_id: u64) -> Option<IoTDevice>{
        self.find_rec(&self.root, search_id)
    }

    fn find_rec(&self, node: &Tree, search_id: u64)-> Option<IoTDevice>{
        match node {
            Some(n)=>{
                if n.dev.numerical_id == search_id {
                    Some(n.dev.clone())
                } else if n.dev.numerical_id < search_id {
                    self.find_rec(&n.left, search_id)
                } else {
                    self.find_rec(&n.right, search_id)
                }
            }
            _ => None, 
        }
    }

    pub fn walk(&self, callback: impl Fn(&IoTDevice)->()){
        self.walk_in_order(&self.root, &callback);
    }

    fn walk_in_order(&self, node: &Tree, callback: impl Fn(&IoTDevice)->()){
        if let Some(n) = node  {
            self.walk_in_order(&n.left, &callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, &callback);
        }
    }


}