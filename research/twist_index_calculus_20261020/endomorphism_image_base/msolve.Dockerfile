# Official msolve v0.11.0 source, released 2026-10-09. The build receipt pins
# the resolved Debian image digest and source commit before measurements.
FROM debian:trixie-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl git autoconf automake libtool make gcc g++ \
    libgmp-dev libmpfr-dev libflint-dev pkg-config && rm -rf /var/lib/apt/lists/*
RUN git clone --depth 1 --branch v0.11.0 https://github.com/algebraic-solving/msolve /opt/msolve
WORKDIR /opt/msolve
RUN git rev-parse HEAD > /opt/msolve-source-commit.txt && \
    ./autogen.sh && ./configure && make -j4 && make check && make install && ldconfig
WORKDIR /work
