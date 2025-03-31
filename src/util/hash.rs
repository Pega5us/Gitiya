use crate::constants;
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct SHA(String);

impl SHA {
    pub fn new(hex: &str) -> Result<Self, String> {
        if hex.len() == constants::SHA_LENGTH  && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(Self(hex.to_string()));
        }
        return Err(format!("invalid hex: {}, can't init SHA!", hex));
    }
    pub fn get_directory(&self) -> &str {
        &self.0[..2]
    }

    pub fn get_file(&self) -> &str {
        &self.0[2..]
    }
}

/// Supports [0..2]
impl Index<std::ops::Range<usize>> for SHA {
    type Output = str;
    fn index(&self, range: std::ops::Range<usize>) -> &Self::Output {
        &self.0[range]
    }
}

/// Supports sha[2..]
impl Index<std::ops::RangeFrom<usize>> for SHA {
    type Output = str;
    fn index(&self, range: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.0[range]
    }
}