/// ff - A minimal file manager in Rust
/// use -> cargo build --profile release-small 

/* Docs:
ff is a minimal rust file manager


A very minimal 'file manager', much more minimal than "midnight commander." 

# Scope:
1. no third party dependencies
2. docstrings required
3. code comments required
4. no unwrap
5. no unsafe code
6. all errors to be handled
7. terminal cli application

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
(q)uit (b)ack|(t)erminal|(f)iles (d)ir|(n)ame (s)ize (m)odified|str>search 
13. 'sort by size' ' 'sort by name' 'sort by last-modified': re-selecting a sort option reverses the order
14. Type a string for a partial match search.
pending 
(pending) 15. 'f' or 'd' to show only files or only directories


# Scrolling
1. MVP: use mouse wheel to scroll up and down

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
- or first MVP, terminal size is default terminal size
- for MVP...mouse to scroll up and down works fine for mvp


(future items, after mvp)

## show just files/dir

## scroll
maybe scrolling but likely not needed

### TUI Size:
- or first MVP, set terminal size to actual default terminal size
- start with standard terminal size
- scroll / offset and range
- optionally say how many rows off screen
- use key commands to increase or decrease TUI display size
- for MVP... only wide and narrow need to adjust... (mouse to scroll up and down works fine for mvp)

*/

use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// Maximum Levenshtein distance to consider a match
const MAX_SEARCH_DISTANCE: usize = 2;

/// Represents a search result with its distance score
/// Represents a search result with its Levenshtein distance score and item details
/// 
/// # Fields
/// * `item_name` - The name of the matching file or directory
/// * `item_path` - The full path to the matching item
/// * `distance` - The Levenshtein distance score (lower is better)
/// * `display_index` - The item's current display position in the file listing
/// 
/// # Usage
/// Used to store and sort fuzzy search matches when a user enters a search term.
/// Lower distance scores indicate closer matches to the search term.
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
/// # Arguments
/// * `timestamp` - SystemTime to format
/// 
/// # Returns
/// * String - Formatted date/time string
/// 
/// # Format
/// - Today: "HH:MM"
/// - This year: "MM-DD HH:MM"
/// - Older: "YYYY-MM-DD"
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

/// Convert seconds since epoch to year, month, day, hour, minute
fn seconds_to_components(secs: u64) -> (u32, u32, u32, u32, u32) {
    let secs_per_minute = 60;
    let secs_per_hour = secs_per_minute * 60;
    let secs_per_day = secs_per_hour * 24;

    let minute = ((secs % secs_per_hour) / secs_per_minute) as u32;
    let hour = ((secs % secs_per_day) / secs_per_hour) as u32;
    
    let (year, month, day) = seconds_to_ymd(secs);
    
    (year, month, day, hour, minute)
}

/// Convert seconds since epoch to year, month, day
fn seconds_to_ymd(secs: u64) -> (u32, u32, u32) {
    // This is a simplified version. For production code,
    // consider using the chrono crate or implementing full
    // calendar calculations including leap years
    let days_since_epoch = (secs / (24 * 60 * 60)) as u32;
    let year = 1970 + (days_since_epoch / 365);
    let month = 1 + ((days_since_epoch % 365) / 30);
    let day = 1 + ((days_since_epoch % 365) % 30);
    
    (year, month, day)
}

/// Sorts directory entries based on specified method while maintaining directories at the top
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
/// ```
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
/// # Arguments
/// * `directory_path` - PathBuf of the directory to open terminal in
/// 
/// # Returns
/// * `io::Result<()>` - Success: () unit type
///                      Error: IO error with description
/// 
/// # Platform-specific Implementation
/// - Uses 'Terminal.app' on macOS
/// - Uses 'gnome-terminal' or other terminals on Linux
/// - Uses 'cmd.exe' on Windows
/// Opens a new terminal window at the specified directory
/// 
/// # Arguments
/// * `directory_path` - PathBuf of the directory to open terminal in
/// 
/// # Returns
/// * `io::Result<()>` - Success: () unit type
///                      Error: IO error with description
fn open_new_terminal(directory_path: &PathBuf) -> io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-a", "Terminal"])
            .arg(directory_path)
            .spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        // Try different terminal emulators in order of preference
        let terminal_commands = [
            ("gnome-terminal", vec!["--working-directory"]),
            ("konsole", vec!["--workdir"]),
            ("xfce4-terminal", vec!["--working-directory"]),
            ("xterm", vec!["-e", "cd"]),  // xterm needs special handling
        ];

        for (terminal, args) in terminal_commands.iter() {
            let mut command = std::process::Command::new(terminal);
            
            if *terminal == "xterm" {
                command.args(args)
                    .arg(directory_path.to_string_lossy().to_string())
                    .arg("&& bash");
            } else {
                command.args(args)
                    .arg(directory_path);
            }

            match command.spawn() {
                Ok(_) => return Ok(()),
                Err(_) => continue,
            }
        }
        
        // Fixed error return with explicit type
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No supported terminal emulator found",
        ));
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", "cmd.exe"])
            .current_dir(directory_path)
            .spawn()?;
    }
    
    Ok(())
}

/// Processes user input and returns the corresponding NavigationAction
/// 
/// # Arguments
/// * `input` - The user's input string
/// * `nav_state` - Current navigation state containing lookup table
/// 
/// # Returns
/// * `io::Result<NavigationAction>` - The determined action to take
/// Updated process_user_input to handle search
fn process_user_input(
    input: &str,
    nav_state: &NavigationState,
    directory_entries: &[FileSystemEntry],
) -> io::Result<NavigationAction> {
    let input = input.trim();
    
    // Handle single-character commands first
    if input.len() == 1 {
        match input.to_lowercase().as_str() {
            "q" => return Ok(NavigationAction::Quit),
            "b" => return Ok(NavigationAction::ParentDirectory),
            "t" => return Ok(NavigationAction::OpenNewTerminal),
            "n" | "s" | "m" => return Ok(NavigationAction::Sort(input.chars().next().unwrap())),
            _ => {}
        }
    }

    // Handle empty input
    if input.is_empty() {
        return Ok(NavigationAction::Refresh);
    }

    // Try to parse as number for direct selection
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
    let search_results = nav_state.fuzzy_search(input, directory_entries);
    display_search_results(&search_results)?;
    
    // Wait for user to select from results or press enter to continue
    print!("\nEnter number to select or press Enter to continue: ");
    io::stdout().flush()?;
    
    let mut selection = String::new();
    io::stdin().read_line(&mut selection)?;
    
    if let Ok(number) = selection.trim().parse::<usize>() {
        if let Some(result) = search_results.iter().find(|r| r.display_index == number) {
            // Check if selected item is in lookup table and get its type
            if let Some(item_info) = nav_state.lookup_item(number) {
                return Ok(match item_info.item_type {
                    FileSystemItemType::Directory => {
                        NavigationAction::ChangeDirectory(result.item_path.clone())
                    }
                    FileSystemItemType::File => {
                        NavigationAction::OpenFile(result.item_path.clone())
                    }
                });
            }
        }
    }

    Ok(NavigationAction::Refresh)
}

/// Represents possible navigation actions based on user input in the file manager
/// 
/// # Purpose
/// This enum centralizes all possible actions that can result from user input,
/// providing a clear interface between input processing and action handling.
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
/// 
/// # Command Characters
/// Sort commands use specific characters:
/// - 'n' - Sort by name
/// - 's' - Sort by size
/// - 'm' - Sort by modification time
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
}

/// Formats file size into human readable format
/// 
/// # Arguments
/// * `size_in_bytes` - The file size in bytes
/// 
/// # Returns
/// * String - Formatted size string (e.g., "1.2 MB", "340 KB", "12 B")
/// 
/// # Format Rules
/// - Uses B, KB, MB, GB
/// - Shows decimal point only when value < 10
/// - Maximum 2 decimal places
/// - Shows unit that allows number to be 0.1 to 99.99
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

// TODO doc string
/// Represents an item's type in the file system
#[derive(Debug, Clone, PartialEq)]
enum FileSystemItemType {
    Directory,
    File,
}

// TODO doc string
/// Represents a displayed item's information for lookup purposes
#[derive(Debug)]
struct DisplayedItemInfo {
    /// The display index (number shown to user)
    display_index: usize,
    /// The full path to the item
    item_path: PathBuf,
    /// The type of the item (file or directory)
    item_type: FileSystemItemType,
    /// Original index in unsorted directory listing
    original_index: usize,
}

/// FileSystemEntry represents a single item (file or directory) in the file system
/// with its essential metadata for display and manipulation in the file manager.
/// 
/// This struct is used to store information about files and directories that
/// will be displayed in the numbered list interface of the file manager.
/// 
/// Properties are deliberately named to be extremely clear and unambiguous,
/// avoiding short or cryptic abbreviations.
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

/// Display search results in a formatted table
/// Displays search results in a formatted table with clear headers
/// 
/// # Arguments
/// * `results` - Vector of SearchResult items to display
/// 
/// # Returns
/// * `io::Result<()>` - Success or IO error
/// 
/// # Display Format
/// ```text
/// Search Results:
/// Num   Name                           Distance
/// ---------------------------------------------
///  1    example.txt                       2
///  2    sample.doc                        3
/// ```
/// 
/// # Notes
/// - Truncates long filenames to fit display
/// - Shows original item numbers from directory listing
/// - Distance indicates how close the match is (lower is better)
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

// TODO doc string
/// Represents the dimensions and navigation state of the terminal UI
struct NavigationState {
    /// Height of terminal display area in rows
    terminal_height_rows: u16,
    /// Width of terminal display area in columns
    terminal_width_columns: u16,
    /// Current scroll position (which row is at top of display)
    scroll_position: usize,
    /// Number of rows available for file listing (terminal height minus headers/footers)
    available_display_rows: u16,
    /// Lookup table mapping displayed numbers to item information
    /// Key: displayed number (1-based index shown to user)
    /// Value: information about the item at that display position
    display_lookup_table: HashMap<usize, DisplayedItemInfo>,
    current_sort_method: DirectorySortingMethodEnum,
    last_sort_command: Option<char>,  // Tracks last sort command used
}

// TODO doc string
impl NavigationState {
    /// Creates a new NavigationState with default terminal dimensions
    /// and empty lookup table
    fn new() -> Self {
        NavigationState {
            terminal_height_rows: 24, // default terminal height
            terminal_width_columns: 80, // default terminal width
            scroll_position: 0,
            available_display_rows: 20, // default visible rows
            display_lookup_table: HashMap::new(),
            current_sort_method: DirectorySortingMethodEnum::Name(true),
            last_sort_command: None,
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
                    display_index: index + 1,
                    item_path: entry.file_system_item_path.clone(),
                    item_type: if entry.is_directory {
                        FileSystemItemType::Directory
                    } else {
                        FileSystemItemType::File
                    },
                    original_index: index,
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

// TODO doc string
/// Simplified sort methods - just the three main types
#[derive(Debug, Clone, Copy, PartialEq)]
enum DirectorySortingMethodEnum {
    Name(bool),    // bool represents ascending (true) or descending (false)
    Size(bool),
    Modified(bool),
}

// TODO doc string
/// Maintains current display information for the file manager
struct DisplayState {
    /// Current directory being displayed
    current_directory_path: PathBuf,
    
    /// Current list of files/directories to display
    directory_contents: Vec<FileSystemEntry>,
    
    /// Display table for TUI (row,column format)
    /// [item_number, name, size, modified_date]
    display_table_rows: Vec<[String; 4]>,
    
    /// Current sort method and direction for directory contents
    directory_sorting_method: DirectorySortingMethodEnum,

    /// Terminal display and navigation information
    navigation_state: NavigationState,
}

/// Reads contents of a directory and returns a Result containing a vector of FileSystemEntry items
/// 
/// # Arguments
/// * `directory_path_to_read` - The PathBuf pointing to the directory to be read
/// 
/// # Returns
/// * `io::Result<Vec<FileSystemEntry>>` - Success: Vector of FileSystemEntry items
///                                       Error: IO error with description
/// 
/// # Error Handling
/// - Handles directory read errors
/// - Handles metadata read errors
/// - Handles timestamp conversion errors
/// 
/// # Example Usage
/// ```
/// let current_path = std::env::current_dir()?;
/// let directory_entries = read_directory_contents(&current_path)?;
/// ```
/// Update read_directory_contents to store SystemTime
fn read_directory_contents(directory_path_to_read: &PathBuf) -> io::Result<Vec<FileSystemEntry>> {
    let mut directory_entries_list: Vec<FileSystemEntry> = Vec::new();
    
    for directory_item_result in fs::read_dir(directory_path_to_read)? {
        let directory_item = directory_item_result?;
        let item_metadata = directory_item.metadata()?;
        
        directory_entries_list.push(FileSystemEntry {
            file_system_item_name: directory_item
                .file_name()
                .to_string_lossy()
                .to_string(),
            file_system_item_path: directory_item.path(),
            file_system_item_size_in_bytes: item_metadata.len(),
            file_system_item_last_modified_time: item_metadata.modified()?,
            is_directory: item_metadata.is_dir(),
        });
    }

    Ok(directory_entries_list)
}

// TODO doc string
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

/// Formats and displays directory contents as a numbered list with columns
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
fn display_directory_contents(
    directory_entries: &[FileSystemEntry],
    current_directory_path: &PathBuf,
) -> io::Result<()> {
    // clear screen
    print!("\x1B[2J\x1B[1;1H");
    println!(
        // legens
        "(q)uit (b)ack|(t)erminal|(f)iles (d)ir|(n)ame (s)ize (m)odified|str>search \n{}\n",
        current_directory_path.display()
    );

    println!(
        "{:>4}  {:<40} {:>15} {:>15}",
        "Num", "Name", "Size", "Modified"
    );
    println!(" {} ", "-".repeat(78));

    for (entry_index, directory_entry) in directory_entries.iter().enumerate() {
        let formatted_name = if directory_entry.is_directory {
            format!("{}/", directory_entry.file_system_item_name)
        } else {
            directory_entry.file_system_item_name.clone()
        };

        let display_name = if formatted_name.chars().count() > 28 {
            let truncated: String = formatted_name.chars().take(25).collect();
            format!("{}...", truncated)
        } else {
            formatted_name
        };

        let size_display = if directory_entry.is_directory {
            String::from("-")
        } else {
            format_file_size(directory_entry.file_system_item_size_in_bytes)
        };

        let time_display = format_timestamp(directory_entry.file_system_item_last_modified_time);

        println!(
            "{:>3}. {:<40} {:>15} {:>15}",
            entry_index + 1,
            display_name,
            size_display,
            time_display
        );
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
/// * `io::Result<()>` - Success or IO error
/// 
/// # Behavior
/// - Prompts user to select editor (e.g., nano, vim, code)
/// - Empty input uses system default opener
/// - Terminal-based editors open in new terminal window
/// - GUI editors (code, sublime, etc.) launch directly
/// - Falls back to system default if editor fails
/// 
/// # Example
/// ```text
/// Open with (enter for default, or type: nano/vim/code/etc): vim
/// ```
fn open_file(file_path: &PathBuf) -> io::Result<()> {
    print!("Open with... (hit enter for default, or enter your editor 'name': hx, lapce, vi, vim, nano, code, etc.): ");
    io::stdout().flush()?;
    
    let mut editor = String::new();
    io::stdin().read_line(&mut editor)?;
    let editor = editor.trim();

    if editor.is_empty() {
        // Use system default
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(file_path)
                .spawn()?;
        }
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(file_path)
                .spawn()?;
        }
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "start", ""])
                .arg(file_path)
                .spawn()?;
        }
    } else {
        // List of known GUI editors that shouldn't need a terminal
        let gui_editors = ["code", "sublime", "subl", "gedit", "kate", "notepad++"];
        
        if gui_editors.contains(&editor.to_lowercase().as_str()) {
            // Launch GUI editors directly
            match std::process::Command::new(editor)
                .arg(file_path)
                .spawn() 
            {
                Ok(_) => return Ok(()),
                Err(e) => {
                    println!("Error launching {}: {}. Falling back to system default...", editor, e);
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
                    .spawn()?;
            }
            #[cfg(target_os = "linux")]
            {
                // Try different terminal emulators
                let terminal_commands = [
                    ("gnome-terminal", vec!["--", editor]),
                    ("konsole", vec!["--e", editor]),
                    ("xfce4-terminal", vec!["--command", editor]),
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
                    println!("No terminal available. Falling back to system default...");
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    return open_file(file_path);
                }
            }
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("cmd")
                    .args(["/C", "start", "cmd", "/C"])
                    .arg(format!("{} {} && pause", editor, file_path.to_string_lossy()))
                    .spawn()?;
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
/// * `io::Result<()>` - Success or IO error
/// 
/// # Behavior
/// - Prompts for editor selection
/// - Opens terminal editors in new window
/// - Launches GUI editors directly
/// - Shows status messages
fn handle_file_open(path: &PathBuf) -> io::Result<()> {
    match open_file(path) {
        Ok(_) => {
            println!("Opening file... \n\nPress Enter to continue");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
        }
        Err(e) => {
            println!("Error opening file: {}. \nPress Enter to continue", e);
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
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

/// Main entry point for the file manager application
/// 
/// # Overview
/// Implements a terminal-based file manager with the following features:
/// - Directory navigation and file operations
/// - Numbered item selection interface
/// - Multiple sort options (name, size, modified date)
/// - Fuzzy search functionality
/// - File opening with custom editor selection
/// - New terminal window opening
/// 
/// # User Interface
/// - Displays current directory path
/// - Shows numbered list of files and directories
/// - Command prompt (>>) for user input
/// 
/// # Commands
/// - Numbers (1-N): Select file or directory
/// - Single letters:
///   * (q)uit: Exit application
///   * (b)ack: Go to parent directory
///   * (t)erminal: Open new terminal in current directory
///   * (n)ame: Sort by name
///   * (s)ize: Sort by size
///   * (m)odified: Sort by modification date
/// - Enter/Return: Refresh display
/// - Text input: Fuzzy search through current directory
/// 
/// # Sort Behavior
/// - Each sort command (n/s/m) toggles ascending/descending
/// - Directories are always grouped together
/// - Secondary sort maintains stable ordering
/// 
/// # Search Behavior
/// - Any input longer than one character triggers fuzzy search
/// - Shows matches within Levenshtein distance threshold
/// - Searches both filenames and directories
/// 
/// # Error Handling
/// - Handles IO errors gracefully
/// - Provides user feedback for all operations
/// - Maintains application state on recoverable errors
/// 
/// # Returns
/// * `io::Result<()>` - Success or IO error
/// 
/// # Implementation Notes
/// - Uses NavigationState to maintain UI state
/// - Updates display after each operation
/// - Handles platform-specific file operations
/// - Implements clean shutdown on quit
/// 
/// # Example Usage
/// ```text
/// >> 5        # Select item number 5
/// >> b        # Go back to parent directory
/// >> cargo    # Search for items matching "cargo"
/// >> q        # Quit application
/// ```
fn main() -> io::Result<()> {
    let mut current_directory_path = std::env::current_dir()?;
    let mut nav_state = NavigationState::new();

    loop {
        let mut directory_entries = read_directory_contents(&current_directory_path)?;
        sort_directory_entries(&mut directory_entries, nav_state.current_sort_method);
        nav_state.update_lookup_table(&directory_entries);
        display_directory_contents(&directory_entries, &current_directory_path)?;

        print!("\n>> ");
        io::stdout().flush()?;
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;

        // match process_user_input(&user_input, &nav_state)? {
        match process_user_input(&user_input, &nav_state, &directory_entries)? {
                    
            NavigationAction::Sort(command) => {
                nav_state.toggle_sort(command);
            },
            NavigationAction::OpenNewTerminal => {
                match open_new_terminal(&current_directory_path) {
                    Ok(_) => {
                        println!("Opening new terminal... Press Enter to continue");
                        let _ = io::stdin().read_line(&mut String::new());
                    }
                    Err(e) => {
                        println!("Error opening new terminal: {}. Press Enter to continue", e);
                        let _ = io::stdin().read_line(&mut String::new());
                    }
                }
            },
            
            // ... rest of the match arms remain the same ...
            NavigationAction::ChangeDirectory(new_path) => {
                current_directory_path = new_path;
            }
            NavigationAction::ParentDirectory => {
                if let Some(parent) = current_directory_path.parent() {
                    current_directory_path = parent.to_path_buf();
                }
            }
            NavigationAction::OpenFile(ref path) => {
                handle_file_open(path)?;
            }
            NavigationAction::Quit => break,
            NavigationAction::Refresh => continue,
            NavigationAction::Invalid => {
                println!("Invalid input. Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
            }
        }
    }

    Ok(())
}

