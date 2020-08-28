pub type Version = usize;

const INITIAL_VERSION: Version = 0;

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
        Self::with_version(self.0.clone(), self.1)
    }
}

impl<T> Copy for Versioned<T>
    where T: Copy
{
    // Empty
}

impl<T> Versioned<T> {
    pub fn new(value: T) -> Self {
        Self::with_version(value, INITIAL_VERSION)
    }

    pub fn with_version(value: T, version: Version) -> Self {
        Self(value, version)
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    #[must_use = "mutation will be counted even if reference returned from get_mut() is not actually used"]
    pub fn get_mut(&mut self) -> &mut T {
        self.1 += 1;

        &mut self.0
    }

    pub fn version(&self) -> Version {
        self.1
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

        assert_eq!(versioned_value.version(), 0);
    }

    #[test]
    fn version_correct_on_with_version() {
        let versioned_value = Versioned::with_version("value", 53);

        assert_eq!(versioned_value.version(), 53);
    }

    #[test]
    fn version_unchanged_on_get() {
        let versioned_value = Versioned::new("some value");

        let _ = versioned_value.get();

        assert_eq!(versioned_value.version(), 0);

        versioned_value.get();
        versioned_value.get();

        assert_eq!(versioned_value.version(), 0);
    }

    #[test]
    fn version_increment_on_get_mut() {
        let mut versioned_value = Versioned::new(255);

        *versioned_value.get_mut() = 10;

        assert_eq!(versioned_value.version(), 1);

        let _ = versioned_value.get_mut();
        let _ = versioned_value.get_mut();

        assert_eq!(versioned_value.version(), 3);
    }
}
