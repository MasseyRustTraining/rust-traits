use std::fmt::{self, Display, Formatter};

/// Temperature in Fahrenheit.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Fahrenheit(i64);

impl Fahrenheit {
    pub fn new(temperature: i64) -> Self {
        Self(temperature)
    }
}

impl Display for Fahrenheit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}°F", self.0)
    }
}

/// Temperature in Celsius.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Celsius(i64);

impl Celsius {
    pub fn new(temperature: i64) -> Self {
        Self(temperature)
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}°C", self.0)
    }
}

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
