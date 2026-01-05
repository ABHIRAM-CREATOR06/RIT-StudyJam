# 🦀 Rust Study Jam – Day 1  
**Level:** Absolute Beginner  
**Focus:** Understanding Rust basics through hands-on coding

---

## 🎯 Day 1 Learning Objectives

By the end of Day 1, students will be able to:
- Understand what a Rust program is
- Use `cargo` and the Rust compiler
- Understand project structure created by `cargo new`
- Write and run a **Hello World** program
- Take **user input** (string & integer)
- Understand **basic data types**
- Use **control statements** and **loops**

---

## 1 What is a Rust Program?

A **Rust program** is a set of instructions written in `.rs` files that:
- Are compiled (not interpreted)
- Produce fast and safe executables

Every Rust program starts execution from the **`main()` function**.

Example:
```rust
fn main() {
    println!("Hello, Rust!");
}
```
## 🔧 Rust Compiler (`rustc`) – From Code to Program

The **Rust compiler (`rustc`)** converts Rust source code (`.rs` files) into a **machine-executable program** that your operating system can run.

You usually do **not** run `rustc` directly.  
Instead, **Cargo uses `rustc` internally**.

---

## 🧠 What Happens When You Build a Rust Program?

When you run:

```bash
cargo build
```
or
```bash
cargo run
```

### Rust does the following:
    1.Reads your Rust source code (.rs)
    2.Checks for syntax and type errors
    3.Converts the code into machine code
    4.Generates output files (executable + debug info)
### In Windows 
#### .exe — Executable File
    1.This is the actual program
    2.Running this file executes your Rust application
    3. You can double-click it to run

#### .pdb — Debug Information File
  - Contains debugging information 
  - Helps debuggers:  
  - Show line numbers
  - Track variables  
  -Identify errors
  -Not required to run the program

  ####  macOS and  Linux (Simple Note)
  On macOS and Linux:  
  - No .exe or .pdb files
  - Rust creates a binary with no extension
  - Debug information is handled internally
## 3 Creating a Rust Project
  
  To create a new Rust project and navigate into it, use the following commands in your terminal:
  
  ```bash
  # Create a new project
  cargo new study_jam_day1
  
  # Move into the project folder
  cd study_jam_day1
```
## 4 Taking User Input (String)

Rust requires explicit handling for reading input from the console.

### Example: Reading a String from User

```rust
use std::io;

fn main() {
    let mut name = String::new(); // Create an empty String

    println!("Enter your name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    println!("Hello, {}", name);
}
```

Key Concepts: 
- String::new()
Creates a new, empty String to store user input.

- mut
Variables in Rust are immutable by default.
mut allows the variable to be modified.

- read_line()
Reads a line of input from standard input and appends it to the string.

- expect()
Handles errors if input reading fails.

- unwrap()
Unwraps the Result type, returning the value if successful or panicking if an error occurs.

## 5 Taking User Input (Integer)

The `read_line()` method reads input as a **String**, so it must be converted into a number.

### Example: Reading an Integer from User

```rust
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Convert string input to integer
    let number: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    println!("You entered: {}", number);
}
```

Important Steps: 
Read input as String
All console input is initially stored as text.

- trim()
Removes whitespace and the newline (\n) added when Enter is pressed.

- parse()
Converts the string into the specified numeric type (i32).

- Type annotation (i32)

## 6 Basic Data Types in Rust

Rust is a **statically typed language**, meaning the type of each variable must be known at compile time.

---

###  Integer Types

Used for whole numbers.

```rust
let a: i32 = 10;   // Signed integer
let b: u32 = 20;   // Unsigned integer
```
### Floating Point Types
Used for decimal numbers.

```rust
let x: f32 = 3.14;
let y: f64 = 9.81;
```
 ### Character Type
Represents a single Unicode character.

```rust
let c: char = 'A';
```
### String Type
Used for text data.

```rust
let name: String = String::from("Rust");
```
### Boolean Type
Used for true or false values.

```rust

let is_active: bool = true;
```

Important Notes
- Rust variables are immutable by default
- Use mut if the value needs to change
- Type annotations help Rust understand how to store data

## 7 Control Statements & Looping

Control statements help a program make decisions and repeat actions.

---

### 🔹 If / Else (Decision Making)

```rust
fn main() {
    let number = 10;

    if number > 0 {
        println!("Positive number");
    } else {
        println!("Negative number");
    }
}
```
### Looping

Rust provides several looping constructs to repeat code blocks.

#### 🔹 For Loop

```rust
fn main() {
    for i in 1..=5 {
        println!("Count: {}", i);
    }
}
```
#### 🔹 While Loop

```rust
fn main() {
    let mut counter = 0;

    while counter < 5 {
        println!("Count: {}", counter);
        counter += 1;
    }
}
```
####  Loop Keyword

```rust
fn main() {
    let mut counter = 0;

    loop {
        println!("Count: {}", counter);
        counter += 1;

        if counter == 5 {
            break;
        }
    }
}
```