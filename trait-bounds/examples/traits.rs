trait MakeI64 {
    fn convert(&self) -> i64;
}

impl MakeI64 for bool {
    fn convert(&self) -> i64 {
        if *self {
            0
        } else {
            1
        }
    }
}

impl MakeI64 for u64 {
    fn convert(&self) -> i64 {
        *self as i64
    }
}

fn main() {
    let boolean: bool = true;
    let number: u64 = u64::MAX;

    println!("{} -> {}", boolean, boolean.convert()); // true -> 0
    println!("{} -> {}", number, number.convert()); // 18446744073709551615 -> -1
}
