fn main() {
    let a = vec![];
    let mut b = a;
    let x = 5;
    let y = x;
    let z = &x;

    // z = 6; // not allowed

    println!("{}", x + y);
    println!("{}", z);
    b.push(1);
    // a.push(1);
    println!("{:?}", b);
}
