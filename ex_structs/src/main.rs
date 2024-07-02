#[derive(Debug)] // opt-in to debug trait
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels", area(&rect1));
    println!("rect1: {:?}", rect1); // all fields on same line
    // println!("rect1: {rect1:?}") //  also valid
    println!("rect1: {:#?}", rect1); //  each field and brace on its own line
    
    // can also use dbg! macro
    // but dbg! takes ownership of value passed in; println! implicitly converts to reference as it
    // expands. it does `return` ownership of the expression, though.
    // dbg! also prints to stderr instead of stdout
    dbg!(&rect1);
    let rect1 = dbg!(rect1); // returns to new shadowed rect1
    println!("rect1: {rect1:?}");
}


fn area(r: &Rect) -> u32 {
    r.width * r.height
}
