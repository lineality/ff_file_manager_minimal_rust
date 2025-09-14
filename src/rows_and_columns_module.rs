// src/rows_and_columns_module.rs

/// Primary module for the rows_and_columns CSV analysis and TUI dashboard system
///
/// This module serves as the main entry point for CSV data processing, analysis, and
/// visualization. It manages the binary-relative directory structure for persistent
/// data storage and coordinates all CSV operations through a terminal user interface.
///
/// # Core Responsibilities
/// - Initialize and manage the rows_columns_data/ directory structure
/// - Coordinate CSV file imports and directory-based storage
/// - Provide the main application interface following FF-style patterns
/// - Integrate with file selection and TUI dashboard modules
///
/// # Directory Structure Created
/// ```
/// rows_columns_data/
/// ├── csv_imports/           # Imported CSV datasets
/// └── analysis_cache/        # Computed statistics cache
/// ```
///
/// # Design Philosophy
/// - Binary-executable-relative paths for portable deployment
/// - Persistent directory-based data storage (not temporary)
/// - No pre-loading: on-demand data processing for scalability
/// - Clear error handling with comprehensive user feedback

use std::path::PathBuf;

// Import enhanced CSV analysis capabilities
use super::csv_processor_module::{
    analyze_csv_file_structure_and_types,
    // CsvAnalysisResults,
    perform_enhanced_statistical_analysis,
    // display_enhanced_csv_analysis_results,
    save_analysis_details_to_file,
    save_analysis_summary_to_file,
};

// Import our custom error types for comprehensive error handling
use super::error_types_module::{
    // RowsAndColumnsError,
    RowsAndColumnsResult,
    create_file_system_error,
    create_configuration_error
};

/// Modified version of process_csv_file_from_command_line that saves reports to file
///
/// This function performs the same analysis as before but saves results to a TOML
/// report file instead of printing to terminal. The report file is created in the
/// same directory as the source CSV file.
///
/// # Arguments
/// * `csv_file_path_argument` - The CSV file path provided as command line argument
/// * `directory_paths` - The application directory structure for data storage
///
/// # Returns
/// * `RowsAndColumnsResult<()>` - Success or detailed error information
///
/// # Errors
/// * `RowsAndColumnsError::FileSystemError` - If file access or validation fails
/// * `RowsAndColumnsError::CsvProcessingError` - If CSV parsing fails
/// * `RowsAndColumnsError::MetadataError` - If metadata operations fail
pub fn rc_analyze_datafile_save_results_to_resultsfile(
    csv_file_path_argument: &str,
) -> RowsAndColumnsResult<PathBuf> {
    // Step 1: Validate the provided file path
    let csv_file_absolute_path = validate_csv_file_path_from_argument(csv_file_path_argument)?;

    // Step 2: Analyze CSV structure and column types (basic analysis)
    let csv_analysis_results = analyze_csv_file_structure_and_types(&csv_file_absolute_path)?;

    // Step 3: Perform enhanced statistical analysis
    let enhanced_analysis_results = perform_enhanced_statistical_analysis(
        &csv_file_absolute_path,
        &csv_analysis_results
    )?;

    // Step 4: Save summary to report file (creates new file)
    let report_file_path = save_analysis_summary_to_file(
        &csv_file_absolute_path,
        &csv_analysis_results,
        // directory_paths
    )?;

    // Step 5: Append detailed analysis to report file
    save_analysis_details_to_file(
        &report_file_path,
        &enhanced_analysis_results
    )?;

    // Step 6: Notify user of report creation (minimal output to terminal)
    println!("Analysis report here: {}", report_file_path.display());

    // Step 7: Return the report file path
    Ok(report_file_path)
}

/// Validates a CSV file path provided as command line argument
///
/// This function checks if the provided path exists, is accessible, and appears
/// to be a CSV file based on its extension and basic validation.
///
/// # Arguments
/// * `csv_file_path_argument` - The file path string from command line
///
/// # Returns
/// * `RowsAndColumnsResult<PathBuf>` - Absolute path to validated CSV file or error
///
/// # Errors
/// * `RowsAndColumnsError::FileSystemError` - If file doesn't exist or isn't accessible
/// * `RowsAndColumnsError::ConfigurationError` - If file doesn't appear to be CSV
fn validate_csv_file_path_from_argument(csv_file_path_argument: &str) -> RowsAndColumnsResult<PathBuf> {
    // Convert to PathBuf for easier manipulation
    let file_path = PathBuf::from(csv_file_path_argument);

    // Check if file exists
    if !file_path.exists() {
        return Err(create_file_system_error(
            &format!("CSV file does not exist: {}", csv_file_path_argument),
            std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")
        ));
    }

    // Check if it's actually a file (not a directory)
    if !file_path.is_file() {
        return Err(create_configuration_error(
            &format!("Path exists but is not a file: {}", csv_file_path_argument)
        ));
    }

    // Check file extension suggests CSV format
    let file_extension = file_path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    match file_extension.as_deref() {
        Some("csv") | Some("tsv") => {
            // File appears to be CSV format
        }
        Some(other_extension) => {
            println!("Warning: File extension '{}' is not typical for CSV files.", other_extension);
            println!("         Proceeding anyway, but ensure this is a comma-separated values file.");
            println!();
        }
        None => {
            println!("Warning: File has no extension. Ensure this is a comma-separated values file.");
            println!();
        }
    }

    // Convert to absolute path for consistent handling
    let absolute_file_path = file_path.canonicalize()
        .map_err(|io_error| {
            create_file_system_error(
                &format!("Failed to resolve absolute path for: {}", csv_file_path_argument),
                io_error
            )
        })?;

    Ok(absolute_file_path)
}
