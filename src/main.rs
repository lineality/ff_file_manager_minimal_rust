// src/main.rs

// import file fantstic module w/ these 2 lines
mod ff_file_fantastic_module;
use ff_file_fantastic_module::file_fantastic;

mod csv_processor_module;
mod error_types_module;
mod manage_absolute_executable_directory_relative_paths;
mod rows_and_columns_module;
// use rows_and_columns_module::rc_analyze_datafile_save_results_to_resultsfile;


fn main() {

    // Let's call File Fantastic Go!!
    if let Err(e) = file_fantastic() {

        // Handle errors
        eprintln!("Error: {}", e);

        // exit code one means ok!
        std::process::exit(1);
    }
}
