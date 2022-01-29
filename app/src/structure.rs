const LUNAR_MASS: f64 = 7.34767309E22;
const EARTH_MASS: f64 = 5.9722E24;
const JOVIAN_MASS: f64 = 1.89813E27;
const SOLAR_MASS: f64 = 1.98847E30;
const AU_METERS: f64 = 149598023E3;
const KM_METERS: f64 = 1E3;

pub struct Velocity {
    pub mmps: f64,
    pub mps: f64,
    pub kps: f64,
}
#[allow(dead_code)]
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
    pub fn update(&mut self, from: f64, conversion: f64) {
        if from != self.mps {self.mps = from * conversion;}
        if from != self.mmps {self.mmps = self.mps / 1E-3;};
        if from != self.kps {self.kps = self.mps / 1E3;}
    }

    pub fn mps_updated(&mut self) {
        self.update(self.mps, 1.0);
    }

    pub fn mmps_updated(&mut self) {
        self.update(self.mmps, 1E-3);
    }

    pub fn kps_updated(&mut self) {
        self.update(self.kps, 1E3);
    }
}

pub struct Mass {
    pub kg: f64,
    pub lunar: f64,
    pub earth: f64,
    pub jovian: f64,
    pub solar: f64,
}
#[allow(dead_code)]
impl Mass {
    pub fn from_kg(mass: f64) -> Self {
        Self {
            kg: mass,
            lunar: mass / LUNAR_MASS,
            earth: mass / EARTH_MASS,
            jovian: mass / JOVIAN_MASS,
            solar: mass / SOLAR_MASS,
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
    #[allow(dead_code)]
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
#[allow(dead_code)]
impl Mass {
    pub fn update(&mut self, from: f64, conversion: f64) {
        if from != self.kg {self.kg = from * conversion;}
        if from != self.lunar {self.lunar = self.kg / LUNAR_MASS;}
        if from != self.earth {self.earth = self.kg / EARTH_MASS;}
        if from != self.jovian {self.jovian = self.kg / JOVIAN_MASS;}
        if from != self.solar {self.solar = self.kg / SOLAR_MASS;}
    }

    pub fn kg_updated(&mut self) {
        self.update(self.kg, 1.0);
    }

    pub fn lunar_updated(&mut self) {
        self.update(self.lunar, LUNAR_MASS);
    }

    pub fn earth_updated(&mut self) {
        self.update(self.earth, EARTH_MASS);
    }

    pub fn jovian_updated(&mut self) {
        self.update(self.jovian, JOVIAN_MASS);
    }

    pub fn solar_updated(&mut self) {
        self.update(self.solar, SOLAR_MASS);
    }
}

#[derive(Debug, PartialEq)]
pub struct SemiMajorAxis {
    pub m: f64,
    pub km: f64,
    pub au: f64,
}

impl SemiMajorAxis {
    pub fn from_m(sma: f64) -> Self {
        Self {
            m: sma,
            km: sma / KM_METERS,
            au: sma / AU_METERS,
        }
    }
    #[allow(dead_code)]
    pub fn from_km(sma: f64) -> Self {
        Self {
            km: sma,
            ..Self::from_m(sma * KM_METERS)
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
    pub fn update(&mut self, from: f64, conversion: f64) {
        if from != self.m {self.m = from * conversion;}
        if from != self.km {self.km = self.m / KM_METERS;}
        if from != self.au {self.au = self.m / AU_METERS;}
    }

    pub fn m_updated(&mut self) {
        self.update(self.m, 1.0);
    }

    pub fn km_updated(&mut self) {
        self.update(self.km, KM_METERS);
    }

    pub fn au_updated(&mut self) {
        self.update(self.au, AU_METERS);
    }
}