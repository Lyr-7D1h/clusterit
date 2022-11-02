use path_resolver::resolve;
use prompter::ask_secret;
use std::fs;

use log::{debug, error};
use ssh2::Session;

use super::{ConnectionError, Destination};

fn authenticate_using_keys(session: &Session, destination: &Destination) -> bool {
    let ssh_path = resolve("~/.ssh");

    let files = match fs::read_dir(&ssh_path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    for key in files {
        if let Ok(f) = key {
            if let Some(publickey) = f.file_name().to_str() {
                if publickey.starts_with("id_") && publickey.ends_with(".pub") {
                    let privatekey = resolve(format!("~/.ssh/{}", publickey.replace(".pub", "")));
                    let publickey = resolve(format!("~/.ssh/{publickey}"));

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
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }

    return false;
}

pub fn authenticate(
    session: &Session,
    destination: &Destination,
    public_key: &str,
    private_key: &str,
) -> Result<(), ConnectionError> {
    debug!("Perfoming authentication");

    if let Err(e) =
        session.userauth_pubkey_memory(&destination.username, Some(public_key), private_key, None)
    {
        error!("Could not connect with given public and private key: {e}");
        return Err(ConnectionError::Ssh(e));
    }

    return Ok(());
}

pub fn authenticate_interactive(
    session: &Session,
    destination: &Destination,
) -> Result<(), ConnectionError> {
    debug!("Perfoming authentication");

    debug!("Authentication using ssh-agent");
    if let Err(e) = session.userauth_agent(&destination.username) {
        debug!("Authentication using ssh-agent failed: {e}");
    } else {
        return Ok(());
    }

    debug!("Authentication using public and private keys");
    authenticate_using_keys(session, destination);

    loop {
        if let Ok(password) = ask_secret(&format!(
            "{}@{}'s password",
            destination.username, destination.hostname
        )) {
            if let Err(e) = session.userauth_password(&destination.username, &password) {
                error!("{e}");
            } else {
                // TODO set max of 2 mistakes due to ssh timeout
                break;
            }
        }
    }

    if !session.authenticated() {
        return Err(ConnectionError::Other("SSH Could not authenticate"));
    }

    return Ok(());
}
