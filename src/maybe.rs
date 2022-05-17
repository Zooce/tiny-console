
use std::io;
use console::{Style, style, Term};

pub struct MaybeConsole {
    buffer: Vec<String>,
    terminal: Term,
}

macro_rules! style_func {
    ($func:ident) => {
        pub fn $func(&mut self, t: &str) -> &mut Self {
            self.text(&style(t).$func().to_string());
            self
        }
    };
}

impl MaybeConsole {
    pub fn text(&mut self, t: &str) -> &mut Self {
        self.buffer.push(t.to_string());
        self
    }

    pub fn styled(&mut self, t: &str, s: &Style) -> &mut Self {
        self.text(&s.apply_to(t).to_string());
        self
    }

    style_func!(black);
    style_func!(red);
    style_func!(green);
    style_func!(yellow);
    style_func!(blue);
    style_func!(magenta);
    style_func!(cyan);
    style_func!(white);

    pub fn color265(&mut self, t: &str, c: u8) -> &mut Self {
        self.text(&style(t).color256(c).to_string());
        self
    }

    style_func!(bold);
    style_func!(dim);
    style_func!(italic);
    style_func!(underlined);
    style_func!(blink);
    style_func!(reverse);
    style_func!(hidden);

    // left

    // center

    // right

    pub fn write(&mut self) -> io::Result<()> {
        self.terminal.write_str(&self.buffer.join(""))?;
        self.terminal.flush()?;
        self.buffer.clear();
        Ok(())
    }

    pub fn writeln(&mut self) -> io::Result<()> {
        self.terminal.write_line(&self.buffer.join(""))?;
        self.terminal.flush()?;
        self.buffer.clear();
        Ok(())
    }

    pub fn read(&self) -> io::Result<String> {
        self.terminal.read_line()
    }

    pub fn secread(&self) -> io::Result<String> {
        self.terminal.read_secure_line()
    }
}

pub fn stdout() -> MaybeConsole {
    MaybeConsole { buffer: Vec::new(), terminal: Term::stdout() }
}

pub fn stderr() -> MaybeConsole {
    MaybeConsole { buffer: Vec::new(), terminal: Term::stderr() }
}
