// Associated Types in Traits

trait Temperature {
    type Value;

    fn zero() -> Self::Value;
}

struct Fahrenheit;

impl Temperature for Fahrenheit {
    type Value = f32;

    fn zero() -> Self::Value {
        255.37
    }
}

#[derive(Debug)]
enum Quantitative {
    Warm,
    JustRight,
    Cold,
}

impl Temperature for Quantitative {
    type Value = usize;

    fn zero() -> Self::Value {
        Quantitative::Cold as usize
    }
}

fn main() {
    println!("{}", Fahrenheit::zero());
    println!("{}", Quantitative::zero());
}
