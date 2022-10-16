use std::fs;
fn main() {
    let f = fs::read_to_string("/etc/services");
    for line in f {
        println!("{}", line);
    }
}
