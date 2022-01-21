use std::path::PathBuf;

use anyhow::Result;

mod config;
mod connection;
mod executer;

use config::Config;
use connection::Connection;
use executer::Executer;

pub struct Clusterit {
    config: Config,
}

impl Clusterit {
    pub fn from_file(path: &PathBuf) -> Result<Clusterit> {
        let config = Config::from_file(path)?;
        println!("{:?}", config);

        Ok(Clusterit { config })
    }

    pub fn setup(self) -> Result<()> {
        let mut connection = Connection::new(self.config.ip, self.config.port as usize)?;

        connection.connect(self.config.user.as_deref(), self.config.password.as_deref())?;

        let e = Executer::new(connection);

        // e.install(self.config.debpkgs);

        todo!()

        // let (sender, receiver) = mpsc::channel();

        // for (device_name, server) in self.config.servers.into_iter() {

        //     let sender = sender.clone();
        //     thread::spawn(move || {
        //         debug!("Spawned new thread");
        //         let mut session = Session::new().unwrap();
        //         session.set_host(&server.ip).unwrap();
        //         session.parse_config(None).unwrap();
        //         session.connect().unwrap();
        //         println!("asdf");
        //         session.userauth_password("asdf").unwrap();
        //         session.userauth_publickey_auto(Some("asdf")).unwrap();

        //         session.disconnect().unwrap();
        //         sender.send("asdf").unwrap();
        //     })
        //     .join()
        //     .unwrap();
        // }
        // debug!("{:?}", receiver.recv());

        // Ok(())
    }
}
