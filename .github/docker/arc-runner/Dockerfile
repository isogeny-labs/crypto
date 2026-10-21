FROM ghcr.io/actions/actions-runner:2.338.0@sha256:4ffadc0002b2581327e06101fc8c06cd189232baf79fe561fac9caeb76f5e807

USER root

# The stock ARC image contains the runner but few tools that GitHub-hosted
# Ubuntu jobs assume. Keep this a general Linux build image; specialized Sage,
# GPU, perf and virtualization workflows still need separate pools.
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      autoconf automake bison build-essential clang cmake flex gfortran \
      libffi-dev libgmp-dev libmpc-dev libmpfr-dev libsqlite3-dev libssl-dev \
      libtool libzstd-dev musl-tools ninja-build pkg-config python-is-python3 \
      python3-pip python3-venv \
    && rm -rf /var/lib/apt/lists/* \
    && for tool in node npm npx corepack; do \
         ln -s "/home/runner/externals/node24/bin/$tool" "/usr/local/bin/$tool"; \
       done

USER runner
ENV RUSTUP_HOME=/home/runner/.rustup \
    CARGO_HOME=/home/runner/.cargo \
    PATH=/home/runner/.cargo/bin:/home/runner/externals/node24/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --profile minimal --default-toolchain 1.98.0 \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo --version && node --version && gcc --version
