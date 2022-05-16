use std::io;

pub fn main() -> io::Result<()> {
    using_console()?;
    using_tiny_console()
}

fn using_console() -> io::Result<()> {
    use console::{Term, style, Style, Color};

    let term = Term::buffered_stdout();

    let white_on_green = Style::new().fg(Color::White).on_green();

    term.write_str("Hello, ")?;
    term.write_str(&format!("{}", white_on_green.apply_to("friend")))?;
    term.write_line("!")?;
    term.flush()?;

    term.write_str("What's your ")?;
    term.write_str(&format!("{}", style("name").fg(Color::Cyan)))?;
    term.write_str("? ")?;
    term.flush()?;

    let name = term.read_line()?;

    term.write_str("Nice to meet you, ")?;
    term.write_line(&format!("{}", white_on_green.apply_to(&name)))?;
    term.flush()?;

    term.write_str("What's the secret code? ")?;
    term.flush()?;

    let secret = term.read_secure_line()?;

    term.write_line(&format!("Nope. The secrect code is not: {}", secret))?;
    term.flush()?;

    Term::stderr().write_line(&format!("Invalid secret code: {}", secret))?;

    Ok(())
}

fn using_tiny_console() -> io::Result<()> {
    use tiny_console::{self as tc, Color, Style};

    let term = tc::buffered_stdout();

    let white_on_green = Style::new().fg(Color::White).on_green();

    term.write("Hello, ")?
        .swrite("friend", &white_on_green)?
        .writeln("!")?
        .flush()?;

    term.write("What's your ")?
        .cwrite("name", Color::Cyan)?
        .write("? ")?
        .flush()?;

    let name = term.read()?;

    term.write("Nice to meet you, ")?
        .swriteln(&name, &white_on_green)?
        .flush()?;

    term.write("What's the secret code? ")?.flush()?;

    let secret = term.secread()?;

    term.write("Nope. The secret code is not: ")?.writeln(&secret)?.flush()?;

    tc::stderr().writeln(&format!("Invalid secret code: {}", &secret))?;

    Ok(())
}
