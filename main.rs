

fn main() {
    // you can place on stack only values with static size
    let a = 10;
    let b = a;
    let c = 15;
    let d = add(a, b);

    let message = String::from("Hello");
    println!("{}", message);
}


fn add(x: u32, y: u32) -> u32 {
    let sum = x + y;
    sum
}
