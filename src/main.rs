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
use source_it_module::{SourcedFile, handle_sourceit_command};

// Developer explicitly lists files to embed
const FF_SOURCE_FILES: &[SourcedFile] = &[
    SourcedFile::new("Cargo.toml", include_str!("../Cargo.toml")),
    SourcedFile::new("src/main.rs", include_str!("main.rs")),
    SourcedFile::new(
        "src/csv_processor_module.rs",
        include_str!("csv_processor_module.rs"),
    ),
    SourcedFile::new(
        "src/error_types_module.rs",
        include_str!("error_types_module.rs"),
    ),
    SourcedFile::new(
        "src/ff_file_fantastic_module.rs",
        include_str!("ff_file_fantastic_module.rs"),
    ),
    SourcedFile::new(
        "src/rows_and_columns_module.rs",
        include_str!("rows_and_columns_module.rs"),
    ),
    SourcedFile::new(
        "src/source_it_module.rs",
        include_str!("source_it_module.rs"),
    ),
    // SourcedFile::new("src/lib.rs", include_str!("lib.rs")),
    SourcedFile::new("README.md", include_str!("../README.md")),
    // SourcedFile::new("LICENSE", include_str!("../LICENSE")),
    SourcedFile::new(".gitignore", include_str!("../.gitignore")),
];

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--source".to_string()) {
        match handle_sourceit_command("ff_file_fantastic", None, FF_SOURCE_FILES) {
            Ok(path) => println!("Source extracted to: {}", path.display()),
            Err(e) => eprintln!("Failed to extract source: {}", e),
        }
        return;
    }

    // Let's call File Fantastic Go!!
    if let Err(e) = file_fantastic() {
        // Handle errors
        eprintln!("Error: {}", e);

        // exit code one means ok!
        std::process::exit(1);
    }
}
