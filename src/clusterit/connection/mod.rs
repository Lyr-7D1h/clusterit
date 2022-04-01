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
use ssh2::{KeyboardInteractivePrompt, Session};

pub struct ConnectionPrompter;
impl KeyboardInteractivePrompt for ConnectionPrompter {
    fn prompt<'a>(
        &mut self,
        username: &str,
        instructions: &str,
        prompts: &[ssh2::Prompt<'a>],
    ) -> Vec<String> {
        println!("asdf");
        println!("{prompts:?}");

        vec!["test".to_string(), "asdf".to_string()]
    }
}

pub struct Connection {
    session: Session,
}

#[derive(Debug)]
pub struct ExecResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Returns most recent (publickey, privatekey)
fn get_most_recent_keys() -> Option<(PathBuf, PathBuf)> {
    let home_path = match env::var("HOME") {
        Ok(home_dir) => home_dir,
        Err(_) => return None,
    };

    let ssh_path = Path::new(&home_path).join(".ssh");

    let files = match fs::read_dir(&ssh_path) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let mut latest_key_set: Option<(PathBuf, PathBuf)> = None;
    let mut latest_modified = SystemTime::UNIX_EPOCH;

    for key in files {
        if let Ok(f) = key {
            if let Some(filename) = f.file_name().to_str() {
                let filename = filename.replace(".pub", "");
                if filename.starts_with("id_") {
                    if let Ok(m) = f.metadata() {
                        if let Ok(m) = m.modified() {
                            if m > latest_modified {
                                let public = ssh_path.clone().join(format!("{filename}.pub"));
                                let private = ssh_path.clone().join(filename);
                                if public.exists() && private.exists() {
                                    latest_modified = m;
                                    latest_key_set = Some((public, private))
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return latest_key_set;
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

        let mut session = Session::new()?;

        let mut parts = destination.split("@");

        let mut user = "root";

        if parts.clone().count() >= 2 {
            user = parts.next().unwrap();
            destination = parts.next().unwrap();
        }

        debug!("Creating TcpStream to: {destination}");
        let tcp = TcpStream::connect(destination).unwrap();

        session.set_tcp_stream(tcp);

        debug!("Performing handshake");
        session.handshake()?;

        if let Some(publickey) = public_key {
            if let Some(privatekey) = private_key {
                debug!("Public and private key given");
                match session.userauth_pubkey_memory(&user, Some(publickey), privatekey, None) {
                    Ok(_) => {
                        if session.authenticated() {
                            return Ok(Connection { session });
                        }
                        debug!("Userauth succeeded but still not authenticated")
                    }
                    Err(e) => debug!("Could not connect using most recent pub/private key: {e}"),
                }
            }
        }

        debug!("Authentication using ssh-agent");
        if let Err(e) = session.userauth_agent(&user) {
            debug!("Authentication using ssh-agent failed: {e}");

            debug!("{:?}", get_most_recent_keys());
            if let Some((publickey, privatekey)) = get_most_recent_keys() {
                debug!("Authentication using most recent public/private key");
                if let Err(e) = session.userauth_hostbased_file(
                    &user,
                    &publickey,
                    &privatekey,
                    None,
                    destination,
                    None,
                ) {
                    debug!("Could not connect using most recent pub/private key: {e}");
                };
            }

            session
                .userauth_keyboard_interactive(&user, &mut ConnectionPrompter {})
                .unwrap();
        }

        if !session.authenticated() {
            return Err(ConnectionError::Other("SSH Could not authenticate"));
        }

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
