FROM rust@sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c

# This is the build host, not the K600 worker image. Its immutable image ID
# is pinned in the charged-build protocol before any measured child runs.
RUN apt-get update \
    && apt-get install --yes --no-install-recommends cmake docker.io libgmp-dev pkg-config zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*
