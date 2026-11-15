# Build from the repository root so the importer can read docs/curves:
#   docker build -f ecdb/Dockerfile -t ecdb .
FROM rust:1.88-bookworm AS build
WORKDIR /src
COPY ecdb/Cargo.toml ecdb/Cargo.lock ecdb/openapi.yaml ecdb/
COPY ecdb/src ecdb/src
COPY ecdb/migrations ecdb/migrations
COPY ecdb/tests ecdb/tests
RUN cd ecdb && cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates git \
    && rm -rf /var/lib/apt/lists/* && useradd --system --uid 10001 ecdb
COPY --from=build /src/ecdb/target/release/ecdb /usr/local/bin/ecdb
# The catalogues the importer reads; `ecdb import --repo-root /srv/crypto all`.
COPY docs/curves /srv/crypto/docs/curves
COPY docs/ic/fixtures/current_head_prime_route_20261024 /srv/crypto/docs/ic/fixtures/current_head_prime_route_20261024
COPY research/isogeny_walk_20261004/runs /srv/crypto/research/isogeny_walk_20261004/runs
COPY research/isogeny_walk_p192_20261004/runs /srv/crypto/research/isogeny_walk_p192_20261004/runs
COPY research/prime61_isogeny_20261016 /srv/crypto/research/prime61_isogeny_20261016
COPY src/cryptanalysis/curve_traits/modular_polynomials.txt /srv/crypto/src/cryptanalysis/curve_traits/modular_polynomials.txt
USER ecdb
EXPOSE 8080
ENTRYPOINT ["ecdb"]
CMD ["serve"]
