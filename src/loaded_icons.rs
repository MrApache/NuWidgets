use std::collections::HashMap;
use eframe::egui::{ColorImage, Context, TextureHandle, TextureOptions, Vec2};
use system_tray::item::StatusNotifierItem;
use crate::tray::IconHandle;

#[derive(Default)]
pub struct LoadedIcons {
    loaded: HashMap<String, Vec<(Vec2, IconHandle)>>,
    requested: HashMap<String, Vec<(Vec2, IconHandle)>>
}

impl LoadedIcons {
    pub fn load_icon(&mut self, ctx: &Context, id: String, item: &StatusNotifierItem) {
        if self.loaded.contains_key(&id) {
            let entry = self.loaded.remove(&id).unwrap();
            self.requested.entry(id).or_insert(entry);
            return;
        }

        if let Some(file) = &item.icon_name && !file.is_empty() {
            let icons = icon::Icons::new();
            if let Some(default) = icons.find_icon(&file, 24, 1, "") {
                let vec = self.requested.entry(id).or_default();
                vec.push((Vec2::new(24.0, 24.0), IconHandle::Icon(format!("file://{}", default.path.to_str().unwrap()))));
            }
            else {
                return;
            }
        }
        else if let Some(icon_pixmap) = &item.icon_pixmap {
            let vec = self.requested.entry(id).or_default();
            for icon in icon_pixmap {
                vec.push((Vec2::new(icon.width as f32, icon.height as f32), IconHandle::TextureHandle(create_texture(ctx, &icon.pixels, icon.width as usize, icon.height as usize))))
            }
        }
        else {
            return;
        };
    }

    pub fn start_frame(&mut self) {
        self.loaded.clear();
        self.loaded.extend(self.requested.drain());
    }

    pub fn end_frame(&mut self) {
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<(Vec2, IconHandle)>)> {
        self.requested.iter()
    }
}

fn convert_argb32_network_to_rgba8(raw_data: &[u8]) -> Vec<u8> {
    assert_eq!(raw_data.len() % 4, 0);
    let mut rgba = Vec::with_capacity(raw_data.len());

    for chunk in raw_data.chunks_exact(4) {
        // собираем u32 из big-endian байт
        let argb = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);

        // теперь распарсим в каналы
        let a = ((argb >> 24) & 0xFF) as u8;
        let r = ((argb >> 16) & 0xFF) as u8;
        let g = ((argb >> 8) & 0xFF) as u8;
        let b = (argb & 0xFF) as u8;

        // egui ожидает RGBA
        rgba.push(r);
        rgba.push(g);
        rgba.push(b);
        rgba.push(a);
    }

    rgba
}
fn create_texture(
    ctx: &Context,
    raw_argb32_network: &[u8],
    width: usize,
    height: usize,
) -> TextureHandle {
    let rgba = convert_argb32_network_to_rgba8(raw_argb32_network);
    let image = ColorImage::from_rgba_unmultiplied([width, height], &rgba);
    ctx.load_texture("tray_icon", image, TextureOptions::default())
}