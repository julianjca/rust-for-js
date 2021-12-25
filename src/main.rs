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
    println!("Total: {} + {} = {}", price, tax, total);

    // array
    let spidermen = ["Tobey Maguire", "Andrew Garfield", "Tom Holland"];

    // initialize array
    let top_scores: [u32; 3] = [292, 170, 140];

    // every item in an array should have the same type
    println!("{}", top_scores.len());
    println!("{}", spidermen.len());

    // printing array
    println!("{:?}", top_scores);
    println!("{:?}", spidermen);

}
