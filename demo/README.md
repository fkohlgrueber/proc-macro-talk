# Demo

Simple demo showing a minimal example of the three different
kinds of procedural macros.

It contains two crates:
- `hello-world-macros`: Contains the macros
- `app`: Uses the macros defined in `hello-world-macros`

To build both crates, run:

```
cargo build
```

This expands the macros used in `app` which produces some debug output:

```
felix@the-machine:~/proc-macro-talk/demo$ cargo build
   Compiling hello-world-macros v0.1.0 (/home/felix/proc-macro-talk/demo/hello-world-macros)
   Compiling app v0.1.0 (/home/felix/proc-macro-talk/demo/app)
Function Macro called! Input:
fn test () { }
Derive Macro called! Input:
struct Foo {
    bar: usize,
}
Attribute Macro called! 
Attr:
attribute argument tokens
Item:
fn attribute_item() { }
    Finished dev [unoptimized + debuginfo] target(s) in 0.87s
```

You can view the code produced by the macros by running `cargo expand`:

To install, run `cargo install cargo-expand`. Then, the following command prints the contents of the `app` crate after macro expansion:
```
cargo expand -p app
```