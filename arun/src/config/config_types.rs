use crate::Runner;
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
	/// Vec of runners by group name (group name is `[[runners._group_name_]]`
	pub grouped_runners: HashMap<String, Vec<Runner>>,
	/// Runner per runner name `[[runner]] name = _runner_name_`
	pub solo_runners: HashMap<String, Runner>,
}

impl Config {
	/// Get the runner for a given group name
	pub fn get_runners<'a>(&'a self, group_name: &str) -> Option<Vec<&'a Runner>> {
		self.grouped_runners.get(group_name).map(|runners| runners.iter().collect())
	}

	/// Get a single runner from group with the notation `group_name.runner_name`
	pub fn get_grouped_runner<'a>(&'a self, group_name: &str, runner_name: &str) -> Option<&'a Runner> {
		let group = self.grouped_runners.get(group_name)?;
		group.iter().find(|r| r.name == runner_name)
	}

	/// Return the solo runner
	pub fn get_solo_runner<'a>(&'a self, name: &str) -> Option<&'a Runner> {
		self.solo_runners.get(name)
	}
}
