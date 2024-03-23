# `::gens`

Provides [`Id`], a cheap numerical identifier that has the ability to create
new, unique `Id`s without external state*, as well as providing basic
generational information.

*technically, there _is_ one static, to ensure that subsequent calls to
`Id::root` don't immediately generate duplicate ID's. Past that, every `Id` is
entirely self-contained.

## Is this useful?

Strictly speaking, no. It's trivial to have an atomic counter that will give
you nice, simple, sequential ID's in a thread-safe manner, no strings attached.
But since when was programming strictly useful?

The one thing this provides over a global ID generator is basic generational
information. Each `Id` keeps track of its parent and its depth, allowing you to
compare ID's for depth and trace a lineage back to the root.

## Why's it so big?

Each `Id` is 32 bytes to avoid collisions. If you're certain that you're done
creating children from a given `Id`, you can convert it into its final `u128`
to half that size via `.id()`.

Note that this is a one-way operation: you cannot turn a `u128` back into an
`Id`, since information is lost in the process (e.g. generational information).

The one exception is with feature `serde`: it enables [`serde`] support. Note
that this serialization loses no information, it serializes the whole `Id` and
thus will never lead to duplicate ID's.

## Give me some code

```rust
use gens::Id;

let mut root = Id::root();

// Two `::root()`s are identical! Be warned!
let other = Id::root();
assert_eq!(root, other);

// Two `.next_id()`s are unique:
let mut a = root.next_id();
let mut b = root.next_id();
assert_ne!(a, b);

// And those `Id`s can continue to proliferate:
let c = a.next_id();
let d = b.next_id();
assert_ne!(c, d);

// And, they carry some generational info:
assert_eq!(c.parent(), a.id());
assert!(c > root);
assert!(d > b);
```

[`Id`]: https://docs.rs/ids/latest/ids/struct.Id.html
[`serde`]: https://crates.io/crates/serde
