FROM public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39
ARG SOURCE_REVISION
ARG BINARY_SHA256
LABEL org.opencontainers.image.source="https://github.com/isogeny-labs/crypto" \
      org.opencontainers.image.revision="${SOURCE_REVISION}"
# The build context contains the tested executable and frozen public inputs only.
COPY --chmod=0755 crypto /usr/local/bin/crypto
COPY --chmod=0644 protocol.json pilot-plan.json build-info.json /opt/ml-dataset/
RUN test "$(sha256sum /usr/local/bin/crypto | cut -d' ' -f1)" = "$BINARY_SHA256" \
    && mkdir -p /work && chown 1000:1000 /work
ENV HOME=/work
ENV AWS_DEFAULT_REGION=us-east-2
ENV RAYON_NUM_THREADS=1
WORKDIR /work
USER 1000:1000
ENTRYPOINT []
CMD ["/usr/local/bin/crypto", "dataset", "capabilities"]
