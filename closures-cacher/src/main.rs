use std::format;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let expensive_calc = |arg: i32| {
        thread::sleep(Duration::from_secs(1));
        println!("Calculating for Arg: {arg}");
        format!("Query Value for: {arg}")
    };
    let queries = vec![2,3,2,4,7,8,2];
    let mut cacher = Cacher::new( expensive_calc);
    for query in queries {
        println!("{}", cacher.value(query));
    }
}

struct Cacher<T> where T: Fn(i32) -> String {
    calculation: T,
    values: HashMap<i32, String>,
}

impl <T> Cacher<T> where T: Fn(i32) -> String {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(& mut self, arg: i32) -> &String {
        if !self.values.contains_key(&arg) {

            self.values.insert(arg, (self.calculation)(arg));
        }
        self.values.get(&arg).unwrap()
    }
}
