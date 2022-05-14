use console::Term;
pub use console::{Color, Style};
use std::io::Result;

pub enum Target {
    Stdout,
    Stderr,
}

pub struct TinyConsole {
    terminal: Term,
}

impl TinyConsole {
    pub fn new(target: Target, buffered: bool) -> Self {
        Self {
            terminal: match target {
                Target::Stdout => if buffered { Term::buffered_stdout() } else { Term::stdout() },
                Target::Stderr => if buffered { Term::buffered_stderr() } else { Term::stderr() },
            },
        }
    }

    pub fn write(&self, s: &str) -> Result<()> {
        self.terminal.write_str(s)
    }

    pub fn cwrite(&self, s: &str, c: Color) -> Result<()> {
        self.swrite(s, &Style::new().fg(c))
    }

    pub fn swrite(&self, s: &str, t: &Style) -> Result<()> {
        let styled = format!("{}", t.apply_to(s));
        self.write(&styled)
    }

    pub fn writeln(&self, s: &str) -> Result<()> {
        self.terminal.write_line(s)
    }

    pub fn cwriteln(&self, s: &str, c: Color) -> Result<()> {
        self.swriteln(s, &Style::new().fg(c))
    }

    pub fn swriteln(&self, s: &str, t: &Style) -> Result<()> {
        let styled = format!("{}", t.apply_to(s));
        self.writeln(&styled)
    }

    pub fn clearln(&self) -> Result<()> {
        self.terminal.clear_line()
    }

    pub fn flush(&self) -> Result<()> {
        self.terminal.flush()
    }
}
