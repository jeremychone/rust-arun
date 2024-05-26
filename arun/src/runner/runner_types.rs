use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Runner {
	pub name: String,

	pub working_dir: Option<String>,
	pub cmd: String,
	pub args: Option<Vec<String>>,

	pub when: Option<When>,

	#[serde(default)]
	pub wait_before: u64,

	#[serde(default)]
	pub concurrent: bool,

	#[serde(default)]
	pub end_all_on_exit: bool,
}

#[derive(Debug, Deserialize)]
pub struct When {
	pub no_file_at: Option<String>,
}

pub enum ShouldRun {
	Yes,
	No(String), // reason
}
