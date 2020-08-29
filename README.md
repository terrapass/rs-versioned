[![crates.io](http://meritbadge.herokuapp.com/versioned)](https://crates.io/crates/versioned)
[![docs.rs](https://docs.rs/versioned/badge.svg)](https://docs.rs/versioned)
[![Build Status](https://travis-ci.org/terrapass/rs-versioned.svg?branch=master)](https://travis-ci.org/terrapass/rs-versioned)

# versioned

This tiny crate provides just the pointer-like [`Versioned<T>`](https://docs.rs/versioned/0.1.0/versioned/struct.Versioned.html) wrapper,
which counts the number of times its contained `T` value has been mutably accessed.

This may be useful when caching certain calculation results based on objects, which are expensive
to compare or hash, such as large collections. In such cases it might be more convenient to store
object version and later check if it changed.

```rust
use versioned::Versioned;

let mut versioned_value = Versioned::new("Hello".to_string());

assert_eq!(versioned_value.version(), 0, "version is 0 initially");

// This is an immutable dereference, so it won't change the version.
let value_len = versioned_value.len();

assert_eq!(versioned_value.version(), 0, "version is unchanged after immutable access");

// Now we mutate the value twice.
versioned_value.push_str(" ");
versioned_value.push_str("World!");

assert_eq!(*versioned_value, "Hello World!");
assert_eq!(versioned_value.version(), 2, "version got incremented once per mutable access");
```

[`Versioned<T>`](https://docs.rs/versioned/0.1.0/versioned/struct.Versioned.html) implements [`Deref`](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html),
[`AsRef`](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html) and their `Mut` counterparts.
In particular, due to [`Deref` coercion](https://doc.rust-lang.org/std/ops/trait.Deref.html#more-on-deref-coercion),
[`Versioned<T>`](https://docs.rs/versioned/0.1.0/versioned/struct.Versioned.html) values can be passed as `&T` and `&mut T`
parameters to functions:
```rust
use versioned::Versioned;

fn look_at(value: &String) {}
fn modify(value: &mut String) {}

let mut versioned_value = Versioned::new("blabla".to_string());

look_at(&versioned_value);
assert_eq!(versioned_value.version(), 0);

modify(&mut versioned_value);
assert_eq!(versioned_value.version(), 1, "version increased due to mutable dereference");
```
Note from the example above that, since mutations are counted based on mutable dereferences,
version got increased on mutable dereference in the call to `modify()`, even though
ultimately no mutation of the value itself took place.
