// src/lib.rs
/*
 * Core library for LayerTwo
 */

use log::{info, error, debug};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

/// Custom result type for this library
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Represents the result of a processing operation
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    /// Whether the operation was successful
    pub success: bool,
    /// A message describing the outcome
    pub message: String,
    /// Optional additional data
    pub data: Option<serde_json::Value>,
}

/// The core processor for LayerTwo
#[derive(Debug)]
pub struct LayerTwoProcessor {
    /// Whether to enable verbose logging
    verbose: bool,
    /// The number of items processed
    processed_count: usize,
}

impl LayerTwoProcessor {
    /// Creates a new processor instance with the given verbosity level
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            processed_count: 0,
        }
    }

    /// Processes the given data and returns the result
    pub fn process(&mut self, data: &str) -> Result<ProcessResult> {
        if self.verbose {
            debug!("Processing data of length: {}", data.len());
        }

        // Simulate processing
        self.processed_count += 1;
        
        let result = ProcessResult {
            success: true,
            message: format!("Successfully processed item #{}", self.processed_count),
            data: Some(serde_json::json!({
                "length": data.len(),
                "processed_at": chrono::Utc::now().to_rfc3339(),
                "item_number": self.processed_count
            })),
        };

        Ok(result)
    }

    /// Returns the current processing statistics
    pub fn get_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "processed_count": self.processed_count,
            "verbose": self.verbose
        })
    }
}

/// Main processing function
pub fn run(verbose: bool, input_path: Option<String>, output_path: Option<String>) -> Result<()> {
    // Initialize logging
    if verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }
    
    info!("Starting LayerTwo processing");
    
    let mut processor = LayerTwoProcessor::new(verbose);
    
    // Read input from file
    let input_data = match input_path {
        Some(path) => {
            info!("Reading from file: {}", path);
            fs::read_to_string(path).map_err(|e| Box::new(e) as _)
        }
        None => {
            info!("Reading from standard input");
            std::io::stdin().lines().map(|line| line.unwrap()).collect()
        }
    }?;

    // Process input data
    for line in input_data.lines() {
        processor.process(line)?;
    }

    // Write output to file
    if let Some(output_path) = output_path {
        info!("Writing output to file: {}", output_path);
        let output = serde_json::to_string(&processor.get_stats())?;
        fs::write(output_path, output).map_err(|e| Box::new(e) as _)?;
    }

    Ok(())
}