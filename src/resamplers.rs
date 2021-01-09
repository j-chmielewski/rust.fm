/// Performs downsampling of real-valued input signal by specified factor.
pub struct Downsampler {
        factor: u16,
        iterator: Box<dyn Iterator<Item=f32>>
}

impl<'a> Downsampler {
    /// Creates Downsampler from Iterator over f32 values
    pub fn from<I>(iterator: I, factor: u16) -> Self where I: Iterator<Item=f32> + 'static {
        Downsampler {
            factor: factor,
            iterator: Box::new(iterator)
        }
    }
}

impl<'a> Iterator for Downsampler {
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
    let mut downsampler = Downsampler::from([0., 1., 2., 3.].iter().cloned(), 2);
    assert_eq!(
        (downsampler.next(), downsampler.next(), downsampler.next()),
        (Some(0.5), Some(2.5), None)
    );

    let mut downsampler = Downsampler::from([0., 1., 2., 3., 4., 5.].iter().cloned(), 3);
    assert_eq!(
        (downsampler.next(), downsampler.next(), downsampler.next()),
        (Some(1.), Some(4.), None)
    );
}