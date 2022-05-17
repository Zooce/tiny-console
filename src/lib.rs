use console::Term;
pub use console::{Color, Style};
use std::io::Result;

pub struct TinyConsole {
    terminal: Term,
}

impl TinyConsole {
    pub fn write(&self, text: &str) -> Result<&Self> {
        self.terminal.write_str(text)?;
        Ok(&self)
    }

    pub fn cwrite(&self, text: &str, color: Color) -> Result<&Self> {
        self.swrite(text, &Style::new().fg(color))
    }

    pub fn swrite(&self, text: &str, style: &Style) -> Result<&Self> {
        let styled = format!("{}", style.apply_to(text));
        self.write(&styled)
    }

    pub fn writeln(&self, text: &str) -> Result<&Self> {
        self.terminal.write_line(text)?;
        Ok(&self)
    }

    pub fn cwriteln(&self, text: &str, color: Color) -> Result<&Self> {
        self.swriteln(text, &Style::new().fg(color))
    }

    pub fn swriteln(&self, text: &str, style: &Style) -> Result<&Self> {
        let styled = format!("{}", style.apply_to(text));
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
