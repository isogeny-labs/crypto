# Task image for the t = 14 cap-6 cells. Template: taskq/deploy/eks-karpenter/Dockerfile.ecbench (same base, UID 1000).
# Build context: pkm_tower_pilot (static x86_64-unknown-linux-musl), run_cell.sh, build-info.json.
FROM public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39
ARG SOURCE_REVISION
LABEL org.opencontainers.image.source="https://github.com/aburan28/crypto" \
      org.opencontainers.image.revision="${SOURCE_REVISION}"
COPY pkm_tower_pilot /usr/local/bin/pkm_tower_pilot
COPY run_cell.sh /usr/local/bin/run_cell
COPY build-info.json /opt/isolab/build-info.json
ENV HOME=/work
WORKDIR /work
USER 1000:1000
ENTRYPOINT []
CMD ["/usr/local/bin/pkm_tower_pilot", "--help"]
