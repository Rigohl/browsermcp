// Performance benchmarking and metrics
use crate::core::Result;
use std::time::Instant;

#[derive(Debug)]
pub struct PerfBenchmark {
    pub name: String,
    pub iterations: usize,
}

impl PerfBenchmark {
    pub fn new(name: &str, iterations: usize) -> Self {
        Self {
            name: name.to_string(),
            iterations,
        }
    }

    pub async fn run<F, Fut>(&self, mut test_fn: F) -> Result<PerfMetrics>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        tracing::info!(
            "Running benchmark: {} ({} iterations)",
            self.name,
            self.iterations
        );
        let start = Instant::now();

        for _ in 0..self.iterations {
            test_fn().await?;
        }

        let duration = start.elapsed();
        Ok(PerfMetrics {
            name: self.name.clone(),
            total_time: duration,
            iterations: self.iterations,
        })
    }
}

#[derive(Debug)]
pub struct PerfMetrics {
    pub name: String,
    pub total_time: std::time::Duration,
    pub iterations: usize,
}

impl PerfMetrics {
    pub fn avg_time_ms(&self) -> f64 {
        self.total_time.as_millis() as f64 / self.iterations as f64
    }
}
