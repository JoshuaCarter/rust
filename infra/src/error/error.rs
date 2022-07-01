use core::fmt;
use std::any::type_name;

#[derive(Debug, Clone)]
pub struct FromStrError {
    value: String,
    r#type: String,
}

impl FromStrError {
    pub fn new<T>(val: &str) -> FromStrError {
        return FromStrError {
            value: val.to_owned(),
            r#type: type_name::<T>().to_string(),
        };
    }
}

impl fmt::Display for FromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return f.write_fmt(format_args!("Failed to create `{}` from '{}'", self.r#type, self.value));
    }
}

