use console::Term;
use std::io::Result;

pub struct TinyConsole {
    terminal: Term,
}

impl TinyConsole {
    pub fn stdout() -> Self {
        Self {
            terminal: Term::stdout(),
        }
    }

    pub fn write(&self, s: &str) -> Result<()> {
        self.terminal.write_str(s)
    }

    pub fn writeln(&self, s: &str) -> Result<()> {
        self.terminal.write_line(s)
    }

    pub fn clearln(&self) -> Result<()> {
        self.terminal.clear_line()
    }
}
