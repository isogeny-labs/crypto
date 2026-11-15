# Wave-6 leads 1 to 4, run in parallel (2026-11-15)

Owner: "1-4 all parallel". Four frozen protocols, one per lead, each in its own directory. One new build of the
repository's binaries serves all four (its sha256 is recorded at each launch); the exponent studies' frozen binary
dc2c30a4 stays the reference for chain cells, and L3 re-runs two chain reference cells with the new binary to check
that field_ops are unchanged. Host: the shared 4-core, 15 GB container; one core per lead; the F4 leads run under
`ulimit -v 6000000` (6 GB each) so that two can run at once. No EKS, Conductor, S3 checkpoint, benchrec record or
ecdb lookup is reachable from this runtime (disclosed, as in the exponent studies). Every number is [pcc:unverified].

## Binaries (built 2026-11-15T01:00Z from branch commit 7695c57489 plus the snapshot of the engine knobs; CARGO_INCREMENTAL=0, release)

- `curve25519_family` sha256 5abc70ba47b1192df71a5e93d697f095e59a0e5f0132cf47db1d0d34d34e3746
- `ecbench` sha256 fc6bd9b710ffe0237ab77ae9338b379baf84071fb78cb5c288e92aff8e2bd4e8
- frozen reference for chain cells: `curve25519_family-dc2c30a4` (dc2c30a4242d80281622b2cb42cbaf9cb457d041ffbc9a4d35794bb893bf3faf)
- toolchain: rustc 1.97.0 (2d8144b78 2026-07-07)
- host: 4 cores, 15 GB, kernel 6.18.44-fc-v470
