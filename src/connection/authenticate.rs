use path_resolver::resolve;
use prompter::ask_secret;
use std::fs;

use log::{debug, error};
use ssh2::Session;

use super::ConnectionError;

fn authenticate_using_keys(session: &Session, user: &str, hostname: &str) -> bool {
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
                            &user,
                            &publickey,
                            &privatekey,
                            None,
                            hostname,
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

///
pub fn authenticate(
    session: &Session,
    username: &str,
    hostname: &str,
    public_key: Option<&str>,
    private_key: Option<&str>,
) -> Result<(), ConnectionError> {
    debug!("Perfoming authentication");
    if let Some(publickey) = public_key {
        if let Some(privatekey) = private_key {
            debug!("Public and private key given");

            if let Err(e) =
                session.userauth_pubkey_memory(&username, Some(publickey), privatekey, None)
            {
                error!("Could not connect with given public and private key: {e}");
                return authenticate(session, username, hostname, None, None);
            }

            return Ok(());
        }
    }

    debug!("Authentication using ssh-agent");
    if let Err(e) = session.userauth_agent(&username) {
        debug!("Authentication using ssh-agent failed: {e}");
    } else {
        return Ok(());
    }

    debug!("Authentication using public and private keys");
    authenticate_using_keys(session, username, hostname);

    loop {
        if let Ok(password) = ask_secret(&format!("{username}@{hostname}'s password")) {
            if let Err(e) = session.userauth_password(username, &password) {
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
