# Cluster IT

Clearly define your servers and let clusterit do its magic.
It will setup the server to be as close to configuration as possible with minimal overhead and sane defaults.
This does not require any prerequisites on the server.

**Unusable atm**

A zero-config solution for setting up one big cluster for all your devices.

**Known working Devices**

- Odroid HC4
- Odroid MC1
- Raspberry Pi B+

# Running

Requirements:

- libssh
- rustup
- cc

```bash
rustup install `cat rust-toolchain`
cargo run
```

## Developing

```bash
cargo install cargo-watch
cargo watch -x run
```

## Tests

Additional Requirements:

- docker

```bash
./script/test
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
