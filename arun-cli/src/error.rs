use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[allow(unused)]
#[derive(Debug, From, Display)]
pub enum Error {
	// -- App Libs
	#[from]
	Arun(arun::Error),

	// -- External
	#[from]
	IO(std::io::Error),
}
