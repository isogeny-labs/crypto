ARG BASE_IMAGE=debian@sha256:a72b4433de1cf7d6bae0c4cee74d0547f58601a643b3375f070c51600d63b6b2
FROM ${BASE_IMAGE} AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    g++ libflint-dev=3.1.3-2 libpari-dev=2.17.2-1 \
    libmpfr-dev=4.2.2-1 libgmp-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY optimized.c complete_pipeline.cpp ./
RUN mkdir -p /opt/conductor \
    && cc -O3 -Wall -Wextra -std=c11 \
       -Dprime_log_norm_jet=optimized -Dprime_log_norm_free=optimized_free \
       -c optimized.c -o optimized.o \
    && c++ -O2 -Wall -Wextra -std=c++17 complete_pipeline.cpp optimized.o \
       -lpari -lflint -lgmpxx -lgmp -o /opt/conductor/complete_pipeline \
    && sha256sum optimized.c complete_pipeline.cpp > /opt/conductor/sources.sha256 \
    && sha256sum /opt/conductor/complete_pipeline > /opt/conductor/binary.sha256 \
    && dpkg-query -W g++ gcc libflint-dev libpari-dev libmpfr-dev libgmp-dev \
       > /opt/conductor/build-packages.tsv

FROM ${BASE_IMAGE}
ARG SOURCE_REVISION
RUN test "${#SOURCE_REVISION}" -eq 40 \
    && case "$SOURCE_REVISION" in *[!0-9a-f]*) exit 1;; esac \
    && apt-get update && apt-get install -y --no-install-recommends \
       libflint19=3.1.3-2 libpari-gmp-tls9=2.17.2-1 libgmpxx4ldbl=2:6.3.0+dfsg-3 \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p /work && chown 1000:1000 /work
COPY --from=builder /opt/conductor /opt/conductor
LABEL org.opencontainers.image.source="https://github.com/isogeny-labs/crypto" \
      org.opencontainers.image.revision="${SOURCE_REVISION}" \
      org.opencontainers.image.title="Complete native conductor-gap evaluator"
ENV HOME=/work
USER 1000:1000
WORKDIR /work
ENTRYPOINT ["/opt/conductor/complete_pipeline"]
