# Rust Learning Roadmap

A hands-on path from Rust fundamentals to intermediate programming and ESP32
embedded development. Each stage builds on the previous one so that language
concepts are understood before they are applied to firmware.

## Learning approach

For each topic:

1. Learn the core concept and, where useful, compare it with C.
2. Write a small program that demonstrates the concept.
3. Build and run it, or flash it to the target board.
4. Read compiler and runtime errors and correct the program.
5. Explain the solution and answer the related interview questions.

Keep programs small at first. Once the basic examples work, combine them into
larger exercises and projects.

## Why Rust instead of C?

Rust is a systems programming language that provides low-level control while
using its type system to prevent many common programming errors.

- **Memory safety:** Ownership and borrowing help prevent use-after-free,
  double-free, and dangling-reference bugs without a garbage collector.
- **Thread safety:** The type system prevents many unsafe data-sharing patterns
  at compile time.
- **Error handling:** `Option` and `Result` make the possibility of missing
  values and failures explicit.
- **Modern tooling:** Cargo supports builds, dependencies, tests, formatting,
  and documentation.
- **Embedded suitability:** Rust can run without a standard library or garbage
  collector and offers control over hardware and memory.

C remains a strong choice where an existing codebase, vendor SDK, or ecosystem
is a better fit. Rust does not eliminate every bug, and some low-level
operations still require carefully audited `unsafe` code.

## Roadmap

### Stage 1: Setup and fundamentals

Start with the detailed [Stage 1 setup guide](./setup_stage.md).

**Topics**

- Install Rust and use `rustc` and Cargo
- Variables, mutability, scalar and compound types
- Operators, expressions, and basic input/output
- Conditions, loops, and `match`

**Programs**

- Unit converter
- Number guessing game
- Simple menu-driven calculator

### Stage 2: Core Rust

**Topics**

- Functions and scope
- Ownership, moves, borrowing, and references
- Slices
- Structs, enums, and pattern matching

**Programs**

- Temperature and measurement records
- Shape calculator
- Small inventory tracker

### Stage 3: Collections and error handling

**Topics**

- `String`, `Vec`, and `HashMap`
- `Option` and `Result`
- Propagating errors with `?`
- Modules and basic project organization

**Programs**

- Word counter
- Contact list
- File reader with clear error handling

### Stage 4: Intermediate Rust

**Topics**

- Traits and generics
- Iterators and closures
- Lifetimes
- Unit and integration tests

**Programs**

- Generic data processor
- Command-line task tracker
- Tested library with a small public API

### Stage 5: Embedded foundations

**Topics**

- `no_std` and embedded project structure
- Cross-compilation and target configuration
- Memory layout and peripheral access
- Logging, flashing, and debugging

**Programs**

- Build and flash a blinking LED program
- Read a button and control an LED
- Add serial logging for diagnostics

### Stage 6: ESP32 peripherals

Choose an exact ESP32 board before beginning this stage. Board support,
peripheral APIs, and project setup depend on the chip and the Rust framework
being used.

**Topics and example programs**

- **GPIO:** Blink an LED; read a button; handle debouncing
- **Timer:** Toggle an output or sample a sensor periodically
- **UART:** Implement an echo program and a simple command interface
- **I2C:** Detect a device and read a sensor
- **SPI:** Communicate with a peripheral such as a display or sensor

Start with one peripheral at a time. Check pin assignments and voltage
requirements for the specific board and connected hardware.

### Stage 7: Concurrency and RTOS concepts

**Topics**

- Tasks, scheduling, and timing
- Shared state and synchronization
- Queues and producer-consumer designs
- Interrupts, watchdogs, and safe peripheral sharing
- How the chosen ESP32 Rust framework relates to FreeRTOS

**Programs**

- Periodic sensor task
- Producer-consumer example
- Separate sensor and UART command tasks

FreeRTOS integration depends on the framework and ESP32 target. First learn
Rust ownership, `no_std`, and the board's basic peripheral APIs; then follow
the concurrency model documented by that framework rather than assuming every
Rust ESP32 project exposes the same FreeRTOS APIs.

### Stage 8: Integration project

Combine the earlier topics into a small, maintainable firmware project. One
suggestion is a sensor logger with:

- Periodic sampling using a timer
- An I2C or SPI sensor
- UART commands and diagnostic output
- A clear task or event structure
- Error handling and a short test plan

## Interview practice

Answer questions throughout the roadmap. Pair each answer with a small code
example where possible.

### Rust fundamentals

- What is ownership, and what problems does it help prevent?
- What is the difference between `&T` and `&mut T`?
- What is a move, and when does it happen?
- How do `Option<T>` and `Result<T, E>` differ?
- What does the `?` operator do?
- What are traits and generics?
- What is a lifetime, and why might a function need an explicit lifetime?

### Embedded and concurrency

- What does `no_std` mean?
- When might embedded Rust require `unsafe` code?
- What is the difference between polling and using an interrupt?
- How can shared mutable state cause problems in concurrent firmware?
- What should be considered when choosing a timer period?
- How would you diagnose a peripheral that does not respond over I2C or SPI?
- How does the selected ESP32 framework schedule concurrent work?

## Suggested workflow

### Host programs

Create a Cargo project for each exercise:

```text
cargo new hello_rust
cd hello_rust
cargo run
```

Run checks and tests as programs grow:

```text
cargo check
cargo test
cargo fmt
```

### ESP32 programs

For embedded exercises, use the setup and commands documented by the selected
Rust ESP32 framework and the exact board target. The usual workflow is:

1. Install the required Rust target, framework, and flashing/debugging tools.
2. Build for the board's target.
3. Flash the firmware.
4. Observe serial or RTT output, if configured.
5. Verify the hardware behavior and record the board, pins, and setup used.

Do not reuse build or flashing commands across different ESP32 chips or
frameworks without checking their documentation.

## Recommended first step

Begin with Stage 1: install Rust, create a Cargo project, and write a program
that reads two values and performs a simple conversion or calculation. Move to
the next stage after you can explain the code and run it independently.
