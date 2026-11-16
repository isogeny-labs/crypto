# Identical official solver revision to the completed S4 external control.
FROM debian:trixie-slim@sha256:62bd423aeee912639ca8ee681c2b762cb3e25137e0a92b20be455898a9f034ad
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates git autoconf automake libtool make gcc g++ \
    libgmp-dev libmpfr-dev libflint-dev pkg-config && rm -rf /var/lib/apt/lists/*
RUN git clone --depth 1 --branch v0.11.0 https://github.com/algebraic-solving/msolve /opt/msolve && \
    test "$(git -C /opt/msolve rev-parse HEAD)" = cc044551f8e202d00691990fd3fd0b1226a88ea2
WORKDIR /opt/msolve
RUN git rev-parse HEAD > /opt/msolve-source-commit.txt && \
    ./autogen.sh && ./configure && make -j1 && make check && make install && ldconfig
WORKDIR /work
