use std::net::TcpStream;

use log::debug;

mod destination;

pub use destination::Destination;

mod connection_error;

pub use connection_error::ConnectionError;

mod authenticate;
use authenticate::authenticate;
use ssh2::Session;

use crate::connection::authenticate::authenticate_interactive;

pub struct Connection {
    session: Session,
}

// #[derive(Debug)]
// pub struct ExecResult {
//     pub exit_code: i32,
//     pub stdout: String,
//     pub stderr: String,
// }

impl Connection {
    pub fn connect(
        destination: &Destination,
        public_key: &str,
        private_key: &str,
    ) -> Result<Connection, ConnectionError> {
        let mut session = Session::new()?;

        debug!("Creating TcpStream to: {destination}");
        let tcp = TcpStream::connect(destination).unwrap();

        session.set_tcp_stream(tcp);

        debug!("Performing handshake");
        session.handshake()?;

        authenticate(&session, destination, public_key, private_key)?;

        Ok(Connection { session })
    }
    /// Connect using OpenSSH definition of destination (ssh://[user@]hostname[:port.])
    /// If not public or private key found it will try
    /// ssh-agent > most recent key in ~/.ssh > ask user input for password
    pub fn connect_interactive(destination: &Destination) -> Result<Connection, ConnectionError> {
        debug!("Connecting to '{destination}'",);

        let mut session = Session::new()?;

        debug!("Creating TcpStream to: {destination}");
        let tcp = TcpStream::connect(destination).unwrap();

        session.set_tcp_stream(tcp);

        debug!("Performing handshake");
        session.handshake()?;

        authenticate_interactive(&session, destination)?;

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
