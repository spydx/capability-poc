use std::thread;

use std::time::{Duration};
#[allow(dead_code)]
struct Person {
    id: i64, 
    firstname: String, 
    lastname: String,
}

struct Capability<T>
where T: Fn(i64) -> i64
{
    operation: T,
    value: Option<i64>
}


impl<T> Capability<T>
where T: Fn(i64) -> i64,
{
    fn new(operation: T) -> Capability<T> {
        Capability {
            operation: operation,
            value: None,
        }
    }

    fn value(&mut self, arg: i64) -> i64{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.operation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    println!("hellowO");
    let _workout = generate_workout(1, 1);

}

fn generate_workout(intensity: i64, random_number: i64) -> i64 {
    let mut expensive_result = Capability::new(|num| {
        println!("slow shit");
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
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
    random_number
}


#[cfg(test)]
mod tests {
    use crate::generate_workout;
    use rstest::rstest;

    #[rstest]
    #[case(25, 3)]
    #[case(3,3)]
    #[case(1,1)]
    fn workout(#[case] input: i64, #[case] expected: i64) {
        assert_eq!(expected, generate_workout(input, expected))
        
    }
}



