fn main() {
    
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}");

    let mut  y: i32 = 0;
    y = y + 1;
    println!("The value of y is: {y}");
}