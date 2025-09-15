# Rust 100 Projects: Complete Learning Journey

[![Rust](https://img.shields.io/badge/rust-1.75-blue?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Two Workspaces](https://img.shields.io/badge/workspaces-two%20independent-orange)](https://doc.rust-lang.org/cargo/reference/workspaces.html)

A comprehensive collection of **100 Rust projects** divided into two parallel learning tracks: **Application Projects** (50 projects) and **Language Mastery Projects** (50 projects). Designed for progressive learning from beginner to expert, covering both practical application development and deep understanding of Rust's unique language features.

## 🎯 Overview

This repository implements **two distinct learning paths**, each with 50 projects organized into **3 difficulty tiers**:

### **Track 1: Application Projects (Algorithmia Track)**
**Focus**: Building real-world applications, algorithms, and systems using Rust. Learn Rust through practical implementation of CLI tools, games, web services, and advanced algorithms.

**Projects**: 50 practical applications ranging from simple calculators to quantum simulators and OS kernels.

**Learning Outcomes**:
- CLI application development
- File I/O, networking, and databases
- Algorithms and data structures
- Concurrency and performance optimization
- Systems programming and embedded development

### **Track 2: Language Mastery Projects**
**Focus**: Deep dive into Rust's unique language features, ownership model, and advanced concepts. Understand *why* Rust works the way it does.

**Projects**: 50 focused exercises demonstrating specific Rust language features from basic ownership to advanced metaprogramming.

**Learning Outcomes**:
- Ownership, borrowing, and lifetimes
- Traits, generics, and type system
- Concurrency primitives and safety
- Unsafe Rust, FFI, and systems programming
- Macros, custom allocators, and metaprogramming

## 📊 Project Structure

```
rust-100-projects/
├── README.md                          # This file
├── LICENSE                            # MIT License
│
├── application-projects/               # Track 1: Practical Applications (Workspace 1)
│   ├── Cargo.toml                     # Application workspace
│   ├── README.md                      # Application track details
│   ├── easy/                          # Projects 1-20: Basic applications
│   │   ├── project-01-hello-world/    # Hello World with Input
│   │   ├── project-02-calculator/     # Simple Calculator
│   │   ├── project-03-fizzbuzz/       # FizzBuzz
│   │   └── ... (18 more basic projects)
│   │
│   ├── medium/                        # Projects 21-35: Intermediate systems
│   │   ├── project-21-generic-stack/  # Generic Stack Implementation
│   │   ├── project-23-todo-app/       # Command-Line Todo App
│   │   ├── project-24-http-server/    # Simple HTTP Server
│   │   └── ... (12 more intermediate projects)
│   │
│   └── hard/                          # Projects 36-50: Advanced systems
│       ├── project-36-memory-allocator/ # Toy Memory Allocator
│       ├── project-43-os-kernel/      # OS Kernel Module (Toy)
│       ├── project-50-quantum-sim/    # Quantum Simulator (Basic)
│       └── ... (14 more advanced projects)
│
└── language-projects/                 # Track 2: Rust Language Features (Workspace 2)
    ├── Cargo.toml                     # Language workspace
    ├── README.md                      # Language track details
    ├── easy/                          # Projects 1-20: Core language concepts
    │   ├── project-01-personal-greeter/ # Basic Ownership & I/O
    │   ├── project-08-person-struct/   # Structs & Methods
    │   ├── project-11-shape-drawer/    # Traits & Polymorphism
    │   └── ... (17 more basic concepts)
    │
    ├── medium/                        # Projects 21-35: Advanced ownership
    │   ├── project-21-rc-arc-sharing/  # Reference Counting
    │   ├── project-24-async-hello/     # Async/Await Basics
    │   ├── project-31-unsafe-pointer/  # Unsafe Rust & Pointers
    │   └── ... (12 more intermediate concepts)
    │
    └── hard/                          # Projects 36-50: Expert features
        ├── project-36-associated-constants/ # Associated Constants
        ├── project-42-custom-allocator/ # Global Allocator
        ├── project-43-no-std-env/      # No_std Environment
        └── ... (14 more expert features)
```

## 🚀 Quick Start

### **Prerequisites**
- **Rust 1.75+** (stable): `rustup update stable`
- **RustRover** or **VS Code** with rust-analyzer
- **Git** for version control
- **~2GB disk space** (100 projects with dependencies)

### **Installation**
```bash
# Clone the repository
git clone https://github.com/yourusername/rust-100-projects.git
cd rust-100-projects

# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup update stable

# For language track advanced projects (nightly features)
rustup install nightly
```

### **Workspace Setup**
```bash
# Build Application workspace (downloads dependencies)
cd application-projects && cargo build && cd ..

# Build Language workspace (downloads dependencies)  
cd language-projects && cargo build && cd ..

# Verify setup (run from each workspace directory)
cd application-projects && cargo check && cd ..
cd language-projects && cargo check && cd ..
```

### **Running Projects**

#### **Application Track**
```bash
# Navigate to Application workspace
cd application-projects

# Run Hello World (Project 1)
cargo run --bin project-01-hello-world

# Run Todo App with arguments (Project 23)  
cargo run --bin project-23-todo-app -- add "Learn Rust"

# Test all Application projects
cargo test --workspace

# Return to root
cd ..
```

#### **Language Track**
```bash
# Navigate to Language workspace
cd language-projects

# Run Personal Greeter (Project 1 - ownership basics)
cargo run --bin project-01-personal-greeter

# Run Shape Drawer (Project 11 - traits)
cargo run --bin project-11-shape-drawer

# Test all Language projects
cargo test --workspace

# Test ownership correctness (shows borrow checker in action)
cargo test -- --nocapture

# Return to root
cd ..
```

### **Nightly Features (Language Track Projects 40-50)**
```bash
# Enable nightly for advanced language features
cd language-projects
rustup override set nightly

# Run with nightly (from language-projects directory)
cargo +nightly run --bin project-47-trait-specialization

# Return to stable for other projects
rustup override unset
cd ..
```

## 📈 Learning Progression

### **Recommended Timeline (3-4 months)**

#### **Phase 1: Foundation (Weeks 1-4)**
**Goal**: Master basic syntax, ownership, and simple applications

**Application Track**: Projects 1-20 (Easy)
- **Week 1**: Projects 1-10 (CLI tools, basic algorithms)
- **Week 2**: Projects 11-20 (Structs, file I/O, simple games)

**Language Track**: Projects 1-20 (Easy)
- **Week 3**: Projects 1-10 (Ownership, structs, enums)
- **Week 4**: Projects 11-20 (Traits, lifetimes, modules)

**Daily Routine**:
- 1-2 hours: Work on 1 application project (`cd application-projects`)
- 1-2 hours: Work on 1 language project (`cd language-projects`)
- 30 minutes: Review tests and documentation

#### **Phase 2: Intermediate (Weeks 5-8)**
**Goal**: Build complex applications and understand advanced ownership

**Application Track**: Projects 21-35 (Medium)
- **Week 5-6**: Projects 21-30 (Data structures, web servers, algorithms)
- **Week 7**: Projects 31-35 (Parallelism, advanced algorithms)

**Language Track**: Projects 21-35 (Medium)
- **Week 6-7**: Projects 21-30 (Concurrency, collections, interior mutability)
- **Week 8**: Projects 31-35 (Unsafe basics, macros, const generics)

#### **Phase 3: Advanced Mastery (Weeks 9-12)**
**Goal**: Systems programming and expert Rust features

**Application Track**: Projects 36-50 (Hard)
- **Week 9-10**: Projects 36-45 (Systems, concurrency, algorithms)
- **Week 11-12**: Projects 46-50 (Advanced systems, optimization)

**Language Track**: Projects 36-50 (Hard)
- **Week 10-11**: Projects 36-45 (Advanced unsafe, FFI, allocators)
- **Week 12**: Projects 46-50 (Metaprogramming, no_std, expert features)

### **Parallel Learning Path (Alternative)**
Work on both tracks simultaneously by difficulty:

**Week 1**: App 1-5 + Lang 1-5
**Week 2**: App 6-10 + Lang 6-10  
**Week 3**: App 11-15 + Lang 11-15
**...and so on

## 🎓 Detailed Project Lists

### **Application Projects Track (50 Projects)**

#### **Easy Tier (Projects 1-20): Basic Applications**
**Focus**: CLI tools, simple algorithms, basic data structures

| # | Project | Description | Key Concepts | Dependencies | Est. Time |
|---|---------|-------------|--------------|--------------|-----------|
| 1 | `project-01-hello-world` | Interactive greeting program | `std::io`, string handling | `anyhow` | 1h |
| 2 | `project-02-calculator` | Basic arithmetic calculator | Conditionals, parsing, errors | `anyhow` | 2h |
| 3 | `project-03-fizzbuzz` | Classic FizzBuzz implementation | Loops, pattern matching | None | 1h |
| 4 | `project-04-temp-converter` | Celsius ↔ Fahrenheit converter | Functions, floating-point math | None | 1h |
| 5 | `project-05-guess-number` | Simple number guessing game | `rand` crate, basic RNG | `rand`, `anyhow` | 2h |
| 6 | `project-06-ascii-art` | Generate ASCII patterns | Nested loops, string repetition | None | 2h |
| 7 | `project-07-prime-checker` | Prime number validation | Math optimization, loops | None | 2h |
| 8 | `project-08-word-counter` | Text analysis tool | String splitting, `Vec` usage | None | 2h |
| 9 | `project-09-todo-list` | In-memory task manager | `Vec` storage, ownership | None | 3h |
| 10 | `project-10-fibonacci` | Fibonacci sequence generator | Recursion vs iteration | None | 2h |
| 11 | `project-11-bank-account` | Banking simulation | Structs, methods, `Result` | `anyhow` | 3h |
| 12 | `project-12-rock-paper` | Rock-paper-scissors game | Enums, `rand`, `match` | `rand` | 3h |
| 13 | `project-13-file-word-count` | File-based word counter | `std::fs`, `HashMap` | `anyhow` | 3h |
| 14 | `project-14-rectangle-calc` | Geometry calculator | Traits, `Option` errors | `anyhow` | 2h |
| 15 | `project-15-json-parser` | Simple JSON parser | Enums, manual parsing | None | 4h |
| 16 | `project-16-password-validator` | Password strength checker | Custom errors, regex | `thiserror`, `regex?` | 3h |
| 17 | `project-17-maze-solver` | 3x3 maze solver | Recursion, backtracking | None | 4h |
| 18 | `project-18-book-library` | Book management CLI | Lifetimes, iterators | `anyhow` | 4h |
| 19 | `project-19-binary-converter` | Binary ↔ decimal converter | Bitwise operations | None | 3h |
| 20 | `project-20-tictactoe` | Console tic-tac-toe game | 2D arrays, win detection | None | 5h |

#### **Medium Tier (Projects 21-35): System Applications**
**Focus**: Data structures, networking, complex algorithms

| # | Project | Description | Key Concepts | Dependencies | Est. Time |
|---|---------|-------------|--------------|--------------|-----------|
| 21 | `project-21-generic-stack` | Generic stack implementation | Generics, traits | None | 3h |
| 22 | `project-22-grade-tracker` | Student grade management | `HashMap`, sorting | None | 3h |
| 23 | `project-23-todo-app` | Persistent CLI todo app | `serde`, file I/O | `serde`, `serde_json` | 6h |
| 24 | `project-24-http-server` | Basic HTTP server | Async, `hyper`/`tokio` | `hyper`, `tokio` | 8h |
| 25 | `project-25-graph-bfs` | Graph shortest path (BFS) | `HashMap`, `VecDeque` | None | 5h |
| 26 | `project-26-anagram-finder` | Anagram detection | String manipulation, `HashSet` | None | 4h |
| 27 | `project-27-lru-cache` | LRU cache implementation | Generics, `LinkedList` | None | 6h |
| 28 | `project-28-simple-db` | In-memory database | Traits, pattern matching | `serde` | 8h |
| 29 | `project-29-mandelbrot` | ASCII Mandelbrot fractal | Complex math, loops | None | 6h |
| 30 | `project-30-chat-sim` | Multi-threaded chat simulator | `std::thread`, channels | None | 5h |
| 31 | `project-31-parallel-sieve` | Parallel prime sieve | `rayon`, `Mutex` | `rayon` | 6h |
| 32 | `project-32-ray-tracer` | Basic ray tracer (PPM output) | Vector math, rendering | None | 10h |
| 33 | `project-33-blockchain-sim` | Simple blockchain simulator | `sha2`, proof-of-work | `sha2` | 8h |
| 34 | `project-34-web-scraper` | Concurrent web scraper | `reqwest`, `Arc<Mutex>` | `reqwest` | 7h |
| 35 | `project-35-sorting-viz` | Sorting algorithm visualizer | Recursive algorithms, generics | None | 8h |

#### **Hard Tier (Projects 36-50): Advanced Systems**
**Focus**: Systems programming, high-performance computing, complex algorithms

| # | Project | Description | Key Concepts | Dependencies | Est. Time |
|---|---------|-------------|--------------|--------------|-----------|
| 36 | `project-36-memory-allocator` | Toy memory allocator | Unsafe Rust, pointers | None | 12h |
| 37 | `project-37-mini-interpreter` | Expression language interpreter | Recursive descent parsing | `nom?` | 15h |
| 38 | `project-38-game-of-life` | Conway's Game of Life | Cellular automata, optimization | None | 10h |
| 39 | `project-39-dist-kv-store` | Distributed key-value store | Channels, fake networking | `tokio` | 12h |
| 40 | `project-40-compiler-frontend` | Tiny language compiler frontend | Parsing combinators, lifetimes | `nom` | 20h |
| 41 | `project-41-lockfree-queue` | Lock-free concurrent queue | Atomics, unsafe CAS | None | 15h |
| 42 | `project-42-neural-net` | Simple neural network (XOR) | Matrix operations, backpropagation | `ndarray?` | 18h |
| 43 | `project-43-os-kernel` | Toy OS kernel module | No_std, bootloader | `bootloader` | 25h |
| 44 | `project-44-sat-solver` | 3-SAT solver | Backtracking, DPLL algorithm | None | 15h |
| 45 | `project-45-gpu-simulator` | GPU compute simulator | SIMD, parallel iteration | None | 12h |
| 46 | `project-46-proof-verifier` | Lambda calculus type checker | Type theory, recursive validation | None | 20h |
| 47 | `project-47-rt-scheduler` | Real-time task scheduler | Priority queues, threads | `tokio` | 15h |
| 48 | `project-48-aes-cipher` | AES block cipher implementation | Bit manipulation, crypto math | None | 25h |
| 49 | `project-49-dynamic-pathfinding` | A* pathfinding with dynamic weights | Heuristics, priority queues | None | 12h |
| 50 | `project-50-quantum-sim` | Basic quantum circuit simulator | Linear algebra, complex numbers | `nalgebra?` | 20h |

### **Language Mastery Projects Track (50 Projects)**

#### **Easy Tier (Projects 1-20): Core Language Concepts**
**Focus**: Ownership, basic types, modules, and simple traits

| # | Project | Description | Key Rust Feature | Dependencies | Est. Time |
|---|---------|-------------|------------------|--------------|-----------|
| 1 | `project-01-personal-greeter` | Interactive name/age greeter | Basic ownership, `std::io` | `anyhow` | 1h |
| 2 | `project-02-math-quiz` | Random math quiz game | Loops, conditionals, ownership | `rand` | 2h |
| 3 | `project-03-string-manipulator` | String reversal and case conversion | String methods, ownership transfer | None | 1h |
| 4 | `project-04-email-validator` | Basic email format validation | Pattern matching, `&str` borrowing | None | 2h |
| 5 | `project-05-number-guesser` | Number guessing with hints | Enums, `rand::thread_rng()` | `rand` | 2h |
| 6 | `project-06-list-printer` | Dynamic list with reverse printing | `Vec`, iteration, borrowing | None | 2h |
| 7 | `project-07-error-handler` | Division with error handling | `Result`, `?` operator, `parse()` | `anyhow` | 2h |
| 8 | `project-08-person-struct` | Person struct with field access | Structs, `#[derive(Debug)]` | None | 1h |
| 9 | `project-09-person-methods` | Person methods (greeting, age check) | `impl`, `&self` borrowing | None | 2h |
| 10 | `project-10-enum-calculator` | Enum-based calculator operations | Enums, `match` expressions | None | 2h |
| 11 | `project-11-shape-drawer` | Circle/Rectangle with area trait | Traits, polymorphism | None | 3h |
| 12 | `project-12-inventory-manager` | Item inventory with enum variants | Enums with data, pattern matching | None | 3h |
| 13 | `project-13-file-reader` | File line numbering | `BufReader`, iterator borrowing | `anyhow` | 3h |
| 14 | `project-14-custom-error` | Custom error enum for parsing | Error enums, `thiserror` | `thiserror` | 3h |
| 15 | `project-15-generic-container` | Generic wrapper type | Generics, trait bounds | None | 3h |
| 16 | `project-16-module-organizer` | Modular greeter/validator | Modules, `pub`, visibility | None | 2h |
| 17 | `project-17-lifetime-basics` | String substring extraction | Lifetimes, `&'a str` | None | 3h |
| 18 | `project-18-iterator-adapter` | Custom iterator (skip evens) | `Iterator` trait, `next()` | None | 4h |
| 19 | `project-19-trait-object` | Dynamic shape collection | Trait objects, `dyn`, `Box` | None | 4h |
| 20 | `project-20-associated-types` | Generic container trait | Associated types, generic impls | None | 4h |

#### **Medium Tier (Projects 21-35): Advanced Ownership & Concurrency**
**Focus**: Smart pointers, concurrency primitives, advanced type system

| # | Project | Description | Key Rust Feature | Dependencies | Est. Time |
|---|---------|-------------|------------------|--------------|-----------|
| 21 | `project-21-rc-arc-sharing` | Shared counter with multiple owners | `Rc<T>`, `Arc<T>`, `RefCell` | None | 3h |
| 22 | `project-22-mutex-guard` | Thread-safe vector appending | `Mutex`, `Guard`, RAII | None | 4h |
| 23 | `project-23-channel-messaging` | Thread communication via channels | `mpsc` channels, `send`/`recv` | None | 4h |
| 24 | `project-24-async-hello` | Async message printing with delays | `async`/`await`, `#[tokio::main]` | `tokio` | 3h |
| 25 | `project-25-borrow-challenge` | Field borrowing without struct move | `&mut` borrowing, reborrowing | None | 4h |
| 26 | `project-26-cell-usage` | Interior mutability counter | `Cell<T>`, `RefCell<T>`, borrowing | None | 3h |
| 27 | `project-27-hashmap-custom-key` | Custom struct as HashMap key | `Hash`, `Eq` traits, derive macros | None | 4h |
| 28 | `project-28-btree-ordering` | Custom ordering for BTreeMap | `Ord` impl, custom comparators | None | 4h |
| 29 | `project-29-phantomdata-trick` | Type ownership without storage | `PhantomData<T>`, variance | None | 5h |
| 30 | `project-30-drop-trait` | RAII cleanup with custom drop | `Drop` trait, cleanup patterns | None | 3h |
| 31 | `project-31-unsafe-pointer` | Manual vector growth with pointers | `unsafe`, raw pointers, `std::ptr` | None | 6h |
| 32 | `project-32-ffi-call` | Call C printf from Rust | FFI, `libc`, `CString` | `libc` | 4h |
| 33 | `project-33-macro-basics` | Declarative macro for struct getters | `macro_rules!`, token trees | None | 5h |
| 34 | `project-34-procedural-macro` | Derive macro for simple serialization | Proc macros, `TokenStream` | `proc-macro2`, `syn`, `quote` | 8h |
| 35 | `project-35-const-generics` | Fixed-size array wrapper | Const generics, `const N: usize` | None | 5h |

#### **Hard Tier (Projects 36-50): Expert Language Features**
**Focus**: Systems programming, metaprogramming, advanced type theory

| # | Project | Description | Key Rust Feature | Dependencies | Est. Time |
|---|---------|-------------|------------------|--------------|-----------|
| 36 | `project-36-associated-constants` | Trait with compile-time constants | Associated constants, statics | None | 4h |
| 37 | `project-37-variance-control` | Covariant reference counting | `PhantomData` variance, subtyping | None | 6h |
| 38 | `project-38-send-sync-bounds` | Thread-safe generic logger | `Send + Sync` bounds, markers | None | 5h |
| 39 | `project-39-pin-unpin` | Self-referential async structure | `Pin`, `Unpin`, future polling | `tokio` | 8h |
| 40 | `project-40-try-operator` | Custom iterator with early exit | `try` blocks, `Result` propagation | None | 5h |
| 41 | `project-41-inline-asm` | Inline assembly for simple operations | `asm!` macro, registers (nightly) | None | 6h |
| 42 | `project-42-custom-allocator` | Global slab allocator | `#[global_allocator]`, `Alloc` trait | None | 12h |
| 43 | `project-43-no-std-env` | No_std hello world with panic handler | `#![no_std]`, `core` only | None | 6h |
| 44 | `project-44-build-script` | Code generation via build.rs | Build scripts, `cargo:rerun-if-changed` | None | 5h |
| 45 | `project-45-dynamic-loading` | Runtime shared library loading | `libloading`, dynamic dispatch | `libloading` | 7h |
| 46 | `project-46-coroutine-sim` | Stackful coroutine simulator | Generators, `yield` (nightly) | `tokio` | 10h |
| 47 | `project-47-trait-specialization` | Overlapping trait implementations | `#[specialization]`, default methods (nightly) | None | 8h |
| 48 | `project-48-const-trait-bounds` | Compile-time generic evaluation | Const contexts, `const fn` (nightly) | None | 7h |
| 49 | `project-49-ffi-callback` | Rust callback for C qsort | `extern "C" fn`, FFI callbacks | `libc` | 6h |
| 50 | `project-50-meta-dsl` | Macro-based DSL parser generator | Advanced `macro_rules!`, hygiene | `syn`, `quote` | 15h |

## 🛠 Development Environment

### **IDE Setup**

#### **RustRover (Recommended)**
1. **Download**: [JetBrains RustRover](https://www.jetbrains.com/rust/)
2. **Open Workspaces Separately**:
    - `File → Open → application-projects/` (for Application track)
    - `File → Open → language-projects/` (for Language track)
    - Or use "Attach to Existing Project" to work on both
3. **Toolchain**: `File → Settings → Rust → Toolchain` → Use system Rust
4. **Nightly Setup**:
    - `File → Settings → Rust → Toolchain` → Add nightly toolchain
    - For language projects: `rustup override set nightly` in project directory

**RustRover Features Used**:
- **Workspace Navigation**: View each track independently
- **Run Configurations**: Auto-generated for each binary in active workspace
- **Test Runner**: Comprehensive test execution with coverage
- **Macro Expansion**: Debug macro definitions (language track)
- **Unsafe Code Analysis**: Highlights and safety warnings
- **Cargo Integration**: Workspace dependency management

#### **VS Code Alternative**
**Extensions**:
- `rust-analyzer` (essential)
- `Error Lens` (inline error display)
- `crates` (dependency management)
- `Even Better TOML` (Cargo.toml support)

**Configuration** (`settings.json`):
```json
{
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.server.extraEnv": {
        "RUSTUP_TOOLCHAIN": "stable"
    },
    "editor.formatOnSave": true,
    "files.trimTrailingWhitespace": true
}
```

**Workspace Setup**: Open each track as separate VS Code workspace or use multi-root workspace.

### **Terminal Commands**

#### **Workspace Management**
```bash
# === Application Track ===
cd application-projects
cargo check                    # Check all Application projects
cargo test --workspace         # Test all Application projects
cargo update --workspace       # Update Application dependencies
cargo clean --workspace        # Clean Application build artifacts
cd ..

# === Language Track ===
cd language-projects
cargo check                    # Check all Language projects
cargo test --workspace         # Test all Language projects  
cargo update --workspace       # Update Language dependencies
cargo clean --workspace        # Clean Language build artifacts
cd ..
```

#### **Project-Specific Commands**
```bash
# === From Application Workspace ===
cd application-projects
cargo run --bin project-01-hello-world
cargo run --bin project-23-todo-app -- add "Task" --priority high
cargo test --bin project-01-hello-world
cd ..

# === From Language Workspace ===
cd language-projects
cargo run --bin project-01-personal-greeter
cargo test --bin project-11-shape-drawer -- --nocapture
cd ..
```

#### **Nightly Features (Language Track)**
```bash
# Enable nightly for specific directory (from root)
cd language-projects/hard
rustup override set nightly

# Run with unstable features (from language-projects)
cd ..
cargo +nightly run --bin project-47-trait-specialization

# Check feature compatibility
cargo +nightly check --bin project-47-trait-specialization

# Return to stable for other projects
rustup override unset
cd ..
```

### **Testing Strategy**

#### **Unit Tests** (Every Project)
All projects include comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        // Core algorithm correctness
    }
    
    #[test]
    fn test_edge_cases() {
        // Boundary conditions, errors
    }
    
    #[test]
    fn test_ownership_rules() -> Result<(), Box<dyn std::error::Error>> {
        // Language track: Verify no unexpected moves
        Ok(())
    }
    
    #[test]
    #[should_panic(expected = "specific error message")]
    fn test_error_conditions() {
        // Error handling verification
    }
}
```

#### **Integration Tests** (Complex Projects)
Located in `tests/integration.rs`:

```rust
// tests/integration.rs
use project_name::{App, Result};

#[test]
fn test_full_application_flow() -> Result<()> {
    let mut app = App::new()?;
    
    // Simulate complete user interaction
    app.process_input("add task")?;
    app.process_input("list")?;
    app.process_input("quit")?;
    
    Ok(())
}
```

#### **Benchmark Tests** (Performance Projects)
Add to `Cargo.toml`:
```toml
[[bench]]
name = "benchmarks"
harness = false

[dev-dependencies]
criterion = "0.5"
```

```rust
// benches/benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_name::algorithm;

pub fn bench_main_algorithm(c: &mut Criterion) {
    c.bench_function("main_algorithm", |b| {
        b.iter(|| algorithm(black_box(1000)));
    });
}

criterion_group!(benches, bench_main_algorithm);
criterion_main!(benches);
```

**Run Benchmarks** (from appropriate workspace):
```bash
cd application-projects  # or language-projects
cargo bench --bin project-31-parallel-sieve
```

## 📖 Documentation Standards

### **Project README Template**
Each project includes a detailed `README.md`:

```markdown
# Project 01: Hello World with Input

## 🎯 Learning Objectives

### Application Track Goals
- Master basic `std::io` operations
- Understand string ownership and borrowing
- Implement proper error handling with `anyhow`
- Practice Cargo project structure

### Key Rust Concepts Demonstrated
- **Ownership**: `String` vs `&str` usage
- **Error Handling**: `Result<T, E>` propagation with `?`
- **I/O Safety**: Proper stdin/stdout buffer management

## 🏗 Project Structure
```
project-01-hello-world/
├── Cargo.toml          # Dependencies and metadata
├── README.md          # This documentation
├── src/
│   └── main.rs        # Main application logic
└── tests/
└── integration.rs # End-to-end tests
```

## 🚀 Usage

### Running the Application
```bash
# From application-projects workspace
cd application-projects
cargo run --bin project-01-hello-world
# Output:
# Hello, World! 👋
# What's your name? Alice
# Nice to meet you, Alice!
```

### Command Line Arguments
```bash
# No arguments required for this project
cargo run --bin project-01-hello-world -- --help  # Shows usage information
```

## 🧪 Testing

### Unit Tests
```bash
# From application-projects workspace
cargo test --bin project-01-hello-world
# Running 4 tests
# test tests::test_string_ownership ... ok
# test tests::test_error_handling ... ok  
# test tests::test_user_interaction ... ok
# test tests::test_borrowing_rules ... ok
```

### Integration Tests
```bash
cargo test --test integration --bin project-01-hello-world
# Tests complete application flow
```

### Coverage (requires cargo-llvm-cov)
```bash
cargo install cargo-llvm-cov
cargo llvm-cov --html --bin project-01-hello-world
# Opens coverage report in browser
```

## 🔍 Code Walkthrough

### Main Function
```rust
fn main() -> Result<()> {
    // 1. Basic output with ownership
    println!("Hello, World! 👋");
    
    // 2. Input collection with buffer management
    let mut name = String::new();
    print!("What's your name? ");
    io::stdout().flush()?;  // Ensure prompt appears
    
    // 3. Safe reading with error propagation
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();  // Ownership transfer
    
    // 4. String formatting and output
    println!("Nice to meet you, {}!", name);
    Ok(())
}
```

### Key Implementation Details

#### Ownership Patterns
- **Input Buffer**: `String::new()` creates owned buffer
- **Trimming**: `trim()` borrows, `to_string()` creates new ownership
- **Error Propagation**: `?` operator for clean error handling

#### Error Handling
- **I/O Errors**: `read_line()` returns `Result<usize, Error>`
- **Parsing**: Not applicable in this simple example
- **Custom Errors**: Handled by `anyhow::Result` for simplicity

## 📈 Performance Considerations
- **I/O Bound**: Primary bottleneck is stdin reading
- **Memory**: Minimal allocation (single `String` buffer)
- **CPU**: Negligible computation

## 🔗 Related Projects
- **Next**: [Project 02: Calculator](project-02-calculator) - Adds parsing and error handling
- **Similar**: [Language Project 01: Personal Greeter](../language-projects/easy/project-01-personal-greeter) - Focuses on ownership details

## 🤝 Contributing
1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open Pull Request

## 📄 License
This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## 🙏 Acknowledgments
- [Rust Programming Language](https://www.rust-lang.org/) - Amazing language and community
- [Rust Book](https://doc.rust-lang.org/book/) - Essential learning resource
- [Cargo Documentation](https://doc.rust-lang.org/cargo/) - Build system excellence
```

### **Documentation Generation**
```bash
# === Application Track Documentation ===
cd application-projects
cargo doc --workspace --no-deps --open
cargo doc --bin project-01-hello-world --open

# === Language Track Documentation ===
cd ../language-projects
cargo doc --workspace --no-deps --open
cargo doc --bin project-01-personal-greeter --open

# Include private items (for learning)
cd ..
RUSTDOCFLAGS="--document-private-items" cargo doc --manifest-path application-projects/Cargo.toml
```

## 🚨 Troubleshooting

### **Common Issues**

#### **Cargo Workspace Problems**
```
Error: no such subcommand: `project-01-hello-world`
```
**Solution**: Ensure you're in the correct workspace directory:
```bash
cd application-projects  # or language-projects
cargo run --bin project-01-hello-world
```

#### **Dependency Resolution**
```
error[E0432]: unresolved import `some_crate`
```
**Solution**: Update workspace dependencies (from appropriate workspace):
```bash
cd application-projects  # or language-projects
cargo update --workspace
cd ..
# Or add missing dependency to project Cargo.toml
```

#### **Nightly Feature Errors**
```
error[E0658]: use of unstable library feature
```
**Solution**: Use nightly toolchain (from language-projects workspace):
```bash
cd language-projects/hard/project-47-trait-specialization
rustup override set nightly
cd ../..
cargo +nightly check --bin project-47-trait-specialization
```

#### **Test Timeouts (Concurrency Projects)**
```
test has been running for over 60 seconds
```
**Solution**: Increase timeout or fix race conditions (from appropriate workspace):
```toml
# In Cargo.toml
[dev-dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
```

Add to tests:
```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_operations() {
    // Test logic
}
```

#### **Memory Leaks (Advanced Projects)**
**Use valgrind** (Linux/macOS) from appropriate workspace:
```bash
cd application-projects  # or language-projects
RUSTFLAGS="-g" cargo build --release --bin project-36-memory-allocator
valgrind --leak-check=full --show-leak-kinds=all ./target/release/project-36-memory-allocator
```

**Rust-specific**: Use `cargo-miri` for undefined behavior detection:
```bash
cargo install cargo-miri
cd language-projects
cargo miri run --bin project-31-unsafe-pointer
```

### **Platform-Specific Issues**

#### **Windows**
- **Path Issues**: Use `\\` or raw strings `r"\"`
- **Threading**: Windows threads behave differently; test on Linux if possible
- **FFI**: Ensure `msvc` toolchain: `rustup default stable-msvc`

#### **macOS**
- **Linking**: May need `brew install openssl` for some crates
- **Async**: `tokio` works well, but test thread pool sizes
- **No_std**: ARM targets may require additional setup

#### **Linux**
- **Dependencies**: Install system libraries:
  ```bash
  # Ubuntu/Debian
  sudo apt install build-essential pkg-config libssl-dev
  
  # For FFI projects
  sudo apt install libc6-dev gcc
  ```
- **Performance**: Best testing environment for concurrency projects

## 🎉 Completion & Certification

### **Project Completion Checklist**
For each project, verify:

- [ ] **Code Compiles**: `cargo check` passes (from appropriate workspace)
- [ ] **Tests Pass**: `cargo test` with 100% pass rate
- [ ] **Documentation**: README.md complete with examples
- [ ] **Performance**: Benchmarks reasonable (where applicable)
- [ ] **Error Handling**: All edge cases covered
- [ ] **Ownership**: No borrow checker errors
- [ ] **Safety**: `unsafe` blocks properly documented (language track)

### **Track Completion Badges**
Add to your GitHub profile README:

```markdown
### Rust 100 Projects Completion

[![Rust Application Master](https://img.shields.io/badge/Rust-Application%20Master-00d4aa)](https://github.com/yourusername/rust-100-projects/tree/main/application-projects)
[![Rust Language Expert](https://img.shields.io/badge/Rust-Language%20Expert-d4aa00)](https://github.com/yourusername/rust-100-projects/tree/main/language-projects)
[![Rust 100 Projects](https://img.shields.io/badge/Rust-100%20Projects%20Complete-000000)](https://github.com/yourusername/rust-100-projects)

**Completed**: 100/100 Rust projects (Application + Language tracks)
- **Application Track**: 50 projects (CLI tools → OS kernels)
- **Language Track**: 50 projects (Ownership → Metaprogramming)
- **Total Time**: ~300 hours of focused Rust development
- **Skills Mastered**: Ownership, concurrency, unsafe, FFI, systems programming
```

### **Portfolio Integration**
**GitHub Profile README**:
```markdown
# Rust Developer

## 🚀 100 Rust Projects Mastered

I completed [Rust 100 Projects](https://github.com/yourusername/rust-100-projects), a comprehensive learning journey covering:

### Application Development (50 Projects)
- **CLI Tools**: Todo apps, calculators, file processors
- **Games**: Tic-tac-toe, rock-paper-scissors, maze solvers  
- **Web Services**: HTTP servers, web scrapers
- **Algorithms**: Sorting visualizers, pathfinding, SAT solvers
- **Systems**: Memory allocators, OS kernels, quantum simulators

### Language Mastery (50 Projects)
- **Core Concepts**: Ownership, borrowing, lifetimes, structs/enums
- **Advanced Types**: Traits, generics, const generics, associated types
- **Concurrency**: Threads, async/await, channels, lock-free data structures
- **Systems Programming**: Unsafe Rust, FFI, no_std, custom allocators
- **Metaprogramming**: Macros, build scripts, procedural macros

## 🛠 Technologies
![Rust](https://img.shields.io/badge/Rust-1.75%2B-informational)
![Cargo](https://img.shields.io/badge/Cargo-Two%20Workspaces-orange)
![Tokio](https://img.shields.io/badge/Tokio-Async-blue)
![Rayon](https://img.shields.io/badge/Rayon-Parallel-green)

## 📊 Stats
- **Projects**: 100 completed
- **Lines of Code**: ~50,000+ (estimated)
- **Test Coverage**: 90%+ across all projects
- **Learning Duration**: 3-4 months intensive study

---
⭐ Star [Rust 100 Projects](https://github.com/yourusername/rust-100-projects) if you found it helpful!
```

### **Job Application Highlights**
When applying for Rust positions, mention:

> "I completed 100 comprehensive Rust projects covering both application development and deep language understanding. My portfolio includes everything from CLI tools and web servers to custom memory allocators and OS kernel modules. I have hands-on experience with async Rust, unsafe code, FFI, and advanced metaprogramming techniques."

## 🤝 Contributing & Community

### **How to Contribute**
1. **Fork** the repository
2. **Create Issue** for bugs or feature requests
3. **Submit Pull Request** with:
    - Clear description of changes
    - Updated tests
    - Documentation updates
    - Follow existing code style

### **Code Style**
- **Formatter**: `cargo fmt` (stable Rust format)
- **Linter**: `cargo clippy` (catch common mistakes)
- **Documentation**: `cargo doc` (/// doc comments)
- **Testing**: 80%+ coverage, comprehensive edge cases

### **Community Standards**
- **Inclusive**: Follow Rust community's Code of Conduct
- **Helpful**: Document solutions to common issues
- **Educational**: Explain *why* solutions work
- **Performance**: Consider efficiency where appropriate

### **Support Channels**
- **Rust Discord**: #projects channel for help
- **Rust Forum**: [users.rust-lang.org](https://users.rust-lang.org)
- **Stack Overflow**: rust tag for specific questions
- **Issues**: Use GitHub Issues for project-specific problems

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

```
MIT License

Copyright (c) 2024 [Your Name]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 🙏 Acknowledgments

### **Learning Resources Used**
- **[The Rust Programming Language](https://doc.rust-lang.org/book/)** - Core concepts
- **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** - Practical examples
- **[Rust Reference](https://doc.rust-lang.org/reference/)** - Advanced features
- **[Cargo Book](https://doc.rust-lang.org/cargo/)** - Build system
- **[Async Book](https://rust-lang.github.io/async-book/)** - Concurrency
- **[Nom Parsing](https://docs.rs/nom/latest/nom/)** - Parser combinators
- **[Tokio Documentation](https://docs.rs/tokio/latest/tokio/)** - Async runtime

### **Tools & Crates**
- **Core**: `std`, `core` (no_std projects)
- **Essentials**: `anyhow`, `thiserror`, `serde`, `tokio`
- **Testing**: `criterion`, `cargo-llvm-cov`, `miri`
- **Parsing**: `nom`, `syn`, `quote`
- **Networking**: `hyper`, `reqwest`, `tokio-tungstenite`
- **Systems**: `bootloader`, `libloading`, `libc`

### **Inspiration**
- **Exercism Rust Track** - Interactive learning
- **Advent of Code** - Algorithmic challenges in Rust
- **Rustlings** - Hands-on exercises
- **100 Exercises To Learn Rust** - Language-focused practice

---

## 🎊 Final Words

Completing these 100 projects represents a **comprehensive mastery of Rust programming**. You'll emerge with:

- **Practical Skills**: Ability to build production-ready Rust applications
- **Deep Understanding**: Knowledge of Rust's unique safety guarantees
- **Systems Expertise**: Experience with low-level programming and optimization
- **Portfolio Ready**: 100 concrete projects to showcase your abilities

**Start with Project 1 from each track today** - navigate to `application-projects` and `language-projects` workspaces and begin with `cargo run`! 🚀

---

*Built with ❤️ for the Rust community. Contributions welcome!*