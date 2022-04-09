use home_path::get_home_path;
use prompter::input;
use std::{
    env, fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use log::{debug, error};
use ssh2::Session;

use super::ConnectionError;

/// Returns most recent (publickey, privatekey)
fn get_most_recent_keys() -> Option<(PathBuf, PathBuf)> {
    let home_path = get_home_path()?;

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

fn get_default_keys() -> Option<(PathBuf, PathBuf)> {
    let home_path = get_home_path()?;
    let ssh_path = Path::new(&home_path).join(".ssh");

    return Some((
        ssh_path.clone().join(format!("id_rsa.pub")),
        ssh_path.join("id_rsa"),
    ));
}

fn get_keys() -> Option<(PathBuf, PathBuf)> {
    let (mut publickey, mut privatekey) = match get_most_recent_keys() {
        Some(keys) => keys,
        None => get_default_keys()?,
    };

    loop {
        if let Ok(pk) = input(&format!("Private key ({})", privatekey.to_string_lossy())) {
            match pk {
                Some(pk) => {
                    let path = PathBuf::from(pk);
                    if path.exists() {
                        break;
                    }
                }
                None => break,
            }
        }
    }

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
