use std::io::{self, Write};

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
        input(message, None);
    };
    ($message:expr, $default:expr) => {
        input($message, $default);
    };
    ($message:expr, $default:expr) => {
        input($message, Some($default));
    };
}
