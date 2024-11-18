//! `impl criterion::async_executor::AsyncExecutor for {Runtime, FusionRuntime}`

use criterion::async_executor::AsyncExecutor;
use monoio::Driver;
use monoio::FusionRuntime;
use monoio::Runtime;
use std::cell::RefCell;

/// Executor wrapper for [`Runtime`].
pub struct MonoioRuntimeExecutor<D: Driver> {
    rt: RefCell<Runtime<D>>,
}

impl<D: Driver> MonoioRuntimeExecutor<D> {
    /// Create an executor from a [`Runtime`] instance.
    pub fn new(rt: Runtime<D>) -> Self {
        Self {
            rt: RefCell::new(rt),
        }
    }
}

impl<D: Driver> AsyncExecutor for MonoioRuntimeExecutor<D> {
    fn block_on<T>(&self, future: impl std::future::Future<Output = T>) -> T {
        self.rt.borrow_mut().block_on(future)
    }
}

/// Executor wrapper for [`FusionRuntime`].
pub struct MonoioFusionRuntimeExecutor<L: Driver, R: Driver> {
    rt: RefCell<FusionRuntime<L, R>>,
}

impl<L: Driver, R: Driver> MonoioFusionRuntimeExecutor<L, R> {
    /// Create an executor from a [`FusionRuntime`] instance.
    pub fn new(rt: FusionRuntime<L, R>) -> Self {
        Self {
            rt: RefCell::new(rt),
        }
    }
}

impl<L: Driver, R: Driver> AsyncExecutor for MonoioFusionRuntimeExecutor<L, R> {
    fn block_on<T>(&self, future: impl std::future::Future<Output = T>) -> T {
        self.rt.borrow_mut().block_on(future)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use criterion::*;

    // Just need to verify the interface works, no need to run anything.

    #[test]
    fn test_runtime() {
        fn _from_elem(c: &mut Criterion) {
            c.bench_function("test_runtime", |b| {
                let rt = monoio::RuntimeBuilder::<monoio::FusionDriver>::new()
                    .enable_all()
                    .build()
                    .unwrap();
                b.to_async(MonoioFusionRuntimeExecutor::new(rt))
                    .iter(|| async { println!("hello") });
            });
        }
    }

    #[test]
    fn test_fusion_runtime() {
        fn _from_elem(c: &mut Criterion) {
            c.bench_function("test_runtime", |b| {
                let rt = monoio::RuntimeBuilder::<monoio::LegacyDriver>::new()
                    .enable_all()
                    .build()
                    .unwrap();
                b.to_async(MonoioRuntimeExecutor::new(rt))
                    .iter(|| async { println!("hello") });
            });
        }
    }
}
