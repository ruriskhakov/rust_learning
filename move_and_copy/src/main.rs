fn main() {
    let a = 10;
    let b = a;
    let _c = 15;
    let sum = add(a, b);

    println!("{}", sum);
    println!("{}", _c);
}

fn add(x: u32, y: u32) -> u32 {
    // let result = x + y;
    // result
    x + y
}
