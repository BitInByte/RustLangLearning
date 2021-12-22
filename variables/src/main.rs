fn main() {
    const MAX_POINTS: u32 = 100_000; // Constants
                                     // let x = 5; // Immutable value, doesn't allow this variable to change
    let mut x = 5; // Mutable value, allow this variable to change
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
