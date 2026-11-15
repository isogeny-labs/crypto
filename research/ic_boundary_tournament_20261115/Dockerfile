FROM rust:1.99-bookworm@sha256:4715c3ad743951a3cea90e8b432f2a17e0ef5f3f3c3abf75108879e783b396f0
RUN apt-get update && apt-get install -y --no-install-recommends python3 awscli jq procps util-linux && rm -rf /var/lib/apt/lists/*
ARG ALGORITHM_REVISION=a10cb7b4f8e48f094efe70c31d9a24b6d72aee4a
RUN git init /opt/source \
 && cd /opt/source \
 && git remote add origin https://github.com/isogeny-labs/crypto.git \
 && git sparse-checkout init --cone \
 && git sparse-checkout set src examples crates isogeny_algos tools docs/ic docs/curves docs/ecbench research/pencil_large_prime_20261029 research/sat_factor_base_review_20260908/autolab research/ic_candidate_tournament_20260915 research/ic_single_target_20260930 experiments/koblitz-single-target-n61-20260925 experiments/koblitz-single-target-n71-20261002 experiments/koblitz-single-target-n73-20261003 experiments/koblitz-single-target-n83-20261006 \
 && git fetch --depth=1 --filter=blob:none origin "$ALGORITHM_REVISION" \
 && git checkout --detach FETCH_HEAD \
 && chmod -R a+rX /opt/source /usr/local/rustup \
 && chmod -R a+rwX /usr/local/cargo
COPY request.json run.sh /opt/tournament/
RUN chmod 755 /opt/tournament/run.sh
LABEL org.opencontainers.image.source="https://github.com/isogeny-labs/crypto" org.opencontainers.image.algorithm-revision="a10cb7b4f8e48f094efe70c31d9a24b6d72aee4a"
WORKDIR /work
USER 1000:1000
ENTRYPOINT []
CMD ["/opt/tournament/run.sh"]
