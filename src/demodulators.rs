use std::f32::consts::PI;

/// Performs FM demodulation of complex signal provided by input iterator (interleaved complex samples).
pub struct FMDemodulator {
    gain: f32,
    quad_gain: f32,
    prev_re: f32,
    prev_im: f32,
    iterator: Box<dyn Iterator<Item=f32>>
}

impl<'a> FMDemodulator {
    /// Create FMDemodulator from Iterator over f32 values (interleaved complex samples).
    pub fn from<I>(iterator: I, quad_rate: f32, gain: f32, max_dev: f32) -> Self where I: Iterator<Item=f32> + 'static {
        Self {
            gain,
            quad_gain: (quad_rate / (2. * PI * max_dev)),
            prev_re: 1.,
            prev_im: 1.,
            iterator: Box::new(iterator),
        }
    }
}

impl<'a> Iterator for FMDemodulator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let re = self.iterator.next()?;
        let im = self.iterator.next()?;

        // quadrature demodulation
        let re_out = re * self.prev_re + im * self.prev_im;
        let im_out = im * self.prev_re - re * self.prev_im;

        self.prev_re = re;
        self.prev_im = im;

        Some(self.gain * self.quad_gain * im_out.atan2(re_out))
    }
}

#[test]
fn test_fmdemodulator() {
    let mut demodulator = FMDemodulator::from([1., 1., 1., 1.].iter().cloned(), 1e+6, 1., 1e+5);
    assert!(demodulator.next().is_some());
    assert!(demodulator.next().is_some());
    assert!(demodulator.next().is_none());
    // TODO: fm demod tests
}