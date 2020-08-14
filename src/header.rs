use std::fmt; 

pub struct Header {
    pub key: String,
    pub value: String,
}

impl Header {
    pub fn new(key: String, val: String) -> Self {
        Header {
            key: key,
            value: val
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}