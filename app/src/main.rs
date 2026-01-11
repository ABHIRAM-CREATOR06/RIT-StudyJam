use std::io;

fn main() {
    // user number input cheyyum then even or odd check
    let mut input = String::new();
    println!("enter a number : ");
    io::stdin().read_line(&mut input).expect("error");
    let number : i32 = input.trim().parse().expect("conversion error");
    if number%2==0{
        println!("even");
    }else{
        println!("odd");
    }

}
