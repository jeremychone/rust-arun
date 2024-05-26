use derive_more::{Display, From};
use std::process::ExitStatus;
use toml::Value;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
	// -- Exec
	#[display("Fail to read line")]
	StdinFailToReadLine,

	#[display("Fail to execute {_0} cause: {_1}")]
	Exec(String, String),

	// -- Run
	#[display("group name or command name invalid. Does not contain at least one element. Was '{_0}'")]
	RunRefNoParts(String),

	// -- Config
	#[display("'arun.toml' file not found. Should be added where 'arun` command get called.")]
	ArunTomlNotFound,

	#[display("Runner has no 'name' or 'ref' property. Value: {_0:?}")]
	RunnerHasNoNameOrRef(Value),

	#[display("Fail to parse arun.toml. Cause: {_0}")]
	FailParsingConfig(toml::de::Error),

	#[display("Fail to parse runner. Cause: {_0}")]
	FailParsingRunner(toml::de::Error),

	#[display("Solo runner '{_0}' defined multiple time")]
	SoloRunnerMultipleDef(String),

	#[display("Do not support Runner ref=... to another Runner that is also a ref=... (for now). {_0}")]
	DoNotSupportRefToRefYet(String),

	#[display("Still have some unresolved fefed Runners. {_0}")]
	StillHaveUnresolvedRefedRunners(Value),

	// -- Externals
	#[from]
	Io(std::io::Error), // as example
}

// region:    --- Froms

type ExecWithExitStatus<'a> = (&'a str, &'a [&'a str], ExitStatus);

impl<'a> From<ExecWithExitStatus<'a>> for Error {
	fn from(val: ExecWithExitStatus) -> Self {
		Error::Exec(val.0.to_string(), "".to_string())
	}
}

// endregion: --- Froms

// region:    --- Error Boilerplate

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
