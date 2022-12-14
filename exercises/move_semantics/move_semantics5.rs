// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.



fn main() {
    let mut x = 100; // mutable x 
    let mut y = &mut x; // y is reference to x and x can only have one reference
    
    *y += 100; //derefence y to add 100
    let z = &mut x; //z = reference to x and it works because y is derefenced
    *z += 1000; //dereference z to add 1000
    assert_eq!(x, 1200); // x is 1200
}
