### Useful Links:
- [rust book](https://doc.rust-lang.org/book/) Rust language docs.
- [cargo ref](https://doc.rust-lang.org/cargo/reference/manifest.html) Reference doc for Cargo.toml files.
- [crates](https://crates.io/) Cargo package repository.
- [rust style](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html) Rust style guide (naming conventions, etc).

### VSCode Extensions:
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) Rust language support.
- [TOML Language Support](https://marketplace.visualstudio.com/items?itemName=be5invis.toml) TOML language support.
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) Rust debugger.

### VSCode Settings:
- `rust-analyzer.checkOnSave.command` = `clippy` This will provide lint checking on save.

### Commands:
- `cargo build` Compile the current package.
- `cargo check` Analyze the current package and report errors.
- `cargo clippy` Perform `cargo check` with additional linting.
- `cargo run` Run a binary or example of the local package.
- `cargo test` Run the tests.

### Project Structure
```
.                               # Package root
├── Cargo.toml                  # Similar to package.json
├── Cargo.lock                  # Similar to package.lock
├── src/                        # Crate root
│   ├── lib.rs                  # Lib entry point (used by main.rs and tests)
│   ├── main.rs                 # Bin entry point
│   └── parent_mod/             # Module (contents determined by mod.rs)
│       ├── mod.rs              # Module entry point, similar to index.ts
│       ├── child_mod.rs        # Submodule (probably exported by mod.ts above)
│       └── child_mod/          # Nested Submodule
│           ├── mod.rs
│           └── descendent_mod.rs
├── tests/                      # Integration tests go here
|   └── my_file.rs              # Test module
└── target/                     # Compilation output

```

### Modules

A module can take two basic forms:

**File Module** (excluding `mod.rs`, `lib.rs`, and `main.rs` files):
- File name determines module name.
- **Expected** to export some `pub` definitions.
- **Cannot** contain other modules.
- **Cannot** create other modules.
- **Avoid** re-exporting definitions from other modules.

**Folder Module**:

We will refer to a folder module (and it's `mod.rs` file) as the **"parent"** and the file/folder modules it contains (i.e. siblings of `mod.rs`) as it's **"children"**. Modules nested more deeply are **"descendents"**.

- Folder name determines module name.
- **Must** contain a `mod.rs` file that defines this module's exports.
- **Avoid** creating definitions in the `mod.rs` file.
- **Expected** to contain child modules.
- **Expected** to create all child modules via `mod ___;` or `pub mod ___;`.
- **Optionally** re-export definitions from child modules via `use ___;` or `pub use ___;`.
- **Avoid** re-exporting definitions from descendent modules.
