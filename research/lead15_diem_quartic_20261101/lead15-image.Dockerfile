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
    chmod 0555 /opt/lead15/lead15_diem_quartic /opt/lead15/lead15-worker; \
    for tool in aws bash jq sha256sum awk date mkdir cp sleep setsid timeout stat tail od tr find sort xargs uname getconf ldd rm rmdir; do \
      command -v "$tool" >/dev/null; \
    done; \
    aws --version >/dev/null 2>&1; \
    AWS_EC2_METADATA_DISABLED=true aws s3 cp /dev/null s3://invalid.example/lead15-probe \
      --checksum-algorithm SHA256 --dryrun --only-show-errors >/dev/null; \
    bash --version >/dev/null; \
    jq --version >/dev/null; \
    sha256sum /dev/null >/dev/null; \
    awk 'BEGIN { exit 0 }'; \
    date -u +%Y-%m-%dT%H:%M:%SZ >/dev/null; \
    mkdir -p /tmp/lead15-image-probe; \
    cp /dev/null /tmp/lead15-image-probe/copy; \
    sleep 0; \
    setsid --version >/dev/null; \
    timeout --version >/dev/null; \
    timeout --help | awk '/--verbose/{found=1} END{exit !found}'; \
    stat -c %s /tmp/lead15-image-probe/copy >/dev/null; \
    tail -c 0 /tmp/lead15-image-probe/copy >/dev/null; \
    od -An -tuC /tmp/lead15-image-probe/copy >/dev/null; \
    printf 'probe\n' | tr -d '\n' >/dev/null; \
    find /tmp/lead15-image-probe -type f -print0 | sort -z | xargs -0 sha256sum >/dev/null; \
    uname -srvm >/dev/null; \
    getconf _NPROCESSORS_ONLN >/dev/null; \
    ldd /opt/lead15/lead15_diem_quartic >/dev/null; \
    if /opt/lead15/lead15-worker >/dev/null 2>&1; then exit 1; else test "$?" -eq 2; fi; \
    rm -f /tmp/lead15-image-probe/copy; \
    rmdir /tmp/lead15-image-probe; \
    chmod 0444 /opt/lead15/build-info.json /opt/lead15/lead15-image.Dockerfile

USER 1000:1000
WORKDIR /work
