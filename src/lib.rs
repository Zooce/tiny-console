use console::Term;
pub use console::{Color, Style};
use std::io::Result;

pub enum Mode {
    Stdout,
    Stderr,
    BufferedStdout,
    BufferedStderr,
}

pub struct TinyConsole {
    terminal: Term,
}

impl TinyConsole {
    pub fn new(mode: Mode) -> Self {
        Self {
            terminal: match mode {
                Mode::Stdout => Term::stdout(),
                Mode::Stderr => Term::stderr(),
                Mode::BufferedStdout => Term::buffered_stdout(),
                Mode::BufferedStderr => Term::buffered_stderr(),
            }
        }
    }

    pub fn write(&self, s: &str) -> Result<&Self> {
        self.terminal.write_str(s)?;
        Ok(&self)
    }

    pub fn cwrite(&self, s: &str, c: Color) -> Result<&Self> {
        self.swrite(s, &Style::new().fg(c))
    }

    pub fn swrite(&self, s: &str, t: &Style) -> Result<&Self> {
        let styled = format!("{}", t.apply_to(s));
        self.write(&styled)
    }

    pub fn writeln(&self, s: &str) -> Result<&Self> {
        self.terminal.write_line(s)?;
        Ok(&self)
    }

    pub fn cwriteln(&self, s: &str, c: Color) -> Result<&Self> {
        self.swriteln(s, &Style::new().fg(c))
    }

    pub fn swriteln(&self, s: &str, t: &Style) -> Result<&Self> {
        let styled = format!("{}", t.apply_to(s));
        self.writeln(&styled)
    }

    pub fn clearln(&self) -> Result<&Self> {
        self.terminal.clear_line()?;
        Ok(&self)
    }

    pub fn flush(&self) -> Result<&Self> {
        self.terminal.flush()?;
        Ok(&self)
    }

    pub fn read(&self) -> Result<String> {
        self.terminal.read_line()
    }

    pub fn secread(&self) -> Result<String> {
        self.terminal.read_secure_line()
    }
}
