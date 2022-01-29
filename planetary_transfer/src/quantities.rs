pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430E-11;
pub const LUNAR_MASS: f64 = 7.34767309E22;
pub const EARTH_MASS: f64 = 5.9722E24;
pub const JOVIAN_MASS: f64 = 1.89813E27;
pub const SOLAR_MASS: f64 = 1.98847E30;
pub const AU_METERS: f64 = 149598023E3;

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
    pub fn from_kg(mass: f64) -> Self {
        Self {
            kg: mass,
            lunar: mass / LUNAR_MASS,
            earth: mass / EARTH_MASS,
            jovian: mass / JOVIAN_MASS,
            solar: mass / SOLAR_MASS,
            gravitational_parameter: mass * GRAVITATIONAL_CONSTANT,
        }
    }

    pub fn from_lunar(mass: f64) -> Self {
        Self {
            lunar: mass,
            ..Self::from_kg(mass * LUNAR_MASS)
        }
    }
    pub fn from_earth(mass: f64) -> Self {
        Self {
            earth: mass,
            ..Self::from_kg(mass * EARTH_MASS)
        }
    }

    pub fn from_jovian(mass: f64) -> Self {
        Self {
            jovian: mass,
            ..Self::from_kg(mass * JOVIAN_MASS)
        }
    }

    pub fn from_solar(mass: f64) -> Self {
        Self {
            solar: mass,
            ..Self::from_kg(mass * SOLAR_MASS)
        }
    }
}

impl Mass {
    pub fn kg_updated(&mut self) {
        *self = Self::from_kg(self.kg);
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
pub struct SemiMajorAxis {
    pub m: f64,
    pub km: f64,
    pub au: f64,
}

impl SemiMajorAxis {
    pub fn from_m(sma: f64) -> Self {
        Self {
            m: sma,
            km: sma / 1E3,
            au: sma / AU_METERS,
        }
    }

    pub fn from_km(sma: f64) -> Self {
        Self {
            km: sma,
            ..Self::from_m(sma * 1E3)
        }
    }

    pub fn from_au(sma: f64) -> Self {
        Self {
            au: sma,
            ..Self::from_m(sma * AU_METERS)
        }
    }
}

impl SemiMajorAxis {
    pub fn m_updated(&mut self) {
        *self = Self::from_m(self.m);
    }

    pub fn km_updated(&mut self) {
        *self = Self::from_km(self.km);
    }

    pub fn au_updated(&mut self) {
        *self = Self::from_au(self.au);
    }
}

pub struct Velocity {
    pub mmps: f64,
    pub mps: f64,
    pub kps: f64,
}

impl Velocity {
    pub fn from_mps(velocity: f64) -> Self {
        Self {
            mmps: velocity / 1E-3,
            mps: velocity,
            kps: velocity / 1E3,
        }
    }

    pub fn from_mmps(velocity: f64) -> Self {
        Self {
            mmps: velocity,
            ..Self::from_mps(velocity * 1E-3)
        }
    }

    pub fn from_kps(velocity: f64) -> Self {
        Self {
            kps: velocity,
            ..Self::from_mps(velocity * 1E3)
        }
    }
}

impl Velocity {
    pub fn mmps_updated(&mut self) {
        *self = Self::from_mmps(self.mmps);
    }

    pub fn mps_updated(&mut self) {
        *self = Self::from_mps(self.mps);
    }

    pub fn kps_updated(&mut self) {
        *self = Self::from_kps(self.kps);
    }
}