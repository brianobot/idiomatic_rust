#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};


fn main() {
    let bark = || println!("Bark!"); // <- println here is a side effect because it's an IO task, this is not a pure function
    bark();
    
    let increment = |value| value + 1;
    println!("Next Value: {}", increment(10));
    
    let print_and_increment = |value| {
        print!("Value to be incremented: {value} ");
        value + 1
    };
    
    println!("{}", print_and_increment(10));
    
    let left  = || 1;
    let right = || 2;
    
    let adder = |left_fn: fn() -> i32, right_fn: fn() -> i32| {
        println!("{} + {} = {}", left_fn(), right_fn(), left_fn() + right_fn())
    };
    
    adder(left, right);
    
    let name = String::from("Brian");
    let greet = || name;
    
    greet();
    // greet(); this would result in an error, because the closure is an FnOnce because it consumes the variable it captures from the environment
    // 
    let age = 10;
    let capture = move || age; // move is actually optional since rust can infer if a move must be made or not
    
    // this is not an issue beucase the capture type implements Copy and so it just copied and move moved
    capture();
    capture();
    capture();
    capture();
    capture();
    
    let dinosaur = LinkedList::new("T-Rex");
    let last_dino = dinosaur.last();
    
    println!("Last Dino: {last_dino:?}");
    
    let numbers = [1, 2, 3, 4, 5, 6];
    let values = numbers.iter().map(i32::to_string).collect::<Vec<_>>();
    println!("Values = {values:?}");
    
}


type ItemData<T> = Rc<RefCell<T>>;
type ListItemPtr<T> = Rc<RefCell<ListItem<T>>>;

#[derive(Debug)]
struct ListItem<T> {
    data: ItemData<T>,
    next: Option<ListItemPtr<T>>
}

impl<T> ListItem<T> {
    fn new(t: T) -> Self {
        Self {
            data: Rc::new(RefCell::new(t)),
            next: None
        }
    }
}


#[derive(Debug)]
struct LinkedList<T> {
    head: ListItemPtr<T>,
    cur_item: Option<ListItemPtr<T>>
}

impl<T> LinkedList<T> {
    fn new(t: T) -> Self {
        Self {
            head: Rc::new(RefCell::new(ListItem::new(t))),
            cur_item: None
        }
    }
    
    fn append(&mut self, t: T) {
        let mut next = self.head.clone();
        while next.as_ref().borrow().next.is_some() {
            let n = next
                .as_ref()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .clone();
            
            next = n
        }
        next.as_ref().borrow_mut().next = Some(Rc::new(RefCell::new(ListItem::new(t))));
    }
    
    fn iter(&self) -> Iter<T> {
        Iter {
            next: Some(self.head.clone())
        }
    }
    
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: Some(self.head.clone())
        }
    }
    
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { next: Some(self.head.clone()) }
    }
} 

impl<T> Iterator for LinkedList<T> {
    type Item = ListItemPtr<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.cur_item.clone() {
            None => self.cur_item = Some(self.head.clone()),
            Some(ptr) => self.cur_item = ptr.borrow().next.clone(),
        }
        
        self.cur_item.clone()
    }
}


struct Iter<T> {
    next: Option<ListItemPtr<T>>,
}


impl<T> Iterator for Iter<T> {
    type Item = ItemData<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            Some(ptr) => {
                self.next.clone_from(&ptr.as_ref().borrow().next);
                Some(ptr.as_ref().borrow().data.clone())
            },
            None => None,
        }
    }
} 

struct IterMut<T> {
    next: Option<ListItemPtr<T>>,
}


impl<T> Iterator for IterMut<T> {
    type Item = ItemData<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            Some(ptr) => {
                self.next.clone_from(&ptr.as_ref().borrow().next);
                Some(ptr.as_ref().borrow().data.clone())
            },
            None => None,
        }
    }
} 

struct IntoIter<T> {
    next: Option<ListItemPtr<T>>,
}


impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            Some(ptr) => {
                self.next = ptr.as_ref().borrow().next.clone();
                let list_item = Rc::try_unwrap(ptr).map(|ref_cell| ref_cell.into_inner());
                match list_item {
                    Ok(list_item) => Rc::try_unwrap(list_item.data).map(|ref_cell| ref_cell.into_inner()).ok(),
                    Err(_) => None,
                }
            },
            None => None,
        }
    }
} 