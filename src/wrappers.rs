#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq, Eq, Hash)]
pub struct Node{
   id : u64
}

impl Node{

    pub fn new( id: u64 )-> Self{
        Node{ id : id }
    }
    pub fn get_id(&self)-> u64{
        self.id
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq, Eq, Hash)]
pub struct Edge {
    u: Node,
    v: Node
}
impl Edge{

    pub fn new( u: Node, v: Node)-> Edge{
        Edge{ u:u, v:v }
    }
    pub fn get_u(&self)-> &Node{
        &self.u
    }
    pub fn get_v(&self)-> &Node{
        &self.v
    }
}
