FROM debian

RUN apt-get update && apt-get -y upgrade && apt-get install -y openssh-server sudo

RUN useradd -rm -d /home/test -s /bin/bash -g root -G sudo -u 1000 test

RUN echo "test:test" | chpasswd

RUN service ssh start

EXPOSE 22

CMD ["/usr/sbin/sshd", "-D"]
