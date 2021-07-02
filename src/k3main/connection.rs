use std::{io::Read, net::TcpStream};

use log::debug;

const DEFAULT_USERS: &'static [&'static str] = &["root", "pi"];
const DEFAULT_PASSWORDS: &'static [&'static str] = &["4321", "raspberry"];

pub struct Connection {
    session: ssh2::Session,
}

#[derive(Debug)]
pub struct ExecResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl Connection {
    pub fn from(session: ssh2::Session) -> Connection {
        Connection { session }
    }

    pub fn connect_using_default(destination: &str) -> anyhow::Result<Connection> {
        let tcp = TcpStream::connect(destination)?;
        let mut session = ssh2::Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;

        for user in DEFAULT_USERS.iter() {
            for password in DEFAULT_PASSWORDS.iter() {
                session.userauth_password(user, password).unwrap_or(());
                if session.authenticated() {
                    return Ok(Connection::from(session));
                }
            }
        }

        Err(anyhow::Error::msg(
            "Could not login using default ssh credentials",
        ))
    }

    pub fn connect(destination: &str) -> anyhow::Result<Connection> {
        debug!("Using standard connection on: {}", destination);
        let tcp = TcpStream::connect(destination)?;
        let mut session = ssh2::Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;

        let mut agent = session.agent()?;
        agent.connect().unwrap();
        agent.list_identities().unwrap();

        session.userauth_agent("root")?;

        Ok(Connection::from(session))
    }

    pub fn exec(&self, command: &str) -> anyhow::Result<ExecResult> {
        let mut channel = self.session.channel_session()?;
        channel.exec(command)?;

        let mut stdout = String::new();
        channel.read_to_string(&mut stdout)?;

        let mut stderr = String::new();
        channel.stderr().read_to_string(&mut stderr)?;

        channel.wait_close()?;

        return Ok(ExecResult {
            exit_code: channel.exit_status()?,
            stdout,
            stderr,
        });
    }
}
