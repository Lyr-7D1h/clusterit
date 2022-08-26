use core::time;
use std::{
    env::{self, join_paths},
    fs::{self, ReadDir},
    hash::Hash,
    net::TcpStream,
    path::{self, Path, PathBuf},
    time::SystemTime,
};

use log::debug;

mod connection_error;

pub use connection_error::ConnectionError;

mod authenticate;
use authenticate::authenticate;
use ssh2::Session;

pub struct Connection {
    session: Session,
}

#[derive(Debug)]
pub struct ExecResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl Connection {
    /// Connect using OpenSSH definition of destination (ssh://[user@]hostname[:port.])
    /// If not public or private key found it will try
    /// ssh-agent > most recent key in ~/.ssh > ask user input for password
    pub fn connect_to_destination(
        destination: &str,
        public_key: Option<&str>,
        private_key: Option<&str>,
    ) -> Result<Connection, ConnectionError> {
        debug!("Connecting to '{destination}'",);

        let mut destination = match destination.strip_prefix("ssh://") {
            Some(d) => d,
            None => destination,
        };

        let mut user = "root";

        let mut parts = destination.split("@");
        if parts.clone().count() >= 2 {
            user = parts.next().unwrap();
            destination = parts.next().unwrap();
        }

        let mut session = Session::new()?;

        debug!("Creating TcpStream to: {destination}");
        let tcp = TcpStream::connect(destination).unwrap();

        session.set_tcp_stream(tcp);

        debug!("Performing handshake");
        session.handshake()?;

        let hostname = destination.split(":").next().unwrap();
        authenticate(&session, user, hostname, public_key, private_key)?;

        Ok(Connection { session })
    }

    // pub fn exec(mut self, command: &str) -> Result<String, ConnectionError> {
    //     let mut s = match self.exec_channel {
    //         Some(s) => s,
    //         None => {
    //             let mut s = self.session.channel_new()?;
    //             s.open_session()?;
    //             s
    //         }
    //     };

    //     s.request_exec(command.as_bytes()).unwrap();
    //     s.send_eof().unwrap();

    //     let mut buf = Vec::new();
    //     s.stdout().read_to_end(&mut buf).unwrap();
    //     return Ok(String::from_utf8_lossy(&buf).into_owned());
    // }

    pub fn write(mut self) {
        todo!()
    }

    pub fn read(mut self) {
        todo!()
    }

    // pub fn exec(&self, command: &str) -> anyhow::Result<ExecResult> {
    //     let mut channel = self.session.channel_session()?;
    //     channel.exec(command)?;

    //     let mut stdout = String::new();
    //     channel.read_to_string(&mut stdout)?;

    //     let mut stderr = String::new();
    //     channel.stderr().read_to_string(&mut stderr)?;

    //     channel.wait_close()?;

    //     return Ok(ExecResult {
    //         exit_code: channel.exit_status()?,
    //         stdout,
    //         stderr,
    //     });
    // }
}
