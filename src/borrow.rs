struct Thing {
    data: u32
}

fn i_take_an_owned(thing: Thing) -> Thing {
    i_take_a_ref(&thing);
    Thing { data: thing.data * 2 }
}

fn i_take_a_ref(thing: &Thing) {
    println!("{}", thing.data);
}

fn i_take_a_mut_ref(thing: &mut Thing) {
    i_take_a_ref(thing);
    thing.data = 8
}

pub fn the_borrow_checker() {
    let mut thing = Thing { data: 4 };

    i_take_a_ref(&thing);
    i_take_a_mut_ref(&mut thing);
    let second_thing = i_take_an_owned(thing);
    i_take_a_ref(&second_thing);

    // Cannot use binding to `thing` after it was moved
    // i_take_a_ref(&thing); // Does not compile
}