# Hello World Rust Project

This folder contains the first Rust program created with Cargo.

## Folder structure

```text
helloworld/
|-- README.md
`-- hello/                 Cargo project
    |-- Cargo.toml         Package metadata and dependencies
    |-- .gitignore         Files Git should ignore, including build output
    |-- .vscode/
    |   |-- launch.json    VS Code CodeLLDB launch configuration
    |   `-- tasks.json     Builds the program before debugging
    `-- src/
        `-- main.rs        Rust program entry point
```

### What the files do

- **`hello/Cargo.toml`** describes the package: its name (`hello`), version,
  Rust edition, and dependencies. The dependencies section is currently empty.
- **`hello/src/main.rs`** contains the program. Rust starts running this
  executable from its `main` function.
- **`hello/.gitignore`** excludes generated build files such as the `target`
  directory from Git.
- **`hello/target/`** is created by Cargo when the project is built. It holds
  generated build output; it is not source code and usually should not be
  edited by hand.
- **`README.md`** (this file) explains the project layout.

## Run the program

Open PowerShell in the `helloworld` folder and run:

```powershell
cd .\hello
cargo run
```

The expected output includes:

```text
Hello, world!
```

Cargo commands must be run from the `hello` project folder, where
`Cargo.toml` is located. To check compilation without running the program, use
`cargo check` from that same folder.

## Debug in VS Code

Open the `hello` folder (the folder containing `Cargo.toml`) in VS Code. Open
`src\main.rs`, click in the gutter beside the `println!` line to set a
breakpoint, then press **F5** and select **Debug Rust program** if prompted.
The CodeLLDB configuration builds the project before launching it.

If VS Code does not start the debugger, open **Run and Debug** and choose
**Debug Rust program** from the configuration dropdown. The active Rust
toolchain in this setup is Windows GNU; if CodeLLDB reports that it cannot
debug the resulting executable, its Windows debugger may require an MSVC
toolchain and Visual Studio C++ Build Tools.
