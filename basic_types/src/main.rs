fn main() {
    let custom_num = 98_000;
    let hex_num = 0xfa;
    let bin_num = 0b0101;
    let byte_num = b'A';

    println!("{}", bin_num);
    println!("{}", hex_num);
    println!("{}", custom_num);
    println!("{}", byte_num);

    let float_num: f32 = 3.14;
    let float_num_2: f64 = 3.2334322178;
    let tup: (i32, &str, u8) = (20, "Hello", 1);

    println!("{}", tup.1);

    let (a,b,c) = tup;
    println!("{}", c);

    let x = [1, 5, 6, 7];
    println!("{:?}", x);

    let y = [66; 88];
    println!("{:?}", y);
}
