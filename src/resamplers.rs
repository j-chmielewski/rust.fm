/// Performs downsampling of real-valued input signal by specified factor.
pub struct Downsampler<'a> {
        factor: u16,
        iterator: Box<dyn Iterator<Item=&'a f32> + 'a>
}

impl<'a> Downsampler<'a> {
    /// Creates Downsampler from iterator over f32 values
    pub fn from<I>(iterator: I, factor: u16) -> Downsampler<'a> where I: Iterator<Item=&'a f32> + 'a {
        Downsampler {
            factor: factor,
            iterator: Box::new(iterator)
        }
    }
}

impl<'a> Iterator for Downsampler<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = 0.0;
        for _ in 0..self.factor {
            match self.iterator.next() {
                Some(x) => result += x,
                None => return None
            }
        }
        result /= self.factor as f32;

        return Some(result);
    }
}

#[test]
fn test_downsampler() {
    let mut downsampler = Downsampler::from([0., 1., 2., 3.].iter(), 2);
    assert_eq!(
        (downsampler.next(), downsampler.next(), downsampler.next()),
        (Some(0.5), Some(2.5), None)
    );

    let mut downsampler = Downsampler::from([0., 1., 2., 3., 4., 5.].iter(), 3);
    assert_eq!(
        (downsampler.next(), downsampler.next(), downsampler.next()),
        (Some(1.), Some(4.), None)
    );
}