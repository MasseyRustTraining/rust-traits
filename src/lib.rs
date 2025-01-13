/// Temperature in Fahrenheit.
pub struct Fahrenheit(i64);

/// Temperature in Celsius.
pub struct Celsius(i64);

impl From<Fahrenheit> for Celsius {
    fn from(Fahrenheit(value): Fahrenheit) -> Celsius {
        Celsius(5 * (value - 32) / 9)
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(Celsius(value): Celsius) -> Fahrenheit {
        Fahrenheit(9 * value / 5 + 32)
    }
}
