use std::thread;
use std::sync::{Arc, Mutex};
use std::f32::consts::PI;
use cpal::{Data, Sample, SampleFormat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::error::RustFmError;
use std::convert::TryInto;

pub fn u8_to_f32(input: &[u8]) -> Vec<f32> {
    input.iter().map(|e| (*e as f32) / (std::u8::MAX as f32)).collect()
    // input.try_into().unwrap()
}

pub struct DownSampler {
    factor: u32,
    rem: u32,
}

impl DownSampler {
    pub fn new(factor: u32) -> Self {
        Self {factor, rem: 0}
    }

    pub fn downsample(&mut self, input: &Vec<f32>) -> Vec<f32> {
        let offset = (self.factor as u32 - self.rem as u32) % self.factor as u32;
        let output: Vec<f32> = input.iter().enumerate().into_iter().filter_map(|(i, v)| {
            if i as u32 % self.factor == offset {
                Some(*v) // FIXME: don't copy
            } else {
                None
            }
        }).collect();
        self.rem = (self.rem + input.len() as u32) % self.factor;

        output
    }
}

pub struct FMDemodulator {
    gain: f32,
    quad_gain: f32,
    prev_re: f32,
    prev_im: f32,
}

impl FMDemodulator {
    pub fn new(quad_rate: f32, gain: f32, max_dev: f32) -> Self {
        Self {
            gain,
            quad_gain: (quad_rate / (2. * PI * max_dev)),
            prev_re: 1.,
            prev_im: 1.,
        }
    }

    pub fn demod(&mut self, input: &Vec<f32>) -> Vec<f32> {
        let mut output = vec![0.; (input.len() as f32 / 2.).floor() as usize];
        let mut iter = input.iter().peekable();
        let mut count = 0;
        while iter.peek().is_some() {
            let re = iter.next().unwrap();
            let im = if iter.peek().is_some() {
                iter.next().unwrap()
            } else {
                break;
            };

            // quadrature demodulation
            let re_out = re * self.prev_re + im * self.prev_im;
            let im_out = im * self.prev_re - re * self.prev_im;

            self.prev_re = *re;
            self.prev_im = *im;
            output[count] = self.gain * self.quad_gain * im_out.atan2(re_out);
            count += 1;
        }

        output
    }
}

pub struct AudioPlayer {
    buffer: Arc<Mutex<Vec<f32>>>
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self{buffer: Arc::new(Mutex::new(vec![]))}
    }

    pub fn start(&mut self) -> cpal::Stream {
        // let host = cpal::default_host();
        // let device = host.default_output_device().expect("no output device available");
        // let mut supported_configs_range = device.supported_output_configs()
        //     .expect("error while querying configs");
        // let supported_config = supported_configs_range.next()
        //     .expect("no supported config?!")
        //     .with_max_sample_rate();
        // let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        // // let sample_format = supported_config.sample_format();
        // let config = supported_config.into();
        // // let stream = match sample_format {
        //     // SampleFormat::F32 => device.build_output_stream(&config, write_silence::<f32>, err_fn),
        //     // SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn),
        //     // SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn),

        // //     SampleFormat::F32 => println!("F32"),
        // //     SampleFormat::I16 => println!("I16"),
        // //     SampleFormat::U16 => println!("U16"),
        // // };
        // // }.unwrap();
        // // stream.play().unwrap();

        // let stream = device.build_output_stream(&config, write_silence, err_fn).unwrap();
        // stream.play().unwrap();
        // sleep(time::Duration::from_secs(2));
        // use cpal::traits::HostTrait;

        let host = cpal::default_host();

        let device = host.default_output_device().expect("no output device available");

        // use cpal::traits::{DeviceTrait, HostTrait};
        let mut supported_configs_range = device.supported_output_configs()
            .expect("error while querying configs");
        let supported_config = supported_configs_range.next()
            .expect("no supported config?!")
            .with_max_sample_rate();

            // use cpal::Data;
        // use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        // let stream = device.build_output_stream(
        //     &config,
        //     move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        //         // react to stream events and read or write stream data here.
        //     },
        //     move |err| {
        //         // react to errors here.
        //     },
        // );

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let sample_format = supported_config.sample_format();
        let config = supported_config.into();

        let buffer = Arc::clone(&self.buffer);
        // std::thread::spawn(move || {
            
            let write_silence = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut buffer = buffer.lock().unwrap();
                // println!("Writing silence");
                for sample in data.iter_mut() {
                    if let Some(s) = buffer.get_mut(0) {
                        *sample = Sample::from(s);
                        buffer.remove(0);
                    } else {
                        println!("Underrun");
                        break;
                    }
                }
                // println!("#");
            };

            // fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
            //     for sample in data.iter_mut() {
            //         *sample = Sample::from(self.buffer[0]);
            //     }
            // }

            // let stream = match sample_format {
            //     SampleFormat::F32 => device.build_output_stream(&config, write_silence::<f32>, err_fn),
            //     SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn),
            //     SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn),
            // }.unwrap();
            
            let stream = device.build_output_stream(&config, write_silence, err_fn).unwrap();
            stream
        // });
    }

    fn play(&self, input: &mut Vec<f32>) {
        println!("Play, {}", self.buffer.lock().unwrap().len());
        self.buffer.lock().unwrap().append(input);
    }
}

pub fn fm_play(freq: u32) -> Result<(), RustFmError> {
    // FIXME: should take &[u8] as argument?
    let (mut ctl, mut reader) = rtlsdr_mt::open(0)?;

    ctl.enable_agc()?;
    // ctl.set_ppm(-2)?;
    // ctl.set_center_freq(freq + 5000)?;
    ctl.set_center_freq(freq)?;
    ctl.set_sample_rate(1102500)?;

    let mut downsampler = DownSampler::new(25);
    let mut demodulator = FMDemodulator::new(44_100., 1., 75_000.);
    let mut player = AudioPlayer::new();
    let stream = player.start();
    stream.play().unwrap();
    reader.read_async(4, 32768, |bytes| {
        // let demodulated = demodulator.demod(&u8_to_f32(bytes));
        // let mut downsampled = downsampler.downsample(&demodulated);
        let mut downsampled = downsampler.downsample(&u8_to_f32(bytes));
        let mut demodulated = demodulator.demod(&downsampled);
        player.play(&mut demodulated);
    })?;
    Ok(())
}
