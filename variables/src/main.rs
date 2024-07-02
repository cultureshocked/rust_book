#[allow(unused_variables)]
fn main() {
    let default_float = 2.0; // f64
    let explicit_float: f32 = 2.0; // f32

    // rust supports consteval numeric operations: + - * / %
    // integer division operates same as in other languages
    
    let regular_bool = true; // bool is its own type; casting it results in 0 or 1
    let ascii_char: char = 'A';
    let unicode_char: char = '⭐';

    // rust chars are fourbyte unicode scalars; not singlebyte ascii
    println!("size_of char: {}", std::mem::size_of::<char>());

    // tuples are a compound type of fixed-length
    let tup: (f64, u32, char) = (5.1, 17, unicode_char);
    
    // can be destructured
    let (a, b, c) = tup;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // or indexed using dot notation
    println!("alternative: a: {}, b: {}, c: {}", tup.0, tup.1, tup.2);

    // arrays are fixed-length; annotation is `[type; length]`
    let arr: [u64; 5] = [1, 2, 3, 4, 5];
    println!("debug of arr: {:?}", arr);

    // similar notation can be used for initialization
    let initialized_arr = [0; 8];
    println!("debug of initialized_arr: {:?}", initialized_arr);

    // squarebracket access; zero indexed
    let mid = arr[2];
    println!("Enter an index: ");

    // rust does have a general purpose string_type
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];
    println!("You accessed index {} and got element {}", index, element);
}
