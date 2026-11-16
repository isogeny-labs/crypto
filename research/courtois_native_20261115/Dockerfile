# Development build recipe for the native correctness command. Build context:
# the parent directory containing the two pinned sibling repository checkouts.
# This is NOT an admitted benchmark image: freeze base image digests, source
# commits, built binary hash and final image digest before any remote execution.
ARG RUST_IMAGE=rust:1.100.0-bookworm
ARG RUNTIME_IMAGE=debian:bookworm-slim
FROM ${RUST_IMAGE} AS build
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY crypto/Cargo.toml crypto/Cargo.lock crypto/
COPY crypto/src/ crypto/src/
COPY crypto/crates/ crypto/crates/
COPY crypto/isogeny_algos/ crypto/isogeny_algos/
COPY crypto/tools/factor-base/ crypto/tools/factor-base/
COPY crypto/tools/volcano-cli/ crypto/tools/volcano-cli/
COPY crypto/research/pencil_large_prime_20261029/ crypto/research/pencil_large_prime_20261029/
COPY crypto/docs/ic/boundary_targets.json crypto/docs/ic/calibration.json crypto/docs/ic/
COPY crypto/docs/ecbench/schema.sql crypto/docs/ecbench/
COPY crypto/docs/curves/registry.json crypto/docs/curves/
COPY crypto/docs/curves/standards/parameters.json crypto/docs/curves/standards/
COPY crypto/research/courtois_native_20261115/ crypto/research/courtois_native_20261115/
COPY cryptanalysis/bindings/rust/hashed-ecdl-tensor/ cryptanalysis/bindings/rust/hashed-ecdl-tensor/
ENV CARGO_BUILD_JOBS=2
RUN cargo build --locked --release --manifest-path crypto/research/courtois_native_20261115/Cargo.toml \
    && cargo build --locked --release --manifest-path crypto/research/courtois_native_20261115/replay-cryptanalysis/Cargo.toml

FROM ${RUNTIME_IMAGE}
RUN apt-get update && apt-get install -y --no-install-recommends libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p /work && chown 1000:1000 /work
COPY --from=build /build/crypto/research/courtois_native_20261115/target/release/courtois-native-study /usr/local/bin/
COPY --from=build /build/crypto/research/courtois_native_20261115/replay-cryptanalysis/target/release/courtois-cryptanalysis-field-replay /usr/local/bin/
USER 1000:1000
WORKDIR /work
ENTRYPOINT ["/usr/local/bin/courtois-native-study"]
