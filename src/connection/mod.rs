use std::{io::Read, net::TcpStream};

use log::debug;

mod destination;
pub use destination::Destination;

mod connection_error;
pub use connection_error::ConnectionError;

mod authenticate;
pub use authenticate::Authentication;
use authenticate::{authenticate, authenticate_interactive};

use ssh2::{Channel, Session};

pub struct Connection {
    session: Session,
    exec_channel: Option<Channel>,
}

impl Connection {
    /// Connect using libssh2 with the given authentication method
    pub fn connect(
        destination: &Destination,
        authentication: &Authentication,
    ) -> Result<Connection, ConnectionError> {
        let mut session = Session::new()?;

        debug!("Creating TcpStream to: {destination}");
        let tcp = TcpStream::connect(destination).unwrap();

        session.set_tcp_stream(tcp);

        debug!("Performing handshake");
        session.handshake()?;

        authenticate(&session, destination, &authentication)?;

        Ok(Connection {
            session,
            exec_channel: None,
        })
    }

    /// Connect using OpenSSH definition of destination (ssh://[user@]hostname[:port.])
    /// If not public or private key found it will try
    /// ssh-agent > most recent key in ~/.ssh > ask user input for password
    pub fn connect_interactive(destination: &Destination) -> Result<(Connection, Authentication), ConnectionError> {
        debug!("Connecting to '{destination}'",);

        let mut session = Session::new()?;

        debug!("Creating TcpStream to: {destination}");
        let tcp = TcpStream::connect(destination).unwrap();

        session.set_tcp_stream(tcp);

        debug!("Performing handshake");
        session.handshake()?;

        let authentication = authenticate_interactive(&session, destination)?;

        Ok((
            Connection {
                session,
                exec_channel: None,
            },
            authentication,
        ))
    }

    /// Execute a command on the given connection
    pub fn exec(&mut self, command: &str) -> Result<String, ConnectionError> {
        let channel = match &mut self.exec_channel {
            Some(c) => c,
            None => {
                self.exec_channel = Some(self.session.channel_session()?);
                self.exec_channel.as_mut().unwrap()
            }
        };

        channel.exec(command).unwrap();

        let mut buf = String::new();
        channel.read_to_string(&mut buf)?;

        return Ok(buf);
    }

    pub fn write(mut self) {
        todo!()
    }

    pub fn read(mut self) {
        todo!()
    }
}
