// src/lib.rs (or src/ff_file_fantastic_module.rs)
/// ff - A minimal file manager in Rust
/// use -> cargo build --profile release-performance
/// or, use -> cargo build --profile release-small 

/* Docs:
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
1. use best practice
2. absolute file paths
3. no third party dependencies
4. docstrings required
4. code comments required
5. clear unique meaningful naming required
6. no unwrap
7. no unsafe code
8. all errors to be handled
9. terminal cli application
10. module to be used in other projects

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
7. open file in new terminal: note, due to using os-default if available,
   File Fantastic can open image or other files, at least sometimes.
8. hit enter to refresh
11. single letter commands
12. legend shows command 'words': use first letter as command
(q)uit (b)ack|(t)erminal|(d)ir (f)ile|(n)ame (s)ize (m)od|str>search|enter>reset
w for up, s for down, a for all 
13. 'sort by size' ' 'sort by name' 'sort by last-modified': re-selecting a sort option reverses the order
14. Type a string for a partial match search.
15. 'f' or 'd' to show only files or only directories


# Scrolling
1. MVP: use mouse wheel to scroll up and down
2. pages using w and s to scroll up and down

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
// const RED: &str = "\x1b[31m";
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
struct DirectoryView<'a> {
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
struct FileSystemEntry {
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
struct NavigationState {
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
enum DirectorySortingMethodEnum {
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
    page_info: Option<(usize, usize)>, // (current_page, total_pages)
    filter: Option<char>,
) -> io::Result<()> {
    // clear screen
    print!("\x1B[2J\x1B[1;1H");

    let filter_status = match filter {
        Some('d') => "[Directories only] ",
        Some('f') => "[Files only] ",
        _ => "",
    };

    // // Add pagination info to legend if applicable
    // let legend = "(q)uit (b)ack|(t)erminal|(n)ame (s)ize (m)odified|(w)up (s)down|str>search";
    let legend = format!(
        "{}{}(q)uit (b)ack|(t)erminal|(d)ir (f)ile|(n)ame (s)ize (m)od|str>search|enter>reset{}", 
        YELLOW,
        filter_status,
        RESET);
    
    let path_display = format!("{}", current_directory_path.display());

    // // Show pagination info if provided
    // if let Some((current_page, total_pages)) = page_info {
    //     if total_pages > 1 {
    //         // println!("{}\n{}\nPage {} of {}",
    //         println!("{}\n{}",
    //             // legend, path_display, current_page, total_pages
    //             legend, path_display,
    //             );
    //     } else {
    //         println!("{}\n{}", legend, path_display);
    //     }
    // } else {
    //     println!("{}\n{}", legend, path_display);
    // }
    println!("{}\n{}", legend, path_display);

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
            println!("--- Page {} of {}: (w)^ for above page, (s)v for below page ---", 
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

        // Create paginated view
        let mut dir_view = DirectoryView::new(&directory_entries);

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
            
            // Handle pagination commands first
            let trimmed_input = user_input.trim();
            if trimmed_input == "s" {
                dir_view.next_page();
                continue; // Stay in inner loop, just change page
            } else if trimmed_input == "w" {
                dir_view.prev_page();
                continue; // Stay in inner loop, just change page
            }
            
            // Handle number input directly to account for pagination
            if let Ok(number) = trimmed_input.parse::<usize>() {
                if let Some(actual_index) = dir_view.get_actual_index(number) {
                    // Only process if within range of full directory listing
                    if actual_index < directory_entries.len() {
                        let entry = &directory_entries[actual_index];
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
                        NavigationAction::Filter(filter_char) => {
                            nav_state.set_filter(filter_char);
                            break; // Break inner loop to apply filter
                        },
                        NavigationAction::ChangeDirectory(new_path) => {
                            current_directory_path = new_path;
                            break; // Break inner loop to read new directory
                        }
                        NavigationAction::ParentDirectory => {
                            match current_directory_path.parent() {
                                Some(parent) => {
                                    current_directory_path = parent.to_path_buf();
                                    break; // Break inner loop to read new directory
                                },
                                None => {
                                    println!("Already at root directory");
                                    // Stay in current directory
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
                        NavigationAction::Quit => return Ok(()),
                        NavigationAction::Sort(command) => {
                            nav_state.toggle_sort(command);
                            break; // Break inner loop to resort directory
                        }
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

