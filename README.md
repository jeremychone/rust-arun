# arun - command line runner

> DISCLAIMER: Version `0.1.x` is still a work in progress and might see some changes. arun-cli will follow semver more strictly for version `0.2.x` and onwards.

**arun** is a command line runner based on a toml file that runs sequential and/or parallel commands (called runners) based on their runner group or name.

This repo is a Rust workspace with the following two crates:

- `arun` - The library that does all the heavy lifting of calling the runners defined in the `arun.toml` file, which can be embedded in other CLI or app.

- `arun-cli` - The command line utility that allows running the `arun` runners. It allows you to describe a set of commands as either groups or individually and run them accordingly.

The configuration file is named `arun.toml` and should be located in the same directory as this file.

A `Runner` is a command description that can be executed by arun-cli.

There are two types of "Runners":

- **Grouped Runners**
	- Described with the `[[runners._group_name_]]` TOML array of tables for each group name (e.g., `[[runners.dev]]`).
	- Group execution: When invoking `arun _group_name_` (e.g., `arun dev`), all the runners in this group will be executed in the order they are listed in the file.
	- Individual execution: Dot notation can be used to execute only one runner in a group with `arun _group_name_._runner_name` (e.g., `arun dev.cargo_build`).
	- Constraint: Runner names in a group must be unique within that group.

- **Solo Runner**
	- Described with the `[[runner]]` TOML array of tables, where each runner has a `name` or `ref` property.
	- These can be called with `arun _runner_name` (e.g., `arun list_files`).
	- Constraint: Solo runner names must be unique and cannot overlap with group names.
	
## Example & Usage

```sh
# install
cargo install arun-cli
```

`arun.toml` in your project root (all working directories will be relative to the `arun.toml` folder)

```toml
# Solo Runner 
[[runner]]
name = "list_files"
cmd = "ls"
args = ["-ll"]

[[runner]]
name = "list_files_human"
cmd = "ls"
args = ["-llh"]

[[runners.build]]
name = "tauri_icons"
working_dir = "crates/app-desktop/"
when.no_file_at = "icons/32x32.png"
cmd = "cargo"
args = ["tauri", "icon", "icons/app-icon.png"]

[[runners.build]]
name = "cargo_build"
working_dir = "crates/app-desktop/"
cmd = "cargo"
args = ["build"]

[[runners.build]]
name = "pcss"
working_dir = "frontend/"
cmd = "npm"
args = ["run", "pcss"]

# Now: runners, can ref other runner (only one hop for now)
[[runners.dev]]
ref = "build.tauri_icons"

[[runners.dev]]
ref = "build.pcss"
# The args from the target will be extended with the args_add items
args_add = ["--", "-w"]
# will run concurrently
concurrent = true 
# if this process exit, end the dev session
end_all_on_exit = true

```

Then we can just run like so: 

- `arun build` - Will run all of the `runners.build` in order
- `arun dev` - Will run all `runnders.dev` in order
- `arun build.tauri_icons` - Will only run tauri icons command
- `arun list_files` - Will execute the solo runner named `list_files`

<br />

[This repo on GitHub](https://github.com/jeremychone/arun-cli)