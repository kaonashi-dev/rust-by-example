
// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32 = 3; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    assert_eq!(y, 3);
    println!("Success!");
}