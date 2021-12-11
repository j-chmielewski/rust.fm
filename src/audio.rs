use std::sync::{Arc, Mutex};
use std::io;
use cpal::{Sample, SampleRate};
use cpal::traits::{DeviceTrait, HostTrait};
use io::Write;

pub struct AudioPlayer {
    buffer: Arc<Mutex<Vec<f32>>>
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self{buffer: Arc::new(Mutex::new(vec![]))}
    }

    pub fn start(&mut self) -> cpal::Stream {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        let mut supported_configs_range = device.supported_output_configs()
            .expect("error while querying configs");
        let supported_config = supported_configs_range.next()
            .expect("no supported config?!")
            .with_sample_rate(SampleRate(44100));
        println!("{:#?}", supported_config);

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let config = supported_config.into();

        let buffer = Arc::clone(&self.buffer);
            
        let write_silence = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut buffer = buffer.lock().unwrap();
            for sample in data.iter_mut() {
                if let Some(s) = buffer.get_mut(0) {
                    *sample = Sample::from(s);
                    buffer.remove(0);
                } else {
                    print!("AU ");
                    io::stdout().flush().unwrap();
                    break;
                }
            }
        };
        
        let stream = device.build_output_stream(&config, write_silence, err_fn).unwrap();
        stream
    }

    pub fn play(&self, input: &mut Vec<f32>) {
        self.buffer.lock().unwrap().append(input);
    }
}

#[test]
fn test_audioplayer() {
    // TODO: audio player tests
}