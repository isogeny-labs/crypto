# Frozen custody of the four N83 cross-family factor bases

The [plan](plan.json), SHA-256
`9b221e6d61dab95689e88a21e72f950f8b9a551be9ea671bba0ddf4e9d2cd343`,
pins the four exact point-defined bases in [provider run
39075872898](https://github.com/aburan28/crypto/actions/runs/39075872898).
It binds all four artifact IDs and downloaded ZIP hashes, source commit
`577c8cbe9ddfa20a2b6a62d8c39eb1dd339497ed`, candidate manifests,
curve parameters, raw base hashes, and storage roles. This is the degree-83
**a=1 diagnostic curve with a 53-bit subgroup**, separate from the a=0
K600 SAT sweep.

Require all 24 repeated byte copies of each base dump, six per independent
stratum, to match its pinned raw SHA-256 and length. Decode one exact copy
and check the curve, base hash, orbit columns, point count, coordinate bounds,
and uniqueness. Deterministically gzip it with empty filename and mtime zero.
The K2816 and K3072 raw dumps are expected to reproduce the two already
audited root11 S3 objects byte for byte; reuse those addresses only if the
compressed hashes, lengths, and ETags also match. Upload K3450/wide3 and
K3500/root4 under the approved
`s3://crypto-autoresearcher/factor-bases/icv1/etc/` prefix using BLAKE3 of
the compressed bytes as each key and `If-None-Match: *`. Never overwrite a
prior object. Fresh-download all four objects and independently check raw
and compressed hashes, lengths, metadata, and single-part ETags. A missing
or changed provider copy, failed conditional PUT, failed GET, or corrupted
download is a custody failure.

This round stores and identifies the factor bases. It does not select an IC
policy or infer relation yield, online speedup, or complete cold runtime from
storage bytes. Keep `selected_best_total_runtime` null. The separate
cross-family experiment has its own four-stratum arithmetic, rank, timing,
rho, calibration, and selector gates.
