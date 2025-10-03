
# ff_file_manager_minimal_rust
ff is a minimal file manager. It's File Fantastic.
...it's a File Fantasy.

```text
quit back|term|dir file|name size mod|get-send file v,y,p|str>search|enter>reset
/home/oops/code/ff_file_manager_minimal_rust
  #   Name                                                     Size    Modified
 ------------------------------------------------------------------------------
  1. .cargo/                                                      - 09-12 23:29
  2. .git/                                                        -       16:41
  3. archive/                                                     -       16:41
  4. executables/                                                 -       16:39
  5. src/                                                         - 09-27 19:32
  6. target/                                                      - 09-29 23:17
  7. test_empty/                                                  - 09-27 19:19
  8. test_temp/                                                   - 09-27 19:19
  9. .gitignore                                               908 B 09-29 23:23
 10. Cargo.lock                                               146 B 06-16 12:32
 11. Cargo.toml                                              2.1 KB 09-12 23:29
 12. LICENSE                                                 1.1 KB 02-20 14:19
 13. README.md                                               9.6 KB 09-25 11:45
 14. build.rs                                                 34 KB 09-29 21:53
 15. code_archive.txt                                         66 KB       16:21
 16. test.csv                                                 241 B 09-15 12:15
--- Page 1 of 2: up/down, j/k, </>, w/x, arrows, etc. Size: tall+N wide-N ---
>>
```

## From Terminal
```bash
ff
ff path/
ff --help
ff --source
ff --version
```

#### Returns: Output path
The GUI version of the 'select file' or 'select folder' feature/functionality
is very frequently used, as with upload, download, open, import, etc.
File Fantastic can provide this 'select item on your device' functionality
as a module for any rust cli terminal/headless applications:
- Last 'navigation current directory' path returned by default on exit
- User-Selected file or directory path returned by command: {selection number} --return-path


## Help Menu
- cli, run
```bash
ff --help
ff -h
```
- As a command inside of ff, run:
```
--help
```

## Build ff
- See https://github.com/lineality/rust_compile_optimizations_cheatsheet for more details.

#### For smallest size, build (~1 mb)
```bash
cargo build --profile release-small
```
#### or for optimal performance (~10 mb)
```bash
cargo build --profile release-performance
```

# ~Install

## 1. local/bin method
- compile or otherwise get a binary/executible
- place the ff binary/executible in the ~user's ~local 'bin' directory

#### in linux:
```path
home/{YOUR_USER_NAME}/.local/bin
```

#### in android:
```path
data/data/com.termux/files/user/bin
```

Then running 'ff' in a terminal should work.


## 2. Alias Method
Put your executable-binary somewhere, and associate that path
with a callable keyword for your command line interface (CLI)
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

5. Reload the bash-shell configuration file, and maybe open a new terminal, to apply and use the changes.
```bash
source ~/.bashrc
```
or bash_profile

Now you should be able to call File Fantastic by typing 'ff' (or whatever you choose) into a terminal.

Then running 'ff' in a terminal should work.


## 3. "cargo install" method (may take experimentation with your OS and hardware)
1. clone
```bash
git clone https://github.com/lineality/ff_file_manager_minimal_rust.git
```
- or ssh
```bash
git clone git@github.com:lineality/ff_file_manager_minimal_rust.git
```

2. go to:
- /...YOURSTUFF.../ff_file_manager_minimal_rust
```bash
cd ff_file_manager_minimal_rust
```


#### Install binary to /usr/local instead of ~/.cargo
- 'root' here means picking a save-location, not "root" permission

3. run in termial (type these literally as they are here)
e.g. Fedora(rpm) / Ubuntu (Linux)
```
cargo install --path . --root ~/.local
```
e.g. Andoid (also linux)
```
cargo install --path . --root $PREFIX
```


Then running 'ff' in a terminal should work.



### Alternately, save in the default .cargo/bin location
- you can use ff to do this, much more easily than raw bash

```bash
cargo install ff
```

The default Cargo binary directory is:
```
$HOME/.cargo/bin
```

Updating: Bashrc/profile/bash_profil
### For Linux:
```
~/.bashrc - Most common for interactive non-login shells
~/.profile - For login shells (some distributions)
~/.bash_profile - Alternative for login shells (if it exists, it overrides ~/.profile)
```

### For macOS:
```
~/.bash_profile (for Bash on older macOS)
~/.zshrc (for Zsh on macOS Catalina and later)
```

### Add this to the Bashrc/profile/bash_profil file: (maybe not working)
```
export PATH="$HOME/.cargo/bin:$PATH"
```



### Optional: config file
Add a file called 'absolute_paths_to_local_partner_fileopening_executibles.txt',
or one will be added automatically by ff.
##### e.g.
```path
/home/USER-NAME-HERE/ff_file_browser/absolute_paths_to_local_partner_fileopening_executibles.txt
```
You can put the paths to local executable files as lines in this file.



## Module: ff can be used in other cli projects that might need a directory-view & navigation.

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
25. rows-count & custom views: see data file size in a directory

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


## Source-It
- Get/Save ff source code (from the binary, not online) to your CWD:
- cli, run
```bash
ff --source
```
- command inside ff
```
--source
```
- inspect, modify, make, create
- https://github.com/lineality/source_it_module
