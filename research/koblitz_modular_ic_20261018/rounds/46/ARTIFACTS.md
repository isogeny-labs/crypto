# Verified R46raw archive

SHA256 `c197a208291677f28a9904eb1a60bc23d01e9f2f439d8e501ac1c6d72e520b64`;837896bytes;71regular files,29historical source pins.
Source revision `a14fcb170dd13af6efe1e246f891aade7fdd654b`. setec-astronomy-artifacts/store holds
all4accepted native runs and1accepted Lean workflow. Fresh GetObject, all
member hashes and byte-identical deterministic repack checked separately.
Publication checks verify historical source blobs and per-attempt hashes.
No independent mathematical replay or semantic review is claimed.

Restore using `ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/46`,
extract runs.tar.xz and run `sha256sum -c runs.sha256` there. Read FORMAT.md.
In-process polynomial factors were not separately exported; complete point
lists and sources support deterministic replay with fresh attempt names.
