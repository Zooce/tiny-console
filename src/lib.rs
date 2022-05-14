use console::{Term, style};
pub use console::{Color, Style};
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

    pub fn cwrite(&self, s: &str, c: Color) -> Result<()> {
        let styled = format!("{}", style(s).fg(c));
        self.write(&styled)
    }

    pub fn writeln(&self, s: &str) -> Result<()> {
        self.terminal.write_line(s)
    }

    pub fn cwriteln(&self, s: &str, c: Color) -> Result<()> {
        let styled = format!("{}", style(s).fg(c));
        self.writeln(&styled)
    }

    pub fn clearln(&self) -> Result<()> {
        self.terminal.clear_line()
    }
}
