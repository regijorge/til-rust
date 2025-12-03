fn main() {
    let x = 5;
    println!("The value of x is {x}. x is immutable");
    let x = x + 7;
    println!("The value of x now is {x}. Because we're shadowing");

    let mut y = 7;
    println!("The value of y is {y}");
    y = 8;
    println!("Now the value of y is {y} Because it  is mutable");

}    

