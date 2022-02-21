use egui::{Response, Widget, Ui, Slider, TextEdit};
use thousands::Separable;

use planetary_transfer::round_to;

pub struct SliderWithText<'a> {
    value: &'a mut f64,
    text: &'a mut String,
    range: std::ops::RangeInclusive<f64>,
    suffix: &'a str,
    max_decimals: usize,
    enabled_slider: bool,
}

impl<'a> SliderWithText<'a> {
    pub fn new(value: &'a mut f64, text: &'a mut String, range: std::ops::RangeInclusive<f64>) -> Self {
        Self {
            value,
            text,
            range,
            suffix: "",
            max_decimals: 8,
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
            .smallest_positive(0.5)
    }
    
    fn value(&mut self) -> TextEdit {
        TextEdit::singleline(self.text)
            .text_style(egui::TextStyle::Monospace)
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
    pub fn suffix(mut self, suffix: &'a str) -> Self {
        self.suffix = suffix;
        self
    }

    pub fn max_decimals(mut self, max_decimals: usize) -> Self {
        self.max_decimals = max_decimals;
        self
    }

    pub fn enabled_slider(mut self, enabled_slider: bool) -> Self {
        self.enabled_slider = enabled_slider;
        self
    }
}