

fn main() {
    let a = 10;
    let b = &a;
    let mut c = &b;
    let d = b;

    let e = &&100;

    c = e;

    println!("e: {:p}", e);
    println!("e: {:p}", *e);
    println!("c: {:p}", c);
    println!("c: {:p}", *c);
}
