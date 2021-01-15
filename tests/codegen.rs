use macros::arcorn;
use std::rc::Rc;

arcorn! {

    enum List {
        Cons(Rc<Cons>),
        Nil(Rc<Nil>),
    }

    struct Cons {
        val: i32,
        tail: Rc<List>,
    }

    struct Nil {}
}

fn test() {
    let tail = Rc::new(List::Nil(Rc::new(Nil {})));
    let tail = Rc::new(List::Cons(Rc::new(Cons { val: 0, tail })));
    let tail = Rc::new(List::Cons(Rc::new(Cons { val: 2, tail })));
    let tail = Rc::new(List::Cons(Rc::new(Cons { val: 3, tail })));
    let tail = Rc::new(List::Cons(Rc::new(Cons { val: 4, tail })));

    let arcon_tail: arcon_types::List = tail.as_ref().into();
    let arc_tail: List = arcon_tail.into();
}

// To view generated code:
// $ cargo install cargo-expand
// $ cargo expand --tests codegen
