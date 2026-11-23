# Minimal local payload: shell/runtime files are allowlisted and hash-bound at staging.
# This is not a qualified EKS recovery image.
FROM scratch
COPY root/ /
USER 1000:1000
ENV HOME=/work TMPDIR=/tmp PATH=/usr/bin:/bin
WORKDIR /work
ENTRYPOINT []
CMD ["/bin/true"]
