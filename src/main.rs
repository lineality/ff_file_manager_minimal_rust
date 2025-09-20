// src/main.rs

// import file fantstic module w/ these 2 lines
mod ff_file_fantastic_module;
use ff_file_fantastic_module::file_fantastic;

// import rows and columns helper module w/ these 3 lines
mod csv_processor_module;
mod error_types_module;
mod rows_and_columns_module;

// Share Source
mod source_it_module;

// ff as a module for any project
fn main() {
    // Let's call File Fantastic Go!!
    if let Err(e) = file_fantastic() {
        // Handle errors
        eprintln!("Error: {}", e);

        // exit code one means ok!
        std::process::exit(1);
    }
}
