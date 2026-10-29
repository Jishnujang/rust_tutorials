# Stage 1: Rust Setup

This guide sets up Rust on Windows, verifies the tools, and runs your first
Rust program from VS Code. Complete these steps before starting the language
fundamentals lessons.

## 1. Install Rust

1. Open the official Rust installation page: <https://www.rust-lang.org/tools/install>.
2. Download and run **rustup-init.exe** for Windows.
3. Follow the installer prompts and choose the default installation unless
   you have a reason to use a different toolchain.
4. If the installer reports that the Microsoft C++ build tools are missing,
   cancel the Rust installation and follow the **Install the C++ build tools**
   steps below. Then rerun `rustup-init.exe`.
5. Close and reopen VS Code so its terminal picks up the updated `PATH`.

Rustup installs and manages the Rust compiler (`rustc`) and Cargo, Rust's
build and package manager.

### Install the C++ build tools

The default Rust toolchain for Windows uses Microsoft's MSVC linker. Install
the C++ build tools before continuing if rustup says they are missing:

1. Open the official [Visual Studio downloads](https://visualstudio.microsoft.com/downloads/)
   page.
2. Find **Tools for Visual Studio** and download **Build Tools for Visual
   Studio**. This is separate from the Rust installer.
3. Run the downloaded Visual Studio Installer. In the **Workloads** tab,
   select **Desktop development with C++**.
4. In the installation details, make sure an MSVC C++ build tools component
   and a Windows SDK are selected. The workload's default selections normally
   include these.
5. Click **Install** and let the installation finish. Restart Windows if the
   installer requests it.
6. Rerun `rustup-init.exe`. At the installation menu, choose **1** (standard
   installation) or press **Enter**.

You do not need the full Visual Studio IDE to follow this guide; the Build
Tools workload is sufficient. If you already have Visual Studio installed,
open **Visual Studio Installer**, choose **Modify** for that installation,
and add the **Desktop development with C++** workload instead.

### Is Cargo a framework?

No. Cargo is a **tool**, not a framework. It creates projects, builds your
code, manages dependencies, and runs tests. A framework is a library or set of
libraries that provides an application structure and APIs for a particular
kind of program. You can use Cargo to build a project that uses a framework,
but Cargo itself does not provide your program's features or design.

For this course, use Cargo to manage and run the beginner Rust exercises. Later,
an ESP32 Rust project may use a hardware framework or SDK; Cargo will still be
the tool used to build that project.

## 2. Install VS Code support

In VS Code, open **Extensions** (`Ctrl+Shift+X`), search for **rust-analyzer**,
and install the extension published by the rust-analyzer project. It provides
Rust code completion, navigation, and diagnostics. You can also install a
CodeLLDB extension later if you want to debug Rust programs inside VS Code;
it is not required for this stage.

## 3. Verify the installation

After rustup finishes successfully, close and reopen VS Code. Open a new
terminal (**Terminal > New Terminal**) and run:

```powershell
rustc --version
cargo --version
rustup show active-toolchain
```

Rust is installed if all three commands succeed:

- `rustc --version` prints a compiler version, such as `rustc 1.xx.x`.
- `cargo --version` prints a Cargo version, such as `cargo 1.xx.x`.
- `rustup show active-toolchain` prints an active toolchain. For the default
  Windows setup, it will usually include `stable-x86_64-pc-windows-msvc`.

To verify that Rust can also compile and run a program, complete the next
section and run `cargo run`. Seeing `Hello, Rust!` confirms the basic setup is
working end to end.

If PowerShell says a command is not recognized, restart VS Code first. If
commands work but compilation reports a linker or MSVC error, install the C++
build tools described above, then retry `cargo run`.

## 4. Create a first project

In the terminal, move to the folder where you want to keep your exercises.
For example:

```powershell
cd "$HOME\Documents"
cargo new hello_rust
cd hello_rust
```

Cargo creates a project with this structure:

```text
hello_rust/
|-- Cargo.toml
`-- src/
    `-- main.rs
```

- `Cargo.toml` contains project metadata and, later, dependencies.
- `src\main.rs` is the starting point for this executable.

Open the project folder in VS Code:

```powershell
code .
```

If the `code` command is unavailable, use **File > Open Folder** in VS Code and
select the `hello_rust` folder.

## 5. Write and run the first program

Replace the contents of `src\main.rs` with:

```rust
fn main() {
    println!("Hello, Rust!");
}
```

Save the file, then run these commands in the project terminal:

```powershell
cargo check
cargo run
cargo test
cargo fmt
```

`cargo run` should print:

```text
Hello, Rust!
```

The commands have different purposes. Cargo is coordinating these project
tasks; it is not a framework:

- `cargo check` checks whether the project compiles without producing an
  executable.
- `cargo run` builds and runs the program.
- `cargo test` runs the project's tests (there are no tests yet).
- `cargo fmt` formats the Rust source code.

You can also compile the source directly with `rustc`, but use Cargo for the
lessons because it is the standard way to manage Rust projects:

```powershell
rustc .\src\main.rs
.\main.exe
```

## 6. First practice task

Change the message to include your name, for example:

```rust
fn main() {
    println!("Hello, Sam! I am learning Rust.");
}
```

Then run `cargo run` again and confirm the output changed. Next, try printing
two separate lines with two `println!` statements.

## 7. Checkpoint

Before moving on, make sure you can:

- Explain what `rustc`, Cargo, and `rustup` each do.
- Create a Cargo project and find its `src\main.rs` file.
- Run `cargo check` and `cargo run`.
- Describe what `fn main()` and `println!` do.

## Troubleshooting

### `cargo` or `rustc` is not recognized

Close and reopen the terminal or VS Code after installation. If that does not
help, rerun the official Rust installer and verify that the Rust toolchain
directory is included in your user `PATH`.

### Compilation fails while linking

On Windows, the Rust MSVC toolchain needs the Microsoft C++ build tools. Install
the **Desktop development with C++** workload from Visual Studio Build Tools,
then open a new terminal and retry `cargo run`.

### VS Code does not show Rust diagnostics

Confirm that the `rust-analyzer` extension is enabled and that VS Code opened
the Cargo project folder containing `Cargo.toml`, not only the `src` folder.

## Next lesson

Continue with variables, mutability, and basic types. Use small examples and
run each one with `cargo run` before moving on to input, conditions, and loops.