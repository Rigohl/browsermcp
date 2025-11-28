pub mod batch;
pub mod extractor;
pub mod parser;
pub mod transformer;

pub use batch::{BatchConfig, BatchProcessor, BatchResult};
pub use extractor::{DataExtractor, ExtractionResult, ExtractorConfig};
pub use parser::{DomParser, ParseError, ParserConfig, SearchConfig, SearchMatch, SearchResult};
pub use transformer::{DataTransformer, TransformationError, TransformationPipeline};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScrapingError {
    #[error("Parser error: {0}")]
    ParserError(#[from] ParseError),

    #[error("Extraction error: {0}")]
    ExtractionError(String),

    #[error("Transformation error: {0}")]
    TransformationError(#[from] TransformationError),

    #[error("Batch processing error: {0}")]
    BatchError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type ScrapingResult<T> = Result<T, ScrapingError>;
