#![allow(unused)]

// generic struct type
pub struct Pair<T> {
    x: T,
    y: T,
}

// writing generic functions for generic types
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// CONDITIONALLY implementing functions IF their generic type follows trait bounds that we declare
// in this case, if the type T implements both PartialOrd (for comparisions) and Display (for
// printing data) then we will also allow the instance to call this method
impl<T: PartialOrd + std::fmt::Display> Pair<T> {
    fn cmp_dbg(&self) {
        if self.x > self.y {
            println!("x: {} is larger than y: {}", self.x, self.y);
            return;
        }
        println!("x: {} is smaller than y: {}", self.x, self.y);
    }
}

pub struct Article {
    pub author: String,
    pub headline: String,
    pub location: String,
    pub content: String,
}

pub struct Tweet {
    pub author: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct RSSEntry {
    pub author: String,
    pub hash: [u8; 16],
}

pub trait Summary {
    // declarations without definition are allowed here by omitting curlybrace block
    // fn summarize(&self) -> String;

    // or allow some default behaviour which will be overrided in implementations
    fn summarize(&self) -> String {
        "Read more...".to_string()
    }
}

// we can make empty traits for additional constraining
pub trait Display {}

impl Display for RSSEntry {}
impl Display for Article {}
impl Display for Tweet {}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.content)
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {} ({})", self.author, self.headline, self.location)
    }
}

impl Summary for RSSEntry {} // empty block/lack of definition fallsback to default implementation
// in the trait declaration

// we cannot call the default implementation from an override implementation, like doing something
// with super:: or otherwise. this is not inheritance.

fn main() {
    let article = Article {
        author: "DHH".to_string(),
        headline: "Rails is the best".to_string(),
        location: "Denmark".to_string(),
        content: "My framework Rails is the best web framework.".to_string(),
    };

    let tweet = Tweet {
        author: "railshater".to_string(),
        content: "I hate DHH and Ruby on Rails!!!".to_string(),
        reply: false,
        retweet: true,
    };

    let rss = RSSEntry {
        author: "Hey.com".to_string(),
        hash: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    };

    notify(&article);
    notify(&tweet);
    notify(&rss);
}

// to pass interfaced data into a function, use impl keyword
fn notify(content: &impl Summary) {
    println!("Notification: {}", content.summarize());
}

// the above function is actually a shorthand for this entire declaration
// traits are effectively a generic type and can be used as-such in-place.
fn long_notify<T: Summary>(content: &T) {
    println!("Notification: {}", content.summarize());
}

// if we want to bind multiple traits, we can use the + operator
fn combine_traits(content: &(impl Summary + Display)) {
    println!("Notified!");
}

// or the long-form...
fn combine_traits_long<T: Summary + Display>(content: &T) {
    println!("Notified!");
}

// we can return impl-bound objects as well
// note that while this seems generic, the function can only return one concrete type;
// we cannot return either an article or a tweet in the same function definition,
// despite both implementing Summary
fn returns_summarizable() -> impl Summary {
    Tweet {
        author: "coolguy".to_string(),
        content: "aww sweet twitter flame war".to_string(),
        reply: true,
        retweet: true,
    }
}


