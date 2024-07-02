fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => println!("Rolled three"),
        7 => println!("Rolled seven"),
        i => println!("Rolled {}", i),
    }
}
