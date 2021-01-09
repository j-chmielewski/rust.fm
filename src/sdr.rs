use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;
use cpal::traits::StreamTrait;
use crate::{converters::TypeConverter, error::RustFmError};
use crate::resamplers::Downsampler;
use crate::demodulators::FMDemodulator;
use crate::audio::AudioPlayer; 

/// Profiling
pub fn _time(marker: &str) {
    let start = SystemTime::now();
    let datetime: DateTime<Utc> = start.into();
    println!("{}: {}", datetime.format("%S%.3f"), marker);
}

pub fn fm_play(freq: u32) -> Result<(), RustFmError> {
    // FIXME: should take &[u8] as argument?
    let (mut ctl, mut reader) = rtlsdr_mt::open(0)?;

    ctl.enable_agc()?;
    ctl.set_center_freq(freq)?;
    ctl.set_sample_rate(1_102_500)?;

    let mut player = AudioPlayer::new();
    let stream = player.start();
    stream.play().unwrap();
    reader.read_async(4, 1_102_500, |bytes| {
        let mut bytes_c = vec![0; bytes.len()];
        bytes_c.clone_from_slice(&bytes);
        let converter = TypeConverter::from(bytes_c.into_iter());
        let demodulator = FMDemodulator::from(converter.into_iter(), 1_102_500., 1., 75_000.);
        let downsampler = Downsampler::from(demodulator.into_iter(), 25);
        // FIXME: avoid copying
        let mut buff: Vec<f32> = downsampler.into_iter().collect();
        player.play(&mut buff);
    })?;
    Ok(())
}
