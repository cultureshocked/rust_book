#![allow(unused)]
fn main() {
    //vectors scale in size as needed, so it doesnt make sense to not `mut` them
    let mut v_verbose: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);

    let third = &v[2]; // this will cause runtime panic if index outofbounds
    // use squarebracket access if we _want_ a crash for debugging, e.g.:
    // let hundredth = &v[100];
    println!("third element: {}", third);

    let third = v.get(2); //.get returns type Option<&T>
    if let Some(n) = third { // n is borrowed immutable reference
        println!("Accessed third element of vector: {n}");
    } else { // none
        println!("Third element does not exist/is out of bounds.");
    }

    // this doesnt work either: holding mutable ref to data in vec _while_
    // modifying the (mutable) vec

    let first = &v[0]; // type &T
    // v.push(6); // bad!
    println!("The first element is {first}");

    // vectors may reallocate new memory adjacent to old memory, but will likely
    // allocate to new location and perform some memcpy operation
    // as such, the old/first reference will be invalidated and cannot exist in the scope

    // iterating...
    for i in &v {
        print!("{i}, ");
    }
    print!("\n");

    // modifying in iteration
    // this is allowed while holding the reference `first` as no new allocations will be made
    // if we were to add v.push() or something similar, we would get the same error(s) as line 25
    for i in &mut v {
        *i *= 2;
    }
    dbg!(&v);

    // when the vector(s) v and v_verbose are dropped, all their elements are _also_ dropped
    // references to the vector can only be used while the vector is valid
    // by the borrow checker's rules
}
