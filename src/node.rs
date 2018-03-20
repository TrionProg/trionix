pub type NodeID=u32;//TODO u16?

static mut NODE_ID:NodeID=0;

pub fn set_node_id(node_id:NodeID) {
    unsafe{
        NODE_ID=node_id;
    }
}

pub fn get_node_id() -> NodeID {
    unsafe{
        NODE_ID
    }
}