FROM rust@sha256:38618e9e8971729f1b9f25bf93aaff2dc7b7ff22a6cde1930d96fb8226756029

RUN apt-get update \
    && apt-get install --yes --no-install-recommends cmake libgmp-dev pkg-config zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*
