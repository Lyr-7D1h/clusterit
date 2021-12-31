FROM debian

RUN apt-get update && apt-get -y upgrade && apt-get install -y openssh-server

RUN systemctl enable --now sshd

RUN echo "asdf" | passwd

