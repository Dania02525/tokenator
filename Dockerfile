FROM debian:jessie AS builder

RUN apt-get update && apt-get install -y curl libmysqlclient-dev build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly-2019-04-23

ENV DATABASE_URL=mysql://root@127.0.0.1/platform_merchant_identity

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo build --release

FROM debian:jessie

RUN apt-get update && apt-get install -y libmysqlclient-dev

COPY --from=builder \
  /target/release/tokenator \
  /usr/local/bin/

WORKDIR /root
CMD ROCKET_PORT=$PORT /usr/local/bin/tokenator
