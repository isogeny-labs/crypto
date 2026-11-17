FROM 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:c044fe7656fffe45a5ced4b7b5c6ebe6cdd4ada5815fe254c6e657fe92535fa2
ARG SOURCE_COMMIT
LABEL org.opencontainers.image.revision=$SOURCE_COMMIT
COPY --chmod=755 centered-panel-stage /centered-panel
COPY --chmod=755 centered-panel-stage/source/validation-centered-binding/fixtures /home/ec2-user/crypto-n83-degree-sweep-20261027/research/n83_candidate_pdp_20261028/validation-centered-binding/fixtures
USER 1000:1000
WORKDIR /work
ENTRYPOINT []
CMD ["/centered-panel/test-instrument"]
