/// ff - A minimal file manager in Rust
/// Main entry point for the file manager application

/* Docs:

ff is a minimal rust file manager

A very minimal 'file manager', much more minimal than midnight commander.

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
2. primarily number + enter/return input
3. select directory to go to by number
4. enter (with no text) to scroll down
5. 'b' to 'go back; first unscroll, then back-up directory path
6. enter file to open by number
7. use config.toml to store 'open a file by type' preference lists (e.g. which editor to use for a .txt file, .csv file)
8. default to default program with another return/enter
9. open file in new terminal
10. last three items in numbered list are 'sort by size' ' 'sort by name' 'sort by last-modified'
11. re-selecting a sort option reverses the order
12. MVP: type 'r' to refresh (in future, check for changes and refresh if user not typing)
13. 'q' for quit

# Scrolling
1. MVP: a default terminal size will be the mvp,
with a known number of lines for offset and range for scrolling
2. in future a terminal size can be zoomed in or out and stored in struct for navigation_state

# Example workflow:
- open terminal
- type fff to start file manager/browser
- see list of directories and files by number
  with sort/re-sort
- select item by number
- select directory by number, that becomes next: like cd /dir
- select file by number

Size:
- show file size in terms of b kb mb or gb depending on
if the size is no more than 99 of that unit
.1 mb, 99 k, 99 b etc.

## lookup table:
- there should be a lookup table depending on how the cwd items are being displayed 
- then advance to next directory could use that maybe
- lookup table should be in navigation struct
- this  may need to also include item-type, e.g. file or directory,
when item is selected by number, the various fields (if only path and type) can be found, type can determine how to handle, e.g. go to directory path or file: handle open file
- 
*/

use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

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

/// Represents possible navigation actions based on user input
#[derive(Debug)]
enum NavigationAction {
    /// Change to specified directory
    ChangeDirectory(PathBuf),
    /// Move back to parent directory
    ParentDirectory,
    /// Open specified file
    OpenFile(PathBuf),
    /// Quit the application
    Quit,
    /// Invalid or unrecognized input
    Invalid,
    /// Refresh current display
    Refresh,
}

/// Processes user input and returns the corresponding NavigationAction
/// 
/// # Arguments
/// * `input` - The user's input string
/// * `nav_state` - Current navigation state containing lookup table
/// 
/// # Returns
/// * `io::Result<NavigationAction>` - The determined action to take
fn process_user_input(
    input: &str,
    nav_state: &NavigationState,
) -> io::Result<NavigationAction> {
    match input.trim().to_lowercase().as_str() {
        "q" => Ok(NavigationAction::Quit),
        "b" => Ok(NavigationAction::ParentDirectory),
        "" => Ok(NavigationAction::Refresh),
        input => {
            // Try to parse input as a number
            match input.parse::<usize>() {
                Ok(number) => {
                    // Look up the selected item
                    match nav_state.lookup_item(number) {
                        Some(item_info) => {
                            match item_info.item_type {
                                FileSystemItemType::Directory => {
                                    Ok(NavigationAction::ChangeDirectory(
                                        item_info.item_path.clone()
                                    ))
                                }
                                FileSystemItemType::File => {
                                    Ok(NavigationAction::OpenFile(
                                        item_info.item_path.clone()
                                    ))
                                }
                            }
                        }
                        None => Ok(NavigationAction::Invalid)
                    }
                }
                Err(_) => Ok(NavigationAction::Invalid)
            }
        }
    }
}

/// Represents an item's type in the file system
#[derive(Debug, Clone, PartialEq)]
enum FileSystemItemType {
    Directory,
    File,
}

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
}

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
        }
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

/// Specifies how directory contents should be sorted and in which direction
#[derive(Debug, Clone, Copy, PartialEq)]
enum DirectorySortingMethodEnum {
    /// Sort by file/directory name
    NameAscending,
    NameDescending,
    /// Sort by file size (directories treated as size 0)
    SizeAscending,
    SizeDescending,
    /// Sort by last modified timestamp
    DateModifiedAscending,
    DateModifiedDescending,
}

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
    print!("\x1B[2J\x1B[1;1H");
    println!(
        "{}    #>go b>back q>quit\n",
        current_directory_path.display()
    );

    println!(
        "{:>4}  {:<30} {:>10} {:>12}",
        "Num", "Name", "Size", "Modified"
    );
    println!("{}", "-".repeat(60));

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
            "{:>3}. {:<30} {:>10} {:>12}",
            entry_index + 1,
            display_name,
            size_display,
            time_display
        );
    }

    io::stdout().flush()?;

    Ok(())
}

/// Opens a file using the system's default program
/// 
/// # Arguments
/// * `file_path` - PathBuf of the file to open
/// 
/// # Returns
/// * `io::Result<()>` - Success: () unit type
///                      Error: IO error with description
/// 
/// # Platform-specific Implementation
/// - Uses 'open' on macOS
/// - Uses 'xdg-open' on Linux
/// - Uses 'start' on Windows
fn open_file(file_path: &PathBuf) -> io::Result<()> {
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
    
    Ok(())
}


// TODO update
// /// Main entry point for the fff (minimal file manager) application
// /// 
// /// # Error Handling
// /// - Handles directory access errors
// /// - Handles display errors
// /// - Handles input/output errors
// /// 
// /// # MVP Functionality
// /// - Shows current directory contents
// /// - Allows quitting with 'q'
// /// - Handles basic error cases
// /// 
// /// # Future Functionality (not in MVP)
// /// - Directory navigation
// /// - File opening
// /// - Sorting options

fn main() -> io::Result<()> {
    let mut current_directory_path = std::env::current_dir()?;
    let mut nav_state = NavigationState::new();

    loop {
        // Read and process directory contents
        let directory_entries = read_directory_contents(&current_directory_path)?;
        
        // Update the lookup table with current entries
        nav_state.update_lookup_table(&directory_entries);
        
        // Display contents
        display_directory_contents(&directory_entries, &current_directory_path)?;

        // Get user input
        print!("\n>> ");
        io::stdout().flush()?;
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;

        match process_user_input(&user_input, &nav_state)? {
            NavigationAction::ChangeDirectory(new_path) => {
                current_directory_path = new_path;
            }
            NavigationAction::ParentDirectory => {
                if let Some(parent) = current_directory_path.parent() {
                    current_directory_path = parent.to_path_buf();
                }
            }
            NavigationAction::OpenFile(ref path) => {  // Note the 'ref' keyword here
                match open_file(path) {
                    Ok(_) => {
                        println!("Opening file... Press Enter to continue");
                        let _ = io::stdin().read_line(&mut String::new());
                    }
                    Err(e) => {
                        println!("Error opening file: {}. Press Enter to continue", e);
                        let _ = io::stdin().read_line(&mut String::new());
                    }
                }
            }
            NavigationAction::Quit => break,
            NavigationAction::Refresh => continue,
            NavigationAction::Invalid => {
                println!("Invalid input. Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
            }
        }
        
        // // Process the input
        // match process_user_input(&user_input, &nav_state)? {
        //     NavigationAction::ChangeDirectory(new_path) => {
        //         current_directory_path = new_path;
        //     }
        //     NavigationAction::ParentDirectory => {
        //         if let Some(parent) = current_directory_path.parent() {
        //             current_directory_path = parent.to_path_buf();
        //         }
        //     }
        //     NavigationAction::OpenFile(_path) => {
        //         // TODO: Implement file opening in future step
        //         println!("File opening not yet implemented");
        //         let _ = io::stdin().read_line(&mut String::new());
        //     }
        //     NavigationAction::Quit => break,
        //     NavigationAction::Refresh => continue,
        //     NavigationAction::Invalid => {
        //         println!("Invalid input. Press Enter to continue...");
        //         let _ = io::stdin().read_line(&mut String::new());
        //     }
        // }
    }

    Ok(())
}