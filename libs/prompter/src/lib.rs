use std::io::{self, Write};

pub fn ask_secret(message: &str) -> Result<String, io::Error> {
    print!("{message}");

    print!(": ");
    io::stdout().flush()?;

    let mut buf = String::new();

    // TODO Dont show input
    io::stdin().read_line(&mut buf)?;

    if buf == "\n" {
        return Ok(String::new());
    }

    println!("");

    return Ok(buf);
}

pub fn input(message: &str, default: Option<&str>) -> Result<Option<String>, io::Error> {
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
macro_rules! input {
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
