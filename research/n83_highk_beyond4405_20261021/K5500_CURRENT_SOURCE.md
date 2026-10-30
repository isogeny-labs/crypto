# Current-source K5500 split-index finalist base

The fixed31 comparison source rebuilt the K5500 signed-Frobenius factor base
with **913,000 distinct subgroup-usable points in 5,500 folded columns**. Its
ordered dump SHA-256 is
`823782912c75d782cdd7e6c9074c94282e28a686b1dd98ea978a367514d3acdd`,
byte-identical to the earlier independently replayed K5500 dump. Its producer
BLAKE3 base hash is
`0789049953a67b51dfe23f6fa2788d11e0bb4b35d52180fc038a3ffd2c0efb5f`.
The prior replay checked every representative, point, subgroup condition,
signed Frobenius label and exact K4405 ordered prefix. The current-source
byte identity carries those checks to this executable's base output.

The base-only process used IC source SHA-256
`58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`
and macOS ARM64 release binary SHA-256
`07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a`.
It read one earlier public N83 fixture with `KIC_BASE_BUILD_ONLY=1` under a
300-second cap. The process ran from `2026-10-21T19:00:59.833400+00:00` to
`2026-10-21T19:01:04.753741+00:00`, and reported 4,003.955167 ms of base
construction. That interval is a local construction diagnostic under
shared-host load.

The [current-source receipt](k5500_current_source_base/receipt.json),
[byte-identity verification](k5500_current_source_base/byte_identity_replay.json),
and [artifact manifest](k5500_current_source_base/artifact_manifest.json) are
versioned. The full dump remains on the evidence volume; its canonical
compressed copy is already versioned at
`research/n83_two_tier_root_20261021/runs/base_only_k5500_20261021/base_dump.jsonl.gz`
with SHA-256
`082fcb7c57f64bc2db8299e748dfcdcd1c7b3317815f3f1f7c1a2b842e0391d2`.
The split index's full build, rank, one-target recovery and Tier 1 pairing
are the next gates. Its `candidate_id` remains `null` until the complete
pipeline configuration is sealed.

## Prepared full-process control

`run_k5500_full_control.py` fixes the ID31 split at K4549, separate `wide3`
Bloom filters, 80% table-request rule, 11-byte root states, four root-build
threads, four rank threads and two target workers. It uses the earlier public
fixture point 0, outside the sealed high-K holdout, and keeps the known scalar
in the verifier-only sidecar. The runner bounds one child to 14,400 seconds,
40 GiB sampled RSS and at least 8 GiB free output space. It records source,
binary, command, flags, phase output and every cap or failure status.
`verify_k5500_full_control.py` binds a completed run to the already replayed
ordered base, checks the 1,255,377,750-state ID31 layout and exact table and
Bloom allocations, requires rank 5,500, and independently replays the target
scalar and four-point relation. The expected state, two-table and two-Bloom
arrays total 33,136,508,082 bytes before other buffers; a completed process
must still pass its actual peak-RSS gate.

The no-launch preflight passed against the current-source macOS ARM64 binary
`07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a`:
it fixed source SHA-256
`58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`,
split K4549, 14,400 seconds and 42,949,672,960 RSS bytes. This preflight
does not allocate the index. Run the control on a host with at least 48 GiB
effective memory and enough free output space, then replay the output:

```sh
python3 -m research.n83_highk_beyond4405_20261021.run_k5500_full_control \
  --binary /absolute/path/to/koblitz_orbit_dlp_fast \
  --output /absolute/path/to/new-k5500-output
python3 -m research.n83_highk_beyond4405_20261021.verify_k5500_full_control \
  --output /absolute/path/to/new-k5500-output
```

This source bridge is a correctness and resource stage. A Tier 1 single-target
comparison with K1107 and the selected fixed31 arm still needs its own frozen
same-point workload, rho pair and complete-cost receipts before an N83
factor-base choice can include K5500.
