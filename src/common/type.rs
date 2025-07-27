use crate::*;

/// Thread-safe reference-counted read-write lock wrapper.
pub(crate) type ArcRwLock<T> = Arc<RwLock<T>>;
