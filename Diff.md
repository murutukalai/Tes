While there isn't a dedicated "HTML diff" library in Rust's ecosystem like `htmldiff` in Python, there are several crates you can combine to achieve this. Here's a breakdown of relevant libraries and approaches:

---

### 1. **`similar-html`** (Recommended)
A specialized HTML-aware diffing crate built on top of the popular `similar` library. It understands HTML structure and attributes:
```toml
[dependencies]
similar-html = "0.5"
```

**Example Usage:**
```rust
use similar_html::{diff, ChangeTag};

fn generate_html_diff(old: &str, new: &str) -> String {
    let changes = diff(old, new);
    let mut output = String::new();
    
    for change in changes {
        let (class, content) = match change.tag() {
            ChangeTag::Delete => ("diff-removed", format!("-{}", change.value())),
            ChangeTag::Insert => ("diff-added", format!("+{}", change.value())),
            ChangeTag::Equal => ("diff-unchanged", format!(" {}", change.value())),
        };
        
        output.push_str(&format!(
            "<div class=\"{}\">{}</div>",
            class,
            html_escape::encode_text(&content)
        ));
    }
    
    output
}
```

---

### 2. **HTML Parsing + Diff Combo**
Combine HTML parsing crates with general-purpose diff libraries:

#### Key Crates:
| Crate | Purpose |
|-------|---------|
| [`html5ever`](https://crates.io/crates/html5ever) | Industrial-strength HTML parser |
| [`similar`](https://crates.io/crates/similar) | Text/structured diffing |
| [`kuchiki`](https://crates.io/crates/kuchiki) | DOM tree manipulation |
| [`ego-tree`](https://crates.io/crates/ego-tree) | Lightweight DOM tree |

**Example Workflow:**
1. Parse HTML → `html5ever`/`kuchiki`
2. Normalize DOM → Custom logic
3. Diff trees → `similar`/tree edit distance algorithms
4. Generate annotated HTML → String manipulation

---

### 3. **Inline Styling Approach**
For quick visual diffs (not semantic HTML diffs), use `similar` directly:
```rust
use similar::TextDiff;

fn simple_html_diff(old: &str, new: &str) -> String {
    TextDiff::from_lines(old, new)
        .unified_diff()
        .header("Old", "New")
        .to_string()
}
```

---

### Full Example with `similar-html`
```rust
use similar_html::{diff, ChangeTag};
use html_escape::encode_text;

fn main() {
    let html1 = r#"<div class="old"><p>Hello World</p></div>"#;
    let html2 = r#"<div class="new"><h1>Hello Rust</h1></div>"#;
    
    let diff = generate_styled_diff(html1, html2);
    println!("{}", diff);
}

fn generate_styled_diff(old: &str, new: &str) -> String {
    let mut output = String::from(r#"<style>
        .diff-removed { color: red; text-decoration: line-through; }
        .diff-added { color: green; }
        .diff-unchanged { color: gray; }
    </style><div class="diff">"#);
    
    for change in diff(old, new) {
        let (class, marker) = match change.tag() {
            ChangeTag::Delete => ("diff-removed", "-"),
            ChangeTag::Insert => ("diff-added", "+"),
            ChangeTag::Equal => ("diff-unchanged", " "),
        };
        
        output.push_str(&format!(
            "<div class=\"{}\">{} {}</div>",
            class,
            marker,
            encode_text(change.value())
        ));
    }
    
    output.push_str("</div>");
    output
}
```

---

### When to Use What:
| Use Case | Recommendation |
|----------|----------------|
| Simple text diffs | `similar` alone |
| HTML-aware diffs | `similar-html` |
| Full DOM analysis | `html5ever` + tree diffing |
| Visual output | Combine with `html_escape` |

For production use, `similar-html` is the most robust choice today. For research/advanced needs, consider implementing a tree-diff algorithm like [Zhang-Shasha](https://crates.io/crates/zhang-shasha) on parsed HTML trees.
