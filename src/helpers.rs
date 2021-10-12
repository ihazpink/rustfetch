use std::io;

/// Unwraps expression or returns expression v
macro_rules! unwrap_or_return {
    ( $e:expr, $v:expr) => {
        match $e {
            Ok(x) => x,
            Err(_) => return $v,
        }
    };
}
pub(crate) use unwrap_or_return;

/// Finds value for a given key in a key-pair string
pub fn find_val(s: &str, key: &'static str, delim: &'static str) -> io::Result<String> {
    for line in s.lines() {
        let mut line = line.split(delim);
        if line.next() == Some(key) {
            return Ok(line.next().unwrap_or_default().to_string());
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Value not found for given key",
    ))
}

