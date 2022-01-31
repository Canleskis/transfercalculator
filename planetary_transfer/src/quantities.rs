use std::ops::{Add, Sub, Mul, Div};
use std::iter::once;
use std::fmt::Debug;

use crate::round_to;
use crate::{Calculus, calculus};

pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430E-11;
pub const KILOGRAMS_LUNAR: f64 = 7.34767309E22;
pub const KILOGRAMS_EARTH: f64 = 5.9722E24;
pub const KILOGRAMS_JOVIAN: f64 = 1.89813E27;
pub const KILOGRAMS_SOLAR: f64 = 1.98847E30;

pub const SECONDS_MINUTE: f64 = 60.0;
pub const SECONDS_HOUR: f64 = 3600.0;
pub const SECONDS_DAY: f64 = 86400.0;
pub const SECONDS_MONTH: f64 = 2629746.0;
pub const SECONDS_YEAR: f64 = 31556952.0;

pub const METERS_AU: f64 = 149598023E3;

#[derive(Copy, Clone, Debug)]
pub struct Duration {
    pub s: f64,
    pub min: f64,
    pub h: f64,
    pub d: f64,
    pub m: f64,
    pub y: f64,
}

impl Duration {
    pub fn from_seconds(seconds: f64) -> Self {
        Self {
            s: seconds,
            min: seconds / SECONDS_MINUTE,
            h: seconds / SECONDS_HOUR,
            d: seconds / SECONDS_DAY,
            m: seconds / SECONDS_MONTH,
            y: seconds / SECONDS_YEAR,
        }
    }

    pub fn from_minutes(duration: f64) -> Self {
        Self {
            ..Self::from_seconds(duration * SECONDS_MINUTE)
        }
    }

    pub fn from_hours(duration: f64) -> Self {
        Self {
            ..Self::from_seconds(duration * SECONDS_HOUR)
        }
    }

    pub fn from_days(duration: f64) -> Self {
        Self {
            ..Self::from_seconds(duration * SECONDS_DAY)
        }
    }

    pub fn from_months(duration: f64) -> Self {
        Self {
            ..Self::from_seconds(duration * SECONDS_MONTH)
        }
    }

    pub fn from_years(duration: f64) -> Self {
        Self {
            ..Self::from_seconds(duration * SECONDS_YEAR)
        }
    }
}

impl Duration {
    pub fn round_to(mut self, decimal: usize) -> Duration {
        self.s = round_to(self.s, decimal);
        self.min = round_to(self.min, decimal);
        self.h = round_to(self.h, decimal);
        self.d = round_to(self.d, decimal);
        self.m = round_to(self.m, decimal);
        self.y = round_to(self.y, decimal);
        self
    }

    fn iter_with_string(&self) -> Box<dyn Iterator<Item = (f64, String)> + '_> {
        Box::new(once((self.y, "years".to_string()))
        .chain(once((self.m, "months".to_string())))
        .chain(once((self.d, "days".to_string())))
        .chain(once((self.h, "hours".to_string())))
        .chain(once((self.min, "minutes".to_string())))
        .chain(once((self.s, "seconds".to_string()))))
    }

    pub fn smallest_duration(&mut self) -> (f64, String) {
        let duration = self.iter_with_string().find(|item| item.0 >= 1.0);
        if let Some(value) = duration {
            value
        } else {
            (self.s, "seconds".to_string())
        }
    }

    pub fn smallest_duration_formatted(&mut self) -> String {
        let duration = self.smallest_duration();
        format!("{} {}", duration.0, duration.1)
    }
}

#[derive(Copy, Clone)]
pub struct Mass {
    pub kg: f64,
    pub lunar: f64,
    pub earth: f64,
    pub jovian: f64,
    pub solar: f64,
    pub gravitational_parameter: f64,
}

impl Mass {
    pub fn from_kilograms(mass: f64) -> Mass {
        Self {
            kg: mass,
            lunar: mass / KILOGRAMS_LUNAR,
            earth: mass / KILOGRAMS_EARTH,
            jovian: mass / KILOGRAMS_JOVIAN,
            solar: mass / KILOGRAMS_SOLAR,
            gravitational_parameter: mass * GRAVITATIONAL_CONSTANT,
        }
    }

    pub fn from_lunar(mass: f64) -> Mass {
        Self {
            ..Self::from_kilograms(mass * KILOGRAMS_LUNAR)
        }
    }
    pub fn from_earth(mass: f64) -> Mass {
        Self {
            ..Self::from_kilograms(mass * KILOGRAMS_EARTH)
        }
    }

    pub fn from_jovian(mass: f64) -> Mass {
        Self {
            ..Self::from_kilograms(mass * KILOGRAMS_JOVIAN)
        }
    }

    pub fn from_solar(mass: f64) -> Mass {
        Self {
            ..Self::from_kilograms(mass * KILOGRAMS_SOLAR)
        }
    }
}

impl Mass {
    pub fn kg_updated(&mut self) {
        *self = Self::from_kilograms(self.kg);
    }

    pub fn lunar_updated(&mut self) {
        *self = Self::from_lunar(self.lunar);
    }

    pub fn earth_updated(&mut self) {
        *self = Self::from_earth(self.earth);
    }

    pub fn jovian_updated(&mut self) {
        *self = Self::from_jovian(self.jovian);
    }

    pub fn solar_updated(&mut self) {
        *self = Self::from_solar(self.solar);
    }
}

#[derive(Copy, Clone)]
pub struct Distance {
    pub m: f64,
    pub km: f64,
    pub au: f64,
}

impl Distance {
    pub fn from_meters(sma: f64) -> Distance {
        Self {
            m: sma,
            km: sma / 1E3,
            au: sma / METERS_AU,
        }
    }

    pub fn from_kilometers(sma: f64) -> Distance {
        Self {
            ..Self::from_meters(sma * 1E3)
        }
    }

    pub fn from_astronomical_unit(sma: f64) -> Distance {
        Self {
            ..Self::from_meters(sma * METERS_AU)
        }
    }
}

impl Distance {
    pub fn m_updated(&mut self) {
        *self = Self::from_meters(self.m);
    }

    pub fn km_updated(&mut self) {
        *self = Self::from_kilometers(self.km);
    }

    pub fn au_updated(&mut self) {
        *self = Self::from_astronomical_unit(self.au);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Velocity {
    pub mmps: f64,
    pub mps: f64,
    pub kps: f64,
}

impl Velocity {
    pub fn from_meters_per_second(velocity: f64) -> Velocity {
        Self {
            mmps: velocity / 1E-3,
            mps: velocity,
            kps: velocity / 1E3,
        }
    }

    pub fn from_millimeters_per_second(velocity: f64) -> Velocity {
        Self {
            ..Self::from_meters_per_second(velocity * 1E-3)
        }
    }

    pub fn from_kilometers_per_second(velocity: f64) -> Velocity {
        Self {
            ..Self::from_meters_per_second(velocity * 1E3)
        }
    }
}

impl Velocity {
    pub fn mmps_updated(&mut self) {
        *self = Self::from_millimeters_per_second(self.mmps);
    }

    pub fn mps_updated(&mut self) {
        *self = Self::from_meters_per_second(self.mps);
    }

    pub fn kps_updated(&mut self) {
        *self = Self::from_kilometers_per_second(self.kps);
    }
}

impl Calculus for Duration {
    type Output = Duration;
    fn base_quantity(&self) -> f64 {
        self.s
    }

    fn new(quantity: f64) -> Self {
        Self::from_seconds(quantity)
    }
}

impl Calculus for Distance {
    type Output = Distance;

    fn base_quantity(&self) -> f64 {
        self.m
    }

    fn new(quantity: f64) -> Self {
        Self::from_meters(quantity)
    }
}

impl Calculus for Velocity {
    type Output = Velocity;

    fn base_quantity(&self) -> f64 {
        self.mps
    }

    fn new(quantity: f64) -> Self {
        Self::from_meters_per_second(quantity)
    }
}

calculus!{Duration, Distance, Velocity}