use std::{
    fs::read_to_string,
    io,
    net::IpAddr,
    process::{Command, Output},
    str::FromStr,
};

use anyhow::{anyhow, Context, Result};
use log::{debug, error, info};
use regex::Regex;

use super::K3main;

#[derive(Debug)]
enum Distribution {
    Armbian,
    RaspberryPiOS,
    Debian,
    Ubuntu,
}

#[derive(Debug)]
pub enum Device {
    OdroidHC4,
    OdroidMC1,
    RaspberryPi,
}

impl FromStr for Device {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let device = match s.trim() {
            "odroidhc4" => Device::OdroidHC4,
            "odroidmc1" => Device::OdroidMC1,
            "raspberrypi" => Device::RaspberryPi,
            _ => return Err(anyhow!("Could not find divice: {}", s)),
        };

        Ok(device)
    }
}

fn command(command: &str, args: &[&str]) -> io::Result<Output> {
    debug!("Executing `{} {}`", command, args.join(" "));
    let output = Command::new(command).args(args).output();

    if output.is_ok() {
        output
    } else {
        error!("{:?}", output);
        output
    }
}

fn apt_install() -> anyhow::Result<()> {
    info!("Installing needed packages");
    command("apt-get", &["update"])?;

    Ok(())
}

impl K3main {
    fn setup_armbian(&self, _device: Device) -> anyhow::Result<()> {
        apt_install().with_context(|| "Install failed")
    }

    fn setup_raspian(&self, _device: Device) -> anyhow::Result<()> {
        apt_install().with_context(|| "Install failed")
    }

    fn validate(&self) -> io::Result<()> {
        let ouput = command("zcat", &["/proc/config.gz"])?;
        println!("{:?}", ouput.stdout);

        unimplemented!()
    }

    fn get_distro(&self) -> anyhow::Result<Distribution> {
        let release_os =
            read_to_string("/etc/os-release").with_context(|| "Could not read /etc/os-release")?;

        if Regex::new(r#"(?m)^PRETTY_NAME="Armbian.*"#)
            .unwrap()
            .is_match(&release_os)
        {
            return Ok(Distribution::Armbian);
        }

        if Regex::new(r#"(?m)^ID="raspbian"$"#)
            .unwrap()
            .is_match(&release_os)
        {
            return Ok(Distribution::RaspberryPiOS);
        }

        if Regex::new(r#"(?m)^ID="ubuntu"$"#)
            .unwrap()
            .is_match(&release_os)
        {
            return Ok(Distribution::Ubuntu);
        }

        if Regex::new(r#"(?m)^ID="debian"$"#)
            .unwrap()
            .is_match(&release_os)
        {
            return Ok(Distribution::Debian);
        }

        return Err(anyhow!("Could not find supported distribution"));
    }

    fn get_device(&self) -> anyhow::Result<Device> {
        let hostname =
            read_to_string("/etc/hostname").with_context(|| "Could not read hostname")?;

        Device::from_str(&hostname).with_context(|| format!("Could not find device: {}", hostname))
    }

    pub fn setup(&self, ip: IpAddr, device: Option<Device>) -> anyhow::Result<()> {
        let device = match device {
            Some(d) => d,
            None => self.get_device()?,
        };

        let distro = self.get_distro()?;

        info!("Setting up {:?} for {:?}", distro, ip);

        self.validate()?;

        match distro {
            Distribution::Armbian | Distribution::Debian | Distribution::Ubuntu => {
                self.setup_armbian(device)
            }
            Distribution::RaspberryPiOS => self.setup_raspian(device),
        }
    }
}
