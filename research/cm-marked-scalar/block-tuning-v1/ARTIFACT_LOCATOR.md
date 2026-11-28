# Managed raw archive locator

The repository bot externalized runs.tar.xz in commit be6490725cd3299f85562136df7b3ba231755757. Its committed .artifacts.json pins110312bytes and SHA25611f5de7252cc1a9d2f26dd878b8eb8c5b93780e843e9ee79de9415bb71c559c8. The remote in .artifacts/config.json and key rule in tools/artifacts/artifacts.py produce this content-addressed locator:

`s3://setec-astronomy-artifacts/store/sha256/11/11f5de7252cc1a9d2f26dd878b8eb8c5b93780e843e9ee79de9415bb71c559c8`

This is the locator derived from committed store metadata, not a claim that this agent independently read the remote object. The local archive was restored from its retained exact backup after a provenance-preserving fast-forward and its checksum passes. No agent-initiated S3 GET or scientific scan retry was performed. The ignored local runs tree remains available; a fresh extracted copy and native replay output are byte-identical. [pcc:unverified]

Use the repository's approved artifact hydration path, then the checksum/member/replay commands in README.md. Keep the producer's local integrity checks distinct from signed PCC standing.
