# syntax=docker/dockerfile:1.7
FROM public.ecr.aws/docker/library/rust@sha256:ade39a710ef85f3506cffcfec3e8a24ba090badef1dff2fb798f71d878833fea

ARG SOURCE_REVISION
ARG JQ_VERSION=1.7.1
ARG JQ_SHA256=5942c9b0934e510ee61eb3e30273f1b3fe2590df93933a93d7c58b81d19c8ff5
ARG AWSCLI_VERSION=2.40.3
ARG AWSCLI_SHA256=4770c06c4ecfc8ba94b60d89e6c7018ab7ea2cf5341b1ef05f6f93c5d6e8b705

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

# The builder creates a controlled context containing only the clean sparse
# repository and two verified runtime clients. A read-only BuildKit bind keeps
# that context out of the image and creates one filesystem layer over Rust.
RUN --mount=type=bind,source=.,target=/tmp/ghs-build-context \
    printf '%s  %s\n' "$JQ_SHA256" /tmp/ghs-build-context/tooling/jq-linux-amd64 | sha256sum -c - \
 && install -m 0755 /tmp/ghs-build-context/tooling/jq-linux-amd64 /usr/local/bin/jq \
 && printf '%s  %s\n' "$AWSCLI_SHA256" /tmp/ghs-build-context/tooling/awscliv2.zip | sha256sum -c - \
 && unzip -q /tmp/ghs-build-context/tooling/awscliv2.zip -d /tmp/awscli-install \
 && /tmp/awscli-install/aws/install --bin-dir /usr/local/bin --install-dir /usr/local/aws-cli \
 && cp -a /tmp/ghs-build-context/repo /opt/crypto \
 && rm -rf /tmp/awscli-install /usr/local/aws-cli/v2/current/dist/awscli/examples \
 && [[ "$SOURCE_REVISION" =~ ^[a-f0-9]{40}$ ]] \
 && test "$(git -C /opt/crypto rev-parse HEAD)" = "$SOURCE_REVISION" \
 && test -z "$(git -C /opt/crypto status --porcelain)" \
 && printf '%s  %s\n' \
      b2244ffece12855622a88f9a2af4926b1fac6113b9519fdf11e2bc7449fa27c8 \
      /opt/crypto/research/quartic_ghs_strong_rho_20261114/main.rs \
      | sha256sum -c - \
 && printf '%s  %s\n' \
      73abd342613dee852add1e3459c94bc7f949801c94eb875a309cdfc4f6b0f5de \
      /opt/crypto/research/quartic_ghs_strong_rho_20261114/strong_rho.rs \
      | sha256sum -c - \
 && GHS_SOURCE_REVISION="$SOURCE_REVISION" rustc --edition 2021 -C opt-level=3 \
      /opt/crypto/research/quartic_ghs_strong_rho_20261114/main.rs \
      -o /tmp/ghs-strong-rho-selftest \
 && test "$(/tmp/ghs-strong-rho-selftest selftest)" = "SELFTEST OK" \
 && rm -f /tmp/ghs-strong-rho-selftest \
 && if ! getent group 1000 >/dev/null; then groupadd --gid 1000 isolab; fi \
 && if ! getent passwd 1000 >/dev/null; then \
      useradd --uid 1000 --gid 1000 --create-home --shell /usr/sbin/nologin isolab; \
    fi \
 && jq --version \
 && aws --version

ENV AWS_CA_BUNDLE=/etc/ssl/certs/ca-certificates.crt
USER 1000:1000

LABEL org.opencontainers.image.source="https://github.com/isogeny-labs/crypto" \
      org.opencontainers.image.revision="$SOURCE_REVISION" \
      org.opencontainers.image.title="GHS strong-rho round A"
