use std::io;

pub fn main() -> io::Result<()> {
    using_console()?;
    using_tiny_console()?;
    using_maybe_console()?;

    Ok(())
}

fn using_console() -> io::Result<()> {
    use console::{Term, style, Style, Color};

    let term = Term::buffered_stdout();

    term.write_str(">>> Using ")?;
    term.write_str(&format!("{}", style("console").fg(Color::Yellow)))?;
    term.write_line(" <<<")?;

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

    term.write(">>> Using ")?
        .cwrite("tiny-console", Color::Yellow)?
        .writeln(" <<<")?
        .flush()?;

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


fn using_maybe_console() -> io::Result<()> {
    use tiny_console::{maybe, Color, Style};

    let mut term = maybe::stdout();

    term.text(">>> Using ").yellow("maybe-console").text(" <<<")
        .writeln()?;

    let white_on_green = Style::new().fg(Color::White).on_green();

    term.text("Hello, ").styled("friend", &white_on_green).text("!")
        .writeln()?;

    term.text("What's your ").cyan("name").text("? ")
        .write()?;

    let name = term.read()?;

    term.text("Nice to meet you, ").styled(&name, &white_on_green)
        .writeln()?;

    term.text("What's the secret code? ").write()?;

    let secret = term.secread()?;

    term.text("Nope. The secret code is not: ").text(&secret).writeln()?;

    maybe::stderr().text(&format!("Invalid secret code: {}", &secret)).writeln()?;

    Ok(())
}
