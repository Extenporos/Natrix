![License](https://img.shields.io/github/license/Extenporos/Natrix)
![Issues](https://img.shields.io/github/issues/Extenporos/Natrix)
![Pull Requests Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)
![Status](https://img.shields.io/badge/status-alpha-orange)

# Natrix

Natrix is a small Rust-based CLI project for creating and managing Python environments in a simple shell interface.

## Current state

This repository currently contains a working prototype, not a full-featured environment manager yet. The existing code implements:

- an interactive shell
- built-in commands: `version`, `about`, `clear`, `create`, and `exit`
- Python environment creation under `~/.natrix/env/<name>`
- Python runtime lookup and symlink creation for the environment
- generation of basic files such as `pyproject.toml`, `natConf.cfg`, and `pyvenv.cfg`

## How it works today

### Run the CLI

```bash
./natrix
```

Then use the shell prompt:

```text
$ create myenv
```

The `create` command will prompt for a Python version, for example:

```text
Python version? (e.g. 3.13): 3.13
```

After that, Natrix will create a directory structure similar to:

```text
~/.natrix/env/myenv/
├── pyproject.toml
├── natConf.cfg
├── pyvenv.cfg
├── .natEnv/
│   ├── bin/
│   └── lib/
```

## Built-in commands

The shell currently supports these commands:

- `version` — shows the current CLI version banner
- `about` — prints build metadata and project information
- `clear` — clears the terminal screen
- `create <name>` — creates a Python environment with the provided name
- `exit` — exits the shell

## Important notes

- The project is in early alpha stage.
- The code currently focuses on Python environment creation.
- Features such as package installation, environment activation, deletion, listing, and multi-language support are not implemented yet in the current codebase.
- The README and version metadata may be slightly inconsistent across files; the source currently declares version `1.2.1` in `Cargo.toml`.

## Project structure

```text
src/
├── commands.rs
├── handler.rs
├── main.rs
├── shell.rs
├── create_environment/
├── environment_manager/
├── runtime_manager/
├── utils/
```

## Contributions

Contributions are welcome. If you want to improve the project, the most useful areas right now are documentation, command behavior, environment creation, and runtime handling.

