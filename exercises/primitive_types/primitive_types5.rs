// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.



fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age)/* your pattern here */ = cat; //Destructure tuple elements into bindings is basically giving tuple elements assigned names to be used. 

    println!("{} is {} years old.", name, age);
}
