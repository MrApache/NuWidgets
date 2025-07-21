use std::time::Duration;
use eframe::egui::{CentralPanel, Context};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use system_tray::client::Event;
use tokio::sync::broadcast::Receiver;
use crate::audio::AlsaVolumeController;
use crate::loaded_icons::LoadedIcons;
use crate::tray;
use crate::widgets::{clock, volume, memory};

pub struct AppState {
    audio: AlsaVolumeController,
    system: System,
    _tray_client: system_tray::client::Client,
    tray_reciever: Receiver<Event>,
    tray_icons: LoadedIcons,
}

impl AppState {
    pub async fn new() -> Self {
        let audio = AlsaVolumeController::new().unwrap();
        let system = System::new_with_specifics(RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram()));
        let _tray_client = system_tray::client::Client::new().await.unwrap();
        let tray_reciever = _tray_client.subscribe();

        Self {
            audio,
            system,
            _tray_client,
            tray_reciever,
            tray_icons: Default::default(),
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                clock::clock(ui);
                ui.separator();
                if ui.button("+").clicked() {
                    self.audio.change_volume_by(5).unwrap();
                };
                volume::volume(ui, &mut self.audio);
                if ui.button("-").clicked() {
                    self.audio.change_volume_by(-5).unwrap();
                };
                ui.separator();
                memory::memory(ui, &mut self.system);
                ui.separator();
                tray::tray(ui, &mut self._tray_client, &mut self.tray_reciever, &mut self.tray_icons);
            });
        });

        ctx.request_repaint_after(Duration::from_secs_f32(0.3))
        //ctx.request_repaint();
    }
}