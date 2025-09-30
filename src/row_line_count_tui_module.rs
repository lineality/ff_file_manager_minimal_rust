use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Represents a file with its metadata for display
#[derive(Debug, Clone)]
struct FileLineCount {
    /// Absolute path to the file
    file_path: PathBuf,
    /// Display name (filename only)
    display_name: String,
    /// Number of lines in the file
    line_count: usize,
}

/// Sort modes for file display
#[derive(Debug, Clone, Copy, PartialEq)]
enum SortMode {
    /// Sort by filename alphabetically ascending
    NameAscending,
    /// Sort by filename alphabetically descending
    NameDescending,
    /// Sort by line count ascending
    CountAscending,
    /// Sort by line count descending
    CountDescending,
}

/// Display configuration for the TUI
#[derive(Debug, Clone)]
struct DisplayConfig {
    /// Current sort mode
    sort_mode: SortMode,
    /// Whether to include header row in line count (true = normal count, false = subtract 1)
    show_header_in_line_count: bool,
    /// Terminal width (default 80)
    terminal_width: usize,
    // Terminal height (default 24)
    // terminal_height: usize,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            sort_mode: SortMode::NameAscending,
            show_header_in_line_count: true,
            terminal_width: 80,
            // terminal_height: 24,
        }
    }
}

/// Tier 1: Exact filename matches for specific configuration files
/// Only includes files NOT covered by extension matching in Tier 2
/// These are typically files without extensions or with extensions we don't want to match broadly
static EXACT_FILENAME_LOOKUP: OnceLock<HashSet<&'static str>> = OnceLock::new();

/// Tier 2: File extension matches for general file types
/// Extensions are stored lowercase without the leading dot
/// If an extension is here, ALL files with that extension are included
static EXTENSION_LOOKUP: OnceLock<HashSet<&'static str>> = OnceLock::new();

/// Initializes and returns the exact filename lookup set
/// Only contains files that aren't covered by extension matching
///
/// # Returns
/// Reference to static HashSet containing exact filenames to match
fn get_exact_filename_lookup() -> &'static HashSet<&'static str> {
    EXACT_FILENAME_LOOKUP.get_or_init(|| {
        HashSet::from([
            // Shell configuration files (no common extension)
            ".bashrc",
            ".bash_profile",
            ".bash_logout",
            ".bash_history",
            ".bash_aliases",
            ".profile",
            ".login",
            ".logout",
            ".zshrc",
            ".zprofile",
            ".zshenv",
            ".zlogin",
            ".zlogout",
            ".fishrc",
            ".fish_profile",
            ".kshrc",
            ".tcshrc",
            ".cshrc",
            ".shrc",
            ".shinit",
            // Editor configuration files (no common extension)
            ".vimrc",
            ".gvimrc",
            ".viminfo",
            ".exrc",
            ".emacs",
            ".spacemacs",
            ".nanorc",
            ".editorconfig",
            // Git configuration files (no common extension)
            ".gitconfig",
            ".gitignore",
            ".gitattributes",
            ".gitmodules",
            ".gitmessage",
            // Linter/Formatter configuration files (no common extension)
            ".prettierrc",
            ".prettierignore",
            ".eslintrc",
            ".eslintignore",
            ".pylintrc",
            ".flake8",
            ".stylelintrc",
            ".stylelintignore",
            ".coveragerc",
            ".bandit",
            ".yapfignore",
            // Version manager files (no common extension)
            ".nvmrc",
            ".node-version",
            ".python-version",
            ".ruby-version",
            ".ruby-gemset",
            ".rvmrc",
            ".tool-versions",
            ".sdkmanrc",
            ".jvmrc",
            ".go-version",
            // Build files without extensions or with non-standard naming
            "Makefile",
            "makefile",
            "GNUmakefile",
            "BSDmakefile",
            "Dockerfile",
            "dockerfile",
            "Containerfile",
            "Jenkinsfile",
            "Vagrantfile",
            "Brewfile",
            "Rakefile",
            "Gulpfile",
            "Gruntfile",
            "gradlew", // shell script without extension
            // Lock files and specific package files not covered by extensions
            "Gemfile", // no extension
            "Pipfile", // no extension
            // Web server configuration
            ".htaccess",
            ".htpasswd",
            "Caddyfile",
            // Database configuration files with dot prefix
            ".my.cnf", // dot-prefixed version
            ".pgpass",
            // Environment files
            ".env",
            ".env.local",
            ".env.development",
            ".env.production",
            ".env.test",
            ".env.staging",
            ".env.example",
            ".env.sample",
            ".env.template",
            ".envrc",
            // Generic config names without extensions
            "config",
            "configuration",
            "settings",
            "preferences",
            // RC files and configs without extensions
            ".babelrc",
            ".postcssrc",
            // Package manager configs (no common extension)
            ".npmrc",
            ".npmignore",
            ".yarnrc",
            ".yarnignore",
            // Various ignore files (no common extension)
            ".dockerignore",
            ".slugignore",
            ".cfignore",
            ".eleventyignore",
            ".vercelignore",
            ".nowignore",
            ".claspignore",
            // Tool configuration files (no common extension)
            ".huskyrc",
            ".lintstagedrc",
            ".commitlintrc",
            ".czrc",
            ".releaserc",
            ".watchmanconfig",
        ])
    })
}

/// Initializes and returns the file extension lookup set
/// Contains all file extensions we want to include (without leading dot)
///
/// # Returns
/// Reference to static HashSet containing file extensions to match
fn get_extension_lookup() -> &'static HashSet<&'static str> {
    EXTENSION_LOOKUP.get_or_init(|| {
        HashSet::from([
            // Programming languages
            "py",
            "pyw",
            "pyx",
            "pyi", // Python
            "rs",
            "rlib", // Rust
            "js",
            "mjs",
            "cjs",
            "jsx", // JavaScript
            "ts",
            "tsx",
            "mts",
            "cts", // TypeScript
            "java",
            "class", // Java
            "go",    // Go
            "rb",
            "erb",
            "rake", // Ruby
            "php",
            "phtml", // PHP
            "c",
            "h", // C
            "cpp",
            "cc",
            "cxx",
            "hpp",
            "hh",
            "hxx", // C++
            "cs",
            "csx",   // C#
            "swift", // Swift
            "kt",
            "kts", // Kotlin
            "scala",
            "sc", // Scala
            "r",
            "rdata",
            "rds",
            "rda", // R
            "m",
            "mm", // Objective-C
            "f90",
            "f95",
            "f03",
            "f08", // Fortran
            "lua", // Lua
            "pl",
            "pm", // Perl
            "jl", // Julia
            "nim",
            "nims", // Nim
            "cr",   // Crystal
            "dart", // Dart
            "elm",  // Elm
            "ex",
            "exs", // Elixir
            "erl",
            "hrl", // Erlang
            "fs",
            "fsi",
            "fsx", // F#
            "ml",
            "mli", // OCaml
            "clj",
            "cljs",
            "cljc", // Clojure
            "rkt",  // Racket
            "scm",
            "ss", // Scheme
            "lisp",
            "lsp",
            "cl", // Lisp
            "hs",
            "lhs", // Haskell
            "v",   // V/Verilog
            "vhd",
            "vhdl", // VHDL
            "pas",
            "pp", // Pascal
            "d",
            "di",  // D
            "zig", // Zig
            "ada",
            "adb",
            "ads", // Ada
            "cob",
            "cbl", // COBOL
            "asm",
            "s", // Assembly
            "wat",
            "wasm", // WebAssembly
            // Web/Markup
            "html",
            "htm",
            "xhtml", // HTML
            "css",   // CSS
            "scss",
            "sass", // Sass
            "less", // Less
            "styl",
            "stylus", // Stylus
            "vue",    // Vue
            "svelte", // Svelte
            // Data/Config formats
            "json",
            "jsonc",
            "json5", // JSON variants
            "xml",
            "xsd",
            "xsl",
            "xslt", // XML
            "yaml",
            "yml", // YAML
            "toml",
            "tml", // TOML
            "ini",
            "cfg",
            "cnf",
            "conf",
            "config",     // Config files
            "properties", // Java properties
            "env",        // Environment files
            // Shell/Script
            "sh",
            "bash",
            "zsh",
            "fish",
            "ksh",
            "tcsh",
            "csh", // Shell scripts
            "ps1",
            "psm1",
            "psd1", // PowerShell
            "bat",
            "cmd", // Windows batch
            "awk", // AWK
            // Editor/IDE
            "vim",
            "nvim", // Vim
            "el",
            "elc", // Emacs Lisp
            // Documentation
            "md",
            "markdown",
            "mdown", // Markdown
            "rst",
            "rest", // reStructuredText
            "txt",
            "text", // Plain text
            "adoc",
            "asciidoc", // AsciiDoc
            "tex",
            "latex",   // LaTeX
            "org",     // Org mode
            "textile", // Textile
            // Database
            "sql",
            "mysql",
            "psql", // SQL
            "sqlite",
            "sqlite3",
            "db",  // SQLite
            "cql", // Cassandra
            // Build/Project files
            "gradle", // Gradle
            "sbt",    // SBT
            "cmake",  // CMake
            "make",
            "mak",
            "mk",    // Make
            "ninja", // Ninja
            "bazel",
            "bzl", // Bazel
            "pro",
            "pri", // Qt project
            // Data files
            "csv",
            "tsv",
            "psv", // Delimited data
            "log", // Log files
            // Package/Project files with extensions
            "csproj",
            "vbproj",
            "fsproj", // .NET projects
            "sln",    // Visual Studio solution
            "xcodeproj",
            "xcworkspace", // Xcode
            "lock",        // Various lock files
        ])
    })
}

/// Checks if a file should be included based on exact name or extension
///
/// # Arguments
/// * `file_path` - Path to the file to check
///
/// # Returns
/// * `true` if the file matches either tier 1 (exact name) or tier 2 (extension)
/// * `false` otherwise
///
/// # Examples
/// ```
/// use std::path::Path;
/// assert!(is_supported_file(Path::new(".bashrc")));       // Exact match
/// assert!(is_supported_file(Path::new("main.rs")));       // Extension match
/// assert!(is_supported_file(Path::new("config")));        // Exact match
/// assert!(!is_supported_file(Path::new("binary.exe")));   // No match
/// ```
fn is_supported_file(file_path: &Path) -> bool {
    // Get the filename for exact matching
    let filename = match file_path.file_name() {
        Some(name) => name.to_string_lossy(),
        None => return false,
    };

    // Tier 1: Check exact filename match
    let exact_filenames = get_exact_filename_lookup();
    if exact_filenames.contains(filename.as_ref()) {
        return true;
    }

    // Tier 2: Check extension match
    if let Some(ext) = file_path.extension() {
        let extension = ext.to_string_lossy().to_lowercase();
        let extensions = get_extension_lookup();
        if extensions.contains(extension.as_str()) {
            return true;
        }
    }

    false
}

/// Scans directory and returns list of supported code/config files with line counts
///
/// # Arguments
/// * `directory_path` - Absolute path to directory to scan
///
/// # Returns
/// * `Vec<FileLineCount>` - List of files with their line counts
///
/// # Note
/// Skips files that cannot be processed (permissions, corruption, etc.)
/// and continues processing remaining files
fn get_code_files_with_counts(directory_path: &Path) -> Vec<FileLineCount> {
    let mut file_counts = Vec::new();

    // Read directory contents with error handling
    let entries = match fs::read_dir(directory_path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!(
                "Error reading directory {}: {}",
                directory_path.display(),
                e
            );
            return file_counts;
        }
    };

    // Process each entry in the directory
    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading directory entry: {}", e);
                continue; // Skip problematic entries
            }
        };

        let file_path = entry.path();

        // Skip directories, only process files
        if !file_path.is_file() {
            continue;
        }

        // Check if file is supported using two-tier system
        if !is_supported_file(&file_path) {
            continue;
        }

        // Get absolute path
        let absolute_path = match file_path.canonicalize() {
            Ok(path) => path,
            Err(e) => {
                eprintln!(
                    "Cannot get absolute path for {}: {}",
                    file_path.display(),
                    e
                );
                continue; // Skip files we can't resolve
            }
        };

        // Extract display name (filename only)
        let display_name = match absolute_path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                eprintln!("Cannot extract filename from {}", absolute_path.display());
                continue;
            }
        };

        // Count lines in file
        match count_file_lines_efficiently(&absolute_path) {
            Ok(line_count) => {
                file_counts.push(FileLineCount {
                    file_path: absolute_path,
                    display_name,
                    line_count,
                });
            }
            Err(e) => {
                eprintln!("Skipping file due to error: {}", e);
                continue; // Skip files we can't count
            }
        }
    }

    file_counts
}

/// Validates that the given path exists and is a readable directory
///
/// # Arguments
/// * `directory_path` - The path to validate
///
/// # Returns
/// * `Ok(PathBuf)` - Absolute path if valid directory
/// * `Err(String)` - Error message if validation fails
///
/// # Examples
/// ```
/// use std::path::Path;
/// let result = validate_directory_path(Path::new("."));
/// assert!(result.is_ok());
/// ```
fn validate_directory_path(directory_path: &Path) -> Result<PathBuf, String> {
    // Check if path exists
    if !directory_path.exists() {
        return Err(format!(
            "Directory does not exist: {}",
            directory_path.display()
        ));
    }

    // Check if it's actually a directory
    if !directory_path.is_dir() {
        return Err(format!(
            "Path is not a directory: {}",
            directory_path.display()
        ));
    }

    // Convert to absolute path
    match directory_path.canonicalize() {
        Ok(absolute_path) => Ok(absolute_path),
        Err(e) => Err(format!(
            "Cannot access directory {}: {}",
            directory_path.display(),
            e
        )),
    }
}

/// Efficiently counts lines in a file without loading entire content into memory
///
/// # Arguments
/// * `file_path` - Absolute path to the file to count
///
/// # Returns
/// * `Ok(usize)` - Number of lines in the file
/// * `Err(String)` - Error message if counting fails
///
/// # Examples
/// ```
/// use std::path::Path;
/// let count = count_file_lines_efficiently(Path::new("/absolute/path/to/file.txt"));
/// ```
fn count_file_lines_efficiently(file_path: &Path) -> Result<usize, String> {
    // Open file with error handling
    let file = match fs::File::open(file_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Cannot open file {}: {}", file_path.display(), e)),
    };

    // Use BufReader for memory-efficient line counting
    let reader = BufReader::new(file);
    let mut line_count = 0;

    // Count lines using iterator to avoid loading entire file
    for line_result in reader.lines() {
        match line_result {
            Ok(_) => line_count += 1,
            Err(e) => {
                // Log error but continue counting what we can
                eprintln!(
                    "Warning: Error reading line in {}: {}",
                    file_path.display(),
                    e
                );
                continue;
            }
        }
    }

    Ok(line_count)
}

/// Sorts file list according to specified sort mode
///
/// # Arguments
/// * `files` - Mutable reference to vector of FileLineCount
/// * `sort_mode` - How to sort the files
///
/// # Note
/// Sorts in-place for memory efficiency
fn sort_files_by_mode(files: &mut Vec<FileLineCount>, sort_mode: SortMode) {
    match sort_mode {
        SortMode::NameAscending => {
            files.sort_by(|a, b| a.display_name.cmp(&b.display_name));
        }
        SortMode::NameDescending => {
            files.sort_by(|a, b| b.display_name.cmp(&a.display_name));
        }
        SortMode::CountAscending => {
            files.sort_by(|a, b| a.line_count.cmp(&b.line_count));
        }
        SortMode::CountDescending => {
            files.sort_by(|a, b| b.line_count.cmp(&a.line_count));
        }
    }
}

/// Formats and displays the TUI with file information
///
/// # Arguments
/// * `files` - List of files to display
/// * `config` - Display configuration (sort mode, header visibility, etc.)
///
/// # Note
/// Handles terminal size constraints gracefully
fn display_file_list_tui(files: &[FileLineCount], config: &DisplayConfig) {
    // Clear screen (simple approach)
    print!("\x1B[2J\x1B[H");
    let _ = io::stdout().flush();

    // Display header/legend always
    println!(
        "# File Name{:width$}Line Count",
        "",
        width = config.terminal_width.saturating_sub(25)
    );

    // Display files up to terminal height limit
    for (index, file) in files.iter().enumerate() {
        // Verify file still exists before displaying
        if !file.file_path.exists() {
            eprintln!(
                "Warning: File no longer exists: {}",
                file.file_path.display()
            );
            continue;
        }

        // Format: "# filename                    line_count"
        let line_number = index + 1;
        let available_width = config.terminal_width.saturating_sub(10); // Reserve space for numbers and count

        // Truncate filename if necessary to fit terminal width
        let display_name = if file.display_name.len() > available_width {
            format!(
                "{}...",
                &file.display_name[..available_width.saturating_sub(3)]
            )
        } else {
            file.display_name.clone()
        };

        // Calculate padding for right-alignment of line count
        let padding_width = config
            .terminal_width
            .saturating_sub(line_number.to_string().len())
            .saturating_sub(2) // "# "
            .saturating_sub(display_name.len())
            .saturating_sub(file.line_count.to_string().len());

        println!(
            "{} {}{:width$}{}",
            line_number,
            display_name,
            "",
            if config.show_header_in_line_count {
                file.line_count
            } else {
                file.line_count.saturating_sub(1)
            },
            width = padding_width
        );
    }

    // Show command prompt
    print!("\nCommands: (n)ame sort, (c)ount sort, (h)eader, (Enter) reset, (b/q)uit: ");
    let _ = io::stdout().flush();
}

/// Processes user input commands and updates display configuration
///
/// # Arguments
/// * `config` - Mutable reference to display configuration
///
/// # Returns
/// * `Ok(true)` - Continue program execution
/// * `Ok(false)` - Exit program (user chose quit)
/// * `Err(String)` - Error reading input
fn handle_user_command(config: &mut DisplayConfig) -> Result<bool, String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let command = input.trim().to_lowercase();

            match command.as_str() {
                // Exit commands
                "q" | "b" => return Ok(false),

                // Name sort toggle
                "n" => {
                    config.sort_mode = match config.sort_mode {
                        SortMode::NameAscending => SortMode::NameDescending,
                        _ => SortMode::NameAscending,
                    };
                }

                // Count sort toggle
                "c" => {
                    config.sort_mode = match config.sort_mode {
                        SortMode::CountAscending => SortMode::CountDescending,
                        _ => SortMode::CountAscending,
                    };
                }

                // Toggle header visibility
                "h" => {
                    config.show_header_in_line_count = !config.show_header_in_line_count;
                }

                // Reset to default (empty string = just Enter pressed)
                "" => {
                    config.sort_mode = SortMode::NameAscending;
                    config.show_header_in_line_count = true;
                }

                // Unknown command - ignore and continue
                _ => {
                    // Silently ignore unknown commands per requirements
                }
            }

            Ok(true) // Continue execution
        }
        Err(e) => Err(format!("Error reading user input: {}", e)),
    }
}

/// Main TUI loop that coordinates file scanning, display, and user interaction
///
/// # Arguments
/// * `directory_path` - Absolute path to directory to scan
///
/// # Returns
/// * `Ok(())` - Program completed successfully
/// * `Err(String)` - Error occurred during execution
///
pub fn show_minimal_linecount_tui(directory_path: &Path) -> Result<(), String> {
    // Validate directory path
    let absolute_directory_path = validate_directory_path(directory_path)?;

    // Initialize display configuration
    let mut display_config = DisplayConfig::default();

    // Main program loop
    loop {
        // Scan directory for code files (re-scan each iteration to catch file changes)
        let mut files = get_code_files_with_counts(&absolute_directory_path);

        // Sort files according to current mode
        sort_files_by_mode(&mut files, display_config.sort_mode);

        // Display the TUI
        display_file_list_tui(&files, &display_config);

        // Process user command
        match handle_user_command(&mut display_config) {
            Ok(continue_program) => {
                if !continue_program {
                    break; // User chose to exit
                }
            }
            Err(e) => {
                eprintln!("Input error: {}", e);
                // Continue program despite input errors
                continue;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod linecount_tui_tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    /// Creates test files in a test_temp directory for use by multiple tests
    ///
    /// # Purpose
    /// Provides a consistent set of test files with known content and line counts
    /// that can be used to verify file processing functions work correctly.
    ///
    /// # Implementation Details
    /// - Creates a directory named "test_temp" in the current working directory
    /// - Only creates files if they don't already exist (preserves existing test files)
    /// - Creates 8 test files with various extensions (some supported, some not)
    ///
    /// # Test Files Created
    /// - test.rs: 4 lines of Rust code
    /// - data.csv: 3 lines of CSV data
    /// - script.py: 2 lines of Python code
    /// - readme.txt: 3 lines of text
    /// - config.json: 4 lines of JSON
    /// - styles.css: 4 lines of CSS
    /// - binary.exe: 1 line (unsupported file type)
    /// - no_extension: 1 line (file without extension)
    ///
    /// # Returns
    /// Absolute path to the test_temp directory
    ///
    /// # Panics
    /// Panics if unable to create directory or files (test setup failure)
    fn setup_test_files() -> PathBuf {
        // Create path to test directory in current working directory
        let test_dir = PathBuf::from("test_temp");

        // Create directory if it doesn't exist
        // We check first to avoid errors on subsequent test runs
        if !test_dir.exists() {
            fs::create_dir(&test_dir).expect("Failed to create test_temp directory");
        }

        // Define test files with known content and line counts
        // These cover various supported file types and some unsupported ones
        let files_to_create = [
            (
                "test.rs",
                "// Rust file\nfn main() {\n    println!(\"Hello\");\n}",
            ),
            ("data.csv", "name,age\nAlice,25\nBob,30"),
            ("script.py", "print('Python')\nprint('Script')"),
            (
                "readme.txt",
                "This is a readme file.\nSecond line.\nThird line.",
            ),
            (
                "config.json",
                "{\n  \"key\": \"value\",\n  \"number\": 42\n}",
            ),
            ("styles.css", "body {\n  margin: 0;\n  padding: 0;\n}"),
            ("binary.exe", "binary content"), // Unsupported extension for testing filtering
            ("no_extension", "content without extension"), // No extension for testing filtering
        ];

        // Create each test file if it doesn't already exist
        for (filename, content) in &files_to_create {
            let file_path = test_dir.join(filename);

            // Only create file if it doesn't exist to preserve any existing test files
            // This prevents overwriting files with empty content on repeated test runs
            if !file_path.exists() {
                // Create and write the file
                let mut file =
                    fs::File::create(&file_path).expect(&format!("Failed to create {}", filename));
                file.write_all(content.as_bytes())
                    .expect(&format!("Failed to write {}", filename));
            }
        }

        // Return absolute path for consistent path handling across tests
        test_dir
            .canonicalize()
            .expect("Failed to get absolute path for test_temp")
    }

    /// Tests that validate_directory_path accepts valid directories and returns absolute paths
    ///
    /// # Purpose
    /// Verifies that the validate_directory_path function correctly validates
    /// existing directories and returns their absolute paths.
    ///
    /// # Test Strategy
    /// 1. Creates a test directory with test files
    /// 2. Passes this directory to validate_directory_path
    /// 3. Verifies the function returns Ok with an absolute path
    ///
    /// # Expected Behavior
    /// - Function returns Ok(PathBuf) for valid directory
    /// - Returned path is absolute (not relative)
    #[test]
    fn test_validate_directory_path_valid() {
        // Setup test directory with files
        let test_dir = setup_test_files();

        // Test validation of existing directory
        let result = validate_directory_path(&test_dir);

        // Verify successful validation
        assert!(result.is_ok(), "Valid directory should pass validation");

        // Verify absolute path is returned
        let validated_path = result.unwrap();
        assert!(validated_path.is_absolute(), "Should return absolute path");
    }

    /// Tests that validate_directory_path rejects nonexistent paths
    ///
    /// # Purpose
    /// Verifies that the function properly detects and reports when
    /// a directory path doesn't exist on the filesystem.
    ///
    /// # Test Strategy
    /// 1. Creates a path to a directory that definitely doesn't exist
    /// 2. Attempts to validate this nonexistent path
    /// 3. Verifies an appropriate error is returned
    ///
    /// # Expected Behavior
    /// - Function returns Err for nonexistent path
    /// - Error message contains "does not exist"
    #[test]
    fn test_validate_directory_path_nonexistent() {
        // Create path that definitely doesn't exist
        let fake_path = PathBuf::from("this_definitely_does_not_exist_12345");

        // Attempt to validate nonexistent path
        let result = validate_directory_path(&fake_path);

        // Verify appropriate error is returned
        assert!(result.is_err(), "Nonexistent path should fail");
        assert!(
            result.unwrap_err().contains("does not exist"),
            "Error should mention path doesn't exist"
        );
    }

    /// Tests that validate_directory_path rejects file paths (not directories)
    ///
    /// # Purpose
    /// Verifies that the function distinguishes between files and directories,
    /// rejecting paths that point to files rather than directories.
    ///
    /// # Test Strategy
    /// 1. Creates test directory with files
    /// 2. Gets path to a file (not directory) within test directory
    /// 3. Attempts to validate this file path as a directory
    /// 4. Verifies appropriate error is returned
    ///
    /// # Expected Behavior
    /// - Function returns Err when given a file path
    /// - Error message indicates path is "not a directory"
    #[test]
    fn test_validate_directory_path_file_not_directory() {
        // Setup test directory and get path to a file
        let test_dir = setup_test_files();
        let file_path = test_dir.join("test.rs");

        // Try to validate a file as directory
        let result = validate_directory_path(&file_path);

        // Verify appropriate error for file vs directory
        assert!(
            result.is_err(),
            "File path should fail directory validation"
        );
        assert!(
            result.unwrap_err().contains("not a directory"),
            "Error should mention it's not a directory"
        );
    }

    /// Tests that is_supported_file_type correctly identifies supported file extensions
    ///
    /// # Purpose
    /// Verifies that the function correctly identifies all file types that
    /// the application should process (code and data files).
    ///
    /// # Test Strategy
    /// Tests a representative sample of supported extensions including:
    /// - Programming languages: .rs, .py, .js
    /// - Data formats: .csv, .json, .xml
    /// - Text formats: .txt, .md
    /// - Web formats: .html, .css
    ///
    /// # Expected Behavior
    /// All tested extensions should return true
    #[test]
    fn test_is_supported_file_type_supported_extensions() {
        // Test various supported file extensions
        let supported_files = [
            "test.rs",     // Rust
            "data.csv",    // CSV data
            "script.py",   // Python
            "config.json", // JSON
            "styles.css",  // CSS
            "readme.txt",  // Text
            "query.sql",   // SQL
            "index.html",  // HTML
        ];

        // Verify each file type is recognized as supported
        for filename in &supported_files {
            let path = Path::new(filename);
            assert!(is_supported_file(path), "{} should be supported", filename);
        }
    }

    /// Tests that is_supported_file_type correctly rejects unsupported file extensions
    ///
    /// # Purpose
    /// Verifies that the function correctly filters out file types that
    /// shouldn't be processed (binaries, media files, files without extensions).
    ///
    /// # Test Strategy
    /// Tests various unsupported file types including:
    /// - Binary files: .exe
    /// - Media files: .png, .jpg, .mp4, .mp3
    /// - Archives: .zip
    /// - Documents: .pdf
    /// - Files without extensions
    /// - Hidden files (starting with .)
    ///
    /// # Expected Behavior
    /// All tested file types should return false
    #[test]
    fn test_is_supported_file_type_unsupported_extensions() {
        // Test various unsupported file types
        let unsupported_files = [
            "binary.exe",   // Windows executable
            "image.png",    // Image file
            "video.mp4",    // Video file
            "no_extension", // File without extension
            ".hidden",      // Hidden file
        ];

        // Verify each file type is recognized as unsupported
        for filename in &unsupported_files {
            let path = Path::new(filename);
            assert!(
                !is_supported_file(path),
                "{} should not be supported",
                filename
            );
        }
    }

    /// Tests that count_file_lines_efficiently correctly counts lines in various file types
    ///
    /// # Purpose
    /// Verifies that the line counting function accurately counts lines
    /// in different types of text files with various line ending styles.
    ///
    /// # Test Strategy
    /// 1. Creates test files with known line counts
    /// 2. Counts lines in each file
    /// 3. Verifies counts match expected values
    ///
    /// # Test Cases
    /// - test.rs: 4 lines (Rust code with standard line endings)
    /// - data.csv: 3 lines (CSV with header and data rows)
    /// - script.py: 2 lines (Python script)
    /// - readme.txt: 3 lines (Plain text)
    /// - config.json: 4 lines (JSON with formatting)
    /// - styles.css: 4 lines (CSS with block structure)
    ///
    /// # Expected Behavior
    /// Each file's line count should match the known number of lines
    #[test]
    fn test_count_file_lines_efficiently_various_files() {
        // Setup test files
        let test_dir = setup_test_files();

        // Define expected line counts for each test file
        // These match the actual content created in setup_test_files()
        let test_cases = [
            ("test.rs", 4),     // "// Rust file\nfn main() {\n    println!(\"Hello\");\n}"
            ("data.csv", 3),    // "name,age\nAlice,25\nBob,30"
            ("script.py", 2),   // "print('Python')\nprint('Script')"
            ("readme.txt", 3),  // "This is a readme file.\nSecond line.\nThird line."
            ("config.json", 4), // "{\n  \"key\": \"value\",\n  \"number\": 42\n}"
            ("styles.css", 4),  // "body {\n  margin: 0;\n  padding: 0;\n}"
        ];

        // Test line counting for each file
        for (filename, expected_lines) in &test_cases {
            let file_path = test_dir.join(filename);

            // Count lines in the file
            let result = count_file_lines_efficiently(&file_path);
            assert!(result.is_ok(), "Should count lines in {}", filename);

            // Verify count matches expected value
            let actual_lines = result.unwrap();
            assert_eq!(
                actual_lines, *expected_lines,
                "Wrong line count for {}",
                filename
            );
        }
    }

    /// Tests that count_file_lines_efficiently properly handles nonexistent files
    ///
    /// # Purpose
    /// Verifies that the function returns an appropriate error when
    /// attempting to count lines in a file that doesn't exist.
    ///
    /// # Test Strategy
    /// 1. Attempts to count lines in a nonexistent file
    /// 2. Verifies an error is returned
    /// 3. Checks that error message is descriptive
    ///
    /// # Expected Behavior
    /// - Function returns Err for nonexistent file
    /// - Error message contains "Cannot open file"
    #[test]
    fn test_count_file_lines_efficiently_nonexistent_file() {
        // Attempt to count lines in nonexistent file
        let fake_file = PathBuf::from("nonexistent.txt");
        let result = count_file_lines_efficiently(&fake_file);

        // Verify appropriate error handling
        assert!(result.is_err(), "Should fail for nonexistent file");
        assert!(
            result.unwrap_err().contains("Cannot open file"),
            "Error should mention cannot open file"
        );
    }

    /// Tests that get_code_files_with_counts correctly scans directories and counts lines
    ///
    /// # Purpose
    /// Verifies the main file scanning function that combines:
    /// - Directory traversal
    /// - File type filtering
    /// - Line counting
    /// - Result aggregation
    ///
    /// # Test Strategy
    /// 1. Creates directory with mix of supported and unsupported files
    /// 2. Scans directory with get_code_files_with_counts
    /// 3. Verifies correct files are included/excluded
    /// 4. Verifies line counts are accurate
    ///
    /// # Expected Behavior
    /// - Function returns exactly 6 supported files (excluding .exe and no_extension)
    /// - All returned files have positive line counts
    /// - Specific files (test.rs, data.csv, etc.) are present
    /// - Unsupported files (binary.exe, no_extension) are excluded
    #[test]
    fn test_get_code_files_with_counts() {
        // Setup test directory with mixed file types
        let test_dir = setup_test_files();

        // Scan directory for code files
        let files = get_code_files_with_counts(&test_dir);

        // Verify correct number of files found
        // Should find 6 supported files (excluding binary.exe and no_extension)
        assert_eq!(files.len(), 6, "Should find 6 supported files");

        // Verify all files have positive line counts
        for file in &files {
            assert!(
                file.line_count > 0,
                "File {} should have positive line count",
                file.display_name
            );
        }

        // Verify expected files are present
        let expected_names = [
            "test.rs",
            "data.csv",
            "script.py",
            "readme.txt",
            "config.json",
            "styles.css",
        ];

        for expected in &expected_names {
            assert!(
                files.iter().any(|f| f.display_name == *expected),
                "Should find {}",
                expected
            );
        }

        // Verify unsupported files are excluded
        let excluded_names = ["binary.exe", "no_extension"];

        for excluded in &excluded_names {
            assert!(
                !files.iter().any(|f| f.display_name == *excluded),
                "Should not include {}",
                excluded
            );
        }
    }

    /// Tests that get_code_files_with_counts correctly handles empty directories
    ///
    /// # Purpose
    /// Verifies that the function gracefully handles the edge case of scanning
    /// a directory that exists but contains no files.
    ///
    /// # Test Strategy
    /// 1. Creates an empty directory named "test_empty"
    /// 2. Calls get_code_files_with_counts on this empty directory
    /// 3. Verifies an empty vector is returned (not an error)
    ///
    /// # Expected Behavior
    /// - Function should not panic or error
    /// - Function should return an empty Vec<FileLineCount>
    /// - Empty directory is treated as normal condition, not an error
    ///
    /// # Why This Test Matters
    /// Empty directories are common in real usage (new projects, cleaned directories).
    /// The function must handle this case without crashing or reporting errors.
    #[test]
    fn test_get_code_files_with_counts_empty_directory() {
        // Create an empty directory for testing
        let empty_dir = PathBuf::from("test_empty");

        // Only create if doesn't exist (preserve across test runs)
        if !empty_dir.exists() {
            fs::create_dir(&empty_dir).expect("Failed to create empty dir");
        }

        // Scan the empty directory
        let files = get_code_files_with_counts(&empty_dir);

        // Verify empty directory returns empty results
        assert_eq!(files.len(), 0, "Empty directory should return no files");
    }

    /// Tests that sort_files_by_mode correctly sorts by name in ascending order
    ///
    /// # Purpose
    /// Verifies alphabetical sorting of files by display name (A to Z).
    ///
    /// # Test Strategy
    /// 1. Creates vector of FileLineCount with unsorted names
    /// 2. Applies NameAscending sort
    /// 3. Verifies files are in alphabetical order
    ///
    /// # Expected Behavior
    /// Files should be ordered: apple.py, mango.csv, zebra.rs
    #[test]
    fn test_sort_files_by_mode_name_ascending() {
        // Create unsorted file list
        let mut files = vec![
            FileLineCount {
                file_path: PathBuf::from("/test/zebra.rs"),
                display_name: "zebra.rs".to_string(),
                line_count: 10,
            },
            FileLineCount {
                file_path: PathBuf::from("/test/apple.py"),
                display_name: "apple.py".to_string(),
                line_count: 5,
            },
            FileLineCount {
                file_path: PathBuf::from("/test/mango.csv"),
                display_name: "mango.csv".to_string(),
                line_count: 15,
            },
        ];

        // Apply ascending name sort
        sort_files_by_mode(&mut files, SortMode::NameAscending);

        // Verify alphabetical order (A to Z)
        assert_eq!(files[0].display_name, "apple.py");
        assert_eq!(files[1].display_name, "mango.csv");
        assert_eq!(files[2].display_name, "zebra.rs");
    }

    /// Tests that sort_files_by_mode correctly sorts by name in descending order
    ///
    /// # Purpose
    /// Verifies reverse alphabetical sorting of files by display name (Z to A).
    ///
    /// # Test Strategy
    /// 1. Creates vector of FileLineCount with unsorted names
    /// 2. Applies NameDescending sort
    /// 3. Verifies files are in reverse alphabetical order
    ///
    /// # Expected Behavior
    /// Files should be ordered: zebra.rs, apple.py
    #[test]
    fn test_sort_files_by_mode_name_descending() {
        // Create unsorted file list
        let mut files = vec![
            FileLineCount {
                file_path: PathBuf::from("/test/apple.py"),
                display_name: "apple.py".to_string(),
                line_count: 5,
            },
            FileLineCount {
                file_path: PathBuf::from("/test/zebra.rs"),
                display_name: "zebra.rs".to_string(),
                line_count: 10,
            },
        ];

        // Apply descending name sort
        sort_files_by_mode(&mut files, SortMode::NameDescending);

        // Verify reverse alphabetical order (Z to A)
        assert_eq!(files[0].display_name, "zebra.rs");
        assert_eq!(files[1].display_name, "apple.py");
    }

    /// Tests that sort_files_by_mode correctly sorts by line count in ascending order
    ///
    /// # Purpose
    /// Verifies numerical sorting of files by line count (smallest to largest).
    ///
    /// # Test Strategy
    /// 1. Creates vector of FileLineCount with various line counts
    /// 2. Applies CountAscending sort
    /// 3. Verifies files are ordered by increasing line count
    ///
    /// # Expected Behavior
    /// Files should be ordered by line count: 5, 50, 100
    #[test]
    fn test_sort_files_by_mode_count_ascending() {
        // Create file list with various line counts
        let mut files = vec![
            FileLineCount {
                file_path: PathBuf::from("/test/large.rs"),
                display_name: "large.rs".to_string(),
                line_count: 100,
            },
            FileLineCount {
                file_path: PathBuf::from("/test/small.py"),
                display_name: "small.py".to_string(),
                line_count: 5,
            },
            FileLineCount {
                file_path: PathBuf::from("/test/medium.csv"),
                display_name: "medium.csv".to_string(),
                line_count: 50,
            },
        ];

        // Apply ascending count sort
        sort_files_by_mode(&mut files, SortMode::CountAscending);

        // Verify numerical order (smallest to largest)
        assert_eq!(files[0].line_count, 5);
        assert_eq!(files[1].line_count, 50);
        assert_eq!(files[2].line_count, 100);
    }

    /// Tests that sort_files_by_mode correctly sorts by line count in descending order
    ///
    /// # Purpose
    /// Verifies numerical sorting of files by line count (largest to smallest).
    ///
    /// # Test Strategy
    /// 1. Creates vector of FileLineCount with various line counts
    /// 2. Applies CountDescending sort
    /// 3. Verifies files are ordered by decreasing line count
    ///
    /// # Expected Behavior
    /// Files should be ordered by line count: 100, 5
    #[test]
    fn test_sort_files_by_mode_count_descending() {
        // Create file list with various line counts
        let mut files = vec![
            FileLineCount {
                file_path: PathBuf::from("/test/small.py"),
                display_name: "small.py".to_string(),
                line_count: 5,
            },
            FileLineCount {
                file_path: PathBuf::from("/test/large.rs"),
                display_name: "large.rs".to_string(),
                line_count: 100,
            },
        ];

        // Apply descending count sort
        sort_files_by_mode(&mut files, SortMode::CountDescending);

        // Verify reverse numerical order (largest to smallest)
        assert_eq!(files[0].line_count, 100);
        assert_eq!(files[1].line_count, 5);
    }

    /// Tests that DisplayConfig::default() returns expected default values
    ///
    /// # Purpose
    /// Verifies that the Default implementation for DisplayConfig
    /// produces the expected initial configuration values.
    ///
    /// # Expected Default Values
    /// - sort_mode: NameAscending (alphabetical by default)
    /// - show_header_in_line_count: true (header visible in line count)
    /// - terminal_width: 80 (standard terminal width)
    /// - terminal_height: 24 (standard terminal height)
    #[test]
    fn test_display_config_default() {
        // Create default configuration
        let config = DisplayConfig::default();

        // Verify all default values
        assert_eq!(config.sort_mode, SortMode::NameAscending);
        assert_eq!(config.show_header_in_line_count, true);
        assert_eq!(config.terminal_width, 80);
        // assert_eq!(config.terminal_height, 24);
    }

    /// Tests FileLineCount struct creation and cloning
    ///
    /// # Purpose
    /// Verifies that the FileLineCount struct correctly stores data
    /// and implements Clone trait properly.
    ///
    /// # Test Strategy
    /// 1. Creates a FileLineCount instance with test data
    /// 2. Verifies all fields are stored correctly
    /// 3. Tests that clone() produces an identical copy
    ///
    /// # Expected Behavior
    /// - Struct stores all fields correctly
    /// - Clone produces identical copy with same values
    #[test]
    fn test_file_line_count_struct() {
        // Create FileLineCount instance
        let file = FileLineCount {
            file_path: PathBuf::from("/test/example.rs"),
            display_name: "example.rs".to_string(),
            line_count: 42,
        };

        // Verify fields are stored correctly
        assert_eq!(file.display_name, "example.rs");
        assert_eq!(file.line_count, 42);

        // Test clone implementation
        let cloned = file.clone();
        assert_eq!(cloned.display_name, file.display_name);
        assert_eq!(cloned.line_count, file.line_count);
    }

    /// Tests SortMode enum equality and inequality
    ///
    /// # Purpose
    /// Verifies that the SortMode enum correctly implements
    /// PartialEq for comparing sort modes.
    ///
    /// # Test Strategy
    /// Tests various equality and inequality comparisons between
    /// different SortMode variants.
    ///
    /// # Expected Behavior
    /// - Same variants should be equal
    /// - Different variants should not be equal
    #[test]
    fn test_sort_mode_enum() {
        // Test equality - same variants should be equal
        assert_eq!(SortMode::NameAscending, SortMode::NameAscending);

        // Test inequality - different variants should not be equal
        assert_ne!(SortMode::NameAscending, SortMode::NameDescending);
        assert_ne!(SortMode::CountAscending, SortMode::CountDescending);
    }
}
