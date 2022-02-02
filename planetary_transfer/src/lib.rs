use std::f64::consts::{TAU, PI};

pub mod quantities;
pub mod calculus;
pub use calculus::*;
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
    sma: Distance,
    parent: Parent,
}

impl Planet {
    pub fn new(sma: Distance, parent: Parent) -> Self {
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

    pub fn orbital_velocity(&self) -> Velocity {
        Velocity::from_meters_per_second((self.parent.mass.gravitational_parameter / self.sma.m).sqrt())
    }
}

pub struct Transfer {
    origin: Planet,
    target: Planet,
    parent: Parent,
    add_delta_v: Velocity,
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
            add_delta_v: Velocity::from_meters_per_second(0.0),
        }
    }

    pub fn origin(&self) -> Planet {
        self.origin
    }

    pub fn target(&self) -> Planet {
        self.target
    }

    pub fn velocity_hohmann(&self) -> Velocity {
        self.origin.orbital_velocity() * ((2.0 * self.target.sma.m) / (self.origin.sma.m + self.target.sma.m)).sqrt()
    }

    pub fn delta_v_hohmann(&self) -> Velocity {
        self.velocity_hohmann() - self.origin.orbital_velocity()
    }

    pub fn set_delta_v(&mut self, delta_v: Velocity) {
        self.add_delta_v = delta_v - self.delta_v_hohmann();
    }

    pub fn launch_velocity(&self) -> Velocity {
        self.velocity_hohmann() + self.add_delta_v
    }

    pub fn sma(&self) -> f64 {
        (self.origin.sma.m * self.parent.mass.gravitational_parameter) / (2.0 * self.parent.mass.gravitational_parameter - self.origin.sma.m * self.launch_velocity().mps.powi(2))
    }

    pub fn eccentricity(&self) -> f64 {
        1.0 - self.origin.sma.m / self.sma()
    }

    pub fn time_of_flight(&self) -> Duration {
        if self.eccentricity().abs() < 1.0 {
            let e = ((self.eccentricity() + self.target_true_anomaly().cos()) / (1.0 + self.eccentricity() * self.target_true_anomaly().cos())).acos();
            Duration::from_seconds((e - (self.eccentricity()) * e.sin()) * ((self.sma().powi(3)) / self.parent.mass.gravitational_parameter).sqrt())
        } else {
            let h = ((self.eccentricity() + self.target_true_anomaly().cos()) / (1.0 + self.eccentricity() * self.target_true_anomaly().cos())).acosh();
            Duration::from_seconds(-(h - (self.eccentricity()) * h.sinh()) * ((-self.sma().powi(3)) / self.parent.mass.gravitational_parameter).sqrt())
        }
    }

    pub fn phase(&self) -> f64 {
        let angle = self.target_true_anomaly() - (2.0 * PI * self.time_of_flight().s / self.target.period()) % TAU;
        (angle + 3.0 * PI) % (2.0 * PI) - PI
    }

    pub fn origin_true_anomaly(&self) -> f64 {
        (2.0 * PI * self.time_of_flight().s / self.origin.period()) % TAU
    }

    pub fn target_true_anomaly(&self) -> f64 {
        round_to((((self.sma() * (1.0 - self.eccentricity().powi(2))) / self.target.sma.m) - 1.0) / (self.eccentricity()), 5).acos()
    }

    pub fn min_velocity(&self) -> Velocity {
        self.delta_v_hohmann()
    }

    pub fn max_velocity(&self) -> Velocity {
        if self.origin.sma.au < self.target.sma.au {
            self.delta_v_hohmann() + self.velocity_hohmann() * 0.6
        } else {
            self.delta_v_hohmann() - self.velocity_hohmann()
        }
    }
}

//Make this a trait
pub fn round_to(value: f64, decimal: usize) -> f64 {
    (value * (10 as f64).powi(decimal as i32)).round() / (10 as f64).powi(decimal as i32)
}