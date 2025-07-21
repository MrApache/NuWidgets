use eframe::egui::{Ui, RichText};
use sysinfo::System;

pub fn memory(ui: &mut Ui, system: &mut System) {
    system.refresh_memory();

    let used = system.used_memory() / 1024;
    let total = system.total_memory() / 1024;

    ui.label(RichText::new(format!(" {} / {} MB", used, total)).monospace());
}