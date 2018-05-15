use std::fmt;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, HuffcodeError>;

#[derive(Debug, PartialEq)]
pub struct HuffcodeError(pub u8);

impl fmt::Display for HuffcodeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(match self.0 {
			0 => "not found",
			1 => "stream exhausted",
			_ => "unknown error"
		})
	}
}
