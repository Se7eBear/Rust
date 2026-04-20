<div align="center">
  <img src="https://raw.githubusercontent.com/rust-lang/rust-artwork/master/logo/rust-logo-512x512.png" width="120" alt="Rust Logo" />
  <h1>🦀 Rust Cursus 42 — Engineering Portfolio</h1>
  <p><i>Reimplementing low-level systems and algorithms from scratch to master memory safety and performance.</i></p>
</div>

---

## 🛠️ Tech Stack & Environment

This repository is developed in a dedicated **Linux Ubuntu** environment, focusing on system optimization and the standard 42 logic.

<p align="left">
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=rust,c,ubuntu,git,github,vscode" />
  </a>
</p>

* **Focus:** Logic, memory safety, and low-level software engineering.
* **Toolchain:** Cargo Workspaces for multi-crate management.

---

## 📜 The "Norma Rustacean"

Following the 42 School tradition, all projects must strictly adhere to these mandatory rules:

* **Zero Warnings:** Every project must compile cleanly via `cargo build`.
* **Safety First:** No `unwrap()` allowed; all errors are handled explicitly with `match`, `if let`, or the `?` operator.
* **Function Limits:** Maximum of **30 lines** per function to ensure modularity.
* **Quality Control:** Must pass `cargo clippy` without any warnings before final submission.

---

## 📊 Project Progress & Roadmap

| Rank | Ex | Project Name | Key Concepts | Status |
| :---: | :---: | :--- | :--- | :---: |
| **00** | 00 | `hello_rust` | Cargo toolchain and basic output. | ✅ |
| **00** | 01 | `ft_print_numbers` | ASCII arithmetic and character casting. | ✅ |
| **00** | 02 | `ft_print_alphabets` | Byte literals and loop iterations. | ✅ |
| **00** | 03 | `ft_args` | CLI argument processing and iterators. | ✅ |
| **01** | 00 | `ft_swap` | Mutable references and dereferencing (`*`). | ✅ |
| **01** | 01 | `ft_strlen` | Manual string manipulation (length, case, capitalize). | ✅ |
| **01** | 02 | `ft_atoi` | String to integer conversion handling signals/overflow. | ⏳ |
| **02** | 00 | `ft_collections` | Generics and custom Stack/Queue implementations. | 🔒 |

---

## 🚀 Technical Highlights

### Rank 01 — Deep Memory & Strings
In this rank, the challenge is to manage **UTF-8** strings and memory without using the "magic" methods of the standard library.

* **Manual Iteration:** Using `.bytes()` and `.chars()` to handle string data directly.
* **Ownership Mastery:** Understanding the move/copy semantics of different data types.
* **Test Driven:** Unit tests integrated within each crate to ensure compliance.

---

## ⚙️ How to Run

To execute a specific exercise from the root directory:

```bash
# Example: Running the strlen tester
cargo run -p tester
