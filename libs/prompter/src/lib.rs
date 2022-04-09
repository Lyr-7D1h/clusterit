use std::io;

pub fn input(message: &str) -> Result<Option<String>, io::Error> {
    println!("{message}");
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;

    if buf == "\n" {
        return Ok(None);
    }

    return Ok(Some(buf));
}
