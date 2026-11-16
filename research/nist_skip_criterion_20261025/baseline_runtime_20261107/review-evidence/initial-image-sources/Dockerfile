# Local development base only. A published runtime still needs source/image admission.
FROM public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39
COPY root/ /
USER 1000:1000
ENV HOME=/work TMPDIR=/tmp
WORKDIR /work
ENTRYPOINT []
CMD ["/bin/true"]
