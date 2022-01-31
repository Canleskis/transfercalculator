use eframe::epi;
use egui::{TopBottomPanel, CentralPanel, Color32, Vec2};
use egui::plot::Plot;

use planetary_transfer::{Mass, Distance, Velocity, Parent, Planet, Transfer};

use crate::widgets::SliderWithText;
use crate::plotting::{Protractor, TransferPlot, round_to};

pub struct Gui {
    origin_sma: Distance,
    target_sma: Distance,
    mass: Mass,
    velocity: Velocity,
    hohmann: bool,

    origin_sma_text: String,
    target_sma_text: String,
    mass_text: String,
    velocity_text: String,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            origin_sma: Distance::from_astronomical_unit(1.0),
            target_sma: Distance::from_astronomical_unit(1.52366),
            mass: Mass::from_solar(1.0),
            velocity: Velocity::from_kilometers_per_second(3.0),
            hohmann: true,

            origin_sma_text: "".to_string(),
            target_sma_text: "".to_string(),
            mass_text: "".to_string(),
            velocity_text: "".to_string(),
        }
    }
}

impl epi::App for Gui {
    fn max_size_points(&self) -> Vec2 {
        Vec2::new(f32::MAX, f32::MAX)
    }
    
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &eframe::epi::Frame) {

        let portrait = ctx.input().screen_rect.aspect_ratio() <= 0.6;

        let color_mode = if *&ctx.style().visuals.dark_mode {Color32::WHITE} else {Color32::BLACK};

        let plot_bounds = self.origin_sma.m.max(self.target_sma.m);

        //Create the parent
        let parent = Parent::new(self.mass);
            
        //Create the two planet used for the transfer
        let origin = Planet::new(self.origin_sma, parent);
        let target = Planet::new(self.target_sma, parent);

        //Create a transfer with the two previously created planets
        let mut transfer = Transfer::new(origin, target);
        if self.hohmann {self.velocity = transfer.min_velocity()};
        transfer.set_delta_v(self.velocity);

        //let min = transfer.min_velocity();
        let min = Velocity::from_meters_per_second(0.0);
        let max = transfer.max_velocity();
        
        //Orbits of the planets and their markers at departure and arrival and the transfer orbit
        let mut transfer_plot = TransferPlot::new(&transfer, color_mode);

        //Angle measurer
        let protractor = Protractor::new(transfer.phase(), plot_bounds * 0.96)
            .color(Color32::GRAY);

        if portrait {
            TopBottomPanel::bottom("bottom")
        } else {
            TopBottomPanel::top("top")
        }
            .show(ctx, |ui| {
            
            ui.spacing_mut().slider_width = ui.available_width() - 110.0;

            ui.add_enabled_ui(self.hohmann, |ui| {

                ui.add_space(5.0);

                ui.label("Semi-major axis of the origin body:");

                let sma_min = Distance::from_kilometers(10.0);
                let sma_max = Distance::from_astronomical_unit(50.0);

                if self.origin_sma.km > 7_500_000.0 {
                    let slider = ui.add(SliderWithText::new(
                        &mut self.origin_sma.au, &mut self.origin_sma_text, 
                        sma_min.au..=sma_max.au
                    )
                        .suffix(" au")
                    );
                    self.origin_sma.au_updated();
                    if slider.hovered() {transfer_plot.highlight_origin()}
                    if slider.dragged() | slider.has_focus() {transfer_plot.set_color_origin(Color32::RED)}

                } else if self.origin_sma.m > 100_000.0 {
                    let slider = ui.add(SliderWithText::new(
                        &mut self.origin_sma.km, &mut self.origin_sma_text, 
                        sma_min.km..=sma_max.km
                    )
                        .suffix(" km")
                    );
                    self.origin_sma.km_updated();
                    if slider.hovered() {transfer_plot.highlight_origin()}
                    if slider.dragged() | slider.has_focus() {transfer_plot.set_color_origin(Color32::RED)}
                } else {
                    let slider = ui.add(SliderWithText::new(
                        &mut self.origin_sma.m, &mut self.origin_sma_text, 
                        sma_min.m..=sma_max.m
                    )
                        .suffix(" m")
                    );
                    self.origin_sma.m_updated();
                    if slider.hovered() {transfer_plot.highlight_origin()}
                    if slider.dragged() | slider.has_focus() {transfer_plot.set_color_origin(Color32::RED)}
                }

                ui.add_space(5.0);

                ui.label("Semi-major axis of the target body:");

                if self.target_sma.km > 7500000.0 {
                    let slider = ui.add(SliderWithText::new(
                        &mut self.target_sma.au, &mut self.target_sma_text, 
                        sma_min.au..=sma_max.au
                    )
                        .suffix(" au")
                    );
                    self.target_sma.au_updated();
                    if slider.hovered() {transfer_plot.highlight_target()}
                    if slider.dragged() | slider.has_focus() {transfer_plot.set_color_target(Color32::RED)}

                } else if self.target_sma.m > 100_000.0 {
                    let slider = ui.add(SliderWithText::new(
                        &mut self.target_sma.km, &mut self.target_sma_text, 
                        sma_min.km..=sma_max.km
                    )
                        .suffix(" km")
                    );
                    self.target_sma.km_updated();
                    if slider.hovered() {transfer_plot.highlight_target()}
                    if slider.dragged() | slider.has_focus() {transfer_plot.set_color_target(Color32::RED)}
                } else {
                    let slider = ui.add(SliderWithText::new(
                        &mut self.target_sma.m, &mut self.target_sma_text, 
                        sma_min.m..=sma_max.m
                    )
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

                if self.mass.jovian > 97.0 {
                    ui.add(SliderWithText::new(&mut self.mass.solar, &mut self.mass_text, mass_min.solar..=mass_max.solar)
                        .suffix(" M☉")
                    );
                    self.mass.solar_updated();
                    
                } else if self.mass.earth > 35.0 {
                    ui.add(SliderWithText::new(&mut self.mass.jovian, &mut self.mass_text, mass_min.jovian..=mass_max.jovian)
                        .suffix(" Mj")
                    );
                    self.mass.jovian_updated();

                } else if self.mass.lunar > 8.0 {
                    ui.add(SliderWithText::new(&mut self.mass.earth, &mut self.mass_text, mass_min.earth..=mass_max.earth)
                        .suffix(" Me")
                    );
                    self.mass.earth_updated();
                } else {
                    ui.add(SliderWithText::new(&mut self.mass.lunar, &mut self.mass_text, mass_min.lunar..=mass_max.lunar)
                        .suffix(" Ml")
                    );
                    self.mass.lunar_updated(); 
                }
            });

            ui.add_space(5.0);

            ui.checkbox(&mut self.hohmann, "Hohmann");

            if self.velocity.mps.abs() >= 1000.0 {
                let slider = ui.add(SliderWithText::new(
                    &mut self.velocity.kps, &mut self.velocity_text,
                    min.kps..=max.kps
                )
                    .suffix(" km/s")
                    .max_decimals(14)
                    .enabled_slider(!self.hohmann)
                );
                self.velocity.kps_updated();
                if slider.hovered() {transfer_plot.highlight_transfer()}
                if slider.dragged() | slider.has_focus() {transfer_plot.highlight_transfer()}
                
            } else if self.velocity.mmps.abs() >= 1000.0 {
                let slider = ui.add(SliderWithText::new(
                    &mut self.velocity.mps, &mut self.velocity_text,
                    min.mps..=max.mps
                )
                    .suffix(" m/s")
                    .max_decimals(14)
                    .enabled_slider(!self.hohmann)
                );
                self.velocity.mps_updated();
                if slider.hovered() {transfer_plot.highlight_transfer()}
                if slider.dragged() | slider.has_focus() {transfer_plot.highlight_transfer()}

            } else {
                let slider = ui.add(SliderWithText::new(
                    &mut self.velocity.mmps, &mut self.velocity_text, 
                    min.mmps..=max.mmps
                )
                    .suffix(" mm/s")
                    .max_decimals(14)
                    .enabled_slider(!self.hohmann)
                );
                self.velocity.mmps_updated();
                if slider.hovered() {transfer_plot.highlight_transfer()}
                if slider.dragged() | slider.has_focus() {transfer_plot.highlight_transfer()}
            }

            ui.add_space(10.0);
        });

        CentralPanel::default().show(ctx, |ui| {

            let transfer_time = transfer.time_of_flight()
                .round_to(2)
                .smallest_duration_formatted();
    
            ui.label(format!("The transfer will take {}.", transfer_time));
            ui.add_space(5.0);
            if portrait {
                ui.label(format!("The phase angle is {} °.", round_to(transfer.phase().to_degrees(), 2)));
            }
            
            Plot::new("my_plot")
            .allow_zoom(false)
            .allow_drag(false)
            .show_background(false)
            .show_axes([true; 2])
            .show_x(false).show_y(false)
            .data_aspect(1.0)
            .center_x_axis(true)
            .center_y_axis(true)

            .show(ui, |plot_ui| {
                let transfer_orbits = transfer_plot.orbit_all();
                let transfer_markers = transfer_plot.marker_all();

                for orbits in transfer_orbits {
                    plot_ui.line(orbits);
                }
                for plots in protractor.plot() {
                    plot_ui.line(plots);
                }
                for markers in transfer_markers {
                    plot_ui.points(markers);
                }
                if !portrait {
                    plot_ui.text(protractor.text());
                }
                
                plot_ui.points(
                    egui::plot::Points::new(
                        egui::plot::Values::from_values
                        (vec![egui::plot::Value::new(0.0, 0.0)]))
                            .radius(((self.origin_sma.m / self.target_sma.m).min(self.target_sma.m / self.origin_sma.m) as f32) * 20.0)
                            .shape(egui::plot::MarkerShape::Diamond)
                );
            });

        });
    }

    fn name(&self) -> &str {
        "Planetary transfer calculator"
    }
}