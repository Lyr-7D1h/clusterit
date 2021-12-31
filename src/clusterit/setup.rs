use std::{fs::read_to_string, io, net::IpAddr, str::FromStr};

use crate::clusterit::step_executer::StepExecuter;

use super::connection::Connection;
use anyhow::{anyhow, Context, Result};
use log::info;
use regex::Regex;

use super::clusterit;

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

impl clusterit {
    fn setup_connection(&self, destination: &str) -> anyhow::Result<()> {
        info!(
            "Attempting to connect to: {}, using known default credentials",
            destination
        );
        let connection = Connection::connect_using_default(destination)?;

        let mut executer = StepExecuter::new(connection);

        executer.add_step("sudo mkdir /root/.ssh");
        executer.add_step(&format!(
            "sudo sh -c 'echo -n \"{}\" > /root/.ssh/authorized_keys'",
            self.state.get_pub_ssh_key()
        ));

        info!("Setting up root ssh with ");
        executer.exec().or_else(|e| {
            Err(anyhow::Error::msg(format!(
                "Setup connection failed at step {}",
                e
            )))
        })?;

        Ok(())
    }

    fn setup_armbian(&self, _device: Device) -> anyhow::Result<()> {
        // apt_install().with_context(|| "Install failed")
        unimplemented!()
    }

    fn setup_raspian(&self, _device: Device) -> anyhow::Result<()> {
        // apt_install().with_context(|| "Install failed")
        unimplemented!()
    }

    fn setup_nfs(&self) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn validate(&self) -> io::Result<()> {
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

    fn is_storage_device(&self) -> anyhow::Result<bool> {
        unimplemented!()
    }

    /// Setup the standard connection using public key found

    pub fn setup(self, ip: IpAddr, step: Option<u8>) -> anyhow::Result<()> {
        info!("Attempting to connect to: {}", ip);
        let destination = format!("{}:22", ip.to_string());
        let connection = match Connection::connect(&destination) {
            Ok(c) => c,
            Err(_) => {
                info!("Standard connection failed");
                self.setup_connection(&destination)?;
                Connection::connect(&destination)?
            }
        };

        info!("Connected to: {}", ip);

        let device = self.get_device()?;

        let distro = self.get_distro()?;

        info!("Setting up {:?} for {:?}", distro, ip);

        self.validate()?;

        let step = match self.state.find_setup(&ip) {
            Some(exisiting_setup) => {
                info!(
                    "Found existing setup for {} continuing on step {}",
                    ip, exisiting_setup.step
                );
                exisiting_setup.step
            }
            None => step.unwrap_or(0),
        };

        // match distro {
        //     Distribution::Armbian | Distribution::Debian | Distribution::Ubuntu => {
        //         self.setup_armbian(device)
        //     }
        //     Distribution::RaspberryPiOS => self.setup_raspian(device),
        // }
        unimplemented!()
    }
}
