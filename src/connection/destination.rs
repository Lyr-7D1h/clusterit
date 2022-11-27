use std::{
    fmt::Display,
    net::{SocketAddr, ToSocketAddrs},
    str::FromStr,
    vec,
};

use serde::Deserialize;

use super::ConnectionError;

/// Code representation of OpenSSH definition of a destination (ssh://[user@]hostname[:port.])
#[derive(Debug, Deserialize)]
pub struct Destination {
    pub hostname: String,
    pub username: String,
    pub port: u16,
}

impl FromStr for Destination {
    type Err = ConnectionError;

    /// Parses connection strings like 'ssh://username@hostname:port'
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = match s.strip_prefix("ssh://") {
            Some(s) => s,
            None => s,
        };

        let mut username = "root";

        let mut parts = s.split("@");

        s = parts.next().unwrap();
        if let Some(hostname) = parts.next() {
            username = s;
            s = hostname;
        }

        let mut parts = s.split(":");
        let hostname = parts.next().unwrap();

        let mut port = 22;
        if let Some(p) = parts.next() {
            port = p.parse().expect("Invalid port");
        }

        Ok(Destination {
            hostname: hostname.to_string(),
            username: username.to_string(),
            port,
        })
    }
}

impl Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}:{}", self.username, self.hostname, self.port)
    }
}

impl ToSocketAddrs for Destination {
    type Iter = vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        format!("{}:{}", self.hostname, self.port).to_socket_addrs()
    }
}

#[test]
pub fn destination_from_str() {
    let d = Destination::from_str("ssh://test@ahostname:48").unwrap();
    assert_eq!(d.username, "test");
    assert_eq!(d.hostname, "ahostname");
    assert_eq!(d.port, 48);

    let d = Destination::from_str("test@ahostname:48").unwrap();
    assert_eq!(d.username, "test");
    assert_eq!(d.hostname, "ahostname");
    assert_eq!(d.port, 48);

    let d = Destination::from_str("192.168.1.1").unwrap();
    assert_eq!(d.username, "root");
    assert_eq!(d.hostname, "192.168.1.1");
    assert_eq!(d.port, 22);
}
