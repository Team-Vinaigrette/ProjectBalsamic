use std::io::Write;

/// Struct used to define the structure that should be passed to `write`
pub struct WritableString {
    /// The string to write to the console.
    pub str: String,

    /// The x position to write to.
    pub x: usize,
    /// The y position to write to.
    pub y: usize,
}

/// Struct used to manage a given console.
pub struct ConsoleManager {
    /// Reference to the terminal buffer.
    term: console::Term,
}

impl ConsoleManager {
    /// Console manager constructor.
    ///
    /// # Returns
    /// * `ConsoleManager` - The generated console manager instance.
    pub fn new() -> ConsoleManager {
        let mut manager = ConsoleManager {
            term: console::Term::buffered_stdout()
        };
        // Hide the cursor, ignoring errors.
        manager.term.hide_cursor().ok();
        return manager;
    }

    /// Clears the terminal output.
    ///
    /// # Returns
    /// * `Result<()>` - A Result object failed if any error occurred.
    pub fn clear(&self) -> Result<(), std::io::Error> {
        return self.term.clear_screen();
    }

    /// Writes the provided string on the screen.
    ///
    /// # Arguments
    /// * `string: &WritableString` - A reference to the string to write to the terminal.
    ///
    /// # Returns
    /// * `Result<()>` - A Result object failed if any error occurred.
    pub fn write(&mut self, string: &WritableString) -> Result<(), std::io::Error> {
        // Write the string to the buffered terminal.
        self.term.move_cursor_to(string.x, string.y)?;
        self.term.write_all(string.str.as_bytes())?;

        return Ok(());
    }

    /// Flushes the data found in the terminal buffer.
    ///
    /// # Returns
    /// * `Result<()>` - The results of the flush event.
    pub fn flush(&self) -> Result<(), std::io::Error> {
        self.term.move_cursor_to(0, usize::from(self.term.size().1) + 1)?;
        // self.term.clear_to_end_of_screen()?;
        return self.term.flush();
    }

    /// Returns the width of the terminal.
    ///
    /// # Returns
    /// * `usize` - The width of the terminal.
    pub fn width(&self) -> usize {
        return usize::from(self.term.size().0);
    }

    /// Returns the height of the terminal.
    ///
    /// # Returns
    /// * `usize` - The height of the terminal.
    pub fn height(&self) -> usize {
        return usize::from(self.term.size().1);
    }
}
