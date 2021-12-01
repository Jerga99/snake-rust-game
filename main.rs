

fn main() {
    let a = 10;
    let b = &a;
    let mut c = &b;
    let d = b;

    let e = &&100;
    c = e;


    println!("Value of c: {:p}", c);
    println!("Value of e: {:p}", e);
    println!("Address of 100: {:p}", &(**e));

    println!("Value of c: {:p}", *c);
    println!("Value of e: {:p}", *e);
    println!("Address of 100: {:p}", &(**c));


    // println!("Address of a: {:p}", &a);
    // println!("Value of b: {:p}", b);
    // println!("Value of c: {:p}", c);
    // println!("Value of d: {:p}", d);
}
