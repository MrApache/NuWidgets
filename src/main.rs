use thiserror::Error;
use eframe::egui::ViewportBuilder;
use eframe::Renderer;

mod app;
mod widgets;
mod audio;
mod tray;
mod loaded_icons;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Dbus error: {0}")]
    Dbus(#[from] dbus::Error),
    #[error("Egui error: {0}")]
    Eframe(#[from] eframe::Error)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let viewport = ViewportBuilder::default()
        .with_inner_size((1920.0, 30.0))
        .with_transparent(true)
        .with_always_on_top()
        .with_decorations(false)
        .with_resizable(false);

    let options = eframe::NativeOptions {
        viewport,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        renderer: Renderer::Wgpu,
        centered: false,
        run_and_return: false,
        window_builder: None,
        event_loop_builder: None,
        ..Default::default()
    };

    let app_state = app::AppState::new().await;


    eframe::run_native(
        "NuWidget",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(app_state))
        }),
    )?;

    Ok(())
}