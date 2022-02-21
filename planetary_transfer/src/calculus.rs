pub fn round_to(value: f64, decimal: usize) -> f64 {
    (value * (10 as f64).powi(decimal as i32)).round() / (10 as f64).powi(decimal as i32)
}

pub trait Calculus {
    type Output;

    fn base_quantity(&self) -> f64;

    fn new(quantity: f64) -> Self where Self: Sized;
}

macro_rules! calculus {
    ($($t:ty),*) => ($(
        impl Add for $t {
            type Output = $t;

            fn add(self, rhs: $t) -> $t {
                Self::new(self.base_quantity() + rhs.base_quantity())
            }
        }
        impl Sub for $t {
            type Output = $t;

            fn sub(self, rhs: $t) -> $t {
                Self::new(self.base_quantity() - rhs.base_quantity())
            }
        }
        impl Mul<f64> for $t {
            type Output = $t;

            fn mul(self, rhs: f64) -> $t {
                Self::new(self.base_quantity() * rhs)
            }
        }
        impl Div<f64> for $t {
            type Output = $t;

            fn div(self, rhs: f64) -> $t {
                Self::new(self.base_quantity() / rhs)
            }
        }
    )*)
}

pub(crate) use calculus;