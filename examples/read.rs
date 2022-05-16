use tiny_console::*;

pub fn main() -> std::io::Result<()> {
    let term = TinyConsole::new(Mode::BufferedStdout);

    let c = Color::Cyan;

    term.write("What's your ")?
        .cwrite("name", c)?
        .write("? ")?
        .flush()?;

    let name = term.read()?;

    term.write("Nice to meet you, ")?
        .cwriteln(&name, c)?
        .flush()?;

    Ok(())
}
