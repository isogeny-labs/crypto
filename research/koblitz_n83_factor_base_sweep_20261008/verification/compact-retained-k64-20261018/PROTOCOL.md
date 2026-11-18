# Retained primary N83 compact-root capacity probe

This rung runs the existing point-only compact four-summand root index on an
exact S3-backed `a=0` N83 factor base. The selected public factor base is the
K64 hash-derived panel row at seed 2026100801. Its object is
`1d0a6695564972f61db13c62c60a300da0cfa7bcd9b363c2e553a34345fae176.jsonl.gz`
under `s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/a0/objects/`.
The preparer checks the frozen manifest, replay and upload receipts, makes a
fresh S3 GET, and verifies the compressed object BLAKE3 before the worker
independently checks all 10,624 point records and labels.

The Linux worker is built from a clean commit for `x86_64-unknown-linux-musl`.
The existing `compact_probe_supervisor.py` snapshots the three relevant Rust
sources and binds them to that commit and compiled worker. It runs a
networkless, read-only Docker container with one CPU, a 4-GiB hard memory
limit, zero swap, and a 900-second outer wall limit. The unordered K64
candidate-state cap is exactly `83 * 64 * 65 / 2 = 172640`. Docker cleanup,
exit status, source and binary hashes, and every worker receipt are retained.

The measured question is whether the exact primary-field compact index can
be constructed under these bounds and answer one frozen public point query.
A returned four-point candidate must pass original-curve group replay and
the full-width signed-Frobenius relation-row check. A miss is one checked
point query, and a cap is censored. Neither outcome estimates repeated
natural relation yield, useful rank, linear algebra, or a complete cold IC
runtime. Build and preparation are charged separately; the full 900-second
search envelope is charged for the attempted worker. No factor-base winner
can be selected from this rung.
