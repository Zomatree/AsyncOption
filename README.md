# AsyncOption
Async Option and Result

## Example

```rust
#![feature(async_closure)]

use async_option::FutureOption;

let opt = Some(1).async_map(async move |v| v + 1).await;
println!("{opt:?}");
```
