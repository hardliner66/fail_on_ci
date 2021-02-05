# fail_on_ci

Have you ever had the problem that you put in test code, than forget about it and push it to prod?

This crate provides you with a macro which fails to compile on known CI servers. Just wrap your test code in it
and it will only compile it locally.

## Example

```rust
use fail_on_ci::fail_on_ci;

fn main() -> {
    fail_on_ci!{
        println!("This doesn't compile on CI!");
    }
}
```
