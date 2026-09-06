use std::rc::Rc;

use crate::List::Cons;
use crate::List::Nil;

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(3, Box::new(a));
// }



enum List {
    Cons(i32, Rc<List>),
    Nil,
}
fn main() {
    let a = Rc::new(Cons(
        5, Rc::new(
            Cons(10, Rc::new(Nil))
        )
    ));
    println!("1 - {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("2 - {}", Rc::strong_count(&a));
    let b = Cons(4, Rc::clone(&a));
    println!("3 - {}", Rc::strong_count(&a));
}
