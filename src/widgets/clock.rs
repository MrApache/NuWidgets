use chrono::Local;
use eframe::egui::{Ui, RichText};

pub fn clock(ui: &mut Ui) {
    let now = Local::now().format("%H:%M:%S").to_string();
    ui.label(RichText::new(now).monospace());
}