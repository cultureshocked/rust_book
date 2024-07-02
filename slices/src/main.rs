fn main() {
    let mut s = String::from("hello world!"); // must be mut to clear()
    let w_offset = first_word(&s);

    s.clear();
    // w_offset retains its value of 5, but it doesn't have its attached semantics
    // thus, w_offset is effectively invalid, even if the copiler doesnt complain
    println!("{}", w_offset);

    s.push_str("hello world!");
    
    // slices are borrows
    let hello = &s[0..5]; // `..` ranges are exclusive
    let world = &s[6..11]; 
    println!("{} {}!", hello, world);

    let end_word = &s[6..]; // omit end num to indicate "until the end"
    println!("{} {}", hello, end_word);

    let first_word = first_word_slice(&s);
    println!(".{}.", first_word);

    // now this throws an error
    // s.clear(); // this is a mutable borrow
    // println!("{}", first_word);

    // the type &str (a string slice) is an immutable reference
    // string references are effectively identical to string slices
    // thus, working with them in a string slice context...
    println!("{}", first_word_from_slice(&s));

    // while this example illustrates strings,
    // slices of other array types works too
    let my_numbers = [1, 2, 3, 4, 5];
    println!("{:?}", &my_numbers[1..3]);
}

// usize is pointer-sized integer type
// often used in lengths
// same as size_t basically
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter() converts array to range
    // enumerate() converts data to tuple of index and data
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len() // fallback
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[0..]
}

fn first_word_from_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[0..]
}
