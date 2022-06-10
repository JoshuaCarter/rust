# Rust Getting Started Guide

### Useful Links
- [rust book](https://doc.rust-lang.org/book/) Rust language docs.
- [cargo ref](https://doc.rust-lang.org/cargo/reference/manifest.html) Reference doc for Cargo.toml files.
- [crates](https://crates.io/) Cargo package repository.
- [rust style](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html) Rust style guide (naming conventions, etc).

### VSCode Extensions
- [TOML Language Support](https://marketplace.visualstudio.com/items?itemName=be5invis.toml) TOML language support.
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) Rust debugger.
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) Rust language support.
    - `rust-analyzer.checkOnSave.command` = `clippy` for additional linting.

### Commands
- `cargo build` Compile the current package.
- `cargo check` Analyze the current package and report errors.
- `cargo clippy` Perform `cargo check` with additional linting.
- `cargo run` Run a binary or example of the local package.
- `cargo test` Run the tests.

### Project Structure Terms

| Term | Type | Definition | Notes | Contains |
| --- | --- | --- | --- | --- |
| Workspace | Folder | Source for packages. | Defaults to the location of the top `Cargo.toml` file. | 1-n packages |
| Package | Folder | Source from which crates are built.| Defaults to workspace and must have `Cargo.toml` and `src/` in root. | 1-n crates |
| Crate | Folder | Umbrella term for compiled binary, libraries, exercises, and integration tests. | Defaults to `src/main.rs`, `src/lib.rs`, `src/exercises/`, `src/tests/`. | 1-n modules |
| Module | File / Folder | Compilation unit built from `*.rs` files (excluding `src/main.rs` and `src/lib.rs`). | `mod.rs` files required to define modules that are folders. | 0-n submodules |
| Submodule | File / Folder | Any module located directly under the folder for the current module. | | 0-n definitions |

### Project Structure
```
.                               # Workspace root
├── Cargo.toml                  # Package file (similar to package.json)
├── Cargo.lock                  # Package lock (similar to package.lock)
├── src/                        # Crates source
│   ├── lib.rs                  # Lib crate entry point (used by main.rs and tests)
│   ├── main.rs                 # Bin crate entry point
│   └── mymod/                  # Module
│       ├── mod.rs              # Module entry point (similar to index.ts)
│       ├── mymod.rs            # Module definitions (instead of in mod.rs)
│       └── submod.rs           # Submodule (re-exported by mod.rs)
├── tests/                      # Integration tests
|   └── my_test.rs              # Test module
└── target/                     # Compilation output
```

### Modules

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

**Example**:
```
.
├── lib.rs                      # Compilation entry point for lib crate
├── src/
│   ├── venues/                 # Module
│   |   ├── mod.rs              # Creates and re-exports submodules
│   |   ├── venues.rs           # Module definitions
│   |   ├── types.rs            # Module
│   |   └── binance/            # Module
│   |       ├── mod.rs          # Creates and re-exports submodules
│   |       ├── binance.rs      # Module definitions
│   |       ├── trading.rs      # Module
│   |       └── market.rs       # Module
│   └── cli/                    # Module
│       ├── mod.rs              # Creates and re-exports submodules
│       ├── input.rs            # Module
│       └── output.rs           # Module

```
