

fn main() {
    let a = 10;
    let b = &a;
    let c = &b;

    println!("{}", a == **c);
}



