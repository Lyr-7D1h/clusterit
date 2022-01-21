use std::io::Read;

use log::debug;

use anyhow::Result;
use ssh::{Channel, Session};

pub struct Connection<'a> {
    host: String,
    port: usize,
    session: Session,

    exec_channel: Option<Channel<'a>>,
}

#[derive(Debug)]
pub struct ExecResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl<'a> Connection<'a> {
    pub fn new(host: String, port: usize) -> Result<Connection<'a>> {
        let mut session = Session::new().unwrap();
        session.set_host(&host)?;
        session.parse_config(None)?;

        session.set_port(port)?;

        Ok(Connection {
            host,
            port,
            exec_channel: None,
            session,
        })
    }

    pub fn connect(&mut self, user: Option<&str>, password: Option<&str>) -> Result<()> {
        debug!(
            "Connecting to ssh://{}@{}:{}",
            user.unwrap_or_else(|| &""),
            self.host,
            self.port
        );

        if let Some(user) = user {
            self.session.set_username(&user)?;
        }

        self.session.connect()?;
        if let Some(password) = password {
            self.session.userauth_password(&password)?;
        } else {
            self.session.userauth_publickey_auto(None)?;
        }

        Ok(())
    }

    pub fn exec(mut self, command: &str) -> Result<String> {
        let mut s = match self.exec_channel {
            Some(s) => s,
            None => {
                let mut s = self.session.channel_new()?;
                s.open_session()?;
                s
            }
        };

        s.request_exec(command.as_bytes()).unwrap();
        s.send_eof().unwrap();

        let mut buf = Vec::new();
        s.stdout().read_to_end(&mut buf).unwrap();
        return Ok(String::from_utf8_lossy(&buf).into_owned());
    }

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
