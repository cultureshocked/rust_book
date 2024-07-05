fn main() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter(); // creates Iterator object
    let total: i32 = v1_iter.sum(); // his method consumes the iterator; v1_iter is no logner valid
    println!("{total}");

    let v1_iter = v1.iter();
    let squares = v1_iter.map(|x| x * x); // creates a NEW type of iterator which cane take
    // additional methods.
    let v_squares: Vec<i32> = squares.collect(); // collects all values in the iterator to a new
    // collection.
    // since .collect() can make arrays, vectors, slices, or other containers, we _must_ specify
    // the type of the variable we are storing it in for the compiler to recognize what type we
    // want to collect() to.
    println!("{v_squares:?}");

    // we can also chain methods if they return an iterator
    let v1_iter_squares_collection: Vec<i32> = v1.iter()
        .map(|x| x + 1)
        .map(|x| x * x)
        .filter(|n| n % 2 == 0)
        .collect(); // until the iterator is finally consumed.
    // iterators are all lazy, and only really _do something_ when they are consumed.
    // as such, we should probably consume them at the end of the chain.
    
    println!("{v1_iter_squares_collection:?}");
}
