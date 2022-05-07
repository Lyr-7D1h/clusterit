use std::io::{self, stdin, Write};

pub fn input(message: &str, default: Option<&str>) -> Result<Option<String>, io::Error> {
    println!("{message}");

    if let Some(default) = default {
        println!("\\033[2J asdf");
        print!("{default}\r");
        io::stdout().flush()?;
    }

    let mut buf = String::from(default.unwrap_or(""));

    io::stdin().read_line(&mut buf)?;

    if buf == "\n" {
        return Ok(None);
    }

    return Ok(Some(buf));
}

#[macro_export]
macro_rules! input {
    ($message:expr) => {
        input(message, None);
    };
    ($message:expr, $default:expr) => {
        input($message, $default);
    };
    ($message:expr, $default:expr) => {
        input($message, Some($default));
    };
}
