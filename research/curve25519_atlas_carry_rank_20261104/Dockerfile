# syntax=docker/dockerfile:1
FROM rust:1.99.1-bookworm@sha256:c9155318d07341301a78b45ecdd81af2e43b8b297d64e8fd4c5e80d7735607c3 AS build
ARG SOURCE_REVISION
RUN --mount=type=secret,id=proxy_ca,required=true \
    apt-get update && apt-get install -y --no-install-recommends libgmp-dev libssl-dev python3-venv ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && python3 -m venv /opt/z3 \
    && PIP_CERT=/run/secrets/proxy_ca /opt/z3/bin/pip install --no-cache-dir --only-binary=:all: z3-solver==5.3.3.0
WORKDIR /source
COPY Makefile study.cpp curve25519_fixture.hpp bv_lowering.hpp openssl_replay.cpp ./
COPY control/Cargo.toml control/Cargo.lock ./control/
COPY control/src ./control/src/
RUN test "${#SOURCE_REVISION}" = 40 && ! printf '%s' "$SOURCE_REVISION" | grep -q '[^0-9a-f]'
RUN make build OUT=/source/study SOURCE_REVISION="$SOURCE_REVISION" SOURCE_DIRTY=0
RUN --mount=type=secret,id=proxy_ca,required=true \
    CARGO_HTTP_CAINFO=/run/secrets/proxy_ca cargo build --locked --release --manifest-path control/Cargo.toml

FROM debian:bookworm-slim@sha256:99c0b25c46d0214d5a2091092e7908907dcaecd6a962cde0ac700acee222b6f3
ARG SOURCE_REVISION
LABEL org.opencontainers.image.source="https://github.com/isogeny-labs/crypto" \
      org.opencontainers.image.revision="$SOURCE_REVISION"
RUN --mount=type=secret,id=proxy_ca,required=true \
    apt-get update && apt-get install -y --no-install-recommends libgmp10 libgmpxx4ldbl libssl3 libstdc++6 ca-certificates \
    && rm -rf /var/lib/apt/lists/* && install -d -o 1000 -g 1000 /work
COPY --from=build /source/study /usr/local/bin/study
COPY --from=build /source/study-openssl /usr/local/bin/openssl-replay
COPY --from=build /source/control/target/release/atlas-control /usr/local/bin/atlas-control
COPY --from=build /opt/z3/bin/z3 /usr/local/bin/z3
RUN cd /usr/local/bin && sha256sum study openssl-replay atlas-control z3 > /usr/local/bin/ATLAS-BINARIES.sha256
WORKDIR /work
USER 1000:1000
ENTRYPOINT ["/usr/local/bin/atlas-control"]
