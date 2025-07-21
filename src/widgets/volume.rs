use eframe::egui::{Ui, RichText};
use crate::audio::AlsaVolumeController;

pub fn volume(ui: &mut Ui, audio: &mut AlsaVolumeController) {
    let vol = audio.get_volume().unwrap();
    let icon = match vol {
        0 => "",
        1..=50 => "",
        _ => "",
    };
    ui.label(RichText::new(format!("{icon} {vol}%")).monospace());
}