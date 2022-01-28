use eframe::epi;
use egui::{TopBottomPanel, CentralPanel, Color32, Slider, Ui, Widget, Response, TextEdit, vec2, Vec2};
use egui::plot::{Plot};
use thousands::Separable;

use durations::Duration;

use crate::structure::{Parent, SemiMajorAxis, Planet, Transfer, SValue, Mass, Velocity, round_to, AngleMeasurer, TransferPlot};

struct SliderWithText<'a> {
    value: &'a mut f64,
    text: &'a mut String,
    range: std::ops::RangeInclusive<f64>,
    suffix: &'a str,
    max_decimals: usize,
    enabled_slider: bool,
}

impl<'a> SliderWithText<'a> {
    fn new(value: &'a mut SValue, range: std::ops::RangeInclusive<f64>) -> Self {
        Self {
            value: &mut value.value,
            text: &mut value.string,
            range,
            suffix: "",
            max_decimals: 10,
            enabled_slider: true,
        }
    }
}

impl<'a> Widget for SliderWithText<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let inner = ui.horizontal(|ui| self.add_widgets(ui));
        
        inner.inner
    }
}

impl<'a> SliderWithText<'a> {
    fn slider(&mut self) -> Slider {
        Slider::new(self.value, self.range.clone())
            .show_value(false)
            .logarithmic(true)
            .max_decimals(self.max_decimals)
    }
    
    fn value(&mut self) -> TextEdit {
        TextEdit::singleline(self.text)
    }

    fn add_widgets(mut self, ui: &mut Ui) -> Response {
        let slider_response = ui.add_enabled(self.enabled_slider, self.slider());
        let value_response = ui.add(self.value());

        if slider_response.dragged() {
            ui.ctx().memory().stop_text_input();
        }

        if value_response.gained_focus() {
            *self.text = self.value.separate_with_commas();

        } else if !value_response.has_focus() {
            if value_response.lost_focus() {
                let text_input = self.text.trim().replace(',', "").parse::<f64>().unwrap_or(*self.value);
                let start = *self.range.start();
                let end = *self.range.end();
                *self.value = text_input.clamp(start.min(end), start.max(end));
            }
            *self.text = (round_to(*self.value, 4)).separate_with_commas();
            self.text.push_str(self.suffix);
        }
        
        value_response | slider_response
        
    }
}

impl<'a> SliderWithText<'a> {
    fn suffix(mut self, suffix: &'a str) -> Self {
        self.suffix = suffix;
        self
    }

    fn max_decimals(mut self, max_decimals: usize) -> Self {
        self.max_decimals = max_decimals;
        self
    }

    fn enabled_slider(mut self, enabled_slider: bool) -> Self {
        self.enabled_slider = enabled_slider;
        self
    }
}

pub struct Gui {
    origin_sma: SemiMajorAxis,
    target_sma: SemiMajorAxis,
    mass: Mass,
    velocity: Velocity,
    hohmann: bool,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            origin_sma: SemiMajorAxis::from_au(1.0),
            target_sma: SemiMajorAxis::from_au(1.52366),
            mass: Mass::from_solar(1.0),
            velocity: Velocity::from_kps(3.0),
            hohmann: true,
        }
    }
}

impl epi::App for Gui {
    fn max_size_points(&self) -> Vec2 {
        Vec2::new(f32::MAX, f32::MAX)
    }
    
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &eframe::epi::Frame) {

        let plot_bounds = self.origin_sma.m.value.max(self.target_sma.m.value) * 1.1;

        //Create the parent
        let parent = Parent::new(self.mass.kg.value);
            
        //Create the two planet used for the transfer
        let origin = Planet::new(self.origin_sma.m.value, &parent);
        let target = Planet::new(self.target_sma.m.value, &parent);

        //Create a transfer with the two previously created planets
        let mut transfer = Transfer::new(&origin, &target);
        if !self.hohmann {transfer.set_delta_v(self.velocity.mps.value)}

        //Orbits of the planets and their markers at departure and arrival and the transfer orbit
        let mut transfer_plot = TransferPlot::new(&transfer);

        //Angle measurer
        let angle_measurer = AngleMeasurer::new(transfer.phase(), plot_bounds * 0.96)
            .color(Color32::GRAY);

        TopBottomPanel::top("my_panel")
            .show(ctx, |ui| {
            
            ui.spacing_mut().slider_width = ui.available_width() - 110.0;
            
            ui.add_enabled_ui(self.hohmann, |ui| {

                ui.add_space(5.0);

                ui.label("Semi-major axis of the origin body:");

                let sma_min = SemiMajorAxis::from_km(10.0);
                let sma_max = SemiMajorAxis::from_au(50.0);

                if self.origin_sma.km.value > 7_500_000.0 {
                    let slider = ui.add(SliderWithText::new(&mut self.origin_sma.au, sma_min.au.value..=sma_max.au.value)
                        .suffix(" au")
                    );
                    self.origin_sma.au_updated();
                    if slider.hovered() {transfer_plot.highlight_origin()}
                    if slider.dragged() | slider.has_focus() {transfer_plot.set_color_origin(Color32::RED)}

                } else if self.origin_sma.m.value > 100_000.0 {
                    let slider = ui.add(SliderWithText::new(&mut self.origin_sma.km, sma_min.km.value..=sma_max.km.value)
                        .suffix(" km")
                    );
                    self.origin_sma.km_updated();
                    if slider.hovered() {transfer_plot.highlight_origin()}
                    if slider.dragged() | slider.has_focus() {transfer_plot.set_color_origin(Color32::RED)}
                } else {
                    let slider = ui.add(SliderWithText::new(&mut self.origin_sma.m, sma_min.m.value..=sma_max.m.value)
                        .suffix(" m")
                    );
                    self.origin_sma.m_updated();
                    if slider.hovered() {transfer_plot.highlight_origin()}
                    if slider.dragged() | slider.has_focus() {transfer_plot.set_color_origin(Color32::RED)}
                }

                ui.add_space(5.0);

                ui.label("Semi-major axis of the target body:");

                if self.target_sma.km.value > 7500000.0 {
                    let slider = ui.add(SliderWithText::new(&mut self.target_sma.au, sma_min.au.value..=sma_max.au.value)
                        .suffix(" au")
                    );
                    self.target_sma.au_updated();
                    if slider.hovered() {transfer_plot.highlight_target()}
                    if slider.dragged() | slider.has_focus() {transfer_plot.set_color_target(Color32::RED)}

                } else if self.target_sma.m.value > 100_000.0 {
                    let slider = ui.add(SliderWithText::new(&mut self.target_sma.km, sma_min.km.value..=sma_max.km.value)
                        .suffix(" km")
                    );
                    self.target_sma.km_updated();
                    if slider.hovered() {transfer_plot.highlight_target()}
                    if slider.dragged() | slider.has_focus() {transfer_plot.set_color_target(Color32::RED)}
                } else {
                    let slider = ui.add(SliderWithText::new(&mut self.target_sma.m, sma_min.m.value..=sma_max.m.value)
                        .suffix(" m")
                    );
                    self.target_sma.m_updated();
                    if slider.hovered() {transfer_plot.highlight_target()}
                    if slider.dragged() | slider.has_focus() {transfer_plot.set_color_target(Color32::RED)}
                }

                ui.add_space(5.0);

                ui.label("Mass of the parent body:");

                let mass_min = Mass::from_lunar(0.05);
                let mass_max = Mass::from_solar(100.0);

                if self.mass.jovian.value > 100.0 {
                    ui.add(SliderWithText::new(&mut self.mass.solar, mass_min.solar.value..=mass_max.solar.value)
                        .suffix(" M☉")
                    );
                    self.mass.solar_updated();
                    
                } else if self.mass.earth.value > 35.0 {
                    ui.add(SliderWithText::new(&mut self.mass.jovian, mass_min.jovian.value..=mass_max.jovian.value)
                        .suffix(" Mj")
                    );
                    self.mass.jovian_updated();

                } else if self.mass.lunar.value > 8.0 {
                    ui.add(SliderWithText::new(&mut self.mass.earth, mass_min.earth.value..=mass_max.earth.value)
                        .suffix(" Me")
                    );
                    self.mass.earth_updated();
                } else {
                    ui.add(SliderWithText::new(&mut self.mass.lunar, mass_min.lunar.value..=mass_max.lunar.value)
                        .suffix(" Ml")
                    );
                    self.mass.lunar_updated(); 
                }
            });

            ui.add_space(5.0);

            ui.checkbox(&mut self.hohmann, "Hohmann");

            let min = Velocity::from_mps(transfer.delta_v_hohmann());
            let max = Velocity::from_mps(transfer.delta_v_hohmann().signum() * origin.orbital_velocity());

            if self.velocity.mps.value.abs() > 1000.0 {
                if self.hohmann {self.velocity.kps.value = min.kps.value;}
                let slider = ui.add(SliderWithText::new(&mut self.velocity.kps, min.kps.value..=max.kps.value)
                    .suffix(" km/s")
                    .max_decimals(20)
                    .enabled_slider(!self.hohmann)
                );
                self.velocity.kps_updated();
                if slider.hovered() {transfer_plot.highlight_transfer()}
                if slider.dragged() | slider.has_focus() {transfer_plot.highlight_transfer()}
                
            } else if self.velocity.mmps.value.abs() > 1000.0 {
                if self.hohmann {self.velocity.mps.value = min.mps.value;}
                let slider = ui.add(SliderWithText::new(&mut self.velocity.mps, min.mps.value..=max.mps.value)
                    .suffix(" m/s")
                    .max_decimals(20)
                    .enabled_slider(!self.hohmann)
                );
                self.velocity.mps_updated();
                if slider.hovered() {transfer_plot.highlight_transfer()}
                if slider.dragged() | slider.has_focus() {transfer_plot.highlight_transfer()}

            } else {
                if self.hohmann {self.velocity.mmps.value = min.mmps.value;}
                let slider = ui.add(SliderWithText::new(&mut self.velocity.mmps, min.mmps.value..=max.mmps.value)
                    .suffix(" mm/s")
                    .max_decimals(20)
                    .enabled_slider(!self.hohmann)
                );
                self.velocity.mmps_updated();
                if slider.hovered() {transfer_plot.highlight_transfer()}
                if slider.dragged() | slider.has_focus() {transfer_plot.highlight_transfer()}
            }

            ui.add_space(5.0);
        });

        CentralPanel::default().show(ctx, |ui| {

            let transfer_time = Duration::from_seconds(transfer.time_of_flight())
                .smallest_duration()
                .round_to(2)
                .as_string();
    
            ui.label(format!("Transfer will take: {}", transfer_time));

            let transfer_orbits = transfer_plot.orbit_all();
            let transfer_markers = transfer_plot.marker_all();
            
            Plot::new("my_plot")
            .allow_zoom(true)
            .allow_drag(false)
            .show_background(false)
            .show_axes([false; 2])
            .show_x(false).show_y(false)
            .data_aspect(1.0)
            .center_x_axis(true)
            .center_y_axis(true)
            //.include_x(plot_bounds)
            .include_y(plot_bounds)

            .show(ui, |plot_ui| {
                for orbits in transfer_orbits {
                    plot_ui.line(orbits);
                }
                for plots in angle_measurer.plot() {
                    plot_ui.line(plots);
                }
                plot_ui.text(angle_measurer.text());
                for markers in transfer_markers {
                    plot_ui.points(markers);
                }
                plot_ui.points(egui::plot::Points::new(egui::plot::Values::from_values(vec![egui::plot::Value::new(0.0, 0.0)]))
                            .radius(((self.origin_sma.m.value / self.target_sma.m.value).min(self.target_sma.m.value / self.origin_sma.m.value) as f32) * 20.0)
                            .shape(egui::plot::MarkerShape::Diamond)
                        );
            });
        });
    }

    fn name(&self) -> &str {
        "Planetary transfer calculator"
    }
}