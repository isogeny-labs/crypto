FROM 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:25163e42d64075e44ddfa2e1a2d55cdc53d25357ff036c152a6ebff1d417e3e9
COPY radix_neural_probe /usr/local/bin/radix_neural_probe
COPY manifest.json /opt/n83-radix-neural/manifest.json
COPY cases/ /opt/n83-radix-neural/cases/
COPY run-radix-neural.sh /usr/local/bin/run-radix-neural.sh
USER 1000:1000
WORKDIR /work
ENTRYPOINT ["/usr/local/bin/run-radix-neural.sh"]
