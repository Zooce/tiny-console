use console::Term;
pub use console::{Color, Style};
use std::io::Result;

pub struct TinyConsole {
    terminal: Term,
}

impl TinyConsole {
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

pub fn stdout() -> TinyConsole {
    TinyConsole{ terminal: Term::stdout() }
}

pub fn buffered_stdout() -> TinyConsole {
    TinyConsole{ terminal: Term::buffered_stdout() }
}

pub fn stderr() -> TinyConsole {
    TinyConsole{ terminal: Term::stderr() }
}

pub fn buffered_stderr() -> TinyConsole {
    TinyConsole{ terminal: Term::buffered_stderr() }
}
