use crate::config::Config;
use crate::{Error, Result, Runner};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, mem};
use toml::{Table, Value}; // Wide for now.

const AWESOME_FILE_NAME: &str = "arun.toml";

const KEY_RUNNERS: &str = "runners";
const KEY_RUNNER: &str = "runner";

pub fn find_and_parse_arun_toml(root_dir: &Path) -> Result<Config> {
	// --- Create if not present.
	let arun_file = root_dir.join(AWESOME_FILE_NAME);
	if !arun_file.is_file() {
		return Err(Error::ArunTomlNotFound);
	}

	// --- Load and validate.
	let toml_str = fs::read_to_string(&arun_file)?;
	parse_arun_toml(&toml_str)
}

// --- Config Types

// --- arun.toml generator / parser

#[derive(Debug)]
struct RunnerHolder {
	group: Option<String>,
	value: Value,
	kind: RunnerKind,
}

#[derive(Debug)]
enum RunnerKind {
	Named(String),
	Refed(String),
}

impl RunnerHolder {
	fn new(group: Option<String>, value: Value) -> Result<Self> {
		let kind = if let Some(Value::String(name)) = value.get("name") {
			RunnerKind::Named(name.to_string())
		} else if let Some(Value::String(ref_)) = value.get("ref") {
			RunnerKind::Refed(ref_.to_string())
		} else {
			return Err(Error::RunnerHasNoNameOrRef(value));
		};

		Ok(RunnerHolder { kind, group, value })
	}

	fn get_key(&self) -> String {
		let group_name = self.group.as_deref().unwrap_or("");
		match &self.kind {
			RunnerKind::Named(name) => format!("{group_name}.{name}"),
			RunnerKind::Refed(ref_name) => format!("{group_name}.#{ref_name}"),
		}
	}

	fn is_refed(&self) -> bool {
		matches!(self.kind, RunnerKind::Refed(_))
	}
}

/// Parse an `arun.toml`
/// TODOS:
/// - Check that no name conflict between the solo runner and group names
/// - Check that no name conflict within a group
fn parse_arun_toml(toml_str: &str) -> Result<Config> {
	// let config = toml::from_str::<Value>(toml_str).map_err(Error::FailParsingConfig)?;
	let mut root_table: Table = toml_str.parse().map_err(Error::FailParsingConfig)?;

	// let runner: Runner = Runner::deserialize(runner).map_err(Error::FailParsingRunner)?;

	// Validate that it does not already exists (otherwise, fail)
	// if solo_runners.contains_key(&runner.name) {
	// 	return Err(Error::SoloRunnerMultipleDef(runner.name));
	// }

	// -- Collect all RunnerHolders
	//    This will collect all RunnerHolder and store the various indexes to resolve the ref later.
	//    It will effectively flatten everything.
	//    key: `.runner_name` for solo runner
	//    key: `.#ref_name` for the refed ones
	//    key: `group_name.runner_name` for grouped runners
	//    key: `group_name.#ref_name` for grouped runners that are ref
	let mut all_runners: Vec<RunnerHolder> = Vec::new();
	let mut named_idx_by_key: HashMap<String, usize> = HashMap::new();
	let mut refed_idxs: Vec<usize> = Vec::new();

	let mut process_runner_holder = |group_name: Option<&str>, runner: toml::Value| -> Result<()> {
		let rh = RunnerHolder::new(group_name.map(String::from), runner)?;

		let is_refed = rh.is_refed();

		let idx = all_runners.len();

		if is_refed {
			refed_idxs.push(idx);
		} else {
			let key = rh.get_key();
			named_idx_by_key.insert(key, idx);
		}

		all_runners.push(rh);

		Ok(())
	};

	// Collect grouped RunnerHolders
	if let Some(Value::Table(runner_groups)) = root_table.remove(KEY_RUNNERS) {
		for (group_name, runners) in runner_groups.into_iter() {
			if let Value::Array(runners) = runners {
				for runner in runners {
					process_runner_holder(Some(&group_name), runner)?;
				}
			}
		}
	}
	// Collect the solo RunnHolders
	if let Some(Value::Array(runners)) = root_table.remove(KEY_RUNNER) {
		for runner in runners {
			process_runner_holder(None, runner)?;
		}
	}

	// -- Resolve the ref
	for refed_idx in refed_idxs.into_iter() {
		let mut new_runner_holder_for_idx: Option<(RunnerHolder, usize)> = None;

		if let Some(refed_runner) = all_runners.get(refed_idx) {
			if let RunnerKind::Refed(ref_) = &refed_runner.kind {
				if let Some(idx) = named_idx_by_key.get(ref_) {
					if let Some(runner_base) = all_runners.get(*idx) {
						if runner_base.is_refed() {
							return Err(Error::DoNotSupportRefToRefYet(refed_runner.get_key()));
						}
						// TODO: Should find a way to avoid refed_runner value clone
						let new_value = merge_runner_values(runner_base.value.clone(), refed_runner.value.clone());
						let new_runner = RunnerHolder::new(refed_runner.group.clone(), new_value)?;

						// IMPORTANT: needs to give the refed_idx, as this is the one we want to replace
						new_runner_holder_for_idx = Some((new_runner, refed_idx));
					}
				}
			}
		}

		if let Some((new_rh, refed_idx)) = new_runner_holder_for_idx {
			let _ = mem::replace(&mut all_runners[refed_idx], new_rh);
		}
	}

	// -- Build the config properties
	let mut grouped_runners: HashMap<String, Vec<Runner>> = HashMap::new();
	let mut solo_runners: HashMap<String, Runner> = HashMap::new();

	for runner_holder in all_runners {
		if runner_holder.is_refed() {
			return Err(Error::StillHaveUnresolvedRefedRunners(runner_holder.value));
		}

		let runner: Runner = Runner::deserialize(runner_holder.value).map_err(Error::FailParsingRunner)?;
		match runner_holder.group {
			Some(group) => {
				grouped_runners.entry(group).or_default().push(runner);
			}
			None => {
				solo_runners.insert(runner.name.to_string(), runner);
			}
		}
	}

	Ok(Config {
		grouped_runners,
		solo_runners,
	})
}

fn merge_runner_values(base_value: Value, ov_value: Value) -> Value {
	match (base_value, ov_value) {
		(Value::Table(mut base_value), Value::Table(ov_value)) => {
			for (name, value) in ov_value.into_iter() {
				if name == "ref" {
					continue;
				}
				if name == "args_add" {
					if let Value::Array(args_add) = value {
						let base_args = base_value.entry("args").or_insert_with(|| Value::Array(Vec::new()));
						if let Value::Array(base_args) = base_args {
							base_args.extend(args_add);
						}
					}
				} else {
					base_value.insert(name, value);
				}
				// TODO: needs to handle the `args_add`
			}
			Value::Table(base_value)
		}
		(base_value, _) => base_value,
	}
}

// region:    --- Tests
#[cfg(test)]
#[path = "../_tests/tests_config.rs"]
mod tests;
// endregion: --- Tests
