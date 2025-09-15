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
1. minimal text user interface (TUI)
- path, then numbered lines
- like bash: ls, but list by number
- shown as columns: item-number name size modified
2. primarily int + enter/return for user-input
3. select directory to go to by number
4. 'b' to go back; back-up directory path, go to parent directory
5. enter file to open by number; use Q&A to use editor of choice
6. default to default program with another return/enter
7. open file in new terminal
8. hit enter to refresh
11. single letter commands
12. legend shows command 'words': use first letter as command
```
quit back|term|dir file|name size mod|get-send file v,y,p|str>search|enter>reset
```
13. Pagination: up/down, j/k, </>, w/x, arrows, etc.
14. 'sort by size' ' 'sort by name' 'sort by last-modified': re-selecting a sort option reverses the order
15. Type a string for a partial match search.
16. 'f' or 'd' to show only files or only directories
17. Minimal file-moving ("Get-Send Mode") in get-send file v,y,p
18. Save and Change Navigation-State ("Pocket Dimensions") in get-send file v,y,p
19. Archive Selection: basic archive feature for versioning and backup
20. Use your own programs with ff to work on files (such as 'lines' or 'rows_and_columns' for .csv files, or a hex-editor such as 'tofu')
21. User can resize TUI: tall+/-N or wide+/-N (e.g. tall+2 or wide-4)
22. modular to easy integration into other projects

## Scrolling
Instructions appear in info-bar:
- Use various standard keys (and hit enter after that key)
```text
up/down, j/k, </>, w/x, arrows, etc.
```

## Get-Send Mode: Move Files
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
- 'tall+N' or 'tall-N' or 'wide+N' or 'wide-N' will change TUI, not cumulative


General Pattern for Platform-Specific Code
When adding new platforms to conditional compilation, always remember to update all relevant cfg attributes:
// If you have:
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "android"))]
{ /* implementation A */ }

#[cfg(target_os = "windows")]
{ /* implementation B */ }

// Then the fallback must exclude ALL handled platforms:
#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "windows",
    target_os = "android"  // Don't forget to add here too!
)))]
{ /* fallback implementation */ }
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

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::env;
use std::process::Command;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::sync::OnceLock;

/// Maximum Levenshtein distance to consider a match
const MAX_SEARCH_DISTANCE: usize = 2;

/// Default baseline values for TUI display calculations
///
/// These constants define the starting points for display dimensions before
/// user adjustments are applied. Using u8 for memory efficiency since these
/// values are all well within u8 range (0-255).
const MAX_NAME_LENGTH_DEFAULT: usize = 55;

const MAX_TUI_CHAR_LENGTH_DEFAULT: usize = 80;

/// 16 allows an extra line for cases where there cwd/PWD path
/// is long enough to wrap around, a safer default
const ITEMS_PER_PAGE_DEFAULT: u8 = 16;
const FILENAME_SUFFIX_LENGTH: usize = 5;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
// const BLUE: &str = "\x1b[34m";
// const BOLD: &str = "\x1b[1m";
// const ITALIC: &str = "\x1b[3m";
// const UNDERLINE: &str = "\x1b[4m";

/// analyze rows and colums of data file
use super::rows_and_columns_module::{
    rc_analyze_datafile_save_results_to_resultsfile
};

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

/// Calculates the actual name column width based on user adjustments
///
/// # Purpose
/// Determines how many characters wide the name column should be by applying
/// the user's adjustment to the default baseline. This width controls how much
/// of each filename is visible before truncation.
///
/// # Arguments
/// * `adjustment_magnitude` - The positive number of characters to add or remove
/// * `adjustment_direction_true_is_positive_false_is_negative` - Direction of adjustment:
///   - `true` = Add characters (wider column)
///   - `false` = Remove characters (narrower column)
///
/// # Returns
/// * `u16` - The calculated column width in characters
///
/// # Calculation Logic
/// 1. Start with MAX_NAME_LENGTH_DEFAULT (55 characters)
/// 2. Apply adjustment in the specified direction
/// 3. Use saturating arithmetic to prevent overflow
/// 4. Enforce minimum width of 8 characters (FILENAME_SUFFIX_LENGTH + 3 for "...")
///
/// # Minimum Width Rationale
/// The minimum of 8 characters ensures we can always display:
/// - 3 characters for ellipsis "..."
/// - 5 characters for file suffix (e.g., ".docx")
/// This prevents display corruption with very narrow columns.
///
/// # Examples
/// ```rust
/// // Default width (no adjustment)
/// assert_eq!(calculate_name_width(0, true), 55);
///
/// // Increase width by 10
/// assert_eq!(calculate_name_width(10, true), 65);
///
/// // Decrease width by 50 (hits minimum)
/// assert_eq!(calculate_name_width(50, false), 8);
///
/// // Maximum possible width
/// assert_eq!(calculate_name_width(u16::MAX, true), u16::MAX);
/// ```
fn calculate_name_width(
    adjustment_magnitude: u16,
    adjustment_direction_true_is_positive_false_is_negative: bool
) -> u16 {
    // Convert default to u16 for calculation
    let base_width: u16 = MAX_NAME_LENGTH_DEFAULT as u16;

    // Calculate minimum allowed width (suffix + ellipsis)
    let minimum_width: u16 = (FILENAME_SUFFIX_LENGTH + 3) as u16;

    // Apply adjustment based on direction
    if adjustment_direction_true_is_positive_false_is_negative {
        // Positive adjustment: add to base width
        // saturating_add prevents overflow, returns u16::MAX if result would overflow
        base_width.saturating_add(adjustment_magnitude)
    } else {
        // Negative adjustment: subtract from base width
        // saturating_sub prevents underflow, returns 0 if result would be negative
        // max ensures we never go below minimum_width
        base_width.saturating_sub(adjustment_magnitude).max(minimum_width)
    }
}

/// Calculates the actual number of items to display per page based on user adjustments
///
/// # Purpose
/// Determines how many directory entries should be shown on each page by applying
/// the user's height adjustment to the default baseline. This controls vertical
/// pagination behavior.
///
/// # Arguments
/// * `adjustment_magnitude` - The positive number of rows to add or remove
/// * `adjustment_direction_true_is_positive_false_is_negative` - Direction of adjustment:
///   - `true` = Add rows (show more items)
///   - `false` = Remove rows (show fewer items)
///
/// # Returns
/// * `u16` - The calculated number of items per page
///
/// # Calculation Logic
/// 1. Start with ITEMS_PER_PAGE_DEFAULT (16 items)
/// 2. Apply adjustment in the specified direction
/// 3. Use saturating arithmetic to prevent overflow/underflow
/// 4. Allow result to go to 0 (displays header only, no items)
///
/// # Zero Items Behavior
/// When items per page is 0, the display shows:
/// - Current directory path
/// - Column headers
/// - No file/directory entries
/// - Navigation commands still work
///
/// # Examples
/// ```rust
/// // Default height (no adjustment)
/// assert_eq!(calculate_items_per_page(0, true), 16);
///
/// // Increase by 10 rows
/// assert_eq!(calculate_items_per_page(10, true), 26);
///
/// // Decrease by 20 rows (goes to 0)
/// assert_eq!(calculate_items_per_page(20, false), 0);
///
/// // Maximum possible items
/// assert_eq!(calculate_items_per_page(u16::MAX, true), u16::MAX);
/// ```
fn calculate_items_per_page(
    adjustment_magnitude: u16,
    adjustment_direction_true_is_positive_false_is_negative: bool
) -> u16 {
    // Convert default to u16 for calculation
    let base_items: u16 = ITEMS_PER_PAGE_DEFAULT as u16;

    // Apply adjustment based on direction
    if adjustment_direction_true_is_positive_false_is_negative {
        // Positive adjustment: add to base count
        // saturating_add prevents overflow, returns u16::MAX if result would overflow
        base_items.saturating_add(adjustment_magnitude)
    } else {
        // Negative adjustment: subtract from base count
        // saturating_sub prevents underflow, returns 0 if result would be negative
        // No minimum enforced - 0 items per page is valid (header only display)
        base_items.saturating_sub(adjustment_magnitude)
    }
}

/// Calculates name width from NavigationState for convenience
///
/// # Purpose
/// Convenience function that extracts TUI width settings from NavigationState
/// and calculates the actual name column width. Reduces code duplication
/// throughout the display functions.
///
/// # Arguments
/// * `nav_state` - Reference to current navigation state containing TUI settings
///
/// # Returns
/// * `u16` - The calculated name column width in characters
///
/// # Usage
/// This function is typically called by display functions that need to know
/// the current name column width for formatting purposes.
///
/// # Example
/// ```rust
/// let width = calculate_name_width_from_state(&nav_state);
/// println!("{:>3}. {:<width$} {:>6} {:>11}",
///          index, name, size, modified, width = width);
/// ```
fn calculate_name_width_from_state(nav_state: &NavigationState) -> u16 {
    calculate_name_width(
        nav_state.tui_wide_adjustment,
        nav_state.tui_wide_direction_sign
    )
}

/// Calculates items per page from NavigationState for convenience
///
/// # Purpose
/// Convenience function that extracts TUI height settings from NavigationState
/// and calculates the actual items per page. Reduces code duplication
/// throughout the pagination logic.
///
/// # Arguments
/// * `nav_state` - Reference to current navigation state containing TUI settings
///
/// # Returns
/// * `u16` - The calculated number of items to display per page
///
/// # Usage
/// This function is typically called when creating or updating DirectoryView
/// instances to ensure pagination respects user preferences.
///
/// # Example
/// ```rust
/// let items_per_page = calculate_items_per_page_from_state(&nav_state);
/// let mut dir_view = DirectoryView::new(&entries, items_per_page);
/// ```
fn calculate_items_per_page_from_state(nav_state: &NavigationState) -> u16 {
    calculate_items_per_page(
        nav_state.tui_tall_adjustment,
        nav_state.tui_tall_direction_sign
    )
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

// /// Creates a zip file using platform-appropriate system commands
// ///
// /// # Purpose
// /// Executes platform-specific zip creation commands to compress directories,
// /// avoiding external dependencies while providing cross-platform functionality.
// ///
// /// # Arguments
// /// * `source_path` - Directory to compress
// /// * `zip_path` - Output path for the zip file
// ///
// /// # Returns
// /// * `Result<bool>` - True if zip creation succeeded, false if failed
// ///
// /// # Platform Implementation
// /// - **Linux/macOS**: Uses `zip -r` command for recursive compression
// /// - **Windows**: Uses PowerShell `Compress-Archive` cmdlet
// ///
// /// # Command Details
// /// ## Linux/macOS
// /// ```bash
// /// zip -r "output.zip" "source_directory/"
// /// ```
// ///
// /// ## Windows
// /// ```powershell
// /// Compress-Archive -Path "source_directory" -DestinationPath "output.zip"
// /// ```
// ///
// /// # Error Handling
// /// - Handles command execution failures
// /// - Checks exit status of zip commands
// /// - Provides platform-specific error context
// ///
// /// # Example
// /// ```rust
// /// let source = PathBuf::from("/home/user/documents");
// /// let zip_file = PathBuf::from("/home/user/documents_backup.zip");
// ///
// /// match create_zip_with_system_command(&source, &zip_file) {
// ///     Ok(true) => println!("Zip created successfully"),
// ///     Ok(false) => println!("Zip command failed"),
// ///     Err(e) => eprintln!("Error executing zip command: {}", e),
// /// }
// /// ```
// fn create_zip_with_system_command(
//     source_path: &PathBuf,
//     zip_path: &PathBuf,
// ) -> Result<bool> {
//     #[cfg(any(target_os = "linux", target_os = "macos", target_os = "android"))]
//     {
//         // Use zip command on Unix-like systems
//         let output = std::process::Command::new("zip")
//             .arg("-r")  // Recursive
//             .arg(zip_path)
//             .arg(source_path)
//             .output()
//             .map_err(|e| {
//                 eprintln!("Failed to execute zip command: {}", e);
//                 eprintln!("Make sure 'zip' is installed on your system");
//                 FileFantasticError::Io(e)
//             })?;

//         if output.status.success() {
//             Ok(true)
//         } else {
//             let error_msg = String::from_utf8_lossy(&output.stderr);
//             eprintln!("Zip command failed: {}", error_msg);
//             Ok(false)
//         }
//     }

//     #[cfg(target_os = "windows")]
//     {
//         // Use PowerShell Compress-Archive on Windows
//         let output = std::process::Command::new("powershell")
//             .arg("-Command")
//             .arg(format!(
//                 "Compress-Archive -Path '{}' -DestinationPath '{}'",
//                 source_path.display(),
//                 zip_path.display()
//             ))
//             .output()
//             .map_err(|e| {
//                 eprintln!("Failed to execute PowerShell compress command: {}", e);
//                 FileFantasticError::Io(e)
//             })?;

//         if output.status.success() {
//             Ok(true)
//         } else {
//             let error_msg = String::from_utf8_lossy(&output.stderr);
//             eprintln!("PowerShell compress command failed: {}", error_msg);
//             Ok(false)
//         }
//     }

//     #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
//     {
//         Err(FileFantasticError::UnsupportedPlatform)
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
/// - **Linux/macOS/Android**: Uses `zip -r` command for recursive compression
/// - **Windows**: Uses PowerShell `Compress-Archive` cmdlet
///
/// # Command Details
/// ## Linux/macOS/Android
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
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "android"))]
    {
        // Use zip command on Unix-like systems (including Android/Termux)
        let output = std::process::Command::new("zip")
            .arg("-r")  // Recursive
            .arg(zip_path)
            .arg(source_path)
            .output()
            .map_err(|e| {
                eprintln!("Failed to execute zip command: {}", e);
                eprintln!("Make sure 'zip' is installed on your system");
                // On Android/Termux, provide specific installation hint
                if cfg!(target_os = "android") {
                    eprintln!("On Termux, install zip with: pkg install zip");
                }
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

    // Fixed: Added target_os = "android" to the exclusion list
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows", target_os = "android")))]
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

    /// Which item was selected/highlighted (1-based index)
    /// None if no item was specifically selected
    pub selected_item_index: Option<usize>,

    /// The search term that was active, if any
    /// Preserves active search state for complete restoration
    pub active_search_term: Option<String>,

    /// Manual TUI height adjustment magnitude (rows to add/remove)
    /// Stored as positive value, direction determined by tui_tall_direction_sign
    /// Example: adjustment=3 with sign=false means "tall-3" (3 fewer rows)
    pub tui_tall_adjustment: u16,

    /// Direction of height adjustment
    /// true = positive adjustment (more rows), false = negative adjustment (fewer rows)
    /// Combined with tui_tall_adjustment to represent adjustments like "tall+5" or "tall-3"
    pub tui_tall_direction_sign: bool,

    /// Manual TUI width adjustment magnitude (characters to add/remove from name column)
    /// Stored as positive value, direction determined by tui_wide_direction_sign
    /// Example: adjustment=10 with sign=true means "wide+10" (10 more characters)
    pub tui_wide_adjustment: u16,

    /// Direction of width adjustment
    /// true = positive adjustment (wider names), false = negative adjustment (narrower names)
    /// Combined with tui_wide_adjustment to represent adjustments like "wide+20" or "wide-5"
    pub tui_wide_direction_sign: bool,

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
    /// This includes all navigation preferences and TUI display size customizations.
    ///
    /// # Arguments
    /// * `current_directory_path` - Current directory being viewed
    /// * `nav_state` - Current NavigationState with sort/filter settings and TUI adjustments
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
    /// 5. Captures TUI size adjustment settings
    /// 6. Generates nickname and description for user interface
    /// 7. Timestamps the state for management
    ///
    /// # TUI Settings Preservation
    /// The saved state includes the user's TUI size preferences, allowing
    /// different pocket dimensions to have different optimal display configurations.
    /// For example, a "code review" dimension might use wide+20 for long filenames,
    /// while a "log browsing" dimension might use tall+10 for more visible entries.
    ///
    /// # Nickname Generation
    /// If no nickname is provided, automatically generates one based on:
    /// - Directory name
    /// - Timestamp suffix for uniqueness
    /// - Truncation to reasonable length
    ///
    /// # Example
    /// ```rust
    /// // Save current state as a pocket dimension with custom display size
    /// let saved_state = SavedNavigationState::new(
    ///     current_directory_path.clone(),
    ///     &nav_state, // Contains TUI adjustments like wide+10 tall+5
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

        SavedNavigationState {
            current_directory_path,
            current_sort_method: nav_state.current_sort_method,
            current_filter: nav_state.current_filter,
            current_page_number: dir_view.current_page,
            selected_item_index: selected_item,
            active_search_term: active_search,
            // Capture the current TUI size adjustment settings
            tui_tall_adjustment: nav_state.tui_tall_adjustment,
            tui_tall_direction_sign: nav_state.tui_tall_direction_sign,
            tui_wide_adjustment: nav_state.tui_wide_adjustment,
            tui_wide_direction_sign: nav_state.tui_wide_direction_sign,
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

    /*pending use of saved directories*/
    // /// Adds a directory path to the directory stack after validation
    // ///
    // /// # Purpose
    // /// Safely adds a directory path to the directory collection stack after
    // /// verifying that the path exists and actually points to a directory.
    // ///
    // /// # Arguments
    // /// * `dir_path` - PathBuf pointing to the directory to add
    // ///
    // /// # Returns
    // /// * `Result<()>` - Success or error with validation details
    // ///
    // /// # Validation Process
    // /// 1. Checks that the path exists on the filesystem
    // /// 2. Verifies that the path points to a directory (not a file)
    // /// 3. Adds to stack only if both conditions are met
    // ///
    // /// # Error Conditions
    // /// - Path does not exist (NotFound)
    // /// - Path exists but is not a directory (InvalidName)
    // ///
    // /// # Stack Behavior
    // /// Directories are added to the end of the stack (LIFO - Last In, First Out).
    // /// This allows users to work with the most recently added directories first.
    // ///
    // /// # Usage Context
    // /// Used for collecting destination directories for operations like:
    // /// - Copy/move operations
    // /// - Quick navigation bookmarks
    // /// - Batch processing targets
    // ///
    // /// # Example
    // /// ```rust
    // /// let dir_path = PathBuf::from("/home/user/projects");
    // /// match state_manager.add_directory_to_stack(dir_path) {
    // ///     Ok(_) => println!("Directory added to stack"),
    // ///     Err(e) => println!("Failed to add directory: {}", e),
    // /// }
    // /// ```
    // pub fn add_directory_to_stack(&mut self, dir_path: PathBuf) -> Result<()> {
    //     // Validate that the path exists
    //     if !dir_path.exists() {
    //         return Err(FileFantasticError::NotFound(dir_path));
    //     }

    //     // Validate that the path is actually a directory
    //     if dir_path.is_dir() {
    //         self.directory_path_stack.push(dir_path);
    //         Ok(())
    //     } else {
    //         Err(FileFantasticError::InvalidName(
    //             "Path is not a directory".to_string()
    //         ))
    //     }
    // }

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
    /// - **Files**:
    ///   - Asks if user wants to zip the file
    ///   - Creates timestamped copy (with optional zip) in archive directory
    ///   - Supports optional name prefix for both zipped and non-zipped files
    /// - **Directories**:
    ///   - Creates timestamped zip archive in archive directory
    ///   - Supports optional name prefix
    ///
    /// # User Interface Workflow
    ///
    /// ## Step 1: Display and Selection
    /// - Show current directory contents with numbered items
    /// - User selects item by number (same interface as navigation)
    ///
    /// ## Step 2: Confirmation
    /// - Show selected item and ask for archive confirmation
    ///
    /// ## Step 3: Archive Options (based on item type)
    ///
    /// ### For Files:
    /// 1. Ask if user wants to zip the file
    /// 2. Ask for optional name prefix (applies to both zipped and non-zipped)
    /// 3. Create archive based on choices
    ///
    /// ### For Directories:
    /// 1. Ask for optional name prefix
    /// 2. Create zip archive with timestamp
    ///
    /// ## Step 4: Result Display
    /// - Show success/failure message
    /// - Display archive location
    pub fn interactive_archive_selection(
        &mut self,
        nav_state: &NavigationState,
        current_directory_entries: &[FileSystemEntry],
        current_directory_path: &PathBuf,
    ) -> Result<()> {

        // WORKFLOW STEP 1: Display directory contents for user selection
        display_directory_contents(
            current_directory_entries,
            current_directory_path,
            None,
            nav_state.current_filter,
            nav_state,
        ).map_err(|e| FileFantasticError::Io(e))?;

        // WORKFLOW STEP 1 (continued): Prompt for selection
        println!("\n=== Archive Selection ===");
        println!("Select item to archive (creates timestamped copy/zip)");
        print!("Enter item number (or 'b' to back/cancel): ");
        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| FileFantasticError::Io(e))?;
        let input = input.trim();

        // Handle cancellation request
        if input.eq_ignore_ascii_case("b") {
            println!("Back/Cancelled.");
            return Ok(());
        }

        // Validate and process the user's selection
        if let Ok(number) = input.parse::<usize>() {
            if let Some(item_info) = nav_state.lookup_item(number) {

                // Extract item name for display purposes
                let item_name = item_info.item_path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy();

                // WORKFLOW STEP 2: Ask for confirmation based on item type
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

                    // WORKFLOW STEP 3: Process archive options based on item type
                    if item_info.item_type == FileSystemItemType::Directory {
                        // DIRECTORY WORKFLOW: Ask for optional name prefix, then create zip

                        print!("\nAdd custom name prefix to archive? (optional, or Enter to skip): ");
                        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;

                        let mut custom_name = String::new();
                        io::stdin().read_line(&mut custom_name).map_err(|e| FileFantasticError::Io(e))?;
                        let custom_name = custom_name.trim();

                        let custom_name_option = if custom_name.is_empty() {
                            None
                        } else {
                            Some(custom_name)
                        };

                        // Archive directory using existing function
                        match ensure_archive_directory_exists(current_directory_path) {
                            Ok(archive_directory_path) => {
                                match create_directory_zip_archive(
                                    &item_info.item_path,
                                    &archive_directory_path,
                                    custom_name_option,
                                ) {
                                    Ok(zip_path) => {
                                        println!("\n✓ Directory archived successfully!");
                                        println!("Archive location: {}", zip_path.display());
                                    },
                                    Err(e) => {
                                        eprintln!("\n✗ Directory archive creation failed: {}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("\n✗ Failed to create archive directory: {}", e);
                            }
                        }
                    } else {
                        // FILE WORKFLOW: Ask about zip, then name prefix, then create archive

                        // Ask if user wants to zip the file
                        print!("\nWould you like to zip the archived file? (y/N): ");
                        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;

                        let mut zip_response = String::new();
                        io::stdin().read_line(&mut zip_response).map_err(|e| FileFantasticError::Io(e))?;
                        let should_zip = zip_response.trim().eq_ignore_ascii_case("y");

                        // Ask for optional custom name prefix
                        print!("\nAdd custom name prefix to archive? (optional, or Enter to skip): ");
                        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;

                        let mut custom_name = String::new();
                        io::stdin().read_line(&mut custom_name).map_err(|e| FileFantasticError::Io(e))?;
                        let custom_name = custom_name.trim();

                        let custom_name_option = if custom_name.is_empty() {
                            None
                        } else {
                            Some(custom_name)
                        };

                        // Ensure archive directory exists
                        match ensure_archive_directory_exists(current_directory_path) {
                            Ok(archive_directory_path) => {
                                if should_zip {
                                    // Create zipped archive of the file
                                    match self.create_file_zip_archive(
                                        &item_info.item_path,
                                        &archive_directory_path,
                                        custom_name_option,
                                    ) {
                                        Ok(zip_path) => {
                                            println!("\n✓ File archived and zipped successfully!");
                                            println!("Archive location: {}", zip_path.display());
                                        },
                                        Err(e) => {
                                            eprintln!("\n✗ File zip archive creation failed: {}", e);
                                        }
                                    }
                                } else {
                                    // Create timestamped copy of the file
                                    match self.copy_file_with_timestamp_and_prefix(
                                        &item_info.item_path,
                                        &archive_directory_path,
                                        custom_name_option,
                                    ) {
                                        Ok(archived_path) => {
                                            println!("\n✓ File archived successfully!");
                                            println!("Archive location: {}", archived_path.display());
                                        },
                                        Err(e) => {
                                            eprintln!("\n✗ File archive creation failed: {}", e);
                                        }
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("\n✗ Failed to create archive directory: {}", e);
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
            println!("Error: Please enter a valid number or 'b' to cancel.");
        }

        // Wait for user acknowledgment before returning
        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());

        Ok(())
    }

    /// Creates a zip archive of a single file with timestamp and optional prefix
    ///
    /// # Purpose
    /// Archives a single file into a zip-like archive with timestamp in the filename.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to archive
    /// * `archive_directory` - Directory where the archive will be created
    /// * `custom_prefix` - Optional prefix to add before the filename
    ///
    /// # Returns
    /// * `Result<PathBuf>` - Path to the created archive or error
    fn create_file_zip_archive(
        &self,
        file_path: &Path,
        archive_directory: &Path,
        custom_prefix: Option<&str>,
    ) -> Result<PathBuf> {
        // Validate that the source file exists
        if !file_path.exists() {
            return Err(FileFantasticError::NotFound(file_path.to_path_buf()));
        }

        // Validate that it's actually a file
        if !file_path.is_file() {
            return Err(FileFantasticError::InvalidName(
                format!("Path is not a file: {}", file_path.display())
            ));
        }

        // Get the file name without extension for the archive name
        let file_stem = file_path.file_stem()
            .ok_or_else(|| FileFantasticError::InvalidName(
                "Could not extract file name".to_string()
            ))?
            .to_string_lossy();

        // Generate full timestamp
        let timestamp = createarchive_timestamp_with_precision(
            SystemTime::now(),
            true,
            );

        // Build the archive name with optional prefix
        let archive_name = if let Some(prefix) = custom_prefix {
            format!("{}{}_{}.zip", prefix, file_stem, timestamp)
        } else {
            format!("{}_{}.zip", file_stem, timestamp)
        };

        // Create full path for the archive
        let archive_path = archive_directory.join(archive_name);

        // Create simple archive (since we can't use external zip libraries)
        self.create_simple_file_archive(file_path, &archive_path)?;

        Ok(archive_path)
    }

    /// Creates a timestamped copy of a file with optional prefix
    ///
    /// # Purpose
    /// Creates a copy of a file with timestamp and optional prefix in the filename.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to copy
    /// * `archive_directory` - Directory where the copy will be created
    /// * `custom_prefix` - Optional prefix to add before the filename
    ///
    /// # Returns
    /// * `Result<PathBuf>` - Path to the created copy or error
    fn copy_file_with_timestamp_and_prefix(
        &self,
        file_path: &Path,
        archive_directory: &Path,
        custom_prefix: Option<&str>,
    ) -> Result<PathBuf> {
        // Validate that the source file exists
        if !file_path.exists() {
            return Err(FileFantasticError::NotFound(file_path.to_path_buf()));
        }

        // Get file name components
        let file_stem = file_path.file_stem()
            .ok_or_else(|| FileFantasticError::InvalidName(
                "Could not extract file name".to_string()
            ))?
            .to_string_lossy();

        let extension = file_path.extension()
            .map(|ext| format!(".{}", ext.to_string_lossy()))
            .unwrap_or_else(|| String::new());

        // Generate full timestamp
        let timestamp = createarchive_timestamp_with_precision(
            SystemTime::now(),
            true,
            );

        // Build the archive name with optional prefix
        let archive_name = if let Some(prefix) = custom_prefix {
            format!("{}{}_{}{}", prefix, file_stem, timestamp, extension)
        } else {
            format!("{}_{}{}", file_stem, timestamp, extension)
        };

        // Create full path for the archive
        let archive_path = archive_directory.join(archive_name);

        // Copy the file
        fs::copy(file_path, &archive_path)
            .map_err(|e| FileFantasticError::Io(e))?;

        Ok(archive_path)
    }

    /// Creates a simple archive format for a single file
    ///
    /// # Purpose
    /// Since we can't use third-party libraries, this creates a simple
    /// archive format that stores the file with some metadata.
    ///
    /// # Note
    /// This is a simplified implementation. In production, you would
    /// implement proper ZIP format or use a library.
    fn create_simple_file_archive(&self, source_file: &Path, archive_path: &Path) -> Result<()> {
        // Read source file
        let mut source = File::open(source_file)
            .map_err(|e| FileFantasticError::Io(e))?;

        let mut contents = Vec::new();
        source.read_to_end(&mut contents)
            .map_err(|e| FileFantasticError::Io(e))?;

        // Create archive file with simple format
        let mut archive = File::create(archive_path)
            .map_err(|e| FileFantasticError::Io(e))?;

        let filename = source_file.file_name()
            .ok_or_else(|| FileFantasticError::InvalidName(
                "Could not extract filename".to_string()
            ))?
            .to_string_lossy();

        // Write simple archive format
        // Format: [filename_length:u32][filename][file_size:u64][file_contents]
        let filename_bytes = filename.as_bytes();
        archive.write_all(&(filename_bytes.len() as u32).to_le_bytes())
            .map_err(|e| FileFantasticError::Io(e))?;
        archive.write_all(filename_bytes)
            .map_err(|e| FileFantasticError::Io(e))?;
        archive.write_all(&(contents.len() as u64).to_le_bytes())
            .map_err(|e| FileFantasticError::Io(e))?;
        archive.write_all(&contents)
            .map_err(|e| FileFantasticError::Io(e))?;

        Ok(())
    }

    /// Interactive interface to add a file to the file stack
    ///
    /// # Purpose
    /// Provides a simple interactive interface for adding files to the file stack.
    /// Shows the current directory listing and prompts user to select a file by number.
    /// Works with the current page view only, utilizing existing navigation state.
    ///
    /// # Arguments
    /// * `nav_state` - Current navigation state with lookup table for numbered selection
    /// * `current_directory_entries` - Current directory entries to display for selection (current page only)
    /// * `current_directory_path` - Current directory path for display context
    ///
    /// # Returns
    /// * `Result<()>` - Success or error with context
    ///
    /// # User Interface Workflow
    ///
    /// ## Step 1: Display Current View
    /// - Show current directory contents with numbered items (same as main navigation)
    /// - This respects current filter and pagination state
    ///
    /// ## Step 2: Selection Prompt
    /// - User selects item by number (same interface as navigation)
    /// - Can cancel with 'b' for back
    ///
    /// ## Step 3: Validation
    /// - Verify selected item exists in lookup table
    /// - Ensure selected item is a file (not a directory)
    ///
    /// ## Step 4: Add to Stack
    /// - Add file path to the file stack
    /// - Display confirmation with updated stack count
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
    /// === Add File to Stack ===
    /// Select file to add to stack
    /// Enter file number (or 'b' to back/cancel): 2
    ///
    /// ✓ Added 'document.txt' to file stack. Total files: 1
    /// ```
    ///
    /// # Error Handling
    /// - Validates numbered selections against navigation lookup table
    /// - Ensures selected items are files, not directories
    /// - Provides clear error messages for invalid selections
    /// - Handles cancellation gracefully
    /// - Manages IO errors during user interaction
    pub fn interactive_add_file_to_stack(
        &mut self,
        nav_state: &NavigationState,
        current_directory_entries: &[FileSystemEntry],
        current_directory_path: &PathBuf,
    ) -> Result<()> {

        // WORKFLOW STEP 1: Display current directory contents
        // This shows the current page with existing filters and numbering
        display_directory_contents(
            current_directory_entries,
            current_directory_path,
            None,  // Pagination info handled by main navigation
            nav_state.current_filter,
            nav_state,
        ).map_err(|e| FileFantasticError::Io(e))?;

        // WORKFLOW STEP 2: Prompt for file selection
        println!("\n=== Add File to Stack ===");
        println!("Select file to add to stack");
        print!("Enter file number (or 'b' to back/cancel): ");
        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| FileFantasticError::Io(e))?;
        let input = input.trim();

        // Handle cancellation request
        if input.eq_ignore_ascii_case("b") {
            println!("Back/Cancelled.");
            return Ok(());
        }

        // WORKFLOW STEP 3: Validate and process the user's selection
        if let Ok(number) = input.parse::<usize>() {
            // Use navigation state's lookup to find the selected item
            if let Some(item_info) = nav_state.lookup_item(number) {

                // Check if selected item is a file (not a directory)
                if item_info.item_type == FileSystemItemType::Directory {
                    // User selected a directory - show error and return
                    let item_name = item_info.item_path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy();
                    eprintln!("\n✗ Error: '{}' is a directory. Please select a file.", item_name);
                } else {
                    // WORKFLOW STEP 4: It's a file - add to stack

                    // Extract file name for display
                    let file_name = item_info.item_path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy();

                    // Add the file to the stack
                    match self.add_file_to_stack(item_info.item_path.clone()) {
                        Ok(()) => {
                            // Success - show confirmation with stack count
                            println!("\n✓ Added '{}' to file stack. Total files: {}",
                                    file_name,
                                    self.file_path_stack.len());
                        },
                        Err(e) => {
                            // Failed to add to stack - show error
                            eprintln!("\n✗ Failed to add file to stack: {}", e);
                        }
                    }
                }

            } else {
                // Invalid item number - not in lookup table
                println!("Error: Invalid item number {}. Please try again.", number);
            }
        } else {
            // Input was not a valid number
            println!("Error: Please enter a valid number or 'b' to cancel.");
        }

        // Wait for user acknowledgment before returning to main interface
        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());

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

        // // // Version to pop-remove item
        // // Try to parse as index and validate
        // if let Ok(index) = input.parse::<usize>() {
        //     if index > 0 && index <= self.file_path_stack.len() {
        //         // Convert to actual vector index (1-based display to 0-based storage)
        //         let actual_index = self.file_path_stack.len() - index;
        //         let file = self.file_path_stack.remove(actual_index);
        //         println!("Retrieved: {}", file.file_name().unwrap_or_default().to_string_lossy());
        //         return Ok(Some(file));
        //     } else {
        //         println!("Error: Invalid file number {}. Valid range: 1-{}", index, self.file_path_stack.len());
        //     }
        // } else {
        //     println!("Error: Please enter a valid number, press Enter for most recent, or 'c' to cancel.");
        // }

        // Try to parse as index and validate
        if let Ok(index) = input.parse::<usize>() {
            if index > 0 && index <= self.file_path_stack.len() {
                // Convert to actual vector index (1-based display to 0-based storage)
                let actual_index = self.file_path_stack.len() - index;

                // Use .get() for bounds-checked access
                if let Some(file) = self.file_path_stack.get(actual_index) {
                    println!("Retrieved: {}", file.file_name().unwrap_or_default().to_string_lossy());
                    return Ok(Some(file.clone())); // Clone to return ownership
                } else {
                    println!("Error: Index out of bounds");
                }
            } else {
                println!("Error: Invalid file number {}. Valid range: 1-{}", index, self.file_path_stack.len());
            }
        } else {
            println!("Error: Please enter a valid number, press Enter for most recent, or 'c' to cancel.");
        }

        Ok(None)
    }

    /*
    pending use of saved directories
    */
    // /// Q&A interface to save current directory to directory stack
    // ///
    // /// # Purpose
    // /// Provides an interactive interface for adding the current directory
    // /// to the directory stack, with user confirmation.
    // ///
    // /// # Arguments
    // /// * `current_directory` - The current directory to potentially add
    // ///
    // /// # Returns
    // /// * `Result<()>` - Success or error with context
    // ///
    // /// # User Interface Flow
    // /// 1. Display the current directory path
    // /// 2. Ask for user confirmation to add it to the stack
    // /// 3. Add to stack if user confirms (default is yes)
    // /// 4. Display confirmation with current stack size
    // ///
    // /// # Confirmation Logic
    // /// - Empty input or 'y'/'Y': Add to stack
    // /// - Any other input: Do not add to stack
    // ///
    // /// # Example Interaction
    // /// ```text
    // /// === Add Directory to Stack ===
    // /// Current directory: /home/user/projects
    // /// Add current directory to stack? (Y/n):
    // /// Added to directory stack. Total directories: 2
    // /// ```
    // pub fn interactive_save_directory_to_stack(&mut self, current_directory: &PathBuf) -> Result<()> {
    //     println!("\n=== Add Directory to Stack ===");
    //     println!("Current directory: {}", current_directory.display());

    //     print!("Add current directory to stack? (Y/n): ");
    //     io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;

    //     let mut response = String::new();
    //     io::stdin().read_line(&mut response).map_err(|e| FileFantasticError::Io(e))?;

    //     // Default to 'yes' if user just presses enter
    //     if response.trim().is_empty() || response.trim().eq_ignore_ascii_case("y") {
    //         self.add_directory_to_stack(current_directory.clone())?;
    //         println!("Added to directory stack. Total directories: {}", self.directory_path_stack.len());
    //     } else {
    //         println!("Cancelled.");
    //     }

    //     Ok(())
    // }

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
        println!("1. Add file TO file-stack");
        println!("2. Get: Save file here, FROM file-stack");
        // println!("3. Add current directory to stack");
        /*
        Pending future functions
        to use saved directory for something
        */
        println!("3. Save current location as pocket dimension");
        println!("4. Go to pocket dimension");
        println!("5. View stacks & pocket dimensions");
        println!("6. Archive file/directory 'a': zip/timestamp");
        println!("7. Clear all stacks");
        // println!("--  --");
        println!();
        print!("Select Action (1-7)  or (b)ack / empty-Enter ");
        io::stdout().flush().map_err(|e| FileFantasticError::Io(e))?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| FileFantasticError::Io(e))?;

        // Process user selection and return appropriate action
        match input.trim() {
            "1" => Ok(GetSendModeAction::AddFileToStack),
            "2" => Ok(GetSendModeAction::GetFileFromStack),
            // "3" => Ok(GetSendModeAction::AddDirectoryToStack),
            "3" => Ok(GetSendModeAction::SavePocketDimension),
            "4" => Ok(GetSendModeAction::GoToPocketDimension),
            "5" => Ok(GetSendModeAction::ViewStacks),
            "6" => Ok(GetSendModeAction::ArchiveSelection),
            "7" => Ok(GetSendModeAction::ClearAll),
            "" | "b" => Ok(GetSendModeAction::ReturnToBrowser), // Default to return
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

    /*
    pending future functions to use directory stack
    */
    // // Add current directory to the directory path stack
    // // Triggers the directory addition workflow with confirmation
    // AddDirectoryToStack,

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
/// automatically handling filename conflicts by archiving the EXISTING file
/// (not the new one) and replacing it with the new file.
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
///    a. Create temporary backup of existing file
///    b. Copy new file with temporary name
///    c. Move existing file to archive directory
///    d. Rename temporary new file to final name
///    e. Clean up temporary backup on success
///
/// # Error Recovery
/// If any step fails after modifying files, attempts to restore original state
/// using the temporary backup file.
///
/// # File Preservation Guarantee
/// - Never loses the existing file (archived or restored on failure)
/// - Never overwrites without archiving
/// - Atomic operations where possible
/// - Full rollback on failure
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

    // Check if file exists at destination
    let final_destination_path = if primary_destination_path.exists() {
        // File exists, use safe archive strategy with temp files
        println!("File '{}' already exists in destination.", source_filename);
        println!("Creating safe archive of existing file...");

        // Generate timestamp for unique temp names
        let timestamp = generate_archive_timestamp();

        // Step 1: Create temporary backup of the existing file (for rollback if needed)
        // Add process ID to make temp names more unique
        let pid = std::process::id();
        let temp_backup_filename = format!("backup_{}_{}_{}", pid, timestamp, source_filename);
        let temp_backup_path = destination_directory.join(&temp_backup_filename);

        // Check if temp backup already exists (very unlikely but possible)
        if temp_backup_path.exists() {
            return Err(FileFantasticError::InvalidName(
                format!("Temporary backup file already exists: {}", temp_backup_path.display())
            ));
        }

        // Copy existing file to temporary backup - handle race condition where file might have been deleted
        match fs::copy(&primary_destination_path, &temp_backup_path) {
            Ok(_) => {
                println!("Created temporary backup: {}", temp_backup_path.display());
            },
            Err(e) => {
                // Check if the error is because the file no longer exists (race condition)
                if e.kind() == io::ErrorKind::NotFound {
                    // File was deleted between our check and now - just do a normal copy
                    println!("Original file no longer exists, proceeding with normal copy...");

                    fs::copy(source_file_path, &primary_destination_path).map_err(|copy_err| {
                        match copy_err.kind() {
                            io::ErrorKind::PermissionDenied => {
                                FileFantasticError::PermissionDenied(primary_destination_path.clone())
                            },
                            _ => FileFantasticError::Io(copy_err)
                        }
                    })?;

                    println!("File copied to: {}", primary_destination_path.display());
                    return Ok(primary_destination_path);
                }

                // Other error - fail
                return Err(match e.kind() {
                    io::ErrorKind::PermissionDenied => {
                        FileFantasticError::PermissionDenied(primary_destination_path.clone())
                    },
                    _ => FileFantasticError::Io(e)
                });
            }
        }

        // Step 2: Copy new file with temporary name
        let temp_new_filename = format!("newfile_{}_{}_{}", pid, timestamp, source_filename);
        let temp_new_path = destination_directory.join(&temp_new_filename);

        // Check if temp new file already exists
        if temp_new_path.exists() {
            // Clean up temp backup before returning error
            let _ = fs::remove_file(&temp_backup_path);
            return Err(FileFantasticError::InvalidName(
                format!("Temporary new file already exists: {}", temp_new_path.display())
            ));
        }

        // Copy source file to temporary location
        match fs::copy(source_file_path, &temp_new_path) {
            Ok(_) => {
                println!("Copied new file to temporary location: {}", temp_new_path.display());
            },
            Err(e) => {
                // Rollback: remove temp backup since we're failing
                let _ = fs::remove_file(&temp_backup_path);

                return Err(match e.kind() {
                    io::ErrorKind::PermissionDenied => {
                        FileFantasticError::PermissionDenied(temp_new_path)
                    },
                    _ => FileFantasticError::Io(e)
                });
            }
        }

        // Step 3: Ensure archive directory exists
        let archive_directory_path = match ensure_archive_directory_exists(destination_directory) {
            Ok(path) => path,
            Err(e) => {
                // Rollback: remove temp files
                let _ = fs::remove_file(&temp_new_path);
                let _ = fs::remove_file(&temp_backup_path);
                return Err(e);
            }
        };

        // Step 4: Move the existing file to archive (from primary location)
        let archive_filename = generate_archive_filename(&source_filename, &timestamp);
        let archive_destination_path = archive_directory_path.join(&archive_filename);

        // Check if archive destination already exists
        if archive_destination_path.exists() {
            // Clean up temp files before returning error
            let _ = fs::remove_file(&temp_new_path);
            let _ = fs::remove_file(&temp_backup_path);
            return Err(FileFantasticError::InvalidName(
                format!("Archive file already exists: {}", archive_destination_path.display())
            ));
        }

        // Try to rename first (atomic on same filesystem), fall back to copy+delete
        // Explicitly specify the type for the Result
        let archive_result: std::result::Result<(), io::Error> = fs::rename(&primary_destination_path, &archive_destination_path)
            .or_else(|rename_err| {
                // Rename failed, probably cross-filesystem - try copy then delete
                eprintln!("Rename to archive failed ({}), trying copy+delete...", rename_err);
                fs::copy(&primary_destination_path, &archive_destination_path).map_err(|copy_err| {
                    eprintln!("Failed to copy to archive: {}", copy_err);
                    copy_err
                })?;
                fs::remove_file(&primary_destination_path).map_err(|rm_err| {
                    eprintln!("Failed to remove original after archive copy: {} {}", rm_err, rename_err);
                    rm_err
                })?;
                Ok(())
            });

        match archive_result {
            Ok(_) => {
                println!("Archived existing file to: {}", archive_destination_path.display());
            },
            Err(e) => {
                // Rollback: remove temp new file, keep temp backup for manual recovery
                let _ = fs::remove_file(&temp_new_path);

                eprintln!("Failed to archive existing file. Backup preserved at: {}",
                         temp_backup_path.display());

                return Err(match e.kind() {
                    io::ErrorKind::PermissionDenied => {
                        FileFantasticError::PermissionDenied(archive_destination_path)
                    },
                    _ => FileFantasticError::Io(e)
                });
            }
        }

        // Step 5: Move temp new file to final destination
        // Try rename first (atomic), fall back to copy+delete
        // Explicitly specify the type for the Result
        let rename_result: std::result::Result<(), io::Error> = fs::rename(&temp_new_path, &primary_destination_path)
            .or_else(|rename_err| {
                // Rename failed - try copy then delete
                eprintln!("Rename of new file failed ({}), trying copy+delete...", rename_err);
                fs::copy(&temp_new_path, &primary_destination_path).map_err(|copy_err| {
                    eprintln!("Failed to copy new file to destination: {}", copy_err);
                    copy_err
                })?;
                fs::remove_file(&temp_new_path).map_err(|rm_err| {
                    eprintln!("Failed to remove temp new file after copy: {} {}", rm_err, rename_err);
                    rm_err
                })?;
                Ok(())
            });

        match rename_result {
            Ok(_) => {
                println!("New file installed at: {}", primary_destination_path.display());

                // Step 6: Clean up temp backup (success case)
                match fs::remove_file(&temp_backup_path) {
                    Ok(_) => {
                        println!("Cleaned up temporary backup.");
                    },
                    Err(e) => {
                        // Non-critical error, just warn
                        eprintln!("Warning: Could not remove temporary backup at {}: {}",
                                 temp_backup_path.display(), e);
                    }
                }

                primary_destination_path
            },
            Err(e) => {
                // Critical failure: try to restore original state
                eprintln!("Failed to move new file to destination: {}", e);
                eprintln!("Attempting to restore original state...");

                // Try to restore from temp backup (more reliable than from archive)
                match fs::copy(&temp_backup_path, &primary_destination_path) {
                    Ok(_) => {
                        eprintln!("Successfully restored original file from backup.");
                        // Remove the file we put in archive since we're restoring
                        let _ = fs::remove_file(&archive_destination_path);
                    },
                    Err(restore_err) => {
                        // Try to restore from archive as last resort
                        // Explicitly specify type
                        let restore_from_archive: std::result::Result<(), io::Error> =
                            fs::rename(&archive_destination_path, &primary_destination_path)
                            .or_else(|_rename_err| {
                                fs::copy(&archive_destination_path, &primary_destination_path)?;
                                fs::remove_file(&archive_destination_path)?;
                                Ok(())
                            });

                        match restore_from_archive {
                            Ok(_) => {
                                eprintln!("Successfully restored original file from archive.");
                            },
                            Err(archive_restore_err) => {
                                eprintln!("Could not restore original file: {}", restore_err);
                                eprintln!("Archive restore also failed: {}", archive_restore_err);
                                eprintln!("Original file is in archive: {}", archive_destination_path.display());
                                eprintln!("Backup available at: {}", temp_backup_path.display());
                            }
                        }
                    }
                }

                // Clean up temp new file if it still exists
                let _ = fs::remove_file(&temp_new_path);

                return Err(match e.kind() {
                    io::ErrorKind::PermissionDenied => {
                        FileFantasticError::PermissionDenied(primary_destination_path)
                    },
                    _ => FileFantasticError::Io(e)
                });
            }
        }
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
    /// Creates new directory view with dynamic pagination based on TUI settings
    ///
    /// # Purpose
    /// Initializes a paginated view of directory entries with user-configured
    /// items per page, allowing for customizable display density.
    ///
    /// # Arguments
    /// * `entries` - Slice of FileSystemEntry items to paginate
    /// * `items_per_page` - Number of items to show per page (from TUI settings)
    ///
    /// # Edge Cases
    /// - If items_per_page is 0, pagination still works (shows headers only)
    /// - If items_per_page > total entries, shows all entries on one page
    ///
    /// # Examples
    /// ```rust
    /// let items_per_page = calculate_items_per_page_from_state(&nav_state);
    /// let dir_view = DirectoryView::new(&entries, items_per_page);
    /// ```
    fn new(entries: &'a [FileSystemEntry], items_per_page: u16) -> Self {
        Self {
            entries,
            current_page: 0,
            items_per_page: items_per_page as usize, // Convert to usize for slice operations
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
#[derive(Debug, Clone)]
pub struct FuzzySearchResult {
    /// Name of the matching item
    item_name: String,
    /// Full path to the item
    item_path: PathBuf,
    /// Levenshtein distance score
    distance: usize,
    /// Display index from the current view
    display_index: usize,
}

/// Unified search result type that can hold either fuzzy or grep results
///
/// # Purpose
/// Provides a single type that can represent results from different search modes
/// while maintaining type safety and preventing field confusion between the two
/// fundamentally different search types.
///
/// # Variants
/// * `Fuzzy` - Contains a FuzzySearchResult from name-based searching
/// * `Grep` - Contains a GrepSearchResult from content-based searching
///
/// # Design Rationale
/// Using an enum instead of a single struct with optional fields ensures:
/// - No meaningless fields (like Levenshtein distance for grep results)
/// - Clear separation of concerns between search types
/// - Compile-time safety against accessing wrong fields
/// - Self-documenting code that clearly shows which search was performed
///
/// # Usage
/// The search wrapper returns `Vec<UnifiedSearchResult>` and the display
/// function pattern-matches to determine how to display the results.
#[derive(Debug, Clone)]
pub enum UnifiedSearchResult {
    /// Result from fuzzy name matching
    Fuzzy(FuzzySearchResult),
    /// Result from grep content searching
    Grep(GrepSearchResult),
}

/// Result structure specifically for grep (file content) search results
///
/// # Purpose
/// Represents a single match found when searching inside file contents.
/// This struct is specifically designed for grep-style searches where we
/// find text patterns within files, as opposed to fuzzy name matching.
///
/// # Fields
/// * `file_name` - The name of the file containing the match
/// * `file_path` - Full absolute path to the file containing the match
/// * `line_number` - The line number where the match was found (1-based indexing)
/// * `line_content` - The actual content of the line containing the match
/// * `display_index` - The number shown to the user for selection (1-based)
///
/// # Design Rationale
/// This struct is separate from FuzzySearchResult because:
/// - Grep searches don't use Levenshtein distance (that field would be meaningless)
/// - Grep searches need line numbers and line content (fuzzy searches don't)
/// - Keeping them separate prevents confusion and misuse of fields
///
/// # Usage Context
/// Created by `grep_search_files()` when searching file contents.
/// Consumed by display functions and user selection handlers.
///
/// # Example
/// ```rust
/// // A grep result for finding "TODO" in a source file
/// GrepSearchResult {
///     file_name: "main.rs".to_string(),
///     file_path: PathBuf::from("/home/user/project/src/main.rs"),
///     line_number: 42,
///     line_content: "// TODO: Implement this feature".to_string(),
///     display_index: 1,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct GrepSearchResult {
    /// Name of the file containing the match
    pub file_name: String,
    /// Full absolute path to the file
    pub file_path: PathBuf,
    /// Line number where match was found (1-based)
    pub line_number: usize,
    /// Content of the matching line (may be truncated)
    pub line_content: String,
    /// Display number for user selection (1-based)
    pub display_index: usize,
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

/// Creates a timestamp string specifically for archive file naming
///
/// # Purpose
/// Generates a consistent, sortable timestamp string for archive filenames
/// that works identically across all platforms (Windows, Linux, macOS).
///
/// # Arguments
/// * `time` - The SystemTime to format (typically SystemTime::now())
///
/// # Returns
/// * `String` - Timestamp in format: "YY_MM_DD_HH_MM_SS"
///
/// # Format Specification
/// - YY: Two-digit year (00-99)
/// - MM: Two-digit month (01-12)
/// - DD: Two-digit day (01-31)
/// - HH: Two-digit hour in 24-hour format (00-23)
/// - MM: Two-digit minute (00-59)
/// - SS: Two-digit second (00-59)
///
/// # Examples
/// - "24_01_15_14_30_45" for January 15, 2024 at 2:30:45 PM
/// - "23_12_31_23_59_59" for December 31, 2023 at 11:59:59 PM
///
/// # Platform Consistency
/// This function produces identical output on all platforms by using
/// epoch-based calculations rather than platform-specific date commands.
fn create_archive_timestamp(time: SystemTime) -> String {
    // Get duration since Unix epoch
    let duration_since_epoch = match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(_) => {
            // System time before Unix epoch - use fallback
            eprintln!("Warning: System time is before Unix epoch, using fallback timestamp");
            return String::from("70_01_01_00_00_00");
        }
    };

    let total_seconds = duration_since_epoch.as_secs();

    // Use the accurate date calculation
    let (year, month, day, hour, minute, second) = epoch_seconds_to_datetime_components(total_seconds);

    // Format as YY_MM_DD_HH_MM_SS
    format!(
        "{:02}_{:02}_{:02}_{:02}_{:02}_{:02}",
        year % 100,  // Two-digit year
        month,
        day,
        hour,
        minute,
        second
    )
}

/// Converts Unix epoch seconds to accurate date/time components
///
/// # Purpose
/// Provides accurate date/time calculation that properly handles:
/// - Leap years (including century rules)
/// - Correct days per month
/// - Time zones (UTC)
///
/// # Arguments
/// * `epoch_seconds` - Seconds since Unix epoch (1970-01-01 00:00:00 UTC)
///
/// # Returns
/// * `(year, month, day, hour, minute, second)` - All as u32 values
///
/// # Algorithm
/// Uses proper calendar arithmetic to convert epoch seconds to date/time
/// components, accounting for leap years and varying month lengths.
fn epoch_seconds_to_datetime_components(epoch_seconds: u64) -> (u32, u32, u32, u32, u32, u32) {
    // Time component calculations
    const SECONDS_PER_MINUTE: u64 = 60;
    const SECONDS_PER_HOUR: u64 = 3600;
    const SECONDS_PER_DAY: u64 = 86400;

    // Calculate time of day components
    let seconds_today = epoch_seconds % SECONDS_PER_DAY;
    let hour = (seconds_today / SECONDS_PER_HOUR) as u32;
    let minute = ((seconds_today % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u32;
    let second = (seconds_today % SECONDS_PER_MINUTE) as u32;

    // Calculate date components
    let days_since_epoch = epoch_seconds / SECONDS_PER_DAY;
    let (year, month, day) = days_to_ymd(days_since_epoch);

    (year, month, day, hour, minute, second)
}

/// Converts days since Unix epoch to year, month, day
///
/// # Purpose
/// Accurate calendar calculation that properly handles leap years
/// and correct month lengths.
///
/// # Arguments
/// * `days_since_epoch` - Days since 1970-01-01
///
/// # Returns
/// * `(year, month, day)` - Calendar date components
///
/// # Leap Year Rules
/// - Divisible by 4: leap year
/// - Divisible by 100: not a leap year
/// - Divisible by 400: leap year
fn days_to_ymd(days_since_epoch: u64) -> (u32, u32, u32) {
    // Start from 1970-01-01
    let mut year = 1970u32;
    let mut remaining_days = days_since_epoch;

    // Helper function to check if a year is a leap year
    let is_leap_year = |y: u32| -> bool {
        (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
    };

    // Subtract complete years
    while remaining_days > 0 {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days >= days_in_year {
            remaining_days -= days_in_year;
            year += 1;
        } else {
            break;
        }
    }

    // Days in each month for normal and leap years
    const DAYS_IN_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const DAYS_IN_MONTH_LEAP: [u32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let days_in_months = if is_leap_year(year) {
        &DAYS_IN_MONTH_LEAP
    } else {
        &DAYS_IN_MONTH
    };

    // Find the month and day
    let mut month = 1u32;
    let mut days_left = remaining_days as u32;

    for &days_in_month in days_in_months.iter() {
        if days_left >= days_in_month {
            days_left -= days_in_month;
            month += 1;
        } else {
            break;
        }
    }

    // Day of month (1-based), add 1 because we want 1-31, not 0-30
    let day = days_left + 1;

    (year, month, day)
}

/// Creates a timestamp with optional microsecond precision for uniqueness
///
/// # Purpose
/// When multiple archives might be created in the same second, this
/// adds microsecond precision to ensure unique filenames.
///
/// # Arguments
/// * `time` - The SystemTime to format
/// * `include_microseconds` - Whether to append microseconds
///
/// # Returns
/// * `String` - Timestamp, optionally with microseconds appended
///
/// # Format
/// - Without microseconds: "YY_MM_DD_HH_MM_SS"
/// - With microseconds: "YY_MM_DD_HH_MM_SS_UUUUUU"
pub fn createarchive_timestamp_with_precision(
    time: SystemTime,
    include_microseconds: bool
) -> String {
    let base_timestamp = create_archive_timestamp(time);

    if !include_microseconds {
        return base_timestamp;
    }

    // Get microseconds component
    let duration_since_epoch = match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(_) => return base_timestamp, // Fall back to base timestamp
    };

    let microseconds = duration_since_epoch.as_micros() % 1_000_000;

    format!("{}_{:06}", base_timestamp, microseconds)
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

// TODO android has directory_path not used warning here
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

    #[cfg(target_os = "android")]
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

/// Represents a TUI display size adjustment action with clear parameter names
///
/// # Purpose
/// Encapsulates user requests to adjust the Text User Interface display dimensions,
/// making the intent explicit through descriptive field names rather than ambiguous booleans.
///
/// # Fields
/// * `adjustment_type_true_is_tall_false_is_wide` - Explicitly indicates adjustment type:
///   - `true` = Adjusting height (tall command)
///   - `false` = Adjusting width (wide command)
/// * `adjustment_magnitude` - The positive number of units to adjust (1-65535)
/// * `adjustment_direction_true_is_positive_false_is_negative` - Explicitly indicates direction:
///   - `true` = Positive adjustment (adding units)
///   - `false` = Negative adjustment (removing units)
///
/// # Examples
/// - Command "tall+5" creates: (true, 5, true)
/// - Command "wide-10" creates: (false, 10, false)
/// - Command "tall-3" creates: (true, 3, false)
/// - Command "wide+20" creates: (false, 20, true)
#[derive(Debug)]
struct TuiAdjustmentAction {
    /// true = height/tall adjustment, false = width/wide adjustment
    adjustment_type_true_is_tall_false_is_wide: bool,
    /// Positive integer representing units to adjust (never zero)
    adjustment_magnitude: u16,
    /// true = positive/increase adjustment, false = negative/decrease adjustment
    adjustment_direction_true_is_positive_false_is_negative: bool,
}

/// NavigationAction variant for TUI size adjustments
///
/// Add this variant to the NavigationAction enum:
/// ```rust
///  Adjust TUI display size settings
///
///  Contains explicit adjustment parameters that clearly indicate:
/// - What dimension is being adjusted (height vs width)
/// - By how much (magnitude in positive units)
/// - In which direction (increase vs decrease)
///
/// AdjustTuiSize(TuiAdjustmentAction),
/// ```

/// Parses TUI size adjustment commands with explicit return values
///
/// # Purpose
/// Converts user input strings like "tall+5" or "wide-10" into structured
/// adjustment actions with clear, unambiguous field values.
///
/// # Command Format
/// Commands must follow this exact format:
/// - `tall+N` - Increase displayed rows by N (where N is 1-65535)
/// - `tall-N` - Decrease displayed rows by N (where N is 1-65535)
/// - `wide+N` - Increase name column width by N characters (where N is 1-65535)
/// - `wide-N` - Decrease name column width by N characters (where N is 1-65535)
///
/// # Arguments
/// * `input` - The trimmed user input string to parse
///
/// # Returns
/// * `Some(TuiAdjustmentAction)` - Successfully parsed adjustment command
/// * `None` - Input is not a valid TUI adjustment command
///
/// # Validation Rules
/// - Commands are case-sensitive (must be lowercase "tall" or "wide")
/// - No spaces allowed within the command
/// - Zero adjustments are rejected (must be 1 or greater)
/// - Numbers must parse completely (no trailing characters)
/// - Numbers must fit in u16 range (1-65535)
///
/// # Examples
/// ```rust
/// // Valid commands
/// assert!(parse_tui_adjustment_command("tall+5").is_some());
/// assert!(parse_tui_adjustment_command("wide-10").is_some());
/// assert!(parse_tui_adjustment_command("tall+65535").is_some());
///
/// // Invalid commands
/// assert!(parse_tui_adjustment_command("tall+0").is_none()); // Zero not allowed
/// assert!(parse_tui_adjustment_command("TALL+5").is_none()); // Wrong case
/// assert!(parse_tui_adjustment_command("tall +5").is_none()); // Space not allowed
/// assert!(parse_tui_adjustment_command("tall+abc").is_none()); // Not a number
/// assert!(parse_tui_adjustment_command("tall+99999").is_none()); // Exceeds u16
/// assert!(parse_tui_adjustment_command("height+5").is_none()); // Wrong keyword
/// ```
fn parse_tui_adjustment_command(input: &str) -> Option<TuiAdjustmentAction> {
    // Determine if this is a tall or wide command
    let (adjustment_type_true_is_tall_false_is_wide, remaining_after_keyword) =
        if input.starts_with("tall") {
            (true, &input[4..])
        } else if input.starts_with("wide") {
            (false, &input[4..])
        } else {
            // Not a TUI adjustment command
            return None;
        };

    // Ensure there's a character after the keyword
    if remaining_after_keyword.is_empty() {
        return None;
    }

    // Extract the direction character and remaining number
    let first_char = remaining_after_keyword.chars().next()?;
    let (adjustment_direction_true_is_positive_false_is_negative, number_string) =
        match first_char {
            '+' => (true, &remaining_after_keyword[1..]),
            '-' => (false, &remaining_after_keyword[1..]),
            _ => return None, // Must have + or - after keyword
        };

    // Parse the magnitude, ensuring it's valid and non-zero
    match number_string.parse::<u16>() {
        Ok(magnitude) if magnitude > 0 => {
            Some(TuiAdjustmentAction {
                adjustment_type_true_is_tall_false_is_wide,
                adjustment_magnitude: magnitude,
                adjustment_direction_true_is_positive_false_is_negative,
            })
        },
        _ => None, // Parse failed or value was zero
    }
}

/// Applies a TUI adjustment action to the navigation state
///
/// # Purpose
/// Updates the NavigationState with new display size settings based on a parsed
/// adjustment command, ensuring the state accurately reflects user preferences.
///
/// # Arguments
/// * `nav_state` - Mutable reference to the navigation state to update
/// * `adjustment_action` - The parsed adjustment action containing all parameters
///
/// # State Updates
/// Based on the adjustment type:
/// - **Height adjustments** (tall): Updates tui_tall_adjustment and tui_tall_direction_sign,
///   then resets current_page_index to 0 (since items per page changes)
/// - **Width adjustments** (wide): Updates tui_wide_adjustment and tui_wide_direction_sign,
///   page index remains unchanged (same items, just displayed differently)
///
/// # Examples
/// ```rust
/// // User enters "tall+5"
/// let action = TuiAdjustmentAction {
///     adjustment_type_true_is_tall_false_is_wide: true,
///     adjustment_magnitude: 5,
///     adjustment_direction_true_is_positive_false_is_negative: true,
/// };
/// apply_tui_adjustment(&mut nav_state, &action);
/// // Result: nav_state.tui_tall_adjustment = 5
/// //         nav_state.tui_tall_direction_sign = true
/// //         nav_state.current_page_index = 0
/// ```
fn apply_tui_adjustment(nav_state: &mut NavigationState, adjustment_action: &TuiAdjustmentAction) {
    if adjustment_action.adjustment_type_true_is_tall_false_is_wide {
        // This is a height (tall) adjustment
        nav_state.tui_tall_adjustment = adjustment_action.adjustment_magnitude;
        nav_state.tui_tall_direction_sign = adjustment_action.adjustment_direction_true_is_positive_false_is_negative;
        // Reset to first page since the number of items per page has changed
        nav_state.current_page_index = 0;
    } else {
        // This is a width (wide) adjustment
        nav_state.tui_wide_adjustment = adjustment_action.adjustment_magnitude;
        nav_state.tui_wide_direction_sign = adjustment_action.adjustment_direction_true_is_positive_false_is_negative;
        // Page index remains the same - only display width changes, not content
    }
}

/// Formats TUI adjustment settings into human-readable display strings
///
/// # Purpose
/// Converts the current TUI adjustment settings into formatted strings suitable
/// for display in status bars, feedback messages, and pocket dimension descriptions.
///
/// # Arguments
/// * `tall_adjustment` - The magnitude of height adjustment
/// * `tall_direction_sign` - true for positive (more rows), false for negative (fewer rows)
/// * `wide_adjustment` - The magnitude of width adjustment
/// * `wide_direction_sign` - true for positive (wider), false for negative (narrower)
///
/// # Returns
/// * `(String, String)` - Tuple of (tall_display, wide_display) formatted strings
///
/// # Display Format
/// - Zero adjustments display as "default"
/// - Non-zero adjustments display as "tall+N", "tall-N", "wide+N", or "wide-N"
///
/// # Examples
/// ```rust
/// let (tall_str, wide_str) = format_tui_adjustments(5, true, 0, true);
/// assert_eq!(tall_str, "tall+5");
/// assert_eq!(wide_str, "default");
///
/// let (tall_str, wide_str) = format_tui_adjustments(3, false, 10, false);
/// assert_eq!(tall_str, "tall-3");
/// assert_eq!(wide_str, "wide-10");
/// ```
fn format_tui_adjustments(
    tall_adjustment: u16,
    tall_direction_sign: bool,
    wide_adjustment: u16,
    wide_direction_sign: bool
) -> (String, String) {
    let tall_display = if tall_adjustment == 0 {
        String::from("tall+N")
    } else {
        format!("tall{}{}",
                if tall_direction_sign { "+" } else { "-" },
                tall_adjustment)
    };

    let wide_display = if wide_adjustment == 0 {
        String::from("wide-N")
    } else {
        format!("wide{}{}",
                if wide_direction_sign { "+" } else { "-" },
                wide_adjustment)
    };

    (tall_display, wide_display)
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
    current_directory_path: &Path,
) -> Result<NavigationAction> {
    let input = input.trim();

    // Handle empty input first - refresh and clear filters
    if input.is_empty() {
        return Ok(NavigationAction::Refresh);
    }

    // Convert to lowercase for case-insensitive commands
    let lowercase_input = input.to_lowercase();

    // Handle single-character commands first
    if input.len() == 1 {
        match lowercase_input.as_str() {
            "q" => return Ok(NavigationAction::Quit),
            "b" => return Ok(NavigationAction::ParentDirectory),
            "t" => return Ok(NavigationAction::OpenNewTerminal),
            "n" => return Ok(NavigationAction::Sort('n')),
            "s" => return Ok(NavigationAction::Sort('s')),
            "m" => return Ok(NavigationAction::Sort('m')),
            "d" => return Ok(NavigationAction::Filter('d')), // Show directories only
            "f" => return Ok(NavigationAction::Filter('f')), // Show files only

            "a" => return Ok(NavigationAction::ArchiveModeShortcut),
            "v" | "c" | "y" | "p" | "g" => return Ok(NavigationAction::GetSendMode),
            _ => {}
        }
    }

    match lowercase_input.as_str() {
        "vsplit" => return Ok(NavigationAction::VsplitTmux),
        _ => {}
    }

    match lowercase_input.as_str() {
        "hsplit" => return Ok(NavigationAction::HsplitTmux),
        _ => {}
    }

    // In process_user_input, check for TUI commands after single-char but before number parsing:
    if let Some(adjustment_action) = parse_tui_adjustment_command(input) {
        return Ok(NavigationAction::AdjustTuiSize(adjustment_action));
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

    // Get both results and search type from wrapper
    // In process_user_input, you need to pass the current navigation directory
    let search_results = nav_state.fuzzy_search_manager_wrapper(
        input,
        all_entries,
        &current_directory_path
    );

    // Display paginated search results and get user selection
    let selection = display_paginated_search_results(
        &search_results,
        nav_state.tui_tall_adjustment,
        nav_state.tui_tall_direction_sign,
        nav_state.tui_wide_adjustment,
        nav_state.tui_wide_direction_sign,
    ).map_err(|e| {
        eprintln!("Failed to display search results: {}", e);
        FileFantasticError::Io(e)
    })?;

    // Handles user selection from search results
    //
    // # Purpose
    // After search results are displayed, this code processes the user's
    // selection and determines the appropriate action. It must handle
    // different search types (local fuzzy, recursive, grep) correctly.
    //
    // # Critical Implementation Detail
    // For grep and recursive searches, display_index values are renumbered
    // (1, 2, 3...) and do NOT correspond to positions in all_entries.
    // We must determine file/directory type from the SearchResult itself
    // or the filesystem, not from all_entries.
    //
    // # Search Type Behaviors
    // - Local fuzzy search: display_index matches position in all_entries
    // - Recursive search: display_index is renumbered, path may not be in all_entries
    // - Grep search: display_index is renumbered, results are ALWAYS files
    if let Ok(number) = selection.trim().parse::<usize>() {
        // Find the search result with the matching display_index
        if let Some(selected) = search_results.iter().find(|r| {
            match r {
                UnifiedSearchResult::Grep(g) => g.display_index == number,
                UnifiedSearchResult::Fuzzy(f) => f.display_index == number,
            }
        }) {
            let path = match selected {
                UnifiedSearchResult::Grep(g) => &g.file_path,
                UnifiedSearchResult::Fuzzy(f) => &f.item_path,
            };

            // Determine action based on path
            if path.is_dir() {
                return Ok(NavigationAction::ChangeDirectory(path.clone()));
            } else {
                return Ok(NavigationAction::OpenFile(path.clone()));
            }
        }
    }

    Ok(NavigationAction::Invalid)
}

/// Static set of known plaintext file extensions for efficient lookup
///
/// # Purpose
/// Provides a pre-compiled set of file extensions that are known to contain
/// plaintext data. This allows the grep search to skip binary files before
/// attempting to open them, saving I/O operations and preventing errors.
///
/// # Categories Included
/// - Data formats: csv, tsv, txt, json, xml, yaml, yml
/// - Configuration: toml, ini, conf, cfg, env, properties, gitignore, dockerignore
/// - Programming languages: rs, py, js, ts, go, c, cpp, h, hpp, java, rb, php, etc.
/// - Web technologies: html, css, scss, sass, less, vue, jsx, tsx
/// - Documentation: md, rst, tex, adoc, org, pod
/// - Scripts: sh, bash, zsh, fish, ps1, bat, cmd
/// - Build files: makefile, cmake, gradle, sbt, cargo.toml, package.json
/// - Other text: log, sql, graphql, proto, diff, patch
///
/// # Implementation Notes
/// - Uses OnceLock for thread-safe lazy initialization
/// - HashSet provides O(1) average-case lookup performance
/// - Extensions are stored without the leading dot for consistency
static PLAINTEXT_EXTENSIONS: OnceLock<HashSet<&'static str>> = OnceLock::new();

/// Initializes and returns the set of plaintext file extensions
///
/// # Purpose
/// Lazily initializes the static set of known plaintext extensions on first use.
/// Subsequent calls return the already-initialized set without re-creating it.
///
/// # Returns
/// Reference to the HashSet containing plaintext extensions (without dots)
///
/// # Performance
/// - First call: O(n) where n is number of extensions
/// - Subsequent calls: O(1) - just returns reference
///
/// # Thread Safety
/// Uses OnceLock to ensure thread-safe initialization even if called
/// from multiple threads simultaneously.
fn get_plaintext_extensions() -> &'static HashSet<&'static str> {
    PLAINTEXT_EXTENSIONS.get_or_init(|| {
        let mut extensions = HashSet::new();

        // Data and serialization formats
        extensions.insert("csv");
        extensions.insert("tsv");
        extensions.insert("txt");
        extensions.insert("text");
        extensions.insert("json");
        extensions.insert("jsonl");
        extensions.insert("ndjson");
        extensions.insert("xml");
        extensions.insert("yaml");
        extensions.insert("yml");

        // Configuration files
        extensions.insert("toml");
        extensions.insert("ini");
        extensions.insert("conf");
        extensions.insert("config");
        extensions.insert("cfg");
        extensions.insert("env");
        extensions.insert("properties");
        extensions.insert("gitignore");
        extensions.insert("dockerignore");
        extensions.insert("editorconfig");
        extensions.insert("gitconfig");
        extensions.insert("gitattributes");

        // Programming languages - Systems
        extensions.insert("rs");     // Rust
        extensions.insert("c");      // C
        extensions.insert("cpp");    // C++
        extensions.insert("cxx");    // C++
        extensions.insert("cc");     // C++
        extensions.insert("h");      // C/C++ header
        extensions.insert("hpp");    // C++ header
        extensions.insert("hxx");    // C++ header
        extensions.insert("go");     // Go
        extensions.insert("zig");    // Zig
        extensions.insert("nim");    // Nim
        extensions.insert("v");      // V

        // Programming languages - JVM
        extensions.insert("java");   // Java
        extensions.insert("kt");     // Kotlin
        extensions.insert("kts");    // Kotlin Script
        extensions.insert("scala");  // Scala
        extensions.insert("clj");    // Clojure
        extensions.insert("cljs");   // ClojureScript
        extensions.insert("groovy"); // Groovy

        // Programming languages - Scripting
        extensions.insert("py");     // Python
        extensions.insert("pyw");    // Python Windows
        extensions.insert("pyi");    // Python Interface
        extensions.insert("rb");     // Ruby
        extensions.insert("php");    // PHP
        extensions.insert("lua");    // Lua
        extensions.insert("perl");   // Perl
        extensions.insert("pl");     // Perl

        // Programming languages - Web/JavaScript ecosystem
        extensions.insert("js");     // JavaScript
        extensions.insert("mjs");    // JavaScript Module
        extensions.insert("cjs");    // CommonJS
        extensions.insert("ts");     // TypeScript
        extensions.insert("tsx");    // TypeScript JSX
        extensions.insert("jsx");    // React JSX
        extensions.insert("vue");    // Vue.js
        extensions.insert("svelte"); // Svelte

        // Programming languages - Functional
        extensions.insert("hs");     // Haskell
        extensions.insert("lhs");    // Literate Haskell
        extensions.insert("ml");     // OCaml/Standard ML
        extensions.insert("mli");    // OCaml Interface
        extensions.insert("fs");     // F#
        extensions.insert("fsx");    // F# Script
        extensions.insert("elm");    // Elm
        extensions.insert("ex");     // Elixir
        extensions.insert("exs");    // Elixir Script
        extensions.insert("erl");    // Erlang
        extensions.insert("hrl");    // Erlang Header

        // Programming languages - Other
        extensions.insert("r");      // R
        extensions.insert("jl");     // Julia
        extensions.insert("m");      // MATLAB/Objective-C
        extensions.insert("swift");  // Swift
        extensions.insert("dart");   // Dart
        extensions.insert("pas");    // Pascal
        extensions.insert("pp");     // Pascal
        extensions.insert("asm");    // Assembly
        extensions.insert("s");      // Assembly

        // Web technologies
        extensions.insert("html");   // HTML
        extensions.insert("htm");    // HTML
        extensions.insert("xhtml");  // XHTML
        extensions.insert("css");    // CSS
        extensions.insert("scss");   // Sass
        extensions.insert("sass");   // Sass
        extensions.insert("less");   // Less
        extensions.insert("styl");   // Stylus

        // Documentation and markup
        extensions.insert("md");     // Markdown
        extensions.insert("markdown"); // Markdown
        extensions.insert("rst");    // reStructuredText
        extensions.insert("tex");    // LaTeX
        extensions.insert("latex");  // LaTeX
        extensions.insert("adoc");   // AsciiDoc
        extensions.insert("asciidoc"); // AsciiDoc
        extensions.insert("org");    // Org mode
        extensions.insert("pod");    // Perl POD
        extensions.insert("rdoc");   // Ruby Doc

        // Shell scripts
        extensions.insert("sh");     // Shell
        extensions.insert("bash");   // Bash
        extensions.insert("zsh");    // Zsh
        extensions.insert("fish");   // Fish
        extensions.insert("ksh");    // Korn Shell
        extensions.insert("csh");    // C Shell
        extensions.insert("tcsh");   // TC Shell
        extensions.insert("ps1");    // PowerShell
        extensions.insert("psm1");   // PowerShell Module
        extensions.insert("bat");    // Batch
        extensions.insert("cmd");    // Command

        // Build and project files
        extensions.insert("makefile");
        extensions.insert("mk");
        extensions.insert("cmake");
        extensions.insert("gradle");
        extensions.insert("sbt");
        extensions.insert("build");
        extensions.insert("rake");
        extensions.insert("gemfile");
        extensions.insert("podfile");
        extensions.insert("dockerfile");
        extensions.insert("containerfile");
        extensions.insert("vagrantfile");
        extensions.insert("jenkinsfile");
        extensions.insert("procfile");

        // Database and query languages
        extensions.insert("sql");    // SQL
        extensions.insert("psql");   // PostgreSQL
        extensions.insert("mysql");  // MySQL
        extensions.insert("graphql"); // GraphQL
        extensions.insert("gql");    // GraphQL
        extensions.insert("prisma"); // Prisma Schema

        // Data formats and protocols
        extensions.insert("proto");  // Protocol Buffers
        extensions.insert("thrift"); // Apache Thrift
        extensions.insert("avdl");   // Avro IDL
        extensions.insert("avsc");   // Avro Schema

        // Log and output files
        extensions.insert("log");    // Log files
        extensions.insert("out");    // Output files
        extensions.insert("err");    // Error files

        // Diff and patch files
        extensions.insert("diff");   // Diff
        extensions.insert("patch");  // Patch

        // License and readme files (often no extension)
        extensions.insert("license");
        extensions.insert("readme");
        extensions.insert("changelog");
        extensions.insert("authors");
        extensions.insert("contributors");
        extensions.insert("copyright");
        extensions.insert("notice");
        extensions.insert("todo");

        extensions
    })
}

/// Determines if a file path represents a plaintext file based on its extension
///
/// # Purpose
/// Checks whether a file should be treated as plaintext by examining its
/// file extension against a known set of plaintext formats. This allows
/// grep operations to skip binary files without attempting to open them.
///
/// # Arguments
/// * `path` - The file path to check (as string slice or Path-convertible type)
///
/// # Returns
/// * `true` if the file extension indicates a plaintext file
/// * `false` if the extension is unknown or indicates a binary file
///
/// # Algorithm
/// 1. Extracts the file extension from the path
/// 2. Converts to lowercase for case-insensitive comparison
/// 3. Checks against the static set of known plaintext extensions
/// 4. Also checks for common extensionless text files (README, LICENSE, etc.)
///
/// # Special Cases
/// - Files without extensions are checked against common names (README, LICENSE)
/// - Extensions are compared case-insensitively (.TXT matches .txt)
/// - Multiple extensions are handled (.tar.gz returns "gz")
///
/// # Examples
/// ```rust
/// assert!(is_plaintext_file("document.txt"));
/// assert!(is_plaintext_file("script.py"));
/// assert!(is_plaintext_file("README"));
/// assert!(!is_plaintext_file("image.png"));
/// assert!(!is_plaintext_file("binary.exe"));
/// ```
///
/// # Performance
/// O(1) average case due to HashSet lookup after initial string processing
fn is_plaintext_file(path: &str) -> bool {
    use std::path::Path;

    let path = Path::new(path);

    // Get the file name for checking extensionless files
    let file_name = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    // Check for common extensionless text files
    // These are often found in project roots
    let extensionless_text_files = [
        "README", "LICENSE", "CHANGELOG", "AUTHORS",
        "CONTRIBUTORS", "COPYRIGHT", "NOTICE", "TODO",
        "Makefile", "Dockerfile", "Containerfile",
        "Vagrantfile", "Jenkinsfile", "Procfile",
        "Gemfile", "Podfile", "Rakefile"
    ];

    // Check if it's a known extensionless text file (case-insensitive)
    let file_name_upper = file_name.to_uppercase();
    if extensionless_text_files.iter()
        .any(|&known| known.to_uppercase() == file_name_upper) {
        return true;
    }

    // Extract extension and check against known plaintext extensions
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            // Convert to lowercase for case-insensitive comparison
            let ext_lower = ext_str.to_lowercase();
            return get_plaintext_extensions().contains(ext_lower.as_str());
        }
    }

    // No extension and not a known extensionless text file
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plaintext_detection_common_extensions() {
        // Test common text file extensions
        assert!(is_plaintext_file("document.txt"));
        assert!(is_plaintext_file("data.csv"));
        assert!(is_plaintext_file("config.toml"));
        assert!(is_plaintext_file("script.py"));
        assert!(is_plaintext_file("code.rs"));
        assert!(is_plaintext_file("page.html"));
        assert!(is_plaintext_file("style.css"));
        assert!(is_plaintext_file("notes.md"));
    }

    #[test]
    fn test_plaintext_detection_case_insensitive() {
        // Extensions should be detected regardless of case
        assert!(is_plaintext_file("DOCUMENT.TXT"));
        assert!(is_plaintext_file("Script.PY"));
        assert!(is_plaintext_file("Config.TOML"));
        assert!(is_plaintext_file("MixedCase.Md"));
    }

    #[test]
    fn test_plaintext_detection_extensionless() {
        // Test known extensionless text files
        assert!(is_plaintext_file("README"));
        assert!(is_plaintext_file("LICENSE"));
        assert!(is_plaintext_file("Makefile"));
        assert!(is_plaintext_file("Dockerfile"));
        assert!(is_plaintext_file("dockerfile")); // case insensitive
    }

    #[test]
    fn test_plaintext_detection_binary_files() {
        // Test that binary files are not detected as plaintext
        assert!(!is_plaintext_file("image.png"));
        assert!(!is_plaintext_file("video.mp4"));
        assert!(!is_plaintext_file("archive.zip"));
        assert!(!is_plaintext_file("binary.exe"));
        assert!(!is_plaintext_file("library.dll"));
        assert!(!is_plaintext_file("object.o"));
        assert!(!is_plaintext_file("java.class"));
    }

    #[test]
    fn test_plaintext_detection_with_paths() {
        // Test with full paths
        assert!(is_plaintext_file("/home/user/document.txt"));
        assert!(is_plaintext_file("./src/main.rs"));
        assert!(is_plaintext_file("../config/settings.toml"));
        assert!(is_plaintext_file("C:\\Users\\Documents\\notes.md"));
    }

    #[test]
    fn test_plaintext_detection_edge_cases() {
        // Test edge cases
        assert!(!is_plaintext_file("")); // empty path
        assert!(!is_plaintext_file("no_extension")); // unknown file without extension
        assert!(is_plaintext_file(".gitignore")); // hidden file with known extension
        assert!(!is_plaintext_file(".")); // just a dot
        assert!(!is_plaintext_file("..")); // parent directory
    }

    #[test]
    fn test_get_plaintext_extensions_initialized() {
        // Test that the extension set is properly initialized
        let extensions = get_plaintext_extensions();

        // Check for some expected extensions
        assert!(extensions.contains("txt"));
        assert!(extensions.contains("py"));
        assert!(extensions.contains("rs"));
        assert!(extensions.contains("md"));

        // Verify it doesn't contain dots
        assert!(!extensions.contains(".txt"));

        // Check that it has a reasonable number of extensions
        assert!(extensions.len() > 50); // We should have many extensions
    }

    #[test]
    fn test_plaintext_extensions_multiple_calls() {
        // Ensure multiple calls return the same instance
        let ext1 = get_plaintext_extensions();
        let ext2 = get_plaintext_extensions();

        // Both should point to the same static instance
        assert_eq!(ext1 as *const _, ext2 as *const _);
    }
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
/// - "d"/"f": Generate Filter with respective character
/// - "a": archive mode
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

    // vertical split if in tmux at same ff cwd
    VsplitTmux,

    // horizontal split if in tmux at same ff cwd
    HsplitTmux,

    /// Filter to show only directories or files
    ///
    /// 'd' shows only directories
    /// 'f' shows only files
    /// Any other value resets the filter to show everything
    Filter(char),

    /// Enter Get-Send-Mode for advanced file operations
    GetSendMode,

    /// Adjust TUI display size settings
    ///
    /// Contains a TuiAdjustmentAction struct with clear parameter names that indicate:
    /// - What dimension is being adjusted (height vs width)
    /// - By how much (magnitude in positive units)
    /// - In which direction (increase vs decrease)
    ///
    /// Generated by commands like "tall+5", "wide-10", etc.
    AdjustTuiSize(TuiAdjustmentAction),

    /// archive mode
    ArchiveModeShortcut,
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
#[derive(Debug, Clone)]
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

/// Parses user input to extract search term and flags
///
/// # Arguments
/// * `input` - Raw user input string
///
/// # Returns
/// * `(search_term, recursive, grep)` - Parsed components
///
/// # Examples
/// * "document" -> ("document", false, false)
/// * "document -r" -> ("document", true, false)
/// * "TODO --grep" -> ("TODO", false, true)
/// * "TODO -r --grep" -> ("TODO", true, true)
fn parse_input_flags(input: &str) -> (&str, bool, bool, bool) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return ("", false, false, false);
    }

    let search_term = parts[0];
    let mut recursive = false;
    let mut grep = false;
    let mut case_sensitive = false;

    for part in parts.iter().skip(1) {
        match *part {
            "-r" | "--recursive" => recursive = true,
            "-g" | "--grep" => grep = true,
            "-c" | "--case-sensitive" => case_sensitive = true,
            _ => {}
        }
    }

    (search_term, recursive, grep, case_sensitive)
}

/// Displays search results with pagination and handles user selection
///
/// # Purpose
/// Presents search results in paginated format, allowing navigation through
/// pages and selection of items. Combines display and input handling in one
/// cohesive pagination loop.
///
/// # Arguments
/// * `results` - Slice of UnifiedSearchResult items to display
/// * `tui_tall_adjustment` - Height adjustment for terminal size (magnitude of adjustment)
/// * `tui_tall_direction_sign` - Direction of height adjustment (true = add rows, false = subtract rows)
/// * `tui_wide_adjustment` - Width adjustment for name column (magnitude of adjustment)
/// * `tui_wide_direction_sign` - Direction of width adjustment (true = wider names, false = narrower names)
///
/// # Returns
/// * `io::Result<String>` - User's selection as string (number for selection, empty for continue)
///
/// # Pagination Controls
/// - `w`, `j`, `<`, `[`, `up`, `prev`, `,`, `+`, `↑`: Previous page
/// - `x`, `k`, `>`, `]`, `down`, `next`, `.`, `-`, `↓`: Next page
/// - `1-N`: Select item number
/// - `Enter`: Continue without selection
///
/// # Display Adjustment Logic
/// The function respects user's TUI size preferences:
/// - Height: Controls how many items appear per page
/// - Width: Controls how much of the filename is visible before truncation
pub fn display_paginated_search_results(
    results: &[UnifiedSearchResult],
    tui_tall_adjustment: u16,
    tui_tall_direction_sign: bool,
    tui_wide_adjustment: u16,
    tui_wide_direction_sign: bool,
) -> io::Result<String> {

    // Handle empty results early to avoid unnecessary processing
    if results.is_empty() {
        println!("No matches found");
        return Ok(String::new());
    }

    // Calculate items per page based on TUI height adjustments
    // Start with the default number of items per page
    let base_items_per_page = ITEMS_PER_PAGE_DEFAULT as i16;

    // Apply the height adjustment based on direction
    // true = add rows (show more items), false = subtract rows (show fewer items)
    let tall_adjustment = if tui_tall_direction_sign {
        tui_tall_adjustment as i16  // Positive adjustment - more rows
    } else {
        -(tui_tall_adjustment as i16)  // Negative adjustment - fewer rows
    };

    // Calculate final items per page, ensuring minimum of 5 for usability
    let items_per_page = std::cmp::max(5, base_items_per_page + tall_adjustment) as usize;

    // Apply width adjustment based on direction
    // true = wider (show more of filename), false = narrower (show less of filename)
    // Start with the default width constant, convert to signed for arithmetic
    let base_width = MAX_TUI_CHAR_LENGTH_DEFAULT as i16;

    // Calculate the adjustment as signed (can be negative)
    let width_adjustment = if tui_wide_direction_sign {
        tui_wide_adjustment as i16  // Positive adjustment - wider
    } else {
        -(tui_wide_adjustment as i16)  // Negative adjustment - narrower
    };

    // Apply adjustment to base
    let adjusted_width = base_width + width_adjustment;

    // Ensure minimum width of 40 for usability, then convert back to usize
    let max_total_width = std::cmp::max(40, adjusted_width) as usize;

    // Calculate total number of pages needed
    // Uses ceiling division: (total + page_size - 1) / page_size
    let total_pages = (results.len() + items_per_page - 1) / items_per_page;

    // Track current page (0-indexed internally, displayed as 1-indexed to user)
    let mut current_page = 0;

    // Main pagination loop - continues until user makes a selection or exits
    loop {
        // Clear screen for clean display
        // ANSI escape codes: [2J = clear screen, [1;1H = move cursor to top-left
        print!("\x1B[2J\x1B[1;1H");

        // Calculate which results to show on current page
        let start_idx = current_page * items_per_page;
        let end_idx = std::cmp::min(start_idx + items_per_page, results.len());
        let page_results = &results[start_idx..end_idx];

        // Display results based on their type (Grep or Fuzzy)
        // We check the first result to determine the type since all results are the same type
        match &results[0] {
            UnifiedSearchResult::Grep(_) => {
                display_grep_page(
                    page_results,
                    current_page + 1,
                    total_pages,
                    max_total_width,
                )?;
            }
            UnifiedSearchResult::Fuzzy(_) => {
                display_fuzzy_page(
                    page_results,
                    current_page + 1,
                    total_pages,
                    max_total_width
                )?;
            }
        }

        // Display pagination information and navigation instructions
        println!("\nPage {}/{}, {} results, up w/j/<,  down x/k/>",
                current_page + 1,
                total_pages,
                results.len(),
        );
        print!("Enter choice: ");
        io::stdout().flush()?;  // Ensure prompt is displayed before waiting for input

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // Process user input based on type
        if input.is_empty() {
            // User pressed Enter without typing - continue without selection
            return Ok(String::new());

        } else if is_pagination_up_command(input) {
            // User wants to go to previous page
            if current_page > 0 {
                current_page -= 1;
            }
            // If already at first page, just redisplay (no error message to avoid clutter)

        } else if is_pagination_down_command(input) {
            // User wants to go to next page
            if current_page < total_pages - 1 {
                current_page += 1;
            }
            // If already at last page, just redisplay (no error message to avoid clutter)

        } else if let Ok(selection_num) = input.parse::<usize>() {
            // User entered a number - try to select that item
            let page_relative_selection = selection_num;

            // Validate selection is within current page bounds
            if page_relative_selection >= 1 && page_relative_selection <= (end_idx - start_idx) {
                // Convert page-relative index to global index
                // Page shows 1-N, but we need the actual index in the full results vector
                let global_index = start_idx + page_relative_selection;

                return Ok(global_index.to_string());
            } else {
                // Invalid selection number - show error and wait for acknowledgment
                println!("\nInvalid selection. Please choose 1-{}", end_idx - start_idx);
                println!("Press Enter to continue...");
                let mut _dummy = String::new();
                io::stdin().read_line(&mut _dummy)?;
            }
        }
        // Any other input is ignored and loop continues
    }
}

/// Helper function to display a page of grep search results
///
/// # Purpose
/// Formats and displays grep (content search) results for the current page.
/// Handles column width adjustments and text truncation to fit within terminal width.
///
/// # Arguments
/// * `page_results` - Slice of results for current page only
/// * `current_page` - Current page number (1-indexed for display)
/// * `total_pages` - Total number of pages
/// * `max_total_width` - Total terminal width (derived from user's TUI width settings)
///
/// # Width Calculation
/// The function respects terminal width constraints:
/// - Fixed: 3 chars for number column, 7 chars for line column (total 10)
/// - Dynamic: Remaining width split 60/40 between file and content
/// - Layout: [##_][FILE...N1...][_LINE_][CONTENT...N2...]
///           3    N1              7       N2
fn display_grep_page(
    page_results: &[UnifiedSearchResult],
    current_page: usize,
    total_pages: usize,
    max_total_width: usize
) -> io::Result<()> {
    println!("\nContent Search Results (Page {}/{}) (try: -g -r -c)", current_page, total_pages);

    // Calculate column widths based on total terminal width
    // Fixed allocations: 3 for number, 7 for line (includes spacing)
    const FIXED_WIDTH: usize = 10;  // 3 (number) + 7 (line with spaces)

    // Calculate available width for file and content columns
    let available_width = max_total_width.saturating_sub(FIXED_WIDTH);

    // Ensure minimum usable width (at least 20 chars for file+content)
    if available_width < 20 {
        // Fallback to minimum viable display
        let file_width = 10;
        let content_width = 10;
        return display_grep_page_minimal(
            page_results,
            current_page,
            total_pages,
            file_width,
            content_width,
        );
    }

    // Split available width: 60% for file, 40% for content
    let file_width = (available_width * 6) / 10;  // 60% for file
    let content_width = available_width - file_width;  // Remainder for content

    // Display header with exact column widths
    // Format: [##_][File...][_Line_][Content...]
    print!("{:<2} ", "#");  // 2 chars + 1 space = 3 total
    print!("{:<width$}", "File", width = file_width);
    print!(" {:<5} ", "Line");  // 1 + 5 + 1 = 7 total
    println!("{:<width$}", "Content", width = content_width);

    // Display separator line matching exact total width
    println!("{}", "-".repeat(max_total_width));

    // Display each result on current page
    for (index, result) in page_results.iter().enumerate() {
        if let UnifiedSearchResult::Grep(grep_result) = result {
            // Format the row number (1-indexed)
            let row_num = format!("{}", index + 1);

            // Truncate filename if needed
            let display_file = truncate_with_ellipsis(&grep_result.file_name, file_width);

            // Format line number (right-aligned in 5-char field)
            let line_str = format!("{:>5}", grep_result.line_number);

            // Prepare content - trim whitespace first
            let trimmed_content = grep_result.line_content.trim();
            let display_content = truncate_with_ellipsis(trimmed_content, content_width);

            // Display the row with exact spacing to match header
            print!("{:<2} ", row_num);  // 2 chars + 1 space
            print!("{:<width$}", display_file, width = file_width);
            print!(" {} ", line_str);  // 1 + 5 + 1 spaces
            println!("{:<width$}", display_content, width = content_width);
        }
    }

    Ok(())
}

/// Helper function to truncate strings with ellipsis
///
/// # Purpose
/// Safely truncates a string to fit within a specified width,
/// adding "..." if truncation occurs.
///
/// # Arguments
/// * `text` - The text to potentially truncate
/// * `max_width` - Maximum width in characters
///
/// # Returns
/// A String that fits within max_width, with "..." appended if truncated
fn truncate_with_ellipsis(text: &str, max_width: usize) -> String {
    if text.len() <= max_width {
        text.to_string()
    } else if max_width <= 3 {
        // If width is too small for ellipsis, just truncate
        text.chars().take(max_width).collect()
    } else {
        // Leave room for "..."
        let truncate_at = max_width - 3;
        let truncated: String = text.chars().take(truncate_at).collect();
        format!("{}...", truncated)
    }
}

/// Minimal display for very narrow terminals
///
/// # Purpose
/// Fallback display when terminal is too narrow for normal formatting
fn display_grep_page_minimal(
    page_results: &[UnifiedSearchResult],
    current_page: usize,
    total_pages: usize,
    file_width: usize,
    content_width: usize,
) -> io::Result<()> {
    println!("\nGrep Results ({}/{})", current_page, total_pages);
    println!("---");

    for (index, result) in page_results.iter().enumerate() {
        if let UnifiedSearchResult::Grep(grep_result) = result {
            println!("{}. {}:{}",
                     index + 1,
                     truncate_with_ellipsis(&grep_result.file_name, file_width),
                     grep_result.line_number);
            println!("   {}",
                     truncate_with_ellipsis(grep_result.line_content.trim(), content_width));
        }
    }

    Ok(())
}

/// Helper function to display a page of fuzzy search results
///
/// # Purpose
/// Formats and displays fuzzy (name matching) results for the current page.
/// Shows Levenshtein distance to help users understand match quality.
///
/// # Arguments
/// * `page_results` - Slice of results for current page only
/// * `current_page` - Current page number (1-indexed for display)
/// * `total_pages` - Total number of pages
/// * `max_total_width` - Total terminal width (derived from user's TUI width settings)
///
/// # Width Calculation
/// The function respects terminal width constraints:
/// - Fixed: 3 chars for number column, 10 chars for distance column (total 13)
/// - Dynamic: Remaining width for the name/path column
/// - Layout: [##_][NAME...N1...][_DISTANCE_]
///           3    N1              10
fn display_fuzzy_page(
    page_results: &[UnifiedSearchResult],
    current_page: usize,
    total_pages: usize,
    max_total_width: usize
) -> io::Result<()> {
    println!("Fuzzy Search (Page {}/{}) (try: --grep --recursive --case-sensitive, -g -r -c)",
             current_page, total_pages);

    // Calculate column widths based on total terminal width
    // Fixed allocations: 3 for number, 10 for distance (includes spacing)
    const FIXED_WIDTH: usize = 13;  // 3 (number) + 10 (distance with spaces)

    // Calculate available width for name column
    let name_width = max_total_width.saturating_sub(FIXED_WIDTH);

    // Ensure minimum usable width
    if name_width < 10 {
        // Fallback to minimal display
        return display_fuzzy_page_minimal(page_results, current_page, total_pages);
    }

    // Cap name width at reasonable maximum for readability
    let name_width = std::cmp::min(name_width, 80);

    // Display header with exact column widths
    // Format: [##_][Name...][_Distance_]
    print!("{:<2} ", "#");  // 2 chars + 1 space = 3 total
    print!("{:<width$}", "Name", width = name_width);
    println!(" {:<9}", "Distance");  // 1 + 9 = 10 total

    // Display separator line matching exact total width
    // Use the minimum of max_total_width or actual used width (for very wide terminals)
    let actual_width = 3 + name_width + 10;
    let separator_width = std::cmp::min(max_total_width, actual_width);
    println!("{}", "-".repeat(separator_width));

    // Display each result on current page
    for (index, result) in page_results.iter().enumerate() {
        if let UnifiedSearchResult::Fuzzy(fuzzy_result) = result {
            // Format the row number (1-indexed)
            let row_num = format!("{}", index + 1);

            // Truncate name if needed (reuse helper function from grep)
            let display_name = truncate_with_ellipsis(&fuzzy_result.item_name, name_width);

            // Format distance (right-aligned for better readability)
            let distance_str = format!("{:>9}", fuzzy_result.distance);

            // Display the row with exact spacing to match header
            print!("{:<2} ", row_num);  // 2 chars + 1 space
            print!("{:<width$}", display_name, width = name_width);
            println!(" {}", distance_str);  // 1 space + distance
        }
    }

    Ok(())
}

/// Minimal display for very narrow terminals (fuzzy search)
///
/// # Purpose
/// Fallback display when terminal is too narrow for normal formatting
fn display_fuzzy_page_minimal(
    page_results: &[UnifiedSearchResult],
    current_page: usize,
    total_pages: usize,
) -> io::Result<()> {
    println!("\nFuzzy Results ({}/{})", current_page, total_pages);
    println!("---");

    for (index, result) in page_results.iter().enumerate() {
        if let UnifiedSearchResult::Fuzzy(fuzzy_result) = result {
            // For minimal display, show each item on two lines
            println!("{}. {} (dist: {})",
                     index + 1,
                     truncate_with_ellipsis(&fuzzy_result.item_name, 20),
                     fuzzy_result.distance);
        }
    }

    Ok(())
}

/// Search configuration options for controlling search behavior
///
/// # Purpose
/// Encapsulates all search parameters to make the API cleaner and more extensible
///
/// # Fields
/// * `search_term` - The text to search for
/// * `recursive` - Whether to search subdirectories recursively
/// * `grep_mode` - Whether to search file contents instead of names
/// * `case_sensitive` - Whether the search should be case-sensitive (for grep mode)
///
#[derive(Debug, Clone)]
pub struct SearchConfig {
    /// The text pattern to search for
    pub search_term: String,
    /// Whether to recursively search subdirectories
    pub recursive: bool,
    /// Whether to search file contents instead of names
    pub grep_mode: bool,
    /// Whether grep search should be case-sensitive
    pub case_sensitive: bool,
}

impl SearchConfig {
    /// Creates a new SearchConfig with default settings
    ///
    /// # Purpose
    /// Initializes a search configuration with sensible defaults that work
    /// for most use cases. The defaults prioritize safety (file size limits)
    /// and user-friendliness (case-insensitive searches).
    ///
    /// # Arguments
    /// * `search_term` - The text pattern to search for. This can be:
    ///   - A partial filename for fuzzy matching
    ///   - A text pattern to find within files (grep mode)
    ///   - An empty string (though this typically returns no results)
    ///
    /// # Returns
    /// * `SearchConfig` - A new configuration instance with default settings
    ///
    /// # Default Values
    /// - `recursive`: `false` - Only searches current directory
    /// - `grep_mode`: `false` - Searches filenames, not contents
    /// - `case_sensitive`: `false` - Case-insensitive matching
    ///
    /// # Example
    /// ```rust
    /// // Create a simple filename search configuration
    /// let config = SearchConfig::new("document".to_string());
    ///
    /// // Create and customize with builder methods
    /// let config = SearchConfig::new("TODO".to_string())
    ///     .with_recursive(true)
    ///     .with_grep(true);
    /// ```
    pub fn new(search_term: String) -> Self {
        Self {
            search_term,
            recursive: false,
            grep_mode: false,
            case_sensitive: false,
        }
    }

    /// Builder method to enable or disable recursive directory traversal
    ///
    /// # Purpose
    /// Controls whether the search should traverse subdirectories or stay
    /// in the current directory only. Recursive searches are useful for
    /// finding files anywhere in a project hierarchy.
    ///
    /// # Arguments
    /// * `recursive` - `true` to search subdirectories, `false` for current directory only
    ///
    /// # Returns
    /// * `Self` - The modified configuration for method chaining
    ///
    /// # Performance Considerations
    /// Recursive searches can be slow on:
    /// - Large directory trees (many subdirectories)
    /// - Network-mounted filesystems
    /// - Directories with many files (thousands+)
    ///
    /// # Example
    /// ```rust
    /// // Search only in current directory (default)
    /// let config = SearchConfig::new("test".to_string())
    ///     .with_recursive(false);
    ///
    /// // Search current directory and all subdirectories
    /// let config = SearchConfig::new("test".to_string())
    ///     .with_recursive(true);
    /// ```
    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    /// Builder method to enable or disable grep mode (content search)
    ///
    /// # Purpose
    /// Switches between filename matching and file content searching.
    /// When enabled, the search looks for the pattern inside text files
    /// rather than in filenames.
    ///
    /// # Arguments
    /// * `grep_mode` - `true` for content search, `false` for filename search
    ///
    /// # Returns
    /// * `Self` - The modified configuration for method chaining
    ///
    /// # Grep Mode Behavior
    /// When grep mode is enabled:
    /// - Only text files are searched (binary files are skipped)
    /// - Each matching line is found and reported
    /// - Line numbers and context are provided in results
    ///
    /// # Example
    /// ```rust
    /// // Search for "TODO" in filenames (fuzzy match)
    /// let config = SearchConfig::new("TODO".to_string())
    ///     .with_grep(false);
    ///
    /// // Search for "TODO" inside file contents
    /// let config = SearchConfig::new("TODO".to_string())
    ///     .with_grep(true);
    /// ```
    pub fn with_grep(mut self, grep_mode: bool) -> Self {
        self.grep_mode = grep_mode;
        self
    }

    /// Builder method to control case sensitivity of searches
    ///
    /// # Purpose
    /// Determines whether the search should distinguish between uppercase
    /// and lowercase characters. This primarily affects grep (content) searches,
    /// as fuzzy filename searches are always case-insensitive for user convenience.
    ///
    /// # Arguments
    /// * `case_sensitive` - `true` for exact case matching, `false` to ignore case
    ///
    /// # Returns
    /// * `Self` - The modified configuration for method chaining
    ///
    /// # Behavior by Search Type
    /// - **Grep mode**: Respects this setting for content matching
    /// - **Fuzzy name mode**: Always case-insensitive regardless of this setting
    ///   (user expectation is that filename searches ignore case)
    ///
    /// # Use Cases
    /// Case-sensitive searches are useful for:
    /// - Finding specific variable names in code (e.g., "myVar" vs "myvar")
    /// - Searching for acronyms (e.g., "USA" vs "usa")
    /// - Distinguishing between similar terms with different meanings
    ///
    /// # Example
    /// ```rust
    /// // Case-insensitive grep search (default) - finds "todo", "TODO", "ToDo"
    /// let config = SearchConfig::new("todo".to_string())
    ///     .with_grep(true)
    ///     .with_case_sensitive(false);
    ///
    /// // Case-sensitive grep search - only finds exact "TODO"
    /// let config = SearchConfig::new("TODO".to_string())
    ///     .with_grep(true)
    ///     .with_case_sensitive(true);
    /// ```
    ///
    /// # Implementation Note
    /// When case_sensitive is false, both the search pattern and the searched
    /// text are converted to lowercase before comparison, ensuring consistent
    /// behavior across different platforms and locales.
    pub fn with_case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

}

#[cfg(test)]
mod tests_iterative_crawl {
    use super::*;
    use std::path::Path;
    use std::collections::HashMap;

    /// Creates a minimal NavigationState instance for testing
    fn create_test_navigation_state() -> NavigationState {
        NavigationState {
            display_lookup_table: HashMap::new(),
            current_sort_method: DirectorySortingMethodEnum::Name(true),
            last_sort_command: None,
            current_filter: None,
            selected_item_index: None,
            active_search_term: None,
            tui_tall_adjustment: 0,
            tui_tall_direction_sign: true,
            tui_wide_adjustment: 0,
            tui_wide_direction_sign: true,
            current_page_index: 0,
        }
    }

    #[test]
    fn test_collect_entries_iterative_basic() {
        // Test basic functionality - can collect entries from temp directory
        let nav_state = create_test_navigation_state();
        let temp_dir = std::env::temp_dir();

        let result = nav_state.collect_entries_iterative(
            &temp_dir,
            Some(0),  // Only the directory itself
            None,
            None,
        );

        assert!(result.is_ok(), "Should successfully read temp directory");
        let entries = result.expect("Failed to get entries");
        assert!(!entries.is_empty(), "Temp directory should have at least some entries");
    }

    #[test]
    fn test_collect_entries_iterative_depth_limit() {
        // Test that depth limit of 0 only returns immediate children
        let nav_state = create_test_navigation_state();
        let temp_dir = std::env::temp_dir();

        // Depth 0 = only immediate children of temp dir
        let result_depth_0 = nav_state.collect_entries_iterative(
            &temp_dir,
            Some(0),
            None,
            None,
        );

        assert!(result_depth_0.is_ok(), "Depth 0 should succeed");
        let entries_0 = result_depth_0.expect("Failed to get entries");

        // Depth 1 = temp dir children and their children
        let result_depth_1 = nav_state.collect_entries_iterative(
            &temp_dir,
            Some(1),
            None,
            None,
        );

        assert!(result_depth_1.is_ok(), "Depth 1 should succeed");
        let entries_1 = result_depth_1.expect("Failed to get entries");

        // Depth 1 should have at least as many entries as depth 0
        assert!(
            entries_1.len() >= entries_0.len(),
            "Depth 1 should have at least as many entries as depth 0"
        );
    }

    #[test]
    fn test_collect_entries_iterative_max_entries_limit() {
        // Test that max_entries limit works
        let nav_state = create_test_navigation_state();
        let temp_dir = std::env::temp_dir();

        // Collect with a limit of 5 entries
        let result = nav_state.collect_entries_iterative(
            &temp_dir,
            None,
            Some(5),  // Maximum 5 entries
            None,
        );

        assert!(result.is_ok(), "Should succeed with entry limit");
        let entries = result.expect("Failed to get entries");
        assert!(
            entries.len() <= 5,
            "Should not exceed 5 entries when limit is set to 5, got {}",
            entries.len()
        );
    }

    #[test]
    fn test_collect_entries_iterative_nonexistent_directory() {
        // Test handling of non-existent directory
        let nav_state = create_test_navigation_state();
        let nonexistent_path = Path::new("/this/directory/definitely/does/not/exist/anywhere/12345");

        let result = nav_state.collect_entries_iterative(
            nonexistent_path,
            None,
            None,
            None,
        );

        // Should return Ok with empty vec since we skip unreadable directories
        assert!(result.is_ok(), "Should return Ok even for non-existent directory");
        let entries = result.expect("Failed to get entries");
        assert!(entries.is_empty(), "Non-existent directory should return empty vec");
    }

    #[test]
    fn test_collect_entries_iterative_memory_limit() {
        // Test that memory limit stops collection
        let nav_state = create_test_navigation_state();
        let temp_dir = std::env::temp_dir();

        // Set a very small memory limit (1 MB)
        let result = nav_state.collect_entries_iterative(
            &temp_dir,
            None,
            None,
            Some(1),  // 1 MB limit
        );

        assert!(result.is_ok(), "Should succeed even with memory limit");
        // Just verify it completes without panic
    }

    #[test]
    fn test_collect_entries_iterative_combined_limits() {
        // Test multiple limits at once
        let nav_state = create_test_navigation_state();
        let temp_dir = std::env::temp_dir();

        let result = nav_state.collect_entries_iterative(
            &temp_dir,
            Some(1),   // Depth limit
            Some(10),  // Entry limit
            Some(5),   // Memory limit (MB)
        );

        assert!(result.is_ok(), "Should succeed with combined limits");
        let entries = result.expect("Failed to get entries");

        // Should respect the entry limit
        assert!(
            entries.len() <= 10,
            "Should not exceed 10 entries when limit is set, got {}",
            entries.len()
        );
    }

    #[test]
    fn test_collect_entries_iterative_file_attributes() {
        // Test that collected entries have proper attributes
        let nav_state = create_test_navigation_state();
        let temp_dir = std::env::temp_dir();

        let result = nav_state.collect_entries_iterative(
            &temp_dir,
            Some(0),  // Only immediate children
            Some(5),  // Limit to a few entries
            None,
        );

        assert!(result.is_ok(), "Should collect entries successfully");
        let entries = result.expect("Failed to get entries");

        // Verify each entry has valid attributes
        for entry in entries.iter() {
            // Name should not be empty
            assert!(!entry.file_system_item_name.is_empty(), "File name should not be empty");

            // Path should exist as a string representation
            assert!(entry.file_system_item_path.to_str().is_some(), "Path should be valid UTF-8");

            // Last modified time should not be UNIX_EPOCH for real files
            // (though we handle UNIX_EPOCH as fallback)
            // Just verify it doesn't panic when accessed
            let _ = entry.file_system_item_last_modified_time;

            // is_directory should be properly set
            let _ = entry.is_directory;
        }
    }
}

/// Manages navigation state, lookup tables, sort/filter settings, and TUI display preferences
///
/// This struct serves as the central state manager for the File Fantastic UI,
/// tracking display mappings, sort preferences, filter settings, search capabilities,
/// and user-customized TUI size adjustments. It maintains the connection between
/// what the user sees on screen and the underlying file system entities.
///
/// # State Management
/// - Maps displayed item numbers to actual file system items
/// - Tracks current sort method and direction
/// - Maintains filter settings (showing files/directories/all)
/// - Provides fuzzy search capability
/// - Manages TUI display size customization
///
/// # TUI Size Customization
/// The TUI size adjustment system allows users to manually control display dimensions:
/// - **Height (tall)**: Controls number of items shown per page
/// - **Width (wide)**: Controls the width of the filename column
/// These settings persist throughout the session and across directory navigation.
///
/// # Lifecycle
/// The NavigationState is created once at application start and persists
/// throughout the session, being updated as the user navigates, sorts,
/// filters, searches, or adjusts display size preferences.
///
/// # Key Responsibilities
/// 1. **Display Mapping**: Maps display numbers (what user sees) to actual file paths
/// 2. **Sort Management**: Tracks and toggles sort methods/directions
/// 3. **Filter Application**: Applies file/directory filters to listings
/// 4. **Search Functionality**: Performs fuzzy text searches
/// 5. **TUI Size Management**: Maintains user's display size preferences
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
/// - TUI adjustments use u16 for memory efficiency on low-resource systems
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

    /// Manual TUI height adjustment magnitude (always positive value)
    /// Represents how many rows to add or remove from the default display
    /// The actual direction is determined by tui_tall_direction_sign
    /// Example: 5 with sign=true means "tall+5" (5 more rows than default)
    tui_tall_adjustment: u16,

    /// Direction of height adjustment
    /// true = positive adjustment (add rows), false = negative adjustment (remove rows)
    /// This allows representing both "tall+N" and "tall-N" adjustments
    tui_tall_direction_sign: bool,

    /// Manual TUI width adjustment magnitude (always positive value)
    /// Represents how many characters to add or remove from the name column width
    /// The actual direction is determined by tui_wide_direction_sign
    /// Example: 10 with sign=false means "wide-10" (10 fewer characters for names)
    tui_wide_adjustment: u16,

    /// Direction of width adjustment
    /// true = positive adjustment (add characters), false = negative adjustment (remove characters)
    /// This allows representing both "wide+N" and "wide-N" adjustments
    tui_wide_direction_sign: bool,

    /// Current page index (0-based) - what page we're currently viewing
    current_page_index: usize,
}

fn detect_android() -> bool {
    env::var("ANDROID_ROOT").is_ok()
}

impl NavigationState {
    /// Creates a new NavigationState with default settings
    ///
    /// # Platform-Specific Defaults
    /// - **Android devices**: Automatically applies a width reduction of 23 characters
    ///   (tui_wide_adjustment = 23, tui_wide_direction_sign = false) to accommodate
    ///   typical Android terminal constraints
    /// - **Non-Android platforms**: No width adjustment (tui_wide_adjustment = 0)
    ///
    /// Android detection is performed by checking for the ANDROID_ROOT environment variable.
    /// # Returns
    /// * `NavigationState` - A new instance with:
    ///   - Empty lookup table
    ///   - Name sort in ascending order as default
    ///   - No last sort command
    ///   - No active filters
    ///   - Default TUI size (no adjustments)
    ///
    /// # Default Configuration
    /// - Sort by name in ascending order
    /// - Empty lookup table (populated after directory read)
    /// - No last sort command recorded
    /// - No filter applied (show all items)
    /// - TUI adjustments set to 0 (use default sizes)
    /// - Direction signs set to true (positive) by default
    ///
    /// # TUI Size Defaults
    /// When adjustments are 0, the display uses:
    /// - Default name width: 55 characters
    /// - Default items per page: 16 items
    ///
    /// # Example
    /// ```
    /// let nav_state = NavigationState::new();
    /// // nav_state is ready to be used with initial directory read
    /// // Display will use default sizes until user adjusts
    /// ```
    fn new() -> Self {

        // Detect if running on Android platform
        let is_android = detect_android();

        // Set width adjustment based on platform
        // Android terminals typically need reduced width
        let (width_adjustment, width_direction) = if is_android {
            // Android: reduce width by 24 characters
            (24, false)  // false represents negative direction
        } else {
            // Non-Android: no adjustment needed
            (0, true)    // true represents positive direction (though 0 makes direction irrelevant)
        };

        NavigationState {
            display_lookup_table: HashMap::new(),
            current_sort_method: DirectorySortingMethodEnum::Name(true),
            last_sort_command: None,
            current_filter: None, // No filter initially
            selected_item_index: None,
            active_search_term: None,
            // Initialize TUI size adjustments to defaults (no adjustment)
            tui_tall_adjustment: 0,      // No height adjustment
            tui_tall_direction_sign: true, // Positive direction by default
            tui_wide_adjustment: width_adjustment,       // No width adjustment
            tui_wide_direction_sign: width_direction, // Positive direction by default
            current_page_index: 0,        // Always start at page 0
        }
    }

    /// Resets navigation state to clean defaults while preserving location and sort
    ///
    /// # Purpose
    /// Clears filters, pagination, selection, and search state to provide a "clean slate"
    /// view of the current directory while preserving the user's preferred sort method
    /// and current location.
    ///
    /// # State Reset
    /// - Clears any active filters (show all files and directories)
    /// - Resets to first page (page 0)
    /// - Clears any selected item
    /// - Clears any active search term
    /// - Preserves: current sort method, current directory location
    ///
    /// # Usage Context
    /// Called when user presses Enter with no input to "refresh" and clear all
    /// applied filters, searches, and navigation state while staying in the
    /// same directory with the same sort order.
    ///
    /// # Example
    /// ```rust
    /// // User has filtered to files only, on page 3, with item 5 selected
    /// // User presses Enter to reset
    /// nav_state.reset_to_clean_state();
    /// // Now: no filter, page 1, no selection, no search, same sort method
    /// ```
    fn reset_to_clean_state(&mut self) {
        // Clear filter (show all items)
        self.current_filter = None;

        // Reset to first page
        self.current_page_index = 0;

        // Clear any selected item
        self.selected_item_index = None;

        // Clear any active search
        self.active_search_term = None;

        // Note: We preserve current_sort_method and last_sort_command
        // because users typically want to keep their preferred sort order
        // when refreshing/resetting the view
    }

    /// Master wrapper function that handles all search routing and renumbers results
    ///
    /// # Purpose
    /// Takes raw user input, parses flags, routes to appropriate search function,
    /// and renumbers results for proper user selection. This function serves as
    /// the central dispatcher for all search operations.
    ///
    /// # Arguments
    /// * `raw_input` - The raw string input from user including any flags
    ///                 (e.g., "document", "document -r", "TODO --grep")
    /// * `current_dir_entries` - The entries in current directory to search if not recursive
    /// * `current_navigation_path` - The directory the user has navigated to
    ///
    /// # Returns
    /// * `Vec<UnifiedSearchResult>` - Search results wrapped in appropriate enum variant
    ///                                 with display_index renumbered for selection
    ///
    /// # Important: Display Index Renumbering
    /// For recursive and grep searches, the original display_index values are
    /// meaningless since they refer to positions in different directories or
    /// are duplicated for multiple matches. This function renumbers them
    /// sequentially (1, 2, 3...) to match what the user sees in the display.
    ///
    /// # Search Flow
    /// 1. Parse input string for search term and flags (-r, --grep, -c)
    /// 2. Collect appropriate file list based on recursive flag
    /// 3. Route to fuzzy name search or grep content search based on grep flag
    /// 4. Wrap results in appropriate enum variant
    /// 5. Renumber results for proper selection if recursive or grep
    /// 6. Return unified results
    ///
    /// # Type Safety
    /// The returned enum clearly indicates which type of search was performed,
    /// eliminating the need for a separate boolean flag and preventing
    /// confusion between search types.
    pub fn fuzzy_search_manager_wrapper(
        &self,
        raw_input: &str,
        current_dir_entries: &[FileSystemEntry],
        current_navigation_path: &Path,
    ) -> Vec<UnifiedSearchResult> {  // Note: No longer returns tuple with bool

        // Step 1: Parse the raw input for search term and flags
        let (search_term, recursive, grep, case_sensitive) = parse_input_flags(raw_input);

        // Early return for empty search term
        if search_term.is_empty() {
            return Vec::new();
        }

        // Step 2: Get the appropriate file list based on recursive flag
        let entries = if recursive {
            // Collect files recursively from navigation directory
            match self.collect_entries_iterative(
                current_navigation_path,
                Some(20),      // Maximum depth of 20 levels (prevents infinite loops, covers 99.9% of real use cases)
                Some(100_000), // Maximum 100,000 entries (prevents UI freezing, reasonable for display/search)
                Some(500),     // Maximum 500MB memory usage (safe for systems with 4GB+ RAM)
            ) {
                Ok(entries) => entries,
                Err(_) => return Vec::new(),
            }
        } else {
            // Use the provided current directory entries
            current_dir_entries.to_vec()
        };

        // Step 3: Route to appropriate search function and wrap in enum
        let mut results: Vec<UnifiedSearchResult> = if grep {
            // Route to grep content search
            let config = SearchConfig::new(search_term.to_string())
                .with_recursive(recursive)
                .with_grep(true)
                .with_case_sensitive(case_sensitive);

            // Get grep results
            match self.grep_search_files(&config, &entries) {
                Ok(mut grep_results) => {
                    // CRITICAL: Deduplicate grep results before wrapping!
                    // This consolidates multiple matches per file into one entry
                    grep_results = Self::deduplicate_grep_results(grep_results);

                    // Now wrap deduplicated results in enum
                    grep_results.into_iter()
                        .map(|r| UnifiedSearchResult::Grep(r))
                        .collect()
                }
                Err(_) => Vec::new(),
            }
        } else {
            // Route to fuzzy name search
            let config = SearchConfig::new(search_term.to_string())
                .with_recursive(recursive)
                .with_grep(false);

            // Get fuzzy results and wrap each in the enum
            let fuzzy_results = self.fuzzy_search_entries(&config, &entries);
            fuzzy_results.into_iter()
                .map(|r| UnifiedSearchResult::Fuzzy(r))
                .collect()
        };

        // Step 4: Renumber display indices for recursive or grep searches
        // This is critical for user selection to work correctly
        if recursive || grep {
            // Renumber all results sequentially
            for (idx, result) in results.iter_mut().enumerate() {
                // Pattern match to access the display_index field
                match result {
                    UnifiedSearchResult::Grep(grep_result) => {
                        grep_result.display_index = idx + 1;
                    }
                    UnifiedSearchResult::Fuzzy(fuzzy_result) => {
                        fuzzy_result.display_index = idx + 1;
                    }
                }
            }
        }
        // For non-recursive fuzzy search, keep original display_index values
        // since they correspond to the actual position in current directory

        results
    }

    /// Deduplicates grep results to show only one entry per file
    ///
    /// # Purpose
    /// Since grep_search_files creates one SearchResult per matching line,
    /// we need to consolidate these into one result per file for selection.
    /// This function keeps the first match for each file and aggregates
    /// the match contexts.
    ///
    /// # Arguments
    /// * `results` - Vector of SearchResult items from grep search
    ///
    /// # Returns
    /// * `Vec<SearchResult>` - Deduplicated results with one entry per file
    ///
    /// # Implementation Details
    /// - Groups results by file path
    /// - Keeps the first result for each file
    /// - Combines match contexts from multiple matches
    /// - Preserves line number information for display
    ///
    /// # Example
    /// If a file has 3 matching lines, grep_search_files creates 3 SearchResults.
    /// This function consolidates them into 1 SearchResult with aggregated context.
    fn deduplicate_grep_results(results: Vec<GrepSearchResult>) -> Vec<GrepSearchResult> {
        use std::collections::HashMap;

        // Group results by file path
        let mut file_map: HashMap<PathBuf, Vec<GrepSearchResult>> = HashMap::new();

        for result in results {
            file_map
                .entry(result.file_path.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }

        // Create one result per file
        let mut deduplicated: Vec<GrepSearchResult> = Vec::new();

        for (_path, matches) in file_map {
            if let Some(first_match) = matches.into_iter().next() {
                // Keep the first match as the representative
                // The display function will show all matches when needed
                deduplicated.push(first_match);
            }
        }

        // Sort by file name for consistent ordering
        deduplicated.sort_by(|a, b| a.file_name.cmp(&b.file_name));

        deduplicated
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

    /// Iteratively collects file system entries from directory and all subdirectories with optimization controls
    ///
    /// # Purpose
    /// Traverses a directory tree starting from the specified root directory using an iterative
    /// breadth-first search algorithm. This approach avoids stack overflow issues that can occur
    /// with deep recursion and provides better memory control and traversal limits.
    ///
    /// # Algorithm Details
    /// - Uses `VecDeque` for O(1) push/pop operations (more efficient than Vec for queue operations)
    /// - Implements breadth-first traversal for better cache locality and predictable memory patterns
    /// - Provides optional depth, entry count, and memory limits for controlled traversal
    /// - Tracks approximate memory usage to prevent out-of-memory conditions
    ///
    /// # Critical Implementation Note
    /// This function MUST receive the actual directory to search, not assume any particular directory.
    /// The caller is responsible for providing the correct starting directory based on the
    /// application's navigation state.
    ///
    /// # Arguments
    /// * `start_directory` - The root directory from which to start collecting.
    ///                       This should be the user's current navigation location,
    ///                       NOT the process working directory.
    /// * `max_depth` - Optional maximum depth to traverse:
    ///                 - `None` = unlimited depth
    ///                 - `Some(0)` = only the start directory itself
    ///                 - `Some(1)` = start directory and its immediate children
    ///                 - `Some(2)` = start directory, children, and grandchildren, etc.
    /// * `max_entries` - Optional maximum number of entries to collect before stopping
    /// * `max_memory_mb` - Optional approximate memory limit in megabytes
    ///
    /// # Returns
    /// * `Ok(Vec<FileSystemEntry>)` - Successfully collected entries (may be limited by constraints)
    /// * `Err(Error)` - Failed to access the start directory
    ///
    /// # Common Mistake
    /// DO NOT use std::env::current_dir() here or in calling code.
    /// The process working directory is NOT the same as the file manager's
    /// current navigation directory.
    ///
    /// # Memory Characteristics
    /// - Memory usage: O(width of directory tree) for the queue in worst case
    /// - Processing order: breadth-first (all siblings before children)
    /// - Early termination: stops immediately when any limit is reached
    ///
    /// # Performance Notes
    /// - Breadth-first traversal provides better cache locality for file system metadata
    /// - VecDeque provides amortized O(1) operations for queue management
    /// - Memory estimation helps prevent out-of-memory conditions on large directory trees
    pub fn collect_entries_iterative(
        &self,
        start_directory: &Path,
        max_depth: Option<usize>,
        max_entries: Option<usize>,
        max_memory_mb: Option<usize>,
    ) -> Result<Vec<FileSystemEntry>> {
        // Initialize the collection for results
        let mut all_entries = Vec::new();

        // Initialize the breadth-first search queue with (directory_path, depth_level) tuples
        // Depth 0 represents the start directory itself
        let mut directories_queue: VecDeque<(PathBuf, usize)> = VecDeque::new();

        // Begin traversal from the start directory at depth 0
        directories_queue.push_back((start_directory.to_path_buf(), 0));

        // Memory usage tracking
        // These are rough estimates based on typical file system entry sizes
        const ENTRY_SIZE_ESTIMATE_BYTES: usize = 256;  // Approximate size of one FileSystemEntry in memory
        const QUEUE_ITEM_SIZE_ESTIMATE_BYTES: usize = 128;  // Approximate size of one (PathBuf, usize) tuple
        let mut estimated_memory_bytes: usize = 0;

        // Progress tracking for diagnostics
        let mut directories_processed: usize = 0;
        let mut directories_skipped: usize = 0;

        // Main traversal loop - continues until queue is empty or limits are reached
        while let Some((current_dir, current_depth)) = directories_queue.pop_front() {
            directories_processed += 1;

            // Update memory estimate by removing the dequeued item
            estimated_memory_bytes = estimated_memory_bytes.saturating_sub(QUEUE_ITEM_SIZE_ESTIMATE_BYTES);

            // Check depth limit before processing this directory
            // If max_depth is 2, we process depths 0, 1, and 2 (three levels total)
            if let Some(max_d) = max_depth {
                if current_depth > max_d {
                    // Skip this directory as it exceeds depth limit
                    directories_skipped += 1;
                    continue;
                }
            }

            // Check if we've reached the maximum number of entries
            if let Some(max_e) = max_entries {
                if all_entries.len() >= max_e {
                    // Stop traversal - we've collected enough entries
                    break;
                }
            }

            // Check memory limit before processing more entries
            if let Some(max_mem) = max_memory_mb {
                let max_bytes = max_mem.saturating_mul(1024).saturating_mul(1024);
                if estimated_memory_bytes > max_bytes {
                    // Stop traversal - approaching memory limit
                    // Note: This is an estimate, actual memory usage may vary
                    eprintln!("Warning: Estimated memory usage ({} MB) exceeds limit ({} MB), stopping traversal",
                             estimated_memory_bytes / (1024 * 1024), max_mem);
                    break;
                }
            }

            // Attempt to read the directory contents
            let dir_entries = match fs::read_dir(&current_dir) {
                Ok(entries) => entries,
                Err(e) => {
                    // Log the error but continue processing other directories
                    // This handles permission denied, symbolic link issues, etc.
                    eprintln!("Warning: Cannot read directory '{}': {}",
                             current_dir.display(), e);
                    directories_skipped += 1;
                    continue;
                }
            };

            // Process each entry in the current directory
            for entry_result in dir_entries {
                // Check entry limit again (may have been added by other iterations)
                if let Some(max_e) = max_entries {
                    if all_entries.len() >= max_e {
                        break;
                    }
                }

                // Handle potential I/O errors when reading directory entry
                let entry = match entry_result {
                    Ok(e) => e,
                    Err(_) => {
                        // Skip entries we can't read
                        continue;
                    }
                };

                // Retrieve file metadata (size, modification time, type)
                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => {
                        // Skip if we can't read metadata
                        continue;
                    }
                };

                // Convert OsString to String, skipping entries with invalid UTF-8
                let file_name = match entry.file_name().into_string() {
                    Ok(name) => name,
                    Err(_) => {
                        // Skip files with non-UTF-8 names
                        continue;
                    }
                };

                let path = entry.path();
                let is_directory = metadata.is_dir();

                // Create the file system entry structure
                all_entries.push(FileSystemEntry {
                    file_system_item_name: file_name,
                    file_system_item_path: path.clone(),
                    file_system_item_size_in_bytes: metadata.len(),
                    file_system_item_last_modified_time: metadata.modified()
                        .unwrap_or(SystemTime::UNIX_EPOCH),
                    is_directory,
                });

                // Update memory estimate after adding entry
                estimated_memory_bytes = estimated_memory_bytes.saturating_add(ENTRY_SIZE_ESTIMATE_BYTES);

                // Queue subdirectories for future processing (breadth-first)
                if is_directory {
                    // Only queue subdirectories if:
                    // 1. We haven't reached max_entries limit
                    // 2. The subdirectory's depth would not exceed max_depth
                    // 3. We haven't exceeded memory limits

                    let should_queue_subdirectory = {
                        // Check if we're at capacity for entries
                        let within_entry_limit = match max_entries {
                            Some(max_e) => all_entries.len() < max_e,
                            None => true,
                        };

                        // Check if subdirectory would exceed depth limit
                        // Subdirectories would be at depth current_depth + 1
                        let within_depth_limit = match max_depth {
                            Some(max_d) => current_depth < max_d,  // current_depth + 1 <= max_d
                            None => true,
                        };

                        // Check if we have memory budget for more queue items
                        let within_memory_limit = match max_memory_mb {
                            Some(max_mem) => {
                                let max_bytes = max_mem.saturating_mul(1024).saturating_mul(1024);
                                estimated_memory_bytes.saturating_add(QUEUE_ITEM_SIZE_ESTIMATE_BYTES) <= max_bytes
                            },
                            None => true,
                        };

                        within_entry_limit && within_depth_limit && within_memory_limit
                    };

                    if should_queue_subdirectory {
                        directories_queue.push_back((path, current_depth + 1));
                        estimated_memory_bytes = estimated_memory_bytes.saturating_add(QUEUE_ITEM_SIZE_ESTIMATE_BYTES);
                    }
                }
            }

            // Provide progress feedback for very large traversals (every 1000 directories)
            if directories_processed % 1000 == 0 && directories_processed > 0 {
                eprintln!("Progress: Processed {} directories, {} entries collected, {} directories queued",
                         directories_processed,
                         all_entries.len(),
                         directories_queue.len());
            }
        }

        // Log final statistics if any directories were skipped
        if directories_skipped > 0 {
            eprintln!("Traversal complete: {} directories processed, {} skipped, {} entries collected",
                     directories_processed, directories_skipped, all_entries.len());
        }

        Ok(all_entries)
    }

    /// Performs fuzzy search on collected entries using Levenshtein distance
    ///
    /// # Purpose
    /// Searches through file system entries to find items with names similar to
    /// the search term. This function provides typo-tolerant searching, allowing
    /// users to find files even with minor spelling mistakes.
    ///
    /// # Arguments
    /// * `config` - Search configuration containing:
    ///   - `search_term`: The pattern to match against (required, non-empty)
    ///   - Other fields (recursive, grep_mode, case_sensitive) are not used here
    /// * `entries` - File system entries to search through (files and directories)
    ///
    /// # Returns
    /// * `Vec<FuzzySearchResult>` - Matching entries sorted by relevance with:
    ///   - Best matches (lowest distance) first
    ///   - Among equal distances, shorter names first
    ///   - Sequential display indices (1, 2, 3...) for user selection
    ///
    /// # Output Format Requirements
    ///
    /// ## Critical Display Index Contract
    /// The `display_index` field in each result MUST be sequential starting from 1.
    /// This is an absolute requirement for the user interface to function correctly.
    ///
    /// ### Why This Matters
    /// The application presents search results to users as a numbered list:
    /// ```text
    /// Search results for "doc":
    /// 1. document.txt
    /// 2. docker-compose.yml
    /// 3. docs/
    /// ```
    ///
    /// When the user types "2" to select an item, the system looks for the result
    /// with `display_index == 2`. If display indices are not sequential (e.g., 3, 7, 15),
    /// the user cannot select items reliably.
    ///
    /// ### Incorrect Implementation (DO NOT DO THIS)
    /// ```rust
    /// // WRONG: Using enumeration index from original entries
    /// for (idx, entry) in entries.iter().enumerate() {
    ///     if matches_search {
    ///         results.push(FuzzySearchResult {
    ///             display_index: idx + 1,  // WRONG! This is position in entries
    ///             ...
    ///         });
    ///     }
    /// }
    /// ```
    ///
    /// This fails because if entries[5] and entries[10] match, they get
    /// display_index 6 and 11, but the user sees them as items 1 and 2.
    ///
    /// ### Correct Implementation (ALWAYS DO THIS)
    /// ```rust
    /// // Step 1: Collect all matches with placeholder indices
    /// for entry in entries.iter() {
    ///     if matches_search {
    ///         results.push(FuzzySearchResult {
    ///             display_index: 0,  // Temporary placeholder
    ///             ...
    ///         });
    ///     }
    /// }
    ///
    /// // Step 2: Sort by relevance
    /// results.sort_by(|a, b| { ... });
    ///
    /// // Step 3: Assign sequential indices after sorting
    /// for (idx, result) in results.iter_mut().enumerate() {
    ///     result.display_index = idx + 1;  // Sequential: 1, 2, 3...
    /// }
    /// ```
    ///
    /// ## Uniformity Across Search Types
    ///
    /// This function is part of a larger search system that includes:
    /// - **Fuzzy search** (this function): Matches file/directory names
    /// - **Grep search**: Matches file contents
    /// - **Recursive search**: Searches in subdirectories
    /// - **Non-recursive search**: Searches current directory only
    ///
    /// ALL search types MUST produce the same output format:
    /// - Sequential display indices starting from 1
    /// - Sorted by relevance (best matches first)
    /// - Consistent field population (no Option types for required fields)
    ///
    /// ### Integration with UnifiedSearchResult
    /// The results from this function are wrapped in `UnifiedSearchResult::Fuzzy()`
    /// by the calling function. The wrapper function relies on the display indices
    /// being correct and sequential. For recursive searches, the wrapper may
    /// renumber results, but for non-recursive searches, it uses these indices
    /// directly.
    ///
    /// ## Testing the Output Format
    ///
    /// To verify correct implementation, check that:
    /// 1. First result has `display_index == 1`
    /// 2. Each subsequent result increments by exactly 1
    /// 3. No gaps in the sequence
    /// 4. No duplicate indices
    ///
    /// ```rust
    /// #[test]
    /// fn test_display_indices_are_sequential() {
    ///     let results = fuzzy_search_entries(&config, &entries);
    ///     for (i, result) in results.iter().enumerate() {
    ///         assert_eq!(result.display_index, i + 1,
    ///                    "Display index must be sequential");
    ///     }
    /// }
    /// ```
    ///
    /// ## Common Pitfalls to Avoid
    ///
    /// 1. **Don't use original array indices**: The position in the input array
    ///    is not relevant to the user-facing display.
    ///
    /// 2. **Don't assign indices before sorting**: The display order must match
    ///    the relevance order.
    ///
    /// 3. **Don't skip numbers**: Even if you filter out some results after
    ///    initial collection, renumber to maintain sequence.
    ///
    /// 4. **Don't start from 0**: User-facing indices conventionally start at 1.
    ///
    /// # Algorithm Details
    /// 1. **Name Preparation**: For each entry, creates two versions:
    ///    - Full name with extension
    ///    - Name without extension (for files like "document.txt" vs "document")
    /// 2. **Truncation**: Both versions are truncated to search term length
    ///    - This ensures fair comparison (searching "doc" against "document")
    /// 3. **Distance Calculation**: Computes Levenshtein distance for both versions
    /// 4. **Selection**: Uses the better (lower) distance of the two
    /// 5. **Filtering**: Only includes results within MAX_SEARCH_DISTANCE threshold
    /// 6. **Sorting**: Orders by distance, then by name length for equal distances
    /// 7. **Numbering**: Assigns sequential display indices after sorting
    ///
    /// # Display Index Assignment
    /// Display indices are assigned AFTER filtering and sorting to ensure they
    /// match what the user sees on screen. This creates a uniform output format
    /// across all search types (fuzzy, grep, recursive, non-recursive).
    ///
    /// For example, if 3 matches are found:
    /// - First result gets display_index: 1
    /// - Second result gets display_index: 2
    /// - Third result gets display_index: 3
    ///
    /// This allows consistent user selection regardless of search type.
    ///
    /// # Performance Characteristics
    /// - O(n * m) where n is number of entries, m is search term length
    /// - Early termination when distance exceeds MAX_SEARCH_DISTANCE
    /// - Memory usage: O(k) where k is number of matches
    ///
    /// # Edge Cases
    /// - Empty search term: Returns empty vector immediately
    /// - No matches: Returns empty vector
    /// - Files without extensions: Treated as having no extension
    /// - Unicode handling: Correctly counts UTF-8 characters, not bytes
    ///
    /// # Example
    /// ```rust
    /// let config = SearchConfig::new("ducment".to_string()); // Note the typo
    /// let results = nav_state.fuzzy_search_entries(&config, &entries);
    /// // Might find "document.txt" with distance 1 (one character substitution)
    /// // Results will have display_index values: 1, 2, 3, etc.
    /// ```
    fn fuzzy_search_entries(
        &self,
        config: &SearchConfig,
        entries: &[FileSystemEntry],
    ) -> Vec<FuzzySearchResult> {
        // Early return for empty search term to avoid unnecessary processing
        if config.search_term.is_empty() {
            return Vec::new();
        }

        // Initialize results vector - will hold all matches before sorting
        let mut results = Vec::new();

        // Prepare search term for comparison (lowercase for case-insensitive matching)
        let search_term = config.search_term.to_lowercase();

        // Count characters (not bytes) for proper UTF-8 handling
        // This ensures "café" has length 4, not 5
        let search_len = search_term.chars().count();

        // Iterate through all provided entries to find matches
        // Note: We don't use enumerate's index here since we'll assign
        // display indices after sorting for uniform output format
        for entry in entries.iter() {
            // Step 1: Create name without extension for flexible matching
            // This allows "doc" to match both "document" and "document.txt"
            let name_without_ext = match entry.file_system_item_name.rsplit_once('.') {
                Some((name, _ext)) => name.to_string(),
                None => entry.file_system_item_name.clone(), // No extension found
            };

            // Step 2: Create truncated versions for fair comparison
            // We truncate to search term length to avoid penalizing longer names
            // Example: searching "doc" against "document" compares "doc" vs "doc"
            let full_name_truncated: String = entry.file_system_item_name
                .to_lowercase()
                .chars()          // Use chars() for proper UTF-8 handling
                .take(search_len) // Take only as many characters as search term
                .collect();

            let no_ext_truncated: String = name_without_ext
                .to_lowercase()
                .chars()
                .take(search_len)
                .collect();

            // Step 3: Calculate Levenshtein distances for both versions
            // This measures how many single-character edits are needed
            let distance_with_ext = levenshtein_distance(&full_name_truncated, &search_term);
            let distance_without_ext = levenshtein_distance(&no_ext_truncated, &search_term);

            // Step 4: Use the better (lower) distance
            // This gives the user the best possible match
            let distance = distance_with_ext.min(distance_without_ext);

            // Step 5: Filter by maximum acceptable distance
            // MAX_SEARCH_DISTANCE prevents irrelevant results from appearing
            if distance <= MAX_SEARCH_DISTANCE {
                // Add to results with placeholder display_index
                // We'll assign the real display_index after sorting
                results.push(FuzzySearchResult {
                    item_name: entry.file_system_item_name.clone(),
                    item_path: entry.file_system_item_path.clone(),
                    distance,
                    display_index: 0, // Placeholder - will be set after sorting
                });
            }
        }

        // Step 6: Sort results by relevance
        // Primary sort: by distance (best matches first)
        // Secondary sort: by name length (shorter names first for equal distances)
        // This ensures "doc.txt" appears before "document.txt" when both have distance 0
        results.sort_by(|a, b| {
            match a.distance.cmp(&b.distance) {
                std::cmp::Ordering::Equal => {
                    // For equal distances, prefer shorter names
                    // This assumes shorter names are more likely what user wanted
                    a.item_name.len().cmp(&b.item_name.len())
                },
                other => other // Different distances - use distance ordering
            }
        });

        // Step 7: Assign sequential display indices after sorting
        // This ensures display indices match what user sees on screen (1, 2, 3...)
        // This is CRITICAL for maintaining uniform output format across all search types
        for (idx, result) in results.iter_mut().enumerate() {
            result.display_index = idx + 1; // User-facing indices start at 1
        }

        results
    }

    /// Searches file contents for a pattern using memory-efficient line-by-line reading
    ///
    /// # Purpose
    /// Performs grep-like content searching across multiple files without loading
    /// entire files into memory. This prevents memory exhaustion and maintains
    /// responsive performance even with large files.
    ///
    /// # Arguments
    /// * `config` - Search configuration containing:
    ///   - `search_term`: The pattern to search for
    ///   - `case_sensitive`: Whether to match case exactly
    ///   - Other fields (recursive, grep_mode) are not used here
    /// * `entries` - File system entries to search through
    ///
    /// # Returns
    /// * `Result<Vec<SearchResult>, FileFantasticError>` - Vector of matches or error
    ///
    /// # Memory Efficiency Strategy
    /// - Uses `BufReader` to read files line by line
    /// - Only keeps one line in memory at a time
    /// - Typical memory usage: ~8KB buffer + current line
    /// - Can handle files of any size without memory issues
    ///
    /// # Search Behavior
    /// - Skips directories (only searches regular files)
    /// - Skips non-plaintext files based on extension check
    /// - Skips binary files (detected by read errors or null bytes)
    /// - Limits results to MAX_MATCHES_PER_FILE per file to prevent flooding
    /// - Case-insensitive by default (controlled by config.case_sensitive)
    ///
    /// # Error Handling
    /// - Files that cannot be opened are silently skipped
    /// - Read errors (often from binary files) cause file to be skipped
    /// - Continues searching other files even if some fail
    ///
    /// # Performance Characteristics
    /// - O(1) extension check before file open (HashSet lookup)
    /// - O(n) where n is total characters in searched files
    /// - Early exit after MAX_MATCHES_PER_FILE matches per file
    /// - No file size limits needed due to streaming approach
    ///
    /// # Implementation Details
    /// The function uses a line counter to track location of matches and
    /// limits matches per file to prevent overwhelming the user with results
    /// from files with many matches (like log files with repeated patterns).
    ///
    /// # Example
    /// ```rust
    /// let config = SearchConfig::new("TODO".to_string())
    ///     .with_grep(true)
    ///     .with_case_sensitive(false);
    ///
    /// let results = nav_state.grep_search_files(&config, &entries)?;
    /// // Results contain file path, line number, and context for each match
    /// ```
    fn grep_search_files(
        &self,
        config: &SearchConfig,
        entries: &[FileSystemEntry],
    ) -> Result<Vec<GrepSearchResult>> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let mut results = Vec::new();

        // Prepare search pattern based on case sensitivity setting
        let search_pattern = if config.case_sensitive {
            config.search_term.clone()
        } else {
            config.search_term.to_lowercase()
        };

        // Limit matches per file to prevent result flooding
        // This prevents files with hundreds of matches from overwhelming the display
        const MAX_MATCHES_PER_FILE: usize = 10;

        // Iterate through all provided file system entries
        for (idx, entry) in entries.iter().enumerate() {
            // Skip directories - we only search file contents
            if entry.is_directory {
                continue;
            }

            // Check if file is plaintext based on extension
            // This saves I/O operations by not attempting to open binary files
            if !is_plaintext_file(&entry.file_system_item_path.to_string_lossy()) {
                continue;
            }

            // Attempt to open the file, skip if we can't access it
            // Common reasons for failure: permissions, file deleted, symlink broken
            let file = match File::open(&entry.file_system_item_path) {
                Ok(f) => f,
                Err(_) => continue, // Silently skip inaccessible files
            };

            // Wrap file in BufReader for efficient line-by-line reading
            // BufReader uses an 8KB buffer by default, reading ahead for performance
            let reader = BufReader::new(file);

            // Track current line number for reporting match locations
            let mut line_number = 0;

            // Count matches in this file to enforce MAX_MATCHES_PER_FILE limit
            let mut matches_found = 0;

            // Process file line by line - memory efficient approach
            for line_result in reader.lines() {
                // Increment line counter before processing
                line_number += 1;

                // Check if we've hit the match limit for this file
                // This check MUST be before processing to ensure matches_found is read
                if matches_found >= MAX_MATCHES_PER_FILE {
                    // Stop searching this file, move to next file
                    break;
                }

                // Attempt to read the line, handle errors
                let line = match line_result {
                    Ok(l) => l,
                    Err(_) => {
                        // Read error often indicates binary file
                        // Stop processing this file
                        break;
                    }
                };

                // Additional binary file detection - check for null bytes
                // Text files should not contain null bytes
                if line.chars().any(|c| c == '\0') {
                    // Binary file detected, skip rest of file
                    break;
                }

                // Prepare line for comparison based on case sensitivity
                let line_to_search = if config.case_sensitive {
                    line.clone()
                } else {
                    line.to_lowercase()
                };

                // Check if this line contains our search pattern
                if line_to_search.contains(&search_pattern) {
                    // Found a match - increment counter
                    matches_found += 1;

                    // Truncate very long lines for display purposes
                    // This prevents the display from being broken by extremely long lines
                    // Use chars().take() to ensure we truncate at character boundaries,
                    // not byte boundaries (which could split multi-byte UTF-8 characters)
                    const MAX_DISPLAY_CHARS: usize = 97;

                    let context = if line.chars().count() > 100 {
                        // Take first 97 characters and append ellipsis
                        // This handles multi-byte UTF-8 characters correctly
                        let truncated: String = line.chars()
                            .take(MAX_DISPLAY_CHARS)
                            .collect();
                        format!("{}...", truncated)
                    } else {
                        line.clone()
                    };

                    // Create search result for this match
                    results.push(GrepSearchResult {
                        file_name: entry.file_system_item_name.clone(),
                        file_path: entry.file_system_item_path.clone(),
                        line_number: line_number,  // Direct field, not Option
                        line_content: context,      // Direct field, not Option
                        display_index: idx + 1,
                    });
                }
            }
            // File automatically closed when `file` and `reader` go out of scope
        }

        Ok(results)
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
mod tests_formatting {
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
        // let last_month = now - Duration::from_secs(30 * 24 * 60 * 60);

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

    #[test]
    fn test_archive_timestamp_format() {
        // Test known epoch time
        let epoch = UNIX_EPOCH;
        let timestamp = create_archive_timestamp(epoch);
        assert_eq!(timestamp, "70_01_01_00_00_00");
    }

    #[test]
    fn test_leap_year_calculation() {
        // Test leap year handling
        // February 29, 2024 (leap year)
        // 2024-02-29 12:00:00 UTC = 1709208000 seconds since epoch
        let leap_day = UNIX_EPOCH + Duration::from_secs(1709208000);
        let timestamp = create_archive_timestamp(leap_day);
        assert_eq!(timestamp, "24_02_29_12_00_00");
    }

    #[test]
    fn test_year_2000_problem() {
        // Test year 2000 (was a leap year)
        // 2000-02-29 00:00:00 UTC = 951782400 seconds since epoch
        let y2k_leap = UNIX_EPOCH + Duration::from_secs(951782400);
        let timestamp = create_archive_timestamp(y2k_leap);
        assert_eq!(timestamp, "00_02_29_00_00_00");
    }
}

/// Truncates a file name for display in CLI based on current TUI width settings
///
/// # Purpose
/// Ensures long filenames are displayed in a readable format that fits within
/// the user-configured terminal width constraints while preserving the most
/// meaningful parts: the beginning of the name and the file extension.
///
/// # Arguments
/// * `formatted_name` - The original filename to be truncated
/// * `max_name_width` - The calculated maximum width for names based on TUI settings
///
/// # Returns
/// * `String` - Truncated name if necessary, or the original if it's short enough
///
/// # Truncation Method
/// If the name exceeds max_name_width:
/// 1. Takes the first (max_name_width - FILENAME_SUFFIX_LENGTH - ellipsis.len()) characters
/// 2. Adds an ellipsis ("...")
/// 3. Keeps the last FILENAME_SUFFIX_LENGTH (5) characters (typically file extension)
///
/// # Edge Cases
/// - If max_name_width is very small (≤ 8), still ensures minimum display
/// - Handles Unicode characters correctly by counting chars, not bytes
///
/// # Examples
/// ```rust
/// let long_name = "really_long_filename_that_exceeds_the_maximum_length_for_display.txt";
/// // With max_name_width = 20
/// assert_eq!(
///     truncate_filename_for_display(long_name.to_string(), 20),
///     "really_long...y.txt"
/// );
///
/// let short_name = "short.txt";
/// // With max_name_width = 20
/// assert_eq!(
///     truncate_filename_for_display(short_name.to_string(), 20),
///     "short.txt"
/// );
/// ```
fn truncate_filename_for_display(formatted_name: String, max_name_width: usize) -> String {
    let ellipsis = "...";

    // Check if truncation is needed
    if formatted_name.chars().count() <= max_name_width {
        return formatted_name;
    }

    // Calculate how many characters we can take from the start
    // Ensure we don't underflow even with very small max_name_width
    let prefix_length = max_name_width
        .saturating_sub(FILENAME_SUFFIX_LENGTH)
        .saturating_sub(ellipsis.len());

    // Get prefix (start of the filename)
    let prefix: String = formatted_name.chars().take(prefix_length).collect();

    // Get suffix (end of the filename, including extension)
    let suffix: String = formatted_name
        .chars()
        .skip(formatted_name.chars().count().saturating_sub(FILENAME_SUFFIX_LENGTH))
        .collect();

    // Combine prefix, ellipsis, and suffix
    format!("{}{}{}", prefix, ellipsis, suffix)
}

/// Formats and displays directory contents as a numbered list with columns
///
/// # Purpose
/// Renders the file browser interface with user-configured column widths and
/// pagination settings, providing a customizable display experience.
///
/// # Arguments
/// * `directory_entries` - Vector of FileSystemEntry items to display
/// * `current_directory_path` - PathBuf of the directory being displayed
/// * `page_info` - Optional pagination info (current_page, total_pages)
/// * `filter` - Current filter setting (Some('d'), Some('f'), or None)
/// * `nav_state` - Navigation state containing TUI size adjustments
///
/// # Display Adjustments
/// - Name column width adjusts based on nav_state TUI settings
/// - Number of items shown determined by TUI height settings
/// - Other columns (Size, Modified) remain fixed width
///
/// # Layout Calculation
/// The name column width is calculated dynamically:
/// - Base width: 55 characters (MAX_NAME_LENGTH_DEFAULT)
/// - Adjusted by user's wide+N or wide-N settings
/// - Minimum width: 8 characters (for "...suffix" display)
fn display_directory_contents(
    directory_entries: &[FileSystemEntry],
    current_directory_path: &PathBuf,
    page_info: Option<(usize, usize)>,
    filter: Option<char>,
    nav_state: &NavigationState,  // Add nav_state parameter
) -> io::Result<()> {
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");

    // Calculate the actual name column width based on TUI settings
    let name_column_width = calculate_name_width_from_state(nav_state) as usize;

    let filter_status = match filter {
        Some('d') => "[Directories only] ",
        Some('f') => "[Files only] ",
        _ => "",
    };

    let legend = format!(
        "{}{}q{}uit {}b{}ack|{}t{}erm|{}d{}ir {}f{}ile|{}n{}ame {}s{}ize {}m{}od|{}g{}et-send file {}v{},{}y{},{}p{}|{}str{}>search|{}enter{}>reset{}",
        YELLOW,           // Overall legend color
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

    // Directory/file mode on path-display line
    let path_display = format!("{}", current_directory_path.display());
    println!("{}\n{}{}", legend, filter_status, path_display);

    // Column headers with dynamic name width
    println!(
        "{:>4}  {:<width$} {:>5} {:>11}",
        " # ", "Name", "Size", "Modified",
        width = name_column_width
    );

    // Separator line adjusted for dynamic width
    let separator_length = 4 + name_column_width + 7 + 1 + 11;
    println!(" {} ", "-".repeat(separator_length));

    // Display entries with dynamic width
    for (entry_index, directory_entry) in directory_entries.iter().enumerate() {
        let formatted_name = if directory_entry.is_directory {
            format!("{}/", directory_entry.file_system_item_name)
        } else {
            directory_entry.file_system_item_name.clone()
        };

        // Use dynamic width for truncation
        let display_name = truncate_filename_for_display(formatted_name, name_column_width);

        let size_display = if directory_entry.is_directory {
            String::from("-")
        } else {
            format_file_size(directory_entry.file_system_item_size_in_bytes)
        };

        let time_display = format_timestamp(directory_entry.file_system_item_last_modified_time);

        println!(
            "{:>3}. {:<width$} {:>6} {:>11}",
            entry_index + 1,
            display_name,
            size_display,
            time_display,
            width = name_column_width
        );
    }

    // Add pagination footer with TUI size info
    if let Some((current_page, total_pages)) = page_info {
        // Format TUI adjustments for display
        let (tall_display, wide_display) = format_tui_adjustments(
            nav_state.tui_tall_adjustment,
            nav_state.tui_tall_direction_sign,
            nav_state.tui_wide_adjustment,
            nav_state.tui_wide_direction_sign
        );

        if total_pages > 1 {
            println!("\x1b[1m{}--- Page {} of {}: up/down, j/k, </>, w/x, arrows, etc. Size: {} {} ---{}",
                    YELLOW, current_page, total_pages, tall_display, wide_display, RESET);
        } else {
            // Show size info even when only one page
            println!("\x1b[1m{}--- (Re)Size: {} {} ---{}",
                    YELLOW, tall_display, wide_display, RESET);
        }
    }

    io::stdout().flush()?;
    Ok(())
}

/// Name of the data directory that contains File Fantastic configuration files
const FF_DATA_DIRECTORY_NAME: &str = "ff_data";

/// Name of the configuration file that contains partner program paths
const PARTNER_PROGRAMS_CONFIG_FILENAME: &str = "absolute_paths_to_local_partner_fileopening_executibles.txt";

/// Reads the partner programs configuration file and returns valid executable paths
///
/// # Purpose
/// Manages the partner programs configuration file located at:
/// `{executable_directory}/ff_data/absolute_paths_to_local_partner_fileopening_executibles.txt`
/// This file contains absolute paths to local executable programs that users want
/// to use as file opening options alongside system-installed applications.
///
/// # Directory Structure
/// ```
/// /path/to/executable/
/// ├── file_fantastic (or file_fantastic.exe)
/// └── ff_data/                     <- Created if doesn't exist
///     └── absolute_paths_to_local_partner_fileopening_executibles.txt
/// ```
///
/// # Returns
/// * `Vec<PathBuf>` - Vector of valid executable paths, empty if none found or errors occur
///
/// # File Format
/// Each line should contain one absolute path to an executable:
/// ```text
/// /home/user/custom_tools/my_editor
/// /opt/local/bin/special_viewer
/// /usr/local/bin/custom_processor
/// ```
///
/// # Behavior
/// 1. **Directory doesn't exist**: Creates `ff_data` directory
/// 2. **File doesn't exist**: Creates empty file with helpful comments and returns empty vector
/// 3. **File exists**: Reads all lines, validates each path, returns only valid executables
/// 4. **Any errors**: Returns empty vector (graceful degradation to standard functionality)
///
/// # Path Validation
/// For each line in the file:
/// - Skips empty lines and comments (lines starting with #)
/// - Verifies path exists and points to a file
/// - On Unix systems, checks if file has execute permissions
/// - Invalid paths are logged as warnings but don't cause failure
///
/// # Error Handling Philosophy
/// This function uses "graceful degradation" - any errors result in returning an empty
/// vector, which causes the partner programs feature to be unavailable but doesn't
/// break File Fantastic's core functionality. Specific error details are logged
/// for user awareness but don't interrupt the workflow.
///
/// # Example Configuration File Content
/// ```text
/// # File Fantastic - Local Partner Programs Configuration
/// # Add absolute paths to local executables, one per line
/// # Example:
/// # /home/user/my_tools/custom_editor
/// # /opt/local/bin/special_viewer
///
/// /home/user/rust_projects/my_file_processor/target/release/my_processor
/// /usr/local/bin/custom_text_editor
/// ```
///
/// # Usage Context
/// Called during file opening to provide additional program options beyond
/// system-installed applications. Integrates seamlessly with existing file
/// opening functionality as an optional enhancement.
///
/// # Example
/// ```rust
/// let partner_programs = read_partner_programs_file();
/// if partner_programs.is_empty() {
///     // Use standard program selection only
///     show_standard_prompt();
/// } else {
///     // Show enhanced prompt with numbered partner program options
///     show_enhanced_prompt(&partner_programs);
/// }
/// ```
fn read_partner_programs_file() -> Vec<PathBuf> {
    // Step 1: Get the executable directory
    let executable_directory = match std::env::current_exe() {
        Ok(exe_path) => {
            match exe_path.parent() {
                Some(parent) => parent.to_path_buf(),
                None => {
                    eprintln!("Warning: Cannot determine executable directory for partner programs");
                    return Vec::new();
                }
            }
        },
        Err(error) => {
            eprintln!("Warning: Cannot locate executable for partner programs: {}", error);
            return Vec::new();
        }
    };

    // Step 2: Build path to ff_data directory
    let ff_data_directory_path = executable_directory.join(FF_DATA_DIRECTORY_NAME);

    // Step 3: Create ff_data directory if it doesn't exist
    if !ff_data_directory_path.exists() {
        match fs::create_dir(&ff_data_directory_path) {
            Ok(_) => {
                println!("Created File Fantastic data directory: {}",
                        ff_data_directory_path.display());
            },
            Err(error) => {
                eprintln!("Warning: Could not create ff_data directory at {}: {}",
                         ff_data_directory_path.display(),
                         error);
                return Vec::new();
            }
        }
    } else if !ff_data_directory_path.is_dir() {
        // Path exists but is not a directory
        eprintln!("Warning: ff_data path exists but is not a directory: {}",
                 ff_data_directory_path.display());
        return Vec::new();
    }

    // Step 4: Build path to configuration file within ff_data directory
    let config_file_path = ff_data_directory_path.join(PARTNER_PROGRAMS_CONFIG_FILENAME);

    // Step 5: If configuration file doesn't exist, create it with helpful template
    if !config_file_path.exists() {
        // Template content with instructions for users
        let initial_template_content = "# File Fantastic - Local Partner Programs Configuration\n\
                              # This file allows you to register custom executables as file opening options\n\
                              #\n\
                              # Instructions:\n\
                              # - Add absolute paths to local executables, one per line\n\
                              # - Lines starting with # are comments and will be ignored\n\
                              # - Empty lines are also ignored\n\
                              # - Each path must point to an existing, executable file\n\
                              #\n\
                              # Examples:\n\
                              # /home/user/my_tools/custom_editor\n\
                              # /opt/local/bin/special_viewer\n\
                              # /usr/local/bin/custom_processor\n\
                              # C:\\Users\\Username\\Tools\\my_program.exe (Windows)\n\
                              #\n\
                              # These programs will appear as numbered options when opening files\n\
                              # alongside your system's default applications.\n\n";

        match fs::write(&config_file_path, initial_template_content) {
            Ok(_) => {
                println!("Created partner programs configuration file: {}",
                        config_file_path.display());
                println!("Edit this file to add your custom executables for file opening");
            },
            Err(error) => {
                eprintln!("Warning: Could not create partner programs configuration file at {}: {}",
                         config_file_path.display(),
                         error);
            }
        }
        // Return empty vector since new file has no configured programs yet
        return Vec::new();
    }

    // Step 6: Read the existing configuration file
    let file_contents = match fs::read_to_string(&config_file_path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Warning: Could not read partner programs file at {}: {}",
                     config_file_path.display(),
                     error);
            return Vec::new(); // Graceful degradation
        }
    };

    // Step 7: Parse lines and validate each path
    let mut validated_partner_programs = Vec::new();

    for (line_index, line_content) in file_contents.lines().enumerate() {
        // Remove leading and trailing whitespace
        let trimmed_line = line_content.trim();

        // Skip empty lines and comment lines
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        // Convert line to PathBuf for validation
        let program_path = PathBuf::from(trimmed_line);

        // Validation 1: Check if path exists in filesystem
        if !program_path.exists() {
            eprintln!("Warning: Partner program path does not exist (line {}): {}",
                     line_index + 1,
                     trimmed_line);
            continue;
        }

        // Validation 2: Check if path points to a file (not directory)
        if !program_path.is_file() {
            eprintln!("Warning: Partner program path is not a file (line {}): {}",
                     line_index + 1,
                     trimmed_line);
            continue;
        }

        // Validation 3: On Unix systems, check executable permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            match program_path.metadata() {
                Ok(metadata) => {
                    let permissions = metadata.permissions();
                    // Check if any execute bit is set (owner, group, or other)
                    if permissions.mode() & 0o111 == 0 {
                        eprintln!("Warning: Partner program lacks execute permissions (line {}): {}",
                                 line_index + 1,
                                 trimmed_line);
                        continue;
                    }
                },
                Err(error) => {
                    eprintln!("Warning: Could not check permissions for partner program (line {}): {}",
                             line_index + 1,
                             error);
                    continue;
                }
            }
        }

        // All validations passed - add to validated programs list
        validated_partner_programs.push(program_path);
    }

    // Return the list of validated partner programs
    validated_partner_programs
}

/// Extracts the filename from a path for user-friendly display in program selection menus
///
/// # Purpose
/// Converts full absolute paths to just the executable name portion for clean
/// display in the file opening prompt. This makes the numbered partner program
/// options more readable and user-friendly.
///
/// # Arguments
/// * `program_path` - The full absolute path to the executable
///
/// # Returns
/// * `String` - The filename portion of the path, or a descriptive fallback
///
/// # Fallback Strategy
/// If the filename cannot be extracted from the path (rare edge cases):
/// 1. Attempts to use the last component of the path
/// 2. If that fails, uses the full path as a string
/// 3. As a last resort, returns a generic placeholder
///
/// # Examples
/// ```rust
/// let path1 = PathBuf::from("/home/user/my_tools/custom_editor");
/// assert_eq!(extract_program_display_name(&path1), "custom_editor");
///
/// let path2 = PathBuf::from("/usr/local/bin/special-viewer");
/// assert_eq!(extract_program_display_name(&path2), "special-viewer");
///
/// let path3 = PathBuf::from("/opt/tools/file_processor_v2");
/// assert_eq!(extract_program_display_name(&path3), "file_processor_v2");
/// ```
///
/// # Usage Context
/// Used when building the enhanced file opening prompt that shows numbered
/// partner program options. The extracted names are displayed as:
/// "OR by number: 1. custom_editor 2. special_viewer 3. file_processor"
///
/// # Design Considerations
/// - Prioritizes readability over technical accuracy
/// - Handles edge cases gracefully without causing errors
/// - Returns something meaningful even for unusual path structures
/// - Keeps the display compact and scannable for users
fn extract_program_display_name(program_path: &PathBuf) -> String {
    program_path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
        .unwrap_or_else(|| {
            // Fallback: try to use the full path or a placeholder
            program_path.to_string_lossy().to_string()
        })
}

/// Launches a partner program in a new terminal window to open the specified file
///
/// # Purpose
/// Executes a local partner executable in a new terminal window, similar to how
/// terminal-based editors like vim and nano are launched. This ensures the partner
/// program has proper terminal interaction capabilities and doesn't interfere with
/// File Fantastic's interface.
///
/// # Arguments
/// * `program_path` - Absolute path to the partner executable to launch
/// * `file_path` - Path to the file that should be opened by the partner program
///
/// # Returns
/// * `Result<()>` - Success or FileFantasticError with detailed context
///
/// # Terminal Launch Strategy
/// Uses the same cross-platform terminal launching logic as File Fantastic's
/// existing editor support:
/// - **macOS**: Launches via Terminal.app using the `open` command
/// - **Linux**: Tries multiple terminal emulators in preference order
/// - **Windows**: Uses cmd.exe with appropriate arguments
///
/// # Command Execution
/// The partner program is launched with the file path as its first argument:
/// ```bash
/// /path/to/partner_program /path/to/file_to_open
/// ```
///
/// # Error Handling
/// Follows File Fantastic's established error handling patterns:
/// - Specific error types for different failure modes
/// - Clear error messages with context
/// - Graceful fallback opportunities for calling code
/// - No silent failures - all errors are visible to users
///
/// # Platform-Specific Implementation
///
/// ## macOS
/// ```bash
/// open -a Terminal "/path/to/partner_program /path/to/file; exit"
/// ```
///
/// ## Linux (tries in order)
/// 1. gnome-terminal --  /path/to/partner_program /path/to/file
/// 2. ptyxis --  /path/to/partner_program /path/to/file
/// 3. konsole --e /path/to/partner_program /path/to/file
/// 4. (additional terminal emulators...)
///
/// ## Windows
/// ```cmd
/// cmd /C start cmd /C "/path/to/partner_program /path/to/file && pause"
/// ```
///
/// # Security Considerations
/// - Validates that the program path exists before execution
/// - Relies on filesystem permissions for execution control
/// - Does not perform shell injection sanitization (paths come from config file)
/// - Users are responsible for the security of their configured partner programs
///
/// # Example Usage
/// ```rust
/// let partner_program = PathBuf::from("/home/user/my_tools/file_processor");
/// let file_to_open = PathBuf::from("/home/user/documents/data.txt");
///
/// match launch_partner_program_in_terminal(&partner_program, &file_to_open) {
///     Ok(_) => println!("Partner program launched successfully"),
///     Err(e) => {
///         println!("Error launching partner program: {}. \nPress Enter to continue", e);
///         let mut buf = String::new();
///         io::stdin().read_line(&mut buf)?;
///         // Continue with fallback options...
///     }
/// }
/// ```
fn launch_partner_program_in_terminal(program_path: &PathBuf, file_path: &PathBuf) -> Result<()> {
    // Validate program still exists (it might have been deleted since config was read)
    if !program_path.exists() {
        return Err(FileFantasticError::NotFound(program_path.clone()));
    }

    if !program_path.is_file() {
        return Err(FileFantasticError::InvalidName(
            format!("Partner program is not a file: {}", program_path.display())
        ));
    }

    // Launch using platform-specific terminal commands (reusing existing logic patterns)
    #[cfg(target_os = "macos")]
    {
        // Build command string for macOS Terminal.app
        let command_string = format!("{} {}",
                                    program_path.to_string_lossy(),
                                    file_path.to_string_lossy());

        std::process::Command::new("open")
            .args(["-a", "Terminal"])
            .arg(format!("{}; exit", command_string))
            .spawn()
            .map_err(|e| {
                eprintln!("Failed to open Terminal.app for partner program: {}", e);
                FileFantasticError::EditorLaunchFailed(
                    extract_program_display_name(program_path)
                )
            })?;
    }

    #[cfg(target_os = "linux")]
    {
        // Try different terminal emulators in order of preference
        // Pass program and file as separate arguments (safer than shell string)
        let terminal_commands = [
            ("gnome-terminal", vec!["--"]),
            ("ptyxis", vec!["--"]),
            ("konsole", vec!["--e"]),
            ("xfce4-terminal", vec!["--command"]),
            ("mate-terminal", vec!["--command"]),
            ("terminator", vec!["-e"]),
            ("alacritty", vec!["-e"]),
            ("kitty", vec!["-e"]),
            ("tilix", vec!["-e"]),
            ("urxvt", vec!["-e"]),
            ("rxvt", vec!["-e"]),
            ("xterm", vec!["-e"]),
        ];

        let mut terminal_launched = false;
        for (terminal, args) in terminal_commands.iter() {
            let mut cmd = std::process::Command::new(terminal);
            cmd.args(args)
               .arg(program_path)
               .arg(file_path);

            if cmd.spawn().is_ok() {
                terminal_launched = true;
                break;
            }
        }

        if !terminal_launched {
            return Err(FileFantasticError::NoTerminalFound);
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Build command string for Windows cmd.exe
        let command_string = format!("{} {}",
                                    program_path.to_string_lossy(),
                                    file_path.to_string_lossy());

        std::process::Command::new("cmd")
            .args(["/C", "start", "cmd", "/C"])
            .arg(format!("{} && pause", command_string))
            .spawn()
            .map_err(|e| {
                eprintln!("Failed to open cmd.exe for partner program: {}", e);
                FileFantasticError::EditorLaunchFailed(
                    extract_program_display_name(program_path)
                )
            })?;
    }

    println!("Launched partner program: {} with {}",
             extract_program_display_name(program_path),
             file_path.file_name().unwrap_or_default().to_string_lossy());

    Ok(())
}

/// Checks if a command/program is available on the system
///
/// # Arguments
/// * `cmd` - The command name to check
///
/// # Returns
/// * `bool` - true if the command is available, false otherwise
///
/// # Implementation Notes
/// Tests command availability by attempting to run with --version flag
/// Most commands support this flag without side effects
fn is_command_available(cmd: &str) -> bool {
    std::process::Command::new(cmd)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_or(false, |status| status.success())
}

/// Opens a file in the current terminal using available terminal editors
///
/// # Arguments
/// * `editor` - The editor command to use
/// * `file_path` - Path to the file to open
///
/// # Returns
/// * `Result<()>` - Success or error
///
/// # Behavior
/// - Opens the file in the SAME terminal (blocking)
/// - Waits for the editor to exit before returning
/// - Used for headless/server environments
fn open_in_current_terminal(editor: &str, file_path: &PathBuf) -> Result<()> {
    // Check if the specified editor is available
    if !is_command_available(editor) {
        return Err(FileFantasticError::EditorLaunchFailed(
            format!("Editor '{}' is not available on this system", editor)
        ));
    }

    // Launch the editor in the current terminal (blocking call)
    let status = std::process::Command::new(editor)
        .arg(file_path)
        .status()
        .map_err(|e| {
            eprintln!("Failed to launch {} in current terminal: {}", editor, e);
            FileFantasticError::Io(e)
        })?;

    if status.success() {
        println!("You have exited {}.", editor);
        Ok(())
    } else {
        Err(FileFantasticError::EditorLaunchFailed(
            format!("Editor '{}' exited with non-zero status", editor)
        ))
    }
}

/// Opens a file in a tmux split pane
///
/// # Arguments
/// * `editor` - The editor command to use
/// * `file_path` - Path to the file to open
/// * `split_type` - Either "-v" for vertical or "-h" for horizontal split
///
/// # Returns
/// * `Result<()>` - Success or error
///
/// # Prerequisites
/// - tmux must be installed and available
/// - Must be running inside a tmux session
///
/// # Behavior
/// Creates a new tmux pane and opens the editor in it
/// The pane closes automatically when the editor exits
fn open_in_tmux_split(editor: &str, file_path: &PathBuf, split_type: &str) -> Result<()> {

    // Check if the specified editor is available
    if !is_command_available(editor) {
        return Err(FileFantasticError::EditorLaunchFailed(
            format!("Editor '{}' is not available on this system", editor)
        ));
    }

    // Build the command to run in the new split
    let editor_command = format!("{} {}", editor, file_path.to_string_lossy());

    // Create the tmux split with the editor
    let output = std::process::Command::new("tmux")
        .args([
            "split-window",
            split_type,  // "-v" for vertical, "-h" for horizontal
            &editor_command
        ])
        .output()
        .map_err(|e| {
            eprintln!("Failed to create tmux split: {}", e);
            FileFantasticError::Io(e)
        })?;

    if output.status.success() {
        println!("Opened {} in tmux {} split",
                 editor,
                 if split_type == "-v" { "vertical" } else { "horizontal" });
        Ok(())
    } else {
        Err(FileFantasticError::EditorLaunchFailed(
            format!("Failed to create tmux split: {}",
                    String::from_utf8_lossy(&output.stderr))
        ))
    }
}

/// Parses special flags from user input for headless mode, tmux splits, and CSV analysis
///
/// # Arguments
/// * `input` - The user input string
///
/// # Returns
/// * `Option<(String, String)>` - Some((editor, flag)) if a special flag is found, None otherwise
///
/// # Supported Flags
/// * `-h` or `--headless` - Open in current terminal
/// * `-vsplit` or `--vertical-split-tmux` - Open in vertical tmux split
/// * `-hsplit` or `--horizontal-split-tmux` - Open in horizontal tmux split
/// * `-rc` or `--rows-and-columns` - Analyze CSV file before opening (CSV files only)
///
/// # Examples
/// * "vim -h" -> Some(("vim", "-h"))
/// * "nano -rc" -> Some(("nano", "-rc"))
/// * "code -h -rc" -> Some(("code", "-h -rc")) // Combined flags preserved
fn parse_special_flags(input: &str) -> Option<(String, String)> {
    let flags = [
        "-h", "--headless",
        "-vsplit", "--vertical-split-tmux",
        "-hsplit", "--horizontal-split-tmux",
        "-rc", "--rows-and-columns"
    ];

    // Check if input contains any special flags
    let mut found_flags = Vec::new();
    let mut editor_parts = Vec::new();

    let parts: Vec<&str> = input.split_whitespace().collect();

    for part in &parts {
        if flags.contains(&part.as_ref()) {
            found_flags.push(part.to_string());
        } else {
            editor_parts.push(part.to_string());
        }
    }

    if !found_flags.is_empty() {
        let editor = editor_parts.join(" ");
        let flags_str = found_flags.join(" ");
        Some((editor, flags_str))
    } else {
        None
    }
}

/// Checks if a file path points to a CSV file
///
/// # Arguments
/// * `file_path` - The path to check
///
/// # Returns
/// * `bool` - true if the file has a .csv extension (case-insensitive)
///
/// # Implementation Notes
/// Checks the last 4 characters of the file path for ".csv"
/// Case-insensitive comparison to handle .CSV, .Csv, etc.
fn is_csv_file(file_path: &PathBuf) -> bool {
    if let Some(extension) = file_path.extension() {
        extension.to_ascii_lowercase() == "csv"
    } else {
        false
    }
}

/// Handles CSV analysis when -rc flag is used
///
/// # Arguments
/// * `csv_path` - Path to the CSV file to analyze
///
/// # Returns
/// * `Result<PathBuf>` - Path to the analysis results file, or error
///
/// # Workflow
/// 1. Calls rc_analyze_datafile_save_results_to_resultsfile
/// 2. Shows user the analysis will be opened
/// 3. Waits for user confirmation
/// 4. Returns the analysis file path
///
/// # Error Handling
/// Returns error if analysis fails, allowing caller to re-prompt user
fn handle_csv_analysis(csv_path: &PathBuf) -> Result<PathBuf> {
    // Convert PathBuf to string for the analysis function
    let csv_path_str = csv_path.to_str().ok_or_else(|| {
        FileFantasticError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid file path - contains non-UTF8 characters"
        ))
    })?;

    println!("{}Analyzing CSV file...{}", YELLOW, RESET);

    // Call the analysis function
    match rc_analyze_datafile_save_results_to_resultsfile(csv_path_str) {
        Ok(analysis_path) => {
            // Display the analysis file path
            println!("{}Analysis complete! Results saved to:{}", GREEN, RESET);
            println!("  {}", analysis_path.display());
            println!();

            // Prompt user before opening
            println!("{}You will open the analysis file. Press Enter to continue...{}", YELLOW, RESET);
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).map_err(|e| {
                eprintln!("Failed to read input: {}", e);
                FileFantasticError::Io(e)
            })?;

            Ok(analysis_path)
        }
        Err(e) => {
            // Convert the analysis error to our error type
            let error_msg = format!("CSV analysis failed: {}", e);
            println!("{}Error: {}{}", RED, error_msg, RESET);
            Err(FileFantasticError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_msg
            )))
        }
    }
}

/// Opens a file with user-selected editor, partner program, or system default
///
/// # Purpose
/// Enhanced file opening interface that supports traditional system applications,
/// user-configured local partner programs, and system default file associations.
/// Partner programs are custom executables defined in a configuration file.
///
/// # Arguments
/// * `file_path` - PathBuf of the file to open
///
/// # Returns
/// * `Result<()>` - Success or FileFantasticError with context
///
/// # Enhanced User Experience
/// The function now provides three types of file opening options:
/// 1. **System Default**: Empty input uses OS file associations
/// 2. **Named Programs**: Type program names (nano, vim, code, etc.)
/// 3. **Partner Programs**: Select by number from configured local executables
///
/// # Partner Programs Integration
/// - Reads from 'absolute_paths_to_local_partner_fileopening_executibles.txt'
/// - File is located in the same directory as the File Fantastic executable
/// - Shows partner programs as numbered options (1, 2, 3, etc.)
/// - Launches partner programs in new terminal windows
/// - Gracefully handles partner program failures with clear error messages
///
/// # User Interface Examples
///
/// ## With Partner Programs Available
/// ```text
/// Open with... (hit enter for default, or enter software 'name' as called in terminal:
/// gedit, firefox, vi, nano, hx, lapce, ... OR by number: 1. my_editor 2. file_processor):
/// ```
///
/// ## Without Partner Programs
/// ```text
/// Open with... (hit enter for default, or enter software 'name' as called in terminal:
/// gedit, firefox, hx, lapce, vi, vim, nano, code, etc.):
/// ```
///
/// # Input Processing Priority
/// 1. **Empty input**: System default application
/// 2. **Numeric input**: Partner program selection (if valid number)
/// 3. **Text input**: Named system application
///
/// # Error Handling Philosophy
/// Maintains File Fantastic's established error handling patterns:
/// - Clear error messages with context
/// - "Press Enter to continue" user acknowledgment
/// - Graceful fallback to system default when specific programs fail
/// - No silent failures - all errors are visible and acknowledged
///
/// # Backwards Compatibility
/// All existing functionality is preserved:
/// - Traditional program name input works exactly as before
/// - System default behavior is unchanged
/// - GUI vs terminal editor detection logic is maintained
/// - Platform-specific terminal launching is preserved
///
/// # Example User Workflows
///
/// ## Traditional Usage (unchanged)
/// ```text
/// User input: "nano"
/// Result: Opens nano in new terminal
/// ```
///
/// ## Partner Program Usage (new)
/// ```text
/// User input: "1"
/// Result: Launches first configured partner program in new terminal
/// ```
///
/// ## Default Usage (unchanged)
/// ```text
/// User input: [Enter]
/// Result: Opens with system default application
/// ```
fn open_file(file_path: &PathBuf) -> Result<()> {
    /*
     *
     new feature,
     put in the path to the file

     * useing
     * rc_analyze_datafile_save_results_to_resultsfile()
     * pub fn rc_analyze_datafile_save_results_to_resultsfile(
         csv_file_path_argument: &str,
     ) -> RowsAndColumnsResult<PathBuf> {

     if file is .csv
     user can says 'rows and columns'

     then file will be analysed

     and a results file path will be returned

     then the use can view that analysis in whatever editor


     steps
     new flag for user
     -rc
     or
     --rows-and-columns

     this will put the path into the above function
     then use the output path of that function

     as the file to open

     e.g.

     1 gedit -rc
     means open the analysis of file 1 with gedit

     if there is an error or no analysis file is returned
     pass an error message to try again
     but don't crash the application,
     empty return from this function (or similar) is fine
     user can

     */

    // Read partner programs configuration (gracefully handles all errors)
    let partner_programs = read_partner_programs_file();

    // check if suffi

    // Build the user prompt based on whether partner programs are available
    let prompt = if partner_programs.is_empty() {
        // Standard prompt when no partner programs are configured
        format!(
            "{}Open with... (hit enter for default, or software 'name' as used in terminal: vi --headless, gedit, firefox...tmux: nano -hsplit|vsplit, -rc for .csv stats) {}",
            YELLOW, RESET
        )
    } else {
        // Enhanced prompt showing numbered partner program options
        let mut numbered_options = String::new();
        for (index, program_path) in partner_programs.iter().enumerate() {
            if index > 0 {
                numbered_options.push(' ');
            }
            numbered_options.push_str(&format!("{}. {}",
                                              index + 1,
                                              extract_program_display_name(program_path)));
        }

        format!(
            "{}Open with... (Enter for default, or software 'name' as w/ terminal: vi --headless, nano -hsplit, gedit, firefox, -rc for .csv stats OR by number: {}): {}",
            YELLOW, numbered_options, RESET
        )
    };

    // Display the prompt and get user input
    print!("{}", prompt);
    io::stdout().flush().map_err(|e| {
        eprintln!("Failed to flush stdout: {}", e);
        FileFantasticError::Io(e)
    })?;

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).map_err(|e| {
        eprintln!("Failed to read input: {}", e);
        FileFantasticError::Io(e)
    })?;
    let user_input = user_input.trim();

    // Check for special flags (headless, tmux splits, and CSV analysis)
    if let Some((editor, flags)) = parse_special_flags(user_input) {
        // User must provide an editor with these flags
        if editor.is_empty() {
            println!("{}Error: You must specify an editor with the {} flag (e.g., 'vim {}'){}",
                        RED, flags, flags, RESET);
            println!("Press Enter to continue...");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).map_err(|e| {
                eprintln!("Failed to read input: {}", e);
                FileFantasticError::Io(e)
            })?;
            return open_file(file_path); // Re-prompt
        }

        // Check if -rc flag is present and file is CSV
        let mut file_to_open = file_path.clone();
        if flags.contains("-rc") || flags.contains("--rows-and-columns") {
            if is_csv_file(file_path) {
                // Perform CSV analysis
                match handle_csv_analysis(file_path) {
                    Ok(analysis_path) => {
                        // Use the analysis file instead of the original CSV
                        file_to_open = analysis_path;
                    }
                    Err(e) => {
                        // Analysis failed, let user try again
                        println!("{}Analysis failed: {}. Press Enter to try again...{}",
                                    RED, e, RESET);
                        let mut buf = String::new();
                        io::stdin().read_line(&mut buf).map_err(|e| {
                            eprintln!("Failed to read input: {}", e);
                            FileFantasticError::Io(e)
                        })?;
                        return open_file(file_path); // Re-prompt
                    }
                }
            } else {
                // Not a CSV file, silently ignore -rc flag
                println!("{}Note: -rc flag ignored (not a CSV file){}", YELLOW, RESET);
            }
        }

        // Now handle the other flags with the appropriate file
        // Extract the primary action flag (for headless/tmux)
        let primary_flag = if flags.contains("-h") || flags.contains("--headless") {
            "-h"
        } else if flags.contains("-vsplit") || flags.contains("--vertical-split-tmux") {
            "-vsplit"
        } else if flags.contains("-hsplit") || flags.contains("--horizontal-split-tmux") {
            "-hsplit"
        } else if flags.contains("-rc") || flags.contains("--rows-and-columns") {
            // If only -rc flag, open normally (no special terminal mode)
            ""
        } else {
            ""
        };

        // // Handle the different flag types
        // let result = match primary_flag {
        //     "-h" => {
        //         // Open in current terminal (headless mode)
        //         println!("Opening file in current terminal with {}...", editor);
        //         open_in_current_terminal(&editor, &file_to_open)
        //     },
        //     "-vsplit" => {
        //         // Open in vertical tmux split
        //         open_in_tmux_split(&editor, &file_to_open, "-v")
        //     },
        //     "-hsplit" => {
        //         // Open in horizontal tmux split
        //         open_in_tmux_split(&editor, &file_to_open, "-h")
        //     },
        //     "" if !editor.is_empty() => {
        //         // Just -rc flag or no special terminal flag, open normally
        //         // Continue to the regular editor opening logic below
        //         // by falling through to the standard editor handling

        //         // Check if it's a GUI editor
        //         let gui_editors = ["code", "sublime", "subl", "gedit", "kate", "notepad++"];
        //         if gui_editors.contains(&editor.to_lowercase().as_str()) {
        //             // Launch GUI editor directly
        //             std::process::Command::new(&editor)
        //                 .arg(&file_to_open)
        //                 .spawn()
        //                 .map_err(|e| {
        //                     FileFantasticError::EditorLaunchFailed(
        //                         format!("{}: {}", editor, e)
        //                     )
        //                 })?;
        //             Ok(())
        //         } else {
        //             // Open terminal-based editor in new terminal window
        //             #[cfg(target_os = "macos")]
        //             {
        //                 std::process::Command::new("open")
        //                     .args(["-a", "Terminal"])
        //                     .arg(format!("{} {}; exit", editor, file_to_open.to_string_lossy()))
        //                     .spawn()
        //                     .map_err(|e| {
        //                         FileFantasticError::EditorLaunchFailed(
        //                             format!("{}: {}", editor, e)
        //                         )
        //                     })?;
        //                 Ok(())
        //             }
        //             #[cfg(target_os = "linux")]
        //             {
        //                 // Try different terminal emulators
        //                 let terminal_commands = [
        //                     ("gnome-terminal", vec!["--", &editor]),
        //                     ("ptyxis", vec!["--", &editor]),
        //                     ("konsole", vec!["--e", &editor]),
        //                     ("xfce4-terminal", vec!["--command", &editor]),
        //                     ("terminator", vec!["-e", &editor]),
        //                     ("tilix", vec!["-e", &editor]),
        //                     ("kitty", vec!["-e", &editor]),
        //                     ("alacritty", vec!["-e", &editor]),
        //                     ("xterm", vec!["-e", &editor]),
        //                 ];

        //                 let mut success = false;
        //                 for (terminal, args) in terminal_commands.iter() {
        //                     let mut cmd = std::process::Command::new(terminal);
        //                     cmd.args(args).arg(&file_to_open);

        //                     if cmd.spawn().is_ok() {
        //                         success = true;
        //                         break;
        //                     }
        //                 }

        //                 if success {
        //                     Ok(())
        //                 } else {
        //                     Err(FileFantasticError::EditorLaunchFailed(
        //                         "No terminal emulator found".to_string()
        //                     ))
        //                 }
        //             }
        //             #[cfg(target_os = "windows")]
        //             {
        //                 std::process::Command::new("cmd")
        //                     .args(["/C", "start", "cmd", "/C"])
        //                     .arg(format!("{} {} && pause", editor, file_to_open.to_string_lossy()))
        //                     .spawn()
        //                     .map_err(|e| {
        //                         FileFantasticError::EditorLaunchFailed(
        //                             format!("{}: {}", editor, e)
        //                         )
        //                     })?;
        //                 Ok(())
        //             }
        //         }
        //     },
        //     _ => {
        //         // This shouldn't happen
        //         Err(FileFantasticError::EditorLaunchFailed(
        //             format!("Unknown flag: {}", primary_flag)
        //         ))
        //     }
        // };

        // Handle the different flag types
        let result = match primary_flag {
            "-h" => {
                // Open in current terminal (headless mode)
                println!("Opening file in current terminal with {}...", editor);
                open_in_current_terminal(&editor, &file_to_open)
            },
            "-vsplit" => {
                // Open in vertical tmux split
                open_in_tmux_split(&editor, &file_to_open, "-v")
            },
            "-hsplit" => {
                // Open in horizontal tmux split
                open_in_tmux_split(&editor, &file_to_open, "-h")
            },
            "" if !editor.is_empty() => {
                // Just -rc flag or no special terminal flag, open normally
                // Continue to the regular editor opening logic below
                // by falling through to the standard editor handling

                // Check if it's a GUI editor
                let gui_editors = ["code", "sublime", "subl", "gedit", "kate", "notepad++"];
                if gui_editors.contains(&editor.to_lowercase().as_str()) {
                    // Launch GUI editor directly
                    std::process::Command::new(&editor)
                        .arg(&file_to_open)
                        .spawn()
                        .map_err(|e| {
                            FileFantasticError::EditorLaunchFailed(
                                format!("{}: {}", editor, e)
                            )
                        })?;
                    Ok(())
                } else {
                    // Open terminal-based editor in new terminal window
                    // Store the result to ensure all paths return Result<()>
                    let terminal_result = {
                        #[cfg(target_os = "macos")]
                        {
                            std::process::Command::new("open")
                                .args(["-a", "Terminal"])
                                .arg(format!("{} {}; exit", editor, file_to_open.to_string_lossy()))
                                .spawn()
                                .map_err(|e| {
                                    FileFantasticError::EditorLaunchFailed(
                                        format!("{}: {}", editor, e)
                                    )
                                })
                                .map(|_| ())
                        }
                        #[cfg(target_os = "linux")]
                        {
                            // Check if we're in Termux environment first
                            if std::env::var("TERMUX_VERSION").is_ok() ||
                               std::path::Path::new("/data/data/com.termux").exists() {
                                // In Termux, try to open directly without spawning new terminal
                                // since Termux doesn't have traditional terminal emulators
                                std::process::Command::new("termux-open")
                                    .arg(&file_to_open)
                                    .spawn()
                                    .or_else(|_| {
                                        // Fallback to direct editor launch in current terminal
                                        std::process::Command::new(&editor)
                                            .arg(&file_to_open)
                                            .spawn()
                                    })
                                    .map_err(|e| {
                                        FileFantasticError::EditorLaunchFailed(
                                            format!("Termux launch failed for {}: {}", editor, e)
                                        )
                                    })
                                    .map(|_| ())
                            } else {
                                // Try different terminal emulators for desktop Linux
                                let terminal_commands = [
                                    ("gnome-terminal", vec!["--", &editor]),
                                    ("ptyxis", vec!["--", &editor]),
                                    ("konsole", vec!["--e", &editor]),
                                    ("xfce4-terminal", vec!["--command", &editor]),
                                    ("terminator", vec!["-e", &editor]),
                                    ("tilix", vec!["-e", &editor]),
                                    ("kitty", vec!["-e", &editor]),
                                    ("alacritty", vec!["-e", &editor]),
                                    ("xterm", vec!["-e", &editor]),
                                ];

                                let mut success = false;
                                for (terminal, args) in terminal_commands.iter() {
                                    let mut cmd = std::process::Command::new(terminal);
                                    cmd.args(args).arg(&file_to_open);

                                    if cmd.spawn().is_ok() {
                                        success = true;
                                        break;
                                    }
                                }

                                if success {
                                    Ok(())
                                } else {
                                    Err(FileFantasticError::EditorLaunchFailed(
                                        "No terminal emulator found".to_string()
                                    ))
                                }
                            }
                        }
                        #[cfg(target_os = "windows")]
                        {
                            std::process::Command::new("cmd")
                                .args(["/C", "start", "cmd", "/C"])
                                .arg(format!("{} {} && pause", editor, file_to_open.to_string_lossy()))
                                .spawn()
                                .map_err(|e| {
                                    FileFantasticError::EditorLaunchFailed(
                                        format!("{}: {}", editor, e)
                                    )
                                })
                                .map(|_| ())
                        }
                        // Fallback for any other platform not covered by cfg directives
                        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
                        {
                            // Try to launch editor directly as last resort
                            std::process::Command::new(&editor)
                                .arg(&file_to_open)
                                .spawn()
                                .map_err(|e| {
                                    FileFantasticError::EditorLaunchFailed(
                                        format!("Platform not fully supported. Direct launch failed for {}: {}", editor, e)
                                    )
                                })
                                .map(|_| ())
                        }
                    };

                    terminal_result
                }
            },
            _ => {
                // This shouldn't happen
                Err(FileFantasticError::EditorLaunchFailed(
                    format!("Unknown flag: {}", primary_flag)
                ))
            }
        };

        // Handle any errors from the special mode operations
        match result {
            Ok(_) => return Ok(()),
            Err(e) => {
                // Display error and re-prompt
                println!("{}Error: {}{}",RED, e, RESET);
                println!("Press Enter to continue...");
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).map_err(|e| {
                    eprintln!("Failed to read input: {}", e);
                    FileFantasticError::Io(e)
                })?;
                return open_file(file_path); // Re-prompt for new selection
            }
        }
    }


    // Handle empty input - use system default (existing functionality)
    if user_input.is_empty() {
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
        return Ok(());
    }

    // Try to parse input as a number for partner program selection
    if let Ok(program_number) = user_input.parse::<usize>() {
        if program_number > 0 && program_number <= partner_programs.len() {
            let selected_program = &partner_programs[program_number - 1];
            println!("Launching partner program: {}", extract_program_display_name(selected_program));

            // Launch partner program in terminal with proper error handling
            match launch_partner_program_in_terminal(selected_program, file_path) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    // Follow File Fantastic's error handling pattern
                    println!("Error launching partner program: {}. \nPress Enter to continue", e);
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).map_err(|e| {
                        eprintln!("Failed to read input: {}", e);
                        FileFantasticError::Io(e)
                    })?;

                    // After user acknowledgment, fall back to asking again
                    println!("Falling back to system default...");
                    return open_file(file_path); // Recursive call for new selection
                }
            }
        } else if !partner_programs.is_empty() {
            // Invalid partner program number
            println!("Invalid partner program number. Valid range: 1-{}. \nPress Enter to continue",
                    partner_programs.len());
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).map_err(|e| {
                eprintln!("Failed to read input: {}", e);
                FileFantasticError::Io(e)
            })?;
            return open_file(file_path); // Ask again
        }
        // If no partner programs configured, fall through to treat as program name
    }

    // Handle traditional program name input (existing functionality preserved)
    let editor = user_input;

    // List of known GUI editors that shouldn't need a terminal (existing logic)
    let gui_editors = ["code", "sublime", "subl", "gedit", "kate", "notepad++"];

    if gui_editors.contains(&editor.to_lowercase().as_str()) {
        // Launch GUI editors directly (existing functionality)
        match std::process::Command::new(editor)
            .arg(file_path)
            .spawn()
        {
            Ok(_) => return Ok(()),
            Err(e) => {
                // Follow existing error handling pattern
                eprintln!("Error launching {}: {}", editor, e);
                let error = FileFantasticError::EditorLaunchFailed(editor.to_string());
                println!("Falling back to system default due to: {}. \nPress Enter to continue", error);
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).map_err(|e| {
                    eprintln!("Failed to read input: {}", e);
                    FileFantasticError::Io(e)
                })?;
                return open_file(file_path); // Ask again
            }
        }
    } else {
        // Open terminal-based editors in new terminal window (existing logic preserved)
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .args(["-a", "Terminal"])
                .arg(format!("{} {}; exit", editor, file_path.to_string_lossy()))
                .spawn()
                .map_err(|e| {
                    eprintln!("Failed to open Terminal.app for editor: {}", e);
                    FileFantasticError::EditorLaunchFailed(editor.to_string())
                })?;
        }
        #[cfg(target_os = "linux")]
        {
            // Try different terminal emulators (existing logic)
            let terminal_commands = [
                ("gnome-terminal", vec!["--", editor]),
                ("ptyxis", vec!["--", editor]),
                ("konsole", vec!["--e", editor]),
                ("xfce4-terminal", vec!["--command", editor]),
                ("terminator", vec!["-e", editor]),
                ("tilix", vec!["-e", editor]),
                ("kitty", vec!["-e", editor]),
                ("alacritty", vec!["-e", editor]),
                ("xterm", vec!["-e", editor]),
            ];

            let mut success = false;
            for (terminal, args) in terminal_commands.iter() {
                let mut cmd = std::process::Command::new(terminal);
                cmd.args(args).arg(file_path);

                if cmd.spawn().is_ok() {
                    success = true;
                    break;
                }
            }

            if !success {
                // Follow existing error handling pattern
                println!("No terminal available. Falling back to system default... \nPress Enter to continue");
                let error = FileFantasticError::EditorLaunchFailed(editor.to_string());
                eprintln!("Error: {}", error);
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).map_err(|e| {
                    eprintln!("Failed to read input: {}", e);
                    FileFantasticError::Io(e)
                })?;
                return open_file(file_path); // Ask again
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
    // Convert strings to vectors of chars for easier indexing
    // Do this FIRST to get correct character counts
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // Get the CHARACTER lengths, not byte lengths
    let m = s_chars.len();
    let n = t_chars.len();

    // Handle empty string cases
    if m == 0 { return n; }
    if n == 0 { return m; }

    // Create two work vectors
    let mut v0: Vec<usize> = (0..=n).collect();
    let mut v1: Vec<usize> = vec![0; n + 1];

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
        std::mem::swap(&mut v0, &mut v1);
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


#[cfg(test)]
mod archive_tests_2 {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::env;
    use std::io::{Read, Write};
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Static counter to ensure unique test directories
    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    /// Helper function to create a temporary test directory with guaranteed uniqueness
    ///
    /// # Purpose
    /// Creates a unique temporary directory for test isolation using a counter
    /// to avoid conflicts between parallel tests
    ///
    /// # Returns
    /// * `PathBuf` - Path to the created temporary directory
    fn create_temp_test_dir() -> PathBuf {
        // Use atomic counter for uniqueness across parallel tests
        let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);

        // Create unique directory name with process ID, counter, and timestamp
        let temp_dir = env::temp_dir().join(format!(
            "test_archive_{}_{}_{}",
            std::process::id(),
            counter,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_nanos() // Use nanoseconds for even more uniqueness
        ));

        // Create the directory
        fs::create_dir_all(&temp_dir)
            .unwrap_or_else(|e| panic!("Failed to create temp dir at {}: {}", temp_dir.display(), e));

        temp_dir
    }

    /// Helper function to clean up test directory
    ///
    /// # Purpose
    /// Removes the test directory and all its contents after test completion
    ///
    /// # Arguments
    /// * `dir` - Path to the directory to remove
    fn cleanup_test_dir(dir: &Path) {
        if dir.exists() {
            let _ = fs::remove_dir_all(dir);
        }
    }

    #[test]
    fn test_archive_file_creation() {
        // Setup: Create temporary test directory and file
        let test_dir = create_temp_test_dir();

        // Ensure test directory exists
        assert!(test_dir.exists(), "Test directory should exist at: {}", test_dir.display());

        let test_file = test_dir.join("test_document.txt");
        fs::write(&test_file, b"Test content for archiving")
            .unwrap_or_else(|e| panic!("Failed to create test file at {}: {}", test_file.display(), e));

        // Verify source file was created
        assert!(test_file.exists(), "Test file should exist at: {}", test_file.display());

        // Create archive subdirectory
        let archive_dir = test_dir.join("archive");
        fs::create_dir_all(&archive_dir)
            .unwrap_or_else(|e| panic!("Failed to create archive dir at {}: {}", archive_dir.display(), e));

        // Verify archive directory was created
        assert!(archive_dir.exists(), "Archive directory should exist at: {}", archive_dir.display());
        assert!(archive_dir.is_dir(), "Archive path should be a directory");

        // Test: Copy file with timestamp using the archive timestamp function
        let timestamp = create_archive_timestamp(SystemTime::now());

        // Validate timestamp format before using it
        println!("Generated timestamp: {}", timestamp);
        assert!(!timestamp.is_empty(), "Timestamp should not be empty");

        let archived_file = archive_dir.join(format!("test_document_{}.txt", timestamp));

        // Debug: Print paths before copy
        println!("Source file: {} (exists: {})", test_file.display(), test_file.exists());
        println!("Archive dir: {} (exists: {})", archive_dir.display(), archive_dir.exists());
        println!("Destination: {}", archived_file.display());

        // Ensure parent directory of destination exists
        if let Some(parent) = archived_file.parent() {
            assert!(parent.exists(), "Parent directory of destination should exist: {}", parent.display());
        }

        let result = fs::copy(&test_file, &archived_file);

        assert!(result.is_ok(), "Failed to copy file: {:?}\nFrom: {}\nTo: {}",
            result, test_file.display(), archived_file.display());
        assert!(archived_file.exists(), "Archived file should exist at: {}", archived_file.display());

        // Verify content matches
        let original_content = fs::read(&test_file)
            .expect("Failed to read original");
        let archived_content = fs::read(&archived_file)
            .expect("Failed to read archive");
        assert_eq!(original_content, archived_content, "Content should match");

        // Cleanup
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_archive_with_prefix() {
        // Setup: Create temporary test directory and file
        let test_dir = create_temp_test_dir();

        // Ensure test directory exists
        assert!(test_dir.exists(), "Test directory should exist at: {}", test_dir.display());

        let test_file = test_dir.join("report.pdf");
        fs::write(&test_file, b"PDF content")
            .unwrap_or_else(|e| panic!("Failed to create test file at {}: {}", test_file.display(), e));

        // Double-check file was created
        assert!(test_file.exists(), "Test file should exist at: {}", test_file.display());
        assert!(test_file.is_file(), "Test file should be a file");

        // Read back to verify write succeeded
        let content = fs::read(&test_file)
            .unwrap_or_else(|e| panic!("Failed to read test file: {}", e));
        assert_eq!(content, b"PDF content", "File content should match what was written");

        // Create archive subdirectory
        let archive_dir = test_dir.join("archive");
        fs::create_dir_all(&archive_dir)
            .unwrap_or_else(|e| panic!("Failed to create archive dir at {}: {}", archive_dir.display(), e));

        // Verify archive directory was created
        assert!(archive_dir.exists(), "Archive directory should exist at: {}", archive_dir.display());
        assert!(archive_dir.is_dir(), "Archive path should be a directory");

        // Test: Manually create file with prefix using the correct archive timestamp
        let prefix = "backup_";
        let timestamp = create_archive_timestamp(SystemTime::now());

        // Validate timestamp
        println!("Generated timestamp: {}", timestamp);
        assert!(!timestamp.is_empty(), "Timestamp should not be empty");

        let file_stem = "report";
        let extension = ".pdf";

        // Build the archived filename: prefix + stem + _ + timestamp + extension
        let archived_name = format!("{}{}_{}{}", prefix, file_stem, timestamp, extension);
        let archived_file = archive_dir.join(&archived_name);

        // Debug: Print all paths and their status before copy
        println!("Source file: {} (exists: {}, is_file: {})",
            test_file.display(),
            test_file.exists(),
            test_file.is_file());
        println!("Archive dir: {} (exists: {}, is_dir: {})",
            archive_dir.display(),
            archive_dir.exists(),
            archive_dir.is_dir());
        println!("Destination filename: {}", archived_name);
        println!("Destination path: {}", archived_file.display());

        // List contents of test directory for debugging
        println!("Test directory contents:");
        if let Ok(entries) = fs::read_dir(&test_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  - {}", entry.path().display());
                }
            }
        }

        // Copy the file to the archive location
        let result = fs::copy(&test_file, &archived_file);
        assert!(result.is_ok(), "Failed to copy file with prefix: {:?}\nSource: {}\nDest: {}",
            result, test_file.display(), archived_file.display());

        // Verify the file exists and has correct prefix
        assert!(archived_file.exists(), "Archived file should exist at: {}", archived_file.display());

        let filename = archived_file.file_name()
            .expect("Should have filename")
            .to_string_lossy();
        assert!(filename.starts_with(prefix), "Filename '{}' should start with prefix '{}'", filename, prefix);

        // Verify the timestamp format is correct (YY_MM_DD_HH_MM_SS)
        assert!(filename.contains('_'), "Filename should contain underscores from timestamp");

        // Verify content integrity
        let original_content = fs::read(&test_file)
            .expect("Failed to read original");
        let archived_content = fs::read(&archived_file)
            .expect("Failed to read archive");
        assert_eq!(original_content, archived_content, "Content should match");

        // Cleanup
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_ensure_archive_directory() {
        // Setup: Create temporary test directory
        let test_dir = create_temp_test_dir();

        // Test: Check that archive directory can be created
        let archive_dir = test_dir.join("archive");
        let result = fs::create_dir_all(&archive_dir);

        assert!(result.is_ok(), "Should create archive directory");
        assert!(archive_dir.exists(), "Archive directory should exist");
        assert!(archive_dir.is_dir(), "Should be a directory");

        // Test: Creating again should not fail (idempotent operation)
        let result_again = fs::create_dir_all(&archive_dir);
        assert!(result_again.is_ok(), "Should succeed when directory already exists");

        // Cleanup
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_archive_nonexistent_file() {
        // Setup: Create temporary test directory
        let test_dir = create_temp_test_dir();
        let archive_dir = test_dir.join("archive");
        fs::create_dir_all(&archive_dir).expect("Failed to create archive dir");

        let nonexistent_file = test_dir.join("nonexistent.txt");
        let target_file = archive_dir.join("copy.txt");

        // Test: Try to copy non-existent file
        let result = fs::copy(&nonexistent_file, &target_file);

        assert!(result.is_err(), "Should fail for non-existent file");

        // Cleanup
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_simple_archive_format() {
        // Setup: Create temporary test directory and file
        let test_dir = create_temp_test_dir();
        let test_file = test_dir.join("data.json");
        let test_content = b"{\"key\": \"value\"}";
        fs::write(&test_file, test_content).expect("Failed to create test file");

        let archive_file = test_dir.join("data.archive");

        // Test: Create simple archive using the documented format
        let mut source = File::open(&test_file).expect("Failed to open source");
        let mut contents = Vec::new();
        source.read_to_end(&mut contents).expect("Failed to read source");

        let mut archive = File::create(&archive_file).expect("Failed to create archive");
        let filename = "data.json";
        let filename_bytes = filename.as_bytes();

        // Write archive format: [filename_length:u32][filename][file_size:u64][file_contents]
        archive.write_all(&(filename_bytes.len() as u32).to_le_bytes())
            .expect("Failed to write name length");
        archive.write_all(filename_bytes)
            .expect("Failed to write filename");
        archive.write_all(&(contents.len() as u64).to_le_bytes())
            .expect("Failed to write content length");
        archive.write_all(&contents)
            .expect("Failed to write contents");

        // Verify archive was created and has expected size
        assert!(archive_file.exists(), "Archive file should exist");
        let archive_size = fs::metadata(&archive_file)
            .expect("Failed to get metadata")
            .len();

        // Expected size: 4 (name length) + 9 (filename) + 8 (content length) + content
        let expected_size = 4 + filename_bytes.len() + 8 + contents.len();
        assert_eq!(archive_size as usize, expected_size, "Archive size should match expected");

        // Cleanup
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_timestamp_format_consistency() {
        // Test that timestamp format is consistent and valid for filenames
        let test_time = SystemTime::now();
        let timestamp = create_archive_timestamp(test_time);

        // Verify format: YY_MM_DD_HH_MM_SS
        let parts: Vec<&str> = timestamp.split('_').collect();
        assert_eq!(parts.len(), 6, "Timestamp should have 6 parts separated by underscores");

        // Verify each part is numeric and has correct length
        assert_eq!(parts[0].len(), 2, "Year should be 2 digits");
        assert_eq!(parts[1].len(), 2, "Month should be 2 digits");
        assert_eq!(parts[2].len(), 2, "Day should be 2 digits");
        assert_eq!(parts[3].len(), 2, "Hour should be 2 digits");
        assert_eq!(parts[4].len(), 2, "Minute should be 2 digits");
        assert_eq!(parts[5].len(), 2, "Second should be 2 digits");

        // Verify all parts are numeric
        for part in parts {
            assert!(part.chars().all(|c| c.is_ascii_digit()),
                "Timestamp part '{}' should only contain digits", part);
        }
    }

    #[test]
    fn test_timestamp_no_invalid_characters() {
        // Ensure timestamp doesn't contain characters invalid for filenames
        let timestamp = create_archive_timestamp(SystemTime::now());

        // List of characters that are problematic in filenames across platforms
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', ' '];

        for invalid_char in &invalid_chars {
            assert!(!timestamp.contains(*invalid_char),
                "Timestamp should not contain invalid character '{}'", invalid_char);
        }
    }
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
        // Calculate items per page based on TUI height settings
        let items_per_page = calculate_items_per_page_from_state(&nav_state);
        let mut dir_view = DirectoryView::new(&directory_entries, items_per_page);
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
                &nav_state, // Pass nav_state for TUI size calculations
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

            // print!("\n>> "); // for extra space, maybe easier to see
            print!(">> "); // saves space
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
                            // IMPORTANT: Clear the selected item when changing directories
                            nav_state.selected_item_index = None;
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
                &current_directory_path,
            ) {
                Ok(action) => {
                    match action {
                        NavigationAction::Refresh => {
                            // Reset navigation state to clean defaults (clear filters, pagination, etc.)
                            nav_state.reset_to_clean_state();
                            break; // Break inner loop to refresh directory
                        },
                        NavigationAction::AdjustTuiSize(adjustment_action) => {
                            // Apply the adjustment to the navigation state
                            apply_tui_adjustment(&mut nav_state, &adjustment_action);

                            // Format current settings for display
                            let (tall_display, wide_display) = format_tui_adjustments(
                                nav_state.tui_tall_adjustment,
                                nav_state.tui_tall_direction_sign,
                                nav_state.tui_wide_adjustment,
                                nav_state.tui_wide_direction_sign
                            );

                            // Provide clear feedback about what changed
                            let dimension_name = if adjustment_action.adjustment_type_true_is_tall_false_is_wide {
                                "Height"
                            } else {
                                "Width"
                            };

                            let change_description = if adjustment_action.adjustment_direction_true_is_positive_false_is_negative {
                                "increased"
                            } else {
                                "decreased"
                            };

                            println!("{} {} by {}. Current settings: {} {}",
                                    dimension_name,
                                    change_description,
                                    adjustment_action.adjustment_magnitude,
                                    tall_display,
                                    wide_display);

                            // Brief pause so user can see the feedback
                            std::thread::sleep(std::time::Duration::from_millis(500));

                            // Break inner loop to refresh display with new settings
                            break;
                        },
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
                            nav_state.selected_item_index = None;  // clears selection upon new directroy
                            break; // Break inner loop to read new directory
                        }
                        NavigationAction::ParentDirectory => {
                            match current_directory_path.parent() {
                                Some(parent) => {
                                    current_directory_path = parent.to_path_buf();
                                    nav_state.current_page_index = 0; // Reset to first page
                                    nav_state.selected_item_index = None;  // clears fields
                                    break; // Break inner loop to read new directory
                                },
                                None => {
                                    println!("Already at root directory");
                                }
                            }
                        }
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

                        // give path blurb
                        NavigationAction::Quit => {
                            // Print the current directory path for cd command
                            println!("\nTo continue from this location, run:");
                            println!("cd {}", current_directory_path.display());
                            return Ok(());
                        },

                        NavigationAction::OpenNewTerminal => {
                            println!("For tmux: Try vsplit | hsplit   Toggle: ctrl+b -> o");
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

                        NavigationAction::VsplitTmux => {
                            // This command should simply create a new vertical pane with your default shell.
                            let _ = Command::new("tmux")
                                // .args(&["split-window", "-v"])
                                .args([
                                    "split-window",
                                    "-v", // "-v" for vertical or "-h" for horizontal
                                          // "vi editor_test.txt", // opens editor and closes split after
                                          // "top", // opens 'top' system monitor, 'q' to close
                                    "-c", &current_directory_path.to_string_lossy(),
                                ])
                                .output()?;
                        },

                        NavigationAction::HsplitTmux => {
                            // This command should create a new horizontal pane with your default shell.
                            let _ = Command::new("tmux")
                                // .args(&["split-window", "-v"])
                                .args([
                                    "split-window",
                                    "-h", // "-v" for vertical or "-h" for horizontal
                                          // "vi editor_test.txt", // opens editor and closes split after
                                          // "top", // opens 'top' system monitor, 'q' to close
                                    "-c", &current_directory_path.to_string_lossy(),
                                ])
                                .output()?;
                        },

                        NavigationAction::ArchiveModeShortcut => {
                            match state_manager.interactive_archive_selection(
                                &nav_state,
                                page_entries,
                                &current_directory_path,
                            ) {
                                Ok(_) => println!("Archive operation completed."),
                                Err(e) => println!("Error during archive operation: {}", e),
                            }
                        },
                        // NEW: Handle Get-Send-Mode
                        NavigationAction::GetSendMode => {

                            // Enter Get-Send-Mode loop
                            loop {
                                match state_manager.interactive_get_send_mode()? {
                                    GetSendModeAction::AddFileToStack => {
                                        // Get currently selected file if any
                                        // let selected_file_path = nav_state.get_selected_item_path();

                                        match state_manager.interactive_add_file_to_stack(
                                            &nav_state,
                                            page_entries,// &all_entries, // Pass ALL page entries for pagination
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
                                    /*
                                    pending future functions to used saved dir-stack items
                                    */
                                    // GetSendModeAction::AddDirectoryToStack => {
                                    //     match state_manager.interactive_save_directory_to_stack(&current_directory_path) {
                                    //         Ok(_) => println!("Directory added to stack successfully."),
                                    //         Err(e) => println!("Error adding directory to stack: {}", e),
                                    //     }
                                    // },
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
                                                        // Restore the complete navigation state including all preferences
                                                        current_directory_path = saved_state.current_directory_path;
                                                        nav_state.current_sort_method = saved_state.current_sort_method;
                                                        nav_state.current_filter = saved_state.current_filter;
                                                        nav_state.selected_item_index = saved_state.selected_item_index;
                                                        nav_state.active_search_term = saved_state.active_search_term;

                                                        // Restore TUI size adjustment settings
                                                        // These settings allow each pocket dimension to have its own optimal display configuration
                                                        nav_state.tui_tall_adjustment = saved_state.tui_tall_adjustment;
                                                        nav_state.tui_tall_direction_sign = saved_state.tui_tall_direction_sign;
                                                        nav_state.tui_wide_adjustment = saved_state.tui_wide_adjustment;
                                                        nav_state.tui_wide_direction_sign = saved_state.tui_wide_direction_sign;

                                                        // Restore pagination state
                                                        nav_state.current_page_index = saved_state.current_page_number;

                                                        // Build a human-readable size adjustment string for display
                                                        let tall_adjustment_display = if saved_state.tui_tall_adjustment == 0 {
                                                            String::from("default")
                                                        } else {
                                                            format!("tall{}{}",
                                                                    if saved_state.tui_tall_direction_sign { "+" } else { "-" },
                                                                    saved_state.tui_tall_adjustment)
                                                        };

                                                        let wide_adjustment_display = if saved_state.tui_wide_adjustment == 0 {
                                                            String::from("default")
                                                        } else {
                                                            format!("wide{}{}",
                                                                    if saved_state.tui_wide_direction_sign { "+" } else { "-" },
                                                                    saved_state.tui_wide_adjustment)
                                                        };

                                                        // Inform user of successful restoration with details
                                                        println!("Jumped to pocket dimension: {} (page {}, size: {} {})",
                                                                nickname,
                                                                saved_state.current_page_number + 1,
                                                                tall_adjustment_display,
                                                                wide_adjustment_display);

                                                        break; // Exit Get-Send-Mode and refresh directory with restored settings
                                                    },
                                                    Err(e) => {
                                                        // Handle restoration errors gracefully
                                                        println!("Error restoring pocket dimension: {}", e);
                                                        println!("Press Enter to continue...");
                                                        let _ = io::stdin().read_line(&mut String::new());
                                                    }
                                                }
                                            },
                                            Ok(None) => {
                                                // User cancelled selection
                                                println!("No pocket dimension selected.");
                                            },
                                            Err(e) => {
                                                // Handle selection errors
                                                println!("Error selecting pocket dimension: {}", e);
                                                println!("Press Enter to continue...");
                                                let _ = io::stdin().read_line(&mut String::new());
                                            }
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
                            println!("Not an item number. Please press Enter to continue...");  // TODO
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
