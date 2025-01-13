use rust_traits::*;

fn main() {
    let f = Fahrenheit::from(Celsius::new(17));
    println!("{}", f);

    let c = Celsius::from(Fahrenheit::new(35));
    println!("{}", f);

    if f == c {
        println!("{} == {}", f, c);
    } else if f < c {
        println!("{} < {}", f, c);
    } else {
        println!("{} > {}", f, c);
    }
}
