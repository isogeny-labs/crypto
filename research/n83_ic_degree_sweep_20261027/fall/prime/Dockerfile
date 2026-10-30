FROM 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:2fc0eeffbefef8dbb507b4704d48156b2859c195e8e2217a625c06d1e9df3251
USER 0
COPY prime_fall /opt/prime_fall
COPY run.sh /opt/prime-run.sh
COPY protocol.json /opt/prime-protocol.json
COPY source-revision.txt /opt/prime-source-revision.txt
COPY r3-results.json /opt/r3-results.json
COPY prime_fall.rs /opt/prime_fall.rs
RUN chmod 755 /opt/prime_fall /opt/prime-run.sh && chmod 644 /opt/prime-protocol.json /opt/prime-source-revision.txt /opt/r3-results.json /opt/prime_fall.rs
USER 1000
ENTRYPOINT ["/bin/bash", "/opt/prime-run.sh"]
