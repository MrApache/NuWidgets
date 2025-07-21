use alsa::mixer::{Mixer, SelemId, SelemChannelId};
use std::error::Error;

pub struct AlsaVolumeController {
    mixer: Mixer,
}

impl AlsaVolumeController {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mixer = Mixer::new("default", false)?;
        Ok(Self { mixer })
    }

    /// Установить громкость от 0 до 100 (проценты)
    #[allow(dead_code)]
    pub fn set_volume(&self, volume_percent: i64) -> Result<(), Box<dyn Error>> {
        let selem_id = SelemId::new("Master", 0);
        let selem = self.mixer.find_selem(&selem_id).ok_or("Master element not found")?;

        let (min, max) = selem.get_playback_volume_range();

        // Конвертируем процент в диапазон ALSA
        let volume = min + (volume_percent * (max - min) / 100);

        // Устанавливаем громкость для всех каналов
        selem.set_playback_volume_all(volume)?;

        Ok(())
    }

    pub fn get_volume(&mut self) -> Result<i64, Box<dyn Error>> {
        // Обновляем состояние микшера
        self.mixer.handle_events()?;

        let selem_id = SelemId::new("Master", 0);
        let selem = self.mixer.find_selem(&selem_id).ok_or("Master element not found")?;

        let (min, max) = selem.get_playback_volume_range();

        // Берём громкость первого канала (обычно левый)
        let volume = selem.get_playback_volume(SelemChannelId::FrontLeft)?;

        // Переводим значение из диапазона [min..max] в проценты [0..100]
        let volume_percent = (volume - min) * 100 / (max - min);

        Ok(volume_percent)
    }

    pub fn change_volume_by(&mut self, delta: i64) -> Result<(), Box<dyn Error>> {
        let selem_id = SelemId::new("Master", 0);
        let selem = self
            .mixer
            .find_selem(&selem_id)
            .ok_or("Master element not found")?;

        let (min, max) = selem.get_playback_volume_range();
        let current_volume = selem.get_playback_volume(SelemChannelId::FrontLeft)?;

        // Преобразуем текущую громкость в проценты
        let current_percent = (current_volume - min) * 100 / (max - min);

        // Добавим delta, сохранив границы от 0 до 100
        let new_percent = (current_percent + delta).clamp(0, 100);

        // Конвертируем обратно в ALSA-уровень
        let new_volume = min + (new_percent * (max - min) / 100);

        // Установим новую громкость на все каналы
        selem.set_playback_volume_all(new_volume)?;

        Ok(())
    }
}
