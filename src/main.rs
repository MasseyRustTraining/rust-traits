use std::cmp::Ordering;

use rust_traits::*;

fn main() {
    let f = Fahrenheit::from(Celsius::new(17));
    println!("{}", f);

    let c = Celsius::from(Fahrenheit::new(35));
    println!("{}", c);

    match f.cmp(&c.into()) {
        Ordering::Equal => println!("{} == {}", f, c),
        Ordering::Less => println!("{} < {}", f, c),
        Ordering::Greater => println!("{} == {}", f, c),
    }
}
