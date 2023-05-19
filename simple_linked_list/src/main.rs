#[derive(Debug)]
struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList(None)
    }

    fn is_tail(&self) -> bool {
        match &self.0 {
            None => false, // Empty list has no element so it has no tail
            Some((_, child)) => match child.0 {
                None => true,
                _ => false,
            },
        }
    }

    fn len(&self) -> u32 {
        match &self.0 {
            None => 0,
            Some((_, child)) => 1 + child.len(),
        }
    }

    fn add_head(&mut self, data: T) {
        let old_head = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(old_head))));
    }

    fn add_tail(&mut self, data: T) {
        match self.0 {
            None => self.add_head(data),
            Some((_, ref mut next)) => {
                next.add_tail(data);
            }
        }
    }

    fn pop_head(&mut self) -> Option<T> {
        let old_head = self.0.take();
        match old_head {
            None => None,
            Some((value, next)) => {
                self.0 = next.0;
                Some(value)
            }
        }
    }

    fn pop_tail(&mut self) -> Option<T> {
        if self.is_tail() {
            let (val, _) = self.0.take().unwrap();
            return Some(val);
        } else {
            match &mut self.0 {
                None => None,
                Some((_, child)) => child.pop_tail(),
            }
        }
    }
}

impl<T: std::fmt::Display> LinkedList<T> {
    fn stringify(&self) -> String {
        match &self.0 {
            None => "/".to_owned(),
            Some((val, child)) => {
                format!("{} -> {}", *val, child.stringify())
            }
        }
    }
}

fn main() {
    let mut ll: LinkedList<i32> = LinkedList::default();

    ll.add_head(12);
    ll.add_head(45);
    ll.add_tail(4);
    ll.add_tail(38);
    ll.add_head(21);

    println!("list: {}", ll.stringify());
    println!("len : {}", ll.len());

    println!("popped head: {:?}", ll.pop_head());
    println!("new list   : {}", ll.stringify());

    println!("popped tail: {:?}", ll.pop_tail());
    println!("new list   : {}", ll.stringify());

    println!("popped tail: {:?}", ll.pop_tail());
    println!("popped tail: {:?}", ll.pop_tail());
    println!("popped tail: {:?}", ll.pop_tail());
    println!("popped tail: {:?}", ll.pop_tail());
    println!("popped tail: {:?}", ll.pop_tail());

    println!("popped head: {:?}", ll.pop_head());
}
