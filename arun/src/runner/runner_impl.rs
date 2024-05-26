use super::{Runner, ShouldRun};
use crate::utils::cli::{spawn_and_wait, spawn_tokio};
use crate::utils::XStringVec;
use crate::Result;
use std::path::Path;
use std::time::Duration;
use tokio::process::Child;
use tokio::time::sleep;

#[cfg(target_os = "windows")]
const NPM_CMD: &str = "npm.cmd";
#[cfg(not(target_os = "windows"))]
const NPM_CMD: &str = "npm";

impl Runner {
	pub async fn exec(&self) -> Result<Option<Child>> {
		// --- Process the wait_before.
		if self.wait_before > 0 {
			println!(
				"Waiting {}ms (from runner {}.wait_before property)",
				self.wait_before, self.name
			);
			sleep(Duration::from_millis(self.wait_before)).await;
		}

		// --- Compute the cmd name.
		// Note: Special handling of "npm" which on Windows must be called as "npm.cmd"
		// TODO: Needs to generalize this. Could be more downstream, on ProgramNotFound error.
		let cmd_str: &str = self.cmd.as_ref();
		let cmd_str = if cmd_str.starts_with("npm") && cmd_str != NPM_CMD {
			NPM_CMD
		} else {
			cmd_str
		};

		// --- Compute the cmd args and working dir
		let args: Vec<&str> = self.args.x_strs();
		let cwd = self.working_dir.as_ref().map(Path::new);

		// --- Execute the command
		if !self.concurrent {
			spawn_and_wait(cwd, cmd_str, args.as_slice(), true)?;
			Ok(None)
		}
		// start the concurrent mode and add it in the concurrent watch list.
		else {
			let child = spawn_tokio(cwd, cmd_str, args.as_slice(), true)?;
			Ok(Some(child))
		}
	}

	pub fn should_run(&self, root_dir: &Path) -> Result<ShouldRun> {
		let no_file_at = self.when.as_ref().and_then(|w| w.no_file_at.as_ref());

		if let Some(no_file_at) = no_file_at {
			let no_file = root_dir.join(no_file_at);
			if Path::exists(&no_file) {
				return Ok(ShouldRun::No(format!("Path '{no_file_at}' found.")));
			}
		}

		Ok(ShouldRun::Yes)
	}
}
