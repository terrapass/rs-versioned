use std::{
    ops::{
        Deref,
        DerefMut
    }
};

pub type Version = usize;

pub const INITIAL_VERSION: Version = 0;

#[derive(Debug)]
pub struct Versioned<T>(T, Version);

impl<T> Default for Versioned<T>
    where T: Default
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Clone for Versioned<T>
    where T: Clone
{
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

    #[must_use]
    fn deref(&self) -> &Self::Target {
        self.as_ref_impl()
    }
}

impl<T> DerefMut for Versioned<T> {
    #[must_use = "mutation will be counted even if mutable dereference result is not actually used"]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_impl()
    }
}

impl<T> AsRef<T> for Versioned<T> {
    #[must_use]
    fn as_ref(&self) -> &T {
        self.as_ref_impl()
    }
}

impl<T> AsMut<T> for Versioned<T> {
    #[must_use = "mutation will be counted even if mutable reference returned from as_mut() is not actually used"]
    fn as_mut(&mut self) -> &mut T {
        self.as_mut_impl()
    }
}

impl<T> Versioned<T> {
    //
    // Interface
    //

    pub fn new(value: T) -> Self {
        Self::with_version(value, INITIAL_VERSION)
    }

    pub fn with_version(value: T, version: Version) -> Self {
        Self(value, version)
    }

    pub fn version(&self) -> Version {
        self.1
    }

    //
    // Service
    //

    fn as_ref_impl(&self) -> &T {
        &self.0
    }

    //#[must_use = "mutation will be counted even if reference returned from get_mut() is not actually used"]
    fn as_mut_impl(&mut self) -> &mut T {
        self.1 += 1;

        &mut self.0
    }
}

impl<T> Versioned<T>
    where T: Default
{
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
}
