use std::{ops::RangeInclusive, f64::consts::TAU};

use egui::{plot::{Line, Value, Values, Points, LineStyle, Text}, Color32, remap};
use planetary_transfer::{Planet, Transfer};

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
            let equation = self.sma() * (1.0 - self.eccentricity().powi(2)) / (1.0 + self.eccentricity() * theta.cos());

            Value::new(
                equation * theta.cos(),
                equation * theta.sin(),
            )});

        Line::new(Values::from_values_iter(orbit))
            .style(LineStyle::Solid)
    }
}

trait Marker {
    fn sma(&self) -> f64;

    fn marker(&self, angle: f64) -> Points {
        let coord = Value::new(
            self.sma() * angle.cos(),
            self.sma() * angle.sin(),
        );
        Points::new(Values::from_values(vec![coord]))
            .radius(10.0)
    }
}

pub struct TransferPlot<'a> {
    transfer: &'a Transfer,
    color_origin: Color32,
    color_target: Color32,
    width_origin: f32,
    width_target: f32,
    width_transfer: f32,
}

impl<'a> TransferPlot<'a> {
    pub fn new(transfer: &'a Transfer, color: Color32) -> Self {
        Self {
            transfer,
            color_origin: color,
            color_target: color,
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
        self.transfer.origin().plot()
            .color(self.color_origin)
            .width(self.width_origin)
    }

    pub fn orbit_target(&self) -> Line {
        self.transfer.target().plot()
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
        self.transfer.origin().marker(0.0), 
        self.transfer.origin().marker(self.transfer.origin_true_anomaly())
        ]
    }

    pub fn marker_target(&self) -> Vec<Points> {
        vec![
        self.transfer.target().marker(self.transfer.phase()), 
        self.transfer.target().marker(self.transfer.target_true_anomaly())
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

impl OrbitPlot for Planet {
    fn sma(&self) -> f64 {
        self.sma()
    }

    fn eccentricity(&self) -> f64 {
        0.0
    }
}

impl Marker for Planet {
    fn sma(&self) -> f64 {
        self.sma()
    }
}


impl OrbitPlot for Transfer {
    fn sma(&self) -> f64 {
        self.sma()
    }

    fn eccentricity(&self) -> f64 {
        self.eccentricity()
    }

    fn range(&self) -> RangeInclusive<f64> {
        0.0..=self.target_true_anomaly()
    }
}

pub struct Protractor {
    angle: f64,
    length: f64,
    color: Color32,
    style: LineStyle,
    width: f32,
    protrusion: f64,
}

impl Protractor {
    pub fn new(angle: f64, length: f64) -> Self {
        Self {
            angle,
            length,
            color: Color32::WHITE,
            style: LineStyle::dashed_loose(),
            width: 3.0,
            protrusion: 1.0 - 0.05,
        }
    }

    pub fn plot(&self) -> Vec<Line> {

        let angle = self.angle;
        let base_length = self.length * self.angle.cos();

        let x_axis = Line::new(Values::from_explicit_callback(|_x| 0.0, 0.0..self.length, 2))
            .color(self.color)
            .style(self.style)
            .width(self.width);

        let hypothenuse = Line::new(Values::from_explicit_callback(move |x| x * angle.tan(), base_length.min(0.0)..base_length.max(0.0), 2))
            .color(self.color)
            .style(self.style)
            .width(self.width);

        let n = 512;
        let angle = (0..=n).map(|i| {

            let theta = remap(i as f64, 0.0..=(n as f64), 0.0..=angle);
            Value::new(
                (self.length * self.protrusion) * theta.cos(),
                (self.length * self.protrusion) * theta.sin(),
            )});

        let measure = Line::new(Values::from_values_iter(angle))
            .color(self.color)
            .style(self.style)
            .width(self.width);

        vec![hypothenuse, x_axis, measure]
    }

    pub fn text(&self) -> Text {
        let text_length = (self.length * self.protrusion) * 0.9;
        let text_angle = if self.angle.abs() > 0.2 {self.angle / 2.0} else {self.angle + 0.15 * self.angle.signum()};
        Text::new(
            Value::new(
                text_length * text_angle.cos(), 
                text_length * text_angle.sin()),
            format!("{} °", round_to(self.angle.to_degrees(), 2).to_string())
        )
        .style(egui::TextStyle::Heading)
        .color(Color32::WHITE)
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