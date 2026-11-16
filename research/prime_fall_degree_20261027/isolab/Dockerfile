# Build context: the repository at the frozen commit. UID 1000 as isolab requires.
FROM rust:1.88-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release --example pkm_tower_pilot --example prime_fall_degree_tools

FROM public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39
COPY --from=build /src/target/release/examples/pkm_tower_pilot /usr/local/bin/pkm_tower_pilot
COPY --from=build /src/target/release/examples/prime_fall_degree_tools /usr/local/bin/prime_fall_degree_tools
COPY research/prime_fall_degree_20261027/run.sh /opt/study/run.sh
COPY research/prime_fall_degree_20261027/isolab/entry.sh /usr/local/bin/entry.sh
ENV HOME=/work
WORKDIR /work
USER 1000:1000
ENTRYPOINT []
