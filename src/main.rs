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

// show data & code file line counts
mod row_line_count_tui_module;

// ff as a module for any project, e.g.
fn main() {
    // Let's call File Fantastic
    match file_fantastic() {
        // // For testing / Inspection
        // /* Since ff has a cwd message, we don't need to reprint it
        //  * but as a module for another application,
        //  * you may want to do something with the output directory path
        //  * e.g.
        //  * Ok(returned_directory) => {
        //  * println!("Last directory: {}", returned_directory.display());
        //  */
        // // No need to re-print output
        // Ok(returned_directory) => {
        //     println!("Last directory: {}", returned_directory.display());
        // }
        // // Handle errors
        // Err(e) => {
        //     eprintln!("Error: {}", e);

        //     // exit code 1 (not zero) means there was an error
        //     std::process::exit(1);
        // }

        // No need to re-print output
        Ok(_) => {}
        // Handle errors
        Err(e) => {
            eprintln!("Error: {}", e);

            // exit code 1 (not zero) means there was an error
            std::process::exit(1);
        }
    }
}
