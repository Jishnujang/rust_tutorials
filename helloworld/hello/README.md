# Hello Rust: debugging in VS Code

This project is a small Rust program intended for practicing breakpoints and
stepping through code. It prints `Count: 1` through `Count: 5`.

## Requirements

- Install the [Rust toolchain](https://www.rust-lang.org/tools/install). In a
  VS Code terminal, check that `cargo` is available:

  ```powershell
  rustc --version
  cargo --version
  ```

- Install the **CodeLLDB** extension in VS Code (extension ID:
  `vadimcn.vscode-lldb`).
- Open this project folder in VS Code, the folder containing `Cargo.toml`.
- This project's `.vscode/launch.json` configures CodeLLDB to launch
  `target/debug/hello.exe`. The matching build task runs `cargo build` before
  each debug session.

## Start a debug session

1. Open `src/main.rs`.
2. Click in the gutter (left of the line number) beside a line you want to
   pause at. For example, set a breakpoint on `counter += 1;`. A red dot
   indicates the breakpoint is enabled. You can also press **F9** with the
   cursor on that line to toggle a breakpoint.
3. Open **Run and Debug** from the Activity Bar, or press **Ctrl+Shift+D**.
4. Select **Debug Rust program** if the configuration selector is shown.
5. Press **F5** or click the green Start button. The `cargo build` task runs
   first; VS Code then launches the program under CodeLLDB.
6. When execution reaches the breakpoint, VS Code pauses and highlights the
   current statement. The Variables and Call Stack panels appear in the Run
   and Debug view.

On some keyboards, function keys are media keys by default. If **F5** does not
start or control debugging, hold **Fn** while pressing it (for example,
**Fn+F5**), or use the Run and Debug buttons in VS Code.

## Step through this program

When paused on `counter += 1;`, the local variable `counter` starts at `0`.
Use these controls to follow the loop:

| Action | Key | What it does |
| --- | --- | --- |
| Continue | **F5** | Runs until the next breakpoint or until the program ends. |
| Step Over | **F10** | Executes the current line and pauses on the next line. The counter becomes `1`. |
| Step Into | **F11** | Enters a function called on the current line, when source is available. |
| Step Out | **Shift+F11** | Finishes the current function and pauses in its caller. |
| Restart | **Ctrl+Shift+F5** | Stops and starts the debug session again. |
| Stop | **Shift+F5** | Ends the debug session. |

For this example, press **F10** to execute the increment, then **F10** again
to execute the `println!` line. The Debug Console should show `Count: 1`.
Press **F5** to continue. Because the breakpoint is inside the loop, execution
will pause there again on the next iteration. Remove or disable the breakpoint
and press **F5** to let the program finish; it prints the remaining counts
through `Count: 5`.

## Inspect values and execution

- **Variables → Local** shows local variables such as `counter`. Expand items
  to inspect them.
- **Watch** lets you monitor an expression such as `counter`. Use the **+**
  button in that section to add it.
- **Call Stack** shows the active function calls. Select a frame to inspect
  that point in the program.
- **Debug Console** displays program output and debugger messages. The
  **Terminal** may also show output, depending on the launch configuration.
- Hover over a variable in the editor to see its current value.
- To pause only when a condition is true, right-click a breakpoint and add a
  condition such as `counter == 3`.

## Build or run without the debugger

Open **Terminal → New Terminal** in VS Code and run these commands from the
project folder:

```powershell
cargo build
cargo run
```

`cargo build` creates a debug build by default, which includes debug
information. `cargo run` builds and runs the program without attaching the
VS Code debugger.

## Troubleshooting

- **F5 asks for a debugger or says no configuration is available:** Make sure
  the project folder (the one containing `Cargo.toml` and `.vscode`) is open,
  install or enable CodeLLDB, then reopen **Run and Debug** and choose
  **Debug Rust program**.
- **The build task fails or `cargo` is not recognized:** Open a new VS Code
  terminal and check `cargo --version`. Install Rust with `rustup` if needed,
  then restart VS Code so it picks up the updated `PATH`.
- **The program runs without pausing:** Confirm the breakpoint has a red dot
  (not a hollow/unverified marker), that it is on an executable line, and
  that the correct configuration is selected.
- **F5 does not respond:** First click the editor or Run and Debug view. On a
  laptop, try **Fn+F5** or use the on-screen Continue button.
- **You see `gdb is not recognized`:** This project's VS Code configuration
  uses CodeLLDB, not GDB. You do not need to run a `gdb` command to debug it
  in VS Code.
