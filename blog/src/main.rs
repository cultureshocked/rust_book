use blog::Post;

// a blog post should have four states:
// 1. creation
// 2. drafting
// 3. requesting approval
// 4. approved/posted

fn main() {
    let mut p = Post::new(); // create empty draft

    p.add_text("I ate salad for lunch today."); // finish draft
    assert_eq!("", p.content());
    p.request_review(); // perform review
    assert_eq!("", p.content());
    p.approve(); // finalize approval
    assert_eq!("I ate salad for lunch today.", p.content());
}
