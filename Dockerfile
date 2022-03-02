FROM ubuntu:21.10

WORKDIR /home

RUN apt-get update

# install git
RUN apt-get -y install git

RUN DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt-get -y install tzdata
RUN apt-get install -y curl gcc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://getmic.ro | sh -s -- -y && mv micro /usr/bin/

# copy File-Client repo
RUN git clone https://github.com/Shubham-Shingate/file-client.git

COPY src ./src
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build

CMD ["/bin/bash"]