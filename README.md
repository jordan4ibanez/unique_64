Unique64
--------
A handy struct for dispatching out unique u64s as IDs.

Please see the generated docs for how to use. It's pretty simple.

It also doesn't hold your hand. You do whatever you want with it.

But to show you without any context:
```rust
fn main() {
  let mut dispatcher = Unique64::new();

  // 0
  let x = dispatcher.get_next();

  // 1
  let y = dispatcher.get_next();

  // 0 is free again.
  dispatcher.remove(x);

  // 0
  let z = dispatcher.get_next();

  assert!(z == 0);
}

```