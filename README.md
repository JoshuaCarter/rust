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

## Cargo

Cargo is the official rust package manager (think npm, composer, etc), but it also helps with local development by making it easy to perform common operations in our rust workspace.

### Useful Commands

- `cargo build` Compile the packages in this workspace.
- `cargo check` Analyze the packages in this workspace.
- `cargo clippy` Does `cargo check` with additional linting.
- `cargo run` Run a binary or example.
- `cargo test` Run the tests for packages in this workspace.

## Project Anatomy

### Terms

| Term | Type | Definition | Notes | Min-Max | Contains |
| --- | --- | --- | --- | --- | --- |
| Workspace | Folder | Contains packages. | Defaults to the location of the top `Cargo.toml` file. | 1 | 1-n packages |
| Package | Folder | Source from which crates are built.| Defaults to workspace and must have `Cargo.toml` and `src/` in root. | 1-n | 1-n crates |
| Crate | Folder | Umbrella term for compiled binary, libraries, exercises, integration tests, and the source that builds them. | Entry points default to `src/main.rs`, `src/lib.rs`, `src/exercises/`, `src/tests/`. | 1-n | 1-n modules |
| Module | File / Folder | Compilation unit built from `*.rs` files (excluding `src/main.rs` and `src/lib.rs`). | `mod.rs` files required to define modules that are folders. | 1-n | 0-n items, 0-n submodules |
| Submodule | See module | Any module located directly under the folder for the current module. | Only root module cannot be a submodule. | | See module |
| Item | Code | Component of a crate, mostly definitions. | Many types [see the docs](https://doc.rust-lang.org/reference/items.html). | 0-n | |
| Library | File | Compiled code library. | Defaults to `src/lib.rs`. | 0-1 | |
| Binary | File | Executable compiled from local source code and imported libraries. | Defaults to `src/main.rs`. | 0-n | |

### Layout

**Note:** I have intentionally excluded the supported module layout that replaces the `mod.rs` file with a `{module_name}.rs` file as a sibling to it's matching `{module_name}/` folder.

```
.                                   # Workspace root
├── Cargo.toml                      # Workspace config (defines sub-packages, etc)
├── Cargo.lock                      # Workspace lock (think shared package.lock)
├── my_lib/                         # Package root
|   ├── Cargo.toml                  # Package config (think package.json)
|   ├── src/                        # Crates source
|   │   ├── lib.rs                  # Lib crate entry point
|   │   └── my_module/              # Module
|   │       ├── mod.rs              # Module entry point (think index.ts)
|   │       ├── my_module.rs        # Module definitions (instead of in mod.rs)
|   │       └── my_submodule.rs     # Submodule (re-exported by mod.rs)
|   └── tests/                      # Integration tests
|       └── my_test.rs              # Test module
├── my_bin/                         # Package root
|   ├── Cargo.toml                  # Package config (has my_lib dependency)
|   ├── src/                        # Crates source
|   │   ├── main.rs                 # Bin crate entry point
|   |   └── my_module.rs            # Module
|   └── tests/                      # Integration tests
|       └── my_test.rs              # Test module
└── target/                         # Shared compilation output
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


## Documentation

DocBlocks are denoted by `///` and use markdown formatting.

```
/// First line is short summary.
///
/// Next is detailed/verbose documentation.
///
/// Code blocks have implicit `fn main()` and `extern crate <cratename>`,
/// allowing them to run as unit tests or on demand by developers when
/// part of a lib crate.
///
/// The `no_run` flag will prevent code from running as a test, and you
/// can allow panics with `should_panic`.
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compiled!
/// let result = crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
```

For a more complete overview [see the docs](https://doc.rust-lang.org/stable/rust-by-example/testing/doc_testing.html).


## Tests

### Units Tests

Unit tests are located alongside the code being tested, within the same module scope, generally in the same file. They can also be located in the DocBlock for a function.

### Integration Tests

Integration tests are located in the `package/tests/` directory next to `package/src/`. They import our compiled libraries and test the public-facing contents (often in combination).


## Linting

The Rust compilation will perform some default linting (warnings about unused code, etc), but we use `cargo clippy` for additional linting on file save. Any modifications to the linting rules are declared inside the crate entry-point file (e.g. `lib.rs`).
