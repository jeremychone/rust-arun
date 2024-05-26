use clap::{crate_version, Arg, Command};

pub const VERSION: &str = crate_version!();

pub fn app_cmd() -> Command {
	Command::new("arun").version(VERSION).about("arun CLI Runner").arg(
		Arg::new("INPUT")
			.help(
				r#"- `group_name` to execute all commands in a group (from `[[runners.group_name]]`).
- `group_name.runner_name` to execute a specific command from a group (found in `[[runners.group_name]]`, then search by name).
- `solo_runner_name` from the `[[runner]]` table (matched by the name property)."#,
			)
			.required(true)
			.index(1),
	)
}
