// rc_weak.rs

use std::rc::Rc; 
use std::rc::Weak; 

#[derive(Debug)] 
struct LinkedList<T> { 
    head: Option<Rc<LinkedListNode<T>>> 
} 

#[derive(Debug)] 
struct LinkedListNode<T> { 
    next: Option<Rc<LinkedListNode<T>>>, 
    prev: Option<Weak<LinkedListNode<T>>>, 
    data: T 
} 

impl<T> LinkedList<T> { 
    fn new() -> Self { 
        LinkedList { head: None } 
    } 

    fn append(&mut self, data: T) -> Self { 
        let new_node = Rc::new(LinkedListNode { 
            data: data, 
            next: self.head.clone(), 
            prev: None 
        }); 

        match self.head.clone() { 
            Some(node) => { 
                node.prev = Some(Rc::downgrade(&new_node)); 
            }, 
            None => { 
            } 
        } 

        LinkedList { 
            head: Some(new_node) 
        } 
    } 
} 

fn main() { 
    let list_of_nums = LinkedList::new().append(1).append(2).append(3); 
    println!("nums: {:?}", list_of_nums); 
}
