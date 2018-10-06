use std::thread;
use std::time::Duration;
use std::collections::HashMap;


fn main() {
    let simulated_user_specific_value = 10;
    let simulated_random_number = 7;

//    generate_workout(
//        simulated_user_specific_value,
//        simulated_random_number,
//    )
//
}


struct Cacher<T, U> where T: Fn(U) -> U {
    calculation: T,
    value: HashMap<U, U>,
}


impl<T, U> Cacher<T, U> 
    where 
        T: Fn(U) -> U, 
        U: std::hash::Hash, 
        U: std::cmp::Eq, 
        U: Copy 
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher { calculation, value: HashMap::new() }
    }

    fn value(&mut self, arg: U) -> U {
        self.value.entry(arg).or_insert((self.calculation)(arg));
        self.value[&arg]
    }
}


#[test]
fn iterator_map() {
    let v = vec![1, 2, 3];
    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}


#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn iteration_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}


#[test]
fn test_cacher() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value("pewpew");
    let v2 = c.value("sdsdf");

    assert_eq!(v1, "pewpew");
    assert_eq!(v2, "sdsdf");
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));

        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}
