//! Conditional runtime abstractions for WASM + native support
//!
//! This module provides WASM-compatible async primitives by using:
//! - `tokio::sync::RwLock` on native targets (async-capable)
//! - `parking_lot::RwLock` on WASM targets (sync-only, but compatible)

/// WASM-compatible async/sync lock wrapper
///
/// # Examples
///
/// ```ignore
/// use extreme_browser_mcp::runtime::AsyncRwLock;
///
/// #[tokio::main]
/// async fn main() {
///     let lock = AsyncRwLock::new(42);
///     let value = lock.read().await;
///     println!("{}", value);
/// }
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub use tokio::sync::RwLock as AsyncRwLock;

/// Parking lot RwLock for WASM (synchronous only)
#[cfg(target_arch = "wasm32")]
pub use parking_lot::RwLock as AsyncRwLock;

/// WASM-compatible Mutex wrapper
#[cfg(not(target_arch = "wasm32"))]
pub use tokio::sync::Mutex as AsyncMutex;

/// Parking lot Mutex for WASM (synchronous only)
#[cfg(target_arch = "wasm32")]
pub use parking_lot::Mutex as AsyncMutex;

/// Conditional runtime initialization
///
/// On native targets: Creates Tokio runtime
/// On WASM targets: No-op (WASM uses browser event loop)
#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_task<F>(future: F)
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    tokio::spawn(future);
}

/// WASM-compatible task spawning (no-op on WASM)
#[cfg(target_arch = "wasm32")]
pub fn spawn_task<F>(_future: F)
where
    F: std::future::Future + 'static,
{
    // WASM: Tasks handled by browser event loop via wasm-bindgen-futures
    // This is a no-op in pure WASM; use wasm_bindgen_futures::spawn_local instead
}

/// Get lock read guard (native: async, WASM: sync)
#[cfg(not(target_arch = "wasm32"))]
pub async fn lock_read<T>(lock: &AsyncRwLock<T>) -> tokio::sync::RwLockReadGuard<'_, T> {
    lock.read().await
}

/// Get lock read guard (WASM: sync only)
#[cfg(target_arch = "wasm32")]
pub fn lock_read<T>(lock: &AsyncRwLock<T>) -> parking_lot::RwLockReadGuard<'_, T> {
    lock.read()
}

/// Get lock write guard (native: async, WASM: sync)
#[cfg(not(target_arch = "wasm32"))]
pub async fn lock_write<T>(lock: &AsyncRwLock<T>) -> tokio::sync::RwLockWriteGuard<'_, T> {
    lock.write().await
}

/// Get lock write guard (WASM: sync only)
#[cfg(target_arch = "wasm32")]
pub fn lock_write<T>(lock: &AsyncRwLock<T>) -> parking_lot::RwLockWriteGuard<'_, T> {
    lock.write()
}

/// Feature check: returns true if running on WASM target
pub fn is_wasm_target() -> bool {
    cfg!(target_arch = "wasm32")
}

/// Feature check: returns true if running on native target
pub fn is_native_target() -> bool {
    !cfg!(target_arch = "wasm32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_detection() {
        if cfg!(target_arch = "wasm32") {
            assert!(is_wasm_target());
            assert!(!is_native_target());
        } else {
            assert!(!is_wasm_target());
            assert!(is_native_target());
        }
    }

    #[tokio::test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn test_async_rwlock_native() {
        let lock = AsyncRwLock::new(42);
        {
            let read = lock.read().await;
            assert_eq!(*read, 42);
        }
        {
            let mut write = lock.write().await;
            *write = 100;
        }
        {
            let read = lock.read().await;
            assert_eq!(*read, 100);
        }
    }
}
