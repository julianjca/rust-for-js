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

    println!("{}", IS_AVAILABLE);

    // Scalars
    let price = 129;
    let tax = 23.22;

    // f64::from converts price.
    // :: is like . in javascript
    let total = f64::from(price) + tax;
    println!("Total: {} + {} = {}", price, tax, total)
}
