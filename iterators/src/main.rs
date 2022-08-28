fn main() {
    let mut range = 0..10;
    loop {
        match range.next() {
            Some(x) => {
                println!("{}", x);
            },
            None => { break }
        }

    }
    let one_to_one_hundred: _ = (1..101).collect()::<Vec<_>>();
    let one_to_one_hundred = (1..101).collect::<Vec<i32>>();
}
