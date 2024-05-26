// region:    --- XVec

#[allow(unused)] // for now.
pub trait XVec<T> {
	/// Returns the Option<&T> for the idx as vec.get(idx)
	/// However, support negative indexes that will go from the end.
	/// where `-1` will be the eventual last element.
	fn x_get(&self, idx: isize) -> Option<&T>;

	fn x_refs(&self) -> Vec<&T>;
}

impl<T> XVec<T> for Vec<T> {
	fn x_get(&self, idx: isize) -> Option<&T> {
		if idx >= 0 {
			self.get(idx as usize)
		} else {
			let adjusted_index = self.len() as isize + idx;
			if adjusted_index >= 0 {
				self.get(adjusted_index as usize)
			} else {
				None
			}
		}
	}

	fn x_refs(&self) -> Vec<&T> {
		self.iter().collect()
	}
}

// endregion: --- XVec

// region:    --- XStringVec

pub trait XStringVec {
	fn x_contains(&self, val: &str) -> bool;
	fn x_strs(&self) -> Vec<&str>;
	fn x_strings(&self) -> Vec<String>;
}

// -- Vec String, &String, &str

impl XStringVec for Vec<String> {
	fn x_strs(&self) -> Vec<&str> {
		self.iter().map(String::as_str).collect()
	}

	fn x_contains(&self, val: &str) -> bool {
		self.iter().any(|s| s == val)
	}

	fn x_strings(&self) -> Vec<String> {
		self.clone()
	}
}

impl XStringVec for Vec<&String> {
	fn x_strs(&self) -> Vec<&str> {
		self.iter().map(|&s| s.as_str()).collect()
	}

	fn x_contains(&self, val: &str) -> bool {
		self.iter().any(|&s| s == val)
	}

	fn x_strings(&self) -> Vec<String> {
		self.iter().map(|s| s.to_string()).collect()
	}
}

// impl for &str
impl XStringVec for Vec<&str> {
	fn x_strs(&self) -> Vec<&str> {
		self.clone()
	}

	fn x_contains(&self, val: &str) -> bool {
		self.contains(&val)
	}

	fn x_strings(&self) -> Vec<String> {
		self.iter().map(|s| s.to_string()).collect()
	}
}

// -- Option<Vec ...
impl XStringVec for Option<Vec<String>> {
	fn x_strs(&self) -> Vec<&str> {
		match self {
			Some(v) => v.x_strs(),
			None => vec![],
		}
	}

	fn x_contains(&self, val: &str) -> bool {
		match self {
			Some(v) => v.x_contains(val),
			None => false,
		}
	}

	fn x_strings(&self) -> Vec<String> {
		match self {
			Some(v) => v.x_strings(),
			None => vec![],
		}
	}
}
// endregion: --- XStringVec
