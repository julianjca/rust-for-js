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

    // tuple
    // tuple needs to have the same type on each element
    let mut product = ("iPhone 12 Pro Max", 1099, true);
    println!("{:?}", product);
    product = ("PS5", 499, false);
    // deconstruct
    let (_, _, is_available) = product;

    // rest operator
    let [my_score, ..] = top_scores;

    println!("{}", is_available);
    println!("{}", my_score);

    // calling return function
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}


fn plus_one(x: i32) -> i32 {
    x + 1
}