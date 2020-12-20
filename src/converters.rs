/// Performs u8 to f32 type conversion.
// TODO: generic in & out types
pub struct TypeConverter<'a> { 
    iterator: Box<dyn Iterator<Item=&'a u8> + 'a>
}

impl<'a> TypeConverter<'a> {
    /// Creates TypeConverter from Iterator over &u8
    pub fn from<I>(iterator: I) -> Self where I: Iterator<Item=&'a u8> + 'a {
        Self {
            iterator: Box::new(iterator)
        }
    }
}

impl<'a> Iterator for TypeConverter<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.iterator.next() {
            Some(*value as f32 * 2. / std::u8::MAX as f32 - 1.)
        } else {
            None
        } 
    }
}

#[test]
fn test_type_converter() {
    let mut converter = TypeConverter::from([0u8, std::u8::MAX].iter());
    assert_eq!(
        (converter.next(), converter.next(), converter.next()),
        (Some(-1.), Some(1.), None)
    );
}