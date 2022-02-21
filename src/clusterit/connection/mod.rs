use std::hash::Hash;

use log::debug;

use ssh::Session;

mod connection_error;

pub use connection_error::ConnectionError;

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
    pub fn connect_to_destination(destination: &str) -> Result<Connection, ConnectionError> {
        debug!("Connecting to '{destination}'",);

        let destination = destination
            .strip_prefix("ssh://")
            .ok_or(ConnectionError::ParseError("Invalid destination"))?;

        let mut session =
            Session::new().or(Err(ssh::Error::Ssh("Could not create session".into())))?;

        let mut parts = destination.split("@");
        if parts.clone().count() >= 2 {
            let user = parts.next().unwrap();
            session.set_username(user);

            let mut dest = parts.next().unwrap().split(":");
            let hostname = dest.next().unwrap();
            session.set_host(hostname);

            let port = dest.next();
            if let Some(port) = port {
                let port = port
                    .parse()
                    .or(Err(ConnectionError::ParseError("Invalid port")))?;
                session.set_port(port);
            }
        } else {
            session.set_host(destination);
        }

        session.userauth_publickey_auto(None)?;

        session.connect()?;

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
