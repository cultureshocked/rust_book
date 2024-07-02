#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// use `impl` (implementation) block to define functions attached to data
// these functions are called 'associated functions'
// a type may have multiple `impl` blocks, which is useful when implementing traits
impl Rectangle {
    // instance methods take reference to the instance as first parameter
    // effectively borrowing from `Self` instance
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // some associated functions do not need a reference to an instance 
    // (e.g., class methods) but these will need additional scope-resolution to call them
    // (see line 50)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    let r2 = Rectangle {
        width: 10,
        height: 40,
    };
    let r3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("area of r1: {}", r1.area());
    println!("r1 can_hold r2? {}", r1.can_hold(&r2));
    println!("r1 can_hold r3? {}", r1.can_hold(&r3));

    let r4 = Rectangle::square(25); //  associated methods use :: namespace/scope resolution
    //  operator to resolve symbol
    dbg!(&r4);
}
