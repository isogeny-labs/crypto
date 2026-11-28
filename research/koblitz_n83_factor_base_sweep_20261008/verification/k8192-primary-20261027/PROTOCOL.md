# Primary N83 K8192 factor-base construction gate

Freeze the remaining primary `v2-size-frontier` cell: curve `a=0`, subgroup
order `2417851639230796216685689`, `public_x_hash` policy, seed
`2026100801`, and 8,192 signed-Frobenius orbit columns. A completed
full-orbit base must have 1,359,872 distinct point records. Use the same
exporter source commit as the K4096 and K16627 controls,
`f61b33c1b1bb98efa22177a615270c160859a605`, with source SHA-256
`cf31a5e9e8d5aad35aeb3c73be352992bc4e40ca6cbd425ad0ccf8e14de89340`.

Run `v2_object_supervisor.py` into a new external directory with the frozen
Linux arm64 exporter, immutable image ID, one CPU, 6 GiB Docker memory,
zero swap, no network, a read-only repository mount, and 1,800-second outer
and worker caps. Preserve the complete object or cap, manifest, config,
stdout, stderr, and outer status. On construction pass, run bounded generic
`v2-replay-one` under a separate 1,800-second cap, checking every point,
representative, subgroup projection, Frobenius identity, orbit closure and
point-set digest. Compute independent compressed-object SHA-256/BLAKE3 and
check the exact versioned S3 key by fresh HEAD and GET. Preserve any
pre-existing matching object without overwrite; record storage failure or
mismatch explicitly.

This gate measures construction and verifies the object. The frozen support
model gives the four-summand smoothness ceiling `0.05893227` and
924,626,608,128 full-pair multisets. Neither is a relation yield or
runtime measurement. Keep total IC runtime and selected winner null until
natural relation/rank search and a fully charged same-target comparison.
