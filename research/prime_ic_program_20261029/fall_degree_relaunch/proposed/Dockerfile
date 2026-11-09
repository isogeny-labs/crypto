# PROPOSED replacement for research/prime_fall_degree_20261027/isolab/Dockerfile (plan, untested:
# no Docker daemon was available where this was written). Differences from the original:
# - Rust 1.99.1 and a static x86-64 musl build, matching taskq/deploy/eks-karpenter/build-bundle,
#   because the runtime base (the aws-cli image, same digest as Dockerfile.ecbench) is paired with
#   a static binary in the lab ("so it runs in the existing runtime image independently of ...
#   glibc", HARNESS.md). The original copied glibc binaries built on rust:1.88-bookworm.
# - Builds hky_fp_tower_check too, so gate G1 runs from the measured image.
# - Pins RAYON_NUM_THREADS (Amendment 2 fixed thread counts; nproc in a pod may report node CPUs).
# Build context: the repository at the frozen commit. Pin the builder by digest at freeze time.
FROM rust:1.99.1-bookworm AS build
RUN apt-get update -qq && apt-get install -y --no-install-recommends musl-tools \
 && rustup target add x86_64-unknown-linux-musl
WORKDIR /src
COPY . .
ENV RUSTFLAGS="-C target-cpu=x86-64"
RUN cargo build --locked --release --no-default-features --target x86_64-unknown-linux-musl \
      --example pkm_tower_pilot --example prime_fall_degree_tools --example hky_fp_tower_check \
 && cd target/x86_64-unknown-linux-musl/release/examples \
 && sha256sum pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check > /src/binaries.sha256

FROM public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39
ARG SOURCE_REVISION
LABEL org.opencontainers.image.revision="${SOURCE_REVISION}"
COPY --from=build /src/target/x86_64-unknown-linux-musl/release/examples/pkm_tower_pilot /usr/local/bin/pkm_tower_pilot
COPY --from=build /src/target/x86_64-unknown-linux-musl/release/examples/prime_fall_degree_tools /usr/local/bin/prime_fall_degree_tools
COPY --from=build /src/target/x86_64-unknown-linux-musl/release/examples/hky_fp_tower_check /usr/local/bin/hky_fp_tower_check
COPY --from=build /src/binaries.sha256 /opt/study/binaries.sha256
COPY research/prime_fall_degree_20261027/run.sh /opt/study/run.sh
COPY research/prime_ic_program_20261029/fall_degree_relaunch/proposed/entry.sh /usr/local/bin/entry.sh
ENV HOME=/work RAYON_NUM_THREADS=14
WORKDIR /work
USER 1000:1000
ENTRYPOINT []
