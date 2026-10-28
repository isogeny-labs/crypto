FROM 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:0f591bb57d9c2c0a694d0fec1b83ef822453df0a10632f2ad4ffff731f51b2ab
USER 0
COPY n83-factor-base /opt/n83-factor-base
COPY run.sh /opt/fb-run.sh
COPY protocol.json /opt/fb-protocol.json
COPY source-revision.txt /opt/fb-source-revision.txt
RUN chmod 755 /opt/n83-factor-base /opt/fb-run.sh && chmod 644 /opt/fb-protocol.json /opt/fb-source-revision.txt
USER 1000
ENTRYPOINT ["/bin/bash", "/opt/fb-run.sh"]
