FROM 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:194ec662a87c1aca1be0a0420b230a45766da3b5217c79e8d05fdd03086d7de3
USER 0
COPY fall /opt/fall
COPY run.sh /opt/fall-run.sh
COPY protocol.json /opt/fall-protocol.json
COPY source-revision.txt /opt/fall-source-revision.txt
COPY fall.rs /opt/fall.rs
RUN chmod 755 /opt/fall /opt/fall-run.sh && chmod 644 /opt/fall-protocol.json /opt/fall-source-revision.txt /opt/fall.rs
USER 1000
ENTRYPOINT ["/bin/bash", "/opt/fall-run.sh"]
