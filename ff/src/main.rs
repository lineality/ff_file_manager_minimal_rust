// src/main.rs

// import file fantstic module w/ these 2 lines
mod ff_file_fantastic_module;
use ff_file_fantastic_module::file_fantastic;

fn main() {
    
    // Let's call File Fantastic Go!!
    if let Err(e) = file_fantastic() {
        
        // Handle errors
        eprintln!("Error: {}", e);
        
        // exit code one means ok!
        std::process::exit(1);
    }
}

