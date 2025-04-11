## 🟢 Beginner-Level

### 📌 Language Fundamentals

| Concept | Summary | Example |
|--------|---------|---------|
| `String` vs `&str` | `String` = heap, growable. `&str` = slice. | `let s: String = "Hi".to_string();`<br>`let slice: &str = &s;` |
| Ownership | One owner at a time. Dropped when out of scope. | `let a = String::from("hi"); let b = a;` |
| Borrowing | Reference without ownership. Immutable `&T`, mutable `&mut T`. | `fn len(s: &String) -> usize { s.len() }` |
| `Result<T, E>` | For recoverable errors. | `Result<i32, String>` |
| `panic!()` vs Error | `panic!()` crashes; returning error gives control. | `Err("fail".into())` |
| Lifetimes | Tracks how long refs are valid. Use when compiler can’t infer. | `fn foo<'a>(x: &'a str) -> &'a str` |
| Traits | Define shared behavior (like interfaces). | `trait Speak { fn speak(&self); }` |
| Enums & Match | Combines data and behavior. Use `match`. | `enum Msg { Quit, Move{x: i32} }` |

### 📌 Memory Safety

| Concept | Summary | Example |
|--------|---------|---------|
| Null Safety | No null; use `Option<T>`. | `let val: Option<i32> = Some(5);` |
| Move Semantics | Ownership transfers on assignment. | `let b = a; // a is invalid` |
| `Copy` vs `Clone` | `Copy` = shallow, auto. `Clone` = deep, manual. | `let y = x; let b = a.clone();` |

---

## 🟡 Intermediate-Level

### 📌 Traits & Error Handling

| Concept | Summary | Example |
|--------|---------|---------|
| Custom Trait | Implement for struct. | `impl Trait for Struct {}` |
| `?` vs `unwrap()` | `?` = propagate, `unwrap()` = crash. | `fs::read("f.txt")?;` |
| Multiple Error Types | Use `Box<dyn Error>` or enums. | `Result<(), Box<dyn Error>>` |
| Trait Objects | For dynamic dispatch. | `Box<dyn Trait>` |

### 📌 Async & Concurrency

| Concept | Summary | Example |
|--------|---------|---------|
| `async`/`await` | `async fn` returns `Future`. | `let val = say().await;` |
| `Send`/`Sync` | Traits for thread safety. | Needed for concurrency. |
| `tokio` Runtime | Async executor for IO, timers, etc. | `#[tokio::main] async fn main()` |

---

## 🟠 Advanced-Level

### 📌 Lifetimes & Generics

| Concept | Summary | Example |
|--------|---------|---------|
| Lifetime Elision | Compiler infers lifetimes. | fn(x: &str) -> &str |
| Explicit Lifetimes | Needed for multiple refs. | `fn foo<'a>(x: &'a str) -> &'a str` |
| HRTB (`for<'a>`) | Trait must work for all lifetimes. | `F: for<'a> Fn(&'a str)` |

### 📌 Unsafe Rust

| Concept | Summary | Example |
|--------|---------|---------|
| Why `unsafe` | Required for raw pointers, FFI, etc. | `unsafe { *ptr }` |
| Raw Pointers | Manual memory access. | `let ptr = &x as *const i32;` |
| Lost Guarantees | Manual safety responsibility. | Aliasing, bounds, etc. |

### 📌 Performance & Compilation

| Concept | Summary | Example |
|--------|---------|---------|
| Compile Optimizations | Inlining, monomorphization, LTO. | Enabled in release builds. |
| Zero-Cost Abstractions | High-level = low-cost. | Iterators, traits. |
| Binary Size Reduction | `--release`, LTO, strip. | `RUSTFLAGS="-C link-arg=-s"` |

---
