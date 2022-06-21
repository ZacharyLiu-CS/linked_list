mod linked_list {
    use std::{cell::RefCell, rc::Rc};

    //RefCell can ensure the address of data is not change, but we can modify the data inside
    type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

    struct Node<T>
    where
        T: std::fmt::Debug + Copy,
    {
        data: T,
        next: NodePtr<T>,
        prev: NodePtr<T>,
    }
    impl<T> Node<T>
    where
        T: std::fmt::Debug + Copy,
    {
        fn new(data: T) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Node {
                data: data,
                next: None,
                prev: None,
            }))
        }
    }

    impl<T> std::fmt::Debug for Node<T>
    where
        T: std::fmt::Debug + Copy,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "<{:?}>", self.data)
        }
    }
    impl<T> Drop for Node<T>
    where
        T: std::fmt::Debug + Copy,
    {
        fn drop(&mut self) {
            println!("Drop Node : {:?}", self);
        }
    }
    pub struct LinkedList<T>
    where
        T: std::fmt::Debug + Copy,
    {
        head: NodePtr<T>,
        tail: NodePtr<T>,
    }

    impl<T> std::fmt::Debug for LinkedList<T>
    where
        T: std::fmt::Debug + Copy,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut result: Vec<T> = vec![];
            let mut iter = self.head.clone();
            while iter.is_some() {
                let node = match iter {
                    Some(ref n) => Rc::clone(n),
                    None => break,
                };
                result.push(node.as_ref().borrow().data);
                iter = node.borrow().next.clone();
            }

            write!(f, "List: {:?}", result)
        }
    }

    impl<T> LinkedList<T>
    where
        T: std::fmt::Debug + Copy,
    {
        pub fn new() -> Self {
            LinkedList {
                head: None,
                tail: None,
            }
        }

        pub fn new_with_data(data: T) -> Self {
            let first_node = Node::new(data);

            LinkedList {
                head: Some(Rc::clone(&first_node)),
                tail: Some(Rc::clone(&first_node)),
            }
        }

        pub fn push_back(&mut self, data: T) {
            let new_node = Node::new(data);
            match self.tail.take() {
                Some(old_tail) => {
                    old_tail.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = Some(old_tail);
                    self.tail = Some(new_node);
                }
                None => {
                    self.tail = Some(new_node.clone());
                    self.head = Some(new_node);
                }
            }
        }

        pub fn push_front(&mut self, data: T) {
            let new_node = Node::new(data);
            match self.head.take() {
                Some(old_head) => {
                    old_head.borrow_mut().prev = Some(new_node.clone());
                    new_node.borrow_mut().next = Some(old_head);
                    self.head = Some(new_node);
                }
                None => {
                    self.tail = Some(new_node.clone());
                    self.head = Some(new_node);
                }
            }
        }

        pub fn pop_back(&mut self) -> Option<T> {
            self.tail.take().map(|old_tail| {
                match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                    }
                    None => {
                        self.head.take();
                    }
                }
                Rc::try_unwrap(old_tail).ok().unwrap().into_inner().data
            })
        }

        pub fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|old_head| {
                match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take();
                        self.head = Some(new_head);
                    }
                    None => {
                        self.tail.take();
                    }
                }
                Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
            })
        }
    }
}

use linked_list::LinkedList;
fn main() {
    let mut list = LinkedList::new_with_data(1);
    println!("After initialization with data 1 : {:?}", list);

    list.push_back(33);
    println!("After push back an element 33 : {:?}", list);

    list.push_front(22);
    println!("After push front an element 22 : {:?}", list);

    list.push_back(44);
    println!("After push back an element 44 : {:?}", list);

    list.pop_back();
    println!("After pop back an element : {:?}", list);


    let mut list = LinkedList::new();
    println!("After initialization : {:?}", list);

    list.push_back(33);
    println!("After push back an element 33 : {:?}", list);

    list.push_front(22);
    println!("After push front an element 22 : {:?}", list);

    list.push_back(44);
    println!("After push back an element 44 : {:?}", list);

    list.pop_front();
    println!("After pop front an element : {:?}", list);
}
