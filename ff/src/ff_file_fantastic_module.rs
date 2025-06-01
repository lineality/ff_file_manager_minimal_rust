// src/lib.rs (or src/ff_file_fantastic_module.rs)
/// ff - A minimal file manager in Rust
/// use -> cargo build --profile release-performance
/// or, use -> cargo build --profile release-small 
/*Rust
Always best practice.
Always extensive doc strings.
Always comments.
Never remove documentation. 
Always clear, meaningful, unique names.
Always absolute file paths.
Always error handling.
Never unsafe code.
Never use unwrap.
*/
/*
*/
/* Docs: README.md
# ff_file_manager_minimal_rust



## Build
see https://github.com/lineality/rust_compile_optimizations_cheatsheet

#### For smallest size, build (~0.48 mb)
```bash
cargo build --profile release-small 
```
#### or for optimal performance (~6 mb)
```bash
cargo build --profile release-performance
```

## ~Install
Put your executable-binary somewhere, and set the file-name of that file
as a keyword for your command line interface (CLI) 
so that entering that keyword calls the executable (starts the program):

1. Make or get the binary executable and put it somewhere: e.g.
```path
/home/YOURCOMPUTERNAME/ff_file_browser/ff
```
2. Open the bash shell configuration file in a text editor. The configuration file is usually located at ~/.bashrc or ~/.bash_profile. (use whatever editor: vim, nano, hx (helix), gedit, lapce, teehee, lapce, etc.)
```bash
hx ~/.bashrc
```
or in some systems it may be called 'bash_profile'

3. Add an "alias" for your executable at the end of your bash file. Replace /path/to/your_executable with the path of your executable. And replace "your_keyword" with whatever you want to call File Fantastic by typing into your terminal. Add this line (with your details put in):
```text
alias your_keyword='/path/to/your_executable'
```
e.g. add:
```text
alias ff='/home/COMPUTERNAME/ff_file_browser/ff'
```

4. Save and close the text editor. 
- If you used nano, you can do this by pressing: Ctrl x s (control key, x key, s key)
- If you use Helix(hx), Vim(vi), or Teehee: 'i' to type, then esc for normal mode, then :wq to write and quit

4. Reload the bash-shell configuration file, and maybe open a new terminal, to apply and use the changes.
```bash
source ~/.bashrc
```
or bash_profile

Now you should be able to call File Fantastic by typing 'ff' (or whatever you choose) into a terminal.


# ff is a minimal rust file manager

## A very minimal 'file browser/explorer' module, much more minimal than "midnight commander." 

### Sample main file to use this module
```rust
// src/main.rs

// import file fantastic module w/ these 2 lines
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

```

# Functionality Goal:
ff (File Fantastic) is meant to operate in a 'do one thing well' context.
A plain posix (unix, bsd, linux, etc.) terminal is very useful
and important to use, but common useful features that have become
conventions of file-folder-explorers/managers/browsers are missing,
such as seeing file sizes, sorting by last-modified date, etc. 
with ff (File Fantastic) it is (hopefully) simple to ~bridge or interface the
common conventions of file-explorers with the open-ended utility of the terminal,
without adding too many redundant features to File Fantastic.

Terminals are great for lower level tasks such as 
making new directories, renaming, removing, etc.,
but terminals are not very user-friendly for seeing and navigating 'where'
you are in the data.

### Do One Thing Well
If in a terminal you want the navigation-features of a file-explorer just type 'ff'
and there you are: that terminal view is enhanced with file-folder visibility. 
If in File Fantastic you want the lower-level utility of a terminal, 
type 't' and there you are: a terminal opens where you were. Use the right
tool for the job, not a multi-function-monster that does many things badly.

### A Stable Backup: 
File Fantastic is not expected to completely replace the GUI fancy file-manager
that usually is a default in an OS. But it does happen, often more frequently
than one might expect, that the default fancy GUI file explorer tool does
not work. In such cases it is nice to have a stable backup option. 

# Design Scope:
1. no third party dependencies
2. docstrings required
3. code comments required
4. no unwrap
5. no unsafe code
6. all errors to be handled
7. terminal cli application
8. Module to be used by other projects

# Main functions/features:
1. (very) minimal text user interface
- path, then numbered lines
- like bash: ls, but list by number
- show as columns: item number name size modified
2. primarily int + enter/return for user-input
3. select directory to go to by number
4. 'b' to go back; back-up directory path, go to parent directory
5. enter file to open by number; use Q&A to use editor of choice
6. default to default program with another return/enter
7. open file in new terminal
8. hit enter to refresh
11. single letter commands
12. legend shows command 'words': use first letter as command
(q)uit (b)ack|(t)erminal|(d)ir (f)ile|(n)ame (s)ize (m)od|str>search|enter>reset
w for up, s for down, a for all 
13. 'sort by size' ' 'sort by name' 'sort by last-modified': re-selecting a sort option reverses the order
14. Type a string for a partial match search.
15. 'f' or 'd' to show only files or only directories
16. Minimal file-moving ("Get-Send Mode")
17. Save and Change Navigation-State ("Pocket Dimensions")
18. Archive Selection: basic archive feature for versioning and backup

# Scrolling
1. MVP: use mouse wheel to scroll up and down
2. pages using w and s to scroll up and down

# Get-Send Mode: Move Files
This is a minimal/modular system of ~features for copying files.
if the user enters any of these: v, c, y, p, g

1. store file-path from cwd to file-stack (start Q&A)
2. get file from file-stack (save here) (start Q&A)
3. store directory-path to directory-stack
4. Save: current navigation-state to -> pocket-dimensions
5. Go To: Pocket-Dimension (select from saved navigation states) 
6. Go To: path -> from directory-stack [set this up later]
7. clear 
8. return/exit/back (same as "" or "b")

# Example daily workflow:
- open terminal
- type ff to start file manager/browser
- see list of directories and files by number
  with sort/re-sort
- select item by number
- select directory by number, that becomes next: like cd /dir
- select file by number

## List-item Size:
- show file size in terms of b kb mb or gb depending on
if the size is no more than 99 of that unit
.1 mb, 99 k, 99 b etc.

## TUI Size:
- default terminal size 80/24
- or first MVP, terminal size is default terminal size
- for MVP...mouse to scroll up and down works fine for mvp
*/

/// ff - A minimal file manager in Rust
/// use -> cargo build --profile release-performance
/// or, use -> cargo build --profile release-small 
///
/// # File Fantastic (ff) - A minimal file manager
///
/// ## Overview
/// File Fantastic provides a lightweight, terminal-based file navigation and management
/// interface that bridges the gap between command-line terminals and graphical file explorers.
///
/// ## Sample main file to use this module
/// ```rust
/// // src/main.rs
///
/// // import file fantastic module w/ these 2 lines
/// mod ff_file_fantastic_module;
/// use ff_file_fantastic_module::file_fantastic;
///
/// fn main() {
///     
///     // Let's call File Fantastic Go!!
///     if let Err(e) = file_fantastic() {
///         
///         // Handle errors
///         eprintln!("Error: {}", e);
///         
///         // exit code one means error occurred
///         std::process::exit(1);
///     }
/// }
/// ```
///
/// # Functionality Goal:
/// ff (File Fantastic) is meant to operate in a 'do one thing well' context.
/// A plain posix (unix, bsd, linux, etc.) terminal is very useful
/// and important to use, but common useful features that have become
/// conventions of file-folder-explorers/managers/browsers are missing,
/// such as seeing file sizes, sorting by last-modified date, etc. 
/// with ff (File Fantastic) it is (hopefully) simple to ~bridge or interface the
/// common conventions of file-explorers with the open-ended utility of the terminal,
/// without adding too many redundant features to File Fantastic.
///
/// Terminals are great for lower level tasks such as 
/// making new directories, renaming, removing, etc.,
/// but terminals are not very user-friendly for seeing and navigating 'where'
/// you are in the data.
///
/// ## Do One Thing Well
/// If in a terminal you want the navigation-features of a file-explorer just type 'ff'
/// and there you are: that terminal view is enhanced with file-folder visibility. 
/// If in File Fantastic you want the lower-level utility of a terminal, 
/// type 't' and there you are: a terminal opens where you were. Use the right
/// tool for the job, not a multi-function-monster that does many things badly.
///
/// ## A Stable Backup: 
/// File Fantastic is not expected to completely replace the GUI fancy file-manager
/// that usually is a default in an OS. But it does happen, often more frequently
/// than one might expect, that the default fancy GUI file explorer tool does
/// not work. In such cases it is nice to have a stable backup option.
///
/// # Design Scope:
/// 1. use best practice
/// 2. absolute file paths
/// 3. no third party dependencies
/// 4. docstrings required
/// 4. code comments required
/// 5. clear unique meaningful naming required
/// 6. no unwrap
/// 7. no unsafe code
/// 8. all errors to be handled
/// 9. terminal cli application
/// 10. module to be used in other projects
///
/// # Main functions/features:
/// 1. (very) minimal text user interface
///   - path, then numbered lines
///   - like bash: ls, but list by number
///   - show as columns: item number name size modified
/// 2. primarily int + enter/return for user-input
/// 3. select directory to go to by number
/// 4. 'b' to go back; back-up directory path, go to parent directory
/// 5. enter file to open by number; use Q&A to use editor of choice
/// 6. default to default program with another return/enter
/// 7. open file in new terminal: note, due to using os-default if available,
///    File Fantastic can open image or other files, at least sometimes.
/// 8. hit enter to refresh
/// 11. single letter commands
/// 12. legend shows command 'words': use first letter as command
///     (q)uit (b)ack|(t)erminal|(d)ir (f)ile|(n)ame (s)ize (m)od|str>search|enter>reset
///     w for up, s for down, a for all 
/// 13. 'sort by size' ' 'sort by name' 'sort by last-modified': re-selecting a sort option reverses the order
/// 14. Type a string for a partial match search.
/// 15. 'f' or 'd' to show only files or only directories
///
/// # Scrolling
/// 1. MVP: use mouse wheel to scroll up and down
/// 2. pages using w and s to scroll up and down
///
/// # Example daily workflow:
/// - open terminal
/// - type ff to start file manager/browser
/// - see list of directories and files by number
///   with sort/re-sort
/// - select item by number
/// - select directory by number, that becomes next: like cd /dir
/// - select file by number
///
/// ## List-item Size:
/// - show file size in terms of b kb mb or gb depending on
///   if the size is no more than 99 of that unit
///   .1 mb, 99 k, 99 b etc.
///
/// ## TUI Size:
/// - default terminal size 80/24
/// - or first MVP, terminal size is default terminal size
/// - for MVP...mouse to scroll up and down works fine for mvp

use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// Maximum Levenshtein distance to consider a match
const MAX_SEARCH_DISTANCE: usize = 2;
const MAX_NAME_LENGTH: usize = 55;
const FILENAME_SUFFIX_LENGTH: usize = 5;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
// const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
// const BLUE: &str = "\x1b[34m";
// const BOLD: &str = "\x1b[1m";
// const ITALIC: &str = "\x1b[3m";
// const UNDERLINE: &str = "\x1b[4m";

/*
Error Handling section starts
*/

/// Custom error type for the file manager that provides specific error contexts
/// and better error messages for users.
///
/// # Error Categories
/// This enum categorizes errors that can occur during file manager operations
/// to enable appropriate handling and user-friendly messages.
///
/// # Purpose
/// - Provides meaningful context beyond standard IO errors
/// - Enables specific recovery strategies based on error type
/// - Creates user-friendly error messages
/// - Supports the standard error trait ecosystem
///
/// # Error Handling Strategy
/// Different error variants allow for different recovery approaches:
/// - `NotFound` errors might trigger navigation to parent directory
/// - `PermissionDenied` errors might prompt for elevated privileges
/// - `EditorLaunchFailed` errors can fall back to system defaults
///
/// # Examples
/// ```rust
/// // Creating a not found error
/// let error = FileFantasticError::NotFound(path);
/// 
/// // Converting an IO error
/// let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
/// let error = FileFantasticError::from(io_err);
///
/// // Example error handling
/// match operation() {
///     Err(FileFantasticError::PermissionDenied(path)) => {
///         eprintln!("Cannot access {}: permission denied", path.display());
///         // Try alternative approach with elevated permissions
///     },
///     Err(FileFantasticError::NotFound(path)) => {
///         eprintln!("The path {} no longer exists", path.display());
///         // Navigate to parent directory
///     },
///     // Other error types...
/// }
/// ```
#[derive(Debug)]
pub enum FileFantasticError {
    /// Standard IO errors with the original error preserved
    Io(io::Error),
    
    /// File or directory not found with the path that was attempted
    NotFound(PathBuf),
    
    /// Permission denied when accessing a file or directory
    PermissionDenied(PathBuf),
    
    /// Invalid file or directory name that cannot be processed
    InvalidName(String),
    
    /// No suitable terminal found for the current platform
    NoTerminalFound,
    
    /// Failed to read metadata for a file or directory
    MetadataError(PathBuf),
    
    /// Failed to launch the specified editor
    EditorLaunchFailed(String),
    
    /// Current platform is not supported for a specific operation
    UnsupportedPlatform,
}

impl std::fmt::Display for FileFantasticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {}", err),
            Self::NotFound(path) => write!(f, "Not found: {}", path.display()),
            Self::PermissionDenied(path) => write!(f, "Permission denied: {}", path.display()),
            Self::InvalidName(name) => write!(f, "Invalid file name: {}", name),
            Self::NoTerminalFound => write!(f, "No suitable terminal emulator found"),
            Self::MetadataError(path) => write!(f, "Failed to read metadata for: {}", path.display()),
            Self::EditorLaunchFailed(editor) => write!(f, "Failed to launch editor: {}", editor),
            Self::UnsupportedPlatform => write!(f, "Current platform is not supported"),
        }
    }
}

impl std::error::Error for FileFantasticError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for FileFantasticError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => {
                // Create a generic PathBuf for the error context
                // In practice, the actual path should be provided when possible
                let path = PathBuf::from("<path unknown>");
                Self::NotFound(path)
            }
            io::ErrorKind::PermissionDenied => {
                let path = PathBuf::from("<path unknown>");
                Self::PermissionDenied(path)
            }
            _ => Self::Io(err),
        }
    }
}

/// Creates a zip archive of a directory with timestamped filename and optional custom name
/// 
/// # Purpose
/// Compresses an entire directory into a timestamped zip file for backup
/// or archival purposes, with an optional custom name addition.
/// 
/// # Arguments
/// * `source_directory_path` - Path to the directory to archive
/// * `destination_directory_path` - Where to place the zip file
/// * `custom_name_addition` - Optional custom text to add before timestamp
/// 
/// # Returns
/// * `Result<PathBuf>` - Path to the created zip file, or error
/// 
/// # Filename Format
/// - Without custom name: `directory_name_2025_01_15_14_30_45.zip`
/// - With custom name: `directory_name_custom_2025_01_15_14_30_45.zip`
/// 
/// # Example
/// ```rust
/// // Basic archive
/// let result = create_directory_zip_archive(&source, &dest, None);
/// // Creates: my_project_2025_01_15_14_30_45.zip
/// 
/// // Custom named archive
/// let result = create_directory_zip_archive(&source, &dest, Some("backup"));
/// // Creates: my_project_backup_2025_01_15_14_30_45.zip
/// ```
fn create_directory_zip_archive(
    source_directory_path: &PathBuf,
    destination_directory_path: &PathBuf,
    custom_name_addition: Option<&str>,
) -> Result<PathBuf> {
    // Validate source directory exists and is a directory
    if !source_directory_path.exists() {
        return Err(FileFantasticError::NotFound(source_directory_path.clone()));
    }
    
    if !source_directory_path.is_dir() {
        return Err(FileFantasticError::InvalidName(
            format!("Source is not a directory: {}", source_directory_path.display())
        ));
    }
    
    // Validate destination directory exists and is writable
    if !destination_directory_path.exists() {
        return Err(FileFantasticError::NotFound(destination_directory_path.clone()));
    }
    
    if !destination_directory_path.is_dir() {
        return Err(FileFantasticError::InvalidName(
            format!("Destination is not a directory: {}", destination_directory_path.display())
        ));
    }
    
    // Extract source directory name
    let source_directory_name = source_directory_path.file_name()
        .ok_or_else(|| FileFantasticError::InvalidName(
            format!("Cannot determine directory name from: {}", source_directory_path.display())
        ))?
        .to_string_lossy()
        .to_string();
    
    // Generate timestamped zip filename with optional custom name
    let timestamp = generate_archive_timestamp();
    let zip_filename = if let Some(custom_name) = custom_name_addition {
        if custom_name.trim().is_empty() {
            // Empty custom name, use standard format
            format!("{}_{}.zip", source_directory_name, timestamp)
        } else {
            // Include custom name before timestamp
            format!("{}_{}_{}.zip", source_directory_name, custom_name.trim(), timestamp)
        }
    } else {
        // No custom name, use standard format
        format!("{}_{}.zip", source_directory_name, timestamp)
    };
    
    let zip_destination_path = destination_directory_path.join(&zip_filename);
    
    // Create zip archive using system commands
    let zip_result = create_zip_with_system_command(
        source_directory_path,
        &zip_destination_path,
    )?;
    
    if zip_result {
        println!("Directory archived: {}", zip_destination_path.display());
        Ok(zip_destination_path)
    } else {
        Err(FileFantasticError::InvalidName(
            "Zip creation failed".to_string()
        ))
    }
}

// /// Creates a zip archive of a directory with timestamped filename
// /// 
// /// # Purpose
// /// Compresses an entire directory into a timestamped zip file for backup
// /// or archival purposes, preserving the directory structure and all contents.
// /// 
// /// # Arguments
// /// * `source_directory_path` - Path to the directory to archive
// /// * `destination_directory_path` - Where to place the zip file
// /// 
// /// # Returns
// /// * `Result<PathBuf>` - Path to the created zip file, or error
// /// 
// /// # Zip Creation Strategy
// /// 1. Generate timestamped filename for the zip archive
// /// 2. Use system zip command for cross-platform compatibility
// /// 3. Create zip in destination directory with full directory contents
// /// 4. Preserve directory structure and permissions when possible
// /// 
// /// # Platform Commands
// /// - Linux/macOS: Uses `zip -r archive.zip source_directory`
// /// - Windows: Uses PowerShell `Compress-Archive` command
// /// 
// /// # Error Conditions
// /// - Source directory doesn't exist or isn't accessible
// /// - Destination directory doesn't exist or isn't writable
// /// - System zip command not available or fails
// /// - Insufficient disk space for zip creation
// /// 
// /// # Example
// /// ```rust
// /// let source = PathBuf::from("/home/user/my_project");
// /// let destination = PathBuf::from("/home/user/backups");
// /// 
// /// match create_directory_zip_archive(&source, &destination) {
// ///     Ok(zip_path) => {
// ///         println!("Directory archived: {}", zip_path.display());
// ///     },
// ///     Err(e) => {
// ///         eprintln!("Failed to create archive: {}", e);
// ///     }
// /// }
// /// ```
// fn create_directory_zip_archive(
//     source_directory_path: &PathBuf,
//     destination_directory_path: &PathBuf,
// ) -> Result<PathBuf> {
//     // Validate source directory exists and is a directory
//     if !source_directory_path.exists() {
//         return Err(FileFantasticError::NotFound(source_directory_path.clone()));
//     }
    
//     if !source_directory_path.is_dir() {
//         return Err(FileFantasticError::InvalidName(
//             format!("Source is not a directory: {}", source_directory_path.display())
//         ));
//     }
    
//     // Validate destination directory exists and is writable
//     if !destination_directory_path.exists() {
//         return Err(FileFantasticError::NotFound(destination_directory_path.clone()));
//     }
    
//     if !destination_directory_path.is_dir() {
//         return Err(FileFantasticError::InvalidName(
//             format!("Destination is not a directory: {}", destination_directory_path.display())
//         ));
//     }
    
//     // Extract source directory name
//     let source_directory_name = source_directory_path.file_name()
//         .ok_or_else(|| FileFantasticError::InvalidName(
//             format!("Cannot determine directory name from: {}", source_directory_path.display())
//         ))?
//         .to_string_lossy()
//         .to_string();
    
//     // Generate timestamped zip filename
//     let timestamp = generate_archive_timestamp();
//     let zip_filename = format!("{}_{}.zip", source_directory_name, timestamp);
//     let zip_destination_path = destination_directory_path.join(&zip_filename);
    
//     // Create zip archive using system commands
//     let zip_result = create_zip_with_system_command(
//         source_directory_path,
//         &zip_destination_path,
//     )?;
    
//     if zip_result {
//         println!("Directory archived: {}", zip_destination_path.display());
//         Ok(zip_destination_path)
//     } else {
//         Err(FileFantasticError::InvalidName(
//             "Zip creation failed".to_string()
//         ))
//     }
// }

/// Creates a zip file using platform-appropriate system commands
/// 
/// # Purpose
/// Executes platform-specific zip creation commands to compress directories,
/// avoiding external dependencies while providing cross-platform functionality.
/// 
/// # Arguments
/// * `source_path` - Directory to compress
/// * `zip_path` - Output path for the zip file
/// 
/// # Returns
/// * `Result<bool>` - True if zip creation succeeded, false if failed
/// 
/// # Platform Implementation
/// - **Linux/macOS**: Uses `zip -r` command for recursive compression
/// - **Windows**: Uses PowerShell `Compress-Archive` cmdlet
/// 
/// # Command Details
/// ## Linux/macOS
/// ```bash
/// zip -r "output.zip" "source_directory/"
/// ```
/// 
/// ## Windows
/// ```powershell
/// Compress-Archive -Path "source_directory" -DestinationPath "output.zip"
/// ```
/// 
/// # Error Handling
/// - Handles command execution failures
/// - Checks exit status of zip commands
/// - Provides platform-specific error context
/// 
/// # Example
/// ```rust
/// let source = PathBuf::from("/home/user/documents");
/// let zip_file = PathBuf::from("/home/user/documents_backup.zip");
/// 
/// match create_zip_with_system_command(&source, &zip_file) {
///     Ok(true) => println!("Zip created successfully"),
///     Ok(false) => println!("Zip command failed"),
///     Err(e) => eprintln!("Error executing zip command: {}", e),
/// }
/// ```
fn create_zip_with_system_command(
    source_path: &PathBuf,
    zip_path: &PathBuf,
) -> Result<bool> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        // Use zip command on Unix-like systems
        let output = std::process::Command::new("zip")
            .arg("-r")  // Recursive
            .arg(zip_path)
            .arg(source_path)
            .output()
            .map_err(|e| {
                eprintln!("Failed to execute zip command: {}", e);
                eprintln!("Make sure 'zip' is installed on your system");
                FileFantasticError::Io(e)
            })?;
            
        if output.status.success() {
            Ok(true)
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            eprintln!("Zip command failed: {}", error_msg);
            Ok(false)
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // Use PowerShell Compress-Archive on Windows
        let output = std::process::Command::new("powershell")
            .arg("-Command")
            .arg(format!(
                "Compress-Archive -Path '{}' -DestinationPath '{}'",
                source_path.display(),
                zip_path.display()
            ))
            .output()
            .map_err(|e| {
                eprintln!("Failed to execute PowerShell compress command: {}", e);
                FileFantasticError::Io(e)
            })?;
            
        if output.status.success() {
            Ok(true)
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            eprintln!("PowerShell compress command failed: {}", error_msg);
            Ok(false)
        }
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err(FileFantasticError::UnsupportedPlatform)
    }
}

/// Result type for file manager operations
/// 
/// This type alias simplifies function signatures throughout the codebase
/// and ensures consistent error handling.
/// 
/// # Examples
/// ```
/// fn example_function() -> Result<()> {
///     // Function implementation
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, FileFantasticError>;
/*
End of Error Handling Code
*/

/*
Pocket-Dimensions/Bookmarks Section
*/
/// Complete navigation state that can be saved and restored as a "pocket dimension"
/// 
/// # Purpose
/// Captures the complete state of the user's current view and navigation context,
/// allowing them to save their exact position and return to it later.
/// This enables "pocket dimensions" - bookmarked navigation states that preserve
/// not just the directory location, but the complete user experience including
/// sort order, filters, pagination state, and selected items.
/// 
/// # Design Philosophy
/// The pocket dimension system is designed around the concept of "teleportation"
/// between different navigation contexts. Users can save their current "location"
/// in the file system along with all their preferences and state, then later
/// return to that exact same state as if they never left.
/// 
/// # Fields
/// All navigation context needed to restore the exact user experience:
/// - Directory location and view settings
/// - Pagination and display state  
/// - User interaction context
/// - Metadata for management
/// 
/// # Usage Context
/// Used in workflows where users need to:
/// - Collect files from multiple locations
/// - Navigate between frequently used directories
/// - Maintain different views for different purposes
/// - Save complex filter/sort combinations
/// 
/// # Example Workflow
/// ```text
/// 1. User is in /home/user/projects with files filtered by .rs extension
/// 2. User saves this as pocket dimension "rust_files"
/// 3. User navigates to /var/log to check something
/// 4. User can instantly return to exact same state in /home/user/projects
/// ```
/// 
/// # Persistence
/// Currently stored in memory only. Future versions could persist to disk
/// for cross-session bookmark management.
#[derive(Debug, Clone)]
pub struct SavedNavigationState {
    /// The directory path the user was viewing when state was saved
    /// This is always an absolute path for consistency and reliability
    pub current_directory_path: PathBuf,
    
    /// How the directory contents were sorted at save time
    /// Preserves both the sort method (name/size/modified) and direction
    pub current_sort_method: DirectorySortingMethodEnum,
    
    /// What filter was applied (files only, dirs only, all)
    /// None means no filter was active (show all items)
    pub current_filter: Option<char>,
    
    /// Which page of results the user was viewing (0-based)
    /// Important for large directories where pagination is active
    pub current_page_number: usize,
    
    // /// Total number of pages available at save time
    // /// Used to validate restoration and provide context
    // pub total_pages: usize,
    
    /// Which item was selected/highlighted (1-based index)
    /// None if no item was specifically selected
    pub selected_item_index: Option<usize>,
    
    /// The search term that was active, if any
    /// Preserves active search state for complete restoration
    pub active_search_term: Option<String>,
    
    /// Terminal dimensions at save time (width, height)
    /// Used to detect if display needs adjustment on restoration
    pub terminal_size: (usize, usize),
    
    // /// Flag indicating this state should be returned to automatically
    // /// Used for workflows that involve temporary navigation
    // pub return_to_here_flag: bool,
    
    /// When this state was saved (for chronological sorting and cleanup)
    /// Enables automatic cleanup of old states and temporal organization
    pub timestamp: SystemTime,
    
    /// User-provided or auto-generated nickname for this state
    /// Used for display and selection in pocket dimension menus
    pub nickname: String,
    
    /// Auto-generated short description for display in lists
    /// Provides context about what this state represents
    pub description: String,
}

impl SavedNavigationState {
    /// Creates a new saved navigation state from current context
    /// 
    /// # Purpose
    /// Factory method that captures the complete current state of the file manager
    /// and packages it into a SavedNavigationState that can be stored and restored.
    /// 
    /// # Arguments
    /// * `current_directory_path` - Current directory being viewed
    /// * `nav_state` - Current NavigationState with sort/filter settings
    /// * `dir_view` - Current DirectoryView with pagination info
    /// * `selected_item` - Currently selected item index (if any)
    /// * `active_search` - Active search term (if any)
    /// * `nickname` - Optional user-provided nickname for this state
    /// 
    /// # Returns
    /// * `SavedNavigationState` - Complete state snapshot ready for storage
    /// 
    /// # State Capture Process
    /// 1. Records current directory path (always absolute)
    /// 2. Captures all navigation preferences (sort, filter)
    /// 3. Saves pagination state and selected item
    /// 4. Records search state if active
    /// 5. Captures terminal dimensions for display consistency
    /// 6. Generates nickname and description for user interface
    /// 7. Timestamps the state for management
    /// 
    /// # Nickname Generation
    /// If no nickname is provided, automatically generates one based on:
    /// - Directory name
    /// - Timestamp suffix for uniqueness
    /// - Truncation to reasonable length
    /// 
    /// # Example
    /// ```rust
    /// // Save current state as a pocket dimension
    /// let saved_state = SavedNavigationState::new(
    ///     current_directory_path.clone(),
    ///     &nav_state,
    ///     &dir_view,
    ///     Some(5), // Item 5 was selected
    ///     Some("rust".to_string()), // Search for "rust" was active
    ///     Some("my_workspace".to_string()), // User-provided nickname
    /// );
    /// ```
    pub fn new(
        current_directory_path: PathBuf,
        nav_state: &NavigationState,
        dir_view: &DirectoryView,
        selected_item: Option<usize>,
        active_search: Option<String>,
        nickname: Option<String>,
    ) -> Self {
        let timestamp = SystemTime::now();
        
        // Generate automatic nickname if none provided
        let auto_nickname = if let Some(name) = nickname {
            name
        } else {
            Self::generate_auto_nickname(&current_directory_path, timestamp)
        };
        
        // Generate description for display in pocket dimension lists
        let description = Self::generate_description(
            &current_directory_path,
            nav_state.current_filter,
            &nav_state.current_sort_method,
        );
        
        // Get terminal size (simplified - in real implementation, get from terminal)
        // TODO: Implement actual terminal size detection
        let terminal_size = (80, 24); // Default terminal size for now
        
        SavedNavigationState {
            current_directory_path,
            current_sort_method: nav_state.current_sort_method,
            current_filter: nav_state.current_filter,
            current_page_number: dir_view.current_page,
            // current_page_number: nav_state.current_page_index,
            // total_pages: dir_view.total_pages(),
            selected_item_index: selected_item,
            active_search_term: active_search,
            terminal_size,
            // return_to_here_flag: false, // Default to false, can be set later
            timestamp,
            nickname: auto_nickname,
            description,
        }
    }
    
    /// Generates an automatic nickname based on directory path and timestamp
    /// 
    /// # Purpose
    /// Creates a unique, human-readable identifier for a saved navigation state
    /// when the user doesn't provide a custom nickname.
    /// 
    /// # Arguments
    /// * `path` - The directory path being saved
    /// * `timestamp` - When the state was saved
    /// 
    /// # Returns
    /// * `String` - Generated nickname suitable for display and selection
    /// 
    /// # Nickname Format
    /// - Uses the last component of the path (directory name)
    /// - Appends a unique suffix based on timestamp
    /// - Falls back to "root" for root directory paths
    /// - Keeps nicknames concise but descriptive
    /// 
    /// # Uniqueness Strategy
    /// Uses the last 4 digits of the Unix timestamp to create uniqueness
    /// while keeping the nickname readable. This provides sufficient
    /// uniqueness for typical usage patterns.
    /// 
    /// # Examples
    /// ```text
    /// /home/user/projects -> "projects_1234"
    /// /var/log -> "log_5678"
    /// / -> "root_9012"
    /// ```
    fn generate_auto_nickname(path: &PathBuf, timestamp: SystemTime) -> String {
        // Extract the directory name from the path
        let path_name = path.file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "root".to_string());
            
        // Get timestamp seconds since epoch for uniqueness
        let timestamp_secs = timestamp.duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        // Use last 4 digits of timestamp for uniqueness while keeping it readable
        let unique_suffix = timestamp_secs % 10000;
        
        // Combine directory name with unique suffix
        format!("{}_{}", path_name, unique_suffix)
    }
    
    /// Generates a human-readable description of the navigation state
    /// 
    /// # Purpose
    /// Creates a concise summary of the state's characteristics for display
    /// in pocket dimension lists, helping users identify what each state represents.
    /// 
    /// # Arguments
    /// * `path` - The directory path of the state
    /// * `filter` - Active filter setting
    /// * `sort_method` - Current sort method and direction
    /// 
    /// # Returns
    /// * `String` - Formatted description showing key state characteristics
    /// 
    /// # Description Format
    /// Combines directory name with active settings:
    /// - Base directory name
    /// - Filter indicator (if active)
    /// - Sort method and direction indicator
    /// 
    /// # Filter Indicators
    /// - [dirs] - Only directories visible
    /// - [files] - Only files visible
    /// - (no indicator) - All items visible
    /// 
    /// # Sort Indicators
    /// - name↑ - Name ascending
    /// - name↓ - Name descending
    /// - size↑ - Size ascending
    /// - size↓ - Size descending
    /// - date↑ - Date ascending
    /// - date↓ - Date descending
    /// 
    /// # Examples
    /// ```text
    /// "projects [files] name↓" - Projects directory, files only, name descending
    /// "logs size↑" - Logs directory, all items, size ascending
    /// "src [dirs] date↓" - Source directory, directories only, date descending
    /// ```
    fn generate_description(
        path: &PathBuf,
        filter: Option<char>,
        sort_method: &DirectorySortingMethodEnum,
    ) -> String {
        // Generate filter description
        let filter_desc = match filter {
            Some('d') => " [dirs]",
            Some('f') => " [files]",
            _ => "", // No filter active
        };
        
        // Generate sort description with direction arrows
        let sort_desc = match sort_method {
            DirectorySortingMethodEnum::Name(asc) => if *asc { " name↑" } else { " name↓" },
            DirectorySortingMethodEnum::Size(asc) => if *asc { " size↑" } else { " size↓" },
            DirectorySortingMethodEnum::Modified(asc) => if *asc { " date↑" } else { " date↓" },
        };
        
        // Combine all elements into description
        format!("{}{}{}", 
                path.file_name().unwrap_or_default().to_string_lossy(),
                filter_desc,
                sort_desc)
    }
}

/// Manages stacks for file paths, directory paths, and saved navigation states
/// 
/// # Purpose
/// Provides a complete system for managing user's navigation context,
/// file operations, and "pocket dimension" bookmarks. This is the central
/// management system for all advanced navigation features.
/// 
/// # Design Philosophy
/// The NavigationStateManager implements a "multi-dimensional" approach to
/// file management where users can:
/// 1. Collect files and directories in stacks for batch operations
/// 2. Save and restore complete navigation states (pocket dimensions)
/// 3. Maintain multiple working contexts simultaneously
/// 4. Perform complex workflows spanning multiple directories
/// 
/// # Components
/// - **File path stack**: For collecting files to operate on
/// - **Directory path stack**: For destination directories and bookmarks
/// - **Pocket dimensions**: Saved navigation states with nicknames
/// - **State management**: Creation, storage, retrieval, and cleanup
/// 
/// # Usage Context
/// Enables complex workflows like:
/// 1. Save current location as pocket dimension
/// 2. Navigate to find files, add to file stack
/// 3. Navigate to destination, add to directory stack
/// 4. Return to original pocket dimension
/// 5. Perform batch operations using collected paths
/// 
/// # Memory Management
/// All data is stored in memory during the session. For production use,
/// consider adding persistence for pocket dimensions to survive restarts.
/// 
/// # Thread Safety
/// Not currently thread-safe. If multi-threading is needed, add appropriate
/// synchronization mechanisms.
#[derive(Debug)]
pub struct NavigationStateManager {
    /// Stack of file paths collected by user for operations
    /// Files are added to the end and removed from the end (LIFO)
    /// Used for collecting files for batch operations like copy/move
    pub file_path_stack: Vec<PathBuf>,
    
    /// Stack of directory paths for destinations and bookmarks
    /// Directories are added to the end and removed from the end (LIFO)
    /// Used for storing destination directories and frequently accessed locations
    pub directory_path_stack: Vec<PathBuf>,
    
    /// Saved navigation states with user-friendly nicknames
    /// Key: nickname (user-provided or auto-generated)
    /// Value: complete saved navigation state
    /// Enables instant teleportation between different navigation contexts
    pub pocket_dimensions: HashMap<String, SavedNavigationState>,
    
    /// Counter for auto-generating numeric nicknames when needed
    /// Ensures uniqueness when user doesn't provide custom nicknames
    /// Incremented each time an auto-nickname is needed
    auto_nickname_counter: usize,
}

/// Interactive user interface functions for Get-Send-Mode workflow
/// 
/// # Purpose
/// This implementation block provides all the interactive Q&A functions
/// that implement the Get-Send-Mode user interface. These functions
/// handle user prompts, input validation, and workflow management.
/// 
/// # Design Philosophy
/// Each function follows a consistent pattern:
/// 1. Display clear prompts and options
/// 2. Handle user input with validation
/// 3. Provide feedback on actions taken
/// 4. Graceful error handling with user-friendly messages
/// 5. **INTEGRATES WITH EXISTING NUMBERED SELECTION SYSTEM**
/// 
/// # User Interface Consistency
/// All functions use the same numbered selection system as the main file browser,
/// maintaining consistency throughout the application.
impl NavigationStateManager {
    /// Creates a new navigation state manager with empty stacks and collections
    /// 
    /// # Purpose
    /// Factory method that initializes a new NavigationStateManager with
    /// all collections empty and ready for use.
    /// 
    /// # Returns
    /// * `NavigationStateManager` - New instance ready for operation
    /// 
    /// # Initial State
    /// - Empty file path stack
    /// - Empty directory path stack
    /// - Empty pocket dimensions collection
    /// - Auto-nickname counter starts at 1
    /// 
    /// # Usage
    /// Should be called once at application startup to create the manager
    /// that will persist throughout the session.
    /// 
    /// # Example
    /// ```rust
    /// let mut state_manager = NavigationStateManager::new();
    /// // Ready to use for collecting paths and saving states
    /// ```
    pub fn new() -> Self {
        NavigationStateManager {
            file_path_stack: Vec::new(),
            directory_path_stack: Vec::new(),
            pocket_dimensions: HashMap::new(),
            auto_nickname_counter: 1, // Start at 1 for human-friendly numbering
        }
    }
    
    /// Saves current navigation state as a pocket dimension with optional nickname
    /// 
    /// # Purpose
    /// Captures the complete current state of the file manager and stores it
    /// with a nickname for later retrieval. This enables users to bookmark
    /// specific navigation contexts and return to them instantly.
    /// 
    /// # Arguments
    /// * `current_directory_path` - Current directory being viewed
    /// * `nav_state` - Current navigation state with preferences
    /// * `dir_view` - Current directory view with pagination
    /// * `selected_item` - Currently selected item index
    /// * `active_search` - Active search term if any
    /// * `nickname` - Optional nickname (auto-generated if None)
    /// 
    /// # Returns
    /// * `Result<String>` - The nickname used for the saved state, or error
    /// 
    /// # Duplicate Handling
    /// If a pocket dimension with the same nickname already exists:
    /// 1. Warns the user about overwriting
    /// 2. Asks for confirmation
    /// 3. Proceeds only if user confirms with 'y'
    /// 4. Returns error if user cancels
    /// 
    /// # Error Conditions
    /// - IO errors during user confirmation prompt
    /// - User cancellation of overwrite operation
    /// - Invalid characters in nickname (future enhancement)
    /// 
    /// # Example
    /// ```rust
    /// // Save current state with custom nickname
    /// match state_manager.save_pocket_dimension(
    ///     current_path.clone(),
    ///     &nav_state,
    ///     &dir_view,
    ///     Some(3), // Item 3 selected
    ///     None, // No active search
    ///     Some("workspace".to_string())
    /// ) {
    ///     Ok(nickname) => println!("Saved as: {}", nickname),
    ///     Err(e) => println!("Failed to save: {}", e),
    /// }
    /// ```
    pub fn save_pocket_dimension(
        &mut self,
        current_directory_path: PathBuf,
        nav_state: &NavigationState,
        dir_view: &DirectoryView,
        selected_item: Option<usize>,
        active_search: Option<String>,
        nickname: Option<String>,
    ) -> Result<String> {
        // Create the saved state using the factory method
        let saved_state = SavedNavigationState::new(
            current_directory_path,
            nav_state,
            dir_view,
            selected_item,
            active_search,
            nickname.clone(),
        );
        
        let final_nickname = saved_state.nickname.clone();
        
        // Check for duplicate nicknames and handle them gracefully
        if self.pocket_dimensions.contains_key(&final_nickname) {
            println!("Warning: Pocket dimension '{}' already exists and will be overwritten.", 
                     final_nickname);
            print!("Continue? (y/N): ");
            io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
            
            let mut response = String::new();
            io::stdin().read_line(&mut response).map_err(|e| FileFantasticError::Io(e))?;
            
            // Only proceed if user explicitly confirms with 'y'
            if !response.trim().eq_ignore_ascii_case("y") {
                return Err(FileFantasticError::InvalidName("Operation cancelled by user".to_string()));
            }
        }
        
        // Store the pocket dimension
        self.pocket_dimensions.insert(final_nickname.clone(), saved_state);
        Ok(final_nickname)
    }
    
    /// Lists all saved pocket dimensions sorted by timestamp (newest first)
    /// 
    /// # Purpose
    /// Provides access to all saved pocket dimensions in a sorted format
    /// suitable for display to the user. The temporal sorting helps users
    /// find recently created dimensions easily.
    /// 
    /// # Returns
    /// * `Vec<(&String, &SavedNavigationState)>` - Vector of (nickname, state) tuples
    /// 
    /// # Sorting Behavior
    /// - Primary sort: Timestamp (newest first)
    /// - This puts recently saved states at the top of lists
    /// - Helps users find their most recent work quickly
    /// 
    /// # Usage Context
    /// Used by UI functions that need to display pocket dimensions for selection:
    /// - Interactive selection menus
    /// - Status displays
    /// - Management interfaces
    /// 
    /// # Example
    /// ```rust
    /// let dimensions = state_manager.list_pocket_dimensions();
    /// for (i, (nickname, state)) in dimensions.iter().enumerate() {
    ///     println!("{}. {} - {}", i + 1, nickname, state.description);
    /// }
    /// ```
    pub fn list_pocket_dimensions(&self) -> Vec<(&String, &SavedNavigationState)> {
        let mut dimensions: Vec<_> = self.pocket_dimensions.iter().collect();
        
        // Sort by timestamp (newest first) for better user experience
        dimensions.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
        
        dimensions
    }
    
    /// Restores a navigation state by nickname
    /// 
    /// # Purpose
    /// Retrieves a previously saved navigation state by its nickname,
    /// returning a complete copy that can be used to restore the
    /// file manager to that exact state.
    /// 
    /// # Arguments
    /// * `nickname` - The nickname of the pocket dimension to restore
    /// 
    /// # Returns
    /// * `Result<SavedNavigationState>` - The complete saved state, or error
    /// 
    /// # Error Conditions
    /// - Nickname not found in the pocket dimensions collection
    /// - Invalid or corrupted state data (future enhancement)
    /// 
    /// # State Restoration
    /// The returned state contains all information needed to restore:
    /// - Directory path
    /// - Sort preferences
    /// - Filter settings
    /// - Pagination state
    /// - Selected item
    /// - Search context
    /// - Terminal size information
    /// 
    /// # Usage
    /// Typically called when user selects a pocket dimension to jump to.
    /// The caller is responsible for applying the returned state to
    /// the current navigation context.
    /// 
    /// # Example
    /// ```rust
    /// match state_manager.restore_pocket_dimension("workspace") {
    ///     Ok(saved_state) => {
    ///         // Apply the saved state to current navigation
    ///         current_directory = saved_state.current_directory_path;
    ///         nav_state.current_sort_method = saved_state.current_sort_method;
    ///         // ... restore other state components
    ///     },
    ///     Err(e) => println!("Failed to restore: {}", e),
    /// }
    /// ```
    pub fn restore_pocket_dimension(&self, nickname: &str) -> Result<SavedNavigationState> {
        self.pocket_dimensions
            .get(nickname)
            .cloned() // Return a copy of the state
            .ok_or_else(|| FileFantasticError::NotFound(PathBuf::from(nickname)))
    }
    
    // /// Removes a pocket dimension by nickname
    // /// 
    // /// # Purpose
    // /// Deletes a saved pocket dimension from the collection, freeing up
    // /// the nickname for reuse and cleaning up storage.
    // /// 
    // /// # Arguments
    // /// * `nickname` - The nickname of the pocket dimension to remove
    // /// 
    // /// # Returns
    // /// * `Result<()>` - Success or error if nickname not found
    // /// 
    // /// # Error Conditions
    // /// - Nickname not found in the collection
    // /// 
    // /// # Usage Context
    // /// Used for pocket dimension management:
    // /// - Cleaning up old or unused states
    // /// - Freeing up nickname for reuse
    // /// - Managing memory usage
    // /// 
    // /// # Example
    // /// ```rust
    // /// match state_manager.remove_pocket_dimension("old_workspace") {
    // ///     Ok(_) => println!("Pocket dimension removed"),
    // ///     Err(e) => println!("Failed to remove: {}", e),
    // /// }
    // /// ```
    // pub fn remove_pocket_dimension(&mut self, nickname: &str) -> Result<()> {
    //     self.pocket_dimensions
    //         .remove(nickname)
    //         .map(|_| ()) // Convert Option<SavedNavigationState> to ()
    //         .ok_or_else(|| FileFantasticError::NotFound(PathBuf::from(nickname)))
    // }
    
    /// Adds a file path to the file stack after validation
    /// 
    /// # Purpose
    /// Safely adds a file path to the file collection stack after verifying
    /// that the path exists and actually points to a file (not a directory).
    /// 
    /// # Arguments
    /// * `file_path` - PathBuf pointing to the file to add
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error with validation details
    /// 
    /// # Validation Process
    /// 1. Checks that the path exists on the filesystem
    /// 2. Verifies that the path points to a file (not a directory)
    /// 3. Adds to stack only if both conditions are met
    /// 
    /// # Error Conditions
    /// - Path does not exist (NotFound)
    /// - Path exists but is not a file (InvalidName)
    /// 
    /// # Stack Behavior
    /// Files are added to the end of the stack (LIFO - Last In, First Out).
    /// This allows users to work with the most recently added files first.
    /// 
    /// # Example
    /// ```rust
    /// let file_path = PathBuf::from("/home/user/document.txt");
    /// match state_manager.add_file_to_stack(file_path) {
    ///     Ok(_) => println!("File added to stack"),
    ///     Err(e) => println!("Failed to add file: {}", e),
    /// }
    /// ```
    pub fn add_file_to_stack(&mut self, file_path: PathBuf) -> Result<()> {
        // Validate that the path exists
        if !file_path.exists() {
            return Err(FileFantasticError::NotFound(file_path));
        }
        
        // Validate that the path is actually a file
        if file_path.is_file() {
            self.file_path_stack.push(file_path);
            Ok(())
        } else {
            Err(FileFantasticError::InvalidName(
                "Path is not a file".to_string()
            ))
        }
    }
    
    /// Adds a directory path to the directory stack after validation
    /// 
    /// # Purpose
    /// Safely adds a directory path to the directory collection stack after
    /// verifying that the path exists and actually points to a directory.
    /// 
    /// # Arguments
    /// * `dir_path` - PathBuf pointing to the directory to add
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error with validation details
    /// 
    /// # Validation Process
    /// 1. Checks that the path exists on the filesystem
    /// 2. Verifies that the path points to a directory (not a file)
    /// 3. Adds to stack only if both conditions are met
    /// 
    /// # Error Conditions
    /// - Path does not exist (NotFound)
    /// - Path exists but is not a directory (InvalidName)
    /// 
    /// # Stack Behavior
    /// Directories are added to the end of the stack (LIFO - Last In, First Out).
    /// This allows users to work with the most recently added directories first.
    /// 
    /// # Usage Context
    /// Used for collecting destination directories for operations like:
    /// - Copy/move operations
    /// - Quick navigation bookmarks
    /// - Batch processing targets
    /// 
    /// # Example
    /// ```rust
    /// let dir_path = PathBuf::from("/home/user/projects");
    /// match state_manager.add_directory_to_stack(dir_path) {
    ///     Ok(_) => println!("Directory added to stack"),
    ///     Err(e) => println!("Failed to add directory: {}", e),
    /// }
    /// ```
    pub fn add_directory_to_stack(&mut self, dir_path: PathBuf) -> Result<()> {
        // Validate that the path exists
        if !dir_path.exists() {
            return Err(FileFantasticError::NotFound(dir_path));
        }
        
        // Validate that the path is actually a directory
        if dir_path.is_dir() {
            self.directory_path_stack.push(dir_path);
            Ok(())
        } else {
            Err(FileFantasticError::InvalidName(
                "Path is not a directory".to_string()
            ))
        }
    }
    
    /// Gets and removes the most recent file from the file stack
    /// 
    /// # Purpose
    /// Removes and returns the most recently added file from the file stack,
    /// implementing LIFO (Last In, First Out) behavior.
    /// 
    /// # Returns
    /// * `Option<PathBuf>` - The most recent file path, or None if stack is empty
    /// 
    /// # Stack Behavior
    /// - Removes the last element added to the stack
    /// - Returns None if the stack is empty
    /// - Modifies the stack by removing the returned element
    /// 
    /// # Usage Context
    /// Used when performing operations on collected files:
    /// - Processing files in reverse order of collection
    /// - Undoing file additions
    /// - Batch operations where order matters
    /// 
    /// # Example
    /// ```rust
    /// match state_manager.pop_file_from_stack() {
    ///     Some(file_path) => println!("Processing: {}", file_path.display()),
    ///     None => println!("No files in stack"),
    /// }
    /// ```
    pub fn pop_file_from_stack(&mut self) -> Option<PathBuf> {
        self.file_path_stack.pop()
    }
    
    // /// Gets and removes the most recent directory from the directory stack
    // /// 
    // /// # Purpose
    // /// Removes and returns the most recently added directory from the directory stack,
    // /// implementing LIFO (Last In, First Out) behavior.
    // /// 
    // /// # Returns
    // /// * `Option<PathBuf>` - The most recent directory path, or None if stack is empty
    // /// 
    // /// # Stack Behavior
    // /// - Removes the last element added to the stack
    // /// - Returns None if the stack is empty
    // /// - Modifies the stack by removing the returned element
    // /// 
    // /// # Usage Context
    // /// Used when performing operations with collected directories:
    // /// - Selecting destination directories
    // /// - Quick navigation to recently accessed locations
    // /// - Batch operations targeting multiple directories
    // /// 
    // /// # Example
    // /// ```rust
    // /// match state_manager.pop_directory_from_stack() {
    // ///     Some(dir_path) => println!("Using directory: {}", dir_path.display()),
    // ///     None => println!("No directories in stack"),
    // /// }
    // /// ```
    // pub fn pop_directory_from_stack(&mut self) -> Option<PathBuf> {
    //     self.directory_path_stack.pop()
    // }
    
    /// Clears all stacks and pocket dimensions
    /// 
    /// # Purpose
    /// Resets the NavigationStateManager to its initial empty state by
    /// clearing all collections. This is useful for cleanup operations
    /// or when the user wants to start fresh.
    /// 
    /// # Operations Performed
    /// 1. Clears the file path stack
    /// 2. Clears the directory path stack
    /// 3. Clears all saved pocket dimensions
    /// 4. Resets auto-nickname counter
    /// 
    /// # Usage Context
    /// - User-requested cleanup operations
    /// - Error recovery scenarios
    /// - Starting a new work session
    /// - Memory management in long-running sessions
    /// 
    /// # Warning
    /// This operation is irreversible and will lose all collected paths
    /// and saved navigation states. Use with caution.
    /// 
    /// # Example
    /// ```rust
    /// // Clear everything after user confirmation
    /// if user_confirms_clear() {
    ///     state_manager.clear_all();
    ///     println!("All stacks and pocket dimensions cleared");
    /// }
    /// ```
    pub fn clear_all(&mut self) {
        self.file_path_stack.clear();
        self.directory_path_stack.clear();
        self.pocket_dimensions.clear();
        self.auto_nickname_counter = 1; // Reset counter
    }
    
    /// Gets a summary of current stack and pocket dimension status
    /// 
    /// # Purpose
    /// Provides a concise overview of the current state of all collections
    /// managed by the NavigationStateManager. This is useful for status
    /// displays and user interfaces.
    /// 
    /// # Returns
    /// * `String` - Formatted summary showing counts of each collection
    /// 
    /// # Summary Format
    /// "Files: X | Directories: Y | Pocket Dimensions: Z"
    /// Where X, Y, Z are the counts of items in each collection.
    /// 
    /// # Usage Context
    /// - Status bar displays
    /// - User interface headers
    /// - Debug information
    /// - Help text and prompts
    /// 
    /// # Example Output
    /// ```text
    /// "Files: 3 | Directories: 2 | Pocket Dimensions: 5"
    /// "Files: 0 | Directories: 1 | Pocket Dimensions: 0"
    /// ```
    /// 
    /// # Example Usage
    /// ```rust
    /// let status = state_manager.get_stack_summary();
    /// println!("Current status: {}", status);
    /// ```
    pub fn get_stack_summary(&self) -> String {
        format!(
            "Files: {} | Directories: {} | Pocket Dimensions: {}",
            self.file_path_stack.len(),
            self.directory_path_stack.len(),
            self.pocket_dimensions.len()
        )
    }

    /// Interactive interface to archive a user-selected item
    /// 
    /// # Purpose
    /// Provides an interactive interface for creating timestamped archives of
    /// files or directories. Shows the current directory listing and prompts
    /// user to select an item by number, then confirms archive operation.
    /// 
    /// # Arguments
    /// * `nav_state` - Current navigation state with lookup table for numbered selection
    /// * `current_directory_entries` - Current directory entries to display for selection
    /// * `current_directory_path` - Current directory path for archive context
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error with context
    /// 
    /// # Archive Behavior
    /// - **Files**: Creates timestamped copy in same directory using existing archive handling
    /// - **Directories**: Creates timestamped zip archive in same directory as the source
    /// 
    /// # User Interface Flow
    /// 1. Display current directory contents with numbered items
    /// 2. Prompt user to select item by number (same as normal navigation)
    /// 3. Show selected item and ask for archive confirmation
    /// 4. Perform archive operation based on item type
    /// 5. Display results and status
    /// 
    /// # Integration with Existing System
    /// - Uses the same display_directory_contents function for consistency
    /// - Uses the same nav_state.lookup_item system for numbered selection
    /// - Maintains the same numbered selection interface as main browser
    /// - Only adds the archive-specific workflow and confirmation
    /// 
    /// # Example Interaction
    /// ```text
    /// Current Directory: /home/user/documents
    /// 
    /// Num  Name                    Size     Modified
    /// ------------------------------------------------
    ///  1)  folder1/               -        14:30
    ///  2)  document.txt           1.2 KB   15:45
    ///  3)  image.png              500 KB   16:20
    /// 
    /// === Archive Selection ===
    /// Select item to archive (creates timestamped copy/zip)
    /// Enter item number (or 'c' to cancel): 2
    /// 
    /// Archive 'document.txt' with timestamp? (Y/n): y
    /// ✓ File archived successfully!
    /// Archive location: /home/user/documents/archive/document_2025_01_15_14_30_45.txt
    /// ```
    pub fn interactive_archive_selection(
        &mut self,
        nav_state: &NavigationState,
        current_directory_entries: &[FileSystemEntry],
        current_directory_path: &PathBuf,
    ) -> Result<()> {
        
        // Always display the directory contents first for user selection
        display_directory_contents(
            current_directory_entries,
            current_directory_path,
            None, // No pagination info needed for this context
            nav_state.current_filter,
        ).map_err(|e| FileFantasticError::Io(e))?;

        println!("\n=== Archive Selection ===");
        println!("Select item to archive (creates timestamped copy/zip)");
        print!("Enter item number (or 'c' to cancel): ");
        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| FileFantasticError::Io(e))?;
        let input = input.trim();
        
        // Handle cancellation
        if input.eq_ignore_ascii_case("c") {
            println!("Cancelled.");
            return Ok(());
        }
        
        // Try to parse as number and validate using existing lookup system
        if let Ok(number) = input.parse::<usize>() {
            if let Some(item_info) = nav_state.lookup_item(number) {
                
                // Get the item name for display
                let item_name = item_info.item_path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                
                // Ask for confirmation based on item type
                let confirmation_message = if item_info.item_type == FileSystemItemType::Directory {
                    format!("Archive directory '{}' as zip with timestamp? (Y/n): ", item_name)
                } else {
                    format!("Archive file '{}' with timestamp? (Y/n): ", item_name)
                };
                
                print!("\n{}", confirmation_message);
                io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
                
                let mut response = String::new();
                io::stdin().read_line(&mut response).map_err(|e| FileFantasticError::Io(e))?;
                
                // Default to 'yes' if user just presses enter
                if response.trim().is_empty() || response.trim().eq_ignore_ascii_case("y") {
                    
                    // Perform archive operation based on item type
                    if item_info.item_type == FileSystemItemType::Directory {
                        // Ask for optional custom name addition
                        print!("Add custom name to archive? (optional, or Enter to skip): ");
                        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
                        
                        let mut custom_name = String::new();
                        io::stdin().read_line(&mut custom_name).map_err(|e| FileFantasticError::Io(e))?;
                        let custom_name = custom_name.trim();
                        
                        // Prepare custom name parameter
                        let custom_name_option = if custom_name.is_empty() {
                            None
                        } else {
                            Some(custom_name)
                        };
                        
                        // Archive directory as zip in the archive subfolder for consistency
                        match ensure_archive_directory_exists(current_directory_path) {
                            Ok(archive_directory_path) => {
                                match create_directory_zip_archive(
                                    &item_info.item_path, 
                                    &archive_directory_path,
                                    custom_name_option,
                                ) {
                                    Ok(zip_path) => {
                                        println!("✓ Directory archived successfully!");
                                        println!("Archive location: {}", zip_path.display());
                                    },
                                    Err(e) => {
                                        eprintln!("✗ Directory archive creation failed: {}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("✗ Failed to create archive directory: {}", e);
                            }
                        }
                    } else {
                        // Archive file with timestamp using existing system (already uses archive folder)
                        match copy_file_with_archive_handling(&item_info.item_path, current_directory_path) {
                            Ok(archived_path) => {
                                println!("✓ File archived successfully!");
                                println!("Archive location: {}", archived_path.display());
                            },
                            Err(e) => {
                                eprintln!("✗ File archive creation failed: {}", e);
                            }
                        }
                    }
                } else {
                    println!("Archive cancelled.");
                }
                
            } else {
                println!("Error: Invalid item number {}. Please try again.", number);
            }
        } else {
            println!("Error: Please enter a valid number or 'c' to cancel.");
        }
        
        println!("Press Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
        
        Ok(())
    }
        
    /// Interactive interface to add a file to the file stack
    /// 
    /// # Purpose
    /// Provides an interactive interface for adding files to the file stack,
    /// supporting both pre-selected files (with Y/n confirmation) and 
    /// numbered selection from the current directory listing. This maintains 
    /// consistency with the existing user interface while providing flexible
    /// workflows for different user scenarios.
    /// 
    /// # Arguments
    /// * `nav_state` - Current navigation state with lookup table for numbered selection
    /// * `selected_file` - Optional pre-selected file path to offer as default confirmation
    /// * `current_directory_entries` - Current directory entries to display for selection
    /// * `current_directory_path` - Current directory path for display context
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error with context
    /// 
    /// # User Interface Flow
    /// 1. Clear any previous input to prevent buffer reuse bugs
    /// 2. If pre-selected file exists, offer Y/n confirmation
    /// 3. If declined or no pre-selection, show directory contents
    /// 4. Display "Add File to Stack" prompt with directory listing
    /// 5. User selects file by number (same as normal file browsing)
    /// 6. Validate selection is a file (not directory)
    /// 7. Add file to stack and show confirmation
    /// 
    /// # Integration with Existing System
    /// - Uses the same display_directory_contents function for consistency
    /// - Uses the same nav_state.lookup_item system for numbered selection
    /// - Maintains the same numbered selection interface as main browser
    /// - Only adds the stack-specific messaging and workflow
    /// - Preserves all existing user interface patterns
    /// 
    /// # Input Buffer Management
    /// - Clears stdin buffer to prevent reuse of previous menu selections
    /// - Forces fresh user input for file selection
    /// - Handles both confirmation and selection input patterns
    /// 
    /// # Example Interaction
    /// ```text
    /// === Add File to Stack ===
    /// Add 'document.txt' to file stack? (Y/n): n
    /// 
    /// Current Directory: /home/user/documents
    /// 
    /// Num  Name                    Size     Modified
    /// ------------------------------------------------
    ///  1)  folder1/               -        14:30
    ///  2)  document.txt           1.2 KB   15:45
    ///  3)  image.png              500 KB   16:20
    /// 
    /// === Add File to Stack ===
    /// Type file number & press Enter to add to file-stack.
    /// Enter file number (or 'c' to cancel): 2
    /// Added 'document.txt' to file stack. Total files: 1
    /// ```
    /// 
    /// # Error Handling
    /// - Validates numbered selections against lookup table
    /// - Ensures selected items are files, not directories
    /// - Provides clear error messages for invalid selections
    /// - Handles cancellation gracefully
    /// - Manages IO errors during user interaction
    /// 
    /// # Stack Integration
    /// - Validates file existence before adding to stack
    /// - Provides confirmation with updated stack count
    /// - Maintains LIFO stack behavior for consistent operations
    /// - Integrates with broader Get-Send-Mode workflow system
    pub fn interactive_add_file_to_stack(
        &mut self,
        nav_state: &NavigationState,
        selected_file: Option<&PathBuf>,
        current_directory_entries: &[FileSystemEntry],
        current_directory_path: &PathBuf,
    ) -> Result<()> {

        // If there's a pre-selected file, offer it as the default
        if let Some(file_path) = selected_file {
            println!("\n=== Add File to Stack ===");
            print!("Add '{}' to file stack? (Y/n): ", file_path.file_name().unwrap_or_default().to_string_lossy());
            io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
            
            let mut response = String::new();
            io::stdin().read_line(&mut response).map_err(|e| FileFantasticError::Io(e))?;
            
            // Default to 'yes' if user just presses enter
            if response.trim().is_empty() || response.trim().eq_ignore_ascii_case("y") {
                self.add_file_to_stack(file_path.clone())?;
                println!("Added '{}' to file stack. Total files: {}", 
                        file_path.file_name().unwrap_or_default().to_string_lossy(),
                        self.file_path_stack.len());
                return Ok(());
            }
            // If user declined ('n'), continue to numbered selection below
        }

        // Display directory contents for file selection
        display_directory_contents(
            current_directory_entries,
            current_directory_path,
            None, // No pagination info needed for this context
            nav_state.current_filter,
        ).map_err(|e| FileFantasticError::Io(e))?;

        println!("\n=== Add File to Stack ===");
        println!("Type file number & press Enter to add to file-stack. (or 'c' to cancel)");
        // print!("Enter file number (or 'c' to cancel): ");
        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| FileFantasticError::Io(e))?;
        let input = input.trim();
        
        // Handle cancellation
        if input.eq_ignore_ascii_case("c") {
            println!("Cancelled.");
            return Ok(());
        }
        
        // Try to parse as number and validate using existing lookup system
        if let Ok(number) = input.parse::<usize>() {
            if let Some(item_info) = nav_state.lookup_item(number) {
                // Ensure it's a file, not a directory
                if item_info.item_type == FileSystemItemType::File {
                    self.add_file_to_stack(item_info.item_path.clone())?;
                    println!("Added '{}' to file stack. Total files: {}", 
                            item_info.item_path.file_name().unwrap_or_default().to_string_lossy(),
                            self.file_path_stack.len());
                } else {
                    println!("Error: Item {} is a directory. Please select a file.", number);
                }
            } else {
                println!("Error: Invalid file number {}. Please try again.", number);
            }
        } else {
            println!("Error: Please enter a valid number or 'c' to cancel.");
        }
        
        Ok(())
    }
    
    /// Q&A interface to select and return file from stack
    /// 
    /// # Purpose
    /// Provides an interactive interface for selecting a file from the file stack,
    /// displaying all available files and allowing selection by number.
    /// This uses the same numbered selection paradigm as the rest of the application.
    /// 
    /// # Returns
    /// * `Result<Option<PathBuf>>` - Selected file path, None if canceled, or error
    /// 
    /// # User Interface Flow
    /// 1. Check if file stack is empty
    /// 2. Display all files in the stack with numbers (most recent first)
    /// 3. Allow user to select by number or default to most recent
    /// 4. Remove and return the selected file
    /// 5. Display confirmation of selection
    /// 
    /// # Selection Options
    /// - Enter number: Select specific file by index
    /// - Enter (empty): Select most recent file (top of stack)
    /// - 'c': Cancel operation
    /// - Invalid number: Display error and return None
    /// 
    /// # Display Format
    /// Files are displayed in reverse order (most recent first) with 1-based indexing
    /// to match user expectations and maintain consistency with main interface.
    /// 
    /// # Example Interaction
    /// ```text
    /// === File Stack ===
    /// 1. document.txt
    /// 2. image.png
    /// 3. script.sh
    /// Select file number (Enter for most recent, 'c' to cancel): 2
    /// Retrieved: image.png
    /// ```
    pub fn interactive_get_file_from_stack(&mut self) -> Result<Option<PathBuf>> {
        // Check if stack is empty
        if self.file_path_stack.is_empty() {
            println!("File stack is empty.");
            return Ok(None);
        }
        
        println!("\n=== File Stack ===");
        // Display files in reverse order (most recent first) for user-friendly numbering
        for (i, file) in self.file_path_stack.iter().enumerate().rev() {
            println!("{}. {}", 
                     self.file_path_stack.len() - i, 
                     file.file_name().unwrap_or_default().to_string_lossy());
        }
        
        print!("Select file number (Enter for most recent, 'c' to cancel): ");
        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| FileFantasticError::Io(e))?;
        let input = input.trim();
        
        // Handle cancellation
        if input.eq_ignore_ascii_case("c") {
            println!("Cancelled.");
            return Ok(None);
        }
        
        // Default to most recent (pop from end) if no input
        if input.is_empty() {
            if let Some(file) = self.pop_file_from_stack() {
                println!("Retrieved: {}", file.file_name().unwrap_or_default().to_string_lossy());
                return Ok(Some(file));
            }
        }
        
        // Try to parse as index and validate
        if let Ok(index) = input.parse::<usize>() {
            if index > 0 && index <= self.file_path_stack.len() {
                // Convert to actual vector index (1-based display to 0-based storage)
                let actual_index = self.file_path_stack.len() - index;
                let file = self.file_path_stack.remove(actual_index);
                println!("Retrieved: {}", file.file_name().unwrap_or_default().to_string_lossy());
                return Ok(Some(file));
            } else {
                println!("Error: Invalid file number {}. Valid range: 1-{}", index, self.file_path_stack.len());
            }
        } else {
            println!("Error: Please enter a valid number, press Enter for most recent, or 'c' to cancel.");
        }
        
        Ok(None)
    }
    
    /// Q&A interface to save current directory to directory stack
    /// 
    /// # Purpose
    /// Provides an interactive interface for adding the current directory
    /// to the directory stack, with user confirmation.
    /// 
    /// # Arguments
    /// * `current_directory` - The current directory to potentially add
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error with context
    /// 
    /// # User Interface Flow
    /// 1. Display the current directory path
    /// 2. Ask for user confirmation to add it to the stack
    /// 3. Add to stack if user confirms (default is yes)
    /// 4. Display confirmation with current stack size
    /// 
    /// # Confirmation Logic
    /// - Empty input or 'y'/'Y': Add to stack
    /// - Any other input: Do not add to stack
    /// 
    /// # Example Interaction
    /// ```text
    /// === Add Directory to Stack ===
    /// Current directory: /home/user/projects
    /// Add current directory to stack? (Y/n): 
    /// Added to directory stack. Total directories: 2
    /// ```
    pub fn interactive_save_directory_to_stack(&mut self, current_directory: &PathBuf) -> Result<()> {
        println!("\n=== Add Directory to Stack ===");
        println!("Current directory: {}", current_directory.display());
        
        print!("Add current directory to stack? (Y/n): ");
        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
        
        let mut response = String::new();
        io::stdin().read_line(&mut response).map_err(|e| FileFantasticError::Io(e))?;
        
        // Default to 'yes' if user just presses enter
        if response.trim().is_empty() || response.trim().eq_ignore_ascii_case("y") {
            self.add_directory_to_stack(current_directory.clone())?;
            println!("Added to directory stack. Total directories: {}", self.directory_path_stack.len());
        } else {
            println!("Cancelled.");
        }
        
        Ok(())
    }
    
    /// Q&A interface to select pocket dimension to jump to
    /// 
    /// # Purpose
    /// Provides an interactive interface for selecting a saved pocket dimension
    /// from the available collection, displaying them with descriptions.
    /// Uses numbered selection consistent with the rest of the application.
    /// 
    /// # Returns
    /// * `Result<Option<String>>` - Selected nickname, None if canceled, or error
    /// 
    /// # User Interface Flow
    /// 1. Check if any pocket dimensions exist
    /// 2. Display all available dimensions with numbers and descriptions
    /// 3. Allow user to select by number
    /// 4. Validate selection and return the nickname
    /// 
    /// # Display Format
    /// Each pocket dimension is shown with:
    /// - Sequential number for selection
    /// - Nickname for identification
    /// - Description showing key characteristics
    /// 
    /// # Selection Validation
    /// - Must be a valid number within the range
    /// - 'c' cancels the operation
    /// - Returns None for invalid selections or cancellation
    /// 
    /// # Example Interaction
    /// ```text
    /// === Pocket Dimensions ===
    /// 1. workspace - projects [files] name↓
    /// 2. logs_1234 - logs size↑
    /// 3. temp_5678 - temp
    /// Select pocket dimension number ('c' to cancel): 1
    /// Selected: workspace
    /// ```
    pub fn interactive_select_pocket_dimension(&self) -> Result<Option<String>> {
        let dimensions = self.list_pocket_dimensions();
        
        // Check if any pocket dimensions exist
        if dimensions.is_empty() {
            println!("No saved pocket dimensions.");
            return Ok(None);
        }
        
        println!("\n=== Pocket Dimensions ===");
        // Display all dimensions with numbers and descriptions
        for (i, (nickname, state)) in dimensions.iter().enumerate() {
            println!("{}. {} - {}", 
                     i + 1, 
                     nickname, 
                     state.description);
        }
        
        print!("Select pocket dimension number ('c' to cancel): ");
        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| FileFantasticError::Io(e))?;
        let input = input.trim();
        
        // Handle cancellation
        if input.eq_ignore_ascii_case("c") {
            println!("Cancelled.");
            return Ok(None);
        }
        
        // Parse and validate selection
        if let Ok(index) = input.parse::<usize>() {
            if index > 0 && index <= dimensions.len() {
                let (nickname, _) = dimensions[index - 1];
                return Ok(Some(nickname.clone()));
            } else {
                println!("Error: Invalid pocket dimension number {}. Valid range: 1-{}", index, dimensions.len());
            }
        } else {
            println!("Error: Please enter a valid number or 'c' to cancel.");
        }
        
        Ok(None)
    }
    
    
    
    /// Interactive Get-Send-Mode landing page and command processor
    /// 
    /// # Purpose
    /// Provides the main menu interface for Get-Send-Mode, displaying current
    /// status and available actions, then processing user selection.
    /// 
    /// # Returns
    /// * `Result<GetSendModeAction>` - The action selected by user, or error
    /// 
    /// # Menu Options
    /// 1. Add file to stack (using numbered selection)
    /// 2. Get: Save file here, from file-stack (using numbered selection)
    /// 3. Add current directory to stack
    /// 4. Save current location as pocket dimension
    /// 5. Go to pocket dimension (using numbered selection)
    /// 6. View stacks and pocket dimensions
    /// 7. Clear all stacks
    /// 8. Return to file browser
    /// 
    /// # Status Display
    /// Shows current counts of files, directories, and pocket dimensions
    /// to give user context about their current collections.
    /// 
    /// # Input Validation
    /// - Accepts numbers 1-8 for menu options
    /// - Empty input defaults to return to browser
    /// - Invalid input defaults to return to browser
    /// 
    /// # Example Interaction
    /// ```text
    /// === Get-Send-Mode ===
    /// Current status: Files: 2 | Directories: 1 | Pocket Dimensions: 3
    /// 
    /// 1. Add file to stack (select by number)
    /// 2. Get: Save file here, from file-stack
    /// 3. Add current directory to stack
    /// 4. Save current location as pocket dimension
    /// 5. Go to pocket dimension
    /// 6. View stacks and pocket dimensions
    /// 7. Clear all stacks
    /// 8. Return to file browser
    /// 
    /// Select action (1-8): 1
    /// ```
    pub fn interactive_get_send_mode(&mut self) -> Result<GetSendModeAction> {
        println!("\n=== Get-Send-Mode ===");
        println!("Current status: {}", self.get_stack_summary());
        println!();
        println!("1. Add file to stack (select by number)");
        println!("2. Get: Save file here, from file-stack");
        println!("3. Add current directory to stack");
        println!("4. Save current location as pocket dimension");
        println!("5. Go to pocket dimension");
        println!("6. View stacks and pocket dimensions");
        println!("7. Archive Selection (file or dir zipped + timestamp)");
        println!("8. Clear all stacks");
        println!("9. Return to file browser ( or empty Enter or (b)ack )");
        println!();
        print!("Select action (1-8): ");
        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| FileFantasticError::Io(e))?;
        
        // Process user selection and return appropriate action
        match input.trim() {
            "1" => Ok(GetSendModeAction::AddFileToStack),
            "2" => Ok(GetSendModeAction::GetFileFromStack),
            "3" => Ok(GetSendModeAction::AddDirectoryToStack),
            "4" => Ok(GetSendModeAction::SavePocketDimension),
            "5" => Ok(GetSendModeAction::GoToPocketDimension),
            "6" => Ok(GetSendModeAction::ViewStacks),
            "7" => Ok(GetSendModeAction::ArchiveSelection),
            "8" => Ok(GetSendModeAction::ClearAll),
            "9" | "" | "b" => Ok(GetSendModeAction::ReturnToBrowser), // Default to return
            _ => {
                println!("Invalid selection.");
                Ok(GetSendModeAction::ReturnToBrowser) // Default for invalid input
            }
        }
    }
}

/// Actions available in Get-Send-Mode interface
/// 
/// # Purpose
/// Defines all possible actions that can be selected in the Get-Send-Mode
/// interface, providing a clear enumeration of available operations.
/// 
/// # Design Philosophy
/// Each variant represents a distinct user operation with specific behavior.
/// The enum serves as a clean interface between the user interface layer
/// and the action processing logic.
/// 
/// # Variants
/// - `AddFileToStack` - Initiate file addition workflow
/// - `GetFileFromStack` - Initiate file retrieval workflow
/// - `AddDirectoryToStack` - Initiate directory addition workflow
/// - `SavePocketDimension` - Initiate pocket dimension saving workflow
/// - `GoToPocketDimension` - Initiate pocket dimension selection and restoration
/// - `ViewStacks` - Display current status and contents of all collections
/// - `ClearAll` - Initiate cleanup workflow with confirmation
/// - `ReturnToBrowser` - Exit Get-Send-Mode and return to normal file browsing
/// 
/// # Usage Context
/// Returned by interactive_get_send_mode() and processed by the main
/// application loop to determine what operation to perform.
/// 
/// # Example
/// ```rust
/// match action {
///     GetSendModeAction::AddFileToStack => {
///         // Handle file addition workflow
///     },
///     GetSendModeAction::ReturnToBrowser => {
///         // Exit Get-Send-Mode
///         break;
///     },
///     // ... other actions
/// }
/// ```
#[derive(Debug)]
pub enum GetSendModeAction {
    /// Add a file to the file path stack
    /// Triggers the interactive file selection and addition workflow
    AddFileToStack,
    
    /// Retrieve a file from the file path stack
    /// Triggers the interactive file selection and retrieval workflow
    GetFileFromStack,
    
    /// Add current directory to the directory path stack
    /// Triggers the directory addition workflow with confirmation
    AddDirectoryToStack,
    
    /// Save current navigation state as a pocket dimension
    /// Triggers the pocket dimension creation workflow with nickname selection
    SavePocketDimension,
    
    /// Navigate to a saved pocket dimension
    /// Triggers the pocket dimension selection and restoration workflow
    GoToPocketDimension,
    
    /// Display current status and contents of all stacks and pocket dimensions
    /// Shows detailed view of all collected items and saved states
    ViewStacks,
    
    /// Archive the currently selected item (file copy or directory zip with timestamp)
    ArchiveSelection,
    
    /// Clear all stacks and pocket dimensions
    /// Triggers the cleanup workflow with user confirmation
    ClearAll,
    
    /// Exit Get-Send-Mode and return to normal file browser
    /// Default action for cancellation or completion of workflows
    ReturnToBrowser,
}

/*
End Of Pocket-Dimensions 
*/

/// Generates a timestamp string for archive file naming
/// 
/// # Purpose
/// Creates a standardized timestamp string in the format YYYY_MM_DD_HH_MM_SS
/// for use in archived file names when avoiding overwrites.
/// 
/// # Returns
/// * `String` - Formatted timestamp string (e.g., "2025_01_15_14_30_45")
/// 
/// # Format Details
/// - Uses local system time
/// - Format: YYYY_MM_DD_HH_MM_SS with underscores as separators
/// - 24-hour time format
/// - Zero-padded numbers for consistent length
/// 
/// # Usage Context
/// Used when copying files to avoid overwriting existing files by
/// creating unique archived versions with timestamps.
/// 
/// # Example
/// ```rust
/// let timestamp = generate_archive_timestamp();
/// // Returns something like: "2025_01_15_14_30_45"
/// 
/// // Used in archive filename: "cats_2025_01_15_14_30_45.toml"
/// ```
fn generate_archive_timestamp() -> String {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0));
    let seconds = duration_since_epoch.as_secs();
    
    let (year, month, day, hour, minute) = seconds_to_components(seconds);
    let second = (seconds % 60) as u32;
    
    format!("{:04}_{:02}_{:02}_{:02}_{:02}_{:02}", 
            year, month, day, hour, minute, second)
}

/// Creates archive directory if it doesn't exist
/// 
/// # Purpose
/// Ensures that an "archive" subdirectory exists in the specified parent directory,
/// creating it if necessary. This directory is used to store copies of files
/// when avoiding overwrites.
/// 
/// # Arguments
/// * `parent_directory` - The directory where the archive folder should exist
/// 
/// # Returns
/// * `Result<PathBuf>` - Absolute path to the archive directory, or error
/// 
/// # Error Conditions
/// - IO errors when creating the directory
/// - Permission denied when writing to parent directory
/// - Invalid parent directory path
/// 
/// # Archive Directory Structure
/// ```text
/// parent_directory/
/// ├── existing_files...
/// └── archive/          <- Created by this function
///     ├── file1_timestamp.ext
///     └── file2_timestamp.ext
/// ```
/// 
/// # Example
/// ```rust
/// let current_dir = PathBuf::from("/home/user/documents");
/// match ensure_archive_directory_exists(&current_dir) {
///     Ok(archive_path) => {
///         // archive_path is "/home/user/documents/archive"
///         println!("Archive directory ready: {}", archive_path.display());
///     },
///     Err(e) => eprintln!("Failed to create archive directory: {}", e),
/// }
/// ```
fn ensure_archive_directory_exists(parent_directory: &PathBuf) -> Result<PathBuf> {
    let archive_directory_path = parent_directory.join("archive");
    
    // Check if archive directory already exists
    if !archive_directory_path.exists() {
        // Create the archive directory
        fs::create_dir(&archive_directory_path).map_err(|e| {
            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    FileFantasticError::PermissionDenied(archive_directory_path.clone())
                },
                _ => FileFantasticError::Io(e)
            }
        })?;
        
        println!("Created archive directory: {}", archive_directory_path.display());
    }
    
    // Verify it's actually a directory
    if !archive_directory_path.is_dir() {
        return Err(FileFantasticError::InvalidName(
            format!("Archive path exists but is not a directory: {}", 
                   archive_directory_path.display())
        ));
    }
    
    Ok(archive_directory_path)
}

/// Generates an archive filename with timestamp insertion
/// 
/// # Purpose
/// Creates a new filename for archived files by inserting a timestamp
/// before the file extension, preserving the original name structure
/// while ensuring uniqueness.
/// 
/// # Arguments
/// * `original_filename` - The original name of the file
/// * `timestamp` - The timestamp string to insert
/// 
/// # Returns
/// * `String` - New filename with timestamp inserted before extension
/// 
/// # Filename Transformation Rules
/// - Files with extensions: "name.ext" → "name_timestamp.ext"
/// - Files without extensions: "name" → "name_timestamp"
/// - Multiple extensions: "file.tar.gz" → "file.tar_timestamp.gz" (only last extension)
/// - Hidden files: ".gitignore" → ".gitignore_timestamp"
/// 
/// # Examples
/// ```rust
/// let timestamp = "2025_01_15_14_30_45";
/// 
/// assert_eq!(
///     generate_archive_filename("cats.toml", timestamp),
///     "cats_2025_01_15_14_30_45.toml"
/// );
/// 
/// assert_eq!(
///     generate_archive_filename("README", timestamp),
///     "README_2025_01_15_14_30_45"
/// );
/// 
/// assert_eq!(
///     generate_archive_filename("data.tar.gz", timestamp),
///     "data.tar_2025_01_15_14_30_45.gz"
/// );
/// ```
fn generate_archive_filename(original_filename: &str, timestamp: &str) -> String {
    // Find the last dot to separate name from extension
    if let Some(dot_position) = original_filename.rfind('.') {
        // Split into name and extension
        let (name_part, extension_part) = original_filename.split_at(dot_position);
        format!("{}_{}{}", name_part, timestamp, extension_part)
    } else {
        // No extension found, just append timestamp
        format!("{}_{}", original_filename, timestamp)
    }
}

/// Copies a file to the destination directory with automatic archive handling
/// 
/// # Purpose
/// Safely copies a file from the source path to the destination directory,
/// automatically handling filename conflicts by creating archived versions
/// with timestamps when necessary.
/// 
/// # Arguments
/// * `source_file_path` - Absolute path to the source file to copy
/// * `destination_directory` - Absolute path to the destination directory
/// 
/// # Returns
/// * `Result<PathBuf>` - Absolute path to the final copied file location, or error
/// 
/// # Conflict Resolution Strategy
/// 1. If destination file doesn't exist: copy directly to destination
/// 2. If destination file exists:
///    a. Create "archive" subdirectory if it doesn't exist
///    b. Generate timestamped filename for the copy
///    c. Copy to archive directory with timestamped name
/// 
/// # Error Conditions
/// - Source file doesn't exist or isn't readable
/// - Destination directory doesn't exist or isn't writable
/// - IO errors during file copy operation
/// - Permission denied for file or directory access
/// - Insufficient disk space for copy operation
/// 
/// # File Preservation Guarantee
/// - Never overwrites existing files
/// - Preserves original file permissions and timestamps when possible
/// - Creates complete directory structure as needed
/// 
/// # Example Workflow
/// ```text
/// Source: /path/to/source/document.txt
/// Destination: /current/working/directory/
/// 
/// Scenario 1 - No conflict:
/// Result: /current/working/directory/document.txt
/// 
/// Scenario 2 - File exists:
/// Creates: /current/working/directory/archive/
/// Result: /current/working/directory/archive/document_2025_01_15_14_30_45.txt
/// ```
/// 
/// # Usage
/// ```rust
/// let source = PathBuf::from("/home/user/important.txt");
/// let destination = PathBuf::from("/home/user/projects");
/// 
/// match copy_file_with_archive_handling(&source, &destination) {
///     Ok(final_path) => {
///         println!("File copied to: {}", final_path.display());
///     },
///     Err(e) => {
///         eprintln!("Copy failed: {}", e);
///     }
/// }
/// ```
fn copy_file_with_archive_handling(
    source_file_path: &PathBuf,
    destination_directory: &PathBuf,
) -> Result<PathBuf> {
    // Validate source file exists and is a file
    if !source_file_path.exists() {
        return Err(FileFantasticError::NotFound(source_file_path.clone()));
    }
    
    if !source_file_path.is_file() {
        return Err(FileFantasticError::InvalidName(
            format!("Source is not a file: {}", source_file_path.display())
        ));
    }
    
    // Validate destination directory exists and is a directory
    if !destination_directory.exists() {
        return Err(FileFantasticError::NotFound(destination_directory.clone()));
    }
    
    if !destination_directory.is_dir() {
        return Err(FileFantasticError::InvalidName(
            format!("Destination is not a directory: {}", destination_directory.display())
        ));
    }
    
    // Extract source filename
    let source_filename = source_file_path.file_name()
        .ok_or_else(|| FileFantasticError::InvalidName(
            format!("Cannot determine filename from: {}", source_file_path.display())
        ))?
        .to_string_lossy()
        .to_string();
    
    // Determine destination path
    let primary_destination_path = destination_directory.join(&source_filename);
    
    let final_destination_path = if primary_destination_path.exists() {
        // File exists, use archive strategy
        println!("File '{}' already exists in destination.", source_filename);
        println!("Creating archived copy to avoid overwrite...");
        
        // Ensure archive directory exists
        let archive_directory_path = ensure_archive_directory_exists(destination_directory)?;
        
        // Generate timestamped filename
        let timestamp = generate_archive_timestamp();
        let archive_filename = generate_archive_filename(&source_filename, &timestamp);
        let archive_destination_path = archive_directory_path.join(&archive_filename);
        
        // Copy to archive location
        fs::copy(source_file_path, &archive_destination_path).map_err(|e| {
            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    FileFantasticError::PermissionDenied(archive_destination_path.clone())
                },
                _ => FileFantasticError::Io(e)
            }
        })?;
        
        println!("Archived copy created: {}", archive_destination_path.display());
        archive_destination_path
    } else {
        // No conflict, copy directly to destination
        fs::copy(source_file_path, &primary_destination_path).map_err(|e| {
            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    FileFantasticError::PermissionDenied(primary_destination_path.clone())
                },
                _ => FileFantasticError::Io(e)
            }
        })?;
        
        println!("File copied to: {}", primary_destination_path.display());
        primary_destination_path
    };
    
    Ok(final_destination_path)
}


/// Handles paginated viewing of directory contents
///
/// # Purpose
/// This struct manages the pagination of directory listings, allowing users to
/// navigate through large directories page by page rather than seeing all entries
/// at once, which improves usability in terminals with limited display space.
///
/// # Pagination Behavior
/// - Divides directory entries into pages of fixed size
/// - Tracks current page and allows navigation between pages
/// - Maps display indices to actual entry indices
/// - Handles boundary cases (first/last page)
///
/// # Fields
/// * `entries` - Reference to the full list of directory entries
/// * `current_page` - Zero-based index of the current visible page
/// * `items_per_page` - Number of items to display per page
///
/// # Usage
/// ```rust
/// // Create a paginated view of directory entries
/// let mut dir_view = DirectoryView::new(&directory_entries);
/// 
/// // Display current page
/// let current_page_entries = dir_view.current_page_entries();
/// display_directory_contents(current_page_entries, &current_path)?;
/// 
/// // Navigate to next page if user presses 's'
/// if user_input == "s" {
///     dir_view.next_page();
/// }
/// 
/// // Navigate to previous page if user presses 'w'
/// if user_input == "w" {
///     dir_view.prev_page();
/// }
///
/// // Convert a display index to actual index in the full list
/// if let Some(actual_index) = dir_view.get_actual_index(selected_number) {
///     // Use the entry at the actual index
///     let selected_entry = &directory_entries[actual_index];
/// }
/// ```
pub struct DirectoryView<'a> {
    entries: &'a [FileSystemEntry],
    current_page: usize,
    items_per_page: usize,
}

impl<'a> DirectoryView<'a> {
    /// Creates new directory view with pagination
    fn new(entries: &'a [FileSystemEntry]) -> Self {
        Self {
            entries,
            current_page: 0,
            items_per_page: 16, // Show 25 items per page
        }
    }
    
    /// Returns only entries for current page
    fn current_page_entries(&self) -> &[FileSystemEntry] {
        let start = self.current_page * self.items_per_page;
        let end = (start + self.items_per_page).min(self.entries.len());
        &self.entries[start..end]
    }
    
    /// Moves to next page, returns success
    fn next_page(&mut self) -> bool {
        let max_page = (self.entries.len() + self.items_per_page - 1) / self.items_per_page - 1;
        if self.current_page < max_page {
            self.current_page += 1;
            true
        } else {
            false
        }
    }
    
    /// Moves to previous page, returns success
    fn prev_page(&mut self) -> bool {
        if self.current_page > 0 {
            self.current_page -= 1;
            true
        } else {
            false
        }
    }
    
    /// Gets total number of pages
    fn total_pages(&self) -> usize {
        (self.entries.len() + self.items_per_page - 1) / self.items_per_page
    }
    
    /// Converts display index to actual entry index
    fn get_actual_index(&self, display_index: usize) -> Option<usize> {
        let actual_index = self.current_page * self.items_per_page + display_index - 1;
        if actual_index < self.entries.len() {
            Some(actual_index)
        } else {
            None
        }
    }
    
    
    /// Sets the current page to a specific index with bounds checking
    /// 
    /// # Purpose
    /// Allows setting pagination state when restoring from saved navigation states,
    /// ensuring users can return to the exact same page they were viewing.
    /// 
    /// # Arguments
    /// * `page_index` - Zero-based page index to set as current
    /// 
    /// # Returns
    /// * `bool` - True if page was set successfully, false if out of bounds
    /// 
    /// # Bounds Checking
    /// - Validates page_index is within valid range (0 to max_page)
    /// - Clamps to valid bounds if out of range
    /// - Returns success/failure status for caller feedback
    /// 
    /// # Usage Context
    /// Used when restoring pocket dimensions or setting initial page state
    /// from NavigationState.current_page_index.
    /// 
    /// # Example
    /// ```rust
    /// // Set to page 3 (0-based = page 4 in display)
    /// if dir_view.set_current_page(3) {
    ///     println!("Set to page 4 successfully");
    /// } else {
    ///     println!("Page 4 doesn't exist, clamped to valid page");
    /// }
    /// ```
    fn set_current_page(&mut self, page_index: usize) -> bool {
        if self.entries.is_empty() {
            self.current_page = 0;
            return page_index == 0;
        }
        
        let max_page = (self.entries.len() + self.items_per_page - 1) / self.items_per_page - 1;
        
        if page_index <= max_page {
            self.current_page = page_index;
            true
        } else {
            // Page index is out of bounds, clamp to last valid page
            self.current_page = max_page;
            false
        }
    }
    
    /// Gets the current page index (0-based)
    /// 
    /// # Purpose
    /// Provides read access to the current page index for state management
    /// and synchronization with NavigationState.
    /// 
    /// # Returns
    /// * `usize` - Current page index (0-based)
    /// 
    /// # Usage Context
    /// Used when synchronizing DirectoryView state back to NavigationState
    /// after user navigation or when saving pocket dimensions.
    /// 
    /// # Example
    /// ```rust
    /// let current_page = dir_view.get_current_page();
    /// nav_state.current_page_index = current_page;  // Sync state
    /// ```
    fn get_current_page(&self) -> usize {
        self.current_page
    }
    
}

/// Represents a search result with its Levenshtein distance score and item details
/// 
/// # Purpose
/// This struct stores information about items that match a user's search query,
/// including their similarity score and position in the directory listing.
/// It enables displaying search results and selecting items from search results.
/// 
/// # Fields
/// * `item_name` - The name of the matching file or directory
/// * `item_path` - The full path to the matching item
/// * `distance` - The Levenshtein distance score (lower is better)
/// * `display_index` - The item's current display position in the file listing
/// 
/// # Levenshtein Distance
/// The distance field measures similarity between the search term and item name.
/// Lower values indicate closer matches:
/// - 0: Exact match
/// - 1: Single character difference
/// - 2: Two character differences
/// - etc.
/// 
/// # Usage
/// Used to store and sort fuzzy search matches when a user enters a search term.
/// Results are typically sorted by distance (best matches first), then displayed
/// for user selection.
///
/// # Example
/// ```rust
/// // Example search results for query "doc"
/// let results = vec![
///     SearchResult {
///         item_name: "document.txt".to_string(),
///         item_path: PathBuf::from("/path/to/document.txt"),
///         distance: 0,  // "doc" matches the prefix exactly
///         display_index: 3,
///     },
///     SearchResult {
///         item_name: "code.rs".to_string(),
///         item_path: PathBuf::from("/path/to/code.rs"),
///         distance: 1,  // One letter difference between "doc" and "cod"
///         display_index: 7,
///     },
/// ];
///
/// // Sort results by distance (best matches first)
/// results.sort_by_key(|r| r.distance);
///
/// // Display to user for selection
/// display_search_results(&results)?;
/// ```
#[derive(Debug)]
struct SearchResult {
    /// Name of the matching item
    item_name: String,
    /// Full path to the item
    item_path: PathBuf,
    /// Levenshtein distance score
    distance: usize,
    /// Display index from the current view
    display_index: usize,
}

/// Formats a timestamp into a human-readable format
/// 
/// # Purpose
/// Converts system timestamps into user-friendly date/time representations
/// that adapt based on how recent the timestamp is, prioritizing relevant
/// information over complete timestamps.
/// 
/// # Arguments
/// * `timestamp` - SystemTime to format
/// 
/// # Returns
/// * String - Formatted date/time string
/// 
/// # Format Rules
/// The function uses different formats based on the age of the timestamp:
/// - Today: "HH:MM" (e.g., "14:30")
/// - This year: "MM-DD HH:MM" (e.g., "09-15 14:30")
/// - Older: "YYYY-MM-DD" (e.g., "2022-09-15")
/// 
/// # Timezone Behavior
/// All times are displayed in the local system timezone.
/// 
/// # Edge Cases
/// - For timestamps that can't be compared with now (future with TryFrom error),
///   falls back to displaying them as if they're old timestamps
/// - The Unix epoch (1970-01-01) is handled correctly and displayed as "1970-01-01"
/// 
/// # Examples
/// ```rust
/// // Format the current time (will show HH:MM)
/// let now = SystemTime::now();
/// let formatted = format_timestamp(now);
/// 
/// // Format a file's modification time
/// let metadata = fs::metadata("example.txt")?;
/// let modified = metadata.modified()?;
/// let formatted = format_timestamp(modified);
/// ```
fn format_timestamp(timestamp: SystemTime) -> String {
    // Get current time and the file time as Duration since UNIX_EPOCH
    let now = SystemTime::now();
    
    // Convert timestamps to Duration since UNIX_EPOCH, handling errors
    let now_duration = now.duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0));
    let file_duration = timestamp.duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0));
    
    // Convert to seconds
    let now_secs = now_duration.as_secs();
    let file_secs = file_duration.as_secs();
    
    // Calculate time components
    let secs_per_day = 24 * 60 * 60;
    let days_diff = (now_secs - file_secs) / secs_per_day;
    
    // Get components for the file timestamp
    let (year, month, day, hour, minute) = seconds_to_components(file_secs);
    let (current_year, _, _) = seconds_to_ymd(now_secs);

    // Format based on how old the file is
    if days_diff == 0 {
        // Today: show time only
        format!("{:02}:{:02}", hour, minute)
    } else if year == current_year {
        // This year: show month-day and time
        format!("{:02}-{:02} {:02}:{:02}", month, day, hour, minute)
    } else {
        // Older: show full date
        format!("{}-{:02}-{:02}", year, month, day)
    }
}

/// Convert seconds since epoch to year, month, day, hour, minute components
/// 
/// # Purpose
/// Decomposes a Unix timestamp (seconds since 1970-01-01) into individual
/// date and time components for formatted display.
/// 
/// # Arguments
/// * `secs` - Seconds since Unix epoch (1970-01-01 00:00:00 UTC)
/// 
/// # Returns
/// * Tuple of (year, month, day, hour, minute) as u32 values
/// 
/// # Implementation Notes
/// - This is a simplified implementation that doesn't use the chrono crate
/// - Time components (hour, minute) are calculated using modular arithmetic
/// - Date components use the seconds_to_ymd helper function
/// 
/// # Limitations
/// - Does not account for leap seconds
/// - Uses simplified rules for leap years
/// - Does not handle timezone conversions (assumes UTC)
/// 
/// # Example
/// ```rust
/// let timestamp = 1632145200; // 2021-09-20 12:00:00 UTC
/// let (year, month, day, hour, minute) = seconds_to_components(timestamp);
/// assert_eq!(year, 2021);
/// assert_eq!(month, 9);
/// assert_eq!(day, 20);
/// assert_eq!(hour, 12);
/// assert_eq!(minute, 0);
/// ```
fn seconds_to_components(secs: u64) -> (u32, u32, u32, u32, u32) {
    let secs_per_minute = 60;
    let secs_per_hour = secs_per_minute * 60;
    let secs_per_day = secs_per_hour * 24;

    let minute = ((secs % secs_per_hour) / secs_per_minute) as u32;
    let hour = ((secs % secs_per_day) / secs_per_hour) as u32;
    
    let (year, month, day) = seconds_to_ymd(secs);
    
    (year, month, day, hour, minute)
}

/// Convert seconds since epoch to year, month, day components
/// 
/// # Purpose
/// A low-level helper function that calculates year, month, and day
/// from a Unix timestamp without external date/time libraries.
/// 
/// # Arguments
/// * `secs` - Seconds since Unix epoch (1970-01-01 00:00:00 UTC)
/// 
/// # Returns
/// * Tuple of (year, month, day) as u32 values
/// 
/// # Implementation Details
/// 1. Calculates days since epoch by dividing seconds by seconds per day
/// 2. Determines the year by counting full years from 1970
/// 3. Accounts for leap years by adding an extra day when appropriate
/// 4. Calculates month and day using days remaining after year calculation
/// 
/// # Limitations
/// - Uses simplified leap year calculation (doesn't handle all edge cases)
/// - Assumes Gregorian calendar rules for the entire period
/// - Does not apply timezone adjustments
/// 
/// # Example
/// ```rust
/// let timestamp = 1632145200; // 2021-09-20 12:00:00 UTC
/// let (year, month, day) = seconds_to_ymd(timestamp);
/// assert_eq!(year, 2021);
/// assert_eq!(month, 9);
/// assert_eq!(day, 20);
/// ```
fn seconds_to_ymd(secs: u64) -> (u32, u32, u32) {
    // This simplified implementation doesn't handle leap years correctly
    // Consider a more accurate algorithm or the chrono crate for production
    
    let secs_per_day = 24 * 60 * 60;
    let days_since_epoch = secs / secs_per_day;
    
    // Base date is 1970-01-01
    let mut year = 1970;
    let mut days_remaining = days_since_epoch;
    
    // Account for leap years
    loop {
        let days_in_year = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 366 } else { 365 };
        if days_remaining < days_in_year as u64 {
            break;
        }
        days_remaining -= days_in_year as u64;
        year += 1;
    }
    
    // Simplified month calculation
    let days_in_month = [31, 
        if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 }, 
        31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    
    let mut month = 0;
    while month < 12 && days_remaining >= days_in_month[month] as u64 {
        days_remaining -= days_in_month[month] as u64;
        month += 1;
    }
    
    // Add 1 to month (1-based) and day (1-based)
    (year as u32, (month + 1) as u32, (days_remaining + 1) as u32)
}

/// Sorts directory entries based on specified method while maintaining directories at the top
/// 
/// # Purpose
/// This function provides consistent sorting of directory contents according to user preferences,
/// while always preserving the convention that directories appear before files.
/// 
/// # Arguments
/// * `entries` - Mutable reference to vector of FileSystemEntry items to sort
/// * `sort_method` - Enum specifying sort method and direction
/// 
/// # Sort Methods
/// - Name: Alphabetical sort by filename
/// - Size: Sort by file size in bytes
/// - Modified: Sort by last modification timestamp
/// 
/// # Behavior Details
/// 1. Directory Priority:
///    - Directories always appear before files regardless of sort method
///    - Directory-to-directory comparisons use the chosen sort method
///    - File-to-file comparisons use the chosen sort method
/// 
/// 2. Sort Directions:
///    - Ascending: (default) A-Z, smallest-largest, oldest-newest
///    - Descending: Z-A, largest-smallest, newest-oldest
/// 
/// # Implementation Notes
/// - Uses stable sort to maintain relative order of equal elements
/// - Handles all three sort methods with consistent directory priority
/// - Each sort criterion has its own comparison logic:
///   * Name: String comparison of file_system_item_name
///   * Size: Numeric comparison of file_system_item_size_in_bytes
///   * Modified: DateTime comparison of file_system_item_last_modified_time
/// 
/// # Examples
/// ```rust
/// // Sort by name ascending
/// sort_directory_entries(&mut entries, DirectorySortingMethodEnum::Name(true));
/// 
/// // Sort by size descending
/// sort_directory_entries(&mut entries, DirectorySortingMethodEnum::Size(false));
/// 
/// // Sort by modification time ascending
/// sort_directory_entries(&mut entries, DirectorySortingMethodEnum::Modified(true));
/// ```
/// 
/// # Display Order Example
/// ```text
/// Sorting by size (ascending):
/// 1. Directory1/           (directories always first)
/// 2. Directory2/
/// 3. small_file.txt       10 B
/// 4. medium_file.doc      1 KB
/// 5. large_file.pdf       1 MB
/// ```
fn sort_directory_entries(
    entries: &mut Vec<FileSystemEntry>,
    sort_method: DirectorySortingMethodEnum,
) {
    match sort_method {
        DirectorySortingMethodEnum::Name(ascending) => {
            entries.sort_by(|a, b| {
                // Directories always first
                match (a.is_directory, b.is_directory) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => {
                        let cmp = a.file_system_item_name.cmp(&b.file_system_item_name);
                        if ascending { cmp } else { cmp.reverse() }
                    }
                }
            });
        },
        DirectorySortingMethodEnum::Size(ascending) => {
            entries.sort_by(|a, b| {
                match (a.is_directory, b.is_directory) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => {
                        let cmp = a.file_system_item_size_in_bytes.cmp(&b.file_system_item_size_in_bytes);
                        if ascending { cmp } else { cmp.reverse() }
                    }
                }
            });
        },
        DirectorySortingMethodEnum::Modified(ascending) => {
            entries.sort_by(|a, b| {
                match (a.is_directory, b.is_directory) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => {
                        let cmp = a.file_system_item_last_modified_time.cmp(&b.file_system_item_last_modified_time);
                        if ascending { cmp } else { cmp.reverse() }
                    }
                }
            });
        },
    }
}


/// Opens a new terminal window at the specified directory
/// 
/// # Purpose
/// This function launches a new terminal emulator window that starts in the
/// specified directory, allowing users to perform command-line operations
/// directly from their current file browsing location.
/// 
/// # Arguments
/// * `directory_path` - PathBuf of the directory to open terminal in
/// 
/// # Returns
/// * `Result<()>` - Success: () unit type
///                  Error: FileFantasticError with context
/// 
/// # Platform-specific Implementation
/// - **macOS**: Uses 'Terminal.app' via the 'open' command
/// - **Linux**: Tries multiple terminal emulators in order of preference:
///   1. gnome-terminal
///   2. ptyxis (Fedora 41+ default)
///   3. konsole
///   4. xfce4-terminal
///   5. mate-terminal
///   6. terminator
///   7. alacritty
///   8. kitty
///   9. tilix
///   10. urxvt
///   11. rxvt
///   12. xterm
/// - **Windows**: Uses 'cmd.exe' with appropriate arguments
/// 
/// # Error Handling
/// - Handles process spawn errors with specific error types
/// - Tries multiple terminal emulators on Linux
/// - Returns NoTerminalFound if no suitable terminal is available
/// - Returns UnsupportedPlatform for unsupported platforms
/// 
/// # Security Considerations
/// - Launches external processes, which could fail in restricted environments
/// - Requires execute permissions on terminal emulators
/// 
/// # Examples
/// ```rust
/// // When user presses 't' to open terminal at current location
/// match open_new_terminal(&current_directory) {
///     Ok(_) => println!("Terminal opened successfully"),
///     Err(FileFantasticError::NoTerminalFound) => {
///         eprintln!("No suitable terminal emulator found on your system");
///     },
///     Err(e) => eprintln!("Failed to open terminal: {}", e),
/// }
/// ```
fn open_new_terminal(directory_path: &PathBuf) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-a", "Terminal"])
            .arg(directory_path)
            .spawn()
            .map_err(|e| {
                // Provide context about the failed operation
                eprintln!("Failed to open Terminal.app: {}", e);
                FileFantasticError::Io(e)
            })?;
        return Ok(());
    }
    
    #[cfg(target_os = "linux")]
    {
        // Try different terminal emulators in order of preference
        let terminal_commands = [
            ("gnome-terminal", vec!["--working-directory"]),
            ("ptyxis", vec!["--working-directory"]),  // New Fedora 41+ default
            ("konsole", vec!["--workdir"]),
            ("xfce4-terminal", vec!["--working-directory"]),
            ("mate-terminal", vec!["--working-directory"]),
            ("terminator", vec!["--working-directory"]),
            ("alacritty", vec!["--working-directory"]),
            ("kitty", vec!["--directory"]),
            ("tilix", vec!["--working-directory"]),
            ("urxvt", vec!["-cd"]),
            ("rxvt", vec!["-cd"]),
            ("xterm", vec!["-e", "cd"]),  // xterm needs special handling
        ];

        for (terminal, args) in terminal_commands.iter() {
            let mut command = std::process::Command::new(terminal);
            
            if *terminal == "xterm" || *terminal == "urxvt" || *terminal == "rxvt" {
                // These terminals need special handling with the shell
                command.args(args)
                    .arg(directory_path.to_string_lossy().to_string())
                    .arg("&& bash");
            } else if *terminal == "alacritty" || *terminal == "kitty" {
                // Some newer terminals handle working directory differently
                command.arg(args[0])
                    .arg(directory_path);
            } else {
                command.args(args)
                    .arg(directory_path);
            }

            match command.spawn() {
                Ok(_) => return Ok(()),
                Err(e) => {
                    // Log the attempt but continue trying other terminals
                    eprintln!("Failed to open terminal {}: {}", terminal, e);
                    continue;
                }
            }
        }
        
        // None of the terminals worked
        return Err(FileFantasticError::NoTerminalFound);
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", "cmd.exe"])
            .current_dir(directory_path)
            .spawn()
            .map_err(|e| {
                eprintln!("Failed to open cmd.exe: {}", e);
                FileFantasticError::Io(e)
            })?;
        return Ok(());
    }
    
    // This is a fallback for platforms not explicitly handled
    #[allow(unreachable_code)]
    Err(FileFantasticError::UnsupportedPlatform)
}

/// Processes user input and returns the corresponding NavigationAction
/// 
/// # Purpose
/// This function serves as the central command interpreter for File Fantastic,
/// translating user text input into specific actions the application should take.
/// It handles commands, item selection, and search functionality in one unified interface.
/// 
/// # Arguments
/// * `input` - The user's input string
/// * `nav_state` - Current navigation state containing lookup table
/// * `all_entries` - Complete list of directory entries
/// 
/// # Returns
/// * `Result<NavigationAction>` - The determined action to take or error with context
/// 
/// # Command Processing Rules
/// 1. Empty input triggers refresh action
/// 2. Single-character inputs are checked against command mappings:
///    - `q` -> Quit
///    - `b` -> Parent directory
///    - `t` -> Open terminal
///    - `n/s/m` -> Sort commands
///    - `d/f/a` -> Filter commands
/// 3. Numeric inputs are treated as item selection
///    - Validates against current display lookup table
///    - Returns appropriate action based on item type (file/directory)
/// 4. Multi-character non-numeric inputs trigger fuzzy search
///    - Search results are displayed for user selection
///    - User can then select from results by number
/// 
/// # Error Handling
/// - Validates user input
/// - Handles IO errors during search result display
/// - Validates selected numbers against available items
/// - Returns specific FileFantasticError types with context
/// 
/// # Examples
/// ```rust
/// // Process user input and take appropriate action
/// match process_user_input(&input, &nav_state, &all_entries)? {
///     NavigationAction::Quit => break,
///     NavigationAction::ChangeDirectory(path) => current_directory = path,
///     NavigationAction::OpenFile(path) => handle_file_open(&path)?,
///     NavigationAction::Sort(c) => nav_state.toggle_sort(c),
///     NavigationAction::OpenNewTerminal => open_new_terminal(&current_directory)?,
///     NavigationAction::Filter(c) => nav_state.set_filter(c),
///     NavigationAction::Refresh => { /* reload directory */ },
///     NavigationAction::ParentDirectory => {
///         if let Some(parent) = current_directory.parent() {
///             current_directory = parent.to_path_buf();
///         }
///     },
///     NavigationAction::Invalid => println!("Invalid input"),
/// }
/// ```
fn process_user_input(
    input: &str,
    nav_state: &NavigationState,
    all_entries: &[FileSystemEntry],
) -> Result<NavigationAction> {
    let input = input.trim();
        
    // Handle empty input first - refresh and clear filters
    if input.is_empty() {
        return Ok(NavigationAction::Refresh);
    } 
        
    // Handle single-character commands first
    if input.len() == 1 {
        // Convert to lowercase for case-insensitive commands
        let lowercase_input = input.to_lowercase();
        
        match lowercase_input.as_str() {
            "q" => return Ok(NavigationAction::Quit),
            "b" => return Ok(NavigationAction::ParentDirectory),
            "t" => return Ok(NavigationAction::OpenNewTerminal),
            "n" => return Ok(NavigationAction::Sort('n')),
            "s" => return Ok(NavigationAction::Sort('s')),
            "m" => return Ok(NavigationAction::Sort('m')),
            "d" => return Ok(NavigationAction::Filter('d')), // Show directories only
            "f" => return Ok(NavigationAction::Filter('f')), // Show files only
            "a" => return Ok(NavigationAction::Filter('a')),
            "v" | "c" | "y" | "p" | "g" | "get" | "send" => return Ok(NavigationAction::GetSendMode),
            // u and d are handled in main loop for pagination
            _ => {}
        }
    }

    // Try to parse as number for direct selection
    // This will be used as a fallback when not handled by pagination
    if let Ok(number) = input.parse::<usize>() {
        if let Some(item_info) = nav_state.lookup_item(number) {
            return Ok(match item_info.item_type {
                FileSystemItemType::Directory => {
                    NavigationAction::ChangeDirectory(item_info.item_path.clone())
                }
                FileSystemItemType::File => {
                    NavigationAction::OpenFile(item_info.item_path.clone())
                }
            });
        }
    }

    // If not a command or number, treat as search
    let search_results = nav_state.fuzzy_search(input, all_entries);
    display_search_results(&search_results).map_err(|e| {
        eprintln!("Failed to display search results: {}", e);
        FileFantasticError::Io(e)
    })?;
    
    // Wait for user to select from results or press enter to continue
    print!("\nEnter number to select or press Enter to continue: ");
    io::stdout().flush().map_err(|e| {
        eprintln!("Failed to flush stdout: {}", e);
        FileFantasticError::Io(e)
    })?;
    
    let mut selection = String::new();
    io::stdin().read_line(&mut selection).map_err(|e| {
        eprintln!("Failed to read input: {}", e);
        FileFantasticError::Io(e)
    })?;
    
    if let Ok(number) = selection.trim().parse::<usize>() {
        // Find the search result with the matching display_index
        if let Some(result) = search_results.iter().find(|r| r.display_index == number) {
            // Get the original entry by its index to determine if it's a directory or file
            if let Some(entry) = all_entries.get(number - 1) {
                return Ok(if entry.is_directory {
                    NavigationAction::ChangeDirectory(result.item_path.clone())
                } else {
                    NavigationAction::OpenFile(result.item_path.clone())
                });
            }
        }
    }

    Ok(NavigationAction::Invalid)
}

/// Represents possible navigation actions based on user input in the file manager
/// 
/// # Purpose
/// This enum centralizes all possible actions that can result from user input,
/// providing a clear interface between input processing and action handling.
/// It serves as the primary control flow mechanism for the file manager's main loop.
/// 
/// # Variants
/// - `ChangeDirectory(PathBuf)` - Navigate into a specified directory
/// - `ParentDirectory` - Move up one directory level
/// - `OpenFile(PathBuf)` - Open a file with editor prompt
/// - `Quit` - Exit the application
/// - `Invalid` - Handle unrecognized or malformed input
/// - `Refresh` - Reload current directory contents
/// - `Sort(char)` - Change sort order based on command char
/// - `OpenNewTerminal` - Open new terminal in current directory
/// - `Filter(char)` - Filter directory contents by type
/// 
/// # Command Characters
/// Sort commands use specific characters:
/// - 'n' - Sort by name
/// - 's' - Sort by size
/// - 'm' - Sort by modification time
///
/// Filter commands use specific characters:
/// - 'd' - Show only directories
/// - 'f' - Show only files
/// - 'a' - Show all items (clear filter)
/// 
/// # Usage Example
/// ```rust
/// match process_user_input(&input, &nav_state)? {
///     NavigationAction::ChangeDirectory(path) => {
///         current_directory = path;
///     },
///     NavigationAction::OpenFile(path) => {
///         handle_file_open(&path)?;
///     },
///     NavigationAction::Sort('n') => {
///         // Toggle name sorting
///     },
///     // ... other actions ...
/// }
/// ```
/// 
/// # Input Mapping
/// - Numbers (1-N): Generate ChangeDirectory or OpenFile based on item type
/// - Empty string: Generates Refresh
/// - "q": Generates Quit
/// - "b": Generates ParentDirectory
/// - "t": Generates OpenNewTerminal
/// - "n"/"s"/"m": Generate Sort with respective character
/// - "d"/"f"/"a": Generate Filter with respective character
/// - Invalid input: Generates Invalid
/// 
/// # Error Handling
/// While the enum itself doesn't handle errors, actions using PathBuf
/// should handle potential file system errors when executed.
#[derive(Debug)]
enum NavigationAction {
    /// Change current directory to the specified path
    /// 
    /// Generated when user selects a directory by number
    ChangeDirectory(PathBuf),

    /// Move up one directory level to parent
    /// 
    /// Generated by 'b' command or when attempting to navigate
    /// up from current directory
    ParentDirectory,

    /// Open the specified file with editor prompt
    /// 
    /// Generated when user selects a file by number
    /// Triggers editor selection prompt before opening
    OpenFile(PathBuf),

    /// Exit the application cleanly
    /// 
    /// Generated by 'q' command
    Quit,

    /// Handle unrecognized or malformed input
    /// 
    /// Generated when input doesn't match any valid command
    /// or when selected item number is out of range
    Invalid,

    /// Reload and redisplay current directory contents
    /// 
    /// Generated by empty input (Enter key)
    /// Also used after operations that modify directory contents
    Refresh,

    /// Change sort order of directory listings
    /// 
    /// Character parameter indicates sort type:
    /// - 'n': Toggle name sort (ascending/descending)
    /// - 's': Toggle size sort (ascending/descending)
    /// - 'm': Toggle modification time sort (ascending/descending)
    Sort(char),

    /// Open a new terminal window in current directory
    /// 
    /// Generated by 't' command
    /// Uses platform-specific terminal launching mechanism
    OpenNewTerminal, 
    
    /// Filter to show only directories or files
    /// 
    /// 'd' shows only directories
    /// 'f' shows only files
    /// Any other value resets the filter to show everything
    Filter(char),
    
    /// Enter Get-Send-Mode for advanced file operations
    GetSendMode,
}

/// Formats file size into human readable format
/// 
/// # Purpose
/// Converts raw byte counts into user-friendly size representations
/// that are both concise and informative, using appropriate units.
/// 
/// # Arguments
/// * `size_in_bytes` - The file size in bytes
/// 
/// # Returns
/// * String - Formatted size string (e.g., "1.2 MB", "340 KB", "12 B")
/// 
/// # Format Rules
/// - Uses B, KB, MB, GB units
/// - Shows decimal point only when value < 10 in the chosen unit
/// - Maximum 1 decimal place
/// - Uses the largest unit that allows the number to be 0.1 to 99.99
/// - Zero bytes displayed as "0 B"
/// 
/// # Examples
/// ```rust
/// assert_eq!(format_file_size(0), "0 B");
/// assert_eq!(format_file_size(100), "100 B");
/// assert_eq!(format_file_size(1024), "1.0 KB");
/// assert_eq!(format_file_size(1536), "1.5 KB");
/// assert_eq!(format_file_size(10240), "10 KB");
/// assert_eq!(format_file_size(1048576), "1.0 MB");
/// assert_eq!(format_file_size(1073741824), "1.0 GB");
/// ```
///
/// # Implementation Notes
/// - Uses binary units (1 KB = 1024 bytes) rather than decimal (1 KB = 1000 bytes)
/// - Does not implement TB or larger units, which may be needed for very large files
fn format_file_size(size_in_bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    match size_in_bytes {
        0 => String::from("0 B"),
        bytes if bytes < KB => format!("{} B", bytes),
        bytes if bytes < MB => {
            let kb = bytes as f64 / KB as f64;
            if kb < 10.0 {
                format!("{:.1} KB", kb)
            } else {
                format!("{:.0} KB", kb)
            }
        }
        bytes if bytes < GB => {
            let mb = bytes as f64 / MB as f64;
            if mb < 10.0 {
                format!("{:.1} MB", mb)
            } else {
                format!("{:.0} MB", mb)
            }
        }
        bytes => {
            let gb = bytes as f64 / GB as f64;
            if gb < 10.0 {
                format!("{:.1} GB", gb)
            } else {
                format!("{:.0} GB", gb)
            }
        }
    }
}

/// Represents the type of an item in the file system
/// 
/// # Purpose
/// Provides a clear distinction between files and directories throughout the
/// application, allowing for type-specific handling and operations.
/// 
/// # Variants
/// - `Directory` - Represents a directory/folder
/// - `File` - Represents a regular file
/// 
/// # Usage
/// Used throughout the file manager to determine how to handle
/// different types of file system entries, particularly for:
/// - Display formatting (directories show trailing slash)
/// - Navigation behavior (directories can be entered)
/// - Operation selection (files can be opened)
/// - Filtering (showing only files or directories)
/// 
/// # Examples
/// ```rust
/// // Determine action based on item type
/// match item_info.item_type {
///     FileSystemItemType::Directory => {
///         // Navigate into directory
///         current_directory = item_path;
///     },
///     FileSystemItemType::File => {
///         // Open file with editor
///         open_file(&item_path)?;
///     },
/// }
/// ```
///
/// # Implementation Notes
/// - Implements PartialEq to allow direct comparison
/// - Implements Debug and Clone for utility
#[derive(Debug, Clone, PartialEq)]
enum FileSystemItemType {
    Directory,
    File,
}

/// Stores information about a displayed file system item for lookup by its display number
/// 
/// # Purpose
/// This struct maintains the mapping between display numbers shown to the user
/// and the actual file system items they represent, enabling selection by number.
/// It serves as a key component of the NavigationState's display lookup table.
/// 
/// # Fields
/// * `item_path` - The full path to the file system item
/// * `item_type` - Whether the item is a file or directory
/// 
/// # Usage Context
/// Used in the NavigationState's display_lookup_table to enable quick lookup
/// when a user selects an item by its display number. The mapping allows
/// translating user input (e.g., "5") into the corresponding file system action.
/// 
/// # Example
/// ```rust
/// // When user enters a number
/// if let Ok(number) = input.parse::<usize>() {
///     if let Some(item_info) = nav_state.lookup_item(number) {
///         match item_info.item_type {
///             FileSystemItemType::Directory => {
///                 // Handle directory selection
///                 current_directory = item_info.item_path.clone();
///             },
///             FileSystemItemType::File => {
///                 // Handle file selection
///                 open_file(&item_info.item_path)?;
///             }
///         }
///     } else {
///         println!("Invalid item number");
///     }
/// }
/// ```
#[derive(Debug)]
struct DisplayedItemInfo {
    /// The full path to the item
    item_path: PathBuf,
    /// The type of the item (file or directory)
    item_type: FileSystemItemType,
}

/// FileSystemEntry represents a single item (file or directory) in the file system
/// with its essential metadata for display and manipulation in the file manager.
/// 
/// # Purpose
/// This struct is the fundamental data structure of File Fantastic, holding
/// all relevant information about each file or directory that will be displayed
/// to the user. It provides a consistent interface to file system entries
/// regardless of platform.
/// 
/// # Design Philosophy
/// Properties are deliberately named to be extremely clear and unambiguous,
/// avoiding short or cryptic abbreviations to maximize code readability and
/// maintainability.
/// 
/// # Fields
/// * `file_system_item_name` - The complete name of the file or directory
/// * `file_system_item_path` - The full path to the file or directory
/// * `file_system_item_size_in_bytes` - Size of the file in bytes (0 for directories)
/// * `file_system_item_last_modified_time` - Last modification time as a SystemTime
/// * `is_directory` - Boolean flag indicating if this entry is a directory
/// 
/// # Usage Context
/// Instances are created during directory reading and used for:
/// - Displaying in the file browser interface
/// - Sorting based on various criteria (name, size, date)
/// - Searching by name
/// - Navigating when selected by user
/// - Determining appropriate actions (open file vs. enter directory)
/// 
/// # Example
/// ```rust
/// // Creating a FileSystemEntry from directory read results
/// let entry = FileSystemEntry {
///     file_system_item_name: dir_entry.file_name().to_string_lossy().to_string(),
///     file_system_item_path: dir_entry.path(),
///     file_system_item_size_in_bytes: metadata.len(),
///     file_system_item_last_modified_time: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
///     is_directory: metadata.is_dir(),
/// };
/// ```
#[derive(Debug)]
pub struct FileSystemEntry {
    /// The complete name of the file or directory
    file_system_item_name: String,
    
    /// The full path to the file or directory
    file_system_item_path: std::path::PathBuf,
    
    /// Size of the file in bytes (0 for directories)
    file_system_item_size_in_bytes: u64,
    
    /// Last modification time as a SystemTime
    file_system_item_last_modified_time: SystemTime,
    
    /// Boolean flag indicating if this entry is a directory
    is_directory: bool,
}

/// Displays search results in a formatted table with clear headers
/// 
/// # Purpose
/// Presents fuzzy search results to the user in a readable format,
/// allowing them to select a matching file or directory by number.
/// 
/// # Arguments
/// * `results` - Vector of SearchResult items to display
/// 
/// # Returns
/// * `io::Result<()>` - Success or IO error
/// 
/// # Display Format
/// ```text
/// Search Results   (Levenshtein < 3)
/// Num   Name                           Distance
/// ---------------------------------------------
///  1    example.txt                       2
///  2    sample.doc                        3
/// ```
/// 
/// # User Interface
/// - Clears the screen before displaying results
/// - Shows header with column names
/// - Displays each result with its original item number
/// - Shows the Levenshtein distance (lower is better)
/// - Handles empty results with a "No matches found" message
/// 
/// # Notes
/// - Truncates long filenames to fit display width (max 30 characters)
/// - Shows original item numbers from directory listing for selection
/// - Distance indicates how close the match is (lower is better)
/// 
/// # Error Handling
/// - Returns IO errors from terminal output operations
/// 
/// # Example
/// ```rust
/// // After performing a search
/// let results = nav_state.fuzzy_search("doc", &directory_entries);
/// if !results.is_empty() {
///     display_search_results(&results)?;
///     
///     // Get user selection from search results
///     print!("\nEnter number to select or press Enter to continue: ");
///     io::stdout().flush()?;
///     let mut selection = String::new();
///     io::stdin().read_line(&mut selection)?;
///     
///     // Process selection...
/// } else {
///     println!("No matches found");
/// }
/// ```
fn display_search_results(results: &[SearchResult]) -> io::Result<()> {
    if results.is_empty() {
        println!("No matches found");
        return Ok(());
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("\nSearch Results   (Levenshtein < 3)");
    println!("{:<5} {:<30} {:<10}", "Num", "Name", "Distance");
    println!("{}", "-".repeat(45));

    for result in results {
        println!("{:<5} {:<30} {:<10}", 
                result.display_index,
                if result.item_name.len() > 30 {
                    format!("{}...", &result.item_name[..27])
                } else {
                    result.item_name.clone()
                },
                result.distance);
    }
    
    Ok(())
}

/// Manages navigation state, lookup tables, and sort/filter settings
/// 
/// This struct serves as the central state manager for the File Fantastic UI,
/// tracking display mappings, sort preferences, filter settings, and search capabilities.
/// It maintains the connection between what the user sees on screen and the underlying
/// file system entities.
/// 
/// # State Management
/// - Maps displayed item numbers to actual file system items
/// - Tracks current sort method and direction
/// - Maintains filter settings (showing files/directories/all)
/// - Provides fuzzy search capability
/// 
/// # Lifecycle
/// The NavigationState is created once at application start and persists
/// throughout the session, being updated as the user navigates, sorts,
/// filters, or searches the file system.
/// 
/// # Key Responsibilities
/// 1. **Display Mapping**: Maps display numbers (what user sees) to actual file paths
/// 2. **Sort Management**: Tracks and toggles sort methods/directions
/// 3. **Filter Application**: Applies file/directory filters to listings
/// 4. **Search Functionality**: Performs fuzzy text searches
/// 
/// # Usage Context
/// The NavigationState is passed to various UI functions to maintain
/// consistency between user actions and display state.
/// 
/// # Implementation Notes
/// - Uses HashMap for O(1) lookup of items by display number
/// - Maintains last sort command to enable toggling behavior
/// - Filters are implemented as Option<char> for three states
/// - Fuzzy search implements Levenshtein distance algorithm
/// Manages navigation state, lookup tables, and sort settings for the file manager
/// 
/// # Purpose
/// `NavigationState` is the central component that tracks:
/// - Display lookup tables to map list numbers to file system items
/// - Current sort method and direction
/// - Search and navigation history
/// 
/// # Fields
/// * `display_lookup_table` - Maps displayed numbers to item information
/// * `current_sort_method` - Current active sort method and direction
/// * `last_sort_command` - Tracks last sort command for toggling direction
/// 
/// # Usage
/// The `NavigationState` maintains the stateful aspects of the UI,
/// allowing the application to map user input (numbers) to file system 
/// operations, track sort preferences, and handle search functionality.
/// 
/// # Lifecycle
/// 1. Created at application start with default values
/// 2. Updated when directory contents change or sort preferences change
/// 3. Consulted when processing user input to resolve actions
/// 
/// # Example
/// ```
/// let mut nav_state = NavigationState::new();
/// // After reading directory contents:
/// nav_state.update_lookup_table(&directory_entries);
/// // When processing user input:
/// if let Some(item_info) = nav_state.lookup_item(user_input_number) {
///     // Perform action on the item
/// }
/// // When changing sort method:
/// nav_state.toggle_sort('n'); // Toggle name sort
/// ```
pub struct NavigationState {
    /// Lookup table mapping displayed numbers to item information
    /// Key: displayed number (1-based index shown to user)
    /// Value: information about the item at that display position
    display_lookup_table: HashMap<usize, DisplayedItemInfo>,
    
    /// Current sort method and direction for directory contents
    current_sort_method: DirectorySortingMethodEnum,
    
    /// Tracks last sort command used to handle toggles between ascending/descending
    /// None if no sort command has been used yet
    last_sort_command: Option<char>,
    
    /// Current filter setting (None = show all, Some('d') = dirs only, 
    /// Some('f') = files only)
    current_filter: Option<char>,
    
    /// Currently selected item index (1-based, None if no selection)
    selected_item_index: Option<usize>,
    
    /// Active search term if user is searching
    active_search_term: Option<String>,
    
    /// Terminal size for display calculations (width, height)
    terminal_size: (usize, usize),
    
    /// Current page index (0-based) - what page we're currently viewing
    current_page_index: usize,
}


impl NavigationState {
    /// Creates a new NavigationState with default settings
    /// 
    /// # Returns
    /// * `NavigationState` - A new instance with:
    ///   - Empty lookup table
    ///   - Name sort in ascending order as default
    ///   - No last sort command
    /// 
    /// # Default Configuration
    /// - Sort by name in ascending order
    /// - Empty lookup table (populated after directory read)
    /// - No last sort command recorded
    /// 
    /// # Example
    /// ```
    /// let nav_state = NavigationState::new();
    /// // nav_state is ready to be used with initial directory read
    /// ```
    fn new() -> Self {
        NavigationState {
            display_lookup_table: HashMap::new(),
            current_sort_method: DirectorySortingMethodEnum::Name(true),
            last_sort_command: None,
            current_filter: None, // No filter initially
            selected_item_index: None,
            active_search_term: None,
            terminal_size: (80, 24), // Default terminal size
            current_page_index: 0, // Always start at page 0
        }
    }

    /// Set or toggle filter mode
    /// 
    /// # Purpose
    /// Controls which types of file system items are displayed in the directory listing,
    /// allowing the user to focus on just files, just directories, or all items.
    /// 
    /// # Arguments
    /// * `filter_char` - Character indicating the filter type to apply:
    ///   - 'd': Show only directories
    ///   - 'f': Show only files
    ///   - 'a' or others: Show all items (clear filter)
    /// 
    /// # Behavior
    /// - Toggling behavior: selecting the same filter twice turns it off
    /// - Only one filter can be active at a time
    /// - Used by the 'd' and 'f' keyboard commands
    /// 
    /// # Example
    /// ```
    /// // Show only directories
    /// nav_state.set_filter('d');
    /// 
    /// // Show only files
    /// nav_state.set_filter('f');
    /// 
    /// // Show all items
    /// nav_state.set_filter('a');
    /// 
    /// // Toggle directories filter off (if currently showing only directories)
    /// nav_state.set_filter('d');
    /// ```
    fn set_filter(&mut self, filter_char: char) {
        match filter_char {
            'd' => {
                // Toggle directory filter
                self.current_filter = if self.current_filter == Some('d') {
                    None // Turn off filtering if already showing dirs only
                } else {
                    Some('d') // Show directories only
                };
            }
            'f' => {
                // Toggle file filter
                self.current_filter = if self.current_filter == Some('f') {
                    None // Turn off filtering if already showing files only
                } else {
                    Some('f') // Show files only
                };
            }
            _ => self.current_filter = None, // Reset filter for any other character
        }
    }

    /// Sets the selected item index
    fn set_selected_item(&mut self, index: Option<usize>) {
        self.selected_item_index = index;
    }

    /// Gets the currently selected item path if any
    fn get_selected_item_path(&self) -> Option<PathBuf> {
        self.selected_item_index
            .and_then(|idx| self.lookup_item(idx))
            .map(|info| info.item_path.clone())
    }

    // /// Sets the active search term
    // fn set_search_term(&mut self, term: Option<String>) {
    //     self.active_search_term = term;
    // }

    // /// Updates terminal size
    // fn update_terminal_size(&mut self, width: usize, height: usize) {
    //     self.terminal_size = (width, height);
    // }
    
    /// Apply current filter to entries
    /// 
    /// # Purpose
    /// Applies the current filter setting to the directory entries, returning
    /// only the entries that should be displayed according to the filter.
    /// 
    /// # Arguments
    /// * `entries` - Slice of FileSystemEntry items to be filtered
    /// 
    /// # Returns
    /// * `Vec<&'a FileSystemEntry>` - Vector of references to entries that pass the filter
    /// 
    /// # Filter Modes
    /// - Some('d'): Show only directories
    /// - Some('f'): Show only files
    /// - None: Show all entries (no filtering)
    /// 
    /// # Usage Context
    /// Called during the main loop before displaying directory contents to
    /// present only the file types the user wants to see.
    /// 
    /// # Example
    /// ```
    /// // Apply current filter and get filtered entries to display
    /// let filtered_entries = nav_state.apply_filter(&all_entries);
    /// display_directory_contents(&filtered_entries, &current_directory)?;
    /// ```
    fn apply_filter<'a>(&self, entries: &'a [FileSystemEntry]) -> Vec<&'a FileSystemEntry> {
        match self.current_filter {
            Some('d') => entries.iter()
                .filter(|e| e.is_directory)
                .collect(),
            Some('f') => entries.iter()
                .filter(|e| !e.is_directory)
                .collect(),
            _ => entries.iter().collect(), // No filtering
        }
    }

    /// Performs a fuzzy text search on current directory contents using Levenshtein distance
    /// 
    /// # Arguments
    /// * `search_term` - The text to search for
    /// * `directory_entries` - Vector of current directory entries to search through
    /// 
    /// # Returns
    /// * `Vec<SearchResult>` - Vector of matching items sorted by Levenshtein distance
    /// 
    /// # Search Behavior
    /// - Only compares against the same number of characters as in the search term
    /// - For example, searching for "car" only looks at the first 3 characters of each item
    /// - Searches both filenames and directories
    /// - Removes file extensions before comparing
    /// - Converts both search term and filenames to lowercase
    /// - Only includes results with distance <= MAX_SEARCH_DISTANCE
    /// - Results are sorted by distance (closest matches first)
    /// 
    /// # Example
    /// ```
    /// // Searching for "car" will effectively compare against:
    /// // "Cargo.toml" -> "car"
    /// // "carpenter.txt" -> "car"
    /// // "calendar.pdf" -> "cal"
    /// ```
    fn fuzzy_search(&self, search_term: &str, directory_entries: &[FileSystemEntry]) -> Vec<SearchResult> {
        
        // Early return for empty search term to avoid unnecessary processing
        if search_term.is_empty() {
            return Vec::new();
        }
        
        let mut results = Vec::new();
        let search_term = search_term.to_lowercase();
        let search_len = search_term.chars().count();
        
        for (idx, entry) in directory_entries.iter().enumerate() {
            // Remove file extension for comparison
            let name_without_ext = match entry.file_system_item_name.rsplit_once('.') {
                Some((name, _ext)) => name.to_string(),
                None => entry.file_system_item_name.clone(),
            };
            
            // Get truncated versions of the names (matching search term length)
            let full_name_truncated: String = entry.file_system_item_name
                .to_lowercase()
                .chars()
                .take(search_len)
                .collect();
                
            let no_ext_truncated: String = name_without_ext
                .to_lowercase()
                .chars()
                .take(search_len)
                .collect();
            
            // Compare both truncated versions
            let distance_with_ext = levenshtein_distance(
                &full_name_truncated, 
                &search_term
            );
            let distance_without_ext = levenshtein_distance(
                &no_ext_truncated,
                &search_term
            );
            
            // Use the better of the two distances
            let distance = distance_with_ext.min(distance_without_ext);
            
            if distance <= MAX_SEARCH_DISTANCE {
                results.push(SearchResult {
                    item_name: entry.file_system_item_name.clone(),
                    item_path: entry.file_system_item_path.clone(),
                    distance,
                    display_index: idx + 1,
                });
            }
        }
        
        // Sort first by distance, then by original name length
        // This prioritizes exact prefix matches
        results.sort_by(|a, b| {
            match a.distance.cmp(&b.distance) {
                std::cmp::Ordering::Equal => a.item_name.len().cmp(&b.item_name.len()),
                other => other
            }
        });
        
        results
    }

    /// Toggle sort method based on input command
    fn toggle_sort(&mut self, command: char) {
        let new_sort_method = match command {
            'n' => {
                if self.last_sort_command == Some('n') {
                    // If already sorting by name, toggle direction
                    match self.current_sort_method {
                        DirectorySortingMethodEnum::Name(ascending) => DirectorySortingMethodEnum::Name(!ascending),
                        _ => DirectorySortingMethodEnum::Name(true),
                    }
                } else {
                    DirectorySortingMethodEnum::Name(true)
                }
            },
            's' => {
                if self.last_sort_command == Some('s') {
                    match self.current_sort_method {
                        DirectorySortingMethodEnum::Size(ascending) => DirectorySortingMethodEnum::Size(!ascending),
                        _ => DirectorySortingMethodEnum::Size(true),
                    }
                } else {
                    DirectorySortingMethodEnum::Size(true)
                }
            },
            'm' => {
                if self.last_sort_command == Some('m') {
                    match self.current_sort_method {
                        DirectorySortingMethodEnum::Modified(ascending) => DirectorySortingMethodEnum::Modified(!ascending),
                        _ => DirectorySortingMethodEnum::Modified(true),
                    }
                } else {
                    DirectorySortingMethodEnum::Modified(true)
                }
            },
            _ => return,
        };

        self.current_sort_method = new_sort_method;
        self.last_sort_command = Some(command);
    }

    /// Updates the lookup table based on current directory contents
    /// 
    /// # Arguments
    /// * `directory_entries` - The current directory entries being displayed
    fn update_lookup_table(&mut self, directory_entries: &[FileSystemEntry]) {
        self.display_lookup_table.clear();
        
        for (index, entry) in directory_entries.iter().enumerate() {
            self.display_lookup_table.insert(
                index + 1, // 1-based display index
                DisplayedItemInfo {
                    item_path: entry.file_system_item_path.clone(),
                    item_type: if entry.is_directory {
                        FileSystemItemType::Directory
                    } else {
                        FileSystemItemType::File
                    },
                }
            );
        }
    }

    /// Looks up an item by its display number
    /// 
    /// # Arguments
    /// * `display_number` - The number shown to the user (1-based index)
    /// 
    /// # Returns
    /// * `Option<&DisplayedItemInfo>` - The item info if found, None if not
    fn lookup_item(&self, display_number: usize) -> Option<&DisplayedItemInfo> {
        self.display_lookup_table.get(&display_number)
    }
}

/// Represents available sort methods and their directions for directory listings
/// 
/// # Variants
/// 
/// * `Name(bool)` - Sort by filename alphabetically
///   - `true`: A-Z (ascending)
///   - `false`: Z-A (descending)
/// 
/// * `Size(bool)` - Sort by file size
///   - `true`: Smallest to largest (ascending)
///   - `false`: Largest to smallest (descending)
/// 
/// * `Modified(bool)` - Sort by modification timestamp
///   - `true`: Oldest to newest (ascending)
///   - `false`: Newest to oldest (descending)
/// 
/// # Usage Example
/// ```
/// let name_ascending = DirectorySortingMethodEnum::Name(true);
/// let size_descending = DirectorySortingMethodEnum::Size(false);
/// let modified_ascending = DirectorySortingMethodEnum::Modified(true);
/// ```
/// 
/// # Implementation Notes
/// - Boolean parameter indicates direction (true=ascending, false=descending)
/// - When sorting by any method, directories are always grouped before files
/// - Used by `sort_directory_entries()` to determine sort behavior
/// - Used by `NavigationState` to track current sort settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DirectorySortingMethodEnum {
    /// Sort alphabetically by filename (true=A-Z, false=Z-A)
    Name(bool),
    
    /// Sort by file size in bytes (true=smallest first, false=largest first)
    Size(bool),
    
    /// Sort by last modification time (true=oldest first, false=newest first)
    Modified(bool),
}


/// Reads contents of a directory and returns a Result containing a vector of FileSystemEntry items
/// 
/// # Arguments
/// * `directory_path_to_read` - The PathBuf pointing to the directory to be read
/// 
/// # Returns
/// * `Result<Vec<FileSystemEntry>>` - Success: Vector of FileSystemEntry items
///                                  Error: FileFantasticError with context
/// 
/// # Error Handling
/// - Handles directory read errors with specific error types:
///   * NotFound: When directory doesn't exist
///   * PermissionDenied: When access is denied
///   * MetadataError: When file metadata can't be read
/// 
/// # Example Usage
/// ```
/// let current_path = std::env::current_dir()?;
/// let directory_entries = read_directory_contents(&current_path)?;
/// ```
fn read_directory_contents(directory_path_to_read: &PathBuf) -> Result<Vec<FileSystemEntry>> {
    let mut directory_entries_list: Vec<FileSystemEntry> = Vec::new();
    
    // Handle directory read errors with specific error types
    let read_dir_result = match fs::read_dir(directory_path_to_read) {
        Ok(dir) => dir,
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => 
                    return Err(FileFantasticError::NotFound(directory_path_to_read.clone())),
                io::ErrorKind::PermissionDenied => 
                    return Err(FileFantasticError::PermissionDenied(directory_path_to_read.clone())),
                _ => return Err(FileFantasticError::Io(e)),
            }
        }
    };

    for directory_item_result in read_dir_result {
        // Handle errors reading directory entries
        let directory_item = match directory_item_result {
            Ok(item) => item,
            Err(e) => {
                // Log error but continue with other entries
                eprintln!("Warning: Failed to read entry in {}: {}", 
                          directory_path_to_read.display(), e);
                continue;
            }
        };
        
        let item_path = directory_item.path();
        
        // Get metadata with better error context - actually use MetadataError
        let item_metadata = match directory_item.metadata() {
            Ok(meta) => meta,
            Err(_) => {
                // Instead of just skipping, at least log the error using our specific type
                let meta_error = FileFantasticError::MetadataError(item_path.clone());
                eprintln!("Warning: {}", meta_error);
                continue;
            }
        };
        
        // Handle modification time error with a fallback to UNIX_EPOCH
        let modified_time = match item_metadata.modified() {
            Ok(time) => time,
            Err(_) => {
                // When modification time is not available, use epoch as fallback
                eprintln!("Warning: Cannot determine modification time for {}", 
                          item_path.display());
                SystemTime::UNIX_EPOCH
            }
        };
        
        directory_entries_list.push(FileSystemEntry {
            file_system_item_name: directory_item
                .file_name()
                .to_string_lossy()
                .to_string(),
            file_system_item_path: item_path,
            file_system_item_size_in_bytes: item_metadata.len(),
            file_system_item_last_modified_time: modified_time,
            is_directory: item_metadata.is_dir(),
        });
    }

    Ok(directory_entries_list)
}

/// Test suite for file manager functionality
/// 
/// # Test Coverage
/// 
/// ## File Size Formatting Tests
/// Verifies that `format_file_size()` correctly formats file sizes:
/// - Zero bytes displays as "0 B"
/// - Bytes display with B suffix (e.g., "100 B")
/// - Kilobytes display with KB suffix and decimal precision for values < 10 (e.g., "1.0 KB")
/// - Larger values use appropriate units (KB, MB, GB) with correct formatting
/// 
/// ## Timestamp Formatting Tests
/// Verifies that `format_timestamp()` produces correctly formatted time strings:
/// - Current day timestamps use HH:MM format
/// - Recent timestamps (this year) use MM-DD HH:MM format
/// - Older timestamps use YYYY-MM-DD format
/// - Ensures consistent formatting with correct string lengths
/// - Verifies epoch time (1970-01-01) is correctly formatted
/// 
/// # Usage Notes
/// - These tests use deterministic inputs to verify consistent outputs
/// - Time-based tests use relative offsets from current time
/// - Some assertions check string length to verify format patterns
/// - Specific edge cases (like epoch time) are explicitly tested
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(100), "100 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(10240), "10 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }
    
    #[test]
    fn test_format_timestamp() {
        let now = SystemTime::now();
        let yesterday = now - Duration::from_secs(24 * 60 * 60);
        let last_month = now - Duration::from_secs(30 * 24 * 60 * 60);
        
        // Test current time format (HH:MM)
        let now_formatted = format_timestamp(now);
        assert!(now_formatted.len() == 5, "Current time should be in HH:MM format");
        
        // Test this year format (MM-DD HH:MM)
        let yesterday_formatted = format_timestamp(yesterday);
        assert!(yesterday_formatted.len() == 11, "Recent dates should be in MM-DD HH:MM format");
        
        // Test old date format (YYYY-MM-DD)
        let old_date = SystemTime::UNIX_EPOCH + Duration::from_secs(0);
        let old_formatted = format_timestamp(old_date);
        assert_eq!(old_formatted, "1970-01-01");
    }
}

/// Truncates a file name for display in CLI, keeping both the beginning and the file extension.
/// 
/// # Purpose
/// Ensures long filenames are displayed in a readable format that fits within
/// the terminal width constraints while preserving the most meaningful parts:
/// the beginning of the name and the file extension.
/// 
/// # Arguments
/// * `formatted_name` - The original filename to be truncated
/// 
/// # Returns
/// * `String` - Truncated name if necessary, or the original if it's short enough
/// 
/// # Truncation Method
/// If the name exceeds `MAX_NAME_LENGTH` (55 characters):
/// 1. Takes the first (MAX_NAME_LENGTH - SUFFIX_LENGTH - ellipsis.len()) characters
/// 2. Adds an ellipsis ("...")
/// 3. Keeps the last SUFFIX_LENGTH (5) characters (typically file extension)
/// 
/// # Examples
/// ```rust
/// let long_name = "really_long_filename_that_exceeds_the_maximum_length_for_display.txt";
/// assert_eq!(
///     truncate_filename_for_display(long_name.to_string()),
///     "really_long_filename_that_exceeds_the_maximum_leng...e.txt"
/// );
/// 
/// let short_name = "short.txt";
/// assert_eq!(
///     truncate_filename_for_display(short_name.to_string()),
///     "short.txt"
/// );
/// ```
/// 
/// # Constants Used
/// - MAX_NAME_LENGTH = 55
/// - FILENAME_SUFFIX_LENGTH = 5
fn truncate_filename_for_display(formatted_name: String) -> String {
    // MAX_NAME_LENGTH
    // FILENAME_SUFFIX_LENGTH
    let ellipsis = "...";
    
    if formatted_name.chars().count() <= MAX_NAME_LENGTH {
        return formatted_name;
    }
    
    // Calculate how many characters we can take from the start
    // (max_length - suffix_length - ellipsis.len())
    let prefix_length = MAX_NAME_LENGTH - FILENAME_SUFFIX_LENGTH - ellipsis.len();
    
    // Get prefix (start of the filename)
    let prefix: String = formatted_name.chars().take(prefix_length).collect();
    
    // Get suffix (end of the filename, including extension)
    let suffix: String = formatted_name
        .chars()
        .skip(formatted_name.chars().count() - FILENAME_SUFFIX_LENGTH)
        .collect();
    
    // Combine prefix, ellipsis, and suffix
    format!("{}{}{}", prefix, ellipsis, suffix)
}

/// Formats and displays directory contents as a numbered list with columns
/// 
/// For adjustments:
/// then name and data displays are different
/// the size date field uses a max of 6 char
/// the modified (date) uses a max of 11 char
/// the max text with elipsis is 55
/// so 52 is the trim point if over 55
/// numbering... if under 100 is 3 spaces
///
/// # Arguments
/// * `directory_entries` - Vector of FileSystemEntry items to display
/// * `current_directory_path` - PathBuf of the directory being displayed
/// 
/// # Returns
/// * `io::Result<()>` - Success: () unit type
///                      Error: IO error with description
/// 
/// # Display Format
/// ```text
/// Current Directory: /path/to/current/dir
/// 
/// Num  Name                    Size (B)    Modified
/// ------------------------------------------------
///  1)  Documents/             0           1696789200
///  2)  example.txt           1024        1696789100
/// ```
/// 
/// # Error Handling
/// - Handles display formatting errors
/// - Handles IO write errors
/// 
/// # Notes
/// - Directory entries are marked with a trailing '/'
/// - Sizes are displayed in bytes for MVP (future: human readable sizes)
/// - Modified times are in Unix timestamp for MVP (future: human readable dates)
/// Formats and displays directory contents as a numbered list with columns
/// Formats and displays directory contents as a numbered list with columns
/// Update display_directory_contents to use formatted timestamp
// fn display_directory_contents(
//     directory_entries: &[FileSystemEntry],
//     current_directory_path: &PathBuf,
// ) -> io::Result<()> {
/// Formats and displays directory contents as a numbered list with columns
fn display_directory_contents(
    directory_entries: &[FileSystemEntry],
    current_directory_path: &PathBuf,
    page_info: Option<(usize, usize)>,
    filter: Option<char>,
) -> io::Result<()> {
    // clear screen
    print!("\x1B[2J\x1B[1;1H");

    let filter_status = match filter {
        Some('d') => "[Directories only] ",
        Some('f') => "[Files only] ",
        _ => "",
    };
                    
    let legend = format!(
        "{}{}{}q{}uit {}b{}ack|{}t{}erm|{}d{}ir {}f{}ile|{}n{}ame {}s{}ize {}m{}od|{}g{}et-send file {}v{},{}y{},{}p{}|{}str{}>search|{}enter{}>reset{}", 
        YELLOW,           // Overall legend color
        filter_status,    // Filter status text
        RED, YELLOW,      // RED q + YELLOW uit
        RED, YELLOW,      // RED b + YELLOW ack
        RED, YELLOW,      // RED t + YELLOW erm
        RED, YELLOW,      // RED d + YELLOW ir
        RED, YELLOW,      // RED f + YELLOW ile
        RED, YELLOW,      // RED n + YELLOW ame
        RED, YELLOW,      // RED s + YELLOW ize
        RED, YELLOW,      // RED m + YELLOW od
        RED, YELLOW,      // RED g + YELLOW et
        RED, YELLOW,      // RED v + YELLOW ,
        RED, YELLOW,      // RED y + YELLOW ,
        RED, YELLOW,      // RED p + YELLOW ,
        RED, YELLOW,      // RED str + YELLOW ...
        RED, YELLOW,      // RED enter + YELLOW ...
        RESET);
    
    // // Updated legend to include Get-Send-Mode commands
    // let legend = format!(
    //     "{}{}(q)uit (b)ack|(t)erminal|(d)ir (f)ile|(n)ame (s)ize (m)od|(g)et|str>search|enter>reset{}", 
    //     YELLOW,
    //     filter_status,
    //     RESET);

    let path_display = format!("{}", current_directory_path.display());
    println!("{}\n{}", legend, path_display);

    // Rest of the function remains the same...
    // Column headers
    println!(
        "{:>4}  {:<53} {:>7} {:>11}",
        "Num", "Name", "Size", "Modified"
    );
    println!(" {} ", "-".repeat(78));

    // Display entries
    for (entry_index, directory_entry) in directory_entries.iter().enumerate() {
        let formatted_name = if directory_entry.is_directory {
            format!("{}/", directory_entry.file_system_item_name)
        } else {
            directory_entry.file_system_item_name.clone()
        };

        let display_name = truncate_filename_for_display(formatted_name);

        let size_display = if directory_entry.is_directory {
            String::from("-")
        } else {
            format_file_size(directory_entry.file_system_item_size_in_bytes)
        };

        let time_display = format_timestamp(directory_entry.file_system_item_last_modified_time);

        println!(
            "{:>3}. {:<55} {:>6} {:>11}",
            entry_index + 1,
            display_name,
            size_display,
            time_display
        );
    }

    // Add pagination footer if applicable
    if let Some((current_page, total_pages)) = page_info {
        if total_pages > 1 {
            println!("--- Page {} of {}: up/down, j/k, </>, w/x, arrows, etc. ---", 
                    current_page, total_pages);
        }
    }

    io::stdout().flush()?;
    Ok(())
}

/// Opens a file with user-selected editor in a new terminal window
/// 
/// # Arguments
/// * `file_path` - PathBuf of the file to open
/// 
/// # Returns
/// * `Result<()>` - Success or FileFantasticError with context
/// 
/// # Behavior
/// - Prompts user to select editor (e.g., nano, vim, code)
/// - Empty input uses system default opener
/// - Terminal-based editors open in new terminal window
/// - GUI editors (code, sublime, etc.) launch directly
/// - Falls back to system default if editor fails
/// 
/// # Error Handling
/// - Handles IO errors for user input/output
/// - Handles process spawn failures
/// - Provides fallbacks when editor launch fails
/// - Gives user feedback for all error cases
/// 
/// # Example Output
/// ```text
/// Open with (enter for default, or type: nano/vim/code/etc): vim
/// ```
fn open_file(file_path: &PathBuf) -> Result<()> {
    print!("Open with... (hit enter for default, or enter your software 'name' as called in a terminal: gedit, firefox, hx, lapce, vi, vim, nano, code, etc.): ");
    io::stdout().flush().map_err(|e| {
        eprintln!("Failed to flush stdout: {}", e);
        FileFantasticError::Io(e)
    })?;
    
    let mut editor = String::new();
    io::stdin().read_line(&mut editor).map_err(|e| {
        eprintln!("Failed to read input: {}", e);
        FileFantasticError::Io(e)
    })?;
    let editor = editor.trim();

    if editor.is_empty() {
        // Use system default
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(file_path)
                .spawn()
                .map_err(|e| {
                    eprintln!("Failed to open file with default application: {}", e);
                    FileFantasticError::Io(e)
                })?;
        }
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(file_path)
                .spawn()
                .map_err(|e| {
                    eprintln!("Failed to open file with xdg-open: {}", e);
                    FileFantasticError::Io(e)
                })?;
        }
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "start", ""])
                .arg(file_path)
                .spawn()
                .map_err(|e| {
                    eprintln!("Failed to open file with default application: {}", e);
                    FileFantasticError::Io(e)
                })?;
        }
    } else {
        // List of known GUI editors that shouldn't need a terminal
        let gui_editors = ["code", "sublime", "subl", "gedit", "kate", "notepad++"];
        
        if gui_editors.contains(&editor.to_lowercase().as_str()) {
            // Launch GUI editors directly - use EditorLaunchFailed
            match std::process::Command::new(editor)
                .arg(file_path)
                .spawn() 
            {
                Ok(_) => return Ok(()),
                Err(e) => {
                    eprintln!("Error launching {}: {}", editor, e);
                    let error = FileFantasticError::EditorLaunchFailed(editor.to_string());
                    println!("Falling back to system default due to: {}", error);
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    return open_file(file_path);
                }
            }
        } else {
            // Open terminal-based editors in new terminal window
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open")
                    .args(["-a", "Terminal"])
                    .arg(format!("{}; exit", editor))
                    .spawn()
                    .map_err(|e| {
                        eprintln!("Failed to open Terminal.app for editor: {}", e);
                        FileFantasticError::EditorLaunchFailed(editor.to_string())
                    })?;
            }
            #[cfg(target_os = "linux")]
            {
                // Try different terminal emulators
                let terminal_commands = [
                    ("gnome-terminal", vec!["--", editor]),
                    ("ptyxis", vec!["--", editor]),              // Fedora 41's default
                    ("konsole", vec!["--e", editor]),
                    ("xfce4-terminal", vec!["--command", editor]),
                    ("terminator", vec!["-e", editor]),
                    ("tilix", vec!["-e", editor]),
                    ("kitty", vec!["-e", editor]),
                    ("alacritty", vec!["-e", editor]),
                    ("terminology", vec!["-e", editor]),
                    ("xterm", vec!["-e", editor]),
                ];

                let mut success = false;
                for (terminal, args) in terminal_commands.iter() {
                    let mut cmd = std::process::Command::new(terminal);
                    cmd.args(args).arg(file_path);
                    
                    if let Ok(_) = cmd.spawn() {
                        success = true;
                        break;
                    }
                }

                if !success {
                    println!("No terminal available. Falling back to system default...");
                    let error = FileFantasticError::EditorLaunchFailed(editor.to_string());
                    eprintln!("Error: {}", error);
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    return open_file(file_path);
                }
            }
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("cmd")
                    .args(["/C", "start", "cmd", "/C"])
                    .arg(format!("{} {} && pause", editor, file_path.to_string_lossy()))
                    .spawn()
                    .map_err(|e| {
                        eprintln!("Failed to open cmd.exe for editor: {}", e);
                        FileFantasticError::EditorLaunchFailed(editor.to_string())
                    })?;
            }
        }
    }
    
    Ok(())
}

/// Handles opening a file with optional editor selection
/// 
/// # Arguments
/// * `path` - PathBuf of the file to open
/// 
/// # Returns
/// * `Result<()>` - Success or FileFantasticError with context
/// 
/// # Behavior
/// - Calls open_file() to handle the file opening process
/// - Provides user feedback on success or failure
/// - Waits for user confirmation before continuing
/// 
/// # Error Handling
/// - Catches and displays errors from open_file()
/// - Handles input/output errors during user prompts
/// - Ensures user is informed of all outcomes
fn handle_file_open(path: &PathBuf) -> Result<()> {
    match open_file(path) {
        Ok(_) => {
            println!("Opening file... \n\nPress Enter to continue");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).map_err(|e| {
                eprintln!("Failed to read input: {}", e);
                FileFantasticError::Io(e)
            })?;
        }
        Err(e) => {
            println!("Error opening file: {}. \nPress Enter to continue", e);
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).map_err(|e| {
                eprintln!("Failed to read input: {}", e);
                FileFantasticError::Io(e)
            })?;
        }
    }
    Ok(())
}

/*
See: https://en.wikipedia.org/wiki/Levenshtein_distance

Use code like this to compare and check if the rust code returns
comparable results:

```python
!pip install python-Levenshtein
# or
!pip install distance

# Using python-Levenshtein
from Levenshtein import distance as lev_distance1

def test_levenshtein1():
    # Same test cases as in Rust
    test_cases = [
        ("kitten", "sitting"),
        ("Saturday", "Sunday"),
        ("hello", "world"),
        ("rust", "dust"),
        ("", "test"),
        ("test", "")
    ]

    for s, t in test_cases:
        distance = lev_distance1(s, t)
        print(f"Distance between '{s}' and '{t}' is {distance}")

print( test_levenshtein1() )

# Alternative using 'distance' package:
from distance import levenshtein as lev_distance2

def test_levenshtein2():
    # Same test cases as in Rust
    test_cases = [
        ("kitten", "sitting"),
        ("Saturday", "Sunday"),
        ("hello", "world"),
        ("rust", "dust"),
        ("", "test"),
        ("test", "")
    ]

    for s, t in test_cases:
        distance = lev_distance2(s, t)
        print(f"Distance between '{s}' and '{t}' is {distance}")


print( test_levenshtein2() )
```

e.g.
Distance between 'kitten' and 'sitting' is 3
Distance between 'Saturday' and 'Sunday' is 3
Distance between 'hello' and 'world' is 4
Distance between 'rust' and 'dust' is 1
Distance between '' and 'test' is 4
Distance between 'test' and '' is 4
*/

/// Vanilla home made pair compair levenshtein_distance
/// e.g. for simple fuzzy search
/// Calculates the Levenshtein distance between two strings
/// 
/// # Purpose
/// Provides fuzzy text matching capability for the search functionality,
/// measuring how many single-character edits (insertions, deletions, substitutions)
/// are needed to transform one string into another.
/// 
/// # Arguments
/// * `s` - First string for comparison
/// * `t` - Second string for comparison
/// 
/// # Returns
/// * `usize` - The edit distance between the strings (lower = more similar)
/// 
/// # Algorithm
/// Uses a dynamic programming approach with two work vectors to calculate
/// the minimum edit distance between strings:
/// - 0 means strings are identical
/// - Higher values indicate greater differences
/// - Equal to max(s.len(), t.len()) when strings share no characters
/// 
/// # Performance Considerations
/// - O(m*n) time complexity where m and n are string lengths
/// - O(n) space complexity using the two-vector approach
/// - Efficient for short strings like filenames, but may not scale well
///   for very long strings
/// 
/// # Usage Context
/// Used in the `fuzzy_search` method to find files matching a partial query,
/// allowing for approximate/inexact matches when users don't know the exact filename.
/// 
/// # Examples
/// ```
/// assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
/// assert_eq!(levenshtein_distance("rust", "dust"), 1);
/// assert_eq!(levenshtein_distance("", "test"), 4);
/// ```
fn levenshtein_distance(s: &str, t: &str) -> usize {
    // Get the lengths of both strings
    let m = s.len();
    let n = t.len();

    // Handle empty string cases
    if m == 0 { return n; }
    if n == 0 { return m; }

    // Create two work vectors
    let mut v0: Vec<usize> = (0..=n).collect();
    let mut v1: Vec<usize> = vec![0; n + 1];

    // Convert strings to vectors of chars for easier indexing
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // Iterate through each character of s
    for i in 0..m {
        // First element of v1 is the deletion cost
        v1[0] = i + 1;

        // Calculate costs for each character of t
        for j in 0..n {
            let deletion_cost = v0[j + 1] + 1;
            let insertion_cost = v1[j] + 1;
            let substitution_cost = v0[j] + if s_chars[i] == t_chars[j] { 0 } else { 1 };

            v1[j + 1] = deletion_cost
                .min(insertion_cost)
                .min(substitution_cost);
        }

        // Swap vectors for next iteration
        v0.clone_from_slice(&v1);
    }

    // Return final distance
    v0[n]
}

/// Determines the starting directory path from command line arguments
/// 
/// # Returns
/// * `Result<PathBuf>` - The absolute path to start in or error with context
///
/// # Behavior
/// - If a valid path is provided as first argument, uses that
/// - If a file path is provided, uses its parent directory
/// - If path doesn't exist or no args provided, uses current directory
/// - Converts all paths to absolute paths for clarity
/// 
/// # Error Handling
/// - Validates path existence and type
/// - Provides clear error messages for invalid paths
/// - Falls back to current directory when appropriate
/// - Handles failures to determine current or parent directories
fn get_starting_path_from_args_or_cwd_default() -> Result<PathBuf> {
    // Get command line arguments
    let args: Vec<String> = std::env::args().skip(1).collect();
    
    if args.is_empty() {
        // No arguments provided, use current directory
        return std::env::current_dir().map_err(|e| {
            eprintln!("Failed to get current directory: {}", e);
            FileFantasticError::Io(e)
        });
    }
    
    // Use first argument as path
    let path_arg = PathBuf::from(&args[0]);
    
    // Convert to absolute path if possible
    let absolute_path = if path_arg.is_relative() {
        // Join with current directory to make absolute
        match std::env::current_dir() {
            Ok(current_dir) => current_dir.join(&path_arg),
            Err(e) => {
                eprintln!("Failed to get current directory: {}", e);
                return Err(FileFantasticError::Io(e));
            }
        }
    } else {
        path_arg
    };
    
    if absolute_path.exists() {
        if absolute_path.is_dir() {
            // Path is a directory, use it directly
            Ok(absolute_path)
        } else {
            // Path is a file, use its parent directory
            match absolute_path.parent() {
                Some(parent) => {
                    // Print notice about using parent directory
                    println!("Note: Using parent directory of file: {}", absolute_path.display());
                    println!("Directory: {}", parent.display());
                    println!("Press Enter to continue...");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).map_err(|e| {
                        eprintln!("Failed to read input: {}", e);
                        FileFantasticError::Io(e)
                    })?;
                    
                    Ok(PathBuf::from(parent))
                },
                None => {
                    // This should rarely happen (e.g., with root files on Windows)
                    eprintln!("Cannot determine parent directory of '{}'", absolute_path.display());
                    Err(FileFantasticError::InvalidName(absolute_path.display().to_string()))
                }
            }
        }
    } else {
        // Path doesn't exist, notify user and fall back to current directory
        eprintln!("Warning: Path '{}' does not exist. Starting in current directory.", 
                 absolute_path.display());
        std::env::current_dir().map_err(|e| {
            eprintln!("Failed to get current directory: {}", e);
            FileFantasticError::Io(e)
        })
    }
}

/// Checks if input is a "previous page" command
/// Supports multiple key options: j, <, [
fn is_pagination_up_command(input: &str) -> bool {
    matches!(input, "w" | "j" | "<" | "[" | "up" | "prev" | "," | "+" | "\x1b[A")
}

/// Checks if input is a "next page" command  
/// Supports multiple key options: k, >, ]
fn is_pagination_down_command(input: &str) -> bool {
    matches!(input, "x" | "k" | ">" | "]" | "down" | "next" | "." | "-" |"\x1b[B")
}

/// Public entry point for File Fantastic file manager module
/// 
/// # Usage as a Module
/// This function is designed to be imported and called from a main program:
/// ```rust
/// // src/main.rs
/// mod ff_file_fantastic_module;
/// use ff_file_fantastic_module::file_fantastic;
/// 
/// fn main() {
///     if let Err(e) = file_fantastic() {
///         eprintln!("Error: {}", e);
///         std::process::exit(1);
///     }
/// }
/// ```
///
/// # Function Operation
/// 1. Determines starting directory:
///    - Uses first command-line argument if valid path
///    - Falls back to current working directory
///    - For file paths, uses parent directory
///
/// 2. Handles all user interaction:
///    - Displays directory contents with numbered entries
///    - Processes keyboard commands for navigation/operations
///    - Manages file viewing and directory traversal
///    - Continues until user quits with 'q' command
///
/// # Returns
/// * `Result<()>` - Ok on clean exit, Err with context for critical failures
///
/// # Error Cases
/// - Returns errors only for critical failures that prevent operation
/// - Handles recoverable errors internally (permission issues, etc.)
/// - Callers should display the error and exit with non-zero code
///
/// # Dependencies
/// - Requires read access to file system
/// - Uses standard terminal for display
/// - Launches external programs for file viewing and terminal operations
///
/// # Performance Considerations
/// - Minimal memory usage with on-demand directory reading
/// - No background threads or async operations
/// - Suitable for use in resource-constrained environments
/// Public entry point for File Fantastic file manager module
pub fn file_fantastic() -> Result<()> {
    // Get starting directory from args or default to current directory
    let mut current_directory_path = match get_starting_path_from_args_or_cwd_default() {
        Ok(path) => path,
        Err(e) => {
            // Critical failure - unable to determine any starting directory
            eprintln!("Unable to determine starting directory: {}", e);
            eprintln!("This may be due to permission issues or missing directories.");
            return Err(e);
        }
    };
    
    // Display startup information for transparency
    println!("Using directory: {}", current_directory_path.display());
    
    let mut nav_state = NavigationState::new();
    let mut state_manager = NavigationStateManager::new(); // Initialize here at the top

    loop {
        // Read directory contents with proper error handling
        let mut all_entries = match read_directory_contents(&current_directory_path) {
            Ok(entries) => entries,
            Err(e) => {
                match e {
                    FileFantasticError::PermissionDenied(_) => {
                        eprintln!("Permission denied: Cannot read directory {}", 
                                 current_directory_path.display());
                        println!("Press Enter to go back to previous directory or 'q' to quit...");
                        
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).map_err(|e| {
                            eprintln!("Failed to read input: {}", e);
                            FileFantasticError::Io(e)
                        })?;
                        
                        if input.trim().eq_ignore_ascii_case("q") {
                            return Ok(());
                        }
                        
                        // Try to go up one directory
                        match current_directory_path.parent() {
                            Some(parent) => {
                                current_directory_path = parent.to_path_buf();
                                continue;
                            },
                            None => {
                                eprintln!("Cannot navigate further up. Exiting.");
                                return Ok(());
                            }
                        }
                    },
                    FileFantasticError::NotFound(_) => {
                        eprintln!("Directory not found: {}", current_directory_path.display());
                        
                        // Try to go up one directory
                        match current_directory_path.parent() {
                            Some(parent) => {
                                current_directory_path = parent.to_path_buf();
                                continue;
                            },
                            None => {
                                // Last resort: use current working directory
                                eprintln!("Falling back to current working directory");
                                match std::env::current_dir() {
                                    Ok(cwd) => {
                                        current_directory_path = cwd;
                                        continue;
                                    },
                                    Err(io_err) => {
                                        eprintln!("Cannot determine current directory: {}", io_err);
                                        return Err(FileFantasticError::Io(io_err));
                                    }
                                }
                            }
                        }
                    },
                    // Other errors are critical
                    _ => {
                        eprintln!("Error reading directory: {}", e);
                        return Err(e);
                    }
                }
            }
        };

        // Sort entries according to current sort method
        sort_directory_entries(&mut all_entries, nav_state.current_sort_method);
        
        // Apply the current filter to get filtered entries
        let filtered_entries = nav_state.apply_filter(&all_entries);
        
        // Convert from Vec<&FileSystemEntry> to Vec<FileSystemEntry> for pagination
        let directory_entries: Vec<FileSystemEntry> = filtered_entries.iter()
            .map(|&entry| FileSystemEntry {
                file_system_item_name: entry.file_system_item_name.clone(),
                file_system_item_path: entry.file_system_item_path.clone(),
                file_system_item_size_in_bytes: entry.file_system_item_size_in_bytes,
                file_system_item_last_modified_time: entry.file_system_item_last_modified_time,
                is_directory: entry.is_directory,
            })
            .collect();

        // Create paginated view starting at current page from navigation state
        let mut dir_view = DirectoryView::new(&directory_entries);
        dir_view.set_current_page(nav_state.current_page_index);

        // Inner loop for pagination within the same directory
        loop {
            // Get current page entries
            let page_entries = dir_view.current_page_entries();
            nav_state.update_lookup_table(page_entries);

            // Display with pagination info and filter status
            match display_directory_contents(
                page_entries, 
                &current_directory_path,
                Some((dir_view.current_page + 1, dir_view.total_pages())),
                nav_state.current_filter,
            ) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error displaying directory contents: {}", e);
                    eprintln!("Press Enter to try again or 'q' to quit...");
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).map_err(|e| {
                        eprintln!("Failed to read input: {}", e);
                        FileFantasticError::Io(e)
                    })?;
                    
                    if input.trim().eq_ignore_ascii_case("q") {
                        return Ok(());
                    }
                    continue;
                }
            }

            print!("\n>> ");
            match io::stdout().flush() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Failed to flush stdout: {}", e);
                    // Non-critical error, continue
                }
            }
            
            let mut user_input = String::new();
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    continue; // Try again
                }
            }
            
            // Trim the input for processing
            let trimmed_input = user_input.trim();
            
            // // Handle pagination commands first
            // if trimmed_input == "x" {
            //     if dir_view.next_page() {
            //         // Sync NavigationState with DirectoryView after successful page change
            //         nav_state.current_page_index = dir_view.get_current_page();
            //     }
            //     continue; // Stay in inner loop, just change page
            // } else if trimmed_input == "w" {
            //     if dir_view.prev_page() {
            //         // Sync NavigationState with DirectoryView after successful page change
            //         nav_state.current_page_index = dir_view.get_current_page();
            //     }
            //     continue; // Stay in inner loop, just change page
            // }
                        
            // Handle pagination commands with multiple key options
            if is_pagination_up_command(trimmed_input) {
                if dir_view.prev_page() {
                    // Sync NavigationState with DirectoryView after successful page change
                    nav_state.current_page_index = dir_view.get_current_page();
                }
                continue; // Stay in inner loop, just change page
            } else if is_pagination_down_command(trimmed_input) {
                if dir_view.next_page() {
                    // Sync NavigationState with DirectoryView after successful page change
                    nav_state.current_page_index = dir_view.get_current_page();
                }
                continue; // Stay in inner loop, just change page
            }
            
            // Handle number input directly to account for pagination
            if let Ok(number) = trimmed_input.parse::<usize>() {
                if let Some(actual_index) = dir_view.get_actual_index(number) {
                    // Only process if within range of full directory listing
                    if actual_index < directory_entries.len() {
                        let entry = &directory_entries[actual_index];
                        // Update selected item in nav_state
                        nav_state.set_selected_item(Some(number));
                        
                        if entry.is_directory {
                            current_directory_path = entry.file_system_item_path.clone();
                            break; // Break inner loop to read new directory
                        } else {
                            match handle_file_open(&entry.file_system_item_path) {
                                Ok(_) => {},
                                Err(e) => {
                                    eprintln!("Error opening file: {}", e);
                                    println!("Press Enter to continue...");
                                    let _ = io::stdin().read_line(&mut String::new());
                                }
                            }
                            continue; // Stay in inner loop
                        }
                    }
                }
            }

            // For other commands, use normal processing
            match process_user_input(
                &user_input, 
                &nav_state, 
                &all_entries,
            ) {
                Ok(action) => {
                    match action {
                        NavigationAction::Refresh => {
                            // Clear any filters when refreshing
                            nav_state.current_filter = None;
                            break; // Break inner loop to refresh directory
                        },
                        // NavigationAction::Filter(filter_char) => {
                        //     nav_state.set_filter(filter_char);
                        //     break; // Break inner loop to apply filter
                        // },
                        NavigationAction::Filter(filter_char) => {
                            nav_state.set_filter(filter_char);
                            nav_state.current_page_index = 0; // Reset to first page after filter change
                            break; // Break inner loop to apply filter
                        },
                        NavigationAction::Sort(command) => {
                            nav_state.toggle_sort(command);
                            nav_state.current_page_index = 0; // Reset to first page after sort change
                            break; // Break inner loop to resort directory
                        }
                        NavigationAction::ChangeDirectory(new_path) => {
                            current_directory_path = new_path;
                            nav_state.current_page_index = 0; // Reset to first page in new directory
                            break; // Break inner loop to read new directory
                        }
                        NavigationAction::ParentDirectory => {
                            match current_directory_path.parent() {
                                Some(parent) => {
                                    current_directory_path = parent.to_path_buf();
                                    nav_state.current_page_index = 0; // Reset to first page
                                    break; // Break inner loop to read new directory
                                },
                                None => {
                                    println!("Already at root directory");
                                }
                            }
                        }
                        // NavigationAction::ChangeDirectory(new_path) => {
                        //     current_directory_path = new_path;
                        //     break; // Break inner loop to read new directory
                        // }
                        // NavigationAction::ParentDirectory => {
                        //     match current_directory_path.parent() {
                        //         Some(parent) => {
                        //             current_directory_path = parent.to_path_buf();
                        //             break; // Break inner loop to read new directory
                        //         },
                        //         None => {
                        //             println!("Already at root directory");
                        //             // Stay in current directory
                        //         }
                        //     }
                        // }
                        NavigationAction::OpenFile(ref path) => {
                            match handle_file_open(path) {
                                Ok(_) => {},
                                Err(e) => {
                                    eprintln!("Error opening file: {}", e);
                                    println!("Press Enter to continue...");
                                    let _ = io::stdin().read_line(&mut String::new());
                                }
                            }
                        }
                        NavigationAction::Quit => return Ok(()),
                        // NavigationAction::Sort(command) => {
                        //     nav_state.toggle_sort(command);
                        //     break; // Break inner loop to resort directory
                        // }
                        NavigationAction::OpenNewTerminal => {
                            match open_new_terminal(&current_directory_path) {
                                Ok(_) => {
                                    println!("Opening new terminal... Press Enter to continue");
                                },
                                Err(e) => {
                                    println!("Error opening new terminal: {}. Press Enter to continue", e);
                                }
                            }
                            let _ = io::stdin().read_line(&mut String::new());
                        },
                        // NEW: Handle Get-Send-Mode
                        NavigationAction::GetSendMode => {

                            // Enter Get-Send-Mode loop
                            loop {
                                match state_manager.interactive_get_send_mode()? {
                                    GetSendModeAction::AddFileToStack => {
                                        // Get currently selected file if any
                                        let selected_file_path = nav_state.get_selected_item_path();
                                        
                                        match state_manager.interactive_add_file_to_stack(
                                            &nav_state, 
                                            selected_file_path.as_ref(),
                                            page_entries, // Pass current page entries for display
                                            &current_directory_path, // Pass current directory path
                                        ) {
                                            Ok(_) => println!("File stack operation completed."),
                                            Err(e) => println!("Error: {}", e),
                                        }
                                    },
                                    GetSendModeAction::GetFileFromStack => {
                                        match state_manager.interactive_get_file_from_stack() {
                                            Ok(Some(source_file_path)) => {
                                                println!("Retrieved file: {}", source_file_path.display());
                                                println!("Copying to current directory: {}", current_directory_path.display());
                                                
                                                // Copy the file to current directory with archive handling
                                                match copy_file_with_archive_handling(&source_file_path, &current_directory_path) {
                                                    Ok(final_destination_path) => {
                                                        println!("✓ Copy operation completed successfully!");
                                                        println!("Final location: {}", final_destination_path.display());
                                                    },
                                                    Err(e) => {
                                                        eprintln!("✗ Copy operation failed: {}", e);
                                                        println!("The file remains in the stack for retry if needed.");
                                                        
                                                        // Re-add the file to the stack since copy failed
                                                        if let Err(re_add_error) = state_manager.add_file_to_stack(source_file_path) {
                                                            eprintln!("Warning: Could not re-add file to stack: {}", re_add_error);
                                                        }
                                                    }
                                                }
                                                
                                                println!("Press Enter to continue...");
                                                let _ = io::stdin().read_line(&mut String::new());
                                            },
                                            Ok(None) => println!("No file selected."),
                                            Err(e) => println!("Error getting file from stack: {}", e),
                                        }
                                    },
                                    GetSendModeAction::AddDirectoryToStack => {
                                        match state_manager.interactive_save_directory_to_stack(&current_directory_path) {
                                            Ok(_) => println!("Directory added to stack successfully."),
                                            Err(e) => println!("Error adding directory to stack: {}", e),
                                        }
                                    },
                                    GetSendModeAction::SavePocketDimension => {
                                        print!("Enter nickname for this location (or Enter for auto): ");
                                        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
                                        let mut nickname = String::new();
                                        io::stdin().read_line(&mut nickname).map_err(|e| FileFantasticError::Io(e))?;
                                        let nickname = if nickname.trim().is_empty() { None } else { Some(nickname.trim().to_string()) };
                                        
                                        match state_manager.save_pocket_dimension(
                                            current_directory_path.clone(),
                                            &nav_state,
                                            &dir_view,
                                            nav_state.selected_item_index,
                                            nav_state.active_search_term.clone(),
                                            nickname,
                                        ) {
                                            Ok(saved_name) => println!("Saved as pocket dimension: {}", saved_name),
                                            Err(e) => println!("Error saving pocket dimension: {}", e),
                                        }
                                    },
                                    GetSendModeAction::GoToPocketDimension => {
                                        match state_manager.interactive_select_pocket_dimension() {
                                            Ok(Some(nickname)) => {
                                                match state_manager.restore_pocket_dimension(&nickname) {
                                                    Ok(saved_state) => {
                                                        // Restore the complete navigation state
                                                        current_directory_path = saved_state.current_directory_path;
                                                        nav_state.current_sort_method = saved_state.current_sort_method;
                                                        nav_state.current_filter = saved_state.current_filter;
                                                        nav_state.selected_item_index = saved_state.selected_item_index;
                                                        nav_state.active_search_term = saved_state.active_search_term;
                                                        nav_state.terminal_size = saved_state.terminal_size;
                                                        nav_state.current_page_index = saved_state.current_page_number; // Restore page
                                                        
                                                        println!("Jumped to pocket dimension: {} (page {})", 
                                                                nickname, saved_state.current_page_number + 1);
                                                        break; // Exit Get-Send-Mode and refresh directory
                                                    },
                                                    Err(e) => println!("Error restoring pocket dimension: {}", e),
                                                }
                                            },
                                            Ok(None) => println!("No pocket dimension selected."),
                                            Err(e) => println!("Error selecting pocket dimension: {}", e),
                                        }
                                    },
                                    GetSendModeAction::ViewStacks => {
                                        println!("\n=== Current Status ===");
                                        println!("{}", state_manager.get_stack_summary());
                                        
                                        if !state_manager.file_path_stack.is_empty() {
                                            println!("\nFile Stack:");
                                            for (i, file) in state_manager.file_path_stack.iter().enumerate() {
                                                println!("  {}. {}", i + 1, file.display());
                                            }
                                        }
                                        
                                        if !state_manager.directory_path_stack.is_empty() {
                                            println!("\nDirectory Stack:");
                                            for (i, dir) in state_manager.directory_path_stack.iter().enumerate() {
                                                println!("  {}. {}", i + 1, dir.display());
                                            }
                                        }
                                        
                                        if !state_manager.pocket_dimensions.is_empty() {
                                            println!("\nPocket Dimensions:");
                                            let dimensions = state_manager.list_pocket_dimensions();
                                            for (i, (nickname, state)) in dimensions.iter().enumerate() {
                                                println!("  {}. {} - {}", i + 1, nickname, state.description);
                                            }
                                        }
                                        
                                        println!("\nPress Enter to continue...");
                                        let _ = io::stdin().read_line(&mut String::new());
                                    },
                                    GetSendModeAction::ArchiveSelection => {
                                        match state_manager.interactive_archive_selection(
                                            &nav_state,
                                            page_entries,
                                            &current_directory_path,
                                        ) {
                                            Ok(_) => println!("Archive operation completed."),
                                            Err(e) => println!("Error during archive operation: {}", e),
                                        }
                                    },
                                    GetSendModeAction::ClearAll => {
                                        print!("Clear all stacks and pocket dimensions? (y/N): ");
                                        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;
                                        let mut response = String::new();
                                        io::stdin().read_line(&mut response).map_err(|e| FileFantasticError::Io(e))?;
                                        
                                        if response.trim().eq_ignore_ascii_case("y") {
                                            state_manager.clear_all();
                                            println!("All stacks and pocket dimensions cleared.");
                                        }
                                    },
                                    GetSendModeAction::ReturnToBrowser => break,
                                }
                            }
                            // After exiting Get-Send-Mode, break to refresh directory
                            break;
                        },
                        NavigationAction::Invalid => {
                            println!("Invalid input. Press Enter to continue...");
                            let _ = io::stdin().read_line(&mut String::new());
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error processing input: {}", e);
                    println!("Press Enter to continue...");
                    let _ = io::stdin().read_line(&mut String::new());
                }
            }
        }
    }
}