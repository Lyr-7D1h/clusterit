use path::Path;
use prompter::input;
use std::{default, fs, path::PathBuf, time::SystemTime};

use log::{debug, error};
use ssh2::Session;

use super::ConnectionError;

fn get_most_recent_private_key() -> Option<Path> {
    let ssh_path = match Path::resolve("~/.ssh") {
        Ok(p) => p,
        Err(_) => return None,
    };

    let files = match fs::read_dir(&ssh_path) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let mut latest_modified = SystemTime::UNIX_EPOCH;
    let mut latest_private = None;

    for key in files {
        if let Ok(f) = key {
            if let Some(filename) = f.file_name().to_str() {
                let filename = filename.replace(".pub", "");
                if filename.starts_with("id_") {
                    if let Ok(m) = f.metadata() {
                        if let Ok(m) = m.modified() {
                            if m > latest_modified {
                                latest_private = Some(ssh_path.clone().join(filename));
                                latest_modified = m
                            }
                        }
                    }
                }
            }
        }
    }

    return latest_private;
}

fn ask_key(message: &str, key: Option<&str>) -> Path {
    if let Ok(Some(pk)) = input!(message, key) {
        match Path::resolve(&pk) {
            Ok(p) => return p,
            Err(e) => error!("Invalid key: {e}"),
        }
    }

    return ask_key(message, key);
}

fn get_keys() -> Option<(Path, Path)> {
    let default_privatekey = get_most_recent_private_key().map(|p| p.to_string());

    let privatekey = ask_key("Private key", default_privatekey.as_deref());

    let default_publickey = privatekey
        .clone()
        .join(format!("../{}.pub", privatekey.filename()))
        .to_string();

    let publickey = ask_key("Public key", Some(&default_publickey));

    todo!()
}

pub fn authenticate(
    session: &Session,
    user: &str,
    hostname: &str,
    public_key: Option<&str>,
    private_key: Option<&str>,
) -> Result<(), ConnectionError> {
    debug!("Perfoming authentication");
    if let Some(publickey) = public_key {
        if let Some(privatekey) = private_key {
            debug!("Public and private key given");

            if let Err(e) = session.userauth_pubkey_memory(&user, Some(publickey), privatekey, None)
            {
                error!("Could not connect with given public and private key: {e}");
                return authenticate(session, user, hostname, None, None);
            }

            return Ok(());
        }
    }

    debug!("Authentication using ssh-agent");
    if let Err(e) = session.userauth_agent(&user) {
        debug!("Authentication using ssh-agent failed: {e}");
    } else {
        return Ok(());
    }

    if let Some((publickey, privatekey)) = get_keys() {
        debug!("Authentication using provided public/private key");
        if let Err(e) =
            session.userauth_hostbased_file(&user, &publickey, &privatekey, None, hostname, None)
        {
            debug!("Could not connect using most recent pub/private key: {e}");
        };
    }

    if !session.authenticated() {
        return Err(ConnectionError::Other("SSH Could not authenticate"));
    }

    return Ok(());
}
