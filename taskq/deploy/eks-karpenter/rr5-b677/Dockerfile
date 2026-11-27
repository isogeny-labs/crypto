FROM public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39
ARG SOURCE_REVISION
LABEL org.opencontainers.image.source="https://github.com/isogeny-labs/crypto" \
      org.opencontainers.image.revision="${SOURCE_REVISION}"
# Explicit runtime tools; the final OCI digest binds the installed packages.
RUN yum install -y jq coreutils findutils util-linux && yum clean all \
    && for tool in bash jq sha256sum timeout setsid find sort xargs realpath aws; do command -v "$tool" || exit 1; done
COPY rr5-b677-gate rr5-b677-receipt-check run sequence verify-call /opt/isolab/rr5-b677/
COPY build-info.json PROTOCOL.md source-manifest.json source.sha256 /opt/isolab/rr5-b677/
RUN chmod 555 /opt/isolab/rr5-b677/rr5-b677-gate /opt/isolab/rr5-b677/rr5-b677-receipt-check \
    /opt/isolab/rr5-b677/run /opt/isolab/rr5-b677/sequence /opt/isolab/rr5-b677/verify-call \
    && chmod 444 /opt/isolab/rr5-b677/*.json /opt/isolab/rr5-b677/*.md /opt/isolab/rr5-b677/*.sha256 \
    && rpm -qa | sort > /opt/isolab/rr5-b677/runtime-packages.txt
ENV HOME=/work
WORKDIR /work
USER 1000:1000
ENTRYPOINT []
CMD ["/bin/false"]
