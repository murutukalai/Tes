Here’s a simple Rust program to generate a random alphabet code of a specified length:

```rust
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn generate_random_code(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .filter(|c| c.is_ascii_alphabetic()) // Only keep alphabetic characters
        .take(length)
        .map(char::from)
        .collect()
}

fn main() {
    let code_length = 10; // Specify the desired length of the code
    let random_code = generate_random_code(code_length);
    println!("Generated random alphabet code: {}", random_code);
}
```

### Explanation:
1. **`rand` crate**: This program uses the `rand` crate for generating random values. Add it to your `Cargo.toml` file:
   ```toml
   [dependencies]
   rand = "0.8"
   ```

2. **Functionality**:
   - `thread_rng` creates a random number generator.
   - `.sample_iter(&Alphanumeric)` generates an iterator of alphanumeric characters.
   - `.filter(|c| c.is_ascii_alphabetic())` filters only alphabetic characters (letters).
   - `.take(length)` limits the number of characters to the specified length.
   - `.map(char::from)` converts the result to `char`.
   - `.collect()` creates a `String` from the characters.

3. **Output**: It generates a random string with only alphabetic characters (e.g., `AbcDefGhIJ`).

Run the code, and it will print a random alphabet code each time! Let me know if you need further clarification.
