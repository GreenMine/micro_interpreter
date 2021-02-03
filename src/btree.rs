
type ChildNode<T> = Option<Box<BNode<T>>>;

#[derive(Debug)]
pub struct BNode<T> {
    pub left: ChildNode<T>,
    pub right: ChildNode<T>,
    pub value: T
}


impl<T> BNode<T> {
    pub fn new(left: Option<BNode<T>>, right: Option<BNode<T>>, value: T)  -> Self {
        Self {
            left: if let Some(node) = left   { Some(Box::new(node)) } else { None },
            right: if let Some(node) = right { Some(Box::new(node)) } else { None },
            value
        }
    }
}