use path_resolver::resolve_path;
use prompter::ask_secret;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, read_to_string},
    io,
};

use log::{debug, error, warn};
use ssh2::Session;

use super::{ConnectionError, Destination};

// TODO check for encrypted keys too
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Authentication {
    Keys {
        public_key: String,
        private_key: String,
    },
    Password(String),
}

// TODO find way to save ssh_agent credentials
// pub fn authenticate_using_ssh_agent(
//     session: &Session,
//     destination: &Destination,
// ) -> Result<Authentication, ConnectionError> {
//     let mut agent = session.agent()?;
//     agent.connect()?;
//     agent.list_identities()?;
//     let identities = agent.identities()?;
//     let identity = match identities.get(0) {
//         Some(identity) => identity,
//         None => {
//             return Err(ConnectionError::Ssh(Error::new(
//                 ErrorCode::Session(-34), // LIBSSH2_ERROR_INVAL
//                 "no identities found in the ssh agent",
//             )))
//         }
//     };
//     agent.userauth(username, &identity)
//
//     return Ok()
// }

pub fn authenticate(
    session: &Session,
    destination: &Destination,
    authentication: &Authentication,
) -> Result<(), ConnectionError> {
    debug!("Perfoming key authentication");

    match authentication {
        Authentication::Keys {
            public_key,
            private_key,
        } => {
            if let Err(e) = session.userauth_pubkey_memory(
                &destination.username,
                Some(&public_key),
                &private_key,
                None,
            ) {
                error!("Could not connect with given public and private key: {e}");
                return Err(ConnectionError::Ssh(e));
            }
        }
        Authentication::Password(password) => {
            if let Err(e) = session.userauth_password(&destination.username, password) {
                error!("Could not connect with given public and private key: {e}");
                return Err(ConnectionError::Ssh(e));
            }
        }
    }

    return Ok(());
}

/// Authenticate for every public and private key pair found in ~/.ssh
fn authenticate_using_stored_keys(
    session: &Session,
    destination: &Destination,
) -> Result<Option<Authentication>, io::Error> {
    let ssh_path = resolve_path("~/.ssh");

    let files = match fs::read_dir(&ssh_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Could not read ~/.ssh");
            return Err(e);
        }
    };

    for key in files {
        if let Ok(f) = key {
            if let Some(publickey) = f.file_name().to_str() {
                if publickey.starts_with("id_") && publickey.ends_with(".pub") {
                    let privatekey = resolve_path(format!("~/.ssh/{}", publickey.replace(".pub", "")));
                    let publickey = resolve_path(format!("~/.ssh/{publickey}"));

                    if privatekey.exists() && publickey.exists() {
                        debug!("Authenticating using {privatekey:?}");
                        if let Ok(()) = session.userauth_hostbased_file(
                            &destination.username,
                            &publickey,
                            &privatekey,
                            None,
                            &destination.hostname,
                            None,
                        ) {
                            if session.authenticated() {
                                return Ok(Some(Authentication::Keys {
                                    public_key: read_to_string(publickey)?,
                                    private_key: read_to_string(privatekey)?,
                                }));
                            }
                        }
                    }
                }
            }
        }
    }

    return Ok(None);
}

pub fn authenticate_interactive(
    session: &Session,
    destination: &Destination,
) -> Result<Authentication, ConnectionError> {
    debug!("Perfoming authentication");

    // TODO find way to save user agent credentials
    // debug!("Authentication using ssh-agent");
    // if let Err(e) = session.userauth_agent(&destination.username) {
    //     debug!("Authentication using ssh-agent failed: {e}");
    // } else {
    //     return Ok(());
    // }

    debug!("Authentication using public and private keys");
    match authenticate_using_stored_keys(session, destination) {
        Ok(authentication) => match authentication {
            Some(authentication) => return Ok(authentication),
            None => {
                debug!("No working public or privatekey found")
            }
        },
        Err(e) => {
            warn!("Could not read keys in: {e}")
        }
    }

    loop {
        if let Ok(password) = ask_secret(&format!(
            "{}@{}'s password",
            destination.username, destination.hostname
        )) {
            if let Err(e) = session.userauth_password(&destination.username, &password) {
                error!("{e}");
            } else {
                if session.authenticated() {
                    return Ok(Authentication::Password(password));
                }
                break;
            }
        }
    }

    return Err(ConnectionError::Other("SSH Could not authenticate"));
}
