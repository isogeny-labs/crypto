# syntax=docker/dockerfile:1
# The caller must provide a digest-pinned x86_64 base that already contains
# bash, jq, aws, GNU timeout/setsid, coreutils and the required runtime libc.
ARG BASE_IMAGE
FROM ${BASE_IMAGE}
ARG BASE_IMAGE

USER 0:0
COPY --chown=1000:1000 lead15_diem_quartic /opt/lead15/lead15_diem_quartic
COPY --chown=1000:1000 lead15-worker /opt/lead15/lead15-worker
COPY --chown=1000:1000 build-info.json /opt/lead15/build-info.json
COPY --chown=1000:1000 lead15-image.Dockerfile /opt/lead15/lead15-image.Dockerfile

RUN set -eu; \
    test "$(jq -r .schema /opt/lead15/build-info.json)" = lead15.build-info/v1; \
    test "$(jq -r .base_image_uri /opt/lead15/build-info.json)" = "$BASE_IMAGE"; \
    test "$(sha256sum /opt/lead15/lead15_diem_quartic | awk '{print $1}')" = \
      "$(jq -r .lead15_binary_sha256 /opt/lead15/build-info.json)"; \
    test "$(sha256sum /opt/lead15/lead15-worker | awk '{print $1}')" = \
      "$(jq -r .lead15_worker_sha256 /opt/lead15/build-info.json)"; \
    test "$(sha256sum /opt/lead15/lead15-image.Dockerfile | awk '{print $1}')" = \
      "$(jq -r .image_recipe_sha256 /opt/lead15/build-info.json)"; \
    command -v aws >/dev/null; \
    command -v bash >/dev/null; \
    command -v jq >/dev/null; \
    command -v setsid >/dev/null; \
    command -v timeout >/dev/null; \
    chmod 0555 /opt/lead15/lead15_diem_quartic /opt/lead15/lead15-worker; \
    chmod 0444 /opt/lead15/build-info.json /opt/lead15/lead15-image.Dockerfile

USER 1000:1000
WORKDIR /work
