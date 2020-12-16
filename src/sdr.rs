use std::f32::consts::PI;
use crate::error::RustFmError;


pub fn u8_to_f32(input: &[u8]) -> Vec<f32> {
    input.iter().map(|e| (*e as f32) / (std::u8::MAX as f32)).collect()
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

pub fn fm_play(freq: u32) -> Result<(), RustFmError> {
    // FIXME: should take &[u8] as argument?
    let (mut ctl, mut reader) = rtlsdr_mt::open(0)?;

    ctl.enable_agc()?;
    ctl.set_ppm(-2)?;
    ctl.set_center_freq(freq + 5000)?;

    let mut downsampler = DownSampler::new(10);
    let mut demodulator = FMDemodulator::new(44_100., 1., 75_000.);
    reader.read_async(4, 32768, |bytes| {
        let demodulated = demodulator.demod(&u8_to_f32(bytes));
        let downsampled = downsampler.downsample(&demodulated);
        println!("{:?}", demodulated);
    })?;
    Ok(())
}
