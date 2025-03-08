# ff_file_manager_minimal_rust



## linux: for small build, use (for me executible is 1.8mb)
see https://github.com/lineality/rust_compile_optimizations_cheatsheet
```bash
cargo build --profile release-small 
```
or 
```bash
cargo build --profile release-performance
```

## ~Install
Set an executable file as a keyword in the command line interface (CLI) so that entering that keyword calls the executable:

1. Open the bash shell configuration file in a text editor. The configuration file is usually located at ~/.bashrc or ~/.bash_profile. (use whatever edictor: vim, nano, hx (helix), teehee, lapce, etc.)
```bash
hx ~/.bashrc
```
or in some systems it may be called 'b'ash_profile'

2. Add an alias for your executable at the end of the file. Replace your_executable with the name of your executable and /path/to/your_executable with the full path to your executable. Add:
```text
alias your_keyword='/path/to/your_executable'
```
e.g. add:
```text
alias ff='/home/COMPUTERNAME/ff_file_browser/ff'
```

3. Save and close the text editor. 
- If you used nano, you can do this by pressing: Ctrl x s (control key, x key, s key)
- If you use Helix(hx), Vim(vi), or Teehee: 'i' to type, then esc for normal mode, then :wq to write and quit

4. Reload the bash shell configuration file to apply the changes.
```bash
source ~/.bashrc
```
or bash_profile


# ff is a minimal rust file manager

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

