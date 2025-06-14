# ff_file_manager_minimal_rust



## Build
see https://github.com/lineality/rust_compile_optimizations_cheatsheet

#### For smaller size, build (~0.53 mb)
```bash
cargo build --profile release-small 
```
#### Or for optimized performance (~6.4 mb)
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


