FROM ubuntu

SHELL ["/bin/bash", "-c"] 

# Install needed packages
RUN apt-get update && apt-get -y upgrade && apt-get install -y build-essential libc6 libc-bin curl

# Install rustup
WORKDIR /tmp
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup
RUN sh rustup -y

WORKDIR /usr/src/k3main

# Cache rust dependencies
RUN echo "fn main() {}" > dummy.rs
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN source $HOME/.cargo/env && cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

# Build app
COPY . /usr/src/k3main
RUN source $HOME/.cargo/env && cargo build --release  
RUN cp ./target/release/k3main /usr/bin

# ENTRYPOINT [ "k3main" ]
CMD k3main setup