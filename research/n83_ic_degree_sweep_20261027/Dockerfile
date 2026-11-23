FROM debian:bookworm-slim@sha256:7ff6ba31527b9e336162fa061746c366c4b2032f4d2cff2078bea43f78efe3f8
RUN apt-get update && apt-get install -y --no-install-recommends singular ca-certificates awscli && rm -rf /var/lib/apt/lists/*
RUN useradd --uid 1000 --create-home worker
COPY sweep /opt/sweep
COPY run.sh /opt/run.sh
COPY protocol.json /opt/protocol.json
COPY source-revision.txt /opt/source-revision.txt
COPY src /opt/src
RUN chmod 755 /opt/sweep /opt/run.sh && chmod 644 /opt/protocol.json /opt/source-revision.txt && chmod -R a+rX /opt/src
USER 1000
WORKDIR /work
ENTRYPOINT ["/bin/bash", "/opt/run.sh"]
