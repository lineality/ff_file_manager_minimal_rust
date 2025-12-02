// src/lib.rs (or src/ff_file_fantastic_module.rs)

use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::{self};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Module: analyze rows and colums of data file
use super::rows_and_columns_module::rc_analyze_datafile_save_results_to_resultsfile;

// Module: Share Source
use super::source_it_module::{SourcedFile, handle_sourceit_command};

use super::row_line_count_tui_module::show_minimal_linecount_tui;

// mod ribbon_external_counter_module;
use super::ribbon_external_counter_module::CascadingHexCounter;

use super::lines_editor_module::{LinesError, lines_full_file_editor};

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
2. Open the bash shell configuration file in a text editor.
The configuration file is usually located at ~/.bashrc or ~/.bash_profile.
(use whatever editor: vim, nano, hx (helix), gedit, lapce, teehee, lapce, etc.)
```bash
hx ~/.bashrc
```
or in some systems it may be called 'bash_profile'

3. Add an "alias" for your executable at the end of your bash file.
Replace /path/to/your_executable with the path of your executable.
And replace "your_keyword" with whatever you want to call File Fantastic
by typing into your terminal. Add this line (with your details put in):
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
9. memory slim

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
13. Search: -g --grep, -r --recursive, -c --case-sensitive
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
23. headless and tmux support: instead of a new terminal emulator, can use -h terminal editor in same terminal, or new tmux split
24. 'Rows & Columns' -rc to inspect .csv tabular data when opening file

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


## Run Tests
```bash
RUST_TRACEBACK=full cargo test
```
```bash
RUST_TRACEBACK=full cargo test --profile release-performance
```
```bash
RUST_TRACEBACK=full cargo test --profile release-small
```
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
/// / src/main.rs
///
/// / import file fantastic module w/ these 2 lines
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

/*
Error Handling section starts
error section
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
/// / Creating a not found error
/// let error = FileFantasticError::NotFound(path);
///
/// / Converting an IO error
/// let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
/// let error = FileFantasticError::from(io_err);
///
/// / Example error handling
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
    /// I/O operation error with underlying system error
    ///
    /// # Usage
    /// Wraps standard I/O errors from file operations
    /// Example: Permission denied, disk full, etc.
    Io(io::Error),

    /// Path or file not found
    ///
    /// # Usage
    /// When a specified path doesn't exist in the file system
    /// Contains the path that was not found for context
    NotFound(PathBuf),

    /// Permission denied when accessing a file or directory
    /// Operation not permitted
    ///
    /// # Usage
    /// When an operation is blocked by permissions or system policies
    /// More specific than a general I/O error
    PermissionDenied(PathBuf),

    /// Invalid file or directory name
    ///
    /// # Usage
    /// When a file/directory name contains invalid characters
    /// or doesn't meet system requirements
    InvalidName(String),

    /// No suitable terminal found for the current platform
    NoTerminalFound,

    /// Failed to read metadata for a file or directory
    MetadataError(PathBuf),

    /// Failed to launch the specified editor
    EditorLaunchFailed(String),

    /// Current platform is not supported for a specific operation
    UnsupportedPlatform,

    /// Levenshtein distance calculation failed due to string length constraints
    /// Contains the problematic filename and the length that exceeded limits
    LevenshteinError { filename: String, length: usize },

    /// Path or file already exists (operation would overwrite)
    ///
    /// # Usage
    /// When an operation would overwrite an existing file or directory
    /// and overwriting is not desired or safe. Common in copy operations
    /// where we want to prevent accidental data loss.
    ///
    /// # Example
    /// ```rust
    /// if destination_path.exists() {
    ///     return Err(FileFantasticError::AlreadyExists(destination_path));
    /// }
    /// ```
    AlreadyExists(PathBuf),

    /// Wraps a LinesError from the lines editor module
    ///
    /// # Usage
    /// When operations delegate to the lines_full_file_editor and need
    /// to convert its errors into the FileFantasticError type
    LinesError(LinesError),
}

impl std::fmt::Display for FileFantasticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {}", err),
            Self::NotFound(path) => write!(f, "Not found: {}", path.display()),
            Self::PermissionDenied(path) => write!(f, "Permission denied: {}", path.display()),
            Self::InvalidName(name) => write!(f, "Invalid file name: {}", name),
            Self::NoTerminalFound => write!(f, "No suitable terminal emulator found"),
            Self::MetadataError(path) => {
                write!(f, "Failed to read metadata for: {}", path.display())
            }
            Self::EditorLaunchFailed(editor) => write!(f, "Failed to launch editor: {}", editor),
            Self::UnsupportedPlatform => write!(f, "Current platform is not supported"),
            Self::LevenshteinError { filename, length } => write!(
                f,
                "String too long for fuzzy search: '{}' ({} chars, max 1000)",
                // Truncate filename for display if it's too long
                if filename.len() > 50 {
                    format!("{}...", &filename[..50])
                } else {
                    filename.clone()
                },
                length
            ),
            Self::AlreadyExists(path) => {
                // Enhanced error message with suggestion
                write!(
                    f,
                    "Already exists: {}. Consider renaming or removing the existing item first.",
                    path.display()
                )
            }
            Self::LinesError(err) => write!(f, "Lines editor error: {}", err),
        }
    }
}

impl std::error::Error for FileFantasticError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::NotFound(_) => None,
            Self::PermissionDenied(_) => None,
            Self::InvalidName(_) => None,
            Self::AlreadyExists(_) => None,
            Self::NoTerminalFound => None,
            Self::MetadataError(_) => None,
            Self::EditorLaunchFailed(_) => None,
            Self::UnsupportedPlatform => None,
            Self::LevenshteinError { .. } => None,
            Self::LinesError(err) => Some(err),
            // _ => None,
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

impl From<LinesError> for FileFantasticError {
    fn from(err: LinesError) -> Self {
        Self::LinesError(err)
    }
}
// end of error section

// =========
// Utilities
// =========

// experimental formatting... e.g. :<3
/// Formats a message with placeholders supporting alignment and width specifiers.
///
/// ## Project Context
/// Provides string formatting for UI messages, tables, and aligned output using
/// stack-allocated buffers. Supports basic format specifiers for padding and
/// alignment without heap allocation.
///
/// ## Supported Format Specifiers
/// - `{}` - Plain replacement
/// - `{:<N}` - Left-align with width N (pad right with spaces)
/// - `{:>N}` - Right-align with width N (pad left with spaces)
/// - `{:^N}` - Center-align with width N (pad both sides with spaces)
/// - `{:N}` - Default right-align with width N
///
/// Examples:
/// - ("ID: {:<5}", &["42"]) -> "ID: 42   " (left-align, width 5)
/// - ("ID: {:>5}", &["42"]) -> "ID:    42" (right-align, width 5)
/// - ("ID: {:^5}", &["42"]) -> "ID:  42  " (center-align, width 5)
///
/// ## Safety & Error Handling
/// - No panic: Always returns valid string or fallback
/// - No unwrap: All error paths return fallback
/// - Uses 256-byte stack buffer
/// - Returns fallback if result exceeds buffer
/// - Returns fallback if format specifiers are invalid
/// - Maximum 8 inserts supported
///
/// ## Parameters
/// - `template`: String with format placeholders
/// - `inserts`: Slice of strings to insert
/// - `fallback`: Message to return if formatting fails
///
/// ## Returns
/// Formatted string on success, fallback string on any error
///
/// ## Use Examples:
/// ```rust
/// // Table-like alignment
/// let id = "42";
/// let name = "Alice";
/// let row = stack_format_it(
///     "ID: {:<5} Name: {:<10}",
///     &[id, name],
///     "Data unavailable"
/// );
/// // Result: "ID: 42    Name: Alice     "
/// ```
///
///
/// ```rust
/// let bytes = total_bytes_written.saturating_sub(1);
/// let num_str = bytes.to_string();
/// let message = stack_format_it("inserted {} bytes", &[&num_str], "inserted data");
/// ```
///
/// Error Formatting:
/// ```
/// io::stdout().flush().map_err(|e| {
///     LinesError::DisplayError(stack_format_it(
///         "Failed to flush stdout: {}",
///         &[&e.to_string()],
///         "Failed to flush stdout",
///     ))
/// })?;
/// ```
///
/// ```rust
/// let num_1 = start_byte.to_string();
/// let num_2 = end_byte.to_string();
/// let formatted_string = stack_format_it(
///     "Invalid byte range: start={} > end={}",
///     &[&num_1, &num_2],
///     "Invalid byte range"
/// );
/// ```
fn stack_format_it(template: &str, inserts: &[&str], fallback: &str) -> String {
    // Internal stack buffer for result
    let mut buf = [0u8; 512];

    // Maximum number of inserts to prevent abuse
    const MAX_INSERTS: usize = 128;

    // Check insert count
    if inserts.is_empty() {
        #[cfg(debug_assertions)]
        eprintln!("stack_format_it: No inserts provided");
        return fallback.to_string();
    }

    if inserts.len() > MAX_INSERTS {
        #[cfg(debug_assertions)]
        eprintln!("stack_format_it: Too many inserts (max {})", MAX_INSERTS);
        return fallback.to_string();
    }

    // Parse format specifiers and validate
    let format_specs = match parse_format_specs(template, inserts.len()) {
        Some(specs) => specs,
        None => {
            #[cfg(debug_assertions)]
            eprintln!("stack_format_it: Failed to parse format specifiers");
            return fallback.to_string();
        }
    };

    // Build the result
    let mut pos = 0;
    let mut insert_idx = 0;
    let mut search_start = 0;

    while insert_idx < inserts.len() {
        // Find next placeholder
        let placeholder_start = match template[search_start..].find('{') {
            Some(offset) => search_start + offset,
            None => break,
        };

        let placeholder_end = match template[placeholder_start..].find('}') {
            Some(offset) => placeholder_start + offset + 1,
            None => {
                #[cfg(debug_assertions)]
                eprintln!("stack_format_it: Unclosed placeholder");
                return fallback.to_string();
            }
        };

        // Copy text before placeholder
        let before = &template[search_start..placeholder_start];
        if pos + before.len() > buf.len() {
            #[cfg(debug_assertions)]
            eprintln!("stack_format_it: Buffer overflow");
            return fallback.to_string();
        }
        buf[pos..pos + before.len()].copy_from_slice(before.as_bytes());
        pos += before.len();

        // Apply format specifier and insert
        let spec = &format_specs[insert_idx];
        let insert = inserts[insert_idx];

        let formatted = apply_format_spec(insert, spec);

        if pos + formatted.len() > buf.len() {
            #[cfg(debug_assertions)]
            eprintln!("stack_format_it: Buffer overflow during insert");
            return fallback.to_string();
        }
        buf[pos..pos + formatted.len()].copy_from_slice(formatted.as_bytes());
        pos += formatted.len();

        search_start = placeholder_end;
        insert_idx += 1;
    }

    // Copy remaining text after last placeholder
    let remaining = &template[search_start..];
    if pos + remaining.len() > buf.len() {
        #[cfg(debug_assertions)]
        eprintln!("stack_format_it: Buffer overflow during final copy");
        return fallback.to_string();
    }
    buf[pos..pos + remaining.len()].copy_from_slice(remaining.as_bytes());
    pos += remaining.len();

    // Validate UTF-8 and return
    match std::str::from_utf8(&buf[..pos]) {
        Ok(s) => s.to_string(),
        Err(_) => {
            #[cfg(debug_assertions)]
            eprintln!("stack_format_it: Invalid UTF-8 in result");
            fallback.to_string()
        }
    }
}

/// Format specifier parsed from placeholder
#[derive(Debug, Clone, Copy)]
enum Alignment {
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone, Copy)]
struct FormatSpec {
    alignment: Alignment,
    width: Option<usize>,
}

/// Parse format specifiers from template
/// Returns None if parsing fails or placeholder count doesn't match insert count
fn parse_format_specs(template: &str, expected_count: usize) -> Option<Vec<FormatSpec>> {
    let mut specs = Vec::new();
    let mut remaining = template;

    while let Some(start) = remaining.find('{') {
        let end = remaining[start..].find('}')?;
        let placeholder = &remaining[start + 1..start + end];

        let spec = if placeholder.is_empty() {
            // Plain {} placeholder
            FormatSpec {
                alignment: Alignment::Left,
                width: None,
            }
        } else if placeholder.starts_with(':') {
            // Format specifier like {:<5} or {:>10}
            parse_single_spec(&placeholder[1..])?
        } else {
            // Invalid format
            return None;
        };

        specs.push(spec);
        remaining = &remaining[start + end + 1..];
    }

    if specs.len() == expected_count {
        Some(specs)
    } else {
        #[cfg(debug_assertions)]
        eprintln!(
            "parse_format_specs: Placeholder count ({}) doesn't match insert count ({})",
            specs.len(),
            expected_count
        );
        None
    }
}

/// Parse a single format specifier like "<5" or ">10" or "^8"
fn parse_single_spec(spec: &str) -> Option<FormatSpec> {
    if spec.is_empty() {
        return Some(FormatSpec {
            alignment: Alignment::Right,
            width: None,
        });
    }

    let (alignment, width_str) = if spec.starts_with('<') {
        (Alignment::Left, &spec[1..])
    } else if spec.starts_with('>') {
        (Alignment::Right, &spec[1..])
    } else if spec.starts_with('^') {
        (Alignment::Center, &spec[1..])
    } else if spec.chars().next()?.is_ascii_digit() {
        // No alignment character means right-align
        (Alignment::Right, spec)
    } else {
        return None;
    };

    let width = if width_str.is_empty() {
        None
    } else {
        match width_str.parse::<usize>() {
            Ok(w) if w <= 64 => Some(w), // Reasonable width limit
            _ => return None,
        }
    };

    Some(FormatSpec { alignment, width })
}

/// Apply format specifier to a string value
fn apply_format_spec(value: &str, spec: &FormatSpec) -> String {
    let width = match spec.width {
        Some(w) => w,
        None => return value.to_string(), // No width, return as-is
    };

    let value_len = value.len();

    if value_len >= width {
        // Value already meets or exceeds width
        return value.to_string();
    }

    let padding = width - value_len;

    match spec.alignment {
        Alignment::Left => {
            // Pad right: "42   "
            let mut result = String::with_capacity(width);
            result.push_str(value);
            for _ in 0..padding {
                result.push(' ');
            }
            result
        }
        Alignment::Right => {
            // Pad left: "   42"
            let mut result = String::with_capacity(width);
            for _ in 0..padding {
                result.push(' ');
            }
            result.push_str(value);
            result
        }
        Alignment::Center => {
            // Pad both sides: " 42  "
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;
            let mut result = String::with_capacity(width);
            for _ in 0..left_pad {
                result.push(' ');
            }
            result.push_str(value);
            for _ in 0..right_pad {
                result.push(' ');
            }
            result
        }
    }
}

/// Makes, verifies, or creates a directory path relative to the executable directory location.
///
/// This function performs the following sequential steps:
/// 1. Converts the provided directory path string to an absolute path relative to the executable directory
/// 2. Checks if the directory exists at the calculated absolute path location
/// 3. If the directory does not exist, creates it and all necessary parent directories
/// 4. Returns the canonicalized (absolute path with all symlinks resolved) path to the directory
///
/// # Arguments
///
/// * `dir_path_string` - A string representing the directory path relative to the executable directory
///
/// # Returns
///
/// * `Result<PathBuf, std::io::Error>` - The canonicalized absolute path to the directory if successful,
///   or an error if any step fails (executable path determination, directory creation, or canonicalization)
///
/// # Errors
///
/// This function may return an error in the following situations:
/// - If the executable's directory cannot be determined
/// - If directory creation fails due to permissions or other I/O errors
/// - If path canonicalization fails
///
/// use example:
/// // Ensure the project graph data directory exists relative to the executable
/// let project_graph_directory_result = make_verify_or_create_executabledirectoryrelative_canonicalized_dir_path("project_graph_data");

/// // Handle any errors that might occur during directory creation or verification
/// let project_graph_directory = match project_graph_directory_result {
///     Ok(directory_path) => directory_path,
///     Err(io_error) => {
///         // Log the error and handle appropriately for your application
///         return Err(format!("Failed to ensure project graph directory exists: {}", io_error).into());
///     }
/// };
///
pub fn make_verify_or_create_executabledirectoryrelative_canonicalized_dir_path(
    dir_path_string: &str,
) -> Result<PathBuf> {
    // Step 1: Convert the provided directory path to an absolute path relative to the executable
    let absolute_dir_path =
        make_input_path_name_abs_executabledirectoryrelative_nocheck(dir_path_string)?;

    // Step 2: Check if the directory exists at the calculated absolute path
    let directory_exists = abs_executable_directory_relative_exists(&absolute_dir_path)?;

    if !directory_exists {
        // Step 3: Directory doesn't exist, create it and all parent directories
        // Note: mkdir_new_abs_executabledirectoryrelative_canonicalized will also canonicalize the path
        mkdir_new_abs_executabledirectoryrelative_canonicalized(dir_path_string)
    } else {
        absolute_dir_path
            .canonicalize()
            .map_err(|canonicalization_error| {
                FileFantasticError::Io(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Failed to canonicalize existing directory path: {}",
                        canonicalization_error
                    ),
                ))
            })
    }
}

/// Creates a new directory at the specified path relative to the executable directory.
/// Returns an error if the directory already exists.
///
/// # Arguments
///
/// * `dir_path` - The directory path relative to the executable directory
///
/// # Returns
///
/// * `Result<PathBuf, io::Error>` - The absolute, canonicalized path to the newly created directory
pub fn mkdir_new_abs_executabledirectoryrelative_canonicalized<P: AsRef<Path>>(
    dir_path: P,
) -> Result<PathBuf> {
    // Get the absolute path without checking existence
    let abs_path = make_input_path_name_abs_executabledirectoryrelative_nocheck(dir_path)?;

    // Check if the directory already exists
    if abs_executable_directory_relative_exists(&abs_path)? {
        return Err(FileFantasticError::Io(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Directory already exists",
        )));
    }

    // Create the directory and all parent directories
    std::fs::create_dir_all(&abs_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create directory: {}", e),
        )
    })?;

    // Canonicalize the path (should succeed because we just created it)
    abs_path.canonicalize().map_err(|e| {
        FileFantasticError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to canonicalize newly created directory path: {}", e),
        ))
    })
}

/// Converts a path to an absolute path based on the executable's directory location.
/// Does NOT check if the path exists or attempt to create anything.
///
/// # Arguments
///
/// * `path_to_make_absolute` - A path to convert to an absolute path relative to
///   the executable's directory location.
///
/// # Returns
///
/// * `Result<PathBuf, io::Error>` - The absolute path based on the executable's directory or an error
///   if the executable's path cannot be determined or if the path cannot be resolved.
///
/// # Examples
///
/// ```
/// use manage_absolute_executable_directory_relative_paths::make_input_path_name_abs_executabledirectoryrelative_nocheck;
///
/// // Get an absolute path for "data/config.json" relative to the executable directory
/// let abs_path = make_input_path_name_abs_executabledirectoryrelative_nocheck("data/config.json").unwrap();
/// println!("Absolute path: {}", abs_path.display());
/// ```
pub fn make_input_path_name_abs_executabledirectoryrelative_nocheck<P: AsRef<Path>>(
    path_to_make_absolute: P,
) -> Result<PathBuf> {
    // Get the directory where the executable is located
    let executable_directory = get_absolute_path_to_executable_parentdirectory()?;

    // Create a path by joining the executable directory with the provided path
    let target_path = executable_directory.join(path_to_make_absolute);

    // If the path doesn't exist, we still return the absolute path without trying to canonicalize
    if !abs_executable_directory_relative_exists(&target_path)? {
        // Ensure the path is absolute (it should be since we joined with executable_directory)
        if target_path.is_absolute() {
            return Ok(target_path);
        } else {
            // Wrap io::Error in LinesError::Io
            return Err(FileFantasticError::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Failed to create absolute path",
            )));
        }
    }

    // Path exists, so we can canonicalize it to resolve any ".." or "." segments
    target_path.canonicalize().map_err(|e| {
        // Wrap in LinesError::Io
        FileFantasticError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to canonicalize path: {}", e),
        ))
    })
}

/// Gets the directory where the current executable is located.
///
/// # Returns
///
/// * `Result<PathBuf, io::Error>` - The absolute directory path containing the executable or an error
///   if it cannot be determined.
pub fn get_absolute_path_to_executable_parentdirectory() -> Result<PathBuf> {
    // Get the path to the current executable
    let executable_path = std::env::current_exe().map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to determine current executable path: {}", e),
        )
    })?;

    // Get the directory containing the executable
    let executable_directory = executable_path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to determine parent directory of executable",
        )
    })?;

    Ok(executable_directory.to_path_buf())
}
/// Checks if a path exists (either as a file or directory).
///
/// # Arguments
///
/// * `path_to_check` - The path to check for existence
///
/// # Returns
///
/// * `Result<bool, io::Error>` - Whether the path exists or an error
pub fn abs_executable_directory_relative_exists<P: AsRef<Path>>(path_to_check: P) -> Result<bool> {
    let path = path_to_check.as_ref();
    Ok(path.exists())
}

/// Represents the type of file system item retrieved from stack
///
/// # Purpose
/// Used to distinguish between files and directories when retrieving
/// items from stacks, enabling appropriate handling for each type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackItemType {
    /// A regular file
    File,
    /// A directory
    Directory,
}

/// Copies a file from stack to destination with proper handling
///
/// # Purpose
/// Helper function that handles file copying from a stack-retrieved path
/// to a destination directory, with archive handling and error management.
///
/// # Arguments
/// * `source_file_path` - The source file path from stack
/// * `destination_directory` - The destination directory path
///
/// # Returns
/// * `Result<PathBuf>` - The final destination path after copying
///
/// # Features
/// - Handles archive files appropriately
/// - Provides clear progress messages
/// - Returns the final destination path for confirmation
///
/// # Error Handling
/// - Validates source file exists
/// - Validates destination directory exists
/// - Handles permission errors
/// - Manages disk space issues
pub fn copy_file_from_stack(
    source_file_path: &PathBuf,
    destination_directory: &PathBuf,
) -> Result<PathBuf> {
    // Validate source exists and is a file
    if !source_file_path.exists() {
        return Err(FileFantasticError::NotFound(source_file_path.clone()));
    }

    if !source_file_path.is_file() {
        return Err(FileFantasticError::InvalidName(format!(
            "{} is not a file",
            source_file_path.display()
        )));
    }

    // Validate destination directory exists
    if !destination_directory.exists() {
        return Err(FileFantasticError::NotFound(destination_directory.clone()));
    }

    if !destination_directory.is_dir() {
        return Err(FileFantasticError::InvalidName(format!(
            "{} is not a directory",
            destination_directory.display()
        )));
    }

    println!("Copying file: {}", source_file_path.display());
    println!("To directory: {}", destination_directory.display());

    // Use existing copy_file_with_archive_handling function
    // This function already handles archives and returns the final path
    copy_file_with_archive_handling(source_file_path, destination_directory)
}

/// Recursively copies a directory from stack to destination
///
/// # Purpose
/// Helper function that handles recursive directory copying from a stack-retrieved
/// path to a destination directory. Creates a new subdirectory in the destination.
///
/// # Arguments
/// * `source_directory_path` - The source directory path from stack
/// * `destination_directory` - The destination directory where copy will be placed
///
/// # Returns
/// * `Result<PathBuf>` - The final destination directory path after copying
///
/// # Behavior
/// - Creates a new directory with the same name in the destination
/// - Recursively copies all contents (files and subdirectories)
/// - Preserves directory structure
/// - Shows progress for large operations
///
/// # Safety Notes
/// - This function should only be called after user confirmations
/// - The confirmations are handled in `get_directory_from_stack_helper`
/// - This performs the actual copy operation
///
/// # Error Handling
/// - Validates source directory exists
/// - Handles permission errors
/// - Manages disk space issues
/// - Reports on partial failures during recursive copy
pub fn copy_directory_from_stack(
    source_directory_path: &PathBuf,
    destination_directory: &PathBuf,
) -> Result<PathBuf> {
    // Validate source exists and is a directory
    if !source_directory_path.exists() {
        return Err(FileFantasticError::NotFound(source_directory_path.clone()));
    }

    if !source_directory_path.is_dir() {
        return Err(FileFantasticError::InvalidName(format!(
            "{} is not a directory",
            source_directory_path.display()
        )));
    }

    // Validate destination directory exists
    if !destination_directory.exists() {
        return Err(FileFantasticError::NotFound(destination_directory.clone()));
    }

    if !destination_directory.is_dir() {
        return Err(FileFantasticError::InvalidName(format!(
            "{} is not a directory",
            destination_directory.display()
        )));
    }

    // Get the source directory name for the destination
    let dir_name = source_directory_path.file_name().ok_or_else(|| {
        FileFantasticError::InvalidName("Could not extract directory name".to_string())
    })?;

    // Create the full destination path
    let final_destination = destination_directory.join(dir_name);

    // Check if destination already exists
    if final_destination.exists() {
        return Err(FileFantasticError::AlreadyExists(final_destination));
    }

    println!("Starting recursive directory copy...");
    println!("From: {}/", source_directory_path.display());
    println!("To: {}/", final_destination.display());

    // Perform recursive copy
    recursive_copy_directory(source_directory_path, &final_destination)?;

    println!("✓ Directory copy completed successfully!");
    Ok(final_destination)
}

/// Internal function to recursively copy directory contents
///
/// # Purpose
/// Performs the actual recursive copy operation for directories.
/// This is separated from the main function for clarity and reusability.
///
/// # Arguments
/// * `source` - Source directory path
/// * `destination` - Destination directory path (will be created)
///
/// # Returns
/// * `Result<()>` - Success or error with details
///
/// # Implementation Details
/// - Creates destination directory
/// - Iterates through all entries
/// - Recursively handles subdirectories
/// - Copies files individually
/// - Maintains permissions where possible
fn recursive_copy_directory(source: &PathBuf, destination: &PathBuf) -> Result<()> {
    // Create the destination directory
    std::fs::create_dir_all(destination).map_err(|e| FileFantasticError::Io(e))?;

    // Read source directory entries
    let entries = std::fs::read_dir(source).map_err(|e| FileFantasticError::Io(e))?;

    let mut copied_count = 0;
    let mut error_count = 0;

    // Process each entry
    for entry_result in entries {
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Warning: Could not read entry: {}", e);
                error_count += 1;
                continue;
            }
        };

        let entry_path = entry.path();
        let entry_name = entry.file_name();
        let dest_path = destination.join(&entry_name);

        // Check if entry is a directory or file
        match entry.file_type() {
            Ok(file_type) if file_type.is_dir() => {
                // Recursively copy subdirectory
                if let Err(e) = recursive_copy_directory(&entry_path, &dest_path) {
                    eprintln!(
                        "Warning: Failed to copy directory {}: {}",
                        entry_path.display(),
                        e
                    );
                    error_count += 1;
                } else {
                    copied_count += 1;
                }
            }
            Ok(_) => {
                // Copy file
                match std::fs::copy(&entry_path, &dest_path) {
                    Ok(_) => {
                        copied_count += 1;
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to copy file {}: {}",
                            entry_path.display(),
                            e
                        );
                        error_count += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Warning: Could not determine type for {}: {}",
                    entry_path.display(),
                    e
                );
                error_count += 1;
            }
        }
    }

    // Report results
    println!("  Copied {} items", copied_count);
    if error_count > 0 {
        eprintln!("  ⚠️  {} items failed to copy", error_count);
        // Don't fail the entire operation for partial errors
    }

    Ok(())
}

/// Creates a zip file containing a single file using platform-appropriate system commands
///
/// # Purpose
/// Executes platform-specific zip creation commands to compress a single file into a proper
/// zip archive, avoiding external dependencies while providing cross-platform functionality.
/// Unlike `create_simple_file_archive()`, this creates genuine zip files compatible with
/// standard zip tools.
///
/// # Arguments
/// * `source_file_path` - Path to the file to compress
/// * `zip_destination_path` - Output path for the zip file
///
/// # Returns
/// * `Result<bool>` - True if zip creation succeeded, false if failed
///
/// # Platform Implementation
/// - **Linux/macOS/Android/BSD**: Uses `zip` command to compress single file
/// - **Windows**: Uses PowerShell `Compress-Archive` cmdlet
///
/// # Command Details
/// ## Linux/macOS/Android/BSD
/// ```bash
/// zip "output.zip" "source_file.txt"
/// ```
/// The file will be stored in the zip with its original name (no directory structure).
///
/// ## Windows
/// ```powershell
/// Compress-Archive -Path "source_file.txt" -DestinationPath "output.zip"
/// ```
///
/// # Zip Structure
/// The created zip file will contain the single file with its original filename,
/// stored at the root level of the archive (no directory paths).
///
/// # Error Handling
/// - Validates source file exists and is a file (not directory)
/// - Handles command execution failures
/// - Checks exit status of zip commands
/// - Provides platform-specific error context and installation hints
///
/// # Example
/// ```rust
/// let source_file = PathBuf::from("/home/user/document.txt");
/// let zip_file = PathBuf::from("/home/user/document_backup.zip");
///
/// match create_single_file_zip_archive_via_command(&source_file, &zip_file) {
///     Ok(true) => println!("File zipped successfully"),
///     Ok(false) => println!("Zip command failed"),
///     Err(e) => eprintln!("Error executing zip command: {}", e),
/// }
/// ```
///
/// # Notes
/// - Source file must exist and be a regular file (not a directory or symlink)
/// - Destination directory must be writable
/// - Requires zip utilities to be installed on the system
fn create_single_file_zip_archive_via_command(
    source_file_path: &Path,
    zip_destination_path: &Path,
) -> Result<bool> {
    // Validate source file exists
    if !source_file_path.exists() {
        return Err(FileFantasticError::NotFound(source_file_path.to_path_buf()));
    }

    // Validate source is actually a file (not directory)
    if !source_file_path.is_file() {
        return Err(FileFantasticError::InvalidName(format!(
            "Source is not a file: {}",
            source_file_path.display()
        )));
    }

    // Validate destination directory exists (create parent directories if needed)
    if let Some(parent_dir) = zip_destination_path.parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).map_err(|e| {
                eprintln!(
                    "Failed to create destination directory: {}",
                    parent_dir.display()
                );
                FileFantasticError::Io(e)
            })?;
        }
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "android",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "redox",
    ))]
    {
        // Use zip command on Unix-like systems for single file
        // Note: We use -j flag to store just the file without directory path
        let output = std::process::Command::new("zip")
            .arg("-j") // Store files without directory structure
            .arg(zip_destination_path) // Output zip file
            .arg(source_file_path) // Input file to zip
            .output()
            .map_err(|e| {
                eprintln!("Failed to execute zip command: {}", e);
                eprintln!("Make sure 'zip' is installed on your system");

                // Provide platform-specific installation hints
                if cfg!(target_os = "android") {
                    eprintln!("On Termux, install zip with: pkg install zip");
                }
                #[cfg(target_os = "freebsd")]
                eprintln!("On FreeBSD, install zip with: pkg install zip");
                #[cfg(target_os = "openbsd")]
                eprintln!("On OpenBSD, install zip with: pkg_add zip");
                #[cfg(target_os = "netbsd")]
                eprintln!("On NetBSD, install zip with: pkgin install zip");
                #[cfg(target_os = "dragonfly")]
                eprintln!("On DragonFly BSD, install zip with: pkg install zip");
                #[cfg(target_os = "redox")]
                eprintln!("On Redox OS, install zip through the package manager");
                #[cfg(any(target_os = "linux", target_os = "macos"))]
                {
                    eprintln!("On Ubuntu/Debian: apt install zip");
                    eprintln!("On RHEL/CentOS/Fedora: yum install zip (or dnf install zip)");
                    eprintln!("On macOS: brew install zip (or use built-in zip)");
                }

                FileFantasticError::Io(e)
            })?;

        if output.status.success() {
            println!(
                "Single file archived successfully: {}",
                zip_destination_path.display()
            );
            Ok(true)
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            eprintln!("Zip command failed for single file: {}", error_msg);
            eprintln!("Source file: {}", source_file_path.display());
            eprintln!("Destination zip: {}", zip_destination_path.display());
            Ok(false)
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Use PowerShell Compress-Archive on Windows for single file
        // PowerShell handles single files naturally
        let output = std::process::Command::new("powershell")
            .arg("-Command")
            .arg(&format!(
                "Compress-Archive -Path '{}' -DestinationPath '{}' -Force",
                source_file_path.display(),
                zip_destination_path.display()
            ))
            .output()
            .map_err(|e| {
                eprintln!(
                    "Failed to execute PowerShell Compress-Archive command: {}",
                    e
                );
                eprintln!("Make sure PowerShell is available on your Windows system");
                FileFantasticError::Io(e)
            })?;

        if output.status.success() {
            println!(
                "Single file archived successfully: {}",
                zip_destination_path.display()
            );
            Ok(true)
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            eprintln!(
                "PowerShell Compress-Archive failed for single file: {}",
                error_msg
            );
            eprintln!("Source file: {}", source_file_path.display());
            eprintln!("Destination zip: {}", zip_destination_path.display());
            Ok(false)
        }
    }

    // Fallback for unsupported platforms
    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "android",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "redox",
        target_os = "windows"
    )))]
    {
        eprintln!("Zip creation not supported on this platform");
        Err(FileFantasticError::InvalidName(
            "Platform not supported for zip creation".to_string(),
        ))
    }
}

#[cfg(test)]
mod create_single_file_zip_archive_via_comtests {
    use super::*;
    // use std::fs::File;
    // use std::io::Write;
    // use tempfile::TempDir;

    // #[test]
    // fn test_create_single_file_zip_archive_via_command() -> Result<()> {
    //     // Create temporary directory for test
    //     let temp_dir = TempDir::new().map_err(|e| FileFantasticError::Io(e))?;
    //     let temp_path = temp_dir.path();

    //     // Create a test file
    //     let test_file_path = temp_path.join("test_file.txt");
    //     let mut test_file = File::create(&test_file_path).map_err(|e| FileFantasticError::Io(e))?;
    //     writeln!(test_file, "This is a test file for zip archiving.")
    //         .map_err(|e| FileFantasticError::Io(e))?;
    //     writeln!(test_file, "It contains multiple lines.")
    //         .map_err(|e| FileFantasticError::Io(e))?;
    //     writeln!(test_file, "Line three for good measure.")
    //         .map_err(|e| FileFantasticError::Io(e))?;

    //     // Create zip destination path
    //     let zip_path = temp_path.join("test_file.zip");

    //     // Test the function
    //     match create_single_file_zip_archive_via_command(&test_file_path, &zip_path) {
    //         Ok(true) => {
    //             // Verify zip file was created
    //             assert!(zip_path.exists(), "Zip file should exist after creation");
    //             assert!(zip_path.is_file(), "Zip path should be a file");

    //             // Check file size is reasonable (should be smaller than original due to compression)
    //             let original_size = std::fs::metadata(&test_file_path)
    //                 .map_err(|e| FileFantasticError::Io(e))?
    //                 .len();
    //             let zip_size = std::fs::metadata(&zip_path)
    //                 .map_err(|e| FileFantasticError::Io(e))?
    //                 .len();

    //             // Zip file should exist and have some content (header + compressed data)
    //             assert!(zip_size > 0, "Zip file should not be empty");
    //             println!(
    //                 "Original file: {} bytes, Zip file: {} bytes",
    //                 original_size, zip_size
    //             );
    //         }
    //         Ok(false) => {
    //             println!(
    //                 "Zip command failed - this might be expected if zip tools are not installed"
    //             );
    //             // This is not necessarily a test failure - zip tools might not be available
    //         }
    //         Err(e) => {
    //             println!("Error during zip creation: {}", e);
    //             // This might be expected in CI environments without zip tools
    //         }
    //     }

    //     Ok(())
    // }

    #[test]
    fn test_create_single_file_zip_archive_nonexistent_file() {
        let nonexistent_file = Path::new("/definitely/does/not/exist/file.txt");
        let zip_path = Path::new("/tmp/should_not_be_created.zip");

        let result = create_single_file_zip_archive_via_command(nonexistent_file, zip_path);

        // Should return NotFound error
        assert!(result.is_err());
        match result {
            Err(FileFantasticError::NotFound(_)) => {
                println!("Correctly detected non-existent source file");
            }
            _ => panic!("Should have returned NotFound error for non-existent file"),
        }
    }

    // #[test]
    // fn test_create_single_file_zip_archive_directory_as_source() -> Result<()> {
    //     // Create temporary directory for test
    //     let temp_dir = TempDir::new().map_err(|e| FileFantasticError::Io(e))?;
    //     let temp_path = temp_dir.path();

    //     // Create a subdirectory (not a file)
    //     let test_dir_path = temp_path.join("test_directory");
    //     std::fs::create_dir(&test_dir_path).map_err(|e| FileFantasticError::Io(e))?;

    //     // Create zip destination path
    //     let zip_path = temp_path.join("should_fail.zip");

    //     // Test the function with directory as source (should fail)
    //     let result = create_single_file_zip_archive_via_command(&test_dir_path, &zip_path);

    //     // Should return InvalidName error because source is directory, not file
    //     assert!(result.is_err());
    //     match result {
    //         Err(FileFantasticError::InvalidName(msg)) => {
    //             assert!(msg.contains("not a file"));
    //             println!("Correctly detected directory as source: {}", msg);
    //         }
    //         _ => panic!("Should have returned InvalidName error for directory source"),
    //     }

    //     Ok(())
    // }
}

/// Result of retrieving an item from stack with type information
///
/// # Purpose
/// Bundles together the item type and path for stack retrieval operations,
/// allowing the caller to handle files and directories appropriately.
///
/// # Fields
/// * `item_type` - Whether the item is a file or directory
/// * `path` - The path to the item
#[derive(Debug, Clone)]
pub struct StackRetrievalResult {
    /// The type of the retrieved item
    pub item_type: StackItemType,
    /// The path to the retrieved item
    pub path: PathBuf,
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
/// * `u16` - The calculated column width in characters, never less than minimum_width
///
/// # Calculation Logic
/// 1. Start with MAX_NAME_LENGTH_DEFAULT (55 characters)
/// 2. Apply adjustment in the specified direction using saturating arithmetic
/// 3. Enforce minimum width of 8 characters (FILENAME_SUFFIX_LENGTH + 3 for "...")
/// 4. **Always** enforce minimum width regardless of adjustment direction
///
/// # Minimum Width Rationale
/// The minimum of 8 characters ensures we can always display:
/// - 3 characters for ellipsis "..."
/// - 5 characters for file suffix (e.g., ".docx")
/// This prevents display corruption with very narrow columns.
///
/// # Safety Guarantees
/// - Uses saturating arithmetic to prevent overflow/underflow
/// - Always enforces minimum width constraint in both directions
/// - Validates configuration assumptions in debug builds
///
/// # Examples
/// ```rust
/// / Default width (no adjustment)
/// assert_eq!(calculate_name_width(0, true), 55);
///
/// / Increase width by 10
/// assert_eq!(calculate_name_width(10, true), 65);
///
/// / Decrease width by 50 (hits minimum)
/// assert_eq!(calculate_name_width(50, false), 8);
///
/// / Maximum possible width
/// assert_eq!(calculate_name_width(u16::MAX, true), u16::MAX);
/// ```
fn calculate_name_width(
    adjustment_magnitude: u16,
    adjustment_direction_true_is_positive_false_is_negative: bool,
) -> u16 {
    // Convert default to u16 for calculation
    let base_width: u16 = MAX_NAME_LENGTH_DEFAULT as u16;

    // Calculate minimum allowed width (suffix + ellipsis)
    let minimum_width: u16 = (FILENAME_SUFFIX_LENGTH + 3) as u16;

    // Validate configuration assumptions in debug builds
    // This catches configuration errors during development
    debug_assert!(
        base_width >= minimum_width,
        "Configuration error: MAX_NAME_LENGTH_DEFAULT ({}) must be >= minimum required width ({}). \
         Minimum width = FILENAME_SUFFIX_LENGTH ({}) + 3 chars for ellipsis = {}",
        base_width,
        minimum_width,
        FILENAME_SUFFIX_LENGTH,
        minimum_width
    );

    // Apply adjustment based on direction using saturating arithmetic
    let adjusted_width = if adjustment_direction_true_is_positive_false_is_negative {
        // Positive adjustment: add to base width
        // saturating_add prevents overflow, returns u16::MAX if result would overflow
        base_width.saturating_add(adjustment_magnitude)
    } else {
        // Negative adjustment: subtract from base width
        // saturating_sub prevents underflow, returns 0 if result would be negative
        base_width.saturating_sub(adjustment_magnitude)
    };

    // CRITICAL: Always enforce minimum width regardless of adjustment direction
    // This ensures display integrity and prevents configuration-dependent bugs
    adjusted_width.max(minimum_width)
}

#[cfg(test)]
mod calculate_name_width_tests {
    use super::*;

    /// Test basic functionality with no adjustments
    #[test]
    fn test_baseline_no_adjustment() {
        let expected_width = MAX_NAME_LENGTH_DEFAULT as u16;
        let min_width = (FILENAME_SUFFIX_LENGTH + 3) as u16;

        // Both directions should return same value with zero adjustment
        let result_positive = calculate_name_width(0, true);
        let result_negative = calculate_name_width(0, false);

        // Should return at least the minimum width
        assert!(
            result_positive >= min_width,
            "Zero positive adjustment returned {} but minimum is {}",
            result_positive,
            min_width
        );
        assert!(
            result_negative >= min_width,
            "Zero negative adjustment returned {} but minimum is {}",
            result_negative,
            min_width
        );

        // If configuration is correct, should return expected width
        if expected_width >= min_width {
            assert_eq!(result_positive, expected_width);
            assert_eq!(result_negative, expected_width);
        } else {
            // If base width is less than minimum, should return minimum
            assert_eq!(result_positive, min_width);
            assert_eq!(result_negative, min_width);
        }
    }

    /// Test positive adjustments of various sizes
    #[test]
    fn test_positive_adjustments() {
        let base_width = MAX_NAME_LENGTH_DEFAULT as u16;
        let min_width = (FILENAME_SUFFIX_LENGTH + 3) as u16;

        // Small positive adjustment
        let result_small = calculate_name_width(1, true);
        let expected_small = base_width.saturating_add(1).max(min_width);
        assert_eq!(
            result_small, expected_small,
            "Small positive adjustment failed"
        );

        // Medium positive adjustment
        let result_medium = calculate_name_width(25, true);
        let expected_medium = base_width.saturating_add(25).max(min_width);
        assert_eq!(
            result_medium, expected_medium,
            "Medium positive adjustment failed"
        );

        // Large positive adjustment
        let result_large = calculate_name_width(1000, true);
        let expected_large = base_width.saturating_add(1000).max(min_width);
        assert_eq!(
            result_large, expected_large,
            "Large positive adjustment failed"
        );

        // All results should be at least minimum width
        assert!(result_small >= min_width);
        assert!(result_medium >= min_width);
        assert!(result_large >= min_width);
    }

    /// Test negative adjustments of various sizes
    #[test]
    fn test_negative_adjustments() {
        let base_width = MAX_NAME_LENGTH_DEFAULT as u16;
        let min_width = (FILENAME_SUFFIX_LENGTH + 3) as u16;

        // Small negative adjustment (shouldn't hit minimum)
        let small_adjustment = if base_width > min_width + 5 { 5 } else { 1 };
        let result_small = calculate_name_width(small_adjustment, false);
        assert!(
            result_small >= min_width,
            "Small negative adjustment returned less than minimum"
        );

        // Medium negative adjustment
        let result_medium = calculate_name_width(20, false);
        let expected_medium = base_width.saturating_sub(20).max(min_width);
        assert_eq!(
            result_medium, expected_medium,
            "Medium negative adjustment failed"
        );

        // Large negative adjustment (should hit minimum)
        let result_large = calculate_name_width(base_width + 100, false);
        assert_eq!(
            result_large, min_width,
            "Large negative adjustment should return minimum width"
        );
    }

    /// Test overflow protection with maximum values
    #[test]
    fn test_overflow_protection() {
        // Maximum positive adjustment should not panic or wrap around
        let result_max = calculate_name_width(u16::MAX, true);

        // Should be either u16::MAX or saturated to u16::MAX
        let base_width = MAX_NAME_LENGTH_DEFAULT as u16;
        let expected_max = base_width.saturating_add(u16::MAX);
        assert_eq!(
            result_max, expected_max,
            "Maximum positive adjustment should saturate properly"
        );

        // Near-maximum adjustment that would overflow
        if base_width < u16::MAX {
            let overflow_adjustment = u16::MAX - base_width + 1;
            let result_overflow = calculate_name_width(overflow_adjustment, true);
            assert_eq!(
                result_overflow,
                u16::MAX,
                "Overflow adjustment should saturate to u16::MAX"
            );
        }
    }

    /// Test underflow protection with maximum negative values
    #[test]
    fn test_underflow_protection() {
        let min_width = (FILENAME_SUFFIX_LENGTH + 3) as u16;

        // Maximum negative adjustment should hit minimum, not underflow
        let result_max_negative = calculate_name_width(u16::MAX, false);
        assert_eq!(
            result_max_negative, min_width,
            "Maximum negative adjustment should return minimum width"
        );

        // Various large negative adjustments should all return minimum
        for large_adjustment in [1000, 5000, u16::MAX / 2, u16::MAX] {
            let result = calculate_name_width(large_adjustment, false);
            assert_eq!(
                result, min_width,
                "Large negative adjustment {} should return minimum width",
                large_adjustment
            );
        }
    }

    /// Test minimum width enforcement is consistent
    #[test]
    fn test_minimum_width_always_enforced() {
        let min_width = (FILENAME_SUFFIX_LENGTH + 3) as u16;

        // Test various adjustment magnitudes in both directions
        for magnitude in [0, 1, 10, 100, 1000, u16::MAX] {
            let result_positive = calculate_name_width(magnitude, true);
            let result_negative = calculate_name_width(magnitude, false);

            assert!(
                result_positive >= min_width,
                "Positive adjustment {} returned {} which is less than minimum {}",
                magnitude,
                result_positive,
                min_width
            );

            assert!(
                result_negative >= min_width,
                "Negative adjustment {} returned {} which is less than minimum {}",
                magnitude,
                result_negative,
                min_width
            );
        }
    }

    /// Test boundary values and edge cases
    #[test]
    fn test_boundary_values() {
        let base_width = MAX_NAME_LENGTH_DEFAULT as u16;
        let min_width = (FILENAME_SUFFIX_LENGTH + 3) as u16;

        // Test adjustment of exactly 1
        let plus_one = calculate_name_width(1, true);
        let minus_one = calculate_name_width(1, false);

        assert_eq!(plus_one, base_width.saturating_add(1).max(min_width));
        assert_eq!(minus_one, base_width.saturating_sub(1).max(min_width));

        // Test adjustment equal to base width
        let result_equal = calculate_name_width(base_width, false);
        assert_eq!(
            result_equal, min_width,
            "Adjustment equal to base width should return minimum"
        );

        // Test u16 boundaries
        assert!(calculate_name_width(0, true) <= u16::MAX);
        assert!(calculate_name_width(0, false) <= u16::MAX);
        assert!(calculate_name_width(u16::MAX, true) <= u16::MAX);
        assert!(calculate_name_width(u16::MAX, false) <= u16::MAX);
    }

    /// Test function determinism and consistency
    #[test]
    fn test_deterministic_behavior() {
        // Function should always return the same result for same inputs
        let test_cases = [
            (0, true),
            (0, false),
            (10, true),
            (10, false),
            (100, true),
            (100, false),
            (u16::MAX, true),
            (u16::MAX, false),
        ];

        for (magnitude, direction) in test_cases {
            let result1 = calculate_name_width(magnitude, direction);
            let result2 = calculate_name_width(magnitude, direction);
            let result3 = calculate_name_width(magnitude, direction);

            assert_eq!(
                result1, result2,
                "Function not deterministic for magnitude={}, direction={}",
                magnitude, direction
            );
            assert_eq!(
                result2, result3,
                "Function not deterministic for magnitude={}, direction={}",
                magnitude, direction
            );
        }
    }

    /// Test minimum width calculation correctness
    #[test]
    fn test_minimum_width_calculation() {
        let calculated_min = (FILENAME_SUFFIX_LENGTH + 3) as u16;

        // Verify minimum allows for ellipsis + suffix as documented
        assert!(
            calculated_min >= 8,
            "Minimum width {} should be at least 8 characters",
            calculated_min
        );

        // Verify the calculation makes sense
        let suffix_len = FILENAME_SUFFIX_LENGTH as u16;
        assert_eq!(
            calculated_min,
            suffix_len + 3,
            "Minimum width calculation is incorrect"
        );

        // Test that function actually uses this minimum
        let result = calculate_name_width(u16::MAX, false);
        assert_eq!(
            result, calculated_min,
            "Function doesn't use calculated minimum width"
        );
    }

    /// Test configuration validation (will only fail in debug builds)
    #[cfg(debug_assertions)]
    #[test]
    fn test_configuration_assumptions() {
        let base_width = MAX_NAME_LENGTH_DEFAULT as u16;
        let min_width = (FILENAME_SUFFIX_LENGTH + 3) as u16;

        // This test verifies our configuration is sane
        // If this fails, the constants need to be adjusted
        assert!(
            base_width >= min_width,
            "Configuration error: MAX_NAME_LENGTH_DEFAULT ({}) must be >= minimum width ({})",
            base_width,
            min_width
        );

        // Test that the function works correctly with current configuration
        let result = calculate_name_width(0, true);
        assert_eq!(
            result, base_width,
            "With valid configuration, zero adjustment should return base width"
        );
    }

    /// Test mathematical properties and invariants
    #[test]
    fn test_mathematical_invariants() {
        let min_width = (FILENAME_SUFFIX_LENGTH + 3) as u16;

        // Invariant: result is always >= minimum width
        for magnitude in [0, 1, 10, 100, 1000] {
            for direction in [true, false] {
                let result = calculate_name_width(magnitude, direction);
                assert!(
                    result >= min_width,
                    "Invariant violation: result {} < minimum {} for magnitude={}, direction={}",
                    result,
                    min_width,
                    magnitude,
                    direction
                );
            }
        }

        // Invariant: positive adjustments never decrease width (unless hitting boundaries)
        let base_result = calculate_name_width(0, true);
        for magnitude in [1, 5, 10] {
            let adjusted_result = calculate_name_width(magnitude, true);
            if base_result < u16::MAX - magnitude {
                assert!(
                    adjusted_result >= base_result,
                    "Positive adjustment should not decrease width"
                );
            }
        }
    }

    /// Test extreme scenarios and stress conditions
    #[test]
    fn test_extreme_scenarios() {
        // Test with all possible boolean values explicitly
        assert!(calculate_name_width(0, true) >= (FILENAME_SUFFIX_LENGTH + 3) as u16);
        assert!(calculate_name_width(0, false) >= (FILENAME_SUFFIX_LENGTH + 3) as u16);

        // Test alternating large adjustments don't cause issues
        let large_pos = calculate_name_width(10000, true);
        let large_neg = calculate_name_width(10000, false);

        assert!(large_pos <= u16::MAX);
        assert!(large_neg == (FILENAME_SUFFIX_LENGTH + 3) as u16);

        // Test that we can handle the full range of u16
        let min_adjust = calculate_name_width(u16::MIN, true); // 0
        let max_adjust = calculate_name_width(u16::MAX, true);

        assert!(min_adjust >= (FILENAME_SUFFIX_LENGTH + 3) as u16);
        assert_eq!(max_adjust, u16::MAX);
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
/// / Default height (no adjustment)
/// assert_eq!(calculate_items_per_page(0, true), 16);
///
/// / Increase by 10 rows
/// assert_eq!(calculate_items_per_page(10, true), 26);
///
/// / Decrease by 20 rows (goes to 0)
/// assert_eq!(calculate_items_per_page(20, false), 0);
///
/// / Maximum possible items
/// assert_eq!(calculate_items_per_page(u16::MAX, true), u16::MAX);
/// ```
fn calculate_items_per_page(
    adjustment_magnitude: u16,
    adjustment_direction_true_is_positive_false_is_negative: bool,
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

#[cfg(test)]
mod calculate_items_per_page_tests {
    use super::*;

    /// Test default behavior with no adjustment
    #[test]
    fn test_calculate_items_per_page_no_adjustment() {
        let result = calculate_items_per_page(0, true);
        assert_eq!(result, ITEMS_PER_PAGE_DEFAULT as u16);

        let result = calculate_items_per_page(0, false);
        assert_eq!(result, ITEMS_PER_PAGE_DEFAULT as u16);
    }

    /// Test positive adjustments within normal range
    #[test]
    fn test_calculate_items_per_page_positive_adjustments() {
        // Small positive adjustment
        let result = calculate_items_per_page(5, true);
        assert_eq!(result, (ITEMS_PER_PAGE_DEFAULT as u16) + 5);

        // Medium positive adjustment
        let result = calculate_items_per_page(100, true);
        assert_eq!(result, (ITEMS_PER_PAGE_DEFAULT as u16) + 100);

        // Large positive adjustment
        let result = calculate_items_per_page(1000, true);
        assert_eq!(result, (ITEMS_PER_PAGE_DEFAULT as u16) + 1000);
    }

    /// Test negative adjustments within normal range
    #[test]
    fn test_calculate_items_per_page_negative_adjustments() {
        // Small negative adjustment
        let result = calculate_items_per_page(5, false);
        assert_eq!(result, (ITEMS_PER_PAGE_DEFAULT as u16) - 5);

        // Medium negative adjustment (assuming ITEMS_PER_PAGE_DEFAULT >= 10)
        let result = calculate_items_per_page(10, false);
        assert_eq!(result, (ITEMS_PER_PAGE_DEFAULT as u16) - 10);
    }

    /// Test overflow protection with positive adjustments
    #[test]
    fn test_calculate_items_per_page_positive_overflow_protection() {
        // Test maximum possible adjustment
        let result = calculate_items_per_page(u16::MAX, true);
        assert_eq!(result, u16::MAX);

        // Test adjustment that would cause overflow
        let large_adjustment = u16::MAX - (ITEMS_PER_PAGE_DEFAULT as u16) + 1;
        let result = calculate_items_per_page(large_adjustment, true);
        assert_eq!(result, u16::MAX);
    }

    /// Test underflow protection with negative adjustments
    #[test]
    fn test_calculate_items_per_page_negative_underflow_protection() {
        // Test adjustment equal to base (should result in 0)
        let result = calculate_items_per_page(ITEMS_PER_PAGE_DEFAULT as u16, false);
        assert_eq!(result, 0);

        // Test adjustment greater than base (should result in 0)
        let result = calculate_items_per_page((ITEMS_PER_PAGE_DEFAULT as u16) + 1, false);
        assert_eq!(result, 0);

        // Test maximum negative adjustment
        let result = calculate_items_per_page(u16::MAX, false);
        assert_eq!(result, 0);
    }

    /// Test boundary conditions
    #[test]
    fn test_calculate_items_per_page_boundary_conditions() {
        // Test minimum non-zero result
        let adjustment = (ITEMS_PER_PAGE_DEFAULT as u16) - 1;
        let result = calculate_items_per_page(adjustment, false);
        assert_eq!(result, 1);

        // Test one less than overflow
        let adjustment = u16::MAX - (ITEMS_PER_PAGE_DEFAULT as u16);
        let result = calculate_items_per_page(adjustment, true);
        assert_eq!(result, u16::MAX);
    }

    /// Test that zero items per page is explicitly allowed
    #[test]
    fn test_calculate_items_per_page_zero_items_allowed() {
        let result = calculate_items_per_page(ITEMS_PER_PAGE_DEFAULT as u16, false);
        assert_eq!(
            result, 0,
            "Zero items per page should be allowed for header-only display"
        );
    }

    /// Test consistency of adjustment direction parameter
    #[test]
    fn test_calculate_items_per_page_direction_consistency() {
        let base = ITEMS_PER_PAGE_DEFAULT as u16;
        let adjustment = 5;

        let positive_result = calculate_items_per_page(adjustment, true);
        let negative_result = calculate_items_per_page(adjustment, false);

        assert_eq!(positive_result, base + adjustment);
        assert_eq!(negative_result, base - adjustment);
        assert!(
            positive_result > negative_result,
            "Positive adjustment should yield larger result"
        );
    }

    /// Property-based test for saturating arithmetic
    #[test]
    fn test_calculate_items_per_page_saturating_arithmetic_properties() {
        let base = ITEMS_PER_PAGE_DEFAULT as u16;

        // For any adjustment, positive direction should never be less than base
        for adjustment in [1, 100, 1000, u16::MAX] {
            let result = calculate_items_per_page(adjustment, true);
            assert!(
                result >= base,
                "Positive adjustment result should be >= base"
            );
        }

        // For any adjustment, negative direction should never be greater than base
        for adjustment in [1, 100, 1000, u16::MAX] {
            let result = calculate_items_per_page(adjustment, false);
            assert!(
                result <= base,
                "Negative adjustment result should be <= base"
            );
        }
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
        nav_state.tui_wide_direction_sign,
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
        nav_state.tui_tall_direction_sign,
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
/// / Basic archive
/// let result = create_directory_zip_archive(&source, &dest, None);
/// / Creates: my_project_2025_01_15_14_30_45.zip
///
/// / Custom named archive
/// let result = create_directory_zip_archive(&source, &dest, Some("backup"));
/// / Creates: my_project_backup_2025_01_15_14_30_45.zip
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
        return Err(FileFantasticError::InvalidName(format!(
            "Source is not a directory: {}",
            source_directory_path.display()
        )));
    }

    // Validate destination directory exists and is writable
    if !destination_directory_path.exists() {
        return Err(FileFantasticError::NotFound(
            destination_directory_path.clone(),
        ));
    }

    if !destination_directory_path.is_dir() {
        return Err(FileFantasticError::InvalidName(format!(
            "Destination is not a directory: {}",
            destination_directory_path.display()
        )));
    }

    // Extract source directory name
    let source_directory_name = source_directory_path
        .file_name()
        .ok_or_else(|| {
            FileFantasticError::InvalidName(format!(
                "Cannot determine directory name from: {}",
                source_directory_path.display()
            ))
        })?
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
            format!(
                "{}_{}_{}.zip",
                source_directory_name,
                custom_name.trim(),
                timestamp
            )
        }
    } else {
        // No custom name, use standard format
        format!("{}_{}.zip", source_directory_name, timestamp)
    };

    let zip_destination_path = destination_directory_path.join(&zip_filename);

    // Create zip archive using system commands
    let zip_result =
        create_zip_of_dir_with_system_command(source_directory_path, &zip_destination_path)?;

    if zip_result {
        println!("Directory archived: {}", zip_destination_path.display());
        Ok(zip_destination_path)
    } else {
        Err(FileFantasticError::InvalidName(
            "Zip creation failed".to_string(),
        ))
    }
}

#[cfg(test)]
mod create_directory_zip_archive_tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    /// Test successful creation of zip archive without custom name
    /// Note: This test requires that create_zip_of_dir_with_system_command actually works
    #[test]
    fn test_create_zip_archive_validates_source_exists() {
        // Test with non-existent source
        let nonexistent_source = PathBuf::from("/definitely/does/not/exist/path");
        let some_dest = PathBuf::from("/tmp"); // Assuming /tmp exists on Unix-like systems

        let result = create_directory_zip_archive(&nonexistent_source, &some_dest, None);

        assert!(result.is_err(), "Should error when source doesn't exist");
    }

    /// Test that source must be a directory
    #[test]
    fn test_create_zip_archive_validates_source_is_directory() {
        // Create a temporary file
        let temp_file_path = std::env::temp_dir().join("test_file_not_dir.txt");
        let _ = fs::write(&temp_file_path, "test content");

        let dest_path = std::env::temp_dir();

        let result = create_directory_zip_archive(&temp_file_path, &dest_path, None);

        assert!(
            result.is_err(),
            "Should error when source is not a directory"
        );

        // Clean up
        let _ = fs::remove_file(temp_file_path);
    }
}

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
/// match create_zip_of_dir_with_system_command(&source, &zip_file) {
///     Ok(true) => println!("Zip created successfully"),
///     Ok(false) => println!("Zip command failed"),
///     Err(e) => eprintln!("Error executing zip command: {}", e),
/// }
/// ```
fn create_zip_of_dir_with_system_command(
    source_path: &PathBuf,
    zip_path: &PathBuf,
) -> Result<bool> {
    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "android",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "redox",
    ))]
    {
        // Use zip command on Unix-like systems (including Android/Termux and BSD)
        let output = std::process::Command::new("zip")
            .arg("-r") // Recursive
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
                // On BSD systems, provide specific installation hints
                #[cfg(target_os = "freebsd")]
                eprintln!("On FreeBSD, install zip with: pkg install zip");
                #[cfg(target_os = "openbsd")]
                eprintln!("On OpenBSD, install zip with: pkg_add zip");
                #[cfg(target_os = "netbsd")]
                eprintln!("On NetBSD, install zip with: pkgin install zip");
                #[cfg(target_os = "dragonfly")]
                eprintln!("On DragonFly BSD, install zip with: pkg install zip");
                // After the DragonFly BSD hint, add:
                #[cfg(target_os = "redox")]
                eprintln!("On Redox OS, install zip through the package manager");

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

    // Fixed: Added all BSD variants to the exclusion list
    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "windows",
        target_os = "android",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "redox",
    )))]
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
    /// / Save current state as a pocket dimension with custom display size
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
        let path_name = path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "root".to_string());

        // Get timestamp seconds since epoch for uniqueness
        let timestamp_secs = timestamp
            .duration_since(UNIX_EPOCH)
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
            DirectorySortingMethodEnum::Name(asc) => {
                if *asc {
                    " name↑"
                } else {
                    " name↓"
                }
            }
            DirectorySortingMethodEnum::Size(asc) => {
                if *asc {
                    " size↑"
                } else {
                    " size↓"
                }
            }
            DirectorySortingMethodEnum::Modified(asc) => {
                if *asc {
                    " date↑"
                } else {
                    " date↓"
                }
            }
        };

        // Combine all elements into description
        format!(
            "{}{}{}",
            path.file_name().unwrap_or_default().to_string_lossy(),
            filter_desc,
            sort_desc
        )
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
    /// / Ready to use for collecting paths and saving states
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
    /// / Save current state with custom nickname
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
            println!(
                "Warning: Pocket dimension '{}' already exists and will be overwritten.",
                final_nickname
            );
            print!("Continue? (y/N): ");
            io::stdout()
                .flush()
                .map_err(|e| FileFantasticError::Io(e))?;

            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .map_err(|e| FileFantasticError::Io(e))?;

            // Only proceed if user explicitly confirms with 'y'
            if !response.trim().eq_ignore_ascii_case("y") {
                return Err(FileFantasticError::InvalidName(
                    "Operation cancelled by user".to_string(),
                ));
            }
        }

        // Store the pocket dimension
        self.pocket_dimensions
            .insert(final_nickname.clone(), saved_state);
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
                "Path is not a file".to_string(),
            ))
        }
    }

    /*pending use of saved directories*/
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
                "Path is not a directory".to_string(),
            ))
        }
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
    /// / Clear everything after user confirmation
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
        )
        .map_err(|e| FileFantasticError::Io(e))?;

        // WORKFLOW STEP 1 (continued): Prompt for selection
        println!("\n=== Archive Selection ===");
        println!("Select item to archive (creates timestamped copy/zip)");
        print!("Enter item number (or 'b' to back/cancel): ");
        io::stdout()
            .flush()
            .map_err(|e| FileFantasticError::Io(e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| FileFantasticError::Io(e))?;
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
                let item_name = item_info
                    .item_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();

                // WORKFLOW STEP 2: Ask for confirmation based on item type
                let confirmation_message = if item_info.item_type == FileSystemItemType::Directory {
                    format!(
                        "Archive directory '{}' as zip with timestamp? (Y/n): ",
                        item_name
                    )
                } else {
                    format!("Archive file '{}' with timestamp? (Y/n): ", item_name)
                };

                print!("\n{}", confirmation_message);
                io::stdout()
                    .flush()
                    .map_err(|e| FileFantasticError::Io(e))?;

                let mut response = String::new();
                io::stdin()
                    .read_line(&mut response)
                    .map_err(|e| FileFantasticError::Io(e))?;

                // Default to 'yes' if user just presses enter
                if response.trim().is_empty() || response.trim().eq_ignore_ascii_case("y") {
                    // WORKFLOW STEP 3: Process archive options based on item type
                    if item_info.item_type == FileSystemItemType::Directory {
                        // DIRECTORY WORKFLOW: Ask for optional name prefix, then create zip

                        print!(
                            "\nAdd custom name prefix to archive? (optional, or Enter to skip): "
                        );
                        io::stdout()
                            .flush()
                            .map_err(|e| FileFantasticError::Io(e))?;

                        let mut custom_name = String::new();
                        io::stdin()
                            .read_line(&mut custom_name)
                            .map_err(|e| FileFantasticError::Io(e))?;
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
                                    }
                                    Err(e) => {
                                        eprintln!("\n✗ Directory archive creation failed: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("\n✗ Failed to create archive directory: {}", e);
                            }
                        }
                    } else {
                        // FILE WORKFLOW: Ask about zip, then name prefix, then create archive

                        // Ask if user wants to zip the file
                        print!("\nWould you like to zip the archived file? (y/N): ");
                        io::stdout()
                            .flush()
                            .map_err(|e| FileFantasticError::Io(e))?;

                        let mut zip_response = String::new();
                        io::stdin()
                            .read_line(&mut zip_response)
                            .map_err(|e| FileFantasticError::Io(e))?;
                        let should_zip = zip_response.trim().eq_ignore_ascii_case("y");

                        // Ask for optional custom name prefix
                        print!(
                            "\nAdd custom name prefix to archive? (optional, or Enter to skip): "
                        );
                        io::stdout()
                            .flush()
                            .map_err(|e| FileFantasticError::Io(e))?;

                        let mut custom_name = String::new();
                        io::stdin()
                            .read_line(&mut custom_name)
                            .map_err(|e| FileFantasticError::Io(e))?;
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
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "\n✗ File zip archive creation failed: {}",
                                                e
                                            );
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
                                            println!(
                                                "Archive location: {}",
                                                archived_path.display()
                                            );
                                        }
                                        Err(e) => {
                                            eprintln!("\n✗ File archive creation failed: {}", e);
                                        }
                                    }
                                }
                            }
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
            return Err(FileFantasticError::InvalidName(format!(
                "Path is not a file: {}",
                file_path.display()
            )));
        }

        // Get the file name without extension for the archive name
        let file_stem = file_path
            .file_stem()
            .ok_or_else(|| {
                FileFantasticError::InvalidName("Could not extract file name".to_string())
            })?
            .to_string_lossy();

        // Generate full timestamp
        let timestamp = createarchive_timestamp_with_precision(SystemTime::now(), true);

        // Build the archive name with optional prefix
        let archive_name = if let Some(prefix) = custom_prefix {
            format!("{}{}_{}.zip", prefix, file_stem, timestamp)
        } else {
            format!("{}_{}.zip", file_stem, timestamp)
        };

        // Create full path for the archive
        let archive_path = archive_directory.join(archive_name);

        // Create simple archive (since we can't use external zip libraries)
        create_single_file_zip_archive_via_command(file_path, &archive_path)?;

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
        let file_stem = file_path
            .file_stem()
            .ok_or_else(|| {
                FileFantasticError::InvalidName("Could not extract file name".to_string())
            })?
            .to_string_lossy();

        let extension = file_path
            .extension()
            .map(|ext| format!(".{}", ext.to_string_lossy()))
            .unwrap_or_else(|| String::new());

        // Generate full timestamp
        let timestamp = createarchive_timestamp_with_precision(SystemTime::now(), true);

        // Build the archive name with optional prefix
        let archive_name = if let Some(prefix) = custom_prefix {
            format!("{}{}_{}{}", prefix, file_stem, timestamp, extension)
        } else {
            format!("{}_{}{}", file_stem, timestamp, extension)
        };

        // Create full path for the archive
        let archive_path = archive_directory.join(archive_name);

        // Copy the file
        fs::copy(file_path, &archive_path).map_err(|e| FileFantasticError::Io(e))?;

        Ok(archive_path)
    }

    /// Interactive interface to add any item (file or directory) to the appropriate stack
    ///
    /// # Purpose
    /// Provides a universal interactive interface for adding items to their respective stacks.
    /// Automatically determines whether the selected item is a file or directory and routes
    /// it to the appropriate stack (file_path_stack or directory_path_stack).
    /// This wrapper eliminates the need for users to pre-determine the item type.
    ///
    /// # Arguments
    /// * `nav_state` - Current navigation state with lookup table for numbered selection
    /// * `current_directory_entries` - Current directory entries to display for selection (current page only)
    /// * `current_directory_path` - Current directory path for display context
    ///
    /// # Returns
    /// * `Result<()>` - Success or error with context
    ///
    /// # Stack Routing Logic
    /// - Files → Added to `file_path_stack`
    /// - Directories → Added to `directory_path_stack`
    /// - Each type maintains its own separate stack
    ///
    /// # User Interface Workflow
    ///
    /// ## Step 1: Display Current View
    /// - Show current directory contents with numbered items
    /// - Respects current filter and pagination state
    ///
    /// ## Step 2: Selection Prompt
    /// - User selects any item by number (file or directory)
    /// - Can cancel with 'b' for back
    ///
    /// ## Step 3: Type Detection
    /// - System automatically determines if item is file or directory
    /// - No user input needed for type specification
    ///
    /// ## Step 4: Add to Appropriate Stack
    /// - Routes to correct stack based on detected type
    /// - Displays type-specific confirmation with stack count
    ///
    /// # Example Interaction
    /// ```text
    /// Current Directory: /home/user/documents
    ///
    /// Num  Name                    Size     Modified
    /// ------------------------------------------------
    ///  1)  folder1/               -        14:30
    ///  2)  document.txt           1.2 KB   15:45
    ///  3)  images/                -        16:00
    ///  4)  report.pdf             500 KB   16:20
    ///
    /// === Add Item to Stack ===
    /// Select item to add to stack (file or directory)
    /// Enter item number (or 'b' to back/cancel): 1
    ///
    /// ✓ Added directory 'folder1/' to directory stack. Total directories: 1
    ///
    /// [Another example]
    /// Enter item number (or 'b' to back/cancel): 2
    ///
    /// ✓ Added file 'document.txt' to file stack. Total files: 3
    /// ```
    ///
    /// # Error Handling
    /// - Validates numbered selections against navigation lookup table
    /// - Provides clear error messages for invalid selections
    /// - Handles cancellation gracefully
    /// - Shows appropriate stack type in confirmation messages
    /// - Manages IO errors during user interaction
    pub fn interactive_add_item_to_stack(
        &mut self,
        nav_state: &NavigationState,
        current_directory_entries: &[FileSystemEntry],
        current_directory_path: &PathBuf,
    ) -> Result<()> {
        // STEP 1: Display current directory contents
        // Reuse existing display function to show numbered items
        display_directory_contents(
            current_directory_entries,
            current_directory_path,
            None, // Pagination info handled by main navigation
            nav_state.current_filter,
            nav_state,
        )
        .map_err(|e| FileFantasticError::Io(e))?;

        // STEP 2: Prompt for item selection (generic - not file-specific)
        println!("\n=== Add Item to Stack ===");
        println!("Select item to add to stack (file or directory)");
        print!("Enter item number (or 'b' to back/cancel): ");
        io::stdout()
            .flush()
            .map_err(|e| FileFantasticError::Io(e))?;

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| FileFantasticError::Io(e))?;
        let input = input.trim();

        // Handle cancellation request
        if input.eq_ignore_ascii_case("b") {
            println!("Back/Cancelled.");
            return Ok(());
        }

        // STEP 3: Process the user's selection and determine type
        if let Ok(number) = input.parse::<usize>() {
            // Use navigation state's lookup to find the selected item
            if let Some(item_info) = nav_state.lookup_item(number) {
                // Extract item name for display purposes
                let item_name = item_info
                    .item_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();

                // STEP 4: Route to appropriate stack based on item type
                match item_info.item_type {
                    FileSystemItemType::Directory => {
                        // It's a directory - add to directory stack
                        match self.add_directory_to_stack(item_info.item_path.clone()) {
                            Ok(()) => {
                                // Success - show directory-specific confirmation
                                println!(
                                    "\n✓ Added directory '{}/' to directory stack. Total directories: {}",
                                    item_name,
                                    self.directory_path_stack.len()
                                );
                            }
                            Err(e) => {
                                // Failed to add directory to stack
                                eprintln!("\n✗ Failed to add directory to stack: {}", e);
                            }
                        }
                    }
                    FileSystemItemType::File => {
                        // It's a file - add to file stack
                        match self.add_file_to_stack(item_info.item_path.clone()) {
                            Ok(()) => {
                                // Success - show file-specific confirmation
                                println!(
                                    "\n✓ Added file '{}' to file stack. Total files: {}",
                                    item_name,
                                    self.file_path_stack.len()
                                );
                            }
                            Err(e) => {
                                // Failed to add file to stack
                                eprintln!("\n✗ Failed to add file to stack: {}", e);
                            }
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

    /// Interactive interface to get any item (file or directory) from the appropriate stack
    ///
    /// # Purpose
    /// Provides a universal interactive interface for retrieving items from their respective stacks.
    /// Returns both the item type and path, allowing the caller to handle each appropriately.
    /// For directories, this includes safety confirmations before returning the path for copying.
    ///
    /// # Returns
    /// * `Result<Option<StackRetrievalResult>>` - Item type and path if selected, None if canceled, or error
    ///
    /// # Return Value Structure
    /// The return value includes:
    /// - `item_type`: Indicates whether it's a File or Directory
    /// - `path`: The path to the selected item
    ///
    /// This allows the caller to:
    /// - Use appropriate copy mechanisms (simple file copy vs recursive directory copy)
    /// - Display type-specific messages
    /// - Apply different validation or processing logic
    ///
    /// # Stack Selection Logic
    /// - User first chooses between file stack or directory stack
    /// - Then selects specific item from chosen stack
    /// - Returns type information along with path
    ///
    /// # Safety Features
    /// - Directory operations require double confirmation
    /// - Clear messaging about operation types
    /// - Type information prevents accidental mishandling
    ///
    /// # Example Usage
    /// ```rust
    /// match state_manager.interactive_get_item_from_stack() {
    ///     Ok(Some(result)) => {
    ///         match result.item_type {
    ///             StackItemType::File => {
    ///                 // Handle file copy
    ///                 copy_file_with_archive_handling(&result.path, &destination)?;
    ///             }
    ///             StackItemType::Directory => {
    ///                 // Handle directory copy (recursive)
    ///                 copy_directory_recursively(&result.path, &destination)?;
    ///             }
    ///         }
    ///     }
    ///     Ok(None) => println!("No item selected"),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn interactive_get_item_from_stack(&mut self) -> Result<Option<StackRetrievalResult>> {
        // Check if both stacks are empty
        if self.file_path_stack.is_empty() && self.directory_path_stack.is_empty() {
            println!("Both file and directory stacks are empty.");
            return Ok(None);
        }

        println!("\n=== Get Item from Stack ===");
        println!("Select stack type:");

        // Show available options based on stack contents
        let mut has_files = false;
        let mut has_directories = false;

        if !self.file_path_stack.is_empty() {
            println!("1) File stack ({} items)", self.file_path_stack.len());
            has_files = true;
        }

        if !self.directory_path_stack.is_empty() {
            println!(
                "2) Directory stack ({} items)",
                self.directory_path_stack.len()
            );
            has_directories = true;
        }

        // Prompt for stack type selection
        print!("Enter choice (");
        if has_files && has_directories {
            print!("1/2");
        } else if has_files {
            print!("1");
        } else {
            print!("2");
        }
        print!(", or 'c' to cancel): ");
        io::stdout()
            .flush()
            .map_err(|e| FileFantasticError::Io(e))?;

        // Read user's stack type choice
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| FileFantasticError::Io(e))?;
        let input = input.trim();

        // Handle cancellation
        if input.eq_ignore_ascii_case("c") {
            println!("Cancelled.");
            return Ok(None);
        }

        // Route to appropriate stack based on user choice
        match input {
            "1" if has_files => {
                // User chose file stack
                match self.get_file_from_stack_helper() {
                    Ok(Some(path)) => Ok(Some(StackRetrievalResult {
                        item_type: StackItemType::File,
                        path,
                    })),
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            }
            "2" if has_directories => {
                // User chose directory stack
                match self.get_directory_from_stack_helper() {
                    Ok(Some(path)) => Ok(Some(StackRetrievalResult {
                        item_type: StackItemType::Directory,
                        path,
                    })),
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            }
            _ => {
                println!("Error: Invalid choice. Please enter a valid option.");
                Ok(None)
            }
        }
    }

    /// Helper function to select and retrieve a file from the file stack
    ///
    /// # Purpose
    /// Internal helper that handles the file-specific retrieval logic.
    /// This is essentially the existing `interactive_get_file_from_stack` function
    /// but renamed to clarify its role as a helper in the new architecture.
    ///
    /// # Returns
    /// * `Result<Option<PathBuf>>` - Selected file path, None if canceled, or error
    ///
    /// # Behavior
    /// - Displays all files in the stack (most recent first)
    /// - Allows selection by number or default to most recent
    /// - Returns the file path without removing it from stack
    /// - No destructive operations performed
    ///
    /// # Note
    /// This function maintains the exact same behavior as the original
    /// `interactive_get_file_from_stack` for consistency.
    fn get_file_from_stack_helper(&mut self) -> Result<Option<PathBuf>> {
        // Check if stack is empty (redundant check for safety)
        if self.file_path_stack.is_empty() {
            println!("File stack is empty.");
            return Ok(None);
        }

        println!("\n=== File Stack ===");

        // Display files in reverse order (most recent first) for user-friendly numbering
        for (i, file) in self.file_path_stack.iter().enumerate().rev() {
            println!(
                "{}. {}",
                self.file_path_stack.len() - i,
                file.file_name().unwrap_or_default().to_string_lossy()
            );
        }

        print!("Select file number (Enter for most recent, 'c' to cancel): ");
        io::stdout()
            .flush()
            .map_err(|e| FileFantasticError::Io(e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| FileFantasticError::Io(e))?;
        let input = input.trim();

        // Handle cancellation
        if input.eq_ignore_ascii_case("c") {
            println!("Cancelled.");
            return Ok(None);
        }

        // Default to most recent if no input
        if input.is_empty() {
            if let Some(file) = self.file_path_stack.last() {
                println!(
                    "Retrieved: {}",
                    file.file_name().unwrap_or_default().to_string_lossy()
                );
                return Ok(Some(file.clone()));
            }
        }

        // Try to parse as index and validate
        if let Ok(index) = input.parse::<usize>() {
            if index > 0 && index <= self.file_path_stack.len() {
                // Convert to actual vector index (1-based display to 0-based storage)
                let actual_index = self.file_path_stack.len() - index;

                // Use .get() for bounds-checked access
                if let Some(file) = self.file_path_stack.get(actual_index) {
                    println!(
                        "Retrieved: {}",
                        file.file_name().unwrap_or_default().to_string_lossy()
                    );
                    return Ok(Some(file.clone()));
                } else {
                    println!("Error: Index out of bounds");
                }
            } else {
                println!(
                    "Error: Invalid file number {}. Valid range: 1-{}",
                    index,
                    self.file_path_stack.len()
                );
            }
        } else {
            println!(
                "Error: Please enter a valid number, press Enter for most recent, or 'c' to cancel."
            );
        }

        Ok(None)
    }

    /// Helper function to select a directory from stack and initiate recursive copy
    ///
    /// # Purpose
    /// Internal helper that handles directory-specific retrieval and copy operations.
    /// Implements double confirmation for safety when performing recursive directory copies.
    /// This function does NOT delete the original - it creates a copy.
    ///
    /// # Returns
    /// * `Result<Option<PathBuf>>` - Selected directory path if confirmed, None if canceled
    ///
    /// # Safety Features
    /// - Double confirmation required before any operation
    /// - Clear warnings about recursive nature of operation
    /// - Explicit messaging that this is a COPY, not a move
    /// - Multiple cancellation points
    ///
    /// # User Flow
    /// 1. Display available directories in stack
    /// 2. User selects directory by number
    /// 3. First confirmation: "Are you sure you want to recursively copy?"
    /// 4. Second confirmation: "Are you REALLY sure?"
    /// 5. Return directory path for copy operation if both confirmed
    ///
    /// # Important Notes
    /// - This function returns the path but does NOT perform the actual copy
    /// - The calling code is responsible for executing the copy operation
    /// - Original directory remains unchanged in its location
    ///
    /// # Example Interaction
    /// ```text
    /// === Directory Stack ===
    /// 1. /home/user/projects/webapp/
    /// 2. /home/user/documents/reports/
    /// Select directory number (Enter for most recent, 'c' to cancel): 1
    ///
    /// Selected directory: /home/user/projects/webapp/
    /// |o|  This will RECURSIVELY COPY the entire directory and all its contents.
    /// >*<  The original directory will NOT be deleted or moved.
    ///
    /// Are you sure you want to recursively copy this directory? (y/N): y
    ///
    /// |o|  SECOND CONFIRMATION REQUIRED
    /// Are you REALLY sure you want to copy '/home/user/projects/webapp/'? (y/N): y
    ///
    /// ✓ Directory copy confirmed: /home/user/projects/webapp/
    /// ```
    fn get_directory_from_stack_helper(&mut self) -> Result<Option<PathBuf>> {
        // Check if stack is empty (redundant check for safety)
        if self.directory_path_stack.is_empty() {
            println!("Directory stack is empty.");
            return Ok(None);
        }

        println!("\n=== Directory Stack ===");

        // Display directories in reverse order (most recent first)
        for (i, dir) in self.directory_path_stack.iter().enumerate().rev() {
            println!(
                "{}. {}/",
                self.directory_path_stack.len() - i,
                dir.display()
            );
        }

        print!("Select directory number (Enter for most recent, 'c' to cancel): ");
        io::stdout()
            .flush()
            .map_err(|e| FileFantasticError::Io(e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| FileFantasticError::Io(e))?;
        let input = input.trim();

        // Handle cancellation
        if input.eq_ignore_ascii_case("c") {
            println!("Cancelled.");
            return Ok(None);
        }

        // Determine which directory was selected
        let selected_directory: Option<PathBuf> = if input.is_empty() {
            // Default to most recent (last in stack)
            self.directory_path_stack.last().cloned()
        } else if let Ok(index) = input.parse::<usize>() {
            // Validate index and get directory
            if index > 0 && index <= self.directory_path_stack.len() {
                // Convert to actual vector index
                let actual_index = self.directory_path_stack.len() - index;
                self.directory_path_stack.get(actual_index).cloned()
            } else {
                println!(
                    "Error: Invalid directory number {}. Valid range: 1-{}",
                    index,
                    self.directory_path_stack.len()
                );
                return Ok(None);
            }
        } else {
            println!(
                "Error: Please enter a valid number, press Enter for most recent, or 'c' to cancel."
            );
            return Ok(None);
        };

        // Process selected directory with safety confirmations
        if let Some(dir_path) = selected_directory {
            println!("\nSelected directory: {}/", dir_path.display());
            println!("|o|  This will RECURSIVELY COPY the entire directory and all its contents.");
            println!(">*<  The original directory will NOT be deleted or moved.");

            // FIRST CONFIRMATION
            print!("\nAre you sure you want to recursively copy this directory? (y/N): ");
            io::stdout()
                .flush()
                .map_err(|e| FileFantasticError::Io(e))?;

            let mut first_confirmation = String::new();
            io::stdin()
                .read_line(&mut first_confirmation)
                .map_err(|e| FileFantasticError::Io(e))?;

            // Check first confirmation (default to NO for safety)
            if !first_confirmation.trim().eq_ignore_ascii_case("y") {
                println!("Operation cancelled.");
                return Ok(None);
            }

            // SECOND CONFIRMATION - Extra safety for recursive operations
            println!("\n|oo|  SECOND CONFIRMATION REQUIRED");
            print!(
                "Are you SUPER REALLY sure you want to copy '{}'? (y/N): ",
                dir_path.display()
            );
            io::stdout()
                .flush()
                .map_err(|e| FileFantasticError::Io(e))?;

            let mut second_confirmation = String::new();
            io::stdin()
                .read_line(&mut second_confirmation)
                .map_err(|e| FileFantasticError::Io(e))?;

            // Check second confirmation (default to NO for safety)
            if !second_confirmation.trim().eq_ignore_ascii_case("y") {
                println!("Operation cancelled at second confirmation.");
                return Ok(None);
            }

            // Both confirmations passed - return the directory path
            println!("\n✓ Directory copy confirmed: {}/", dir_path.display());
            Ok(Some(dir_path))
        } else {
            // Should not reach here, but handle as error case
            println!("Error: Could not retrieve selected directory.");
            Ok(None)
        }
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
            println!("{}. {} - {}", i + 1, nickname, state.description);
        }

        print!("Select pocket dimension number ('c' to cancel): ");
        io::stdout()
            .flush()
            .map_err(|e| FileFantasticError::Io(e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| FileFantasticError::Io(e))?;
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
                println!(
                    "Error: Invalid pocket dimension number {}. Valid range: 1-{}",
                    index,
                    dimensions.len()
                );
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
        println!("1. Add item TO stack (file or dir)");
        println!("2. Get: Save item here, FROM stack");
        println!("3. Save current location as pocket dimension");
        println!("4. Go to pocket dimension");
        println!("5. View stacks & pocket dimensions");
        println!("6. Archive file/directory 'a': zip/timestamp");
        println!("7. Clear all stacks");
        // println!("--  --");
        println!();
        print!("Select Action (1-7)  or (b)ack / empty-Enter ");
        io::stdout()
            .flush()
            .map_err(|e| FileFantasticError::Io(e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| FileFantasticError::Io(e))?;

        // Process user selection and return appropriate action
        match input.trim() {
            "1" => Ok(GetSendModeAction::AddItemToStack),
            "2" => Ok(GetSendModeAction::GetItemFromStack),
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
/// - `AddItemToStack` - Initiate file addition workflow
/// - `GetItemFromStack` - Initiate file retrieval workflow
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
///     GetSendModeAction::AddItemToStack => {
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
    AddItemToStack,

    /// Retrieve a file from the file path stack
    /// Triggers the interactive file selection and retrieval workflow
    GetItemFromStack,

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
/// / Returns something like: "2025_01_15_14_30_45"
///
/// / Used in archive filename: "cats_2025_01_15_14_30_45.toml"
/// ```
fn generate_archive_timestamp() -> String {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0));
    let seconds = duration_since_epoch.as_secs();

    let (year, month, day, hour, minute) = seconds_to_components(seconds);
    let second = (seconds % 60) as u32;

    format!(
        "{:04}_{:02}_{:02}_{:02}_{:02}_{:02}",
        year, month, day, hour, minute, second
    )
}

/// Creates archive directory if it doesn't exist, avoiding nested archive directories
///
/// # Purpose
/// Ensures that an "archive" subdirectory exists in the specified parent directory,
/// creating it if necessary. This directory is used to store copies of files
/// when avoiding overwrites. If the parent directory is already named "archive",
/// returns the parent directory itself to avoid creating nested archive directories.
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
/// Normal case:
/// parent_directory/
/// ├── existing_files...
/// └── archive/          <- Created by this function
///     ├── file1_timestamp.ext
///     └── file2_timestamp.ext
///
/// Already in archive case:
/// some_path/archive/    <- If parent is already "archive"
/// ├── file1_timestamp.ext
/// └── file2_timestamp.ext  <- Files go here directly, no nested archive/
/// ```
///
/// # Example
/// ```rust
/// / Normal case - creates archive subdirectory
/// let current_dir = PathBuf::from("/home/user/documents");
/// match ensure_archive_directory_exists(&current_dir) {
///     Ok(archive_path) => {
///         // archive_path is "/home/user/documents/archive"
///         println!("Archive directory ready: {}", archive_path.display());
///     },
///     Err(e) => eprintln!("Failed to create archive directory: {}", e),
/// }
///
/// / Already in archive - returns same directory
/// let archive_dir = PathBuf::from("/home/user/documents/archive");
/// match ensure_archive_directory_exists(&archive_dir) {
///     Ok(archive_path) => {
///         // archive_path is "/home/user/documents/archive" (same as input)
///         println!("Already in archive: {}", archive_path.display());
///     },
///     Err(e) => eprintln!("Failed to process archive directory: {}", e),
/// }
/// ```
fn ensure_archive_directory_exists(parent_directory: &PathBuf) -> Result<PathBuf> {
    // Step 1: Check if the parent directory itself is already named "archive"
    // This prevents creating nested archive directories like archive/archive/
    if let Some(dir_name) = parent_directory.file_name() {
        if dir_name == "archive" {
            // We're already in an archive directory, return the current directory
            println!(
                "Already in archive directory: {}",
                parent_directory.display()
            );

            // Verify it exists and is a directory before returning
            if !parent_directory.exists() {
                return Err(FileFantasticError::InvalidName(format!(
                    "Archive directory doesn't exist: {}",
                    parent_directory.display()
                )));
            }

            if !parent_directory.is_dir() {
                return Err(FileFantasticError::InvalidName(format!(
                    "Archive path exists but is not a directory: {}",
                    parent_directory.display()
                )));
            }

            return Ok(parent_directory.clone());
        }
    }

    // Step 2: Not already in archive, proceed with normal archive directory creation
    let archive_directory_path = parent_directory.join("archive");

    // Check if archive directory already exists
    if !archive_directory_path.exists() {
        // Create the archive directory
        fs::create_dir(&archive_directory_path).map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => {
                FileFantasticError::PermissionDenied(archive_directory_path.clone())
            }
            _ => FileFantasticError::Io(e),
        })?;

        println!(
            "Created archive directory: {}",
            archive_directory_path.display()
        );
    }

    // Verify it's actually a directory
    if !archive_directory_path.is_dir() {
        return Err(FileFantasticError::InvalidName(format!(
            "Archive path exists but is not a directory: {}",
            archive_directory_path.display()
        )));
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
        return Err(FileFantasticError::InvalidName(format!(
            "Source is not a file: {}",
            source_file_path.display()
        )));
    }

    // Validate destination directory exists and is a directory
    if !destination_directory.exists() {
        return Err(FileFantasticError::NotFound(destination_directory.clone()));
    }

    if !destination_directory.is_dir() {
        return Err(FileFantasticError::InvalidName(format!(
            "Destination is not a directory: {}",
            destination_directory.display()
        )));
    }

    // Extract source filename
    let source_filename = source_file_path
        .file_name()
        .ok_or_else(|| {
            FileFantasticError::InvalidName(format!(
                "Cannot determine filename from: {}",
                source_file_path.display()
            ))
        })?
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
            return Err(FileFantasticError::InvalidName(format!(
                "Temporary backup file already exists: {}",
                temp_backup_path.display()
            )));
        }

        // Copy existing file to temporary backup - handle race condition where file might have been deleted
        match fs::copy(&primary_destination_path, &temp_backup_path) {
            Ok(_) => {
                println!("Created temporary backup: {}", temp_backup_path.display());
            }
            Err(e) => {
                // Check if the error is because the file no longer exists (race condition)
                if e.kind() == io::ErrorKind::NotFound {
                    // File was deleted between our check and now - just do a normal copy
                    println!("Original file no longer exists, proceeding with normal copy...");

                    fs::copy(source_file_path, &primary_destination_path).map_err(|copy_err| {
                        match copy_err.kind() {
                            io::ErrorKind::PermissionDenied => {
                                FileFantasticError::PermissionDenied(
                                    primary_destination_path.clone(),
                                )
                            }
                            _ => FileFantasticError::Io(copy_err),
                        }
                    })?;

                    println!("File copied to: {}", primary_destination_path.display());
                    return Ok(primary_destination_path);
                }

                // Other error - fail
                return Err(match e.kind() {
                    io::ErrorKind::PermissionDenied => {
                        FileFantasticError::PermissionDenied(primary_destination_path.clone())
                    }
                    _ => FileFantasticError::Io(e),
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
            return Err(FileFantasticError::InvalidName(format!(
                "Temporary new file already exists: {}",
                temp_new_path.display()
            )));
        }

        // Copy source file to temporary location
        match fs::copy(source_file_path, &temp_new_path) {
            Ok(_) => {
                println!(
                    "Copied new file to temporary location: {}",
                    temp_new_path.display()
                );
            }
            Err(e) => {
                // Rollback: remove temp backup since we're failing
                let _ = fs::remove_file(&temp_backup_path);

                return Err(match e.kind() {
                    io::ErrorKind::PermissionDenied => {
                        FileFantasticError::PermissionDenied(temp_new_path)
                    }
                    _ => FileFantasticError::Io(e),
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
            return Err(FileFantasticError::InvalidName(format!(
                "Archive file already exists: {}",
                archive_destination_path.display()
            )));
        }

        // Try to rename first (atomic on same filesystem), fall back to copy+delete
        // Explicitly specify the type for the Result
        let archive_result: std::result::Result<(), io::Error> = fs::rename(
            &primary_destination_path,
            &archive_destination_path,
        )
        .or_else(|rename_err| {
            // Rename failed, probably cross-filesystem - try copy then delete
            eprintln!(
                "Rename to archive failed ({}), trying copy+delete...",
                rename_err
            );
            fs::copy(&primary_destination_path, &archive_destination_path).map_err(|copy_err| {
                eprintln!("Failed to copy to archive: {}", copy_err);
                copy_err
            })?;
            fs::remove_file(&primary_destination_path).map_err(|rm_err| {
                eprintln!(
                    "Failed to remove original after archive copy: {} {}",
                    rm_err, rename_err
                );
                rm_err
            })?;
            Ok(())
        });

        match archive_result {
            Ok(_) => {
                println!(
                    "Archived existing file to: {}",
                    archive_destination_path.display()
                );
            }
            Err(e) => {
                // Rollback: remove temp new file, keep temp backup for manual recovery
                let _ = fs::remove_file(&temp_new_path);

                eprintln!(
                    "Failed to archive existing file. Backup preserved at: {}",
                    temp_backup_path.display()
                );

                return Err(match e.kind() {
                    io::ErrorKind::PermissionDenied => {
                        FileFantasticError::PermissionDenied(archive_destination_path)
                    }
                    _ => FileFantasticError::Io(e),
                });
            }
        }

        // Step 5: Move temp new file to final destination
        // Try rename first (atomic), fall back to copy+delete
        // Explicitly specify the type for the Result
        let rename_result: std::result::Result<(), io::Error> =
            fs::rename(&temp_new_path, &primary_destination_path).or_else(|rename_err| {
                // Rename failed - try copy then delete
                eprintln!(
                    "Rename of new file failed ({}), trying copy+delete...",
                    rename_err
                );
                fs::copy(&temp_new_path, &primary_destination_path).map_err(|copy_err| {
                    eprintln!("Failed to copy new file to destination: {}", copy_err);
                    copy_err
                })?;
                fs::remove_file(&temp_new_path).map_err(|rm_err| {
                    eprintln!(
                        "Failed to remove temp new file after copy: {} {}",
                        rm_err, rename_err
                    );
                    rm_err
                })?;
                Ok(())
            });

        match rename_result {
            Ok(_) => {
                println!(
                    "New file installed at: {}",
                    primary_destination_path.display()
                );

                // Step 6: Clean up temp backup (success case)
                match fs::remove_file(&temp_backup_path) {
                    Ok(_) => {
                        println!("Cleaned up temporary backup.");
                    }
                    Err(e) => {
                        // Non-critical error, just warn
                        eprintln!(
                            "Warning: Could not remove temporary backup at {}: {}",
                            temp_backup_path.display(),
                            e
                        );
                    }
                }

                primary_destination_path
            }
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
                    }
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
                            }
                            Err(archive_restore_err) => {
                                eprintln!("Could not restore original file: {}", restore_err);
                                eprintln!("Archive restore also failed: {}", archive_restore_err);
                                eprintln!(
                                    "Original file is in archive: {}",
                                    archive_destination_path.display()
                                );
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
                    }
                    _ => FileFantasticError::Io(e),
                });
            }
        }
    } else {
        // No conflict, copy directly to destination
        fs::copy(source_file_path, &primary_destination_path).map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => {
                FileFantasticError::PermissionDenied(primary_destination_path.clone())
            }
            _ => FileFantasticError::Io(e),
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
/// / Create a paginated view of directory entries
/// let mut dir_view = DirectoryView::new(&directory_entries);
///
/// / Display current page
/// let current_page_entries = dir_view.current_page_entries();
/// display_directory_contents(current_page_entries, &current_path)?;
///
/// / Navigate to next page if user presses 's'
/// if user_input == "s" {
///     dir_view.next_page();
/// }
///
/// / Navigate to previous page if user presses 'w'
/// if user_input == "w" {
///     dir_view.prev_page();
/// }
///
/// / Convert a display index to actual index in the full list
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
        // Do not try to divide by zero
        if self.items_per_page == 0 {
            return 0;
        }
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
    /// / Set to page 3 (0-based = page 4 in display)
    /// if dir_view.set_current_page(3) {
    ///     println!("Set to page 4 successfully");
    /// } else {
    ///     println!("Page 4 doesn't exist, clamped to valid page");
    /// }
    /// ```
    fn set_current_page(&mut self, page_index: usize) -> bool {
        // Do not try to paginate if items_per_page is zero
        if self.items_per_page == 0 {
            // Cannot paginate if items_per_page is zero
            return false;
        }

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
/// / Example search results for query "doc"
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
/// / Sort results by distance (best matches first)
/// results.sort_by_key(|r| r.distance);
///
/// / Display to user for selection
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
/// / A grep result for finding "TODO" in a source file
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

/// Formats a timestamp into a human-readable format with proper error handling
///
/// # Purpose
/// Internal version that returns Result for error propagation.
/// The public format_timestamp function wraps this for backward compatibility.
///
/// # Arguments
/// * `timestamp` - SystemTime to format
///
/// # Returns
/// * `Result<String>` - Formatted date/time string or error
fn format_timestamp_internal(timestamp: SystemTime) -> Result<String> {
    // Get current time
    let now = SystemTime::now();

    // Convert timestamps to Duration since UNIX_EPOCH
    let now_duration = now.duration_since(UNIX_EPOCH).map_err(|e| {
        FileFantasticError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to get current time: {}", e),
        ))
    })?;

    let file_duration = timestamp.duration_since(UNIX_EPOCH).map_err(|e| {
        FileFantasticError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to convert timestamp: {}", e),
        ))
    })?;

    // Convert to seconds
    let now_secs = now_duration.as_secs();
    let file_secs = file_duration.as_secs();

    // Get the calendar date components for both timestamps
    let (file_year, file_month, file_day) = seconds_to_ymd(file_secs);
    let (current_year, current_month, current_day) = seconds_to_ymd(now_secs);

    // Get time components for the file
    let (_, _, _, hour, minute) = seconds_to_components(file_secs);

    // Determine format based on actual calendar dates
    if file_year == current_year && file_month == current_month && file_day == current_day {
        // Same calendar day: show time only
        Ok(format!("{:02}:{:02}", hour, minute))
    } else if file_year == current_year {
        // Same calendar year but different day: show month-day and time
        Ok(format!(
            "{:02}-{:02} {:02}:{:02}",
            file_month, file_day, hour, minute
        ))
    } else {
        // Different year: show full date
        Ok(format!("{}-{:02}-{:02}", file_year, file_month, file_day))
    }
}

/// Public wrapper that maintains backward compatibility
///
/// # Purpose
/// Provides the original String return type for existing code.
/// Handles errors by returning a placeholder string.
fn format_timestamp(timestamp: SystemTime) -> String {
    match format_timestamp_internal(timestamp) {
        Ok(formatted) => formatted,
        Err(_) => String::from("????-??-??"),
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
    let (year, month, day, hour, minute, second) =
        epoch_seconds_to_datetime_components(total_seconds);

    // Format as YY_MM_DD_HH_MM_SS
    format!(
        "{:02}_{:02}_{:02}_{:02}_{:02}_{:02}",
        year % 100, // Two-digit year
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
    let is_leap_year = |y: u32| -> bool { (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) };

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
    include_microseconds: bool,
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
        let days_in_year = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
            366
        } else {
            365
        };
        if days_remaining < days_in_year as u64 {
            break;
        }
        days_remaining -= days_in_year as u64;
        year += 1;
    }

    // Simplified month calculation
    let days_in_month = [
        31,
        if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
            29
        } else {
            28
        },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

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
/// / Sort by name ascending
/// sort_directory_entries(&mut entries, DirectorySortingMethodEnum::Name(true));
///
/// / Sort by size descending
/// sort_directory_entries(&mut entries, DirectorySortingMethodEnum::Size(false));
///
/// / Sort by modification time ascending
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
        }
        DirectorySortingMethodEnum::Size(ascending) => {
            entries.sort_by(|a, b| match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => {
                    let cmp = a
                        .file_system_item_size_in_bytes
                        .cmp(&b.file_system_item_size_in_bytes);
                    if ascending { cmp } else { cmp.reverse() }
                }
            });
        }
        DirectorySortingMethodEnum::Modified(ascending) => {
            entries.sort_by(|a, b| match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => {
                    let cmp = a
                        .file_system_item_last_modified_time
                        .cmp(&b.file_system_item_last_modified_time);
                    if ascending { cmp } else { cmp.reverse() }
                }
            });
        }
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
/// / When user presses 't' to open terminal at current location
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
            ("ptyxis", vec!["--working-directory"]), // New Fedora 41+ default
            ("konsole", vec!["--workdir"]),
            ("xfce4-terminal", vec!["--working-directory"]),
            ("mate-terminal", vec!["--working-directory"]),
            ("terminator", vec!["--working-directory"]),
            ("alacritty", vec!["--working-directory"]),
            ("kitty", vec!["--directory"]),
            ("tilix", vec!["--working-directory"]),
            ("urxvt", vec!["-cd"]),
            ("rxvt", vec!["-cd"]),
            ("xterm", vec!["-e", "cd"]), // xterm needs special handling
        ];

        for (terminal, args) in terminal_commands.iter() {
            let mut command = std::process::Command::new(terminal);

            if *terminal == "xterm" || *terminal == "urxvt" || *terminal == "rxvt" {
                // These terminals need special handling with the shell
                command
                    .args(args)
                    .arg(directory_path.to_string_lossy().to_string())
                    .arg("&& bash");
            } else if *terminal == "alacritty" || *terminal == "kitty" {
                // Some newer terminals handle working directory differently
                command.arg(args[0]).arg(directory_path);
            } else {
                command.args(args).arg(directory_path);
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
    #[cfg(any(
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    {
        // Try different terminal emulators common on BSD systems
        // xterm is usually available on most BSD installations with X11
        let terminal_commands = [
            ("xterm", vec!["-e", "cd"]), // xterm is most common on BSD
            ("urxvt", vec!["-cd"]),
            ("rxvt", vec!["-cd"]),
            ("konsole", vec!["--workdir"]),
            ("gnome-terminal", vec!["--working-directory"]),
            ("xfce4-terminal", vec!["--working-directory"]),
            ("kitty", vec!["--directory"]),
            ("alacritty", vec!["--working-directory"]),
            ("sakura", vec!["--working-directory"]),
        ];

        for (terminal, args) in terminal_commands.iter() {
            let mut command = std::process::Command::new(terminal);

            if *terminal == "xterm" || *terminal == "urxvt" || *terminal == "rxvt" {
                // These terminals need special handling with the shell
                // BSD systems often use sh or ksh by default
                command
                    .args(args)
                    .arg(directory_path.to_string_lossy().to_string())
                    .arg("&& $SHELL");
            } else if *terminal == "alacritty" || *terminal == "kitty" {
                // Some newer terminals handle working directory differently
                command.arg(args[0]).arg(directory_path);
            } else {
                command.args(args).arg(directory_path);
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
            ("ptyxis", vec!["--working-directory"]), // New Fedora 41+ default
            ("konsole", vec!["--workdir"]),
            ("xfce4-terminal", vec!["--working-directory"]),
            ("mate-terminal", vec!["--working-directory"]),
            ("terminator", vec!["--working-directory"]),
            ("alacritty", vec!["--working-directory"]),
            ("kitty", vec!["--directory"]),
            ("tilix", vec!["--working-directory"]),
            ("urxvt", vec!["-cd"]),
            ("rxvt", vec!["-cd"]),
            ("xterm", vec!["-e", "cd"]), // xterm needs special handling
        ];

        for (terminal, args) in terminal_commands.iter() {
            let mut command = std::process::Command::new(terminal);

            if *terminal == "xterm" || *terminal == "urxvt" || *terminal == "rxvt" {
                // These terminals need special handling with the shell
                command
                    .args(args)
                    .arg(directory_path.to_string_lossy().to_string())
                    .arg("&& bash");
            } else if *terminal == "alacritty" || *terminal == "kitty" {
                // Some newer terminals handle working directory differently
                command.arg(args[0]).arg(directory_path);
            } else {
                command.args(args).arg(directory_path);
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
    #[cfg(target_os = "redox")]
    {
        // Redox OS uses Orbital windowing system with its own terminal
        let result = std::process::Command::new("orbital")
            .arg("terminal")
            .arg("--working-directory")
            .arg(directory_path)
            .spawn();

        match result {
            Ok(_) => return Ok(()),
            Err(e) => {
                eprintln!("Failed to open Orbital terminal on Redox OS: {}", e);

                // Fallback: try to open a basic terminal without working directory
                let fallback_result = std::process::Command::new("orbital")
                    .arg("terminal")
                    .spawn();

                match fallback_result {
                    Ok(_) => {
                        eprintln!("Note: Terminal opened but not in the requested directory");
                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("Failed to open any terminal on Redox OS: {}", e);
                        return Err(FileFantasticError::NoTerminalFound);
                    }
                }
            }
        }
    }
    // This is a fallback for platforms not explicitly handled
    #[cfg(not(any(
        target_os = "macos",
        target_os = "linux",
        target_os = "windows",
        target_os = "android",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "redox",
    )))]
    {
        Err(FileFantasticError::UnsupportedPlatform)
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
/// / Valid commands
/// assert!(parse_tui_adjustment_command("tall+5").is_some());
/// assert!(parse_tui_adjustment_command("wide-10").is_some());
/// assert!(parse_tui_adjustment_command("tall+65535").is_some());
///
/// / Invalid commands
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
    let (adjustment_direction_true_is_positive_false_is_negative, number_string) = match first_char
    {
        '+' => (true, &remaining_after_keyword[1..]),
        '-' => (false, &remaining_after_keyword[1..]),
        _ => return None, // Must have + or - after keyword
    };

    // Parse the magnitude, ensuring it's valid and non-zero
    match number_string.parse::<u16>() {
        Ok(magnitude) if magnitude > 0 => Some(TuiAdjustmentAction {
            adjustment_type_true_is_tall_false_is_wide,
            adjustment_magnitude: magnitude,
            adjustment_direction_true_is_positive_false_is_negative,
        }),
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
/// / User enters "tall+5"
/// let action = TuiAdjustmentAction {
///     adjustment_type_true_is_tall_false_is_wide: true,
///     adjustment_magnitude: 5,
///     adjustment_direction_true_is_positive_false_is_negative: true,
/// };
/// apply_tui_resize_adjustment(&mut nav_state, &action);
/// / Result: nav_state.tui_tall_adjustment = 5
/// /         nav_state.tui_tall_direction_sign = true
/// /         nav_state.current_page_index = 0
/// ```
fn apply_tui_resize_adjustment(
    nav_state: &mut NavigationState,
    adjustment_action: &TuiAdjustmentAction,
) {
    if adjustment_action.adjustment_type_true_is_tall_false_is_wide {
        // This is a height (tall) adjustment
        nav_state.tui_tall_adjustment = adjustment_action.adjustment_magnitude;
        nav_state.tui_tall_direction_sign =
            adjustment_action.adjustment_direction_true_is_positive_false_is_negative;
        // Reset to first page since the number of items per page has changed
        nav_state.current_page_index = 0;
    } else {
        // This is a width (wide) adjustment
        nav_state.tui_wide_adjustment = adjustment_action.adjustment_magnitude;
        nav_state.tui_wide_direction_sign =
            adjustment_action.adjustment_direction_true_is_positive_false_is_negative;
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
    wide_direction_sign: bool,
) -> (String, String) {
    let tall_display = if tall_adjustment == 0 {
        String::from("tall+N")
    } else {
        format!(
            "tall{}{}",
            if tall_direction_sign { "+" } else { "-" },
            tall_adjustment
        )
    };

    let wide_display = if wide_adjustment == 0 {
        String::from("wide-N")
    } else {
        format!(
            "wide{}{}",
            if wide_direction_sign { "+" } else { "-" },
            wide_adjustment
        )
    };

    (tall_display, wide_display)
}

static LINE_COUNT_LOOKUP: OnceLock<HashMap<&'static str, ()>> = OnceLock::new();

fn get_line_count_options() -> &'static HashMap<&'static str, ()> {
    LINE_COUNT_LOOKUP.get_or_init(|| {
        HashMap::from([
            ("--line-count", ()),
            ("--row-count", ()),
            ("--line-counts", ()),
            ("--row-counts", ()),
            ("--linecounts", ()),
            ("--rowcounts", ()),
            ("--lines-count", ()),
            ("--rows-count", ()),
            ("--count-rows", ()),
            ("--count-lines", ()),
            ("--linescounts", ()),
            ("--rowscounts", ()),
            ("--linecount", ()),
            ("--rowcount", ()),
            ("--linescount", ()),
            ("--rowscount", ()),
            ("--countrows", ()),
            ("--countlines", ()),
        ])
    })
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
/// / Process user input and take appropriate action
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

    // command flags that are more than one character long
    match lowercase_input.as_str() {
        "vsplit" | "-vsplit" => return Ok(NavigationAction::VsplitTmux),
        "hsplit" | "-hsplit" => return Ok(NavigationAction::HsplitTmux),
        "--help" => return Ok(NavigationAction::GoToHelpMenuMode),
        "--source" => return Ok(NavigationAction::GoToSouceCode),
        scurvy_curr if get_line_count_options().contains_key(scurvy_curr) => {
            return Ok(NavigationAction::GoToFileLineCountMode);
        }
        _ => {}
    }

    // if selection number + --return-path
    if lowercase_input.contains("--return-path") {
        // Split the input and try to extract the number before --return-path
        let parts: Vec<&str> = lowercase_input.split_whitespace().collect();

        // Find the index of "--return-path"
        if let Some(return_path_index) = parts.iter().position(|&x| x == "--return-path") {
            // Try to parse the number before --return-path
            if return_path_index > 0 {
                if let Ok(number) = parts[return_path_index - 1].parse::<usize>() {
                    // Use nav_state's lookup_item method to get the path
                    if let Some(item_info) = nav_state.lookup_item(number) {
                        // Get the path (works for both files and directories)
                        let selected_path = item_info.item_path.clone();

                        return Ok(NavigationAction::ReturnPathExitFF(selected_path));
                    }
                }
            }
        }
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
                FileSystemItemType::File => NavigationAction::OpenFile(item_info.item_path.clone()),
            });
        }
    }

    // Get both results and search type from wrapper
    // In process_user_input, you need to pass the current navigation directory
    let search_results =
        nav_state.fuzzy_search_manager_wrapper(input, all_entries, &current_directory_path);

    // Display paginated search results and get user selection
    let selection = display_paginated_search_results(
        &search_results,
        nav_state.tui_tall_adjustment,
        nav_state.tui_tall_direction_sign,
        nav_state.tui_wide_adjustment,
        nav_state.tui_wide_direction_sign,
    )
    .map_err(|e| {
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
        if let Some(selected) = search_results.iter().find(|r| match r {
            UnifiedSearchResult::Grep(g) => g.display_index == number,
            UnifiedSearchResult::Fuzzy(f) => f.display_index == number,
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
        extensions.insert("rs"); // Rust
        extensions.insert("c"); // C
        extensions.insert("cpp"); // C++
        extensions.insert("cxx"); // C++
        extensions.insert("cc"); // C++
        extensions.insert("h"); // C/C++ header
        extensions.insert("hpp"); // C++ header
        extensions.insert("hxx"); // C++ header
        extensions.insert("go"); // Go
        extensions.insert("zig"); // Zig
        extensions.insert("nim"); // Nim
        extensions.insert("v"); // V

        // Programming languages - JVM
        extensions.insert("java"); // Java
        extensions.insert("kt"); // Kotlin
        extensions.insert("kts"); // Kotlin Script
        extensions.insert("scala"); // Scala
        extensions.insert("clj"); // Clojure
        extensions.insert("cljs"); // ClojureScript
        extensions.insert("groovy"); // Groovy

        // Programming languages - Scripting
        extensions.insert("py"); // Python
        extensions.insert("pyw"); // Python Windows
        extensions.insert("pyi"); // Python Interface
        extensions.insert("rb"); // Ruby
        extensions.insert("php"); // PHP
        extensions.insert("lua"); // Lua
        extensions.insert("perl"); // Perl
        extensions.insert("pl"); // Perl

        // Programming languages - Web/JavaScript ecosystem
        extensions.insert("js"); // JavaScript
        extensions.insert("mjs"); // JavaScript Module
        extensions.insert("cjs"); // CommonJS
        extensions.insert("ts"); // TypeScript
        extensions.insert("tsx"); // TypeScript JSX
        extensions.insert("jsx"); // React JSX
        extensions.insert("vue"); // Vue.js
        extensions.insert("svelte"); // Svelte

        // Programming languages - Functional
        extensions.insert("hs"); // Haskell
        extensions.insert("lhs"); // Literate Haskell
        extensions.insert("ml"); // OCaml/Standard ML
        extensions.insert("mli"); // OCaml Interface
        extensions.insert("fs"); // F#
        extensions.insert("fsx"); // F# Script
        extensions.insert("elm"); // Elm
        extensions.insert("ex"); // Elixir
        extensions.insert("exs"); // Elixir Script
        extensions.insert("erl"); // Erlang
        extensions.insert("hrl"); // Erlang Header

        // Programming languages - Other
        extensions.insert("r"); // R
        extensions.insert("jl"); // Julia
        extensions.insert("m"); // MATLAB/Objective-C
        extensions.insert("swift"); // Swift
        extensions.insert("dart"); // Dart
        extensions.insert("pas"); // Pascal
        extensions.insert("pp"); // Pascal
        extensions.insert("asm"); // Assembly
        extensions.insert("s"); // Assembly

        // Web technologies
        extensions.insert("html"); // HTML
        extensions.insert("htm"); // HTML
        extensions.insert("xhtml"); // XHTML
        extensions.insert("css"); // CSS
        extensions.insert("scss"); // Sass
        extensions.insert("sass"); // Sass
        extensions.insert("less"); // Less
        extensions.insert("styl"); // Stylus

        // Documentation and markup
        extensions.insert("md"); // Markdown
        extensions.insert("markdown"); // Markdown
        extensions.insert("rst"); // reStructuredText
        extensions.insert("tex"); // LaTeX
        extensions.insert("latex"); // LaTeX
        extensions.insert("adoc"); // AsciiDoc
        extensions.insert("asciidoc"); // AsciiDoc
        extensions.insert("org"); // Org mode
        extensions.insert("pod"); // Perl POD
        extensions.insert("rdoc"); // Ruby Doc

        // Shell scripts
        extensions.insert("sh"); // Shell
        extensions.insert("bash"); // Bash
        extensions.insert("zsh"); // Zsh
        extensions.insert("fish"); // Fish
        extensions.insert("ksh"); // Korn Shell
        extensions.insert("csh"); // C Shell
        extensions.insert("tcsh"); // TC Shell
        extensions.insert("ps1"); // PowerShell
        extensions.insert("psm1"); // PowerShell Module
        extensions.insert("bat"); // Batch
        extensions.insert("cmd"); // Command

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
        extensions.insert("sql"); // SQL
        extensions.insert("psql"); // PostgreSQL
        extensions.insert("mysql"); // MySQL
        extensions.insert("graphql"); // GraphQL
        extensions.insert("gql"); // GraphQL
        extensions.insert("prisma"); // Prisma Schema

        // Data formats and protocols
        extensions.insert("proto"); // Protocol Buffers
        extensions.insert("thrift"); // Apache Thrift
        extensions.insert("avdl"); // Avro IDL
        extensions.insert("avsc"); // Avro Schema

        // Log and output files
        extensions.insert("log"); // Log files
        extensions.insert("out"); // Output files
        extensions.insert("err"); // Error files

        // Diff and patch files
        extensions.insert("diff"); // Diff
        extensions.insert("patch"); // Patch

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
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    // Check for common extensionless text files
    // These are often found in project roots
    let extensionless_text_files = [
        "README",
        "LICENSE",
        "CHANGELOG",
        "AUTHORS",
        "CONTRIBUTORS",
        "COPYRIGHT",
        "NOTICE",
        "TODO",
        "Makefile",
        "Dockerfile",
        "Containerfile",
        "Vagrantfile",
        "Jenkinsfile",
        "Procfile",
        "Gemfile",
        "Podfile",
        "Rakefile",
        ".gitignore",     // Add this
        ".gitattributes", // And other common dotfiles
        ".editorconfig",
        ".env.example",
        ".dockerignore",
        ".env",
        ".config",
        ".flake8",
    ];

    // Check if it's a known extensionless text file (case-insensitive)
    let file_name_upper = file_name.to_uppercase();
    if extensionless_text_files
        .iter()
        .any(|&known| known.to_uppercase() == file_name_upper)
    {
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

    /// Exit the ff application/process (default return is nav-path)
    ///
    /// Generated by 'q' command
    Quit,

    /// Exit the ff application/process (default return is nav-path)
    ///
    /// Generated by: {selected item number} --return-path
    ReturnPathExitFF(PathBuf),

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

    GoToHelpMenuMode,

    // Module:
    GoToSouceCode,

    GoToFileLineCountMode,
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
/// / Determine action based on item type
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
/// / When user enters a number
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
/// / Creating a FileSystemEntry from directory read results
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
        tui_tall_adjustment as i16 // Positive adjustment - more rows
    } else {
        -(tui_tall_adjustment as i16) // Negative adjustment - fewer rows
    };

    // Calculate final items per page, ensuring minimum of 5 for usability
    let items_per_page = std::cmp::max(5, base_items_per_page + tall_adjustment) as usize;

    // Apply width adjustment based on direction
    // true = wider (show more of filename), false = narrower (show less of filename)
    // Start with the default width constant, convert to signed for arithmetic
    let base_width = MAX_TUI_CHAR_LENGTH_DEFAULT as i16;

    // Calculate the adjustment as signed (can be negative)
    let width_adjustment = if tui_wide_direction_sign {
        tui_wide_adjustment as i16 // Positive adjustment - wider
    } else {
        -(tui_wide_adjustment as i16) // Negative adjustment - narrower
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
                display_grep_page(page_results, current_page + 1, total_pages, max_total_width)?;
            }
            UnifiedSearchResult::Fuzzy(_) => {
                display_fuzzy_page(page_results, current_page + 1, total_pages, max_total_width)?;
            }
        }

        // Display pagination information and navigation instructions
        println!(
            "\nPage {}/{}, {} results, up w/j/<,  down x/k/>",
            current_page + 1,
            total_pages,
            results.len(),
        );
        print!("Enter choice: ");
        io::stdout().flush()?; // Ensure prompt is displayed before waiting for input

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
                println!(
                    "\nInvalid selection. Please choose 1-{}",
                    end_idx - start_idx
                );
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
    max_total_width: usize,
) -> io::Result<()> {
    println!(
        "\nContent Search Results (Page {}/{}) (try: -g -r -c)",
        current_page, total_pages
    );

    // Calculate column widths based on total terminal width
    // Fixed allocations: 3 for number, 7 for line (includes spacing)
    const FIXED_WIDTH: usize = 10; // 3 (number) + 7 (line with spaces)

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
    let file_width = (available_width * 6) / 10; // 60% for file
    let content_width = available_width - file_width; // Remainder for content

    // Display header with exact column widths
    // Format: [##_][File...][_Line_][Content...]
    print!("{:<2} ", "#"); // 2 chars + 1 space = 3 total
    print!("{:<width$}", "File", width = file_width);
    print!(" {:<5} ", "Line"); // 1 + 5 + 1 = 7 total
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
            print!("{:<2} ", row_num); // 2 chars + 1 space
            print!("{:<width$}", display_file, width = file_width);
            print!(" {} ", line_str); // 1 + 5 + 1 spaces
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
            println!(
                "{}. {}:{}",
                index + 1,
                truncate_with_ellipsis(&grep_result.file_name, file_width),
                grep_result.line_number
            );
            println!(
                "   {}",
                truncate_with_ellipsis(grep_result.line_content.trim(), content_width)
            );
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
    max_total_width: usize,
) -> io::Result<()> {
    println!(
        "Fuzzy Search (Page {}/{}) (try: --grep --recursive --case-sensitive, -g -r -c)",
        current_page, total_pages
    );

    // Calculate column widths based on total terminal width
    // Fixed allocations: 3 for number, 10 for distance (includes spacing)
    const FIXED_WIDTH: usize = 13; // 3 (number) + 10 (distance with spaces)

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
    print!("{:<2} ", "#"); // 2 chars + 1 space = 3 total
    print!("{:<width$}", "Name", width = name_width);
    println!(" {:<9}", "Distance"); // 1 + 9 = 10 total

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
            print!("{:<2} ", row_num); // 2 chars + 1 space
            print!("{:<width$}", display_name, width = name_width);
            println!(" {}", distance_str); // 1 space + distance
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
            println!(
                "{}. {} (dist: {})",
                index + 1,
                truncate_with_ellipsis(&fuzzy_result.item_name, 20),
                fuzzy_result.distance
            );
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
    /// / Create a simple filename search configuration
    /// let config = SearchConfig::new("document".to_string());
    ///
    /// / Create and customize with builder methods
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
    /// / Search only in current directory (default)
    /// let config = SearchConfig::new("test".to_string())
    ///     .with_recursive(false);
    ///
    /// / Search current directory and all subdirectories
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
    /// / Search for "TODO" in filenames (fuzzy match)
    /// let config = SearchConfig::new("TODO".to_string())
    ///     .with_grep(false);
    ///
    /// / Search for "TODO" inside file contents
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
    /// / Case-insensitive grep search (default) - finds "todo", "TODO", "ToDo"
    /// let config = SearchConfig::new("todo".to_string())
    ///     .with_grep(true)
    ///     .with_case_sensitive(false);
    ///
    /// / Case-sensitive grep search - only finds exact "TODO"
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
    use std::collections::HashMap;
    use std::path::Path;

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
            lines_editor_session_path: PathBuf::new(),
        }
    }

    #[test]
    fn test_collect_entries_iterative_basic() {
        // Test basic functionality - can collect entries from temp directory
        let nav_state = create_test_navigation_state();
        let current_dir = std::env::current_dir().expect("Should be able to get current directory");

        let result = nav_state.collect_entries_iterative(&current_dir, Some(0), None, None);

        assert!(result.is_ok(), "Should successfully read temp directory");
        let entries = result.expect("Failed to get entries");
        assert!(
            !entries.is_empty(),
            "Temp directory should have at least some entries"
        );
    }

    #[test]
    fn test_collect_entries_iterative_depth_limit() {
        // Test that depth limit of 0 only returns immediate children
        let nav_state = create_test_navigation_state();
        let temp_dir = std::env::temp_dir();

        // Depth 0 = only immediate children of temp dir
        let result_depth_0 = nav_state.collect_entries_iterative(&temp_dir, Some(0), None, None);

        assert!(result_depth_0.is_ok(), "Depth 0 should succeed");
        let entries_0 = result_depth_0.expect("Failed to get entries");

        // Depth 1 = temp dir children and their children
        let result_depth_1 = nav_state.collect_entries_iterative(&temp_dir, Some(1), None, None);

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
            Some(5), // Maximum 5 entries
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
        let nonexistent_path =
            Path::new("/this/directory/definitely/does/not/exist/anywhere/12345");

        let result = nav_state.collect_entries_iterative(nonexistent_path, None, None, None);

        // Should return Ok with empty vec since we skip unreadable directories
        assert!(
            result.is_ok(),
            "Should return Ok even for non-existent directory"
        );
        let entries = result.expect("Failed to get entries");
        assert!(
            entries.is_empty(),
            "Non-existent directory should return empty vec"
        );
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
            Some(1), // 1 MB limit
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
            Some(1),  // Depth limit
            Some(10), // Entry limit
            Some(5),  // Memory limit (MB)
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
            Some(0), // Only immediate children
            Some(5), // Limit to a few entries
            None,
        );

        assert!(result.is_ok(), "Should collect entries successfully");
        let entries = result.expect("Failed to get entries");

        // Verify each entry has valid attributes
        for entry in entries.iter() {
            // Name should not be empty
            assert!(
                !entry.file_system_item_name.is_empty(),
                "File name should not be empty"
            );

            // Path should exist as a string representation
            assert!(
                entry.file_system_item_path.to_str().is_some(),
                "Path should be valid UTF-8"
            );

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

    /// for default file editor
    lines_editor_session_path: PathBuf,
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
    /// / nav_state is ready to be used with initial directory read
    /// / Display will use default sizes until user adjusts
    /// ```
    fn new() -> Self {
        // Detect if running on Android platform
        let is_android = detect_android();

        // Set width adjustment based on platform
        // Android terminals typically need reduced width
        let (width_adjustment, width_direction) = if is_android {
            // Android: reduce width by 24 characters
            (24, false) // false represents negative direction
        } else {
            // Non-Android: no adjustment needed
            (0, true) // true represents positive direction (though 0 makes direction irrelevant)
        };

        NavigationState {
            display_lookup_table: HashMap::new(),
            current_sort_method: DirectorySortingMethodEnum::Name(true),
            last_sort_command: None,
            current_filter: None, // No filter initially
            selected_item_index: None,
            active_search_term: None,
            // Initialize TUI size adjustments to defaults (no adjustment)
            tui_tall_adjustment: 0,                   // No height adjustment
            tui_tall_direction_sign: true,            // Positive direction by default
            tui_wide_adjustment: width_adjustment,    // No width adjustment
            tui_wide_direction_sign: width_direction, // Positive direction by default
            current_page_index: 0,                    // Always start at page 0
            lines_editor_session_path: PathBuf::new(),
        }
    }

    /// Resets navigation state to clean defaults while preserving location and sort
    ///
    /// # Purpose
    /// Clears filters, pagination, selection, sort, and search state to provide a "clean slate"
    /// view of the current directory. Defaults to Name "Ascending" (alphabetal order).
    ///
    /// Alternate design option: preserving the user's preferred sort method
    /// and current location.
    ///
    /// # State Reset
    /// - Clears any active filters (show all files and directories)
    /// - Resets to first page (page 0)
    /// - Clears any selected item
    /// - Clears any active search term
    /// - Defaults to Name "Ascending" (alphabetal order).
    ///
    /// # Usage Context
    /// Called when user presses Enter with no input to "refresh" and clear all
    /// applied filters, searches, and navigation state while staying in the
    /// same directory with the same sort order.
    ///
    /// # Example
    /// ```rust
    /// / User has filtered to files only, on page 3, with item 5 selected
    /// / User presses Enter to reset
    /// nav_state.reset_to_clean_state();
    /// / Now: no filter, page 1, no selection, no search, same sort method
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

        // Defaults to Name "Ascending" (alphabetal order)
        self.current_sort_method = DirectorySortingMethodEnum::Name(true);
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
    ) -> Vec<UnifiedSearchResult> {
        // Note: No longer returns tuple with bool

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
                Some(20), // Maximum depth of 20 levels (prevents infinite loops, covers 99.9% of real use cases)
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
                    grep_results
                        .into_iter()
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
            fuzzy_results
                .into_iter()
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
    /// / Show only directories
    /// nav_state.set_filter('d');
    ///
    /// / Show only files
    /// nav_state.set_filter('f');
    ///
    /// / Toggle directories filter off (if currently showing only directories)
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
    /// / Apply current filter and get filtered entries to display
    /// let filtered_entries = nav_state.apply_filter(&all_entries);
    /// display_directory_contents(&filtered_entries, &current_directory)?;
    /// ```
    fn apply_filter<'a>(&self, entries: &'a [FileSystemEntry]) -> Vec<&'a FileSystemEntry> {
        match self.current_filter {
            Some('d') => entries.iter().filter(|e| e.is_directory).collect(),
            Some('f') => entries.iter().filter(|e| !e.is_directory).collect(),
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
        const ENTRY_SIZE_ESTIMATE_BYTES: usize = 256; // Approximate size of one FileSystemEntry in memory
        const QUEUE_ITEM_SIZE_ESTIMATE_BYTES: usize = 128; // Approximate size of one (PathBuf, usize) tuple
        let mut estimated_memory_bytes: usize = 0;

        // Progress tracking for diagnostics
        let mut directories_processed: usize = 0;
        let mut directories_skipped: usize = 0;

        // Main traversal loop - continues until queue is empty or limits are reached
        while let Some((current_dir, current_depth)) = directories_queue.pop_front() {
            directories_processed += 1;

            // Update memory estimate by removing the dequeued item
            estimated_memory_bytes =
                estimated_memory_bytes.saturating_sub(QUEUE_ITEM_SIZE_ESTIMATE_BYTES);

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
                    eprintln!(
                        "Warning: Estimated memory usage ({} MB) exceeds limit ({} MB), stopping traversal",
                        estimated_memory_bytes / (1024 * 1024),
                        max_mem
                    );
                    break;
                }
            }

            // Attempt to read the directory contents
            let dir_entries = match fs::read_dir(&current_dir) {
                Ok(entries) => entries,
                Err(e) => {
                    // Log the error but continue processing other directories
                    // This handles permission denied, symbolic link issues, etc.
                    eprintln!(
                        "Warning: Cannot read directory '{}': {}",
                        current_dir.display(),
                        e
                    );
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
                    file_system_item_last_modified_time: metadata
                        .modified()
                        .unwrap_or(SystemTime::UNIX_EPOCH),
                    is_directory,
                });

                // Update memory estimate after adding entry
                estimated_memory_bytes =
                    estimated_memory_bytes.saturating_add(ENTRY_SIZE_ESTIMATE_BYTES);

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
                            Some(max_d) => current_depth < max_d, // current_depth + 1 <= max_d
                            None => true,
                        };

                        // Check if we have memory budget for more queue items
                        let within_memory_limit = match max_memory_mb {
                            Some(max_mem) => {
                                let max_bytes = max_mem.saturating_mul(1024).saturating_mul(1024);
                                estimated_memory_bytes
                                    .saturating_add(QUEUE_ITEM_SIZE_ESTIMATE_BYTES)
                                    <= max_bytes
                            }
                            None => true,
                        };

                        within_entry_limit && within_depth_limit && within_memory_limit
                    };

                    if should_queue_subdirectory {
                        directories_queue.push_back((path, current_depth + 1));
                        estimated_memory_bytes =
                            estimated_memory_bytes.saturating_add(QUEUE_ITEM_SIZE_ESTIMATE_BYTES);
                    }
                }
            }

            // Provide progress feedback for very large traversals (every 1000 directories)
            if directories_processed % 1000 == 0 && directories_processed > 0 {
                eprintln!(
                    "Progress: Processed {} directories, {} entries collected, {} directories queued",
                    directories_processed,
                    all_entries.len(),
                    directories_queue.len()
                );
            }
        }

        // Log final statistics if any directories were skipped
        if directories_skipped > 0 {
            eprintln!(
                "Traversal complete: {} directories processed, {} skipped, {} entries collected",
                directories_processed,
                directories_skipped,
                all_entries.len()
            );
        }

        Ok(all_entries)
    }

    /// TODO: make this more parallel?
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
    /// / WRONG: Using enumeration index from original entries
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
    /// / Step 1: Collect all matches with placeholder indices
    /// for entry in entries.iter() {
    ///     if matches_search {
    ///         results.push(FuzzySearchResult {
    ///             display_index: 0,  // Temporary placeholder
    ///             ...
    ///         });
    ///     }
    /// }
    ///
    /// / Step 2: Sort by relevance
    /// results.sort_by(|a, b| { ... });
    ///
    /// / Step 3: Assign sequential indices after sorting
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
    /// / Might find "document.txt" with distance 1 (one character substitution)
    /// / Results will have display_index values: 1, 2, 3, etc.
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
            let full_name_truncated: String = entry
                .file_system_item_name
                .to_lowercase()
                .chars() // Use chars() for proper UTF-8 handling
                .take(search_len) // Take only as many characters as search term
                .collect();

            let no_ext_truncated: String = name_without_ext
                .to_lowercase()
                .chars()
                .take(search_len)
                .collect();

            // Step 3: Calculate Levenshtein distances for both versions
            // This measures how many single-character edits are needed
            // Handle potential errors gracefully by skipping problematic entries
            let (distance_with_ext, distance_without_ext) = match (
                levenshtein_distance(&full_name_truncated, &search_term),
                levenshtein_distance(&no_ext_truncated, &search_term),
            ) {
                (Ok(dist_with), Ok(dist_without)) => (dist_with, dist_without),
                (Err(e), _) | (_, Err(e)) => {
                    // Print warning to stderr and skip this entry
                    eprintln!("Warning: Skipping file in fuzzy search - {}", e);
                    // Optionally include more context
                    eprintln!("  File path: {:?}", entry);
                    // Skip this entry by continuing to next iteration
                    continue;
                }
            };

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
                }
                other => other, // Different distances - use distance ordering
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
    /// / Results contain file path, line number, and context for each match
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
                        let truncated: String = line.chars().take(MAX_DISPLAY_CHARS).collect();
                        format!("{}...", truncated)
                    } else {
                        line.clone()
                    };

                    // Create search result for this match
                    results.push(GrepSearchResult {
                        file_name: entry.file_system_item_name.clone(),
                        file_path: entry.file_system_item_path.clone(),
                        line_number: line_number, // Direct field, not Option
                        line_content: context,    // Direct field, not Option
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
                        DirectorySortingMethodEnum::Name(ascending) => {
                            DirectorySortingMethodEnum::Name(!ascending)
                        }
                        _ => DirectorySortingMethodEnum::Name(true),
                    }
                } else {
                    DirectorySortingMethodEnum::Name(true)
                }
            }
            's' => {
                if self.last_sort_command == Some('s') {
                    match self.current_sort_method {
                        DirectorySortingMethodEnum::Size(ascending) => {
                            DirectorySortingMethodEnum::Size(!ascending)
                        }
                        _ => DirectorySortingMethodEnum::Size(true),
                    }
                } else {
                    DirectorySortingMethodEnum::Size(true)
                }
            }
            'm' => {
                if self.last_sort_command == Some('m') {
                    match self.current_sort_method {
                        DirectorySortingMethodEnum::Modified(ascending) => {
                            DirectorySortingMethodEnum::Modified(!ascending)
                        }
                        _ => DirectorySortingMethodEnum::Modified(true),
                    }
                } else {
                    DirectorySortingMethodEnum::Modified(true)
                }
            }
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
                },
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
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                return Err(FileFantasticError::NotFound(directory_path_to_read.clone()));
            }
            io::ErrorKind::PermissionDenied => {
                return Err(FileFantasticError::PermissionDenied(
                    directory_path_to_read.clone(),
                ));
            }
            _ => return Err(FileFantasticError::Io(e)),
        },
    };

    for directory_item_result in read_dir_result {
        // Handle errors reading directory entries
        let directory_item = match directory_item_result {
            Ok(item) => item,
            Err(e) => {
                // Log error but continue with other entries
                eprintln!(
                    "Warning: Failed to read entry in {}: {}",
                    directory_path_to_read.display(),
                    e
                );
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
                eprintln!(
                    "Warning: Cannot determine modification time for {}",
                    item_path.display()
                );
                SystemTime::UNIX_EPOCH
            }
        };

        directory_entries_list.push(FileSystemEntry {
            file_system_item_name: directory_item.file_name().to_string_lossy().to_string(),
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
        assert!(
            now_formatted.len() == 5,
            "Current time should be in HH:MM format"
        );

        // Test this year format (MM-DD HH:MM)
        let yesterday_formatted = format_timestamp(yesterday);
        assert!(
            yesterday_formatted.len() == 11,
            "Recent dates should be in MM-DD HH:MM format"
        );

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
/// / With max_name_width = 20
/// assert_eq!(
///     truncate_filename_for_display(long_name.to_string(), 20),
///     "really_long...y.txt"
/// );
///
/// let short_name = "short.txt";
/// / With max_name_width = 20
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
        .skip(
            formatted_name
                .chars()
                .count()
                .saturating_sub(FILENAME_SUFFIX_LENGTH),
        )
        .collect();

    // Combine prefix, ellipsis, and suffix
    format!("{}{}{}", prefix, ellipsis, suffix)
}

/// Formats the navigation legend with color-coded keyboard shortcuts
///
/// # Purpose
/// Creates a formatted legend string showing all available keyboard commands
/// with color highlighting (RED for command keys, YELLOW for descriptions).
///
/// # Returns
/// * `Ok(String)` - The formatted legend string with ANSI color codes
/// * `Err(FileFantasticError)` - If string formatting fails (defensive programming)
///
/// # Color Scheme
/// - RED: Single letter command keys (q, b, t, d, f, n, s, m, g, v, y, p)
/// - YELLOW: Command descriptions and separators
/// - RESET: Applied at end to restore terminal defaults
///
/// # Legend Commands
/// - q: quit the application
/// - b: navigate back/parent directory
/// - t: open terminal in current directory
/// - d: filter to show directories only
/// - f: filter to show files only
/// - n: sort by name
/// - s: sort by size
/// - m: sort by modified date
/// - g: get-send file operations
/// - v,y,p: additional file operations
/// - str: search functionality
/// - enter: reset filters/search
///
/// # Example
/// ```rust
/// match format_navigation_legend() {
///     Ok(legend) => println!("{}", legend),
///     Err(e) => eprintln!("Failed to format legend: {}", e),
/// }
/// ```
fn format_navigation_legend() -> Result<String> {
    // Pre-allocate string capacity based on expected legend size
    // Legend is approximately 200 characters plus color codes
    let mut legend = String::with_capacity(300);

    // Build the legend string with error handling for format operations
    let formatted = format!(
        "{}{}q{}uit {}b{}ack|{}t{}erm|{}d{}ir {}f{}ile|{}n{}ame {}s{}ize {}m{}od|{}g{}et-send file {}v{},{}y{},{}p{}|{}str{}>search|{}enter{}>reset{}",
        YELLOW, // Overall legend color
        RED,
        YELLOW, // RED q + YELLOW uit
        RED,
        YELLOW, // RED b + YELLOW ack
        RED,
        YELLOW, // RED t + YELLOW erm
        RED,
        YELLOW, // RED d + YELLOW ir
        RED,
        YELLOW, // RED f + YELLOW ile
        RED,
        YELLOW, // RED n + YELLOW ame
        RED,
        YELLOW, // RED s + YELLOW ize
        RED,
        YELLOW, // RED m + YELLOW od
        RED,
        YELLOW, // RED g + YELLOW et
        RED,
        YELLOW, // RED v + YELLOW ,
        RED,
        YELLOW, // RED y + YELLOW ,
        RED,
        YELLOW, // RED p + YELLOW ,
        RED,
        YELLOW, // RED str + YELLOW ...
        RED,
        YELLOW, // RED enter + YELLOW ...
        RESET
    );

    // Check if the formatted string is reasonable
    // (defensive programming against format! macro issues)
    if formatted.is_empty() {
        return Err(FileFantasticError::InvalidName(String::from(
            "Legend formatting produced empty string",
        )));
    }

    // Verify the string doesn't exceed reasonable bounds
    // Terminal width is typically 80-200 chars, legend should fit
    const MAX_LEGEND_LENGTH: usize = 500;
    if formatted.len() > MAX_LEGEND_LENGTH {
        return Err(FileFantasticError::InvalidName(format!(
            "Legend too long: {} chars (max {})",
            formatted.len(),
            MAX_LEGEND_LENGTH
        )));
    }

    legend.push_str(&formatted);
    Ok(legend)
}

/// Counts all items in a directory using arbitrary-precision counters
///
/// # Purpose
/// Provides summary statistics for directory contents using Ribbon counters
/// that can handle arbitrarily large counts without overflow.
///
/// # Arguments
/// * `path` - Reference to PathBuf of the directory to count
///
/// # Returns
/// A tuple of three Strings: (all_count, file_count, dir_count)
/// - `all_count`: Total number of items (files + dirs + symlinks + special files)
/// - `file_count`: Number of regular files only
/// - `dir_count`: Number of directories only
///
/// # Error Handling
/// Returns ("?", "?", "?") if any error occurs:
/// - Directory cannot be read (permissions, doesn't exist, etc.)
/// - Metadata cannot be accessed for items
/// - Counter increment fails
/// - Counter-to-decimal conversion fails
/// - String conversion fails
/// - Any other I/O error
///
/// # Memory Behavior
/// Uses three CascadingHexCounter instances (~7KB stack allocation).
/// All counters start at Tier 0 (4 bytes active) and grow only if needed.
/// Zero heap allocation - all operations use stack-only memory.
///
/// # Scope
/// - Counts ONLY items in the specified directory (flat, non-recursive)
/// - Includes hidden files and directories (those starting with '.')
/// - Includes ALL item types (files, dirs, symlinks, FIFOs, sockets, etc.)
/// - Does NOT follow symlinks or recurse into subdirectories
///
/// # Loop Safety
/// The iterator from fs::read_dir() is naturally bounded by the number of
/// entries in the directory.
///
/// # Examples
/// ```rust
/// let path = PathBuf::from("/home/user/documents");
/// let (all, files, dirs) = count_directory_items_ribbon(&path);
/// println!("Total: {}, Files: {}, Directories: {}", all, files, dirs);
/// // Output: "Total: 42, Files: 35, Directories: 7"
///
/// // On error:
/// let bad_path = PathBuf::from("/nonexistent");
/// let (all, files, dirs) = count_directory_items_ribbon(&bad_path);
/// // Returns: ("?", "?", "?")
/// ```
fn count_directoryitems_ribbon(path: &PathBuf) -> (String, String, String) {
    // Error handling: Path validation (defensive check)
    if path.as_os_str().is_empty() {
        return (String::from("?"), String::from("?"), String::from("?"));
    }

    // Initialize three arbitrary-precision counters (stack-only, ~7KB)
    // Each starts at Tier 0 (4 bytes active) and grows automatically if needed
    let mut all_counter = CascadingHexCounter::new();
    let mut file_counter = CascadingHexCounter::new();
    let mut dir_counter = CascadingHexCounter::new();

    // Attempt to read directory contents
    // Error handling: Cannot read directory
    let directory_reader = match fs::read_dir(path) {
        Ok(reader) => reader,
        Err(_) => {
            // Cannot read directory - return unknowns
            return (String::from("?"), String::from("?"), String::from("?"));
        }
    };

    // Iterate through directory entries
    // The iterator is naturally bounded by the filesystem
    for entry_result in directory_reader {
        // Error handling: Reading directory entry
        let entry = match entry_result {
            Ok(e) => e,
            Err(_) => {
                // Error reading this specific entry - skip it
                continue;
            }
        };

        // Error handling: Getting metadata for entry
        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(_) => {
                // Cannot read metadata for this entry - skip it
                continue;
            }
        };

        // Error handling: Counter increment
        // Count this item in 'all' using Ribbon's increment
        if let Err(_) = all_counter.increment() {
            // Increment failed (extremely rare - would require tier promotion failure)
            return (String::from("?"), String::from("?"), String::from("?"));
        }

        // Classify the item type and increment appropriate counter
        if metadata.is_file() {
            // Error handling: File counter increment
            if let Err(_) = file_counter.increment() {
                return (String::from("?"), String::from("?"), String::from("?"));
            }
        } else if metadata.is_dir() {
            // Error handling: Directory counter increment
            if let Err(_) = dir_counter.increment() {
                return (String::from("?"), String::from("?"), String::from("?"));
            }
        }
        // Note: Symlinks, FIFOs, sockets, etc. are counted only in 'all'
    }

    // Convert counters to decimal strings
    // Pre-allocate conversion buffer on stack (reused for all three conversions)
    let mut decimal_buffer = [0u8; 2500]; // MAX_DECIMAL_DIGITS from ribbon

    // Convert all_counter to decimal string
    // Error handling: Counter-to-decimal conversion
    let all_str = match all_counter.to_decimal(&mut decimal_buffer) {
        Ok(length) => {
            // Error handling: UTF-8 validation
            match std::str::from_utf8(&decimal_buffer[..length]) {
                Ok(valid_str) => String::from(valid_str),
                Err(_) => {
                    // UTF-8 conversion failed - should never happen with decimal digits
                    return (String::from("?"), String::from("?"), String::from("?"));
                }
            }
        }
        Err(_) => {
            // Decimal conversion failed
            return (String::from("?"), String::from("?"), String::from("?"));
        }
    };

    // // // diagnostics demo
    // // let hex_len = all_counter.to_hex(&mut decimal_buffer);
    // // let hex_str = std::str::from_utf8(&decimal_buffer[..hex_len]);
    // // println!(" all_counter Hex representation: 0x{}", hex_str);
    // let reset_result = all_counter.reset();
    // println!(
    //     "  Ribbon Diagnostic After reset: tier={}, digits={} reset_result={:?}",
    //     all_counter.current_tier(),
    //     all_counter.digit_count(),
    //     reset_result,
    // );

    // Convert file_counter to decimal string
    // Error handling: Counter-to-decimal conversion
    let file_str = match file_counter.to_decimal(&mut decimal_buffer) {
        Ok(length) => {
            // Error handling: UTF-8 validation
            match std::str::from_utf8(&decimal_buffer[..length]) {
                Ok(valid_str) => String::from(valid_str),
                Err(_) => {
                    return (String::from("?"), String::from("?"), String::from("?"));
                }
            }
        }
        Err(_) => {
            return (String::from("?"), String::from("?"), String::from("?"));
        }
    };

    // Convert dir_counter to decimal string
    // Error handling: Counter-to-decimal conversion
    let dir_str = match dir_counter.to_decimal(&mut decimal_buffer) {
        Ok(length) => {
            // Error handling: UTF-8 validation
            match std::str::from_utf8(&decimal_buffer[..length]) {
                Ok(valid_str) => String::from(valid_str),
                Err(_) => {
                    return (String::from("?"), String::from("?"), String::from("?"));
                }
            }
        }
        Err(_) => {
            return (String::from("?"), String::from("?"), String::from("?"));
        }
    };

    // Final sanity check: Ensure strings are valid
    // This is defensive - decimal conversion should never produce empty strings for counts
    if all_str.is_empty() || file_str.is_empty() || dir_str.is_empty() {
        return (String::from("?"), String::from("?"), String::from("?"));
    }

    // Note: Logic consistency check (files + dirs <= all) could be added here
    // but would require parsing strings back to numbers, which adds complexity
    // and potential failure points. For directory counting, inconsistency
    // should not occur if the code above is correct.

    (all_str, file_str, dir_str)
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
/// - Summary line shows total counts: a{all} f{files} d{dirs} {path}
///
/// # Layout Calculation
/// The name column width is calculated dynamically:
/// - Base width: 55 characters (MAX_NAME_LENGTH_DEFAULT)
/// - Adjusted by user's wide+N or wide-N settings
/// - Minimum width: 8 characters (for "...suffix" display)
///
/// # Summary Counts
/// The path line includes directory summary statistics:
/// - a{N}: Total count of ALL items (files + dirs + symlinks + special files)
/// - f{N}: Count of regular files only
/// - d{N}: Count of directories only
/// - These are flat counts (non-recursive, current directory only)
/// - Shows "?" if counting fails (permissions, errors, etc.)
fn display_directory_contents(
    directory_entries: &[FileSystemEntry],
    current_directory_path: &PathBuf,
    page_info: Option<(usize, usize)>,
    filter: Option<char>,
    nav_state: &NavigationState, // Add nav_state parameter
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

    // Format the navigation legend with error handling
    let legend = match format_navigation_legend() {
        Ok(legend_text) => legend_text,
        Err(e) => {
            // Fall back to a simple legend without colors if formatting fails
            // This ensures the UI remains functional even if legend formatting has issues
            eprintln!("Warning: Legend formatting failed: {}", e);
            String::from("quit|back|term|dir|file|name|size|mod|get|search|reset")
        }
    };

    // Get directory summary counts (flat, non-recursive)
    // Returns ("?", "?", "?") if counting fails for any reason
    let (all_count, file_count, dir_count) = count_directoryitems_ribbon(current_directory_path);

    // Format path display with summary counts
    // Format: a{all} f{files} d{dirs} {path}
    // Letters are colored YELLOW for visibility
    let path_display = format!(
        "{}f{}{} {}d{}{} {}a{}{} {}",
        YELLOW,
        RESET,
        file_count, // total file count file_count
        YELLOW,
        RESET,
        dir_count, // total directory count dir_count
        YELLOW,
        RESET,
        all_count, // all items of any kind all_count
        current_directory_path.display()
    );

    // Display legend and path with counts
    println!("{}\n{}{}", legend, filter_status, path_display);

    // Column headers with dynamic name width
    println!(
        "{:>4}  {:<width$} {:>5} {:>11}",
        " # ",
        "Name",
        "Size",
        "Modified",
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
            nav_state.tui_wide_direction_sign,
        );

        if total_pages > 1 {
            println!(
                "\x1b[1m{}--- Page {} of {}: up/down, j/k, </>, w/x, arrows, etc. Size: {} {} ---{}",
                YELLOW, current_page, total_pages, tall_display, wide_display, RESET
            );
        } else {
            // Show size info even when only one page
            println!(
                "\x1b[1m{}--- (Re)Size: {} {} ---   See files' row counts using:  --row-counts{}",
                YELLOW, tall_display, wide_display, RESET
            );
        }
    }

    io::stdout().flush()?;
    Ok(())
}

#[cfg(test)]
mod displaytests {
    use super::*;
    use std::path::PathBuf;

    /// Test counting items in a directory
    ///
    /// # Note
    /// This test uses the current directory as a test case, which should
    /// always exist and be readable during testing.
    #[test]
    fn test_count_directory_items_current_dir() {
        // Test with current directory (should always work)
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => {
                // Skip test if we can't get current directory
                return;
            }
        };

        let (all, files, dirs) = count_directoryitems_ribbon(&current_dir);

        // Should not return error markers
        assert_ne!(all, "?", "Should be able to count current directory");
        assert_ne!(files, "?", "Should be able to count files");
        assert_ne!(dirs, "?", "Should be able to count directories");

        // Counts should be valid numbers
        let all_num: u32 = all.parse().expect("all should be a number");
        let files_num: u32 = files.parse().expect("files should be a number");
        let dirs_num: u32 = dirs.parse().expect("dirs should be a number");

        // Logical assertions
        assert!(
            files_num + dirs_num <= all_num,
            "Files + dirs should not exceed total"
        );
    }

    /// Test counting with a nonexistent directory
    ///
    /// # Expected Behavior
    /// Should return ("?", "?", "?") without crashing
    #[test]
    fn test_count_directory_items_nonexistent() {
        let nonexistent = PathBuf::from("/this/path/should/not/exist/hopefully/12345");
        let (all, files, dirs) = count_directoryitems_ribbon(&nonexistent);

        // Should return error markers
        assert_eq!(all, "?", "Should return ? for nonexistent path");
        assert_eq!(files, "?", "Should return ? for nonexistent path");
        assert_eq!(dirs, "?", "Should return ? for nonexistent path");
    }

    /// Test that counts are internally consistent
    ///
    /// # Logic Test
    /// The sum of files and directories should never exceed the total count
    /// since 'all' includes files, directories, AND other items (symlinks, etc.)
    #[test]
    fn test_count_consistency() {
        let test_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => return, // Skip if can't get directory
        };

        let (all_str, files_str, dirs_str) = count_directoryitems_ribbon(&test_dir);

        // Skip if we got error markers
        if all_str == "?" {
            return;
        }

        let all: u32 = all_str.parse().unwrap_or(0);
        let files: u32 = files_str.parse().unwrap_or(0);
        let dirs: u32 = dirs_str.parse().unwrap_or(0);

        assert!(
            files + dirs <= all,
            "Internal consistency: files({}) + dirs({}) should not exceed all({})",
            files,
            dirs,
            all
        );
    }
}

/// Name of the data directory that contains File Fantastic configuration files
const FF_DATA_DIRECTORY_NAME: &str = "ff_data";

/// Name of the configuration file that contains partner program paths
const PARTNER_PROGRAMS_CONFIG_FILENAME: &str =
    "absolute_paths_to_local_partner_fileopening_executibles.txt";

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
        Ok(exe_path) => match exe_path.parent() {
            Some(parent) => parent.to_path_buf(),
            None => {
                eprintln!("Warning: Cannot determine executable directory for partner programs");
                return Vec::new();
            }
        },
        Err(error) => {
            eprintln!(
                "Warning: Cannot locate executable for partner programs: {}",
                error
            );
            return Vec::new();
        }
    };

    // Step 2: Build path to ff_data directory
    let ff_data_directory_path = executable_directory.join(FF_DATA_DIRECTORY_NAME);

    // Step 3: Create ff_data directory if it doesn't exist
    if !ff_data_directory_path.exists() {
        match fs::create_dir(&ff_data_directory_path) {
            Ok(_) => {
                println!(
                    "Created File Fantastic data directory: {}",
                    ff_data_directory_path.display()
                );
            }
            Err(error) => {
                eprintln!(
                    "Warning: Could not create ff_data directory at {}: {}",
                    ff_data_directory_path.display(),
                    error
                );
                return Vec::new();
            }
        }
    } else if !ff_data_directory_path.is_dir() {
        // Path exists but is not a directory
        eprintln!(
            "Warning: ff_data path exists but is not a directory: {}",
            ff_data_directory_path.display()
        );
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
                println!(
                    "Created partner programs configuration file: {}",
                    config_file_path.display()
                );
                println!("Edit this file to add your custom executables for file opening");
            }
            Err(error) => {
                eprintln!(
                    "Warning: Could not create partner programs configuration file at {}: {}",
                    config_file_path.display(),
                    error
                );
            }
        }
        // Return empty vector since new file has no configured programs yet
        return Vec::new();
    }

    // Step 6: Read the existing configuration file
    let file_contents = match fs::read_to_string(&config_file_path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!(
                "Warning: Could not read partner programs file at {}: {}",
                config_file_path.display(),
                error
            );
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
            eprintln!(
                "Warning: Partner program path does not exist (line {}): {}",
                line_index + 1,
                trimmed_line
            );
            continue;
        }

        // Validation 2: Check if path points to a file (not directory)
        if !program_path.is_file() {
            eprintln!(
                "Warning: Partner program path is not a file (line {}): {}",
                line_index + 1,
                trimmed_line
            );
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
                        eprintln!(
                            "Warning: Partner program lacks execute permissions (line {}): {}",
                            line_index + 1,
                            trimmed_line
                        );
                        continue;
                    }
                }
                Err(error) => {
                    eprintln!(
                        "Warning: Could not check permissions for partner program (line {}): {}",
                        line_index + 1,
                        error
                    );
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
    program_path
        .file_name()
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
        return Err(FileFantasticError::InvalidName(format!(
            "Partner program is not a file: {}",
            program_path.display()
        )));
    }
    // Launch using platform-specific terminal commands (reusing existing logic patterns)
    #[cfg(target_os = "macos")]
    {
        // Build command string for macOS Terminal.app
        let command_string = format!(
            "{} {}",
            program_path.to_string_lossy(),
            file_path.to_string_lossy()
        );

        std::process::Command::new("open")
            .args(["-a", "Terminal"])
            .arg(format!("{}; exit", command_string))
            .spawn()
            .map_err(|e| {
                eprintln!("Failed to open Terminal.app for partner program: {}", e);
                FileFantasticError::EditorLaunchFailed(extract_program_display_name(program_path))
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
            cmd.args(args).arg(program_path).arg(file_path);

            if cmd.spawn().is_ok() {
                terminal_launched = true;
                break;
            }
        }

        if !terminal_launched {
            return Err(FileFantasticError::NoTerminalFound);
        }
    }
    #[cfg(any(
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    {
        // BSD systems - try common terminal emulators
        let terminal_commands = [
            ("xterm", vec!["-e"]),
            ("rxvt", vec!["-e"]),
            ("urxvt", vec!["-e"]),
            ("konsole", vec!["--e"]),
            ("gnome-terminal", vec!["--"]),
            ("xfce4-terminal", vec!["--command"]),
            ("mate-terminal", vec!["--command"]),
            ("alacritty", vec!["-e"]),
            ("kitty", vec!["-e"]),
            ("sakura", vec!["-e"]),
            ("st", vec!["-e"]), // suckless terminal, popular on BSD
        ];

        let mut terminal_launched = false;
        for (terminal, args) in terminal_commands.iter() {
            let mut cmd = std::process::Command::new(terminal);
            cmd.args(args).arg(program_path).arg(file_path);

            if cmd.spawn().is_ok() {
                terminal_launched = true;
                break;
            }
        }

        if !terminal_launched {
            return Err(FileFantasticError::NoTerminalFound);
        }
    }
    #[cfg(target_os = "redox")]
    {
        // Redox OS - try Orbital's terminal emulator
        let mut terminal_launched = false;

        // Try orbital terminal first
        let mut cmd = std::process::Command::new("orbital");
        cmd.arg("terminal")
            .arg("-e")
            .arg(program_path)
            .arg(file_path);

        if cmd.spawn().is_ok() {
            terminal_launched = true;
        } else {
            // Fallback: try to launch the program directly
            // Some Redox programs might work without terminal wrapper
            if std::process::Command::new(program_path)
                .arg(file_path)
                .spawn()
                .is_ok()
            {
                terminal_launched = true;
            }
        }

        if !terminal_launched {
            return Err(FileFantasticError::NoTerminalFound);
        }
    }
    #[cfg(target_os = "windows")]
    {
        // Build command string for Windows cmd.exe
        let command_string = format!(
            "{} {}",
            program_path.to_string_lossy(),
            file_path.to_string_lossy()
        );

        std::process::Command::new("cmd")
            .args(["/C", "start", "cmd", "/C"])
            .arg(format!("{} && pause", command_string))
            .spawn()
            .map_err(
                |_| FileFantasticError::NoTerminalFound,
                // Use NoTerminalFound instead
                // .map_err(|e| {
                //     eprintln!("Failed to open cmd.exe for partner program: {}", e);
                //     FileFantasticError::EditorLaunchFailed(extract_program_display_name(program_path))
                // }
            )?;
    }
    #[cfg(not(any(
        target_os = "macos",
        target_os = "linux",
        target_os = "windows",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "redox",
    )))]
    {
        // Fallback for unsupported platforms - try to launch directly
        std::process::Command::new(program_path)
            .arg(file_path)
            .spawn()
            .map_err(|e| {
                eprintln!(
                    "Failed to launch partner program on unsupported platform: {}",
                    e
                );
                FileFantasticError::EditorLaunchFailed(format!(
                    "Unsupported platform. Direct launch failed for {}",
                    extract_program_display_name(program_path)
                ))
            })?;
    }

    println!(
        "Launched partner program: {} with {}",
        extract_program_display_name(program_path),
        file_path.file_name().unwrap_or_default().to_string_lossy()
    );

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
        return Err(FileFantasticError::EditorLaunchFailed(format!(
            "Editor '{}' is not available on this system",
            editor
        )));
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
        Err(FileFantasticError::EditorLaunchFailed(format!(
            "Editor '{}' exited with non-zero status",
            editor
        )))
    }
}

/// Opens a file in a tmux split pane
///
/// # Arguments
/// * `editor` - The editor command to use
/// * `file_path` - Path to the file to open
/// * `split_type` - Either "-vsplit" for vertical or "-hsplit" for horizontal split
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
        return Err(FileFantasticError::EditorLaunchFailed(format!(
            "Editor '{}' is not available on this system",
            editor
        )));
    }

    // Explicitly translate our split type flags to tmux split-window arguments
    let (tmux_split_arg, split_direction) = match split_type {
        "vsplit" | "-vsplit" | "--vertical-split-tmux" => ("-v", "vertical"),
        "hsplit" | "-hsplit" | "--horizontal-split-tmux" => ("-h", "horizontal"),
        _ => {
            return Err(FileFantasticError::EditorLaunchFailed(format!(
                "Invalid split type '{}'. Expected '-vsplit' or '-hsplit'",
                split_type
            )));
        }
    };

    // Build the command to run in the new split
    let editor_command = format!("{} {}", editor, file_path.to_string_lossy());

    // Create the tmux split with the editor
    let output = std::process::Command::new("tmux")
        .args(["split-window", tmux_split_arg, &editor_command])
        .output()
        .map_err(|e| {
            eprintln!("Failed to create tmux {} split: {}", split_direction, e);
            FileFantasticError::Io(e)
        })?;

    if output.status.success() {
        println!("Opened {} in tmux {} split", editor, split_direction);
        Ok(())
    } else {
        Err(FileFantasticError::EditorLaunchFailed(format!(
            "Failed to create tmux {} split: {}",
            split_direction,
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

/// Parses special flags from user input for headless mode, tmux splits, and CSV analysis
///
/// This function identifies special flags in the input string and normalizes them to their
/// verbose forms. Terse flags are expanded to their full verbose equivalents for consistency
/// and clarity in downstream processing.
///
/// # Arguments
/// * `input` - The user input string containing an editor command and optional flags
///
/// # Returns
/// * `Option<(String, String)>` - Some((editor, flags)) if special flags are found, None otherwise
///   - `editor`: The editor command without the special flags
///   - `flags`: Space-separated string of normalized (verbose) flags
///
/// # Supported Flags
/// * `-h` or `--headless` - Open in current terminal (normalized to `--headless`)
/// * `-vsplit` or `--vertical-split-tmux` - Open in vertical tmux split (normalized to `--vertical-split-tmux`)
/// * `-hsplit` or `--horizontal-split-tmux` - Open in horizontal tmux split (normalized to `--horizontal-split-tmux`)
/// * `-rc` or `--rows-and-columns` - Analyze CSV file before opening (normalized to `--rows-and-columns`)
///
/// # Examples
/// * "vim -h" -> Some(("vim", "--headless"))
/// * "nano -rc" -> Some(("nano", "--rows-and-columns"))
/// * "code -h -rc" -> Some(("code", "--headless --rows-and-columns"))
/// * "vim --headless" -> Some(("vim", "--headless"))
/// * "emacs" -> None
fn parse_special_flags(input: &str) -> Option<(String, String)> {
    // Define the mapping from terse flags to verbose flags
    // Using a slice of tuples for easy lookup and conversion
    let flag_mappings: &[(&str, &str)] = &[
        ("-h", "--headless"),
        ("--headless", "--headless"),
        ("-vsplit", "--vertical-split-tmux"),
        ("vsplit", "--vertical-split-tmux"),
        ("--vertical-split-tmux", "--vertical-split-tmux"),
        ("hsplit", "--horizontal-split-tmux"),
        ("-hsplit", "--horizontal-split-tmux"),
        ("--horizontal-split-tmux", "--horizontal-split-tmux"),
        ("-rc", "--rows-and-columns"),
        ("--rows-and-columns", "--rows-and-columns"),
    ];

    // Create a set of all valid flags (both terse and verbose) for quick lookup
    let valid_flags: Vec<&str> = flag_mappings.iter().map(|(flag, _)| *flag).collect();

    // Parse the input into individual whitespace-separated parts
    let parts: Vec<&str> = input.split_whitespace().collect();

    // Separate editor parts from flag parts and normalize flags
    let mut found_normalized_flags = Vec::new();
    let mut editor_parts = Vec::new();

    for part in &parts {
        // Check if this part is a recognized flag
        if valid_flags.contains(&part.as_ref()) {
            // Find the corresponding verbose flag
            let verbose_flag = flag_mappings
                .iter()
                .find(|(terse, _)| terse == part)
                .map(|(_, verbose)| *verbose);

            // Add the verbose flag to our collection if found
            if let Some(verbose) = verbose_flag {
                found_normalized_flags.push(verbose.to_string());
            }
        } else {
            // This part is not a flag, so it's part of the editor command
            editor_parts.push(part.to_string());
        }
    }

    // Return the result based on whether we found any special flags
    if !found_normalized_flags.is_empty() {
        // Construct the editor string from non-flag parts
        let editor = editor_parts.join(" ");
        // Construct the flags string from normalized flags
        let flags_str = found_normalized_flags.join(" ");
        Some((editor, flags_str))
    } else {
        // No special flags were found
        None
    }
}

#[cfg(test)]
mod flags_tests {
    use super::*;

    #[test]
    fn test_terse_flag_normalization() {
        // Test that terse flags are converted to verbose forms
        assert_eq!(
            parse_special_flags("vim -h"),
            Some(("vim".to_string(), "--headless".to_string()))
        );

        assert_eq!(
            parse_special_flags("nano -rc"),
            Some(("nano".to_string(), "--rows-and-columns".to_string()))
        );

        assert_eq!(
            parse_special_flags("emacs -vsplit"),
            Some(("emacs".to_string(), "--vertical-split-tmux".to_string()))
        );

        assert_eq!(
            parse_special_flags("code -hsplit"),
            Some(("code".to_string(), "--horizontal-split-tmux".to_string()))
        );
    }

    #[test]
    fn test_verbose_flags_unchanged() {
        // Test that verbose flags remain unchanged
        assert_eq!(
            parse_special_flags("vim --headless"),
            Some(("vim".to_string(), "--headless".to_string()))
        );

        assert_eq!(
            parse_special_flags("nano --rows-and-columns"),
            Some(("nano".to_string(), "--rows-and-columns".to_string()))
        );

        assert_eq!(
            parse_special_flags("emacs --vertical-split-tmux"),
            Some(("emacs".to_string(), "--vertical-split-tmux".to_string()))
        );

        assert_eq!(
            parse_special_flags("code --horizontal-split-tmux"),
            Some(("code".to_string(), "--horizontal-split-tmux".to_string()))
        );
    }

    #[test]
    fn test_multiple_flags() {
        // Test combinations of multiple flags
        assert_eq!(
            parse_special_flags("vim -h -rc"),
            Some((
                "vim".to_string(),
                "--headless --rows-and-columns".to_string()
            ))
        );

        assert_eq!(
            parse_special_flags("nano --headless -vsplit"),
            Some((
                "nano".to_string(),
                "--headless --vertical-split-tmux".to_string()
            ))
        );
    }

    #[test]
    fn test_no_special_flags() {
        // Test that None is returned when no special flags are present
        assert_eq!(parse_special_flags("vim"), None);
        assert_eq!(parse_special_flags("nano file.txt"), None);
        assert_eq!(parse_special_flags("code --some-other-flag"), None);
    }

    #[test]
    fn test_complex_editor_command() {
        // Test with editor commands that have multiple parts
        assert_eq!(
            parse_special_flags("/usr/bin/vim -h"),
            Some(("/usr/bin/vim".to_string(), "--headless".to_string()))
        );

        assert_eq!(
            parse_special_flags("env TERM=xterm vim -rc"),
            Some((
                "env TERM=xterm vim".to_string(),
                "--rows-and-columns".to_string()
            ))
        );
    }

    #[test]
    fn test_empty_input() {
        // Test edge case with empty input
        assert_eq!(parse_special_flags(""), None);
    }

    #[test]
    fn test_whitespace_only() {
        // Test edge case with only whitespace
        assert_eq!(parse_special_flags("   "), None);
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
            "Invalid file path - contains non-UTF8 characters",
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
            println!(
                "{}You will open the analysis file. Press Enter to continue...{}",
                YELLOW, RESET
            );
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
                error_msg,
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
/// Also supports -rc --rows-and-columns .csv analysis
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
fn open_file(file_path: &PathBuf, lines_editor_session_path: &PathBuf) -> Result<()> {
    /*
    The user input format/sytax should be as regular/consistent as possible
    given the edge case that Lines-Editor is the default if none is specified.
    After selecting file by number:

    entering name of editor: opens in new terminal

    name of editor + -h or --headless: opens in the same terminal

    name of editor + -vsplit, -hsplit: opens in a tmux split

    Empty Enter: should open lines in a new terminal

    only "-h" or "--headless" (maybe "lines -h"): should open lines in same terminal


    */
    // Read partner programs configuration (gracefully handles all errors)
    let partner_programs = read_partner_programs_file();

    // check if suffi

    // Build the user prompt based on whether partner programs are available
    let prompt = if partner_programs.is_empty() {
        // Standard prompt when no partner programs are configured
        format!(
            "{}(Open file w/  Default: Enter | software 'name': vi --headless, gedit, firefox | tmux: nano -hsplit, hx -vsplit | .csv stats: vi -rc) {}",
            YELLOW, RESET
        )
    } else {
        // Enhanced prompt showing numbered partner program options
        let mut numbered_options = String::new();
        for (index, program_path) in partner_programs.iter().enumerate() {
            if index > 0 {
                numbered_options.push(' ');
            }
            numbered_options.push_str(&format!(
                "{}. {}",
                index + 1,
                extract_program_display_name(program_path)
            ));
        }

        format!(
            "{}Open file w/  Default: Enter | software 'name': vi --headless, gedit, firefox | tmux: -hsplit | .csv: -rc | Partner #: {}): {}",
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

    // TODO
    // ==========================================
    // Headless Default Lines-Editor
    // ==========================================
    if user_input == "-h"
        || user_input == "--headless"
        || user_input == "lines --headless"
        || user_input == "lines -h"
    {
        // =============================
        // Lines-Editor in this terminal
        // =============================
        /*
        pub fn lines_full_file_editor(
            original_file_path: Option<PathBuf>,
            starting_line: Option<usize>,
            use_this_session: Option<PathBuf>,
            state_persists: bool, // if you want to keep session files.
        ) -> Result<()> {
        */

        lines_full_file_editor(
            Some(file_path.clone()),
            None,
            Some(lines_editor_session_path.clone()),
            true,
        )?; // The ? will use From<LinesError> to convert
        return Ok(());
    }

    // ==========================================
    // === MVP: Tmux splits for lines editor ===
    // ==========================================
    // === MVP: Tmux splits for lines editor ===
    if user_input == "-vsplit" || user_input == "vsplit" {
        // Check if in tmux
        if std::env::var("TMUX").is_err() {
            println!("{}Error: -vsplit requires tmux{}", RED, RESET);
            println!("Press Enter to continue...");
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .map_err(|e| FileFantasticError::Io(e))?;
            return open_file(file_path, lines_editor_session_path);
        }

        // Get the path to the current executable
        let exe_path = std::env::current_exe().map_err(|e| FileFantasticError::Io(e))?;

        // Build the command as a single string with full binary path
        let editor_command = format!(
            "{} {} --session {}",
            exe_path.to_string_lossy(),
            file_path.to_string_lossy(),
            lines_editor_session_path.to_string_lossy()
        );

        // Create vertical split (tmux -v = vertical split = horizontal panes)
        let output = std::process::Command::new("tmux")
            .args(["split-window", "-v", &editor_command])
            .output()
            .map_err(|e| FileFantasticError::Io(e))?;

        if !output.status.success() {
            println!(
                "{}Failed to create tmux split: {}{}",
                RED,
                String::from_utf8_lossy(&output.stderr),
                RESET
            );
            println!("Press Enter to continue...");
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .map_err(|e| FileFantasticError::Io(e))?;
            return open_file(file_path, lines_editor_session_path);
        }

        return Ok(());
    }

    if user_input == "-hsplit" || user_input == "hsplit" {
        // Check if in tmux
        if std::env::var("TMUX").is_err() {
            println!("{}Error: -hsplit requires tmux{}", RED, RESET);
            println!("Press Enter to continue...");
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .map_err(|e| FileFantasticError::Io(e))?;
            return open_file(file_path, lines_editor_session_path);
        }

        // Get the path to the current executable
        let exe_path = std::env::current_exe().map_err(|e| FileFantasticError::Io(e))?;

        // Build the command as a single string with full binary path
        let editor_command = format!(
            "{} {} --session {}",
            exe_path.to_string_lossy(),
            file_path.to_string_lossy(),
            lines_editor_session_path.to_string_lossy()
        );

        // Create horizontal split (tmux -h = horizontal split = vertical panes)
        let output = std::process::Command::new("tmux")
            .args(["split-window", "-h", &editor_command])
            .output()
            .map_err(|e| FileFantasticError::Io(e))?;

        if !output.status.success() {
            println!(
                "{}Failed to create tmux split: {}{}",
                RED,
                String::from_utf8_lossy(&output.stderr),
                RESET
            );
            println!("Press Enter to continue...");
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .map_err(|e| FileFantasticError::Io(e))?;
            return open_file(file_path, lines_editor_session_path);
        }

        return Ok(());
    }

    // === Handle "lines" keyword - open in new terminal ===
    // === Handle "lines" keyword - open in new terminal ===
    if user_input == "lines" || user_input.is_empty() {
        let exe_path = std::env::current_exe().map_err(|e| FileFantasticError::Io(e))?;

        // Launch in new terminal (platform-specific)
        #[cfg(target_os = "macos")]
        {
            // macOS needs the command as a single string for Terminal.app
            let lines_command = format!(
                "{} {} --session {}; exit",
                exe_path.to_string_lossy(),
                file_path.to_string_lossy(),
                lines_editor_session_path.to_string_lossy()
            );

            std::process::Command::new("open")
                .args(["-a", "Terminal"])
                .arg(&lines_command)
                .spawn()
                .map_err(|e| FileFantasticError::EditorLaunchFailed(format!("lines: {}", e)))?;
        }

        #[cfg(target_os = "linux")]
        {
            let terminal_commands = [
                ("gnome-terminal", vec!["--"]),
                ("ptyxis", vec!["--"]),
                ("konsole", vec!["-e"]),
                ("xfce4-terminal", vec!["-e"]),
                ("terminator", vec!["-e"]),
                ("tilix", vec!["-e"]),
                ("kitty", vec!["-e"]),
                ("alacritty", vec!["-e"]),
                ("xterm", vec!["-e"]),
            ];

            let mut success = false;
            for (terminal, args) in terminal_commands.iter() {
                let mut cmd = std::process::Command::new(terminal);
                cmd.args(args)
                    .arg(&exe_path) // Separate arg: executable
                    .arg(file_path) // Separate arg: file path
                    .arg("--session") // Separate arg: flag
                    .arg(lines_editor_session_path); // Separate arg: session path

                if cmd.spawn().is_ok() {
                    success = true;
                    break;
                }
            }

            if !success {
                println!(
                    "{}No terminal available. Press Enter to continue...{}",
                    RED, RESET
                );
                let mut buf = String::new();
                io::stdin()
                    .read_line(&mut buf)
                    .map_err(|e| FileFantasticError::Io(e))?;
                return open_file(file_path, lines_editor_session_path);
            }
        }

        #[cfg(target_os = "windows")]
        {
            let lines_command = format!(
                "{} {} --session {} && pause",
                exe_path.to_string_lossy(),
                file_path.to_string_lossy(),
                lines_editor_session_path.to_string_lossy()
            );

            std::process::Command::new("cmd")
                .args(["/C", "start", "cmd", "/C"])
                .arg(&lines_command)
                .spawn()
                .map_err(|e| FileFantasticError::EditorLaunchFailed(format!("lines: {}", e)))?;
        }

        return Ok(());
    }

    // Check for special flags (headless, tmux splits, and CSV analysis)
    if let Some((editor, flags)) = parse_special_flags(user_input) {
        // User must provide an editor with these flags
        if editor.is_empty() {
            println!(
                "{}Error: You must specify an editor with the {} flag (e.g., 'vim {}'){}",
                RED, flags, flags, RESET
            );
            println!("Press Enter to continue...");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).map_err(|e| {
                eprintln!("Failed to read input: {}", e);
                FileFantasticError::Io(e)
            })?;
            return open_file(file_path, lines_editor_session_path); // Re-prompt
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
                        println!(
                            "{}Analysis failed: {}. Press Enter to try again...{}",
                            RED, e, RESET
                        );
                        let mut buf = String::new();
                        io::stdin().read_line(&mut buf).map_err(|e| {
                            eprintln!("Failed to read input: {}", e);
                            FileFantasticError::Io(e)
                        })?;
                        return open_file(file_path, lines_editor_session_path); // Re-prompt
                    }
                }
            } else {
                // Not a CSV file, silently ignore -rc flag
                println!("{}Note: -rc flag ignored (not a CSV file){}", YELLOW, RESET);
            }
        }

        // Parse the flags string into individual flags
        let flag_list: Vec<&str> = flags.split_whitespace().collect();

        // Extract the primary action flag (for headless/tmux)
        let primary_flag = if flag_list.contains(&"--headless") {
            "--headless" // Changed to avoid confusion
        } else if flag_list.contains(&"vsplit")
            || flag_list.contains(&"-vsplit")
            || flag_list.contains(&"--vertical-split-tmux")
        {
            "-vsplit"
        } else if flag_list.contains(&"hsplit")
            || flag_list.contains(&"-hsplit")
            || flag_list.contains(&"--horizontal-split-tmux")
        {
            "-hsplit"
        } else if flag_list.contains(&"-rc") || flag_list.contains(&"--rows-and-columns") {
            ""
        } else {
            ""
        };

        // Handle the different flag types
        let result = match primary_flag {
            "--headless" => {
                // Open in current terminal (headless mode)
                println!("Opening file in current terminal with {}...", editor);
                open_in_current_terminal(&editor, &file_to_open)
            }
            "-vsplit" => {
                // Open in vertical tmux split
                open_in_tmux_split(&editor, &file_to_open, "-vsplit")
            }
            "-hsplit" => {
                // Open in horizontal tmux split
                open_in_tmux_split(&editor, &file_to_open, "-hsplit")
            }
            "" if !editor.is_empty() => {
                // Just -rc flag or no special terminal flag, open normally
                //  Continue to the regular editor opening logic below
                // by falling through to the standard editor handling

                // Check if it's a GUI editor
                let gui_editors = ["code", "sublime", "subl", "gedit", "kate", "notepad++"];
                if gui_editors.contains(&editor.to_lowercase().as_str()) {
                    // Launch GUI editor directly
                    std::process::Command::new(&editor)
                        .arg(&file_to_open)
                        .spawn()
                        .map_err(|e| {
                            FileFantasticError::EditorLaunchFailed(format!("{}: {}", editor, e))
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
                                .arg(format!(
                                    "{} {}; exit",
                                    editor,
                                    file_to_open.to_string_lossy()
                                ))
                                .spawn()
                                .map_err(|e| {
                                    FileFantasticError::EditorLaunchFailed(format!(
                                        "{}: {}",
                                        editor, e
                                    ))
                                })
                                .map(|_| ())
                        }
                        #[cfg(target_os = "linux")]
                        {
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
                                    "No terminal emulator found".to_string(),
                                ))
                            }
                        }
                        #[cfg(target_os = "android")]
                        {
                            // Check if we're in Termux environment
                            if std::env::var("TERMUX_VERSION").is_ok()
                                || std::path::Path::new("/data/data/com.termux").exists()
                            {
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
                                        FileFantasticError::EditorLaunchFailed(format!(
                                            "Termux/Android launch failed for {}: {}",
                                            editor, e
                                        ))
                                    })
                                    .map(|_| ())
                            } else {
                                // Non-Termux Android environment
                                // Try to launch editor directly
                                std::process::Command::new(&editor)
                                    .arg(&file_to_open)
                                    .spawn()
                                    .map_err(|e| {
                                        FileFantasticError::EditorLaunchFailed(format!(
                                            "Android launch failed for {}: {}",
                                            editor, e
                                        ))
                                    })
                                    .map(|_| ())
                            }
                        }
                        #[cfg(any(
                            target_os = "freebsd",
                            target_os = "openbsd",
                            target_os = "netbsd",
                            target_os = "dragonfly"
                        ))]
                        {
                            // BSD systems typically use similar terminal emulators to Linux
                            let terminal_commands = [
                                ("xterm", vec!["-e", &editor]),
                                ("rxvt", vec!["-e", &editor]),
                                ("urxvt", vec!["-e", &editor]),
                                ("konsole", vec!["--e", &editor]),
                                ("gnome-terminal", vec!["--", &editor]),
                                ("xfce4-terminal", vec!["--command", &editor]),
                                ("kitty", vec!["-e", &editor]),
                                ("alacritty", vec!["-e", &editor]),
                                ("sakura", vec!["-e", &editor]),
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
                                    "No terminal emulator found on BSD system".to_string(),
                                ))
                            }
                        }
                        #[cfg(target_os = "redox")]
                        {
                            // Redox OS uses orbital as its windowing system
                            // and has its own terminal emulator
                            let terminal_result = std::process::Command::new("orbital")
                                .arg("terminal")
                                .arg("-e")
                                .arg(&editor)
                                .arg(&file_to_open)
                                .spawn();

                            if terminal_result.is_err() {
                                // Fallback: try to launch the editor directly
                                // Some Redox programs might work without a terminal wrapper
                                std::process::Command::new(&editor)
                                    .arg(&file_to_open)
                                    .spawn()
                                    .map_err(|e| {
                                        FileFantasticError::EditorLaunchFailed(format!(
                                            "Failed to launch {} on Redox OS: {}",
                                            editor, e
                                        ))
                                    })
                                    .map(|_| ())
                            } else {
                                terminal_result
                                    .map_err(|e| {
                                        FileFantasticError::EditorLaunchFailed(format!(
                                            "Failed to open terminal on Redox OS for {}: {}",
                                            editor, e
                                        ))
                                    })
                                    .map(|_| ())
                            }
                        }
                        #[cfg(target_os = "windows")]
                        {
                            std::process::Command::new("cmd")
                                .args(["/C", "start", "cmd", "/C"])
                                .arg(format!(
                                    "{} {} && pause",
                                    editor,
                                    file_to_open.to_string_lossy()
                                ))
                                .spawn()
                                .map_err(|e| {
                                    FileFantasticError::EditorLaunchFailed(format!(
                                        "{}: {}",
                                        editor, e
                                    ))
                                })
                                .map(|_| ())
                        }
                        // Fallback for any other platform not covered by cfg directives
                        #[cfg(not(any(
                            target_os = "macos",
                            target_os = "linux",
                            target_os = "android",
                            target_os = "windows",
                            target_os = "freebsd",
                            target_os = "openbsd",
                            target_os = "netbsd",
                            target_os = "dragonfly",
                            target_os = "redox",
                        )))]
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
            }
            _ => {
                // This shouldn't happen
                Err(FileFantasticError::EditorLaunchFailed(format!(
                    "Unknown flag: {}",
                    primary_flag
                )))
            }
        };

        // Handle any errors from the special mode operations
        match result {
            Ok(_) => return Ok(()),
            Err(e) => {
                // Display error and re-prompt
                println!("{}Error: {}{}", RED, e, RESET);
                println!("Press Enter to continue...");
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).map_err(|e| {
                    eprintln!("Failed to read input: {}", e);
                    FileFantasticError::Io(e)
                })?;
                return open_file(file_path, &lines_editor_session_path); // Re-prompt for new selection
            }
        }
    }

    // // ==================
    // // Handle empty input - use system default (existing functionality)
    // // ==================
    // if user_input.is_empty() {
    //     #[cfg(target_os = "macos")]
    //     {
    //         std::process::Command::new("open")
    //             .arg(file_path)
    //             .spawn()
    //             .map_err(|e| {
    //                 eprintln!("Failed to open file with default application: {}", e);
    //                 FileFantasticError::Io(e)
    //             })?;
    //     }
    //     #[cfg(target_os = "linux")]
    //     {
    //         std::process::Command::new("xdg-open")
    //             .arg(file_path)
    //             .spawn()
    //             .map_err(|e| {
    //                 eprintln!("Failed to open file with xdg-open: {}", e);
    //                 FileFantasticError::Io(e)
    //             })?;
    //     }
    //     #[cfg(any(
    //         target_os = "freebsd",
    //         target_os = "openbsd",
    //         target_os = "netbsd",
    //         target_os = "dragonfly",
    //     ))]
    //     {
    //         // BSD systems typically use xdg-open if available, otherwise try direct opening
    //         let result = std::process::Command::new("xdg-open")
    //             .arg(file_path)
    //             .spawn();

    //         if result.is_err() {
    //             // Fallback: try to open with a common editor
    //             std::process::Command::new("vi")
    //                 .arg(file_path)
    //                 .spawn()
    //                 .map_err(|e| {
    //                     eprintln!("Failed to open file on BSD system: {}", e);
    //                     FileFantasticError::Io(e)
    //                 })?;
    //         } else {
    //             result.map_err(|e| {
    //                 eprintln!("Failed to open file with xdg-open on BSD: {}", e);
    //                 FileFantasticError::Io(e)
    //             })?;
    //         }
    //     }
    //     #[cfg(target_os = "redox")]
    //     {
    //         // Redox OS uses 'open' command for default file associations
    //         // Similar to macOS but it's a different implementation
    //         std::process::Command::new("open")
    //             .arg(file_path)
    //             .spawn()
    //             .or_else(|_| {
    //                 // Fallback: try 'orbital-open' if available
    //                 std::process::Command::new("orbital-open")
    //                     .arg(file_path)
    //                     .spawn()
    //             })
    //             .map_err(|e| {
    //                 eprintln!(
    //                     "Failed to open file with default application on Redox OS: {}",
    //                     e
    //                 );
    //                 FileFantasticError::Io(e)
    //             })?;
    //     }
    //     #[cfg(target_os = "windows")]
    //     {
    //         std::process::Command::new("cmd")
    //             .args(["/C", "start", ""])
    //             .arg(file_path)
    //             .spawn()
    //             .map_err(|e| {
    //                 eprintln!("Failed to open file with default application: {}", e);
    //                 FileFantasticError::Io(e)
    //             })?;
    //     }
    //     #[cfg(not(any(
    //         target_os = "macos",
    //         target_os = "linux",
    //         target_os = "android",
    //         target_os = "windows",
    //         target_os = "freebsd",
    //         target_os = "openbsd",
    //         target_os = "netbsd",
    //         target_os = "dragonfly",
    //         target_os = "redox",
    //     )))]
    //     {
    //         // Fallback for unsupported platforms
    //         eprintln!("Platform not supported for default file opening");
    //         return Err(FileFantasticError::EditorLaunchFailed(
    //             "Platform not supported for default file opening".to_string(),
    //         ));
    //     }
    //     return Ok(());
    // }

    // ==================
    // Handle empty input - use custom editor
    // ==================
    /*
    pub fn lines_full_file_editor(
        original_file_path: Option<PathBuf>,
        starting_line: Option<usize>,
        use_this_session: Option<PathBuf>,
        state_persists: bool, // if you want to keep session files.
    ) -> Result<()> {
    */
    if user_input.is_empty() {
        lines_full_file_editor(
            Some(file_path.clone()),
            None,
            Some(lines_editor_session_path.clone()),
            true,
        )?; // The ? will use From<LinesError> to convert
        return Ok(());
    }

    // Try to parse input as a number for partner program selection
    if let Ok(program_number) = user_input.parse::<usize>() {
        if program_number > 0 && program_number <= partner_programs.len() {
            let selected_program = &partner_programs[program_number - 1];
            println!(
                "Launching partner program: {}",
                extract_program_display_name(selected_program)
            );

            // Launch partner program in terminal with proper error handling
            match launch_partner_program_in_terminal(selected_program, file_path) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    // Follow File Fantastic's error handling pattern
                    println!(
                        "Error launching partner program: {}. \nPress Enter to continue",
                        e
                    );
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).map_err(|e| {
                        eprintln!("Failed to read input: {}", e);
                        FileFantasticError::Io(e)
                    })?;

                    // After user acknowledgment, fall back to asking again
                    println!("Falling back to system default...");
                    return open_file(file_path, &lines_editor_session_path); // Recursive call for new selection
                }
            }
        } else if !partner_programs.is_empty() {
            // Invalid partner program number
            println!(
                "Invalid partner program number. Valid range: 1-{}. \nPress Enter to continue",
                partner_programs.len()
            );
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).map_err(|e| {
                eprintln!("Failed to read input: {}", e);
                FileFantasticError::Io(e)
            })?;
            return open_file(file_path, &lines_editor_session_path); // Ask again
        }
        // If no partner programs configured, fall through to treat as program name
    }

    // Handle traditional program name input (existing functionality preserved)
    let editor = user_input;

    // List of known GUI editors that shouldn't need a terminal (existing logic)
    let gui_editors = ["code", "sublime", "subl", "gedit", "kate", "notepad++"];

    if gui_editors.contains(&editor.to_lowercase().as_str()) {
        // Launch GUI editors directly (existing functionality)
        match std::process::Command::new(editor).arg(file_path).spawn() {
            Ok(_) => return Ok(()),
            Err(e) => {
                // Follow existing error handling pattern
                eprintln!("Error launching {}: {}", editor, e);
                let error = FileFantasticError::EditorLaunchFailed(editor.to_string());
                println!(
                    "Falling back to system default due to: {}. \nPress Enter to continue",
                    error
                );
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).map_err(|e| {
                    eprintln!("Failed to read input: {}", e);
                    FileFantasticError::Io(e)
                })?;
                return open_file(file_path, &lines_editor_session_path); // Ask again
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
                println!(
                    "No terminal available. Falling back to system default... \nPress Enter to continue"
                );
                let error = FileFantasticError::EditorLaunchFailed(editor.to_string());
                eprintln!("Error: {}", error);
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).map_err(|e| {
                    eprintln!("Failed to read input: {}", e);
                    FileFantasticError::Io(e)
                })?;
                return open_file(file_path, &lines_editor_session_path); // Ask again
            }
        }
        #[cfg(target_os = "android")]
        {
            // Note: this compiles but does nothing.
            // Android/Termux environment - vi needs to run in foreground with terminal control
            std::process::Command::new("vi")
                .arg(file_path)
                .status() // <-- Blocks and waits, gives terminal control to vi
                .map_err(|e| {
                    eprintln!("Failed to open file with vi on Android: {}", e);
                    FileFantasticError::Io(e)
                })?;
        }
        #[cfg(any(
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "dragonfly"
        ))]
        {
            // BSD systems - try common terminal emulators
            let terminal_commands = [
                ("xterm", vec!["-e", editor]), // Most common, usually in base X11
                ("rxvt", vec!["-e", editor]),  // Lightweight alternative
                ("urxvt", vec!["-e", editor]), // rxvt-unicode
                ("rxvt-unicode", vec!["-e", editor]), // Alternative name
                ("konsole", vec!["--e", editor]), // KDE terminal
                ("gnome-terminal", vec!["--", editor]), // GNOME terminal
                ("mate-terminal", vec!["--", editor]), // MATE desktop
                ("xfce4-terminal", vec!["--command", editor]), // Xfce
                ("sakura", vec!["-e", editor]), // Lightweight GTK terminal
                ("st", vec!["-e", editor]),    // suckless terminal
                ("kitty", vec![editor]),       // Modern GPU-accelerated terminal
                ("alacritty", vec!["-e", editor]), // Another modern terminal
                ("terminology", vec!["-e", editor]), // Enlightenment terminal
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
                // BSD-specific fallback: try to run editor directly in current terminal
                // as a last resort before showing error
                let direct_result = std::process::Command::new(editor).arg(file_path).spawn();

                if direct_result.is_err() {
                    // Follow existing error handling pattern
                    println!(
                        "No terminal available on BSD system. Falling back to system default... \nPress Enter to continue"
                    );
                    let error = FileFantasticError::EditorLaunchFailed(editor.to_string());
                    eprintln!("Error: {}", error);
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).map_err(|e| {
                        eprintln!("Failed to read input: {}", e);
                        FileFantasticError::Io(e)
                    })?;
                    return open_file(file_path, &lines_editor_session_path); // Ask again
                }
            }
        }
        #[cfg(target_os = "redox")]
        {
            // Redox OS: try to open editor in Orbital's terminal
            let result = std::process::Command::new("orbital")
                .arg("terminal")
                .arg("-e")
                .arg(format!("{} {}", editor, file_path.to_string_lossy()))
                .spawn();

            if result.is_err() {
                // Fallback: try to run the editor directly
                // Some Redox editors might not need a terminal wrapper
                std::process::Command::new(editor)
                    .arg(file_path)
                    .spawn()
                    .map_err(|e| {
                        eprintln!("Failed to open editor on Redox OS: {}", e);
                        FileFantasticError::EditorLaunchFailed(editor.to_string())
                    })?;
            } else {
                result.map_err(|e| {
                    eprintln!(
                        "Failed to open Orbital terminal for editor on Redox OS: {}",
                        e
                    );
                    FileFantasticError::EditorLaunchFailed(editor.to_string())
                })?;
            }
        }
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "start", "cmd", "/C"])
                .arg(format!(
                    "{} {} && pause",
                    editor,
                    file_path.to_string_lossy()
                ))
                .spawn()
                .map_err(|e| {
                    eprintln!("Failed to open cmd.exe for editor: {}", e);
                    FileFantasticError::EditorLaunchFailed(editor.to_string())
                })?;
        }
        #[cfg(not(any(
            target_os = "macos",
            target_os = "linux",
            target_os = "android",
            target_os = "windows",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "dragonfly",
            target_os = "redox",
        )))]
        {
            // Fallback for unsupported platforms - try direct execution
            std::process::Command::new(editor)
                .arg(file_path)
                .spawn()
                .map_err(|e| {
                    eprintln!("Failed to open editor on unsupported platform: {}", e);
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
fn handle_file_open(path: &PathBuf, lines_editor_session_path: &PathBuf) -> Result<()> {
    match open_file(path, lines_editor_session_path) {
        Ok(_) => {
            println!("Opened File: For Tmux toggle windows with: ctrl b -> o ");
            println!(" Tip: Add partner programs (for processing files) to this config file");
            println!(" /path/to/executable/");
            println!(" └── ff_data/                            <- Created if doesn't exist");
            println!("     └── absolute_paths_to_local_partner_fileopening_executibles.txt");
            println!("\nPress Enter to continue");
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
/// Vanilla home made pair compare levenshtein_distance with error handling
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
/// * `Ok(usize)` - The edit distance between the strings (lower = more similar)
/// * `Err(FileFantasticError)` - If either string exceeds the maximum length limit
///
/// # Error Handling
/// Returns an error if either string exceeds 1000 characters to prevent:
/// - Excessive memory allocation
/// - Performance degradation
/// - Potential integer overflow in extreme cases
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
/// - Maximum string length of 1000 characters enforced for safety
/// - Efficient for short strings like filenames, but may not scale well
///   for very long strings
///
/// # Usage Context
/// Used in the `fuzzy_search` method to find files matching a partial query,
/// allowing for approximate/inexact matches when users don't know the exact filename.
///
/// # Examples
/// ```
/// assert_eq!(levenshtein_distance("kitten", "sitting")?, 3);
/// assert_eq!(levenshtein_distance("rust", "dust")?, 1);
/// assert_eq!(levenshtein_distance("", "test")?, 4);
/// ```
fn levenshtein_distance(s: &str, t: &str) -> Result<usize> {
    // Define maximum allowed string length for safety
    const MAX_STRING_LENGTH: usize = 1000;

    // Convert strings to vectors of chars for easier indexing
    // Do this FIRST to get correct character counts
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // Get the CHARACTER lengths, not byte lengths
    let m = s_chars.len();
    let n = t_chars.len();

    // Check length constraints before expensive operations
    if m > MAX_STRING_LENGTH {
        return Err(FileFantasticError::LevenshteinError {
            filename: s.to_string(),
            length: m,
        });
    }

    if n > MAX_STRING_LENGTH {
        return Err(FileFantasticError::LevenshteinError {
            filename: t.to_string(),
            length: n,
        });
    }

    // Also check the combined computational complexity
    // This prevents issues with two 1000-char strings creating a million operations
    const MAX_COMPUTATION_SIZE: usize = 500_000; // 500k operations max
    if m.saturating_mul(n) > MAX_COMPUTATION_SIZE {
        // Return error for the longer string
        if m >= n {
            return Err(FileFantasticError::LevenshteinError {
                filename: s.to_string(),
                length: m,
            });
        } else {
            return Err(FileFantasticError::LevenshteinError {
                filename: t.to_string(),
                length: n,
            });
        }
    }

    // Handle empty string cases
    if m == 0 {
        return Ok(n);
    }
    if n == 0 {
        return Ok(m);
    }

    // Create two work vectors - these allocations could theoretically fail
    // but with our length checks above, they should be safe
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

            v1[j + 1] = deletion_cost.min(insertion_cost).min(substitution_cost);
        }

        // Swap vectors for next iteration
        std::mem::swap(&mut v0, &mut v1);
    }

    // Return final distance
    Ok(v0[n])
}

#[cfg(test)]
mod levenshtein_tests {
    use super::*;

    #[test]
    fn test_levenshtein_normal_strings() {
        // Normal cases should work as before
        assert_eq!(levenshtein_distance("kitten", "sitting").unwrap(), 3);
        assert_eq!(levenshtein_distance("rust", "dust").unwrap(), 1);
        assert_eq!(levenshtein_distance("", "test").unwrap(), 4);
        assert_eq!(levenshtein_distance("same", "same").unwrap(), 0);
    }

    #[test]
    fn test_levenshtein_long_string_error() {
        // Create a string longer than 1000 characters
        let long_string: String = "a".repeat(1001);
        let normal_string = "test";

        // Should return an error for the long string
        let result = levenshtein_distance(&long_string, normal_string);
        assert!(result.is_err());

        // Check the error contains expected information
        if let Err(FileFantasticError::LevenshteinError { length, .. }) = result {
            assert_eq!(length, 1001);
        } else {
            panic!("Expected LevenshteinError");
        }
    }

    #[test]
    fn test_levenshtein_both_strings_long() {
        // Both strings exceed limit
        let long_string1: String = "a".repeat(1001);
        let long_string2: String = "b".repeat(1002);

        // Should return an error
        let result = levenshtein_distance(&long_string1, &long_string2);
        assert!(result.is_err());
    }

    #[test]
    fn test_levenshtein_edge_case_exactly_max() {
        // Strings exactly at the limit should work if they don't exceed computational limit
        // For 500,000 limit: sqrt(500,000) ≈ 707
        // So two 700-char strings should work (700*700 = 490,000 < 500,000)
        let max_string1: String = "a".repeat(700);
        let max_string2: String = "b".repeat(700);

        // Should succeed
        let result = levenshtein_distance(&max_string1, &max_string2);
        assert!(result.is_ok());
    }

    #[test]
    fn no_test_levenshtein_edge_case_exactly_max() {
        // Strings exactly at the limit should work if they don't exceed computational limit
        // For 500,000 limit: sqrt(500,000) ≈ 707
        // So two 700-char strings should work (700*700 = 490,000 < 500,000)
        let max_string1: String = "a".repeat(700);
        let max_string2: String = "b".repeat(700);

        // Should succeed
        let result = levenshtein_distance(&max_string1, &max_string2);
        assert!(result.is_ok());

        // But two 1000-char strings should fail with 500k limit
        let too_long1: String = "a".repeat(1000);
        let too_long2: String = "b".repeat(1000);
        let result = levenshtein_distance(&too_long1, &too_long2);
        assert!(result.is_err());
        // This should fail if MAX_COMPUTATION_SIZE is 500,000
        // Or pass if MAX_COMPUTATION_SIZE is 1,000,000
        // Adjust based on your chosen limit
    }

    #[test]
    fn test_levenshtein_computational_limit() {
        // Test the computational limit check
        // Two 800-char strings = 640,000 operations > 500,000 limit
        let string1: String = "a".repeat(800);
        let string2: String = "b".repeat(800);

        let result = levenshtein_distance(&string1, &string2);
        assert!(result.is_err());
    }
}

// =================================================
// Data Structures for Command-Line Arguments
// =================================================

/// Optional command-line flags for Lines editor configuration
///
/// # Project Context
/// File Fantastic can optionally enhance Lines editor launch with:
/// - Session persistence: Reuse previous editor state from session directory
/// - Line jumping: Open file at specific line number
///
/// These are optional enhancements - Lines editor works without them
///
/// # Fields
/// * `session_path` - Directory containing Lines session files
///   - None if --session flag not provided or path invalid
///   - Validated to exist and be a directory
///
/// * `starting_line` - Line number to jump to when opening file
///   - None if --line flag not provided or value invalid
///   - Any usize value accepted (Lines validates practical range)
struct OptionalFlags {
    /// Directory path for Lines session files (--session flag)
    /// None if flag not provided or path invalid
    session_path: Option<PathBuf>,

    /// Starting line number for Lines editor (--line flag)
    /// None if flag not provided or value invalid
    starting_line: Option<usize>,
}

/// Complete parsed command-line arguments for File Fantastic
///
/// # Project Context
/// Combines all CLI inputs needed to launch FF with optional Lines editor:
/// 1. Starting location (file + directory) from positional argument
/// 2. Optional Lines editor configuration from flags
///
/// This allows flexible launch patterns:
/// - Browse only: `ff` or `ff /path/to/dir`
/// - Edit + browse: `ff /path/to/file.txt`
/// - Edit with session: `ff /path/to/file.txt --session /sess`
/// - Edit at line: `ff /path/to/file.txt --line 42`
/// - All options: `ff /path/to/file.txt --session /sess --line 42`
///
/// # Guarantees
/// - `starting_location` always present with valid directory
/// - Optional fields are None if not provided or invalid
/// - All paths are absolute
/// - All values validated before inclusion
struct CommandLineArgs {
    /// File and directory paths from positional argument
    /// Always present - contains directory even if no file
    starting_location: StartingLocation,

    /// Optional Lines editor session directory (--session)
    /// None if not specified or invalid path
    session_path: Option<PathBuf>,

    /// Optional starting line number for file editing (--line)
    /// None if not specified or invalid number
    starting_line: Option<usize>,
}

// =================================================
// Helper: String Trimming for Flag Values
// =================================================

/// Trims quote characters from start and end of string
///
/// # Project Context
/// Command-line arguments may include quotes for paths with spaces:
/// `ff --session "/path with spaces/session"`
///
/// This helper removes surrounding quotes while preserving internal quotes:
/// - `"/path/to/file"` → `/path/to/file`
/// - `'/path/to/file'` → `/path/to/file`
/// - `"path"with"quotes"` → `path"with"quotes` (keeps internal quotes)
///
/// # Arguments
/// * `value` - String slice to trim
///
/// # Returns
/// * String with leading/trailing quotes removed
///
/// # Safety
/// - Handles both single and double quotes
/// - Does not modify string if no quotes present
/// - Preserves internal quotes
fn trim_quotes(value: &str) -> String {
    value.trim_matches(|c| c == '"' || c == '\'').to_string()
}

// =================================================
// Function 1: Parse Positional Path Argument
// =================================================

/// Extracts the positional path argument from command line
///
/// # Project Context
/// File Fantastic accepts one positional argument - the path to browse/edit:
/// - Directory: Start browsing that directory
/// - File: Open in Lines, then browse parent directory
/// - No argument: Use current directory
///
/// This function ignores all flag arguments (--anything) and extracts
/// only the first non-flag argument as the path.
///
/// # Returns
/// * `Ok(Option<String>)` - The path argument if found, None if no positional arg
/// * `Err(FileFantasticError)` - Only on critical failures (should not happen)
///
/// # Behavior
/// - Scans args for first argument NOT starting with `-` or `--`
/// - Ignores all flag arguments and their values
/// - Returns None if no positional argument found
/// - Trims quotes from path if present
///
/// # Examples
/// ```rust,ignore
/// // ff /path/file.txt --session /sess
/// // Returns: Some("/path/file.txt")
///
/// // ff --session /sess /path/file.txt
/// // Returns: Some("/path/file.txt")
///
/// // ff --session /sess --line 42
/// // Returns: None (no positional arg)
///
/// // ff
/// // Returns: None (no args)
/// ```
///
/// # Error Handling
/// - Never panics
/// - Returns None for invalid scenarios (caller handles)
/// - No side effects (pure function for parsing)
fn parse_positional_path_arg() -> Result<Option<String>> {
    // =================================================
    // Debug-Assert: Validate std::env::args exists
    // =================================================
    debug_assert!(
        std::env::args().len() >= 1,
        "std::env::args() should always have at least program name"
    );

    // Get command line arguments (skip program name)
    let args: Vec<String> = std::env::args().skip(1).collect();

    #[cfg(debug_assertions)]
    eprintln!(
        "[DEBUG] Parsing positional path arg from {} arguments",
        args.len()
    );

    // Scan for first non-flag argument
    for arg in args.iter() {
        // Skip flag arguments (start with - or --)
        if arg.starts_with('-') {
            #[cfg(debug_assertions)]
            eprintln!("[DEBUG] Skipping flag: {}", arg);
            continue;
        }

        // Found positional argument
        let trimmed_path = trim_quotes(arg);

        #[cfg(debug_assertions)]
        eprintln!("[DEBUG] Found positional path argument: {}", trimmed_path);

        return Ok(Some(trimmed_path));
    }

    // No positional argument found
    #[cfg(debug_assertions)]
    eprintln!("[DEBUG] No positional path argument found");

    Ok(None)
}

// =================================================
// Function 2: Parse Optional Flags
// =================================================

/// Extracts optional flags (--session, --line) from command line
///
/// # Project Context
/// File Fantastic supports optional flags to enhance Lines editor:
/// - `--session <path>`: Directory for Lines session persistence
/// - `--line <number>`: Line number to jump to when opening file
///
/// Both flags are optional and independent. Invalid values result in
/// warnings and fallback to None (non-critical).
///
/// # Returns
/// * `Ok(OptionalFlags)` - Always succeeds, fields are None if invalid/missing
///
/// # Behavior
/// - Scans all arguments for `--session` and `--line` flags
/// - First occurrence of each flag wins (ignores duplicates)
/// - Validates session path exists and is directory
/// - Validates line number is valid usize
/// - Warns user (println) for invalid values
/// - Trims quotes from flag values
/// - Unknown flags are silently ignored
///
/// # Examples
/// ```rust,ignore
/// // ff --session /valid/path
/// // Returns: OptionalFlags { session_path: Some("/valid/path"), starting_line: None }
///
/// // ff --line 42
/// // Returns: OptionalFlags { session_path: None, starting_line: Some(42) }
///
/// // ff --session /invalid --line abc
/// // Returns: OptionalFlags { session_path: None, starting_line: None }
/// // Prints: "Warning: Session path does not exist..."
/// // Prints: "Warning: Invalid line number..."
///
/// // ff --line 10 --line 20
/// // Returns: OptionalFlags { ..., starting_line: Some(10) } (first wins)
/// ```
///
/// # Error Handling
/// - Never returns Err (always succeeds with None fallbacks)
/// - Invalid values produce user warnings (println)
/// - Debug builds show detailed diagnostics
/// - Missing flag values handled gracefully
fn parse_optional_flags() -> Result<OptionalFlags> {
    // =================================================
    // Debug-Assert: Validate std::env::args exists
    // =================================================
    debug_assert!(
        std::env::args().len() >= 1,
        "std::env::args() should always have at least program name"
    );

    // Get command line arguments (skip program name)
    let args: Vec<String> = std::env::args().skip(1).collect();

    #[cfg(debug_assertions)]
    eprintln!(
        "[DEBUG] Parsing optional flags from {} arguments",
        args.len()
    );

    let mut session_path: Option<PathBuf> = None;
    let mut starting_line: Option<usize> = None;

    // Scan arguments for flags
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];

        // =================================================
        // Parse --session flag
        // =================================================
        if arg == "--session" {
            // Check if already found (first wins)
            if session_path.is_some() {
                #[cfg(debug_assertions)]
                eprintln!("[DEBUG] Duplicate --session flag, ignoring");
                i += 2; // Skip flag and value
                continue;
            }

            // Check if value exists (next argument)
            if i + 1 < args.len() {
                let session_value = trim_quotes(&args[i + 1]);

                #[cfg(debug_assertions)]
                eprintln!("[DEBUG] Found --session flag with value: {}", session_value);

                // Validate session path
                let session_pathbuf = PathBuf::from(&session_value);

                if session_pathbuf.exists() && session_pathbuf.is_dir() {
                    // Valid session directory
                    session_path = Some(session_pathbuf);

                    #[cfg(debug_assertions)]
                    eprintln!("[DEBUG] Session path validated successfully");
                } else {
                    // Invalid session path
                    println!(
                        "Warning: Session path '{}' does not exist or is not a directory.",
                        session_value
                    );
                    println!("Starting without session.");

                    #[cfg(debug_assertions)]
                    eprintln!("[DEBUG] Session path validation failed");
                }

                i += 2; // Skip flag and value
                continue;
            } else {
                // Missing value for --session
                println!("Warning: --session flag provided without path value.");
                println!("Starting without session.");

                #[cfg(debug_assertions)]
                eprintln!("[DEBUG] --session flag missing value");

                i += 1; // Skip flag only
                continue;
            }
        }

        // =================================================
        // Parse --line flag
        // =================================================
        if arg == "--line" {
            // Check if already found (first wins)
            if starting_line.is_some() {
                #[cfg(debug_assertions)]
                eprintln!("[DEBUG] Duplicate --line flag, ignoring");
                i += 2; // Skip flag and value
                continue;
            }

            // Check if value exists (next argument)
            if i + 1 < args.len() {
                let line_value = trim_quotes(&args[i + 1]);

                #[cfg(debug_assertions)]
                eprintln!("[DEBUG] Found --line flag with value: {}", line_value);

                // Parse line number
                match line_value.parse::<usize>() {
                    Ok(line_num) => {
                        // Valid line number
                        starting_line = Some(line_num);

                        #[cfg(debug_assertions)]
                        eprintln!("[DEBUG] Line number parsed successfully: {}", line_num);
                    }
                    Err(_) => {
                        // Invalid line number
                        println!(
                            "Warning: Invalid line number '{}'. Must be a positive integer.",
                            line_value
                        );
                        println!("Starting at beginning of file.");

                        #[cfg(debug_assertions)]
                        eprintln!("[DEBUG] Failed to parse line number");
                    }
                }

                i += 2; // Skip flag and value
                continue;
            } else {
                // Missing value for --line
                println!("Warning: --line flag provided without number value.");
                println!("Starting at beginning of file.");

                #[cfg(debug_assertions)]
                eprintln!("[DEBUG] --line flag missing value");

                i += 1; // Skip flag only
                continue;
            }
        }

        // Unknown flag or positional argument - ignore and move on
        i += 1;
    }

    #[cfg(debug_assertions)]
    {
        eprintln!("[DEBUG] Optional flags parsed:");
        eprintln!("[DEBUG]   session_path: {:?}", session_path);
        eprintln!("[DEBUG]   starting_line: {:?}", starting_line);
    }

    Ok(OptionalFlags {
        session_path,
        starting_line,
    })
}

// =================================================
// Refactored: Determine Starting Location from Path
// =================================================

/// Determines starting location (file and/or directory) from a path string
///
/// # Project Context
/// File Fantastic needs to convert a user-provided path into:
/// 1. Optional file to open in Lines editor
/// 2. Required directory to browse in FF
///
/// This is the core path resolution logic, refactored to accept
/// a path parameter instead of parsing args internally.
///
/// # Arguments
/// * `path_arg` - Optional path string from command line
///   - None: Use current directory
///   - Some(path): Validate and determine type (file vs directory)
///
/// # Returns
/// * `Ok(StartingLocation)` - Success with optional file and required directory
/// * `Err(FileFantasticError)` - Only if current directory cannot be determined
///
/// # Behavior by Input
/// | Input | `file_to_open` | `directory_to_browse` | Notes |
/// |-------|----------------|----------------------|--------|
/// | None | `None` | Current directory | No argument provided |
/// | Some(dir) | `None` | That directory (absolute) | Valid directory path |
/// | Some(file) | `Some(file)` | Parent directory (absolute) | Valid file path |
/// | Some(invalid) | `None` | Current directory | Fallback with warning |
///
/// # Path Handling
/// - Converts all relative paths to absolute
/// - Validates existence before accepting
/// - Extracts parent directory for files
/// - Falls back to current directory on errors
///
/// # Error Handling
/// - Only fails if current directory cannot be determined (critical)
/// - All other issues result in fallback to current directory
/// - User-facing warnings for invalid paths
/// - Debug diagnostics for troubleshooting
///
/// # Examples
/// ```rust,ignore
/// // No path - use current directory
/// let loc = determine_starting_location_from_path(None)?;
///
/// // Directory path
/// let loc = determine_starting_location_from_path(Some("/home/user".to_string()))?;
///
/// // File path
/// let loc = determine_starting_location_from_path(Some("/home/user/file.txt".to_string()))?;
/// ```
fn determine_starting_location_from_path(path_arg: Option<String>) -> Result<StartingLocation> {
    // =================================================
    // Case 1: No Path Argument Provided
    // =================================================
    let Some(path_string) = path_arg else {
        // No argument - use current directory, no file to open
        #[cfg(debug_assertions)]
        eprintln!("[DEBUG] No path argument provided, using current directory");

        let current_dir = std::env::current_dir().map_err(|e| {
            // Critical failure - cannot determine current directory
            #[cfg(debug_assertions)]
            eprintln!("[DEBUG] Failed to get current directory: {}", e);

            FileFantasticError::Io(e)
        })?;

        return Ok(StartingLocation {
            file_to_open: None,
            directory_to_browse: current_dir,
        });
    };

    // =================================================
    // Case 2: Path Argument Provided - Parse
    // =================================================
    let path_buf = PathBuf::from(&path_string);

    #[cfg(debug_assertions)]
    eprintln!("[DEBUG] Path argument provided: {}", path_buf.display());

    // =================================================
    // Convert to Absolute Path
    // =================================================
    let absolute_path = if path_buf.is_relative() {
        #[cfg(debug_assertions)]
        eprintln!("[DEBUG] Path is relative, converting to absolute");

        // Get current directory to resolve relative path
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                // Cannot determine current directory - critical failure
                #[cfg(debug_assertions)]
                eprintln!(
                    "[DEBUG] Failed to get current directory for relative path resolution: {}",
                    e
                );

                return Err(FileFantasticError::Io(e));
            }
        };

        // Join current directory with relative path
        current_dir.join(&path_buf)
    } else {
        // Already absolute, use as-is
        path_buf
    };

    #[cfg(debug_assertions)]
    eprintln!("[DEBUG] Absolute path: {}", absolute_path.display());

    // =================================================
    // Case 3: Check if Path Exists
    // =================================================
    if !absolute_path.exists() {
        // Path doesn't exist - fall back to current directory
        println!(
            "Warning: Path '{}' does not exist. Starting in current directory.",
            absolute_path.display()
        );

        #[cfg(debug_assertions)]
        eprintln!("[DEBUG] Path does not exist, falling back to current directory");

        // Get current directory as fallback
        let fallback_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                // Cannot get current directory - critical failure
                #[cfg(debug_assertions)]
                eprintln!(
                    "[DEBUG] Failed to get current directory for fallback: {}",
                    e
                );

                return Err(FileFantasticError::Io(e));
            }
        };

        return Ok(StartingLocation {
            file_to_open: None,
            directory_to_browse: fallback_dir,
        });
    }

    // =================================================
    // Case 4: Path Exists - Determine Type
    // =================================================

    if absolute_path.is_dir() {
        // =================================================
        // Case 4a: Path is a Directory
        // =================================================
        #[cfg(debug_assertions)]
        eprintln!("[DEBUG] Path is a directory");

        return Ok(StartingLocation {
            file_to_open: None,
            directory_to_browse: absolute_path,
        });
    } else {
        // =================================================
        // Case 4b: Path is a File
        // =================================================
        #[cfg(debug_assertions)]
        eprintln!("[DEBUG] Path is a file");

        // Notify user about file opening and directory
        println!("Opening file: {}", absolute_path.display());

        // Extract parent directory for browsing
        match absolute_path.parent() {
            Some(parent) => {
                #[cfg(debug_assertions)]
                eprintln!("[DEBUG] Using parent directory: {}", parent.display());

                println!("Starting directory: {}", parent.display());

                // Return file to open and parent directory to browse
                Ok(StartingLocation {
                    file_to_open: Some(absolute_path.clone()),
                    directory_to_browse: PathBuf::from(parent),
                })
            }
            None => {
                // No parent directory (rare edge case)
                println!(
                    "Warning: Cannot determine parent directory of '{}'. Starting in current directory.",
                    absolute_path.display()
                );

                #[cfg(debug_assertions)]
                eprintln!(
                    "[DEBUG] Cannot determine parent directory, falling back to current directory"
                );

                // Get current directory as fallback
                let fallback_dir = match std::env::current_dir() {
                    Ok(dir) => dir,
                    Err(e) => {
                        // Cannot get current directory - critical failure
                        #[cfg(debug_assertions)]
                        eprintln!(
                            "[DEBUG] Failed to get current directory for fallback: {}",
                            e
                        );

                        return Err(FileFantasticError::Io(e));
                    }
                };

                // File exists but no parent - cannot use file
                Ok(StartingLocation {
                    file_to_open: None,
                    directory_to_browse: fallback_dir,
                })
            }
        }
    }
}

// =================================================
// Function 3: Orchestrator - Parse All Arguments
// =================================================

/// Parses all command-line arguments for File Fantastic
///
/// # Project Context
/// File Fantastic launches with flexible command-line options:
/// - Positional argument: Path to file or directory
/// - Optional --session flag: Lines editor session directory
/// - Optional --line flag: Starting line number for file editing
///
/// This orchestrator combines all parsing logic:
/// 1. Extract positional path argument
/// 2. Extract optional flags (--session, --line)
/// 3. Validate and combine into unified structure
///
/// # Returns
/// * `Ok(CommandLineArgs)` - Success with all parsed arguments
/// * `Err(FileFantasticError)` - Only on critical failures (cannot determine directory)
///
/// # Behavior
/// - Delegates positional parsing to `parse_positional_path_arg()`
/// - Delegates flag parsing to `parse_optional_flags()`
/// - Delegates location resolution to `determine_starting_location_from_path()`
/// - Combines results into unified `CommandLineArgs` structure
/// - Only fails if starting directory cannot be determined
///
/// # Error Handling
/// - Critical: Cannot determine starting directory → Err
/// - Non-critical: Invalid flags → None with warnings
/// - Non-critical: Invalid path → Fallback to current directory
///
/// # Examples
/// ```rust,ignore
/// // Parse all arguments
/// let args = parse_all_command_line_args()?;
///
/// // Access parsed values
/// if let Some(file) = args.starting_location.file_to_open {
///     // Open file in Lines editor
/// }
/// let dir = args.starting_location.directory_to_browse;
/// // Start FF in directory
/// ```
fn parse_all_command_line_args() -> Result<CommandLineArgs> {
    #[cfg(debug_assertions)]
    eprintln!("[DEBUG] === Parsing all command-line arguments ===");

    // =================================================
    // Step 1: Parse Positional Path Argument
    // =================================================
    let path_arg = parse_positional_path_arg()?;

    #[cfg(debug_assertions)]
    eprintln!("[DEBUG] Positional path: {:?}", path_arg);

    // =================================================
    // Step 2: Determine Starting Location
    // =================================================
    let starting_location = determine_starting_location_from_path(path_arg)?;

    #[cfg(debug_assertions)]
    {
        eprintln!("[DEBUG] Starting location determined:");
        eprintln!(
            "[DEBUG]   file_to_open: {:?}",
            starting_location.file_to_open
        );
        eprintln!(
            "[DEBUG]   directory_to_browse: {}",
            starting_location.directory_to_browse.display()
        );
    }

    // =================================================
    // Step 3: Parse Optional Flags
    // =================================================
    let optional_flags = parse_optional_flags()?;

    #[cfg(debug_assertions)]
    {
        eprintln!("[DEBUG] Optional flags parsed:");
        eprintln!("[DEBUG]   session_path: {:?}", optional_flags.session_path);
        eprintln!(
            "[DEBUG]   starting_line: {:?}",
            optional_flags.starting_line
        );
    }

    // =================================================
    // Step 4: Combine into CommandLineArgs
    // =================================================
    let cli_args = CommandLineArgs {
        starting_location,
        session_path: optional_flags.session_path,
        starting_line: optional_flags.starting_line,
    };

    #[cfg(debug_assertions)]
    eprintln!("[DEBUG] === Command-line arguments parsed successfully ===");

    Ok(cli_args)
}

/// Represents the starting location for File Fantastic application
///
/// # Project Context
/// File Fantastic (FF) has dual modes:
/// 1. File editing - Opens Lines editor for a specific file
/// 2. Directory navigation - Browses and manages files in a directory
///
/// This struct separates these concerns, allowing FF to:
/// - Open a file in Lines editor first (if user provided a file path)
/// - Then start FF navigation in the appropriate directory
///
/// # Fields
/// * `file_to_open` - Optional file path to open in Lines editor before starting FF
///   - `Some(path)` when user provides a file path as argument
///   - `None` when user provides a directory or no argument
///
/// * `directory_to_browse` - Directory where FF will start browsing
///   - Guaranteed to be a valid, absolute, accessible directory path
///   - Never None - always falls back to current directory if needed
///   - If user provides a file, this is the file's parent directory
///   - If user provides a directory, this is that directory
///   - If no args or invalid args, this is the current working directory
///
/// # Guarantees
/// - All paths are absolute (never relative)
/// - `directory_to_browse` always exists and is accessible
/// - `file_to_open` exists if `Some` (but may not be readable - caller's responsibility)
struct StartingLocation {
    /// Optional file path to open in Lines editor before starting FF
    file_to_open: Option<PathBuf>,

    /// Directory where FF will start browsing (guaranteed valid)
    directory_to_browse: PathBuf,
}

/// Checks if input is a "previous page" command
/// Supports multiple key options: j, <, [
fn is_pagination_up_command(input: &str) -> bool {
    matches!(
        input,
        "w" | "k" | "<" | "[" | "up" | "prev" | "," | "+" | "\x1b[A"
    )
}

/// Checks if input is a "next page" command
/// Supports multiple key options: k, >, ]
fn is_pagination_down_command(input: &str) -> bool {
    matches!(
        input,
        "x" | "j" | ">" | "]" | "down" | "next" | "." | "-" | "\x1b[B"
    )
}

#[cfg(test)]
mod archive_tests_2 {
    use super::*;
    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
        fs::create_dir_all(&temp_dir).unwrap_or_else(|e| {
            panic!("Failed to create temp dir at {}: {}", temp_dir.display(), e)
        });

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
        assert!(
            test_dir.exists(),
            "Test directory should exist at: {}",
            test_dir.display()
        );

        let test_file = test_dir.join("test_document.txt");
        fs::write(&test_file, b"Test content for archiving").unwrap_or_else(|e| {
            panic!(
                "Failed to create test file at {}: {}",
                test_file.display(),
                e
            )
        });

        // Verify source file was created
        assert!(
            test_file.exists(),
            "Test file should exist at: {}",
            test_file.display()
        );

        // Create archive subdirectory
        let archive_dir = test_dir.join("archive");
        fs::create_dir_all(&archive_dir).unwrap_or_else(|e| {
            panic!(
                "Failed to create archive dir at {}: {}",
                archive_dir.display(),
                e
            )
        });

        // Verify archive directory was created
        assert!(
            archive_dir.exists(),
            "Archive directory should exist at: {}",
            archive_dir.display()
        );
        assert!(archive_dir.is_dir(), "Archive path should be a directory");

        // Test: Copy file with timestamp using the archive timestamp function
        let timestamp = create_archive_timestamp(SystemTime::now());

        // Validate timestamp format before using it
        println!("Generated timestamp: {}", timestamp);
        assert!(!timestamp.is_empty(), "Timestamp should not be empty");

        let archived_file = archive_dir.join(format!("test_document_{}.txt", timestamp));

        // Debug: Print paths before copy
        println!(
            "Source file: {} (exists: {})",
            test_file.display(),
            test_file.exists()
        );
        println!(
            "Archive dir: {} (exists: {})",
            archive_dir.display(),
            archive_dir.exists()
        );
        println!("Destination: {}", archived_file.display());

        // Ensure parent directory of destination exists
        if let Some(parent) = archived_file.parent() {
            assert!(
                parent.exists(),
                "Parent directory of destination should exist: {}",
                parent.display()
            );
        }

        let result = fs::copy(&test_file, &archived_file);

        assert!(
            result.is_ok(),
            "Failed to copy file: {:?}\nFrom: {}\nTo: {}",
            result,
            test_file.display(),
            archived_file.display()
        );
        assert!(
            archived_file.exists(),
            "Archived file should exist at: {}",
            archived_file.display()
        );

        // Verify content matches
        let original_content = fs::read(&test_file).expect("Failed to read original");
        let archived_content = fs::read(&archived_file).expect("Failed to read archive");
        assert_eq!(original_content, archived_content, "Content should match");

        // Cleanup
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_archive_with_prefix() {
        // Setup: Create temporary test directory and file
        let test_dir = create_temp_test_dir();

        // Ensure test directory exists
        assert!(
            test_dir.exists(),
            "Test directory should exist at: {}",
            test_dir.display()
        );

        let test_file = test_dir.join("report.pdf");
        fs::write(&test_file, b"PDF content").unwrap_or_else(|e| {
            panic!(
                "Failed to create test file at {}: {}",
                test_file.display(),
                e
            )
        });

        // Double-check file was created
        assert!(
            test_file.exists(),
            "Test file should exist at: {}",
            test_file.display()
        );
        assert!(test_file.is_file(), "Test file should be a file");

        // Read back to verify write succeeded
        let content =
            fs::read(&test_file).unwrap_or_else(|e| panic!("Failed to read test file: {}", e));
        assert_eq!(
            content, b"PDF content",
            "File content should match what was written"
        );

        // Create archive subdirectory
        let archive_dir = test_dir.join("archive");
        fs::create_dir_all(&archive_dir).unwrap_or_else(|e| {
            panic!(
                "Failed to create archive dir at {}: {}",
                archive_dir.display(),
                e
            )
        });

        // Verify archive directory was created
        assert!(
            archive_dir.exists(),
            "Archive directory should exist at: {}",
            archive_dir.display()
        );
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
        println!(
            "Source file: {} (exists: {}, is_file: {})",
            test_file.display(),
            test_file.exists(),
            test_file.is_file()
        );
        println!(
            "Archive dir: {} (exists: {}, is_dir: {})",
            archive_dir.display(),
            archive_dir.exists(),
            archive_dir.is_dir()
        );
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
        assert!(
            result.is_ok(),
            "Failed to copy file with prefix: {:?}\nSource: {}\nDest: {}",
            result,
            test_file.display(),
            archived_file.display()
        );

        // Verify the file exists and has correct prefix
        assert!(
            archived_file.exists(),
            "Archived file should exist at: {}",
            archived_file.display()
        );

        let filename = archived_file
            .file_name()
            .expect("Should have filename")
            .to_string_lossy();
        assert!(
            filename.starts_with(prefix),
            "Filename '{}' should start with prefix '{}'",
            filename,
            prefix
        );

        // Verify the timestamp format is correct (YY_MM_DD_HH_MM_SS)
        assert!(
            filename.contains('_'),
            "Filename should contain underscores from timestamp"
        );

        // Verify content integrity
        let original_content = fs::read(&test_file).expect("Failed to read original");
        let archived_content = fs::read(&archived_file).expect("Failed to read archive");
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
        assert!(
            result_again.is_ok(),
            "Should succeed when directory already exists"
        );

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
        source
            .read_to_end(&mut contents)
            .expect("Failed to read source");

        let mut archive = File::create(&archive_file).expect("Failed to create archive");
        let filename = "data.json";
        let filename_bytes = filename.as_bytes();

        // Write archive format: [filename_length:u32][filename][file_size:u64][file_contents]
        archive
            .write_all(&(filename_bytes.len() as u32).to_le_bytes())
            .expect("Failed to write name length");
        archive
            .write_all(filename_bytes)
            .expect("Failed to write filename");
        archive
            .write_all(&(contents.len() as u64).to_le_bytes())
            .expect("Failed to write content length");
        archive
            .write_all(&contents)
            .expect("Failed to write contents");

        // Verify archive was created and has expected size
        assert!(archive_file.exists(), "Archive file should exist");
        let archive_size = fs::metadata(&archive_file)
            .expect("Failed to get metadata")
            .len();

        // Expected size: 4 (name length) + 9 (filename) + 8 (content length) + content
        let expected_size = 4 + filename_bytes.len() + 8 + contents.len();
        assert_eq!(
            archive_size as usize, expected_size,
            "Archive size should match expected"
        );

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
        assert_eq!(
            parts.len(),
            6,
            "Timestamp should have 6 parts separated by underscores"
        );

        // Verify each part is numeric and has correct length
        assert_eq!(parts[0].len(), 2, "Year should be 2 digits");
        assert_eq!(parts[1].len(), 2, "Month should be 2 digits");
        assert_eq!(parts[2].len(), 2, "Day should be 2 digits");
        assert_eq!(parts[3].len(), 2, "Hour should be 2 digits");
        assert_eq!(parts[4].len(), 2, "Minute should be 2 digits");
        assert_eq!(parts[5].len(), 2, "Second should be 2 digits");

        // Verify all parts are numeric
        for part in parts {
            assert!(
                part.chars().all(|c| c.is_ascii_digit()),
                "Timestamp part '{}' should only contain digits",
                part
            );
        }
    }

    #[test]
    fn test_timestamp_no_invalid_characters() {
        // Ensure timestamp doesn't contain characters invalid for filenames
        let timestamp = create_archive_timestamp(SystemTime::now());

        // List of characters that are problematic in filenames across platforms
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', ' '];

        for invalid_char in &invalid_chars {
            assert!(
                !timestamp.contains(*invalid_char),
                "Timestamp should not contain invalid character '{}'",
                invalid_char
            );
        }
    }
}

/*
 * Help Section
 */

/// ANSI color codes for terminal formatting
///
/// These constants provide color and style formatting for terminal output.
/// Using ANSI escape sequences for maximum compatibility.
mod ansi_colors {
    /// Reset all formatting to default
    pub const RESET: &str = "\x1b[0m";

    /// Bold text for headers
    pub const BOLD: &str = "\x1b[1m";

    /// Cyan color for commands
    pub const CYAN: &str = "\x1b[36m";

    /// Green color for examples
    pub const GREEN: &str = "\x1b[32m";

    /// Yellow color for warnings or important notes
    pub const YELLOW: &str = "\x1b[33m";

    /// Bright white for emphasis
    pub const BRIGHT_WHITE: &str = "\x1b[97m";

    /// Magenta for section numbers
    pub const MAGENTA: &str = "\x1b[35m";
}

/// Help section identifiers for menu navigation
///
/// Each variant represents a distinct help section that can be displayed
/// independently to fit within 80x24 terminal constraints.
#[derive(Debug, Clone, Copy, PartialEq)]
enum HelpSection {
    QuickStartBlurb,
    TopbarLegend,
    Navigation,
    SortingFiltering,
    SearchOptions,
    FileOperations,
    TerminalManagement,
    GetSendModeBlurb,
    ModularViewModes,
    Configuration,
}

/// Main help menu header text
///
/// Displayed at the top of the help menu selection screen
const HELP_MENU_HEADER: &str = r#"
  ╔═══════════════════════════════════════════════════════════════════════════╗
  ║ ff is a minimal file manager. It's File Fantastic! ...it's a File Fantasy.║
  ╚═════════https://github.com/lineality/ff_file_manager_minimal_rust═════════╝
                         get source code -> ff --source

  Goal: Best of both worlds between raw terminal and file manager:
        GUI features in a TUI. Switch easily between terminal and ff.
 "#;

/// Quick start and examples help section content
const HELP_SECTION_QUICK_START: &str = r#"
═══ QUICK START & EXAMPLES ═══     Press Enter to return to help menu
 USAGE in terminal:      ff [OPTIONS] [DIRECTORY]
 OPTIONS:
   -h, --help            Show this help menu
   --source              Get ff source code, Rust 'crate'

 EXAMPLES:
   ff                    Open ff in current directory
   ff /path/to/dir       Open ff in specific directory
   ff ~/Documents        Open ff in Documents folder
   ff -h                 View ff help menu
   ff --help             View ff help menu (Alternative)
   ff /path/to/file.txt --line 42    Open file (to line, optional)
   ff --session /path/to/session     User existing Lines-Session

 BASIC WORKFLOW:
   1. Launch ff in/to any directory
   2. Navigate TO files/directories with selection numbers
   3. Navigate backwards to parent directory with 'b'
   4. Sort with 'n' (name), 's' (size), 'm' (modified)
   5. Filter with 'd' (dirs only), 'f' (files only)
   6. Search by typing a search term (and hitting enter)
   7. 'q' to quit"#;

const HELP_SECTION_TOPBAR_LEGEND: &str = r#"
quit back|term|dir file|name size mod|get-send file v,y,p|str>search|enter>reset

 ═══ THE LEGEND OF TOP-BAR ═══
 quit back...........q for quit ff, b for go-back

 term................t for open a new terminal
                     vsplit for new tmux split vertical
                     hsplit for new tmux split horizontal

 dir file............d/f for view only directories/files

 name size mod.......n/s/m to sort name, size, time-modified

 get-send file v,y,p.....c,v,y,p,g for Enter Get-Send Mode
                         a is a shortcut to archive-menu

 str>search..........enter string for fuzzy directory search
                     -r for recursive directory search
                     or --recursive
                     -g for to string search text files
                     or --grep
 enter>reset.........empty enter to reset view
    Press Enter to return to help menu..."#;

/// Navigation commands help section content
const HELP_SECTION_NAVIGATION: &str = r#"
 ═══ NAVIGATION COMMANDS ═══

 BASIC NAVIGATION:
   [number]              Enter item number to open file/directory
   b                     Go back to parent directory
   q                     Quit File Fantastic
   [Enter]               Reset view and refresh

 PAGINATION:
   When viewing long lists:
   - Items are automatically paginated
   - Navigate pages with standard controls
       up/down = j/k, </>, w/x, +/- arrows keys, etc
   - Current page shown in status

 POCKET DIMENSIONS: saved locations in Get-Send Mode
   Often you want to navigate somewhere, but then come back!
   - Save current location as a "Pocket Dimension"
   - Jump between any saved 'pocket dimensions.'
   - Pocket Dimentions should retain all your filters & sorts.

 Press Enter to return to help menu..."#;

/// Sorting and filtering help section content
const HELP_SECTION_SORTING_FILTERING: &str = r#"
 ═══ SORTING & FILTERING ═══    Press Enter to return to help menu...

One thing that makes raw terminal 'ls' tricky is when there are a
lot of items and you are looking for... the most recent, or want to
see only files. With ff you can have these file-manager features
in your native terminal. Getting a reverse 'modified' 'size' 'name'
sort is easy: toggle! (And use --count-rows to view/sort data-lines!)

 SORTING COMMANDS:       [row-count: (n)ame sort, (c)ount sort)]
   n                     Sort by name (toggle ascending/descending)
   s                     Sort by size (toggle ascending/descending)
   m                     Sort by last-modified date-time (toggle asc/desc)

 FILTERING COMMANDS:     [row-count: (h) to remove headers from counts]
   d                     Show only directories
   f                     Show only files
   [Enter]               Reset filter (show all items)

 SORT ORDER (reverse the order):
   - First press: ascending order (A-Z, smallest-largest, oldest-newest)
   - Second press: descending order! Yay!"#;

/// Search options help section content
const HELP_SECTION_SEARCH: &str = r#"
 ═══ SEARCH OPTIONS ═══

 BASIC SEARCH:
   [search term]        Fuzzy search for files/directories
                        Just start typing to search current directory

 ADVANCED SEARCH:       Tip, combine flags: goodstuff -r -g
   [term] -r            Recursive search in subdirectories
   [term] --recursive   Alternative recursive search syntax
   [term] -g            Grep: search INSIDE text file contents
   [term] --grep        Alternative grep syntax
   [term] -c            Case-sensitive string search
   [term] --case-sensitive

 SEARCH BEHAVIOR:
   - Fuzzy matching: finds partial matches
   - Case-insensitive by default
   - Fuzzy Results shown with relevance scoring
        Distance = Levinshtein-Distance

 Press Enter to return to help menu... "#;

/// File operations help section content
const HELP_SECTION_FILE_OPERATIONS: &str = r#"
 ═══ FILE OPERATIONS ═══    Press  Enter to return to help menu...
 Sometimes you will use a teminal and a GUI desktop.
 Sometimes you will use a headless environment (like with ssh).
 Maybe you use tmux or tiling/window managers.

 FILE OPENING:           (after entering the file number)
   Empty Enter	         Open file with default editor
   {editor}              Open file with chosen editor
   {editor} -h           Headless! Open in current terminal
                           Alternative: -- headless
   {editor} -vsplit      Open in tmux vertical split
                           Alternative: --vertical-split-tmux
   {editor} -hsplit      Open in tmux horizontal split
                           Alternative: --horizontal-split-tmux
 CSV ANALYSIS:
    [number] -rc         Analyzes a CSV file (rows, columns, stats)
                         Opens analysis in temporary file
                           Alternative: --rows-and-columns
 EXAMPLES:
  hx                    Open file with Helix editor, in a new window
  vi -h                 Headless: Open with vi editor in same terminal
  hx -hsplit            Headless Tmux: Open with Helix in a new split
  hx -rc -vsplit        Tmux split view of rows-cols .csv "#;

/// Get-Send Mode
const HELP_SECTION_GET_SEND_MODE: &str = r#"
 ═══ GET-SEND MODE ═══   Press Enter to return to help menu...

 Moving (or copying) a file using only a raw terminal can
 be tricky. In Get-Send mode you will find various features
 to ease this process of jumping between locations, and copying
 files or directories (stacks), or making time-stamped archives, and
 tracking where you are jumping around (pocket-dimensions)!
 To enter Get-Send mode, use any common 'copy paste yank' key.
 For safety, ff only copies or archives, no delete or move.

 GET-SEND MODE ACTIVATION:
   v, c, y, p, g         Enter Get-Send Mode
   a                     Archive mode shortcut (create zip archives)

 GET-SEND MODE OPERATIONS: no deleting, only copying/archiving
   1. Add item TO stack (file or dir)
   2. Get: Save item here, FROM stack
   3. Save current location as pocket dimension
   4. Go to pocket dimension
   5. View stacks & pocket dimensions
   6. Archive file/directory 'a': zip/timestamp
   7. Clear all stacks"#;

/// Get-Send Mode
const HELP_SECTION_VIEW_MODES: &str = r#"
  ═══ ROW-COUNTS & MODULAR VIEW-MODES ═══    Press Enter to return...

  COUNT ROWS / LINES: See how many rows the files in a directory have.
  Enter mode with '--row-counts' or '--line-counts'

  Commands: (n)ame sort, (c)ount sort, (h)eader, (Enter) reset, (b/q)uit
  Sort by name or count (toggle to reverse sort), and
  'h' removes the header from the row count: Count only data rows


  MAKE YOUR OWN DIRECTORY-VIEW MODULES:
    Make your own custom views and add them to FF.
    1. Modify line-count as an example:
        NavigationAction::GoToFileLineCountMode => {
            match show_minimal_linecount_tui(&path) <-(make your own fn)

    2. Add your new NavigationAction:
    match lowercase_input.as_str() {
        "vsplit" => return Ok(NavigationAction::VsplitTmux),
        "--help" => return Ok(NavigationAction::GoToHelpMenuMode),
   -> ->"--new-stuff" => return OK(NavigationAction::NewStuff),"#;

/// Terminal management help section content
const HELP_SECTION_TERMINAL: &str = r#"
 ═══ TERMINAL & DISPLAY MANAGEMENT ═══  Press Enter to return

 Your 'current working directory,' where you go, in ff, does not
 carry over to your terminal after you exit. But you will want
 to keep working where you are in ff:
 - you can open a new terminal or split IN your current ff location.
 - Note: Run tmux before you run ff to use the tmux splits.

 TERMINAL OPERATIONS:
   t                     Open new terminal in current directory
   vsplit                Create vertical tmux split (current directory)
   hsplit                Create horizontal tmux split (current directory)

 DISPLAY RESIZING:       'N' here is whatever number you enter.
   tall+N                Increase display height by N rows
   tall-N                Decrease display height by N rows
   wide+N                Increase display width by N chars
   wide-N                Decrease display width by N chars

 When ff exits, it will tell you where you last were,
 and the ~bash line to run to go back there in a terminal.
 e.g.   To continue from this location, run:
        cd /home/oops/code/ff_file_manager_minimal_rust"#;

/// Configuration help section content
const HELP_SECTION_CONFIGURATION: &str = r#"
 ═══ PARTNER PROGRAMS CONFIGURATION ═══

 You may want to call your own applications or other applications
 that are not fully 'installed' on your system. "Partner Programs"
 allows you to tell File Fantastic where these binary-executible
 files are, wherever they are. Just list each file-path in this file,
 which FF will create:

 CONFIGURATION FILE:
   ~/.ff_data/absolute_paths_to_local_partner_fileopening_executables.txt

 FILE FORMAT:
   - One program path per line
   - Use absolute paths
   - Comments with #, and blank lines, are ignored

 EXAMPLE CONFIGURATION:
   /usr/bin/emacs
   # This is a comment
   /home/user/bin/custom-editor

 Press Enter to return to help menu... "#;

/// Display the main help menu and handle section selection
///
/// This function presents the user with a numbered menu of help sections
/// and processes their selection. It returns to the caller when the user
/// chooses to quit.
///
/// # Returns
/// * `Result<()>` - Ok on successful completion, Err on I/O or other errors
///
/// # Errors
/// - I/O errors when reading user input
/// - Terminal display errors
pub fn display_help_menu_system() -> Result<()> {
    loop {
        // Clear screen for clean display
        clear_terminal_screen()?;

        // Display header with colors
        print!("{}{}", ansi_colors::BOLD, ansi_colors::BRIGHT_WHITE);
        println!("{}", HELP_MENU_HEADER);
        print!("{}", ansi_colors::RESET);

        // Quit instructions (...learning from the vim nightmare...)
        println!(
            "  {}q.{} Type 'q' & hit Enter to quit help menu / File Fantastic",
            ansi_colors::YELLOW,
            ansi_colors::RESET
        );
        println!();

        // Display menu options
        println!(
            "{} Select a help section:{}",
            ansi_colors::CYAN,
            ansi_colors::RESET
        );

        // Menu items with colored numbers
        println!(
            "  {}1.{} Quick Start & Examples",
            ansi_colors::MAGENTA,
            ansi_colors::RESET
        );
        println!(
            "  {}2.{} Top Bar Legend Tips",
            ansi_colors::MAGENTA,
            ansi_colors::RESET
        );
        println!(
            "  {}3.{} Navigation Commands",
            ansi_colors::MAGENTA,
            ansi_colors::RESET
        );
        println!(
            "  {}4.{} Sorting & Filtering",
            ansi_colors::MAGENTA,
            ansi_colors::RESET
        );
        println!(
            "  {}5.{} Search Options",
            ansi_colors::MAGENTA,
            ansi_colors::RESET
        );
        println!(
            "  {}6.{} Go To File: Opening / Processing a File",
            ansi_colors::MAGENTA,
            ansi_colors::RESET
        );
        println!(
            "  {}7.{} Get-Send Mode (Get/Send/Move files & directories)",
            ansi_colors::MAGENTA,
            ansi_colors::RESET
        );
        println!(
            "  {}8.{} See All Files' Row Counts in a Directory (Modular View Modes)",
            ansi_colors::MAGENTA,
            ansi_colors::RESET
        );
        println!(
            "  {}9.{} Terminal & Display Management",
            ansi_colors::MAGENTA,
            ansi_colors::RESET
        );
        println!(
            "  {}10.{} 'Partner Programs' Configuration",
            ansi_colors::MAGENTA,
            ansi_colors::RESET
        );
        println!(
            "  {}11.{} View help menu doc in editor (vi/nano)",
            ansi_colors::GREEN,
            ansi_colors::RESET
        );
        println!();
        print!(
            "{}Enter section number (1-10) or 'q' to quit: {}",
            ansi_colors::BOLD,
            ansi_colors::RESET
        );

        // Flush to ensure prompt appears
        io::stdout().flush().map_err(FileFantasticError::Io)?;

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(FileFantasticError::Io)?;
        let input = input.trim().to_lowercase();

        // Process user selection
        match input.as_str() {
            "1" => display_help_section_content(HelpSection::QuickStartBlurb)?,
            "2" => display_help_section_content(HelpSection::TopbarLegend)?,
            "3" => display_help_section_content(HelpSection::Navigation)?,
            "4" => display_help_section_content(HelpSection::SortingFiltering)?,
            "5" => display_help_section_content(HelpSection::SearchOptions)?,
            "6" => display_help_section_content(HelpSection::FileOperations)?,
            "7" => display_help_section_content(HelpSection::GetSendModeBlurb)?,
            "8" => display_help_section_content(HelpSection::ModularViewModes)?,
            "9" => display_help_section_content(HelpSection::TerminalManagement)?,
            "10" => display_help_section_content(HelpSection::Configuration)?,
            "11" => open_complete_help_in_editor()?,
            "q" | "quit" | "exit" => {
                println!(
                    "{}Exiting help system...{}",
                    ansi_colors::GREEN,
                    ansi_colors::RESET
                );
                return Ok(());
            }
            _ => {
                println!(
                    "{}Try again...Please enter 1-10 or 'q'.{}",
                    ansi_colors::YELLOW,
                    ansi_colors::RESET
                );
                wait_for_enter_keypress()?;
            }
        }
    }
}

/// Display a specific help section with proper formatting
///
/// This function clears the screen and displays the content for the
/// selected help section, waiting for user input before returning.
///
/// # Arguments
/// * `section` - The help section to display
///
/// # Returns
/// * `Result<()>` - Ok on successful display, Err on I/O errors
fn display_help_section_content(section: HelpSection) -> Result<()> {
    clear_terminal_screen()?;

    // Select and display appropriate section content
    let content = match section {
        HelpSection::QuickStartBlurb => HELP_SECTION_QUICK_START,
        HelpSection::TopbarLegend => HELP_SECTION_TOPBAR_LEGEND,
        HelpSection::Navigation => HELP_SECTION_NAVIGATION,
        HelpSection::SortingFiltering => HELP_SECTION_SORTING_FILTERING,
        HelpSection::SearchOptions => HELP_SECTION_SEARCH,
        HelpSection::FileOperations => HELP_SECTION_FILE_OPERATIONS,
        HelpSection::GetSendModeBlurb => HELP_SECTION_GET_SEND_MODE,
        HelpSection::ModularViewModes => HELP_SECTION_VIEW_MODES,
        HelpSection::TerminalManagement => HELP_SECTION_TERMINAL,
        HelpSection::Configuration => HELP_SECTION_CONFIGURATION,
    };

    // Display with color formatting
    print!("{}{}", ansi_colors::BOLD, ansi_colors::CYAN);
    println!("{}", content);
    print!("{}", ansi_colors::RESET);

    // Wait for user to read
    wait_for_enter_keypress()?;

    Ok(())
}

/// Open complete help documentation in external editor
///
/// This function compiles all help sections into a single document
/// and opens it in the user's preferred editor (vi or nano).
///
/// # Returns
/// * `Result<()>` - Ok if editor opened successfully, Err otherwise
///
/// # Errors
/// - Failed to create temp file
/// - Editor not found or failed to launch
fn open_complete_help_in_editor() -> Result<()> {
    // Create complete help content
    let mut complete_help = String::new();

    // Add header
    complete_help.push_str("FILE FANTASTIC (ff) - COMPLETE HELP DOCUMENTATION\n");
    complete_help.push_str("=".repeat(78).as_str());
    complete_help.push_str("\n\n");

    // Add all sections without ANSI codes for editor viewing
    complete_help.push_str("QUICK START & EXAMPLES\n");
    complete_help.push_str("-".repeat(78).as_str());
    complete_help.push_str("\n");
    complete_help.push_str(strip_ansi_codes(HELP_SECTION_QUICK_START).as_str());
    complete_help.push_str("\n\n");

    complete_help.push_str("NAVIGATION COMMANDS\n");
    complete_help.push_str("-".repeat(78).as_str());
    complete_help.push_str("\n");
    complete_help.push_str(strip_ansi_codes(HELP_SECTION_NAVIGATION).as_str());
    complete_help.push_str("\n\n");

    complete_help.push_str("SORTING & FILTERING\n");
    complete_help.push_str("-".repeat(78).as_str());
    complete_help.push_str("\n");
    complete_help.push_str(strip_ansi_codes(HELP_SECTION_SORTING_FILTERING).as_str());
    complete_help.push_str("\n\n");

    complete_help.push_str("SEARCH OPTIONS\n");
    complete_help.push_str("-".repeat(78).as_str());
    complete_help.push_str("\n");
    complete_help.push_str(strip_ansi_codes(HELP_SECTION_SEARCH).as_str());
    complete_help.push_str("\n\n");

    complete_help.push_str("FILE OPERATIONS (GET-SEND MODE)\n");
    complete_help.push_str("-".repeat(78).as_str());
    complete_help.push_str("\n");
    complete_help.push_str(strip_ansi_codes(HELP_SECTION_FILE_OPERATIONS).as_str());
    complete_help.push_str("\n\n");

    complete_help.push_str("TERMINAL & DISPLAY MANAGEMENT\n");
    complete_help.push_str("-".repeat(78).as_str());
    complete_help.push_str("\n");
    complete_help.push_str(strip_ansi_codes(HELP_SECTION_TERMINAL).as_str());
    complete_help.push_str("\n\n");

    complete_help.push_str("PARTNER PROGRAMS CONFIGURATION\n");
    complete_help.push_str("-".repeat(78).as_str());
    complete_help.push_str("\n");
    complete_help.push_str(strip_ansi_codes(HELP_SECTION_CONFIGURATION).as_str());

    // Create temp file
    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("ff_help_documentation.txt");

    // Write content to temp file
    fs::write(&temp_file_path, complete_help).map_err(FileFantasticError::Io)?;

    // Try to open in vi first, then nano
    let editor_result = Command::new("vi").arg(&temp_file_path).status();

    match editor_result {
        Ok(status) if status.success() => {
            // Successfully opened in vi
            println!(
                "{}Help documentation closed.{}",
                ansi_colors::GREEN,
                ansi_colors::RESET
            );
        }
        _ => {
            // Try nano as fallback
            let nano_result = Command::new("nano").arg(&temp_file_path).status();

            match nano_result {
                Ok(status) if status.success() => {
                    println!(
                        "{}Help documentation closed.{}",
                        ansi_colors::GREEN,
                        ansi_colors::RESET
                    );
                }
                _ => {
                    // Neither editor worked
                    eprintln!(
                        "{}Error: Could not open help in vi or nano.{}",
                        ansi_colors::YELLOW,
                        ansi_colors::RESET
                    );
                    eprintln!("Help file saved to: {}", temp_file_path.display());
                    wait_for_enter_keypress()?;
                }
            }
        }
    }

    // Note: We don't delete the temp file immediately in case user wants to reference it
    // OS will clean up temp directory eventually

    Ok(())
}

/// Clear the terminal screen using ANSI escape codes
///
/// This function uses ANSI escape sequences to clear the terminal
/// and reset the cursor to the top-left position.
///
/// # Returns
/// * `Result<()>` - Ok on success, Err on I/O error
fn clear_terminal_screen() -> Result<()> {
    // ANSI escape codes: clear screen and move cursor to top-left
    print!("\x1b[2J\x1b[1;1H");
    io::stdout().flush().map_err(FileFantasticError::Io)?;
    Ok(())
}

/// Wait for user to press Enter key
///
/// Simple utility function to pause execution until the user
/// presses the Enter key. Used between help sections.
///
/// # Returns
/// * `Result<()>` - Ok when Enter pressed, Err on I/O error
fn wait_for_enter_keypress() -> Result<()> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(FileFantasticError::Io)?;
    Ok(())
}

/// Strip ANSI color codes from a string
///
/// Removes all ANSI escape sequences from text for clean display
/// in external editors that don't support color codes.
///
/// # Arguments
/// * `text` - Text potentially containing ANSI codes
///
/// # Returns
/// * `String` - Text with all ANSI codes removed
fn strip_ansi_codes(text: &str) -> String {
    // Simple regex-like replacement without external dependencies
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip ANSI escape sequence
            // Format: ESC [ ... m
            if chars.peek() == Some(&'[') {
                chars.next(); // Skip '['
                // Skip until 'm'
                while let Some(next_ch) = chars.next() {
                    if next_ch == 'm' {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Check if help flag is present in command line arguments
///
/// This function checks if the user has requested help via
/// -h or --help command line flags.
///
/// # Arguments
/// * `args` - Command line arguments iterator
///
/// # Returns
/// * `bool` - true if help flag found, false otherwise
pub fn check_for_help_flag_in_args(args: &[String]) -> bool {
    // args.iter().any(|arg| arg == "-h" || arg == "--help")
    args.iter()
        // .skip(1)
        .any(|arg| arg == "-h" || arg == "--help")
}

/// Display quick usage information
///
/// Shows brief usage information when ff is called incorrectly
/// or when minimal help is needed. This is separate from the
/// full help system.
///
/// # Returns
/// * `Result<()>` - Ok on successful display
pub fn display_quick_usage_info() -> Result<()> {
    println!(
        "{}File Fantastic (ff) - File Manager{}",
        ansi_colors::BOLD,
        ansi_colors::RESET
    );
    println!();
    println!(
        "{}USAGE:{} ff [OPTIONS] [DIRECTORY]",
        ansi_colors::CYAN,
        ansi_colors::RESET
    );
    println!("{}OPTIONS:{}", ansi_colors::CYAN, ansi_colors::RESET);
    println!("  -h, --help    Show complete help menu.");
    println!("  -v, --version Show build version info.");
    println!("  --source      Get ff source code.");
    println!();
    println!("{}EXAMPLES:{}", ansi_colors::GREEN, ansi_colors::RESET);
    println!("  ff            Open in current directory.");
    println!("  ff ~/Path     Open dir or open file. ");
    println!("  ff ~/Path --line       Go to line of file.");
    println!("  ff ~/Path --session    Use existing lines session. ");
    println!();
    println!("  '--help' and '--source' are commands inside ff too.");

    Ok(())
}

// src/help_system.rs (add at the bottom)

#[cfg(test)]
mod helpview_tests {
    use super::*;

    /// Test that help flag detection works correctly
    #[test]
    fn test_help_flag_detection() {
        // Test with -h flag
        let args = vec![String::from("ff"), String::from("-h")];
        assert!(check_for_help_flag_in_args(&args));

        // Test with --help flag
        let args = vec![String::from("ff"), String::from("--help")];
        assert!(check_for_help_flag_in_args(&args));

        // Test with no help flag
        let args = vec![String::from("ff"), String::from("/some/path")];
        assert!(!check_for_help_flag_in_args(&args));

        // Test with help flag in middle
        let args = vec![
            String::from("ff"),
            String::from("/some/path"),
            String::from("--help"),
            String::from("other"),
        ];
        assert!(check_for_help_flag_in_args(&args));
    }

    /// Test ANSI code stripping functionality
    #[test]
    fn test_strip_ansi_codes() {
        // Test basic ANSI code removal
        let input = "\x1b[31mRed Text\x1b[0m";
        let expected = "Red Text";
        assert_eq!(strip_ansi_codes(input), expected);

        // Test multiple ANSI codes
        let input = "\x1b[1m\x1b[32mBold Green\x1b[0m Normal";
        let expected = "Bold Green Normal";
        assert_eq!(strip_ansi_codes(input), expected);

        // Test text without ANSI codes
        let input = "Plain text without codes";
        let expected = "Plain text without codes";
        assert_eq!(strip_ansi_codes(input), expected);

        // Test complex ANSI sequences
        let input = "\x1b[38;5;196mExtended Color\x1b[0m";
        let expected = "Extended Color";
        assert_eq!(strip_ansi_codes(input), expected);
    }

    /// Test that all help sections are defined
    #[test]
    fn test_all_help_sections_exist() {
        // Ensure all help section constants are non-empty
        assert!(!HELP_MENU_HEADER.is_empty());
        assert!(!HELP_SECTION_QUICK_START.is_empty());
        assert!(!HELP_SECTION_NAVIGATION.is_empty());
        assert!(!HELP_SECTION_SORTING_FILTERING.is_empty());
        assert!(!HELP_SECTION_SEARCH.is_empty());
        assert!(!HELP_SECTION_FILE_OPERATIONS.is_empty());
        assert!(!HELP_SECTION_GET_SEND_MODE.is_empty());
        assert!(!HELP_SECTION_TERMINAL.is_empty());
        assert!(!HELP_SECTION_CONFIGURATION.is_empty());
    }

    /// Test that help sections fit in 80x24 terminal
    #[test]
    fn test_help_sections_fit_terminal() {
        // Maximum lines for 80x24 terminal (leaving room for prompt)
        const MAX_LINES: usize = 24;
        const MAX_WIDTH: usize = 82; // maybe an off by one error here

        // Check each section
        let sections = vec![
            HELP_SECTION_QUICK_START,
            HELP_SECTION_NAVIGATION,
            HELP_SECTION_SORTING_FILTERING,
            HELP_SECTION_SEARCH,
            HELP_SECTION_FILE_OPERATIONS,
            HELP_SECTION_GET_SEND_MODE,
            HELP_SECTION_TERMINAL,
            HELP_SECTION_CONFIGURATION,
        ];

        for (i, section) in sections.iter().enumerate() {
            let stripped = strip_ansi_codes(section);
            let lines: Vec<&str> = stripped.lines().collect();

            // Check line count
            assert!(
                lines.len() <= MAX_LINES,
                "Section {} has {} lines (max {})",
                i + 1,
                lines.len(),
                MAX_LINES
            );

            // Check line width
            for (line_num, line) in lines.iter().enumerate() {
                assert!(
                    line.len() <= MAX_WIDTH,
                    "Section {} line {} is {} chars wide (max {})",
                    i + 1,
                    line_num + 1,
                    line.len(),
                    MAX_WIDTH
                );
            }
        }
    }
}

// Developer explicitly lists files to embed
const FF_SOURCE_FILES: &[SourcedFile] = &[
    SourcedFile::new("Cargo.toml", include_str!("../Cargo.toml")),
    SourcedFile::new("build.rs", include_str!("../build.rs")),
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
        "src/buffy_format_write_module.rs",
        include_str!("buffy_format_write_module.rs"),
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
    SourcedFile::new(
        "src/ribbon_external_counter_module.rs",
        include_str!("ribbon_external_counter_module.rs"),
    ),
    SourcedFile::new(
        "src/row_line_count_tui_module.rs",
        include_str!("row_line_count_tui_module.rs"),
    ),
    SourcedFile::new("README.md", include_str!("../README.md")),
    SourcedFile::new("LICENSE", include_str!("../LICENSE")),
    SourcedFile::new(
        "testtesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttest.txt",
        include_str!(
            "../testtesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttest.txt"
        ),
    ),
    SourcedFile::new(
        "src/buttons_reversible_edit_changelog_module.rs",
        include_str!("buttons_reversible_edit_changelog_module.rs"),
    ),
    SourcedFile::new(
        "src/lines_editor_module.rs",
        include_str!("lines_editor_module.rs"),
    ),
    SourcedFile::new(
        "src/toggle_comment_indent_module.rs",
        include_str!("toggle_comment_indent_module.rs"),
    ),
    SourcedFile::new("test.csv", include_str!("../test.csv")),
    SourcedFile::new(".gitignore", include_str!("../.gitignore")),
];

/*
* For build.rs and backup
* code below, with
if is_version_requested(&args) {
    match display_version() {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            // Failed to write to stdout, try stderr as fallback
            eprintln!("Error displaying version: {}", e);
            std::process::exit(1);
        }
    }
}
*/

// At the top of your file, create a module for build info
mod build_info {
    // Include the generated file here
    include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
}

// Create a function to format the build info for display
/// Formats all build information for version display
///
/// # Returns
/// A formatted string containing all build information
fn format_version_info() -> String {
    format!(
        "Version: {}\nBuild Date-Time: {}\nOS: {}\nArchitecture: {}\nRust Edition: {}\nRustc: {}\nProfile: {}",
        build_info::BUILD_PACKAGE_VERSION,
        build_info::BUILD_TIMESTAMP,
        build_info::BUILD_OS,
        build_info::BUILD_ARCHITECTURE,
        build_info::BUILD_RUST_EDITION,
        build_info::BUILD_RUSTC_VERSION,
        build_info::BUILD_PROFILE
    )
}

/// Checks if version information was requested via command line arguments.
///
/// # Arguments
///
/// * `args` - A slice of string references representing command line arguments
///
/// # Returns
///
/// `true` if either "--version" or "-v" is found in the arguments
pub fn is_version_requested(args: &[String]) -> bool {
    args.iter()
        .any(|arg| arg == "--version" || arg == "-v" || arg == "-V")
}

/// Version information embedded at compile time from Cargo.toml
///
/// These constants are populated by the Rust compiler using the env! macro,
/// which reads environment variables that Cargo sets during compilation.
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
/// Fallback version string when all else fails
const FALLBACK_VERSION: &str = "version unknown";

/// Constructs and returns a formatted version statement from compile-time constants.
///
/// This function uses compile-time constants as a reliable backup.
/// It will return SOMETHING even if one constant is empty.
///
/// # Returns
///
/// * `Ok(String)` containing whatever version info is available
/// * `Err` only if both constants are completely empty (should never happen)
///
/// # Example
///
/// ```
/// match backup_get_version_statement() {
///     Ok(version) => println!("{}", version),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// / Output: my-app 1.0.0
/// ```
pub fn backup_get_version_statement() -> Result<String> {
    // Return whatever we have - this is a BACKUP!
    let result = if !PKG_NAME.is_empty() && !VERSION.is_empty() {
        // Best case: both available
        format!("{} {}", PKG_NAME, VERSION)
    } else if !PKG_NAME.is_empty() {
        // Only package name available
        format!("{} (version unknown)", PKG_NAME)
    } else if !VERSION.is_empty() {
        // Only version available
        format!("unknown {}", VERSION)
    } else {
        // Both empty - this should never happen with env! macros
        format!("unknown")
    };

    Ok(result)
}

/// Writes version information to stdout with three levels of failsafe.
///
/// Attempts to display version information in order of preference:
/// 1. Full version info from `format_version_info()`
/// 2. Basic version from compile-time constants (partial data is OK)
/// 3. Fallback constant string
///
/// # Returns
///
/// * `Ok(())` if any version information was successfully written
/// * `Err(FileFantasticError)` only if all attempts to write failed
///
/// # Errors
///
/// Returns an error only if stdout is completely unavailable and
/// all three levels of version output failed.
pub fn display_version() -> Result<()> {
    let mut stdout = io::stdout();

    // Level 1: Try full version info
    let full_version = format_version_info();

    // Check if full version is valid (not empty, not error-like)
    if !full_version.is_empty()
        && !full_version.contains("unknown")
        && !full_version.contains("not detected")
    {
        // Try to write full version
        if writeln!(stdout, "{}", full_version).is_ok() {
            // Successfully wrote, now flush and return
            return stdout.flush().map_err(|e| FileFantasticError::Io(e));
        }
        // Write failed, print warning and continue to fallback
        eprintln!("Warning: Failed to write full version info to stdout");
    }

    // Level 2: Try backup version from compile-time constants
    match backup_get_version_statement() {
        Ok(backup_version) => {
            if writeln!(stdout, "{}", backup_version).is_ok() {
                // Successfully wrote backup version
                return stdout.flush().map_err(|e| FileFantasticError::Io(e));
            }
            // Write failed, print warning and continue to final fallback
            eprintln!("Warning: Failed to write backup version info to stdout");
        }
        Err(e) => {
            // Even backup version construction failed (very unlikely)
            eprintln!("Warning: Could not construct backup version: {}", e);
        }
    }

    // Level 3: Last resort - try to write fallback constant
    if writeln!(stdout, "{}", FALLBACK_VERSION).is_ok() {
        // Even the minimal version worked
        return stdout.flush().map_err(|e| FileFantasticError::Io(e));
    }

    // All three attempts failed - stdout is completely broken
    eprintln!("Error: Could not write any version information to stdout");
    Err(FileFantasticError::Io(io::Error::new(
        io::ErrorKind::Other,
        "Failed to write any version information to stdout",
    )))
}

/// Creates a new session directory and returns its path
///
/// # Purpose
/// Simple session directory creation for wrappers and tools that don't need
/// full EditorState infrastructure. Creates timestamped session directory
/// in standard location and returns absolute path.
///
/// # Project Context
/// Provides session isolation for draft copies without requiring EditorState.
/// Useful for:
/// - Wrappers around lines_core that need session directories
/// - Tools that want session isolation without full editor state
/// - Testing and utilities that need temporary organized workspaces
///
/// # Directory Structure Created
/// ```text
/// {executable_dir}/
///   lines_data/
///     sessions/
///       {timestamp}/          <- Created directory (returned)
/// ```
///
/// # Arguments
/// * `session_time_stamp` - Timestamp string for directory name (e.g., "2025_01_15_14_30_45")
///
/// # Returns
/// * `Ok(PathBuf)` - Absolute path to newly created session directory
/// * `Err(io::Error)` - Directory creation or validation failed
///
/// # Behavior
/// - Creates base infrastructure (lines_data/sessions/) if needed
/// - Creates new timestamped session directory
/// - Returns absolute canonicalized path
/// - Idempotent: Returns path if directory already exists with this timestamp
///
/// # Design Notes
/// - Does NOT use or require EditorState (no phantom state memory)
/// - Does NOT support recovery mode (use full version for that)
/// - Always creates new directory (or validates existing)
/// - Simpler alternative to initialize_session_directory for basic use cases
///
/// # Example
/// ```rust
/// let timestamp = "2025_01_15_14_30_45".to_string();
/// let session_path = simple_make_lines_editor_session_directory(timestamp)?;
/// // session_path is now: "/path/to/exe/lines_data/sessions/2025_01_15_14_30_45"
/// ```
pub fn simple_make_lines_editor_session_directory(
    session_time_stamp: String,
) -> io::Result<PathBuf> {
    // =================================================
    // Debug-Assert, Test-Assert, Production-Catch-Handle
    // =================================================

    // Defensive: Validate timestamp is not empty
    debug_assert!(
        !session_time_stamp.is_empty(),
        "Session timestamp should not be empty"
    );

    #[cfg(test)]
    assert!(
        !session_time_stamp.is_empty(),
        "Session timestamp should not be empty"
    );

    // Production catch: Handle empty timestamp
    if session_time_stamp.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "simple_make_lines_editor_session_directory: Empty timestamp provided",
        ));
    }

    // ===================================================================
    // STEP 1: Ensure base directory structure exists
    // ===================================================================
    // Creates: {executable_dir}/lines_data/sessions/
    let base_sessions_path = "lines_data/sessions";

    let sessions_dir = make_verify_or_create_executabledirectoryrelative_canonicalized_dir_path(
        base_sessions_path,
    )
    .map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            stack_format_it(
                "simple_make_lines_editor_session_directory: Failed to create sessions structure: {}",
                &[&e.to_string()],
                "simple_make_lines_editor_session_directory: Failed to create sessions structure",
            ),
        )
    })?;

    // Defensive: Verify the path is a directory
    if !sessions_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "simple_make_lines_editor_session_directory: Sessions path exists but is not a directory",
        ));
    }

    // ===================================================================
    // STEP 2: Create timestamped session directory
    // ===================================================================
    let session_path = sessions_dir.join(&session_time_stamp);

    // Check if directory already exists (idempotent)
    if session_path.exists() {
        // Defensive: Verify it's actually a directory
        if !session_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "simple_make_lines_editor_session_directory: Path exists but is not a directory",
            ));
        }

        // Already exists as directory - return it (idempotent)
        debug_assert!(
            session_path.is_absolute(),
            "Session path should be absolute"
        );

        return Ok(session_path);
    }

    // Create the session directory
    fs::create_dir(&session_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            stack_format_it(
                "simple_make_lines_editor_session_directory: Failed to create directory: {}",
                &[&e.to_string()],
                "simple_make_lines_editor_session_directory: Failed to create directory",
            ),
        )
    })?;

    // Defensive: Verify creation succeeded
    if !session_path.exists() || !session_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "simple_make_lines_editor_session_directory: Creation reported success but directory not found",
        ));
    }

    // Assertion: Verify path is absolute
    debug_assert!(
        session_path.is_absolute(),
        "Session path should be absolute"
    );

    // Test assertion: Verify path is absolute
    #[cfg(test)]
    assert!(
        session_path.is_absolute(),
        "Session path should be absolute"
    );

    Ok(session_path)
}

/// Public entry point for File Fantastic file manager module
///
/// # Usage as a Module
/// This function is designed to be imported and called from a main program:
/// ```rust
/// / src/main.rs
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
pub fn file_fantastic() -> Result<PathBuf> {
    // Collect command line arguments
    // let args: Vec<String> = env::args().collect();
    let args: Vec<String> = std::env::args().skip(1).collect();

    if is_version_requested(&args) {
        match display_version() {
            Ok(()) => std::process::exit(0),
            Err(e) => {
                // Failed to write to stdout, try stderr as fallback
                eprintln!("Error displaying version: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Module: Source It (export source code)
    if args.contains(&"--source".to_string()) {
        match handle_sourceit_command("ff_file_fantastic", None, FF_SOURCE_FILES) {
            Ok(path) => println!("Source extracted to: {}", path.display()),
            Err(e) => eprintln!("Failed to extract source: {}", e),
        }

        // since ff returns current path
        // return blank new path
        return Ok(PathBuf::new());
    }

    // Check if help was requested
    if check_for_help_flag_in_args(&args) {
        println!("Help requested!");
        println!("pirate's args {:?}", &args);
        // Display help menu instead of launching file manager
        match display_help_menu_system() {
            Ok(()) => {
                // Help displayed successfully, exit cleanly
                // std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error displaying help: {}", e);
                // std::process::exit(1);
            }
        }
        match display_quick_usage_info() {
            Ok(()) => {
                // Help displayed successfully, exit cleanly
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error display_quick_usage_info: {}", e);
                std::process::exit(1);
            }
        }
    }

    // // Get starting directory from args or default to current directory
    // let mut current_directory_path = match get_starting_path_from_args_or_cwd_default() {
    //     Ok(path) => path,
    //     Err(e) => {
    //         // Critical failure - unable to determine any starting directory
    //         eprintln!("Unable to determine starting directory: {}", e);
    //         eprintln!("This may be due to permission issues or missing directories.");
    //         return Err(e);
    //     }
    // };

    // =================================================
    // Parse All Command-Line Arguments
    // =================================================
    let cli_args = match parse_all_command_line_args() {
        Ok(args) => args,
        Err(e) => {
            // Critical failure - cannot determine starting location
            println!("Unable to determine starting location.");

            #[cfg(debug_assertions)]
            eprintln!("[DEBUG] Fatal error: {}", e);

            return Err(e);
        }
    };

    // =================================================
    // Launch Lines Editor if File Provided
    // =================================================
    if let Some(file_path) = cli_args.starting_location.file_to_open {
        #[cfg(debug_assertions)]
        eprintln!(
            "[DEBUG] Launching Lines editor for file: {}",
            file_path.display()
        );

        // Attempt to open file in Lines editor
        // Lines failure is non-fatal - FF will still start
        match lines_full_file_editor(
            Some(file_path.clone()),
            cli_args.starting_line,
            cli_args.session_path.clone(),
            true, // state_persists always true
        ) {
            Ok(_) => {
                #[cfg(debug_assertions)]
                eprintln!("[DEBUG] Lines editor completed successfully");
            }
            Err(_e) => {
                // Lines editor failed - warn but continue to FF
                println!("Warning: Could not open file in Lines editor.");
                println!("Continuing with File Fantastic...");

                #[cfg(debug_assertions)]
                eprintln!("[DEBUG] Lines editor error: {}", _e);
            }
        }
    }

    // =================================================
    // Start File Fantastic in Appropriate Directory
    // =================================================
    let mut current_directory_path = cli_args.starting_location.directory_to_browse;

    // Display startup information for transparency
    println!("Using directory: {}", current_directory_path.display());

    let mut nav_state = NavigationState::new();
    let mut state_manager = NavigationStateManager::new(); // Initialize here at the top

    //  ========================================
    //  Set Up & Build The Path for Lines Editor
    //  ========================================
    let session_time_base = createarchive_timestamp_with_precision(SystemTime::now(), true);
    nav_state.lines_editor_session_path =
        simple_make_lines_editor_session_directory(session_time_base)?;

    loop {
        // Read directory contents with proper error handling
        let mut all_entries = match read_directory_contents(&current_directory_path) {
            Ok(entries) => entries,
            Err(e) => {
                match e {
                    FileFantasticError::PermissionDenied(_) => {
                        eprintln!(
                            "Permission denied: Cannot read directory {}",
                            current_directory_path.display()
                        );
                        println!("Press Enter to go back to previous directory or 'q' to quit...");

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).map_err(|e| {
                            eprintln!("Failed to read input: {}", e);
                            FileFantasticError::Io(e)
                        })?;

                        if input.trim().eq_ignore_ascii_case("q") {
                            return Ok(current_directory_path);
                        }

                        // Try to go up one directory
                        match current_directory_path.parent() {
                            Some(parent) => {
                                current_directory_path = parent.to_path_buf();
                                continue;
                            }
                            None => {
                                eprintln!("Cannot navigate further up. Exiting.");
                                return Ok(current_directory_path);
                            }
                        }
                    }
                    FileFantasticError::NotFound(_) => {
                        eprintln!("Directory not found: {}", current_directory_path.display());

                        // Try to go up one directory
                        match current_directory_path.parent() {
                            Some(parent) => {
                                current_directory_path = parent.to_path_buf();
                                continue;
                            }
                            None => {
                                // Last resort: use current working directory
                                eprintln!("Falling back to current working directory");
                                match std::env::current_dir() {
                                    Ok(cwd) => {
                                        current_directory_path = cwd;
                                        continue;
                                    }
                                    Err(io_err) => {
                                        eprintln!("Cannot determine current directory: {}", io_err);
                                        return Err(FileFantasticError::Io(io_err));
                                    }
                                }
                            }
                        }
                    }
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
        let directory_entries: Vec<FileSystemEntry> = filtered_entries
            .iter()
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
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error displaying directory contents: {}", e);
                    eprintln!("Press Enter to try again or 'q' to quit...");

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).map_err(|e| {
                        eprintln!("Failed to read input: {}", e);
                        FileFantasticError::Io(e)
                    })?;

                    if input.trim().eq_ignore_ascii_case("q") {
                        return Ok(current_directory_path);
                    }
                    continue;
                }
            }

            // print!("\n>> "); // for extra space, maybe easier to see
            print!(">> "); // saves space
            match io::stdout().flush() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to flush stdout: {}", e);
                    // Non-critical error, continue
                }
            }

            let mut user_input = String::new();
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {}
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

            /* 1. Start with selection number
             * 2. ook up path of selection number
             * 3. handle path of selection (e.g. open file)
             */
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
                            match handle_file_open(
                                &entry.file_system_item_path,
                                &nav_state.lines_editor_session_path,
                            ) {
                                Ok(_) => {}
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
                        }
                        NavigationAction::AdjustTuiSize(adjustment_action) => {
                            // Apply the adjustment to the navigation state
                            apply_tui_resize_adjustment(&mut nav_state, &adjustment_action);

                            // Format current settings for display
                            let (tall_display, wide_display) = format_tui_adjustments(
                                nav_state.tui_tall_adjustment,
                                nav_state.tui_tall_direction_sign,
                                nav_state.tui_wide_adjustment,
                                nav_state.tui_wide_direction_sign,
                            );

                            // Provide clear feedback about what changed
                            let dimension_name =
                                if adjustment_action.adjustment_type_true_is_tall_false_is_wide {
                                    "Height"
                                } else {
                                    "Width"
                                };

                            let change_description = if adjustment_action
                                .adjustment_direction_true_is_positive_false_is_negative
                            {
                                "increased"
                            } else {
                                "decreased"
                            };

                            println!(
                                "{} {} by {}. Current settings: {} {}",
                                dimension_name,
                                change_description,
                                adjustment_action.adjustment_magnitude,
                                tall_display,
                                wide_display
                            );

                            // Brief pause so user can see the feedback
                            std::thread::sleep(std::time::Duration::from_millis(500));

                            // Break inner loop to refresh display with new settings
                            break;
                        }
                        NavigationAction::Filter(filter_char) => {
                            nav_state.set_filter(filter_char);
                            nav_state.current_page_index = 0; // Reset to first page after filter change
                            break; // Break inner loop to apply filter
                        }
                        NavigationAction::Sort(command) => {
                            nav_state.toggle_sort(command);
                            nav_state.current_page_index = 0; // Reset to first page after sort change
                            break; // Break inner loop to resort directory
                        }
                        NavigationAction::ChangeDirectory(new_path) => {
                            current_directory_path = new_path;
                            nav_state.current_page_index = 0; // Reset to first page in new directory
                            nav_state.selected_item_index = None; // clears selection upon new directroy
                            break; // Break inner loop to read new directory
                        }
                        NavigationAction::ParentDirectory => {
                            match current_directory_path.parent() {
                                Some(parent) => {
                                    current_directory_path = parent.to_path_buf();
                                    nav_state.current_page_index = 0; // Reset to first page
                                    nav_state.selected_item_index = None; // clears fields
                                    break; // Break inner loop to read new directory
                                }
                                None => {
                                    println!("Already at root directory");
                                }
                            }
                        }
                        NavigationAction::OpenFile(ref path) => {
                            match handle_file_open(path, &nav_state.lines_editor_session_path) {
                                Ok(_) => {}
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
                            println!(
                                "For Help Menu or For Source Code, run:  ff --help; ff --source"
                            );
                            println!("To continue from this location, run:");
                            println!("cd {}", current_directory_path.display());
                            return Ok(current_directory_path);
                        }

                        // exit/quit ff, returning selected path (path to selected item)
                        NavigationAction::ReturnPathExitFF(selected_path) => {
                            println!("Exiting ff, returning {}", selected_path.display());
                            return Ok(selected_path);
                        }

                        NavigationAction::OpenNewTerminal => {
                            println!("For tmux: Try vsplit | hsplit   Toggle: ctrl+b -> o");
                            match open_new_terminal(&current_directory_path) {
                                Ok(_) => {
                                    println!("Opening new terminal... Press Enter to continue");
                                }
                                Err(e) => {
                                    println!(
                                        "Error opening new terminal: {}. Press Enter to continue",
                                        e
                                    );
                                }
                            }
                            let _ = io::stdin().read_line(&mut String::new());
                        }

                        NavigationAction::VsplitTmux => {
                            // This command should simply create a new vertical pane with your default shell.
                            let _ = Command::new("tmux")
                                // .args(&["split-window", "-v"])
                                .args([
                                    "split-window",
                                    "-v", // "-v" for vertical or "-h" for horizontal
                                    // "vi editor_test.txt", // opens editor and closes split after
                                    // "top", // opens 'top' system monitor, 'q' to close
                                    "-c",
                                    &current_directory_path.to_string_lossy(),
                                ])
                                .output()?;
                        }
                        NavigationAction::HsplitTmux => {
                            // This command should create a new horizontal pane with your default shell.
                            let _ = Command::new("tmux")
                                // .args(&["split-window", "-v"])
                                .args([
                                    "split-window",
                                    "-h", // "-v" for vertical or "-h" for horizontal
                                    // "vi editor_test.txt", // opens editor and closes split after
                                    // "top", // opens 'top' system monitor, 'q' to close
                                    "-c",
                                    &current_directory_path.to_string_lossy(),
                                ])
                                .output()?;
                        }
                        NavigationAction::ArchiveModeShortcut => {
                            match state_manager.interactive_archive_selection(
                                &nav_state,
                                page_entries,
                                &current_directory_path,
                            ) {
                                Ok(_) => println!("Archive operation completed."),
                                Err(e) => println!("Error during archive operation: {}", e),
                            }
                        }
                        NavigationAction::GoToHelpMenuMode => match display_help_menu_system() {
                            Ok(()) => {}
                            Err(e) => {
                                eprintln!("Error displaying help: {}", e);
                            }
                        },
                        NavigationAction::GoToSouceCode => {
                            match handle_sourceit_command(
                                "ff_file_fantastic",
                                None,
                                FF_SOURCE_FILES,
                            ) {
                                Ok(path) => println!("Source extracted to: {}", path.display()),
                                Err(e) => eprintln!("Failed to extract source: {}", e),
                            }
                        }
                        NavigationAction::GoToFileLineCountMode => {
                            match show_minimal_linecount_tui(&current_directory_path) {
                                Ok(()) => {}
                                Err(e) => {
                                    eprintln!("Error with GoToFileLineCountMode: {}", e);
                                }
                            }
                        }
                        NavigationAction::GetSendMode => {
                            // Enter Get-Send-Mode loop
                            loop {
                                match state_manager.interactive_get_send_mode()? {
                                    GetSendModeAction::AddItemToStack => {
                                        /*
                                        TODO Plan A:
                                        get 'item' from stack ...
                                        a wrapper that picks if item is a file or a directory...
                                        interactive_add_item_a_file_or_dir_to_stack()

                                        */
                                        // Get currently selected file if any
                                        // let selected_file_path = nav_state.get_selected_item_path();

                                        match state_manager.interactive_add_item_to_stack(
                                            &nav_state,
                                            page_entries, // &all_entries, // Pass ALL page entries for pagination
                                            &current_directory_path, // Pass current directory path
                                        ) {
                                            Ok(_) => println!("Item stack operation completed."),
                                            Err(e) => println!("Error: {}", e),
                                        }
                                    }
                                    // In your main event handling code:
                                    GetSendModeAction::GetItemFromStack => {
                                        match state_manager.interactive_get_item_from_stack() {
                                            Ok(Some(retrieval_result)) => {
                                                match retrieval_result.item_type {
                                                    StackItemType::File => {
                                                        println!(
                                                            "Retrieved file: {}",
                                                            retrieval_result.path.display()
                                                        );
                                                        println!(
                                                            "Copying to current directory: {}",
                                                            current_directory_path.display()
                                                        );

                                                        // Copy the file to current directory
                                                        match copy_file_from_stack(
                                                            &retrieval_result.path,
                                                            &current_directory_path,
                                                        ) {
                                                            Ok(final_destination_path) => {
                                                                println!(
                                                                    "✓ File copy operation completed successfully!"
                                                                );
                                                                println!(
                                                                    "Final location: {}",
                                                                    final_destination_path
                                                                        .display()
                                                                );
                                                            }
                                                            Err(e) => {
                                                                eprintln!(
                                                                    "✗ File copy operation failed: {}",
                                                                    e
                                                                );
                                                            }
                                                        }
                                                    }
                                                    StackItemType::Directory => {
                                                        println!(
                                                            "Retrieved directory: {}/",
                                                            retrieval_result.path.display()
                                                        );
                                                        println!(
                                                            "Copying to current directory: {}",
                                                            current_directory_path.display()
                                                        );

                                                        // Copy the directory recursively to current directory
                                                        match copy_directory_from_stack(
                                                            &retrieval_result.path,
                                                            &current_directory_path,
                                                        ) {
                                                            Ok(final_destination_path) => {
                                                                println!(
                                                                    "✓ Directory copy operation completed successfully!"
                                                                );
                                                                println!(
                                                                    "Final location: {}/",
                                                                    final_destination_path
                                                                        .display()
                                                                );
                                                            }
                                                            Err(e) => {
                                                                eprintln!(
                                                                    "✗ Directory copy operation failed: {}",
                                                                    e
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                                println!("\nPress Enter to continue...");
                                                let _ = io::stdin().read_line(&mut String::new());
                                            }
                                            Ok(None) => println!("No item selected."),
                                            Err(e) => {
                                                println!("Error getting item from stack: {}", e)
                                            }
                                        }
                                    }
                                    GetSendModeAction::SavePocketDimension => {
                                        print!(
                                            "Enter nickname for this location (or Enter for auto): "
                                        );
                                        io::stdout()
                                            .flush()
                                            .map_err(|e| FileFantasticError::Io(e))?;
                                        let mut nickname = String::new();
                                        io::stdin()
                                            .read_line(&mut nickname)
                                            .map_err(|e| FileFantasticError::Io(e))?;
                                        let nickname = if nickname.trim().is_empty() {
                                            None
                                        } else {
                                            Some(nickname.trim().to_string())
                                        };
                                        match state_manager.save_pocket_dimension(
                                            current_directory_path.clone(),
                                            &nav_state,
                                            &dir_view,
                                            nav_state.selected_item_index,
                                            nav_state.active_search_term.clone(),
                                            nickname,
                                        ) {
                                            Ok(saved_name) => println!(
                                                "Saved as pocket dimension: {}",
                                                saved_name
                                            ),
                                            Err(e) => {
                                                println!("Error saving pocket dimension: {}", e)
                                            }
                                        }
                                    }
                                    GetSendModeAction::GoToPocketDimension => {
                                        match state_manager.interactive_select_pocket_dimension() {
                                            Ok(Some(nickname)) => {
                                                match state_manager
                                                    .restore_pocket_dimension(&nickname)
                                                {
                                                    Ok(saved_state) => {
                                                        // Restore the complete navigation state including all preferences
                                                        current_directory_path =
                                                            saved_state.current_directory_path;
                                                        nav_state.current_sort_method =
                                                            saved_state.current_sort_method;
                                                        nav_state.current_filter =
                                                            saved_state.current_filter;
                                                        nav_state.selected_item_index =
                                                            saved_state.selected_item_index;
                                                        nav_state.active_search_term =
                                                            saved_state.active_search_term;

                                                        // Restore TUI size adjustment settings
                                                        // These settings allow each pocket dimension to have its own optimal display configuration
                                                        nav_state.tui_tall_adjustment =
                                                            saved_state.tui_tall_adjustment;
                                                        nav_state.tui_tall_direction_sign =
                                                            saved_state.tui_tall_direction_sign;
                                                        nav_state.tui_wide_adjustment =
                                                            saved_state.tui_wide_adjustment;
                                                        nav_state.tui_wide_direction_sign =
                                                            saved_state.tui_wide_direction_sign;

                                                        // Restore pagination state
                                                        nav_state.current_page_index =
                                                            saved_state.current_page_number;

                                                        // Build a human-readable size adjustment string for display
                                                        let tall_adjustment_display =
                                                            if saved_state.tui_tall_adjustment == 0
                                                            {
                                                                String::from("default")
                                                            } else {
                                                                format!(
                                                                    "tall{}{}",
                                                                    if saved_state
                                                                        .tui_tall_direction_sign
                                                                    {
                                                                        "+"
                                                                    } else {
                                                                        "-"
                                                                    },
                                                                    saved_state.tui_tall_adjustment
                                                                )
                                                            };

                                                        let wide_adjustment_display =
                                                            if saved_state.tui_wide_adjustment == 0
                                                            {
                                                                String::from("default")
                                                            } else {
                                                                format!(
                                                                    "wide{}{}",
                                                                    if saved_state
                                                                        .tui_wide_direction_sign
                                                                    {
                                                                        "+"
                                                                    } else {
                                                                        "-"
                                                                    },
                                                                    saved_state.tui_wide_adjustment
                                                                )
                                                            };

                                                        // Inform user of successful restoration with details
                                                        println!(
                                                            "Jumped to pocket dimension: {} (page {}, size: {} {})",
                                                            nickname,
                                                            saved_state.current_page_number + 1,
                                                            tall_adjustment_display,
                                                            wide_adjustment_display
                                                        );

                                                        break; // Exit Get-Send-Mode and refresh directory with restored settings
                                                    }
                                                    Err(e) => {
                                                        // Handle restoration errors gracefully
                                                        println!(
                                                            "Error restoring pocket dimension: {}",
                                                            e
                                                        );
                                                        println!("Press Enter to continue...");
                                                        let _ = io::stdin()
                                                            .read_line(&mut String::new());
                                                    }
                                                }
                                            }
                                            Ok(None) => {
                                                // User cancelled selection
                                                println!("No pocket dimension selected.");
                                            }
                                            Err(e) => {
                                                // Handle selection errors
                                                println!("Error selecting pocket dimension: {}", e);
                                                println!("Press Enter to continue...");
                                                let _ = io::stdin().read_line(&mut String::new());
                                            }
                                        }
                                    }
                                    GetSendModeAction::ViewStacks => {
                                        println!("\n=== Current Status ===");
                                        println!("{}", state_manager.get_stack_summary());

                                        if !state_manager.file_path_stack.is_empty() {
                                            println!("\nFile Stack:");
                                            for (i, file) in
                                                state_manager.file_path_stack.iter().enumerate()
                                            {
                                                println!("  {}. {}", i + 1, file.display());
                                            }
                                        }

                                        if !state_manager.directory_path_stack.is_empty() {
                                            println!("\nDirectory Stack:");
                                            for (i, dir) in state_manager
                                                .directory_path_stack
                                                .iter()
                                                .enumerate()
                                            {
                                                println!("  {}. {}", i + 1, dir.display());
                                            }
                                        }

                                        if !state_manager.pocket_dimensions.is_empty() {
                                            println!("\nPocket Dimensions:");
                                            let dimensions = state_manager.list_pocket_dimensions();
                                            for (i, (nickname, state)) in
                                                dimensions.iter().enumerate()
                                            {
                                                println!(
                                                    "  {}. {} - {}",
                                                    i + 1,
                                                    nickname,
                                                    state.description
                                                );
                                            }
                                        }

                                        println!("\nPress Enter to continue...");
                                        let _ = io::stdin().read_line(&mut String::new());
                                    }
                                    GetSendModeAction::ArchiveSelection => {
                                        match state_manager.interactive_archive_selection(
                                            &nav_state,
                                            page_entries,
                                            &current_directory_path,
                                        ) {
                                            Ok(_) => println!("Archive operation completed."),
                                            Err(e) => {
                                                println!("Error during archive operation: {}", e)
                                            }
                                        }
                                    }
                                    GetSendModeAction::ClearAll => {
                                        print!("Clear all stacks and pocket dimensions? (y/N): ");
                                        io::stdout()
                                            .flush()
                                            .map_err(|e| FileFantasticError::Io(e))?;
                                        let mut response = String::new();
                                        io::stdin()
                                            .read_line(&mut response)
                                            .map_err(|e| FileFantasticError::Io(e))?;

                                        if response.trim().eq_ignore_ascii_case("y") {
                                            state_manager.clear_all();
                                            println!("All stacks and pocket dimensions cleared.");
                                        }
                                    }
                                    GetSendModeAction::ReturnToBrowser => break,
                                }
                            }
                            // After exiting Get-Send-Mode, break to refresh directory
                            break;
                        }
                        NavigationAction::Invalid => {
                            println!("Not an item number. Please press Enter to continue..."); // TODO
                            let _ = io::stdin().read_line(&mut String::new());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error processing input: {}", e);
                    println!("Press Enter to continue...");
                    let _ = io::stdin().read_line(&mut String::new());
                }
            }
        }
    }
}
