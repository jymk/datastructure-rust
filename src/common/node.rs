//数据结构的节点
pub trait BoxNode {
    type T;
    type U: BoxNode;

    fn get_node(&self, t: &Option<Self::T>) -> Option<&Self::U>;
}

//数据结构本体
pub trait BoxEntity {
    type T;
    type U: BoxNode;

    fn get_node(&self, t: &Self::T) -> Option<&Self::U>;
}
