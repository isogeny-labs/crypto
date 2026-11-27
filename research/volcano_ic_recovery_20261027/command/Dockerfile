# Both caller-supplied base references must be immutable @sha256 digests.
# BUILD_IMAGE supplies g++, static libc/libstdc++ and coreutils.
# RUNTIME_IMAGE supplies bash and coreutils. No package installation or secrets.
ARG BUILD_IMAGE
ARG RUNTIME_IMAGE
FROM ${BUILD_IMAGE} AS build
COPY repo/ /opt/volcano/repo/
COPY SOURCE_REVISION /opt/volcano/SOURCE_REVISION
COPY launch.sh /opt/volcano/launch.sh
WORKDIR /opt/volcano/repo
RUN sha256sum -c research/volcano_ic_recovery_20261027/SOURCES.sha256 \
 && mkdir -p /opt/volcano/bin \
 && g++ -std=c++20 -O2 -static -Wall -Wextra -Wpedantic -Werror research/volcano_ic_recovery_20261027/runner.cpp -o /opt/volcano/bin/runner \
 && g++ -std=c++20 -O2 -static -Wall -Wextra -Wpedantic -Werror research/volcano_ic_recovery_20261027/verify.cpp -o /opt/volcano/bin/verify \
 && cd /opt/volcano \
 && sha256sum bin/runner bin/verify > binaries.sha256 \
 && chmod -R a+rX,go-w /opt/volcano \
 && chmod 555 /opt/volcano/launch.sh /opt/volcano/bin/runner /opt/volcano/bin/verify
FROM ${RUNTIME_IMAGE} AS runtime
COPY --from=build /opt/volcano /opt/volcano
RUN mkdir -p /work/compute && chown 1000:1000 /work/compute
USER 1000:1000
WORKDIR /work/compute
ENTRYPOINT ["/opt/volcano/launch.sh"]
CMD ["plan"]
