#![allow(unused)]

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text); // from state 1 -> 2 or 2 -> 2
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { // take() unwraps the option value but leaves the
            // original reference with a None value (null doesnt exist here)
            self.state = Some(s.request_review()); // calls the method on the state object to get
            // the next state, rather than implementing the whole state machine on the post
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// declare our type of State
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str { // default implementation returns empty
        // stringslice
        ""
    }
}

// three possible states
struct Draft {}

struct PendingReview {}

struct Published {}

// let them be states by behaviour
// each state represents its own rules of the state machine;
// the Post data itself does not worry about this implementation; its methods are always the same.
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> { // goes to next state (1/2 -> 3)
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // from state 3 it stays in state 3
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // already published, no need to regress
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str { // basically we control here what will be returned
        // from the post
        &post.content
    }
}
