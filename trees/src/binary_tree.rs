
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

pub struct BinarySearchTree {
    root: Tree,
    pub length: u64,
}