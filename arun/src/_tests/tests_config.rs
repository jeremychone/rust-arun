//! Note: Tests are very rudimentary. More to come.

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use super::parse_arun_toml;
use super::Config;
use crate::utils::XStringVec;

const SIMPLE_ARUN_TMPL: &str = r#"
Some = "stuff"

[[runner]]
name = "my_solo_runner"
cmd = "ls"
args = ["-llh"]

[[runners.build]]
name = "build_stuff"
working_dir = "some-work-dir"
cmd = "cargo"
args = ["build"]

[[runners.dev]]
name = "tauri_icons"
working_dir = "crates/app-desktop/"
when.no_file_at = "icons/32x32.png"
cmd = "cargo"
args = ["tauri", "icon", "icons/app-icon.png"]

# Note: Does a furst cargo build of the app-desktop.
#       This seems to help the future build process.
[[runners.dev]]
name = "cargo_build_tauri_app"
working_dir = "crates/app-desktop/"
cmd = "cargo"
args = ["build"]

"#;

const REFED_ARUN_TMPL: &str = r#"
Some = "stuff"

[[runner]]
name = "my_solo_runner"
cmd = "ls"
args = ["-llh"]

[[runners.build]]
name = "tauri_icons"
working_dir = "crates/app-desktop/"
when.no_file_at = "icons/32x32.png"
cmd = "cargo"
args = ["tauri", "icon", "icons/app-icon.png"]

[[runners.build]]
name = "pcss"
working_dir = "frontend/"
cmd = "npm"
args = ["run", "pcss"]
concurrent = true 
# if this process exit, end the dev session
end_all_on_exit = true

[[runners.dev]]
ref = "build.tauri_icons"

[[runners.dev]]
ref = "build.pcss"
args = ["run_special", "pcss", "--", "-w"]

[[runners.dev2]]
ref = "build.pcss"
args_add = ["--", "-w"]
"#;

#[test]
fn test_parse_simple_arun_toml() -> Result<()> {
	// -- Exec
	let config: Config = parse_arun_toml(SIMPLE_ARUN_TMPL)?;

	// -- Check dev runners
	// Number of dev runners.
	let runners = config.get_runners("dev").ok_or("Should have dev runners.")?;
	assert_eq!(runners.len(), 2, "Number of dev runners.");
	// Second runner.
	let runner = runners.get(1).unwrap(); // Should be the 'tauri_icons'.
	assert_eq!(runner.name, "cargo_build_tauri_app");

	// -- Check build runners
	// Number of build runners.
	let runners = config.get_runners("build").ok_or("Should have build runners.")?;
	assert_eq!(runners.len(), 1, "Number of build runners.");

	// -- Check solo runner
	let runner = config.get_solo_runner("my_solo_runner").ok_or("Should have the solo runner.")?;
	assert_eq!(runner.name, "my_solo_runner");

	Ok(())
}

#[test]
fn test_parse_refed_arun_toml() -> Result<()> {
	// -- Exec
	let config: Config = parse_arun_toml(REFED_ARUN_TMPL)?;

	// -- Check - dev.tauri_icons
	let runner = config
		.get_grouped_runner("dev", "tauri_icons")
		.ok_or("Should have return a runner")?;
	assert_eq!(runner.name, "tauri_icons");
	assert_eq!(runner.cmd, "cargo");
	let args: Vec<&str> = runner.args.x_strs();
	assert_eq!(args, &["tauri", "icon", "icons/app-icon.png"]);

	// -- Check - dev.pcss (for overrides, e.g., args)
	let runner = config.get_grouped_runner("dev", "pcss").ok_or("Should have return a runner")?;
	let args: Vec<&str> = runner.args.x_strs();
	assert_eq!(args, &["run_special", "pcss", "--", "-w"]);

	// -- Check - dev2.pcss (for args_add)
	let runner = config.get_grouped_runner("dev2", "pcss").ok_or("Should have return a runner")?;
	let args: Vec<&str> = runner.args.x_strs();
	assert_eq!(args, &["run", "pcss", "--", "-w"]);

	Ok(())
}
