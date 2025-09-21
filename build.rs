//! build.rs - Build script for embedding compile-time information
//!
//! This script runs at compile time and generates build information that gets
//! embedded into the final binary. It collects:
//! - Build timestamp (when the binary was compiled)
//! - Operating system type
//! - Target architecture
//! - Rust edition from Cargo.toml
//! - Rust compiler version
//! - Build profile (debug/release)
//! - Package version
//!
//! The information is written to a generated file that can be included in the main program.

/*
your-project/
├── Cargo.toml
├── build.rs          <-- Must be here, at the root
└── src/
    └── main.rs


use in ~main with


above main:


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
    args.iter().any(|arg| arg == "--version" || arg == "-v")
}

/// Writes version information to stdout.
///
/// # Returns
///
/// * `Ok(())` if version information was successfully written
/// * `Err(FileFantasticError)` if writing or flushing failed
///
/// # Errors
///
/// Returns an error if stdout is not available or writing fails
/// (e.g., broken pipe, redirected to read-only location)
pub fn display_version() -> Result<()> {
    let mut stdout = io::stdout();

    // Write version statement
    writeln!(stdout, "{}", format_version_info()).map_err(|e| FileFantasticError::Io(e))?;

    // Ensure output is flushed
    stdout.flush().map_err(|e| FileFantasticError::Io(e))?;

    Ok(())
}


in ~main

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

 */

/*
About:
Build Information System Documentation
Overview
This build information system provides compile-time metadata embedding for Rust binaries, allowing programs to report detailed version and build information at runtime. The system uses a build.rs script that executes during compilation to gather system information and generate constants that are embedded directly into the final binary.
Table of Contents

Architecture
Goals and Design Philosophy
Field Derivation Methods
Implementation Details
Integration Guide
Output Format
Error Handling Strategy
Platform Compatibility
Maintenance and Extension

Architecture
System Components
┌─────────────────┐     ┌──────────────┐     ┌─────────────────┐
│   Cargo.toml    │────>│   build.rs   │────>│  build_info.rs  │
│  (source data)  │     │(build script)│     │   (generated)   │
└─────────────────┘     └──────────────┘     └─────────────────┘
                               │                       │
                               ▼                       ▼
                        ┌──────────────┐     ┌─────────────────┐
                        │ Environment  │     │    main.rs      │
                        │  Variables   │     │   (includes)    │
                        └──────────────┘     └─────────────────┘
                                                      │
                                                      ▼
                                             ┌─────────────────┐
                                             │  Final Binary   │
                                             │  (with embedded │
                                             │     metadata)   │
                                             └─────────────────┘
Build Flow

Compilation Trigger: When cargo build is invoked, Cargo automatically executes build.rs before compiling the main source
Information Collection: The build script gathers information from multiple sources
Code Generation: Creates build_info.rs in the OUT_DIR directory
Inclusion: Main source code includes the generated file using the include! macro
Binary Creation: Final binary contains embedded constants with build information

Goals and Design Philosophy
Primary Goals

Zero Runtime Dependencies: All information is gathered at compile time and embedded as static constants
No External Crates: Uses only Rust standard library to avoid dependency bloat
Cross-Platform Compatibility: Works identically on Windows, Linux, and macOS
Build Reproducibility: Same source produces consistent metadata (except timestamp)
Graceful Degradation: Never fails the build; provides "unknown" values if information cannot be obtained

Design Principles

Simplicity: Flat file structure without complex module hierarchies
Transparency: Extensive documentation for every function and decision
Reliability: Comprehensive error handling without panics or unwraps
Maintainability: Clear, self-documenting code with meaningful names
Performance: No runtime overhead; all work done at compile time

Field Derivation Methods
1. Build Timestamp
Source: System time at compilation
Method: Custom implementation using SystemTime::now()
Format: YYYY-MM-DD HH:MM:SS (UTC)

// Derivation process:
SystemTime::now()
  → duration_since(UNIX_EPOCH)
  → convert to seconds
  → calculate year/month/day/hour/minute/second components
  → format as string
Implementation Details:

Uses epoch-based calculation for cross-platform consistency
Implements proper leap year handling (divisible by 4, except centuries unless divisible by 400)
Accounts for varying month lengths
All calculations in UTC to avoid timezone complications

2. Package Version
Source: Cargo.toml [package] section
Method: Environment variable CARGO_PKG_VERSION
Format: Semantic version string (e.g., "1.0.0")
env::var("CARGO_PKG_VERSION")
Notes:

Automatically provided by Cargo during build
Reflects the version field in Cargo.toml
Falls back to "unknown" if not set

3. Operating System
Source: Rust compiler constants
Method: std::env::consts::OS
Format: OS identifier string
Possible Values:

"linux" - Linux distributions
"windows" - Windows (all versions)
"macos" - macOS
"ios" - iOS
"android" - Android
"freebsd" - FreeBSD
Others as supported by Rust

4. Architecture
Source: Rust compiler constants
Method: std::env::consts::ARCH
Format: Architecture identifier string
Common Values:

"x86_64" - 64-bit x86 (Intel/AMD)
"x86" - 32-bit x86
"aarch64" - 64-bit ARM
"arm" - 32-bit ARM
"wasm32" - WebAssembly 32-bit
"powerpc64" - 64-bit PowerPC

5. Rust Edition
Source: Cargo.toml [package] section
Method: Custom TOML parser reading the edition field
Format: Edition year (e.g., "2021", "2018", "2015")

// Parsing process:
Open Cargo.toml
  → Read line by line
  → Find line starting with "edition"
  → Extract value after "="
  → Strip quotes and whitespace
Implementation Notes:

Does not use external TOML parsing libraries
Handles both quoted and unquoted values
Skips comments and empty lines
Falls back to checking CARGO_MANIFEST_DIR environment variable

6. Rust Compiler Version
Source: System command execution
Method: Executes rustc --version and parses output
Format: Version number only (e.g., "1.75.0")

// Extraction process:
Command::new("rustc").arg("--version")
  → Parse output: "rustc 1.75.0 (abc123 2024-01-15)"
  → Split by whitespace
  → Extract second token
  → Return version number
Error Handling:

Returns "unknown" if rustc is not in PATH
Handles command execution failures gracefully
Parses various rustc output formats

7. Build Profile
Source: Cargo build environment
Method: Environment variable PROFILE
Format: Profile name string
Possible Values:

"debug" - Development build (unoptimized)
"release" - Production build (optimized)
"test" - Test build
"bench" - Benchmark build
Custom profiles as defined in Cargo.toml

Implementation Details
Time Calculation Algorithm
The time calculation uses a precise algorithm for converting Unix epoch seconds to calendar dates:

1. Start with seconds since 1970-01-01 00:00:00 UTC
2. Extract time of day:
   - seconds_today = total_seconds % 86400
   - hour = seconds_today / 3600
   - minute = (seconds_today % 3600) / 60
   - second = seconds_today % 60
3. Calculate date:
   - days = total_seconds / 86400
   - Iterate through years from 1970, subtracting days per year
   - Account for leap years
   - Iterate through months, subtracting days per month
   - Remaining days + 1 = day of month
TOML Parsing Strategy
The custom TOML parser is minimal but sufficient:

1. Open file with BufReader for line-by-line reading
2. For each line:
   - Trim whitespace
   - Skip if starts with '#' (comment) or is empty
   - Check if line starts with field name
   - Split on '=' character
   - Extract and clean value (remove quotes, trim)
3. Return first matching field value
Code Generation Template
The generated build_info.rs follows this structure:

//! Generated build information
pub const BUILD_PACKAGE_VERSION: &str = "...";
pub const BUILD_TIMESTAMP: &str = "...";
pub const BUILD_OS: &str = "...";
pub const BUILD_ARCHITECTURE: &str = "...";
pub const BUILD_RUST_EDITION: &str = "...";
pub const BUILD_RUSTC_VERSION: &str = "...";
pub const BUILD_PROFILE: &str = "...";
Integration Guide
Step 1: Add build.rs to Project Root
Place the build.rs file in your project root directory (same level as Cargo.toml).
Step 2: Include Generated Code
In your main source file, add:

// Create a module for build information
mod build_info {
    include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
}

// Create display function
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
Step 3: Handle Command Line Arguments

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && (args[1] == "-v" || args[1] == "--version") {
        println!("{}", format_version_info());
        std::process::exit(0);
    }

    // Rest of program...
}
Output Format
Standard Version Display

Version: 1.0.0
Build Date-Time: 2024-01-15 14:30:45
OS: linux
Architecture: x86_64
Rust Edition: 2021
Rustc: 1.75.0
Profile: release

Field Specifications
FieldFormatExampleFallbackVersionSemantic
version1.0.0unknownBuild Date-TimeYYYY-MM-DD HH:MM:SS2024-01-15 14:30:45unknownOSLowercase
identifierlinuxunknownArchitectureArchitecture stringx86_64unknownRust
EditionYear2021unknownRustcVersion number1.75.0unknownProfileProfile namereleaseunknown
Error Handling Strategy
Principles

Never Panic: All errors are handled gracefully
Never Fail Build: Build script errors don't stop compilation
Fallback Values: Always provide "unknown" when data unavailable
Silent Degradation: Log warnings to stderr but continue execution

Error Scenarios
ScenarioResponseFallbackCargo.toml missingUse env vars"unknown"Edition
field missingCheck alternate locations"unknown"Rustc not in PATHCatch
command error"unknown"OUT_DIR not setGenerate minimal fileAll fields
"unknown"File write failsLog warningMinimal fallback fileTime
calculation errorCatch arithmetic errors"unknown"
Fallback Generation
If the main generation fails, a minimal fallback file is created:

pub const BUILD_PACKAGE_VERSION: &str = "unknown";
pub const BUILD_TIMESTAMP: &str = "unknown";
// ... all fields set to "unknown"
Platform Compatibility
Tested Platforms
PlatformOS ConstantArchitectureNotesLinux x86_64linuxx86_64Primary
development platformWindows x64windowsx86_64Requires rustc in
PATHmacOS Intelmacosx86_64Full supportmacOS Apple Siliconmacosaarch64Full support
Platform-Specific Considerations

Path Separators: Code uses Path::join() for cross-platform paths
Command Execution: rustc must be in system PATH
Time Zones: All timestamps in UTC for consistency
Line Endings: TOML parser handles both LF and CRLF

Maintenance and Extension
Adding New Fields
To add a new field:

Create getter function in build.rs:

fn get_new_field() -> String {
    // Implementation
    // Return "unknown" on error
}

Add to generation:

let new_field = get_new_field();
// Add to generate_build_info_file parameters

Update template:

pub const BUILD_NEW_FIELD: &str = "{}";

Update display function:

format!("...\nNew Field: {}", build_info::BUILD_NEW_FIELD)
Modifying Time Format
The time format can be changed by modifying the format string in get_build_timestamp():
// Current format: YYYY-MM-DD HH:MM:SS
format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hour, minute, second)

// Alternative ISO 8601: YYYY-MM-DDTHH:MM:SSZ
format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", year, month, day, hour, minute, second)

// Alternative compact: YYYYMMDD_HHMMSS
format!("{:04}{:02}{:02}_{:02}{:02}{:02}", year, month, day, hour, minute, second)
Debugging Build Scripts
To debug the build script:

View output:

cargo build -vv  # Very verbose output

Check generated file:

find target -name "build_info.rs" -type f

Add debug prints:

eprintln!("Debug: field_value = {}", value);  // Goes to stderr

Force rebuild:

touch build.rs  # Triggers rerun
cargo clean && cargo build  # Full rebuild
Best Practices for Modification

Always provide fallbacks: Never let missing data break the build
Document changes: Update this documentation when adding features
Test cross-platform: Verify on at least Linux and Windows
Maintain compatibility: Don't break existing integrations
Keep it simple: Avoid complex dependencies or algorithms
Version carefully: Consider backward compatibility

Troubleshooting
Common Issues and Solutions
IssueCauseSolution"cannot find function format_build_info"Include path wrongUse module wrapper approachBuild
info shows "unknown"Data not availableCheck environment, ensure rustc in PATHCargo.toml not foundWrong working
directoryUse CARGO_MANIFEST_DIR env varTime is wrongUTC vs local timeAll times are UTC by designBuild doesn't
updateCargo cachingTouch build.rs or run cargo clean
Validation Checklist

 Build completes without errors
 Version flag (-v or --version) displays all fields
 No "unknown" values in normal conditions
 Cross-platform build succeeds
 Generated constants are accessible
 No unwrap() or panic!() in code
 All functions have documentation

*/

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Main build script function
///
/// This function executes at compile time and generates build_info.rs
/// containing constants with build metadata.
fn main() {
    // Tell Cargo to rerun this script if Cargo.toml changes
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");

    // Collect all build information with error handling
    let build_timestamp = get_build_timestamp();
    let os_type = get_os_type();
    let architecture = get_architecture();
    let rust_edition = get_rust_edition();
    let rustc_version = get_rustc_version();
    let build_profile = get_build_profile();
    let package_version = get_package_version();

    // Generate the output file
    if let Err(e) = generate_build_info_file(
        &build_timestamp,
        &os_type,
        &architecture,
        &rust_edition,
        &rustc_version,
        &build_profile,
        &package_version,
    ) {
        // If we can't generate the file, print warning but don't fail the build
        eprintln!("Warning: Failed to generate build info file: {}", e);
        // Generate a minimal fallback file
        generate_fallback_build_info();
    }
}

/// Gets the current build timestamp formatted as YYYY-MM-DD HH:MM:SS
///
/// # Returns
/// A formatted string representing the current date and time in UTC
fn get_build_timestamp() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let total_seconds = duration.as_secs();
            let (year, month, day, hour, minute, second) =
                convert_epoch_seconds_to_components(total_seconds);

            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                year, month, day, hour, minute, second
            )
        }
        Err(_) => String::from("unknown"),
    }
}

/// Converts Unix epoch seconds to date/time components
///
/// # Arguments
/// * `epoch_seconds` - Seconds since Unix epoch (1970-01-01 00:00:00 UTC)
///
/// # Returns
/// Tuple of (year, month, day, hour, minute, second)
fn convert_epoch_seconds_to_components(epoch_seconds: u64) -> (u32, u32, u32, u32, u32, u32) {
    const SECONDS_PER_MINUTE: u64 = 60;
    const SECONDS_PER_HOUR: u64 = 3600;
    const SECONDS_PER_DAY: u64 = 86400;

    // Calculate time components
    let seconds_today = epoch_seconds % SECONDS_PER_DAY;
    let hour = (seconds_today / SECONDS_PER_HOUR) as u32;
    let minute = ((seconds_today % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u32;
    let second = (seconds_today % SECONDS_PER_MINUTE) as u32;

    // Calculate date components
    let days_since_epoch = epoch_seconds / SECONDS_PER_DAY;
    let (year, month, day) = convert_days_to_date(days_since_epoch);

    (year, month, day, hour, minute, second)
}

/// Converts days since Unix epoch to year, month, day
///
/// # Arguments
/// * `days_since_epoch` - Number of days since 1970-01-01
///
/// # Returns
/// Tuple of (year, month, day)
fn convert_days_to_date(days_since_epoch: u64) -> (u32, u32, u32) {
    let mut year = 1970u32;
    let mut remaining_days = days_since_epoch;

    // Helper to check if a year is a leap year
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

    // Days in each month
    const DAYS_IN_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const DAYS_IN_MONTH_LEAP: [u32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let days_in_months = if is_leap_year(year) {
        &DAYS_IN_MONTH_LEAP
    } else {
        &DAYS_IN_MONTH
    };

    // Find month and day
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

    let day = days_left + 1; // Days are 1-based

    (year, month, day)
}

/// Gets the operating system type
///
/// # Returns
/// String representing the OS (e.g., "linux", "windows", "macos")
fn get_os_type() -> String {
    env::consts::OS.to_string()
}

/// Gets the target architecture
///
/// # Returns
/// String representing the architecture (e.g., "x86_64", "aarch64")
fn get_architecture() -> String {
    env::consts::ARCH.to_string()
}

/// Gets the Rust edition from Cargo.toml
///
/// # Returns
/// String representing the Rust edition (e.g., "2021", "2018")
fn get_rust_edition() -> String {
    // Try to read from Cargo.toml
    match read_edition_from_cargo_toml() {
        Ok(edition) => edition,
        Err(_) => {
            // Fallback: try to get from environment variable
            env::var("CARGO_MANIFEST_DIR")
                .ok()
                .and_then(|manifest_dir| {
                    let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");
                    read_toml_field(&cargo_toml_path.to_string_lossy(), "edition").ok()
                })
                .unwrap_or_else(|| String::from("unknown"))
        }
    }
}

/// Reads the edition field from Cargo.toml
///
/// # Returns
/// Result containing the edition string or an error
fn read_edition_from_cargo_toml() -> Result<String, String> {
    let cargo_toml_path = "Cargo.toml";

    // First try to read from [package] section
    if let Ok(edition) = read_toml_field(cargo_toml_path, "edition") {
        if !edition.is_empty() {
            return Ok(edition);
        }
    }

    // If not found, return error
    Err(String::from("Edition not found in Cargo.toml"))
}

/// Reads a field from a TOML file
///
/// # Arguments
/// * `path` - Path to the TOML file
/// * `field_name` - Name of the field to read
///
/// # Returns
/// Result containing the field value or an error
fn read_toml_field(path: &str, field_name: &str) -> Result<String, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open {}: {}", path, e))?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }

        // Look for the field
        if trimmed.starts_with(field_name) {
            // Parse "field = value" format
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let value = parts[1]
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();

                if !value.is_empty() {
                    return Ok(value);
                }
            }
        }
    }

    Err(format!("Field '{}' not found", field_name))
}

/// Gets the Rust compiler version by executing rustc --version
///
/// # Returns
/// String containing the rustc version (e.g., "1.75.0")
fn get_rustc_version() -> String {
    match Command::new("rustc").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version_string = String::from_utf8_lossy(&output.stdout);
                // Parse "rustc 1.75.0 (abcdef 2024-01-15)" to get just "1.75.0"
                version_string
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("unknown")
                    .to_string()
            } else {
                String::from("unknown")
            }
        }
        Err(_) => String::from("unknown"),
    }
}

/// Gets the build profile including custom profiles
///
/// # Returns
/// String representing the full build profile name
///
/// # Details
/// Attempts to detect the exact profile name including custom profiles
/// like "release-performance" or "release-small" by checking multiple sources.
fn get_build_profile() -> String {
    // Method 1: Check CARGO_PROFILE environment variable (newer Cargo versions)
    if let Ok(profile) = env::var("CARGO_PROFILE") {
        return profile;
    }

    // Method 2: Try to infer from the target directory structure
    // When building with --profile release-performance, the target directory
    // will be target/release-performance instead of target/release
    if let Ok(out_dir) = env::var("OUT_DIR") {
        // OUT_DIR looks like: /path/to/project/target/{profile}/build/{crate}-{hash}/out
        // We want to extract the {profile} part

        let parts: Vec<&str> = out_dir.split(std::path::MAIN_SEPARATOR).collect();

        // Find "target" and get the next element
        for (i, part) in parts.iter().enumerate() {
            if *part == "target" && i + 1 < parts.len() {
                let potential_profile = parts[i + 1];
                // Verify it's not "build" or other known subdirectories
                if potential_profile != "build"
                    && potential_profile != "deps"
                    && potential_profile != "examples"
                    && potential_profile != "incremental"
                {
                    return potential_profile.to_string();
                }
            }
        }
    }

    // Method 3: Fall back to basic PROFILE variable
    env::var("PROFILE").unwrap_or_else(|_| String::from("unknown"))
}

/// Gets the package version from Cargo environment
///
/// # Returns
/// String representing the package version from Cargo.toml
fn get_package_version() -> String {
    env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| String::from("unknown"))
}

/// Generates the build_info.rs file with all collected information
///
/// # Arguments
/// * All build information as string references
///
/// # Returns
/// Result indicating success or failure
fn generate_build_info_file(
    build_timestamp: &str,
    os_type: &str,
    architecture: &str,
    rust_edition: &str,
    rustc_version: &str,
    build_profile: &str,
    package_version: &str,
) -> Result<(), String> {
    let out_dir = env::var("OUT_DIR").map_err(|e| format!("Failed to get OUT_DIR: {}", e))?;

    let dest_path = Path::new(&out_dir).join("build_info.rs");

    let mut file =
        File::create(&dest_path).map_err(|e| format!("Failed to create build_info.rs: {}", e))?;

    // Write the constants to the file as a module
    writeln!(
        file,
        r#"// Generated build information
// This file is automatically generated by build.rs at compile time
// Do not edit manually

/// Package version from Cargo.toml
pub const BUILD_PACKAGE_VERSION: &str = "{}";

/// Build timestamp in YYYY-MM-DD HH:MM:SS format (UTC)
pub const BUILD_TIMESTAMP: &str = "{}";

/// Operating system the binary was built on
pub const BUILD_OS: &str = "{}";

/// Target architecture
pub const BUILD_ARCHITECTURE: &str = "{}";

/// Rust edition used for compilation
pub const BUILD_RUST_EDITION: &str = "{}";

/// Rust compiler version
pub const BUILD_RUSTC_VERSION: &str = "{}";

/// Build profile (debug/release)
pub const BUILD_PROFILE: &str = "{}";"#,
        package_version,
        build_timestamp,
        os_type,
        architecture,
        rust_edition,
        rustc_version,
        build_profile
    )
    .map_err(|e| format!("Failed to write to build_info.rs: {}", e))?;

    Ok(())
}

/// Generates a minimal fallback build_info.rs file if the main generation fails
fn generate_fallback_build_info() {
    if let Ok(out_dir) = env::var("OUT_DIR") {
        let dest_path = Path::new(&out_dir).join("build_info.rs");

        if let Ok(mut file) = File::create(&dest_path) {
            let _ = writeln!(
                file,
                r#"//! Fallback build information
pub const BUILD_PACKAGE_VERSION: &str = "unknown";
pub const BUILD_TIMESTAMP: &str = "unknown";
pub const BUILD_OS: &str = "unknown";
pub const BUILD_ARCHITECTURE: &str = "unknown";
pub const BUILD_RUST_EDITION: &str = "unknown";
pub const BUILD_RUSTC_VERSION: &str = "unknown";
pub const BUILD_PROFILE: &str = "unknown";"#
            );
        }
    }
}
