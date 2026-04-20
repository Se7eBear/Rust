<div align="center">
  <img src="https://raw.githubusercontent.com/rust-lang/rust-artwork/master/logo/rust-logo-512x512.png" width="120" alt="Rust Logo" />
  <h1>🦀 Rust Cursus 42 — Engineering Portfolio</h1>
  <p><i>Mastering memory safety, concurrency, and low-level systems through the rigorous 42 methodology.</i></p>
</div>

---

## 🛠️ Tech Stack & Environment

This repository is developed in a dedicated **Linux Ubuntu** environment, focusing on system optimization and the native Rust toolchain.

<p align="left">
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=rust,c,ubuntu,git,github,vscode" />
  </a>
</p>

* **Focus:** Logic, memory management, and high-performance software engineering.
* **Toolchain:** Cargo Workspaces for granular crate management and testing.

---

## 📜 The "Norma Rustacean"

To ensure code quality and safety, every project follows these mandatory rules (The Norm):

* **Zero Warnings:** Every crate must compile cleanly via `cargo build`.
* **Safe Error Handling:** No `unwrap()` allowed; all errors are handled explicitly with `Result` and `Option`.
* **Function Modularity:** Maximum of **30 lines** per function to guarantee readability.
* **Strict Linting:** Must pass `cargo clippy` without any warnings before completion.

---

## 📊 Roadmap & Progress

| Rank | Status | Core Focus | Key Projects |
| :---: | :---: | :--- | :--- |
| **00** | ✅ | **Toolchain** | `hello_rust`, `ft_print_numbers`, `ft_args` |
| **01** | ✅ | **Memory & Strings** | `ft_swap`, `ft_strlen`, `ft_atoi`, `ft_hello_macro` |
| **02** | 🛠️ | **Generics & I/O** | `ft_collections`, `ft_matrix`, `ft_map_macro` |
| **03** | 🔒 | **Advanced Traits** | `ft_error`, `ft_linkedlist`, `ft_derive_display` |
| **04** | 🔒 | **Concurrency** | `ft_philosophers`, `ft_channel`, `ft_async_fetch` |
| **05** | 🔒 | **Final Projects** | `ft_minigrep_pro`, `ft_web_api` |

---

## 🚀 Deep Dive: The Ranks

### 🔹 Rank 02 — Generics & Algebra
Moving beyond basic types, this rank focuses on **abstract data types** and operator overloading.
* **ft_collections:** Building generic Stacks and Queues from scratch using `Option<T>`.
* **ft_matrix:** Implementing linear algebra with operator overloading (`Add`, `Sub`, `Mul`) for custom structs.
* **ft_builder_macro:** A declarative macro that auto-generates the "Builder Pattern" for any struct.

### 🔹 Rank 03 — Ownership & Custom Traits
Exploring the "hidden" parts of Rust, such as heap allocation and manual trait implementation.
* **ft_error:** Creating a robust error system implementing `std::error::Error` and `From` traits.
* **ft_linkedlist:** The ultimate "Borrow Checker" boss; managing recursive types with `Box<T>`.
* **Proc-Macros:** Developing `#[derive(FtDisplay)]` to generate code at compile-time.

### 🔹 Rank 04 — Parallelism & Async
Solving the hardest problems in systems programming: **Concurrency without Data Races**.
* **ft_philosophers:** A thread-safe solution to the Dining Philosophers problem using `Arc` and `Mutex`.
* **ft_channel:** Building a custom Worker Pool using multi-producer, single-consumer (mpsc) channels.
* **ft_async_fetch:** High-speed parallel I/O using the `Tokio` runtime and `async/await`.

### 🔹 Rank 05 — Professional Portfolio
Final-tier projects designed to showcase production-ready engineering skills.
* **ft_minigrep_pro:** A high-performance search utility with Regex support and parallel directory walking.
* **ft_web_api:** A complete REST API with Axum, SQLite persistence, and JWT authentication.

---

## ⚙️ Usage

To execute a specific exercise from the root:
```bash
cargo run -p ft_strlen
