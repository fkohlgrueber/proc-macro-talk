# Project

A procedural macro that simplifies parsing text into Rust structs.

Example:

```rust
use pat_parse::parse;

#[parse(r"#{} @ {},{}: {}x{}")]
#[derive(Debug)]
struct Patch {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    heigth: usize,
}

fn main() {
    println!("{:?}", Patch::from_str("#1 @ 2,3: 4x5"));
    // Output:
    // Some(Patch { id: 1, left: 2, top: 3, width: 4, heigth: 5 })

}
```

This folder contains the following crates:
- `pat-parse-macro`: Contains the `parse` attribute proc macro
- `pat-parse`: Wraps the `pat-parse-macro` crate and makes it hygienic
- `app`: Uses the `parse` macro

