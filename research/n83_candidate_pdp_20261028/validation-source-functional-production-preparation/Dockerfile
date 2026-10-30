FROM 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:c044fe7656fffe45a5ced4b7b5c6ebe6cdd4ada5815fe254c6e657fe92535fa2
COPY --chmod=755 source-functional-production-stage /source-functional
USER 1000:1000
WORKDIR /work
ENTRYPOINT []
CMD ["/source-functional/test-instrument"]
