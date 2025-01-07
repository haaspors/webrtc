use std::{ops, sync};

/// A synchronous mutual exclusion primitive useful for protecting shared data.
#[derive(Default, Debug)]
pub struct Mutex<T>(sync::Mutex<T>);

impl<T> Mutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    pub fn new(value: T) -> Self {
        Self(sync::Mutex::new(value))
    }

    /// Acquires a mutex, blocking the current thread until it is able to do so.
    pub fn lock(&self) -> MutexGuard<'_, T> {
        let guard = self.0.lock().unwrap();

        MutexGuard(guard)
    }

    /// Consumes this mutex, returning the underlying data.
    pub fn into_inner(self) -> T {
        self.0.into_inner().unwrap()
    }
}

/// An RAII implementation of a "scoped lock" of a mutex. When this structure is
/// dropped (falls out of scope), the lock will be unlocked.
pub struct MutexGuard<'a, T>(sync::MutexGuard<'a, T>);

impl<T> ops::Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
