# autotrait

Sometimes you want to do dynamic dispatch. First you'd have to define a trait:

```rust
trait Trait {
    fn do_stuff(&self) -> String;
}
```

And then implement it on something:

```rust,ignore
struct Impl;

impl Trait for Impl {
    fn do_stuff(&self) -> String {
        // do stuff here
        todo!()
    }
}
```

We're repeating ourselves a bunch when doing that! What if we could just do:

```rust
struct Impl;

#[autotrait::autotrait]
impl Trait for Impl {
    fn do_stuff(&self) -> String {
        // do stuff here
        todo!()
    }
}
```

That way we wouldn't even have to define the trait!

Well, that's what this crates does.
