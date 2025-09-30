//! IO Module - Input/Output operations
//!
//! Provides comprehensive IO functionality including:
//! - Console input/output
//! - Formatted printing
//! - Stream operations
//! - Buffer management
//! - Terminal control
//! - Color output
//! - Progress indicators
//! - Interactive prompts

use crate::values::Value;
use std::io::{self, Write, Read, BufRead, BufReader, BufWriter};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;


/// IO module for input/output operations
pub struct IO;

impl IO {
    /// Print a value to stdout
    pub fn print(value: &Value) -> Result<(), String> {
        print!("{}", value.to_string());
        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(())
    }
    
    /// Print a value to stdout with newline
    pub fn println(value: &Value) -> Result<(), String> {
        println!("{}", value.to_string());
        Ok(())
    }
    
    /// Print formatted string to stdout
    pub fn printf(format: &str, args: &[Value]) -> Result<(), String> {
        // Simple printf implementation - replace {} with arguments
        let mut result = format.to_string();
        
        // Flatten arrays in arguments
        let mut flattened_args = Vec::new();
        for arg in args {
            match arg {
                Value::Vec(vec) => {
                    // Unpack array elements
                    for item in vec {
                        flattened_args.push(item.clone());
                    }
                },
                _ => {
                    // Keep non-array arguments as-is
                    flattened_args.push(arg.clone());
                }
            }
        }
        
        // Handle indexed placeholders like {0}, {1}, etc.
        for (i, arg) in flattened_args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, &arg.to_string());
        }
        
        // Handle single {} without index (only if no indexed placeholders were found)
        if !result.contains("{0}") && !result.contains("{1}") && !result.contains("{2}") {
            for arg in &flattened_args {
                if result.contains("{}") {
                    result = result.replacen("{}", &arg.to_string(), 1);
                } else {
                    break;
                }
            }
        }
        
        print!("{}", result);
        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(())
    }
    
    /// Read a line from stdin
    pub fn read_line() -> Result<String, String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        Ok(input.trim().to_string())
    }
    
    /// Read a character from stdin
    pub fn read_char() -> Result<char, String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        input.chars().next().ok_or("No character read".to_string())
    }
    
    /// Read an integer from stdin
    pub fn read_int() -> Result<i64, String> {
        let input = Self::read_line()?;
        input.parse::<i64>().map_err(|e| e.to_string())
    }
    
    /// Read a float from stdin
    pub fn read_float() -> Result<f64, String> {
        let input = Self::read_line()?;
        input.parse::<f64>().map_err(|e| e.to_string())
    }
    
    /// Read a boolean from stdin
    pub fn read_bool() -> Result<bool, String> {
        let input = Self::read_line()?.to_lowercase();
        match input.as_str() {
            "true" | "1" | "yes" | "y" | "on" => Ok(true),
            "false" | "0" | "no" | "n" | "off" => Ok(false),
            _ => Err("Invalid boolean value".to_string()),
        }
    }




    /// print with color
    pub fn print_color<C: IntoColor>(text: &str, color: C) -> Result<(), String> {
        let color_enum = color.into_color()?;
    
        let color_code = match color_enum {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Black => "\x1b[30m",
            Color::Reset => "\x1b[0m",
        };
    
        print!("{}{}{}", color_code, text, "\x1b[0m");
        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(())
    }
    
    /// Print error message in red
    pub fn print_error(message: &str) -> Result<(), String> {
        Self::print_color(&format!("Error: {}", message), Color::Red)
    }
    
    /// Print warning message in yellow
    pub fn print_warning(message: &str) -> Result<(), String> {
        Self::print_color(&format!("Warning: {}", message), Color::Yellow)
    }
    
    /// Print success message in green
    pub fn print_success(message: &str) -> Result<(), String> {
        Self::print_color(&format!("Success: {}", message), Color::Green)
    }
    
    /// Print info message in blue
    pub fn print_info(message: &str) -> Result<(), String> {
        Self::print_color(&format!("Info: {}", message), Color::Blue)
    }
    
    /// Print debug message in cyan
    pub fn print_debug(message: &str) -> Result<(), String> {
        Self::print_color(&format!("Debug: {}", message), Color::Cyan)
    }
    
    /// Clear the screen
    pub fn clear_screen() -> Result<(), String> {
        print!("\x1b[2J\x1b[H");
        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(())
    }
    
    /// Move cursor to position
    pub fn move_cursor(row: u16, col: u16) -> Result<(), String> {
        print!("\x1b[{};{}H", row, col);
        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(())
    }
    
    /// Hide cursor
    pub fn hide_cursor() -> Result<(), String> {
        print!("\x1b[?25l");
        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(())
    }
    
    /// Show cursor
    pub fn show_cursor() -> Result<(), String> {
        print!("\x1b[?25h");
        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(())
    }
    
    /// Get terminal size
    pub fn get_terminal_size() -> Result<(u16, u16), String> {
        use terminal_size::{Width, Height, terminal_size};
        
        match terminal_size() {
            Some((Width(w), Height(h))) => Ok((w, h)),
            None => {
                // Fallback to environment variables or default values
                if let Ok(columns) = std::env::var("COLUMNS") {
                    if let Ok(cols) = columns.parse::<u16>() {
                        if let Ok(rows) = std::env::var("LINES") {
                            if let Ok(rows) = rows.parse::<u16>() {
                                return Ok((cols, rows));
                            }
                        }
                        return Ok((cols, 24)); // Default height
                    }
                }
                // Final fallback
                Ok((80, 24))
            }
        }
    }
    
    /// Check if output is a terminal
    pub fn is_terminal() -> bool {
        atty::is(atty::Stream::Stdout)
    }
    
    /// Check if input is a terminal
    pub fn is_input_terminal() -> bool {
        atty::is(atty::Stream::Stdin)
    }
    
    /// Create a progress bar
    pub fn create_progress_bar(total: u64) -> ProgressBar {
        ProgressBar::new(total)
    }
    
    /// Create a spinner
    pub fn create_spinner() -> Spinner {
        Spinner::new()
    }
    
    /// Prompt user for input with message
    pub fn prompt(message: &str) -> Result<String, String> {
        print!("{}: ", message);
        io::stdout().flush().map_err(|e| e.to_string())?;
        Self::read_line()
    }
    
    /// Prompt user for input with default value
    pub fn prompt_with_default(message: &str, default: &str) -> Result<String, String> {
        print!("{} [{}]: ", message, default);
        io::stdout().flush().map_err(|e| e.to_string())?;
        let input = Self::read_line()?;
        if input.is_empty() {
            Ok(default.to_string())
        } else {
            Ok(input)
        }
    }
    
    /// Confirm with yes/no prompt
    pub fn confirm(message: &str) -> Result<bool, String> {
        let input = Self::prompt_with_default(message, "y")?;
        Ok(input.to_lowercase().starts_with('y'))
    }
    
    /// Select from options
    pub fn select(message: &str, options: &[String]) -> Result<usize, String> {
        println!("{}", message);
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }
        let input = Self::read_line()?;
        let choice: usize = input.parse().map_err(|_| "Invalid selection".to_string())?;
        if choice > 0 && choice <= options.len() {
            Ok(choice - 1)
        } else {
            Err("Invalid selection".to_string())
        }
    }
    
    /// Multi-select from options
    pub fn multi_select(message: &str, options: &[String]) -> Result<Vec<usize>, String> {
        println!("{} (comma-separated indices)", message);
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }
        let input = Self::read_line()?;
        let indices: Result<Vec<usize>, _> = input
            .split(',')
            .map(|s| s.trim().parse::<usize>())
            .collect();
        let indices = indices.map_err(|_| "Invalid selection".to_string())?;
        for &idx in &indices {
            if idx == 0 || idx > options.len() {
                return Err("Invalid selection".to_string());
            }
        }
        Ok(indices.into_iter().map(|i| i - 1).collect())
    }
}

/// Color enum for terminal output
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,
    Reset,
}


impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "white" => Ok(Color::White),
            "black" => Ok(Color::Black),
            "reset" => Ok(Color::Reset),
            _ => Err(format!("Unknown color: {}", s)),
        }
    }
}

pub trait IntoColor {
    fn into_color(self) -> Result<Color, String>;
}

impl IntoColor for Color {
    fn into_color(self) -> Result<Color, String> {
        Ok(self)
    }
}

impl<'a> IntoColor for &'a str {
    fn into_color(self) -> Result<Color, String> {
        self.parse::<Color>()
    }
}
/// Progress bar for long operations
pub struct ProgressBar {
    total: u64,
    current: u64,
    width: u16,
}

impl ProgressBar {
    pub fn new(total: u64) -> Self {
        Self {
            total,
            current: 0,
            width: 50,
        }
    }
    
    pub fn update(&mut self, current: u64) -> Result<(), String> {
        self.current = current;
        self.display()
    }
    
    pub fn increment(&mut self) -> Result<(), String> {
        self.current += 1;
        self.display()
    }
    
    fn display(&self) -> Result<(), String> {
        let percentage = (self.current as f64 / self.total as f64) * 100.0;
        let filled = (self.current as f64 / self.total as f64 * self.width as f64) as u16;
        
        print!("\r[");
        for i in 0..self.width {
            if i < filled {
                print!("=");
            } else {
                print!(" ");
            }
        }
        print!("] {:.1}% ({}/{})", percentage, self.current, self.total);
        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(())
    }
    
    pub fn finish(&self) -> Result<(), String> {
        println!();
        Ok(())
    }
}

/// Spinner for indeterminate operations
pub struct Spinner {
    frames: Vec<&'static str>,
    current: usize,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            current: 0,
        }
    }
    
    pub fn spin(&mut self, message: &str) -> Result<(), String> {
        print!("\r{} {}", self.frames[self.current], message);
        io::stdout().flush().map_err(|e| e.to_string())?;
        self.current = (self.current + 1) % self.frames.len();
        Ok(())
    }
    
    pub fn stop(&self) -> Result<(), String> {
        print!("\r");
        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// Stream operations
pub struct Stream {
    reader: Option<BufReader<File>>,
    writer: Option<BufWriter<File>>,
}

impl Stream {
    pub fn new_reader(file: File) -> Self {
        Self {
            reader: Some(BufReader::new(file)),
            writer: None,
        }
    }
    
    pub fn new_writer(file: File) -> Self {
        Self {
            reader: None,
            writer: Some(BufWriter::new(file)),
        }
    }
    
    pub fn read_line(&mut self) -> Result<Option<String>, String> {
        if let Some(reader) = &mut self.reader {
            let mut line = String::new();
            let bytes = reader.read_line(&mut line).map_err(|e| e.to_string())?;
            if bytes == 0 {
                Ok(None)
            } else {
                Ok(Some(line.trim().to_string()))
            }
        } else {
            Err("No reader available".to_string())
        }
    }
    
    pub fn write_line(&mut self, line: &str) -> Result<(), String> {
        if let Some(writer) = &mut self.writer {
            writeln!(writer, "{}", line).map_err(|e| e.to_string())?;
            writer.flush().map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("No writer available".to_string())
        }
    }
    
    pub fn write(&mut self, data: &str) -> Result<(), String> {
        if let Some(writer) = &mut self.writer {
            write!(writer, "{}", data).map_err(|e| e.to_string())?;
            writer.flush().map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("No writer available".to_string())
        }
    }
}

/// Buffer operations
pub struct Buffer {
    data: Vec<u8>,
    position: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            position: 0,
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            position: 0,
        }
    }
    
    pub fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        self.data.extend_from_slice(data);
        Ok(data.len())
    }
    
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        let available = self.data.len() - self.position;
        let to_read = buf.len().min(available);
        
        if to_read > 0 {
            buf[..to_read].copy_from_slice(&self.data[self.position..self.position + to_read]);
            self.position += to_read;
        }
        
        Ok(to_read)
    }
    
    pub fn seek(&mut self, position: usize) -> Result<(), String> {
        if position <= self.data.len() {
            self.position = position;
            Ok(())
        } else {
            Err("Position out of bounds".to_string())
        }
    }
    
    pub fn tell(&self) -> usize {
        self.position
    }
    
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    pub fn clear(&mut self) {
        self.data.clear();
        self.position = 0;
    }
    
    pub fn to_string(&self) -> Result<String, String> {
        String::from_utf8(self.data.clone()).map_err(|e| e.to_string())
    }
}


