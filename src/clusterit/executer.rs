use super::connection::Connection;
use anyhow::Result;

pub struct Executer<'a> {
    connection: Connection<'a>,
}

impl<'a> Executer<'a> {
    pub fn new(connection: Connection) -> Executer {
        Executer { connection }
    }

    pub fn install(self, debpkgs: Vec<String>) -> Result<()> {
        // NOTE: might not be
        let r = self.connection.exec("apt-mark showmanual")?;
        let packages: Vec<&str> = r.split("\n").collect();

        // self.connection
        //     .exec(&format!("apt-get install {}", debpkgs.join(" ")))?;

        println!("{packages:?}");
        Ok(())
    }
}
