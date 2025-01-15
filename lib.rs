use std::fmt::{self, Display, Formatter};

pub trait Temperature {
    fn from_kelvin(self) -> i64;
    fn into_kelvin(self) -> i64;
}

impl<U: Temperature> PartialOrd<U> for Fahrenheit {
    fn partial_cmp(&self, other: &U) -> Option<std::cmp::Ordering> {
        self.into_kelvin().partial_cmp(&other.into_kelvin())
    }
}

impl<U: Temperature> PartialOrd<U> for Celsius {
    fn partial_cmp(&self, other: &U) -> Option<std::cmp::Ordering> {
        self.into_kelvin().partial_cmp(&other.into_kelvin())
    }
}

impl<U: Temperature> PartialEq<U> for Fahrenheit {
    fn eq(&self, other: &U) -> bool {
        self.into_kelvin().partial_eq(&other.into_kelvin())
    }
}

impl<U: Temperature> PartialEq<U> for Celsius {
    fn eq(&self, other: &U) -> bool {
        self.into_kelvin().partial_eq(&other.into_kelvin())
    }
}

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
