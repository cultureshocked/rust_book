#![allow(unused)]

// structs that contain references bust also be valid for the same scope as the reference at best
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// we do not need to specify lifetimes for this particular method because of lifetime elision where
// the compiler can infer our intent based on its deterministic rule-set.
// it's still good practice to annotate them, though.
impl<'a> ImportantExcerpt<'a> {
    fn announce_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part 
    }
}

fn main() {
    let static_string: &'static str = "I have static lifetime."; // special lifetime indicating
    // that the data is available for the entire duration of the program. static keyword in C
    let s1 = "abcd".to_string();
    let s2 = "xyz"; // &str 
    
    let res = longest(s1.as_str(), s2);
    println!("The longest string is {res}");

    {
        let s3 = String::from("lmnop"); // new smaller lifetime 
        let scoped_res = longest(s2, s3.as_str()); // the 'a lifetime will now refer to this scope
        println!("the longest string inside the new scope is {scoped_res}"); // valid reference use
    }
    // scoped_res no longer exists, nor does s3 or any reference to it.
    // println!("the longest string inside the new scope is {scoped_res}"); // wont compile
    

    let novel = "Call me Ishmael. Some years ago...".to_string();
    let first_sentence = novel
        .split('.') // splits on period to Array type 
        .next() // gets next (first) value in collection
        .expect("Could not find a '.'"); // panic if invalid
    let i = ImportantExcerpt { // same lifetime as the first_sentence and novel pieces of data
        part: first_sentence,
    };
    println!("Excerpt: {}", i.part);
}

// lifetime annotations are a type of generic as well and must be declared as such in the function
// declaration.
// both inputs having same lifetime and output being same lifetime means that both inputs must
// _have_ the same lifetime/exist within the same scope, and the return value references some data
// that has the same lifetime as the data that the parameters reference.
//
// suppose both references have differing lifetimes in reality: the 'a lifetime will refer to the
// smallest lifetime. 
//
// lifetimes are effectively a contract/a set of rules we define the compiler to try and follow
// when enforcing borrowing rules.
//
// the borrow checker does not actually know/care about the real lifetimes of the parameters (and
// return value) but rather it just tries to find some scope that satisfies the requirements when
// the function is used.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
