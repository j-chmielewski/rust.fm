/// Performs u8 to f32 type conversion.
pub struct TypeConverter { 
    iterator: Box<dyn Iterator<Item=u8>>
}

impl<'a> TypeConverter {
    /// Creates TypeConverter from Iterator over &u8
    pub fn from<I>(iterator: I) -> Self where I: Iterator<Item=u8> + 'static {
        Self {
            iterator: Box::new(iterator)
        }
    }
}

impl<'a> Iterator for TypeConverter {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.iterator.next() {
            Some(value as f32 * 2. / u8::MAX as f32 - 1.)
        } else {
            None
        } 
    }
}

#[test]
fn test_type_converter() {
    let mut converter = TypeConverter::from([0u8, u8::MAX].iter().cloned());
    assert_eq!(
        (converter.next(), converter.next(), converter.next()),
        (Some(-1.), Some(1.), None)
    );
}