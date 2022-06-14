# Rust Getting Started Guide

## Useful Links

- [Rust docs](https://doc.rust-lang.org/book/)
- [Rust cheat sheet](https://cheats.rs/)
- [Rust style guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- [Cargo docs](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Cargo.toml docs](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Crate registry](https://crates.io/)

## Installation

These steps assume you are using VSCode.

- Install `rustup` and `cargo` (see [this guide](https://doc.rust-lang.org/cargo/getting-started/installation.html)).
- Install these VSCode extensions:
    - [Rust language support](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
    - [Rust debugger](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
    - [TOML Language Support](https://marketplace.visualstudio.com/items?itemName=be5invis.toml)
- Change these VSCode settings:
    - `rust-analyzer.checkOnSave.command` = `clippy` for additional linting.
- Generate your `launch.json` (the debugger extension will do this out-of-the-box).

## Cargo Commands

- `cargo build` Compile the current package.
- `cargo check` Analyze the current package and report errors.
- `cargo clippy` Perform `cargo check` with additional linting.
- `cargo run` Run a binary or example of the local package.
- `cargo test` Run the tests.

## Project Structure

### Terms

| Term | Type | Definition | Notes | Min-Max | Contains |
| --- | --- | --- | --- | --- | --- |
| Workspace | Folder | Contains packages. | Defaults to the location of the top `Cargo.toml` file. | 1 | 1-n packages |
| Package | Folder | Source from which crates are built.| Defaults to workspace and must have `Cargo.toml` and `src/` in root. | 1-n | 1-n crates |
| Crate | Folder | Umbrella term for compiled binary, libraries, exercises, and integration tests. | Entry points default to `src/main.rs`, `src/lib.rs`, `src/exercises/`, `src/tests/`. | 1-n | 1-n modules |
| Module | File / Folder | Compilation unit built from `*.rs` files (excluding `src/main.rs` and `src/lib.rs`). | `mod.rs` files required to define modules that are folders. | 1-n | 0-n submodules |
| Submodule | File / Folder | Any module located directly under the folder for the current module. | | | 0-n definitions |
| Library | File | Compiled code library. | Defaults to `src/lib.rs`. | 0-1 | |
| Binary | File | Executable compiled from local source code and imported libraries. | Defaults to `src/main.rs`. | 0-n | |

### Layout

**Note:** I have intentionally excluded the supported module layout that replaces the `mod.rs` file with a `{module_name}.rs` file as a sibling to it's matching `{module_name}/` folder.

```
.                                   # Workspace root
├── Cargo.toml                      # Workspace config
├── my_lib/                         # Package root
|   ├── Cargo.toml                  # Package config (similar to package.json)
|   ├── src/                        # Crates source
|   │   ├── lib.rs                  # Lib crate entry point
|   │   └── my_module/              # Module
|   │       ├── mod.rs              # Module entry point (similar to index.ts)
|   │       ├── my_module.rs        # Module definitions (instead of in mod.rs)
|   │       └── my_submodule.rs     # Submodule (re-exported by mod.rs)
|   └── tests/                      # Integration tests
|       └── my_test.rs              # Test module
├── my_bin/                         # Package root
|   ├── Cargo.toml                  # Package config (similar to package.json)
|   ├── src/                        # Crates source
|   │   ├── main.rs                 # Bin crate entry point
|   |   └── my_module.rs            # Module
|   └── tests/                      # Integration tests
|       └── my_test.rs              # Test module
└── target/                         # Compilation output
```

## Modules

A module can take two basic forms:

- **`{file}.rs`**
    - File name determines module name.
    - Excludes `mod.rs`, `src/main.rs`, `src/lib.rs` files.
    - **Expected** to export some `pub` definitions.
    - **Cannot** contain have submodules (obviously).
    - **Cannot** create other modules.
    - **Avoid** re-exporting definitions from other modules.
- **`{dir}/mod.rs`**
    - Directory name determines module name.
    - Defined by `mod.rs` but hierarchically positioned at `{dir}`.
    - **Avoid** creating definitions in the `mod.rs` file.
    - **Expected** to contain submodules.
    - **Expected** to create all submodules via `mod`.
    - **Optionally** re-export definitions from submodules via `use`.
    - **Optionally** contains a submodule file with name matching `{dir}` to hold this module's definitions (re-exported in `mod.rs`).
    - **Avoid** re-exporting definitions from descendent modules.
