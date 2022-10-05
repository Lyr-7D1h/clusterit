use std::io::{self, Read, Write};

pub fn ask_secret(message: &str) -> Result<String, io::Error> {
    print!("{message}: ");
    io::stdout().flush()?;

    let mut buf = String::new();

    let lock = io::stdin().bytes();

    // TODO Dont show input
    io::stdin().read_line(&mut buf)?;

    if buf == "\n" {
        return Ok(String::new());
    }

    buf = buf.trim().to_string();

    return Ok(buf);
}

pub fn ask(message: &str, default: Option<&str>) -> Result<Option<String>, io::Error> {
    print!("{message}");

    if let Some(default) = default.clone() {
        print!(" ({default})");
    }

    print!(": ");
    io::stdout().flush()?;

    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;

    if buf == "\n" {
        if let Some(default) = default {
            return Ok(Some(String::from(default)));
        }

        return Ok(None);
    }

    println!("");

    return Ok(Some(buf));
}

#[macro_export]
macro_rules! ask {
    ($message:expr) => {
        input($message, None);
    };
    ($message:expr, $default:expr) => {
        input($message, $default);
    };
    ($message:expr, $default:expr) => {
        input($message, Some($default));
    };
}
