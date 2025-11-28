use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::Semaphore;
use tracing::{debug, error, span, Level};

#[derive(Error, Debug)]
pub enum BatchError {
    #[error("Processing error: {0}")]
    ProcessingError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Timeout error")]
    TimeoutError,

    #[error("Channel error: {0}")]
    ChannelError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    pub batch_size: usize,
    pub max_concurrent: usize,
    pub rate_limit: Option<RateLimit>,
    pub timeout: Duration,
    pub retry_count: u32,
    pub retry_delay: Duration,
    pub continue_on_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_second: f64,
    pub burst_size: usize,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            max_concurrent: 5,
            rate_limit: Some(RateLimit {
                requests_per_second: 10.0,
                burst_size: 20,
            }),
            timeout: Duration::from_secs(30),
            retry_count: 3,
            retry_delay: Duration::from_millis(100),
            continue_on_error: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingItem {
    pub id: String,
    pub data: Value,
    pub status: ProcessingStatus,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Success,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub total_items: usize,
    pub processed_items: usize,
    pub failed_items: usize,
    pub skipped_items: usize,
    pub success_rate: f64,
    pub processing_time_ms: u128,
    pub results: Vec<ProcessingItem>,
}

#[derive(Debug)]
pub struct BatchProcessor {
    config: BatchConfig,
    semaphore: Arc<Semaphore>,
}

impl BatchProcessor {
    /// Creates a new BatchProcessor with default configuration
    ///
    /// # Example
    /// ```ignore
    /// let processor = BatchProcessor::new();
    /// ```
    pub fn new() -> Self {
        let config = BatchConfig::default();
        Self::with_config(config)
    }

    /// Creates a new BatchProcessor with custom configuration
    ///
    /// # Example
    /// ```ignore
    /// let config = BatchConfig {
    ///     batch_size: 50,
    ///     max_concurrent: 10,
    ///     rate_limit: Some(RateLimit {
    ///         requests_per_second: 5.0,
    ///         burst_size: 10,
    ///     }),
    ///     timeout: Duration::from_secs(60),
    ///     retry_count: 5,
    ///     retry_delay: Duration::from_millis(200),
    ///     continue_on_error: true,
    /// };
    /// let processor = BatchProcessor::with_config(config);
    /// ```
    pub fn with_config(config: BatchConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
        Self { config, semaphore }
    }

    /// Processes items in batches
    ///
    /// # Example
    /// ```ignore
    /// let items = vec![
    ///     ProcessingItem {
    ///         id: "1".to_string(),
    ///         data: json!({"name": "item1"}),
    ///         status: ProcessingStatus::Pending,
    ///         error: None,
    ///     }
    /// ];
    /// let result = processor.process_batch(items).await?;
    /// ```
    pub async fn process_batch<F>(
        &self,
        items: Vec<ProcessingItem>,
        processor_fn: F,
    ) -> Result<BatchResult, BatchError>
    where
        F: Fn(
                ProcessingItem,
            )
                -> futures::future::BoxFuture<'static, Result<ProcessingItem, BatchError>>
            + Sync
            + Send
            + 'static,
    {
        let span = span!(Level::DEBUG, "process_batch", item_count = items.len());
        let _guard = span.enter();

        debug!("Starting batch processing of {} items", items.len());

        let start_time = std::time::Instant::now();
        let processor_fn = Arc::new(processor_fn);

        let mut handles = Vec::new();
        let total_items = items.len();

        for item in items {
            let semaphore = Arc::clone(&self.semaphore);
            let processor_fn = Arc::clone(&processor_fn);
            let config = self.config.clone();

            let handle = tokio::spawn(async move {
                let _permit = semaphore
                    .acquire()
                    .await
                    .map_err(|e| BatchError::ChannelError(e.to_string()))?;

                Self::process_item_with_retry(
                    item,
                    &processor_fn,
                    config.retry_count,
                    config.retry_delay,
                )
                .await
            });

            handles.push(handle);
        }

        let mut results = Vec::new();
        let mut processed = 0;
        let mut failed = 0;
        let mut skipped = 0;

        for handle in handles {
            match handle.await {
                Ok(Ok(item)) => {
                    if item.status == ProcessingStatus::Success {
                        processed += 1;
                    } else if item.status == ProcessingStatus::Failed {
                        failed += 1;
                        if !self.config.continue_on_error {
                            error!("Item {} failed", item.id);
                            return Err(BatchError::ProcessingError(
                                item.error.unwrap_or_else(|| "Unknown error".to_string()),
                            ));
                        }
                    } else {
                        skipped += 1;
                    }
                    results.push(item);
                }
                Ok(Err(e)) => {
                    failed += 1;
                    if !self.config.continue_on_error {
                        return Err(e);
                    }
                    error!("Error processing item: {}", e);
                }
                Err(e) => {
                    failed += 1;
                    if !self.config.continue_on_error {
                        return Err(BatchError::ChannelError(e.to_string()));
                    }
                    error!("Task join error: {}", e);
                }
            }
        }

        let processing_time_ms = start_time.elapsed().as_millis();
        let success_rate = if total_items > 0 {
            (processed as f64 / total_items as f64) * 100.0
        } else {
            0.0
        };

        debug!(
            "Batch processing completed: {} success, {} failed, {} skipped in {}ms",
            processed, failed, skipped, processing_time_ms
        );

        Ok(BatchResult {
            total_items,
            processed_items: processed,
            failed_items: failed,
            skipped_items: skipped,
            success_rate,
            processing_time_ms,
            results,
        })
    }

    /// Processes items sequentially with rate limiting
    ///
    /// # Example
    /// ```ignore
    /// let items = vec![/* items */];
    /// let result = processor.process_sequential(items).await?;
    /// ```
    pub async fn process_sequential<F>(
        &self,
        items: Vec<ProcessingItem>,
        processor_fn: F,
    ) -> Result<BatchResult, BatchError>
    where
        F: Fn(
            ProcessingItem,
        ) -> futures::future::BoxFuture<'static, Result<ProcessingItem, BatchError>>,
    {
        let span = span!(Level::DEBUG, "process_sequential", item_count = items.len());
        let _guard = span.enter();

        debug!("Starting sequential processing of {} items", items.len());

        let start_time = std::time::Instant::now();
        let mut results = Vec::new();
        let mut processed = 0;
        let mut failed = 0;
        let total_items = items.len();

        for item in items {
            if let Some(rate_limit) = &self.config.rate_limit {
                let delay = Duration::from_secs_f64(1.0 / rate_limit.requests_per_second);
                tokio::time::sleep(delay).await;
            }

            match processor_fn(item).await {
                Ok(result_item) => {
                    if result_item.status == ProcessingStatus::Success {
                        processed += 1;
                    } else {
                        failed += 1;
                        if !self.config.continue_on_error {
                            return Err(BatchError::ProcessingError(
                                result_item
                                    .error
                                    .unwrap_or_else(|| "Unknown error".to_string()),
                            ));
                        }
                    }
                    results.push(result_item);
                }
                Err(e) => {
                    failed += 1;
                    if !self.config.continue_on_error {
                        return Err(e);
                    }
                    error!("Error in sequential processing: {}", e);
                }
            }
        }

        let processing_time_ms = start_time.elapsed().as_millis();
        let success_rate = if total_items > 0 {
            (processed as f64 / total_items as f64) * 100.0
        } else {
            0.0
        };

        Ok(BatchResult {
            total_items,
            processed_items: processed,
            failed_items: failed,
            skipped_items: 0,
            success_rate,
            processing_time_ms,
            results,
        })
    }

    /// Chunks items into batches
    ///
    /// # Example
    /// ```ignore
    /// let items = vec![/* 1000 items */];
    /// let chunks = processor.chunk_items(items);
    /// ```
    pub fn chunk_items(&self, items: Vec<ProcessingItem>) -> Vec<Vec<ProcessingItem>> {
        let span = span!(
            Level::DEBUG,
            "chunk_items",
            item_count = items.len(),
            batch_size = self.config.batch_size
        );
        let _guard = span.enter();

        debug!(
            "Chunking {} items into batches of {}",
            items.len(),
            self.config.batch_size
        );

        items
            .into_iter()
            .collect::<Vec<_>>()
            .chunks(self.config.batch_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    /// Gets statistics about processing performance
    ///
    /// # Example
    /// ```ignore
    /// let stats = processor.get_stats(&result);
    /// ```
    pub fn get_stats(&self, result: &BatchResult) -> ProcessingStats {
        ProcessingStats {
            total_items: result.total_items,
            processed_items: result.processed_items,
            failed_items: result.failed_items,
            success_rate: result.success_rate,
            processing_time_ms: result.processing_time_ms,
            items_per_second: if result.processing_time_ms > 0 {
                (result.processed_items as f64 / result.processing_time_ms as f64) * 1000.0
            } else {
                0.0
            },
        }
    }

    async fn process_item_with_retry<F>(
        mut item: ProcessingItem,
        processor_fn: &Arc<F>,
        retry_count: u32,
        retry_delay: Duration,
    ) -> Result<ProcessingItem, BatchError>
    where
        F: Fn(
                ProcessingItem,
            )
                -> futures::future::BoxFuture<'static, Result<ProcessingItem, BatchError>>
            + Sync
            + Send,
    {
        let mut attempt = 0;

        loop {
            item.status = ProcessingStatus::Processing;

            match processor_fn(item.clone()).await {
                Ok(processed_item) => {
                    return Ok(processed_item);
                }
                Err(e) => {
                    attempt += 1;
                    if attempt >= retry_count {
                        item.status = ProcessingStatus::Failed;
                        item.error = Some(e.to_string());
                        return Ok(item);
                    }
                    debug!("Retry attempt {} for item {}", attempt, item.id);
                    tokio::time::sleep(retry_delay).await;
                }
            }
        }
    }
}

impl Default for BatchProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStats {
    pub total_items: usize,
    pub processed_items: usize,
    pub failed_items: usize,
    pub success_rate: f64,
    pub processing_time_ms: u128,
    pub items_per_second: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_config_default() {
        let config = BatchConfig::default();
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.max_concurrent, 5);
    }

    #[test]
    fn test_processing_item_creation() {
        let item = ProcessingItem {
            id: "1".to_string(),
            data: Value::Null,
            status: ProcessingStatus::Pending,
            error: None,
        };
        assert_eq!(item.status, ProcessingStatus::Pending);
    }

    #[tokio::test]
    async fn test_processor_creation() {
        let processor = BatchProcessor::new();
        assert_eq!(processor.config.batch_size, 100);
    }

    #[test]
    fn test_chunk_items() {
        let processor = BatchProcessor::new();
        let items: Vec<ProcessingItem> = (0..250)
            .map(|i| ProcessingItem {
                id: i.to_string(),
                data: Value::Null,
                status: ProcessingStatus::Pending,
                error: None,
            })
            .collect();

        let chunks = processor.chunk_items(items);
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].len(), 100);
        assert_eq!(chunks[1].len(), 100);
        assert_eq!(chunks[2].len(), 50);
    }
}
