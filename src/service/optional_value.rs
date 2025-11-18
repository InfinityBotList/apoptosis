use std::cell::RefCell;

/// Helper struct for caching optional values (usually from Luau) where the T is cheap to clone
pub struct OptionalValue<T>(RefCell<Option<T>>);

#[allow(dead_code)]
impl<T> OptionalValue<T> {
    pub fn new() -> Self {
        Self(RefCell::new(None))
    }

    pub fn get(&self, f: impl FnOnce() -> T) -> T
    where
        T: Clone,
    {
        if let Some(cached) = self.0.borrow().as_ref() {
            return cached.clone();
        }

        let value = f();
        *self.0.borrow_mut() = Some(value.clone());
        value
    }

    pub fn get_failable<E>(&self, f: impl FnOnce() -> Result<T, E>) -> Result<T, E>
    where
        T: Clone,
    {
        if let Some(cached) = self.0.borrow().as_ref() {
            return Ok(cached.clone());
        }

        let value = f()?;
        *self.0.borrow_mut() = Some(value.clone());
        Ok(value)
    }
}

impl<T> Default for OptionalValue<T> {
    fn default() -> Self {
        Self::new()
    }
}
