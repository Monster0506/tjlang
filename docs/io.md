# TJLang IO Module Documentation

This document describes the comprehensive input/output functionality available in TJLang's `IO` module. This module provides powerful console operations, formatted output, user interaction, and terminal control features.

## Table of Contents

- [Basic Output Operations](#basic-output-operations) - Printing and console output
- [Input Operations](#input-operations) - Reading user input
- [Colored Output](#colored-output) - Terminal colors and styling
- [Terminal Control](#terminal-control) - Cursor and screen manipulation
- [User Interaction](#user-interaction) - Prompts, confirmations, and selections
- [Progress Indicators](#progress-indicators) - Progress bars and spinners
- [Stream Operations](#stream-operations) - File stream handling
- [Examples](#examples) - Usage examples

## Basic Output Operations

Fundamental printing and console output functionality.

### `print(value: &Value) -> Result<(), str>`
Prints a value to stdout with a newline.

```tjlang
IO.print("Hello, World!")  # Prints: Hello, World!
IO.print(42)  # Prints: 42
IO.print(true)  # Prints: true
```

### `println(value: &Value) -> Result<(), str>`
Prints a value to stdout with a newline (same as print).

```tjlang
IO.println("Hello, World!")  # Prints: Hello, World!
```

### `printf(format: str, args: &[Value]) -> Result<(), str>`
Prints a formatted string to stdout.

```tjlang
name: str = "Alice"
age: int = 25
IO.printf("Name: {}, Age: {}", [name, age])  # Prints: Name: Alice, Age: 25
```

## Input Operations

Reading user input from stdin.

### `read_line() -> Result<str, str>`
Reads a line of text from stdin.

```tjlang
IO.print("Enter your name: ")
name: str = IO.read_line()  # Waits for user input
IO.print("Hello, " + name)
```

### `read_char() -> Result<char, str>`
Reads a single character from stdin.

```tjlang
IO.print("Press any key: ")
ch: char = IO.read_char()  # Waits for single character
IO.print("You pressed: " + ch.to_string())
```

### `read_int() -> Result<int, str>`
Reads an integer from stdin.

```tjlang
IO.print("Enter a number: ")
num: int = IO.read_int()  # Waits for integer input
IO.print("You entered: " + num.to_string())
```

### `read_float() -> Result<float, str>`
Reads a float from stdin.

```tjlang
IO.print("Enter a decimal: ")
num: float = IO.read_float()  # Waits for float input
IO.print("You entered: " + num.to_string())
```

### `read_bool() -> Result<bool, str>`
Reads a boolean from stdin. Accepts: true/1/yes/y/on or false/0/no/n/off.

```tjlang
IO.print("Continue? (y/n): ")
continue: bool = IO.read_bool()  # Waits for boolean input
if continue {
    IO.print("Continuing...")
}
```

## Colored Output

Terminal colors and styled output for better user experience.

### `print_color(text: str, color: str) -> Result<(), str>`
Prints text in the specified color.

```tjlang
IO.print_color("Error message", "red")
IO.print_color("Success message", "green")
IO.print_color("Warning message", "yellow")
```

### Available Colors
- `Red` - Red text
- `Green` - Green text  
- `Yellow` - Yellow text
- `Blue` - Blue text
- `Magenta` - Magenta text
- `Cyan` - Cyan text
- `White` - White text
- `Black` - Black text
- `Reset` - Reset to default

### `print_error(message: str) -> Result<(), str>`
Prints an error message in red.

```tjlang
IO.print_error("Something went wrong!")  # Prints: Error: Something went wrong! (in red)
```

### `print_warning(message: str) -> Result<(), str>`
Prints a warning message in yellow.

```tjlang
IO.print_warning("This is a warning")  # Prints: Warning: This is a warning (in yellow)
```

### `print_success(message: str) -> Result<(), str>`
Prints a success message in green.

```tjlang
IO.print_success("Operation completed")  # Prints: Success: Operation completed (in green)
```

### `print_info(message: str) -> Result<(), str>`
Prints an info message in blue.

```tjlang
IO.print_info("Processing data...")  # Prints: Info: Processing data... (in blue)
```

### `print_debug(message: str) -> Result<(), str>`
Prints a debug message in cyan.

```tjlang
IO.print_debug("Debug information")  # Prints: Debug: Debug information (in cyan)
```

## Terminal Control

Advanced terminal manipulation and cursor control.

### `clear_screen() -> Result<(), str>`
Clears the terminal screen.

```tjlang
IO.clear_screen()  # Clears the entire screen
```

### `move_cursor(row: int, col: int) -> Result<(), str>`
Moves the cursor to the specified position.

```tjlang
IO.move_cursor(10, 5)  # Moves cursor to row 10, column 5
IO.print("Text at position (10,5)")
```

### `hide_cursor() -> Result<(), str>`
Hides the terminal cursor.

```tjlang
IO.hide_cursor()  # Hides the cursor
# Do some work...
IO.show_cursor()  # Shows the cursor again
```

### `show_cursor() -> Result<(), str>`
Shows the terminal cursor.

```tjlang
IO.show_cursor()  # Makes the cursor visible
```

### `get_terminal_size() -> Result<(int, int), str>`
Gets the terminal dimensions (width, height).

```tjlang
size: (int, int) = IO.get_terminal_size()  # Returns (width, height)
IO.print("Terminal size: " + size.0.to_string() + "x" + size.1.to_string())
```

### `is_terminal() -> bool`
Checks if stdout is connected to a terminal.

```tjlang
if IO.is_terminal() {
    IO.print_color("Terminal output", Color::Green)
} else {
    IO.print("File output")
}
```

### `is_input_terminal() -> bool`
Checks if stdin is connected to a terminal.

```tjlang
if IO.is_input_terminal() {
    IO.print("Interactive mode")
} else {
    IO.print("Batch mode")
}
```

## User Interaction

Interactive prompts and user selection interfaces.

### `prompt(message: str) -> Result<str, str>`
Prompts the user for input with a message.

```tjlang
name: str = IO.prompt("Enter your name")  # Prints: Enter your name: and waits for input
IO.print("Hello, " + name)
```

### `prompt_with_default(message: str, default: str) -> Result<str, str>`
Prompts the user with a default value.

```tjlang
# If user presses Enter without typing, returns "localhost"
host: str = IO.prompt_with_default("Enter hostname", "localhost")
IO.print("Connecting to: " + host)
```

### `confirm(message: str) -> Result<bool, str>`
Prompts for yes/no confirmation.

```tjlang
delete: bool = IO.confirm("Delete file?")  # Prints: Delete file? [y]: 
if delete {
    IO.print("File deleted")
} else {
    IO.print("Operation cancelled")
}
```

### `select(message: str, options: &[str]) -> Result<int, str>`
Presents a menu and returns the selected index.

```tjlang
options: &[str] = ["Option 1", "Option 2", "Option 3"]
choice: int = IO.select("Choose an option:", options)
IO.print("You selected: " + options[choice])
```

### `multi_select(message: str, options: &[str]) -> Result<Vec<int>, str>`
Allows multiple selections from a menu.

```tjlang
options: &[str] = ["Apple", "Banana", "Cherry", "Date"]
selections: Vec<int> = IO.multi_select("Choose fruits (comma-separated):", options)
IO.print("You selected: " + selections.to_string())
```

## Progress Indicators

Visual feedback for long-running operations.

### `create_progress_bar(total: int) -> ProgressBar`
Creates a progress bar for tracking progress.

```tjlang
# Create progress bar for 100 operations
progress: ProgressBar = IO.create_progress_bar(100)

# Update progress
for (i:int | 0..100) {
    # Do some work...
    progress.update(i)
}

progress.finish()  # Complete the progress bar
```

### `create_spinner() -> Spinner`
Creates a spinner for indeterminate operations.

```tjlang
spinner: Spinner = IO.create_spinner()

# Show spinner while working
for (i:int | 0..10) {
    spinner.spin("Processing...")
    # Do some work...
}

spinner.stop()  # Stop the spinner
```

## Stream Operations

File stream handling for reading and writing.

### `Stream::new_reader(file: File) -> Stream`
Creates a stream for reading from a file.

```tjlang
file: File = File::open("data.txt")
stream: Stream = Stream::new_reader(file)

# Read lines from file
while true {
    line: Option<str> = stream.read_line()
    if line.is_none() {
        break
    }
    IO.print("Line: " + line.unwrap())
}
```

### `Stream::new_writer(file: File) -> Stream`
Creates a stream for writing to a file.

```tjlang
file: File = File::create("output.txt")
stream: Stream = Stream::new_writer(file)

# Write to file
stream.write_line("Hello, World!")
stream.write_line("This is a test")
stream.flush()  # Ensure data is written
```

## Examples

Here are comprehensive examples showing how to use the IO module:

### Basic Input/Output Example

```tjlang
# Basic input/output
IO.print("Welcome to TJLang!")
name: str = IO.prompt("What's your name?")
age: int = IO.prompt("How old are you?").to_int()

IO.print("Hello, " + name + "! You are " + age.to_string() + " years old.")
```

### Colored Output Example

```tjlang
# Colored output for different message types
IO.print_success("Operation completed successfully!")
IO.print_warning("This is a warning message")
IO.print_error("An error occurred!")
IO.print_info("Here's some information")
IO.print_debug("Debug information")

# Custom colored output
IO.print_color("Custom message", Color::Cyan)
IO.print_color("Another message", Color::Magenta)
```

### Interactive Menu Example

```tjlang
# Interactive menu system
options: &[str] = ["View Profile", "Edit Settings", "Logout", "Exit"]
choice: int = IO.select("What would you like to do?", options)

match choice {
    0 => IO.print("Viewing profile..."),
    1 => IO.print("Opening settings..."),
    2 => IO.print("Logging out..."),
    3 => IO.print("Goodbye!"),
    _ => IO.print_error("Invalid selection")
}
```

### Progress Bar Example

```tjlang
# Progress bar for long operation
IO.print("Processing data...")
progress: ProgressBar = IO.create_progress_bar(1000)

for i in 0..1000 {
    # Simulate work
    # ... do some processing ...
    
    # Update progress every 10 iterations
    if i % 10 == 0 {
        progress.update(i)
    }
}

progress.finish()
IO.print_success("Processing complete!")
```

### Spinner Example

```tjlang
# Spinner for indeterminate operation
IO.print("Loading...")
spinner: Spinner = IO.create_spinner()

# Simulate loading
for i in 0..50 {
    spinner.spin("Loading data...")
    # Simulate work
    # ... do some processing ...
}

spinner.stop()
IO.print_success("Loading complete!")
```

### File Processing Example

```tjlang
# Process file with progress
filename: str = IO.prompt("Enter filename to process")
file: File = File::open(filename)
stream: Stream = Stream::new_reader(file)

IO.print("Processing file...")
line_count: int = 0

while true {
    line: Option<str> = stream.read_line()
    if line.is_none() {
        break
    }
    
    line_count = line_count + 1
    # Process line...
    IO.print("Line " + line_count.to_string() + ": " + line.unwrap())
}

IO.print_success("Processed " + line_count.to_string() + " lines")
```

### Terminal Control Example

```tjlang
# Advanced terminal control
IO.clear_screen()
IO.print("Terminal Control Demo")

# Get terminal size
size: (int, int) = IO.get_terminal_size()
IO.print("Terminal size: " + size.0.to_string() + "x" + size.1.to_string())

# Move cursor and print
IO.move_cursor(5, 10)
IO.print("Text at position (5,10)")

# Hide cursor temporarily
IO.hide_cursor()
IO.print("Cursor is hidden")
# Do some work...
IO.show_cursor()
IO.print("Cursor is visible again")
```

This comprehensive IO module provides powerful input/output capabilities for creating interactive TJLang applications with rich terminal interfaces, user interaction, and visual feedback.
