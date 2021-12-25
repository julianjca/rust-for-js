fn main() {
    println!("Hello, world!");

    // mutable variable
    let mut name = "John";
    println!("{}", name);
    name = "John Doe";
    println!("{}", name);

    // const variable
    // you need to define the type

    const IS_AVAILABLE : bool = true;

    println!("{}", IS_AVAILABLE)
}
