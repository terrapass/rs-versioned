//! This tiny crate provides just the pointer-like [`Versioned<T>`](struct.Versioned.html) wrapper,
//! which counts the number of times its contained `T` value has been mutably accessed.
//!
//! This may be useful when caching certain calculation results based on objects, which are expensive
//! to compare or hash, such as large collections. In such cases it might be more convenient to store
//! object version and later check if it changed.
//!
//! ```
//! use versioned::Versioned;
//!
//! let mut versioned_value = Versioned::new("Hello".to_string());
//!
//! assert_eq!(versioned_value.version(), 0, "version is 0 initially");
//!
//! // This is an immutable dereference, so it won't change the version.
//! let value_len = versioned_value.len();
//!
//! assert_eq!(versioned_value.version(), 0, "version is unchanged after immutable access");
//!
//! // Now we mutate the value twice.
//! versioned_value.push_str(" ");
//! versioned_value.push_str("World!");
//!
//! assert_eq!(*versioned_value, "Hello World!");
//! assert_eq!(versioned_value.version(), 2, "version got incremented once per mutable access");
//! ```
//!
//! [`Versioned<T>`](struct.Versioned.html) implements [`Deref`](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html),
//! [`AsRef`](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html) and their `Mut` counterparts.
//! In particular, due to [`Deref` coercion](https://doc.rust-lang.org/std/ops/trait.Deref.html#more-on-deref-coercion),
//! [`Versioned<T>`](struct.Versioned.html) values can be passed as `&T` and `&mut T`
//! parameters to functions:
//! ```
//! use versioned::Versioned;
//!
//! fn look_at(value: &String) {}
//! fn modify(value: &mut String) {}
//!
//! let mut versioned_value = Versioned::new("blabla".to_string());
//!
//! look_at(&versioned_value);
//! assert_eq!(versioned_value.version(), 0);
//!
//! modify(&mut versioned_value);
//! assert_eq!(versioned_value.version(), 1, "version increased due to mutable dereference");
//! ```
//! Note from the example above that, since mutations are counted based on mutable dereferences,
//! version got increased on mutable dereference in the call to `modify()`, even though
//! ultimately no mutation of the value itself took place.

use std::{
    ops::{
        Deref,
        DerefMut
    }
};

/// Integer type used for version numbers.
pub type Version = usize;

/// Initial [`Version`](type.Version.html) for newly constructed [`Versioned<T>`](struct.Versioned.html) instances,
/// unless a different value was specified via
/// [`with_version()`](struct.Versioned.html#with_version)
/// or [`default_with_version()`](struct.Versioned.html#default_with_version) constructors.
pub const INITIAL_VERSION: Version = 0;

/// Generic pointer-like wrapper, which counts mutable dereferences.
///
/// See [crate level documentation](index.html) for more info and examples.
#[derive(Debug)]
pub struct Versioned<T>(T, Version);

impl<T> Default for Versioned<T>
    where T: Default
{
    /// Constructs new [`Versioned<T>`](struct.Versioned.html) wrapper
    /// containing default value for type `T`
    /// and version set to [`INITIAL_VERSION`](constant.INITIAL_VERSION.html).
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Clone for Versioned<T>
    where T: Clone
{
    /// Clones [`Versioned<T>`](struct.Versioned.html).
    /// The clone has its version set to [`INITIAL_VERSION`](constant.INITIAL_VERSION.html).
    fn clone(&self) -> Self {
        Self::new(self.0.clone())
    }
}

impl<T> Copy for Versioned<T>
    where T: Copy
{
    // Empty
}

impl<T> Deref for Versioned<T> {
    type Target = T;

    /// Dereferences the value. Does not increment version.
    #[must_use]
    fn deref(&self) -> &Self::Target {
        self.as_ref_impl()
    }
}

impl<T> DerefMut for Versioned<T> {
    /// Mutably dereferences the value. Increments version.
    #[must_use = "mutation will be counted even if mutable dereference result is not actually used"]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_impl()
    }
}

impl<T> AsRef<T> for Versioned<T> {
    /// Returns reference to the value. Does not increment version.
    #[must_use]
    fn as_ref(&self) -> &T {
        self.as_ref_impl()
    }
}

impl<T> AsMut<T> for Versioned<T> {
    /// Returns mutable reference to the value. Increments version.
    #[must_use = "mutation will be counted even if mutable reference returned from as_mut() is not actually used"]
    fn as_mut(&mut self) -> &mut T {
        self.as_mut_impl()
    }
}

impl<T> Versioned<T> {
    //
    // Interface
    //

    /// Constructs new [`Versioned<T>`](struct.Versioned.html) wrapper
    /// with version set to [`INITIAL_VERSION`](constant.INITIAL_VERSION.html).
    pub fn new(value: T) -> Self {
        Self::with_version(value, INITIAL_VERSION)
    }

    /// Constructs new [`Versioned<T>`](struct.Versioned.html) wrapper
    /// with the given version.
    pub fn with_version(value: T, version: Version) -> Self {
        Self(value, version)
    }

    /// Returns current version.
    pub fn version(&self) -> Version {
        self.1
    }

    //
    // Service
    //

    fn as_ref_impl(&self) -> &T {
        &self.0
    }

    fn as_mut_impl(&mut self) -> &mut T {
        self.1 += 1;

        &mut self.0
    }
}

impl<T> Versioned<T>
    where T: Default
{
    /// Constructs new [`Versioned<T>`](struct.Versioned.html) wrapper
    /// containing default value for type `T`
    /// and the given version.
    pub fn default_with_version(version: Version) -> Self {
        Self::with_version(T::default(), version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_zero_on_new() {
        let versioned_value = Versioned::new(42);

        assert_eq!(*versioned_value, 42);
        assert_eq!(versioned_value.version(), 0);
    }

    #[test]
    fn version_correct_on_with_version() {
        let versioned_value = Versioned::with_version("value", 53);

        assert_eq!(*versioned_value, "value");
        assert_eq!(versioned_value.version(), 53);
    }

    #[test]
    fn version_correct_on_default_with_version() {
        let versioned_value: Versioned<String> = Versioned::default_with_version(97);

        assert_eq!(*versioned_value, String::default());
        assert_eq!(versioned_value.version(), 97);
    }

    #[test]
    fn version_reset_on_clone() {
        let mut versioned_value_0 = Versioned::new("Hello".to_string());

        versioned_value_0.push_str("World!");
        versioned_value_0.pop();

        assert_eq!(*versioned_value_0, "HelloWorld");
        assert_eq!(versioned_value_0.version(), 2);

        let versioned_value_1 = versioned_value_0.clone();

        assert_eq!(*versioned_value_1, *versioned_value_0);
        assert_eq!(versioned_value_1.version(), INITIAL_VERSION);
    }

    #[allow(unused_must_use)]
    #[test]
    fn version_unchanged_on_as_ref() {
        let versioned_value = Versioned::new("some value");

        let _ = *versioned_value;
        let _ = versioned_value.as_ref();

        assert_eq!(versioned_value.version(), 0);

        *versioned_value;
        versioned_value.as_ref();

        assert_eq!(*versioned_value, "some value");
        assert_eq!(versioned_value.version(), 0);
    }

    #[test]
    fn version_increment_on_as_mut() {
        let mut versioned_value = Versioned::new(255);

        *versioned_value = 10;

        assert_eq!(*versioned_value, 10);
        assert_eq!(versioned_value.version(), 1);

        *versioned_value = 50;
        let _ = versioned_value.as_mut();

        assert_eq!(*versioned_value, 50);
        assert_eq!(versioned_value.version(), 3);
    }

    #[test]
    fn version_on_deref_coercion() {
        fn look_at(_: &String) {}
        fn modify(_: &mut String) {}

        let mut versioned_value = Versioned::new("bla".to_string());

        look_at(&versioned_value);
        assert_eq!(versioned_value.version(), 0);

        modify(&mut versioned_value);
        assert_eq!(versioned_value.version(), 1);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn panic_on_version_overflow() {
        let mut versioned_value: Versioned<String> = Versioned::default_with_version(Version::max_value() - 2);

        versioned_value.push_str("This");
        versioned_value.push_str("Will");
        versioned_value.push_str("Overflow");
        versioned_value.push_str("Version");
    }
}
