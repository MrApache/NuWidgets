use eframe::egui::{Image, ImageSource, TextureHandle, Ui, Vec2};
use system_tray::client::Event;
use tokio::sync::broadcast::Receiver;
use crate::loaded_icons::LoadedIcons;

pub enum IconHandle {
    TextureHandle(TextureHandle),
    Icon(String),
}

impl<'a> Into<ImageSource<'a>> for IconHandle {
    fn into(self) -> ImageSource<'a> {
        match self {
            IconHandle::TextureHandle(handle) => (&handle).into(),
            IconHandle::Icon(file) => file.into(),
        }
    }
}

impl From<&IconHandle> for ImageSource<'_> {
    fn from(value: &IconHandle) -> Self {
        match value {
            IconHandle::TextureHandle(handle) => handle.into(),
            IconHandle::Icon(file) => file.clone().into(),
        }
    }
}

pub fn tray(ui: &mut Ui, client: &mut system_tray::client::Client, tray_rx: &mut Receiver<Event>, icons: &mut LoadedIcons) {
    let items = client.items();

    icons.start_frame();

    items.lock().unwrap().iter().for_each(|(id, (item, _))| {
        //println!("Id: {id}, status: {:?}", item.status);
        icons.load_icon(ui.ctx(), id.clone(), item);
    });

    icons.end_frame();

    drop(items);

    while let Ok(ev) = tray_rx.try_recv() {
        match ev {
            Event::Add(a, _) => println!("Add: {a}"),
            Event::Update(_, _) => {}
            Event::Remove(b) => println!("Remove: {b}"),
        }
    }

    //println!("Icons: {}", icons.iter().count());

    icons.iter().for_each(|(_, icons)| {
        let (_, handle) = choose_best_icon(&icons, 24.0).unwrap();
        ui.add(Image::new(handle)
            .fit_to_exact_size(Vec2::new(24.0, 24.0))
            .max_size(Vec2::new(24.0, 24.0)));
    });
}

fn choose_best_icon(
    icons: &[(Vec2, IconHandle)],
    desired_size: f32,
) -> Option<&(Vec2, IconHandle)> {
    icons.iter()
        .min_by(|(size_a, _), (size_b, _)| {
            let dist_a = (size_a.x - desired_size).abs();
            let dist_b = (size_b.x - desired_size).abs();
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        })
}