## Overview

Rust is a systems programming language designed for safety, concurrency, and performance. It has gained popularity for its innovative features that prevent memory-related bugs and allow for low-latency, performant, and efficient computation. Understanding Rust can provide new perspectives and approaches to building robust, concurrent, and high-performance applications.

In this 3-day workshop, attendees will receive a beginner-level introduction to Rust. By the end of the course, all participants, regardless of background, will have a solid entry-level foundation in Rust and be able to confidently use it for various real-world programming tasks.

**NOTE**: _Although no familiarity with Scala is required, special attention will be given to the similarities and differences between Scala and Rust._

### Who Should Attend

Any developers who are looking to move into Rust, explore systems level programming, or build efficient cloud-native applications. Scala developers who want to explore Rust as an alternative or complementary language, particularly for building safe, concurrent, and high-performance systems.

### Prerequisites

No prior knowledge of Rust is required. Familiarity with procedural programming in some programming languages is required.

### Topics

 - Introduction to Rust
    - Overview and goals of Rust
    - Basic syntax and concepts
 - Ownership and Borrowing in Rust
    - Ownership rules and their benefits
    - Borrowing and references
    - Lifetimes and their implications
 - Rust's Type System
    - Algebraic data types and pattern matching
    - Enums and Options
    - Traits and trait objects
    - Type inference and generics
 - Error Handling in Rust
    - The Result type and its usage
    - Error propagation and handling
    - Custom error types
 - Concurrency and Parallelism in Rust
    - Fearless concurrency and its advantages
    - Threads and message passing
    - Shared state and synchronization primitives
    - Async/await and Futures in Rust
 - Modules, Packages, and Crates
    - Organizing code with modules and packages
    - Using and managing external crates
 - Testing and Debugging in Rust
    - Unit testing and integration testing
    - Rust's built-in test framework
    - Debugging techniques and tools

### Daily Structure

Three days, 7 hours a day starting at 09:00 London Time, until 16:00 London Time.

### Attendance

Attendance at this workshop is fully remote. Attendees will be provided with a link to a remote meeting session the day before the event, in which they can see and hear the workshop, ask the instructor questions, and chat with other attendees.

### Setup Instructions

To install Rust, you'll need to install `rustup`, the Rust toolchain installer.

Rustup will also install the Rust compiler (`rustc`) and the Cargo build tool by default.

On Unix-based systems such as Linux and MacOS, run:

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows, download and run the `rustup-init.exe` from:

<https://rustup.rs/>

After running the above commands, rustup, rustc, and Cargo will be installed.

You may need to restart your terminal or run the following to update your current shell environment:

```sh
$ source $HOME/.cargo/env
```
