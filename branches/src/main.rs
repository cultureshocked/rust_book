fn main() {
    let number = 3;

    // no brackets needed for condition
    if number < 5 {
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }

    // we cannot coerce numerics to bools implicitly
    // use `if number != 0 {` instead of `if number {`
    if number != 0 {
        println!("Number is not zero");
    }

    // chain conditions with `else if`
    if number % 4 == 0 {
        println!("number is divisible by four");
    } else if number % 3 == 0 {
        println!("number is divisible by three");
    } else if number % 2 == 0 {
        println!("number is even");
    } else {
        println!("number has no factors under five");
    }

    // conditionals are expressions and can be bound to variables
    // as a result rust does not have ternaries
    let condition = true;
    let new_number = if condition { 7 } else { 2 }; // both values must have same type -- rustc
    // will catch this

    println!("{}", new_number);

    // three loops -- loop, for, while
    // loops are expressions -- break is like return
    // note: only applies to `loop` keyword -- not other loop types
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2 // effectively return value from loop 
        }
    }; // semicolon here as it is end of expression for binding result
    println!("result: {}", result);

    // loops can be disambiguated by using loop labels
    // starts with singlequote
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1
    }

    println!("end of count: {count}");

    // conditional loops with while use similar syntax to if statements
    let a = [10, 20, 30, 40, 50];
    let mut idx = 0;
    while idx < 5 {
        println!("idx: {idx}, element: {}", a[idx]);
        idx += 1;
    }
    
    // forloops are range-based
    for element in a {
        println!("element: {element}");
    }

    for idx in (0..5).rev() {
        println!("element at idx {idx}: {}", a[idx]);
    }

}
