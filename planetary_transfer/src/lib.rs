use std::f64::consts::{TAU, PI};

pub mod quantities;
pub use quantities::*;

#[derive(Copy, Clone)]
pub struct Parent {
    mass: Mass,
}

impl Parent {
    pub fn new(mass: Mass) -> Self {
        Parent {
            mass,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Planet {
    sma: SemiMajorAxis,
    parent: Parent,
}

impl Planet {
    pub fn new(sma: SemiMajorAxis, parent: Parent) -> Self {
        Planet {
            sma,
            parent,
        }
    }

    pub fn sma(&self) -> f64 {
        self.sma.m
    }

    pub fn period(&self) -> f64 {
        2.0 * PI * (self.sma.m.powi(3) / self.parent.mass.gravitational_parameter).sqrt()
    }

    pub fn orbital_velocity(&self) -> f64 {
        (self.parent.mass.gravitational_parameter / self.sma.m).sqrt()
    }
}

pub struct Transfer {
    origin: Planet,
    target: Planet,
    parent: Parent,
    add_delta_v: f64,
}

impl Transfer {
    pub fn new(origin: Planet, target: Planet) -> Transfer {
        Transfer {
            origin,
            target,
            parent: if origin.parent.mass.gravitational_parameter == target.parent.mass.gravitational_parameter {
                        origin.parent
                    } else {
                        panic!("Different parents!")
                    },
            add_delta_v: 0.0,
        }
    }

    pub fn origin(&self) -> Planet {
        self.origin
    }

    pub fn target(&self) -> Planet {
        self.target
    }

    pub fn velocity_hohmann(&self) -> f64 {
        self.origin.orbital_velocity() * ((2.0 * self.target.sma.m) / (self.origin.sma.m + self.target.sma.m)).sqrt()
    }

    pub fn delta_v_hohmann(&self) -> f64 {
        self.velocity_hohmann() - self.origin.orbital_velocity()
    }

    pub fn set_delta_v(&mut self, delta_v: f64) {
        self.add_delta_v = delta_v - self.delta_v_hohmann();
    }

    pub fn launch_velocity(&self) -> f64 {
        self.velocity_hohmann() + self.add_delta_v
    }

    pub fn eccentricity(&self) -> f64 {
        1.0 - self.origin.sma.m * self.launch_velocity().powi(2) / self.parent.mass.gravitational_parameter
    }

    pub fn sma(&self) -> f64 {
        (self.origin.sma.m * self.parent.mass.gravitational_parameter) / (2.0 * self.parent.mass.gravitational_parameter - self.origin.sma.m * self.launch_velocity().powi(2))
    }

    pub fn time_of_flight(&self) -> f64 {
        if 1.0 - self.eccentricity().abs() > 0.0 {
            let e = round_to((self.target.sma.m - self.sma()) / (self.sma() * self.eccentricity()), 5).acos();
            (e - (self.eccentricity()).abs() * e.sin()) * ((self.sma().abs().powi(3)) / self.parent.mass.gravitational_parameter).sqrt()
        } else {
            let h = -round_to((self.target.sma.m - self.sma()) / (self.sma() * self.eccentricity()), 5).acosh();
            (h - (self.eccentricity()).abs() * h.sinh()) * ((self.sma().abs().powi(3)) / self.parent.mass.gravitational_parameter).sqrt()
        }
    }

    pub fn phase(&self) -> f64 {
        (self.target_true_anomaly() - (2.0 * PI * self.time_of_flight() / self.target.period())) % TAU
    }

    pub fn origin_true_anomaly(&self) -> f64 {
        (2.0 * PI * self.time_of_flight() / self.origin.period()) % TAU
    }

    pub fn target_true_anomaly(&self) -> f64 {
        round_to((self.target.sma.m - self.sma() * (1.0 - self.eccentricity().powi(2))) / (self.eccentricity() * self.target.sma.m), 5).acos()
    }

    pub fn transfer_range(&self) -> std::ops::RangeInclusive<f64> {
        if self.origin.sma.au < self.target.sma.au {
            self.delta_v_hohmann()..=round_to(self.delta_v_hohmann() + self.velocity_hohmann() * 0.6, 1)
        } else {
            self.delta_v_hohmann()..=round_to(self.delta_v_hohmann() - self.velocity_hohmann() * 0.6, 1)
        }
    }
}

pub fn round_to(value: f64, decimal: usize) -> f64 {
    (value * (10 as f64).powi(decimal as i32)).round() / (10 as f64).powi(decimal as i32)
}