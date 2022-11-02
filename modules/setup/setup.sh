#!/usr/bin/env bash

set -eo pipefail

# read keys
public_key=$1

# Run as root
if [[ $(id -u) != 0 ]]; then
	if ! command -v sudo &> /dev/null
	then
		echo "sudo couldn't be found"
		exit 1
	fi

	echo "Running as sudo"
	sudo $0 $@
	exit
fi

mkdir -p /root/.ssh 

echo "$public_key" > /root/.ssh/authorized_keys

# Enable Root Login 
sed -i "s/^#*PermitRootLogin.*/PermitRootLogin yes/g" /etc/ssh/sshd_config

# Disable password authentication
sed -i "s/^#*PasswordAuthentication.*/PasswordAuthentication no/g" /etc/ssh/sshd_config

# Disable root password
passwd -l root

systemctl restart sshd.service
