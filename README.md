# Cluster IT

**Under Development**

A docker like solution for setting up, maintaining and configuring servers over ssh.

# Usage

```bash
clusterit
```

# Running

Requirements:

- libssh
- rustup
- cc

```bash
rustup install `cat rust-toolchain`
cargo run
```

## Tests

Requirements:

- docker

```bash
cargo test
```

# Developing

# Steps

- Setup public/private auth for ssh root login
- If storage device is sd disable swap, otherwise calculate the right amount of swap.
- Set users timezone
- Set static ip
- Remove unused packages (bluetooth, nano, wireless, graphical, sudo, python)
- Unattended upgrades
- Closest source for package manager
- If grub is installed set timeout to 0 for immediate boot

# TODO

- Manage temperature
- Manage updates
- Auto update local kube config
- Local tool that communicates with remote
- Installer
