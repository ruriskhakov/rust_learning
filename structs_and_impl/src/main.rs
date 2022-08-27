use std::{thread, time};

struct Philosopher {
    name: String,
}
impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
    fn eat(&self) {
        println!("{} - starts to eat.", self.name);
        thread::sleep(time::Duration::from_millis(1000));
        println!("{} - finished to eat.", self.name);
    }
}
fn main() {
    let philosophers = vec![
        Philosopher::new("Джудит Батлер"),
        Philosopher::new("Raya Dunaevskaya"),
        Philosopher::new("Зарубина Наталья"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Anna Shmidt"),
    ];
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        thread::spawn(move || {
            p.eat();
        })
    }).collect();
    for h in handles {
        h.join().unwrap();
    }
}
