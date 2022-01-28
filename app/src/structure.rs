use egui::{remap, Color32};
use egui::plot::{Line, Value, Values, LineStyle, Points, MarkerShape, Text};

use std::f64::consts::{TAU, PI};
use std::ops::RangeInclusive;

const LUNAR_MASS: f64 = 7.34767309E22;
const EARTH_MASS: f64 = 5.9722E24;
const JOVIAN_MASS: f64 = 1.89813E27;
const SOLAR_MASS: f64 = 1.98847E30;
const AU_METERS: f64 = 149598023E3;
const KM_METERS: f64 = 1E3;

#[derive(Debug, PartialEq)]
pub struct SValue {
    pub value: f64,
    pub string: String,
}

impl SValue {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            string: value.to_string(),
        }
    }

    pub fn set(&mut self, value: f64) {
        self.value = value;
        self.string = value.to_string();
    }
}

pub struct Velocity {
    pub mmps: SValue,
    pub mps: SValue,
    pub kps: SValue,
}
#[allow(dead_code)]
impl Velocity {
    pub fn from_mps(velocity: f64) -> Self {
        Self {
            mmps: SValue::new(velocity / 1E-3),
            mps: SValue::new(velocity),
            kps: SValue::new(velocity / 1E3),
        }
    }

    pub fn from_mmps(velocity: f64) -> Self {
        Self {
            mmps: SValue::new(velocity),
            ..Self::from_mps(velocity * 1E-3)
        }
    }

    pub fn from_kps(velocity: f64) -> Self {
        Self {
            kps: SValue::new(velocity),
            ..Self::from_mps(velocity * 1E3)
        }
    }
}

impl Velocity {
    pub fn update(&mut self, from: f64, conversion: f64) {
        if from != self.mps.value {self.mps = SValue::new(from * conversion);}
        if from != self.mmps.value {self.mmps = SValue::new(self.mps.value / 1E-3);};
        if from != self.kps.value {self.kps = SValue::new(self.mps.value / 1E3);}
    }

    pub fn mps_updated(&mut self) {
        self.update(self.mps.value, 1.0);
    }

    pub fn mmps_updated(&mut self) {
        self.update(self.mmps.value, 1E-3);
    }

    pub fn kps_updated(&mut self) {
        self.update(self.kps.value, 1E3);
    }
}

pub struct Mass {
    pub kg: SValue,
    pub lunar: SValue,
    pub earth: SValue,
    pub jovian: SValue,
    pub solar: SValue,
}
#[allow(dead_code)]
impl Mass {
    pub fn from_kg(mass: f64) -> Self {
        Self {
            kg: SValue::new(mass),
            lunar: SValue::new(mass / LUNAR_MASS),
            earth: SValue::new(mass / EARTH_MASS),
            jovian: SValue::new(mass / JOVIAN_MASS),
            solar: SValue::new(mass / SOLAR_MASS),
        }
    }

    pub fn from_lunar(mass: f64) -> Self {
        Self {
            lunar: SValue::new(mass),
            ..Self::from_kg(mass * LUNAR_MASS)
        }
    }
    pub fn from_earth(mass: f64) -> Self {
        Self {
            earth: SValue::new(mass),
            ..Self::from_kg(mass * EARTH_MASS)
        }
    }
    #[allow(dead_code)]
    pub fn from_jovian(mass: f64) -> Self {
        Self {
            jovian: SValue::new(mass),
            ..Self::from_kg(mass * JOVIAN_MASS)
        }
    }

    pub fn from_solar(mass: f64) -> Self {
        Self {
            solar: SValue::new(mass),
            ..Self::from_kg(mass * SOLAR_MASS)
        }
    }
}
#[allow(dead_code)]
impl Mass {
    pub fn update(&mut self, from: f64, conversion: f64) {
        if from != self.kg.value {self.kg = SValue::new(from * conversion);}
        if from != self.lunar.value {self.lunar = SValue::new(round_to(self.kg.value / LUNAR_MASS, 2));}
        if from != self.earth.value {self.earth = SValue::new(round_to(self.kg.value / EARTH_MASS, 2));}
        if from != self.jovian.value {self.jovian = SValue::new(round_to(self.kg.value / JOVIAN_MASS, 2));}
        if from != self.solar.value {self.solar = SValue::new(round_to(self.kg.value / SOLAR_MASS, 2));}
    }

    pub fn kg_updated(&mut self) {
        self.update(self.kg.value, 1.0);
    }

    pub fn lunar_updated(&mut self) {
        self.update(self.lunar.value, LUNAR_MASS);
    }

    pub fn earth_updated(&mut self) {
        self.update(self.earth.value, EARTH_MASS);
    }

    pub fn jovian_updated(&mut self) {
        self.update(self.jovian.value, JOVIAN_MASS);
    }

    pub fn solar_updated(&mut self) {
        self.update(self.solar.value, SOLAR_MASS);
    }
}

#[derive(Debug, PartialEq)]
pub struct SemiMajorAxis {
    pub m: SValue,
    pub km: SValue,
    pub au: SValue,
}

impl SemiMajorAxis {
    pub fn from_m(sma: f64) -> Self {
        Self {
            m: SValue::new(sma),
            km: SValue::new(sma / KM_METERS),
            au: SValue::new(sma / AU_METERS),
        }
    }
    #[allow(dead_code)]
    pub fn from_km(sma: f64) -> Self {
        Self {
            km: SValue::new(sma),
            ..Self::from_m(sma * KM_METERS)
        }
    }

    pub fn from_au(sma: f64) -> Self {
        Self {
            au: SValue::new(sma),
            ..Self::from_m(sma * AU_METERS)
        }
    }
}

impl SemiMajorAxis {
    pub fn update(&mut self, from: f64, conversion: f64) {
        if from != self.m.value {self.m = SValue::new(from * conversion);}
        if from != self.km.value {self.km = SValue::new(self.m.value / KM_METERS);}
        if from != self.au.value {self.au = SValue::new(round_to(self.m.value / AU_METERS, 2));}
    }

    pub fn m_updated(&mut self) {
        self.update(self.m.value, 1.0);
    }

    pub fn km_updated(&mut self) {
        self.update(self.km.value, KM_METERS);
    }

    pub fn au_updated(&mut self) {
        self.update(self.au.value, AU_METERS);
    }
}

pub struct Parent {
    gravitational_parameter: f64,
}

impl Parent {
    pub fn new(mass: f64) -> Self {
        Parent {
            gravitational_parameter: mass * 6.67430E-11,
        }
    }
}

pub struct Planet<'a> {
    sma: f64,
    parent: &'a Parent,
}

impl<'a> Planet<'a> {
    pub fn new(sma: f64, parent: &'a Parent) -> Self {
        Planet {
            sma,
            parent,
        }
    }

    pub fn period(&self) -> f64 {
        2.0 * PI * (self.sma.powi(3) / self.parent.gravitational_parameter).sqrt()
    }

    pub fn orbital_velocity(&self) -> f64 {
        (self.parent.gravitational_parameter / self.sma).sqrt()
    }

    pub fn marker(&self, angle: f64) -> Points {
        let coord = Value::new(
            self.sma * angle.cos() as f64,
            self.sma * angle.sin() as f64,
        );

        Points::new(Values::from_values(vec![coord]))
            .radius(10.0)
    }
}

impl OrbitPlot for Planet<'_> {
    fn sma(&self) -> f64 {
        self.sma
    }

    fn eccentricity(&self) -> f64 {
        0.0
    }
}

impl Marker for Planet<'_> {
    fn sma(&self) -> f64 {
        self.sma
    }
}

pub struct Transfer<'a> {
    origin: &'a Planet<'a>,
    target: &'a Planet<'a>,
    parent: &'a Parent,
    velocity: f64,
    velocity_hohmann: f64,
}

impl<'a> Transfer<'a> {
    pub fn new(origin: &'a Planet, target: &'a Planet) -> Transfer<'a> {
        let velocity_hohmann = origin.orbital_velocity() * ((2.0 * target.sma) / (origin.sma + target.sma)).sqrt();
        Transfer {
            origin,
            target,
            parent: if origin.parent.gravitational_parameter == target.parent.gravitational_parameter {
                        origin.parent
                    } else {
                        panic!("Different parents!")
                    },
            velocity: velocity_hohmann,
            velocity_hohmann,
        }
    }

    pub fn set_delta_v(&mut self, delta_v: f64) {
        self.velocity = delta_v + self.origin.orbital_velocity();
    }

    pub fn delta_v_hohmann(&self) -> f64 {
        self.velocity_hohmann - self.origin.orbital_velocity()
    }

    pub fn eccentricity(&self) -> f64 {
        1.0 - self.origin.sma * self.velocity.powi(2) / self.parent.gravitational_parameter
    }

    pub fn sma(&self) -> f64 {
        (self.origin.sma * self.parent.gravitational_parameter) / (2.0 * self.parent.gravitational_parameter - self.origin.sma * self.velocity.powi(2))
    }

    pub fn time_of_flight(&self) -> f64 {
        if 1.0 - self.eccentricity().abs() > 0.0 {
            let e = round_to((self.target.sma - self.sma()) / (self.sma() * self.eccentricity()), 5).acos();
            (e - (self.eccentricity()).abs() * e.sin()) * ((self.sma().abs().powi(3)) / self.parent.gravitational_parameter).sqrt()
        } else {
            let h = -round_to((self.target.sma - self.sma()) / (self.sma() * self.eccentricity()), 5).acosh();
            (h - (self.eccentricity()).abs() * h.sinh()) * ((self.sma().abs().powi(3)) / self.parent.gravitational_parameter).sqrt()
        }
    }

    pub fn phase(&self) -> f64 {
        (self.true_anomaly() - (2.0 * PI * self.time_of_flight() / self.target.period())) % TAU
    }

    pub fn idk_yet(&self) -> f64 {
        (2.0 * PI * self.time_of_flight() / self.origin.period()) % TAU
    }

    pub fn true_anomaly(&self) -> f64 {
        round_to((self.target.sma - self.sma() * (1.0 - self.eccentricity().powi(2))) / (self.eccentricity() * self.target.sma), 5).acos()
    }
}

impl OrbitPlot for Transfer<'_> {
    fn sma(&self) -> f64 {
        self.sma()
    }

    fn eccentricity(&self) -> f64 {
        self.eccentricity()
    }

    fn range(&self) -> RangeInclusive<f64> {
        0.0..=self.true_anomaly()
    }
}

pub trait OrbitPlot {
    fn sma(&self) -> f64;

    fn eccentricity(&self) -> f64;

    fn range(&self) -> RangeInclusive<f64> {
        0.0..=TAU
    }

    fn plot(&self) -> Line {
        let n = 512;
        let orbit = (0..=n).map(|i| {

            let theta = remap(i as f64, 0.0..=(n as f64), self.range());
            let equation = self.sma() * (1.0 - self.eccentricity().powi(2)) / (1.0 - self.eccentricity() * theta.cos());

            Value::new(
                equation * theta.cos(),
                equation * theta.sin(),
            )});

        Line::new(Values::from_values_iter(orbit))
            //.color(color)
            .style(LineStyle::Solid)
    }
}

trait Marker {
    fn sma(&self) -> f64;

    fn marker(&self, angle: f64) -> Points {
        let coord = Value::new(
            self.sma() * angle.cos() as f64,
            self.sma() * angle.sin() as f64,
        );
        Points::new(Values::from_values(vec![coord]))
    }
}

pub struct TransferPlot<'a> {
    transfer: &'a Transfer<'a>,
    color_origin: Color32,
    color_target: Color32,
    width_origin: f32,
    width_target: f32,
    width_transfer: f32,
}

impl<'a> TransferPlot<'a> {
    pub fn new(transfer: &'a Transfer<'a>) -> Self {
        Self {
            transfer,
            color_origin: Color32::WHITE,
            color_target: Color32::WHITE,
            width_origin: 1.0,
            width_target: 1.0,
            width_transfer: 1.0,
        }
    }

    pub fn orbit_all(&self) -> Vec<Line> {
        let mut orbits = Vec::new();
        orbits.push(self.orbit_origin());
        orbits.push(self.orbit_target());
        orbits.push(self.orbit_transfer());
        orbits
    }

    pub fn marker_all(&self) ->  Vec<Points>  {
        let mut markers = Vec::new();
        markers.append(&mut self.marker_origin());
        markers.append(&mut self.marker_target());
        markers
    }

    pub fn orbit_origin(&self) -> Line {
        self.transfer.origin.plot()
            .color(self.color_origin)
            .width(self.width_origin)
    }

    pub fn orbit_target(&self) -> Line {
        self.transfer.target.plot()
            .color(self.color_target)
            .width(self.width_target)
    }

    pub fn orbit_transfer(&self) -> Line {
        self.transfer.plot()
            .color(Color32::from_rgb(255, 115, 0))
            .width(self.width_transfer)
    }

    pub fn marker_origin(&self) -> Vec<Points> {
        vec![
        self.transfer.origin.marker(0.0), 
        self.transfer.origin.marker(self.transfer.idk_yet())
        ]
    }

    pub fn marker_target(&self) -> Vec<Points> {
        vec![
        self.transfer.target.marker(self.transfer.phase()), 
        self.transfer.target.marker(self.transfer.true_anomaly())
        ]
    }

    pub fn set_color_origin(&mut self, color: Color32) {
        self.color_origin = color;
    }

    pub fn set_color_target(&mut self, color: Color32) {
        self.color_target = color;
    }

    pub fn highlight_origin(&mut self) {
        self.width_origin = 2.0;
    }

    pub fn highlight_target(&mut self) {
        self.width_target = 2.0;
    }

    pub fn highlight_transfer(&mut self) {
        self.width_transfer = 2.0;
    }
}

pub struct AngleMeasurer {
    angle: f64,
    length: f64,
    color: Color32,
    style: LineStyle,
    width: f32,
    protrusion: f64,
}

impl AngleMeasurer {
    pub fn new(angle: f64, length: f64) -> Self {
        Self {
            angle: (angle + 3.0 * PI) % (2.0 * PI) - PI,
            length,
            color: Color32::WHITE,
            style: LineStyle::dashed_loose(),
            width: 3.0,
            protrusion: 1.05,
        }
    }

    pub fn plot(&self) -> Vec<Line> {

        let angle = self.angle;
        let base_length = self.protrusion * self.length * self.angle.cos();

        let x_axis = Line::new(Values::from_explicit_callback(|_x| 0.0, 0.0..self.protrusion * self.length, 2))
            .color(self.color)
            .style(self.style)
            .width(self.width);

        let hyp = Line::new(Values::from_explicit_callback(move |x| x * angle.tan(), base_length.min(0.0)..base_length.max(0.0), 2))
            .color(self.color)
            .style(self.style)
            .width(self.width);

        let n = 512;
        let orbit = (0..=n).map(|i| {

            let theta = remap(i as f64, 0.0..=(n as f64), 0.0..=angle);
            Value::new(
                self.length * theta.cos(),
                self.length * theta.sin(),
            )});

        let measure = Line::new(Values::from_values_iter(orbit))
            .color(self.color)
            .style(self.style)
            .width(self.width);

        vec![hyp, x_axis, measure]
    }

    pub fn text(&self) -> Text {
        Text::new(
            Value::new(
                1.1 * self.length * (self.angle / 2.0).cos(), 
                1.1 * self.length * (self.angle / 2.0).sin()),
            format!("{} °", round_to(self.angle.to_degrees(), 2)).to_string()
        )
        .style(egui::TextStyle::Heading)
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

pub fn round_to(value: f64, decimal: i32) -> f64 {
    (value * (10 as f64).powi(decimal)).round() / (10 as f64).powi(decimal)
}