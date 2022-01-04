use log::debug;

use anyhow::Result;
use ssh::Session;

pub struct Connection {
    host: String,
    port: usize,
    session: Session,
}

#[derive(Debug)]
pub struct ExecResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl Connection {
    pub fn new(host: String, port: usize) -> Result<Connection> {
        let mut session = Session::new().unwrap();
        session.set_host(&host)?;
        session.parse_config(None)?;

        session.set_port(port)?;

        Ok(Connection {
            host,
            port,
            session,
        })
    }

    pub fn connect(mut self, user: Option<&str>, password: Option<&str>) -> Result<()> {
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
