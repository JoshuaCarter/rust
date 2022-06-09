#### Useful Links:
- [rust book](https://doc.rust-lang.org/book/) Rust language docs.
- [cargo ref](https://doc.rust-lang.org/cargo/reference/manifest.html) Reference doc for Cargo.toml files.
- [crates](https://crates.io/) Cargo package repository.
- [rust style](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html) Rust style guide (naming conventions, etc).

#### VSCode Extensions:
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) Rust language support.
- [TOML Language Support](https://marketplace.visualstudio.com/items?itemName=be5invis.toml) TOML language support.
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) Rust debugger.

#### VSCode Settings:
- `rust-analyzer.checkOnSave.command` = `clippy` This will provide lint checking on save.

#### Commands:
- `cargo build` Compile the current package.
- `cargo check` Analyze the current package and report errors.
- `cargo clippy` Perform `cargo check` with additional linting.
- `cargo run` Run a binary or example of the local package.
- `cargo test` Run the tests.

#### Project Structure
```
.                               # Package root
├── Cargo.lock                  # Similar to package.json
├── Cargo.toml                  # Similar to package.lock
├── src/                        # Crate root
│   ├── lib.rs                  # Lib entry point (used by main.rs and tests)
│   ├── main.rs                 # Bin entry point
│   └── module/                 # Module (contents determined by mod.rs)
│       ├── mod.rs              # Module entry point, similar to index.ts
│       ├── submodule1.rs       # Submodule (probably exported by mod.ts above)
│       └── nested/             # Nested Submodule
│           ├── mod.rs
│           └── submodule2.rs
├── tests/                      # Integration tests go here
|   └── my_file.rs              # Test module
└── target/                     # Compilation output

```
