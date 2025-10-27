use core::fmt;

// syntatic sugar for:
//  fn add_42_millions<T: Into<i32>>(x: T) -> i32 
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}

#[derive(Clone)]
struct Temp {
    dividend: i32,
    divisor: i32,
}

impl Into<i32> for Temp {
    fn into(self) -> i32 {
        self.dividend / self.divisor
    }
}

impl fmt::Display for Temp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}/{}", self.dividend, self.divisor).as_str());
        Ok(())
    }
}

fn main() {
    let many = add_42_millions(42_i8);
    dbg!(many);
    let many_more = add_42_millions(10_000_000);
    dbg!(many_more);
    let debuggable = pair_of(27);
    dbg!(debuggable);
    let temp = Temp { dividend: 30, divisor: 5 };
    let a: i32 = temp.clone().into();
    println!("{temp}");
    let b: i32 = temp.into();
    // println!("{temp}");
}
