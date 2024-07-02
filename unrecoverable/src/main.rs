fn main() {
    // panic!("crash and burn");

    // to get a full stack trace + unwind, run this program with this command:
    // `RUST_BACKTRACE=1 cargo run`
    let v = vec![1, 2, 3];
    v[99];
}
