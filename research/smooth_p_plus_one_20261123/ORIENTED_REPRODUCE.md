# Reproduce the oriented-chain development controls

Use an isolated checkout of source
`21a7ef9221c38af6328a8bac45a2ce05f82ad973`, materializing the full repository
or all files in `evidence/oriented_v1/production_source_manifest.json`.
The exact-source closure rejects missing dependencies and stale pins.

```sh
export CARGO_TARGET_DIR=/tmp/oriented-chain-build
cargo +1.99.1 test --locked --release \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --features production-controls -- --test-threads=1
cargo +1.99.1 build --locked --release \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --features production-controls

# Separate processes; the positional solver path is unused by native modes.
(ulimit -v 2097152; RAYON_NUM_THREADS=1 timeout --kill-after=5s 60 \
  "$CARGO_TARGET_DIR/release/dickson-four-summand" 127 4 unused \
  oriented-dense oriented-tower)
(ulimit -v 2097152; RAYON_NUM_THREADS=1 timeout --kill-after=5s 60 \
  "$CARGO_TARGET_DIR/release/dickson-four-summand" 8191 4 unused \
  oriented-dense oriented-tower)
(ulimit -v 2097152; RAYON_NUM_THREADS=1 timeout --kill-after=5s 60 \
  "$CARGO_TARGET_DIR/release/dickson-four-summand" 8191 5 unused \
  oriented-dense oriented-tower)
```

No msolve installation is needed for these two modes. The frozen executable
and inputs are identified in `evidence/oriented_v1/pre_call.txt`; exact capture
commands are in `commands.txt`. The whole-query deadline and native limits
are compiled into this revision. The preserved JSONL contains input systems,
native traversal and incomplete status; process exit zero alone is not a
completed query. Do not rerun unchanged capped calls until a favorable status
appears or replace any historical evidence.

`evidence/ORIENTED_SHA256SUMS` hashes every raw/control file in this protocol.
The original pair example's SHA-256 is retained before calls. Its equations
are the prior source; this four-leaf interface is new adapter code. The native
F4 source closure and its diagnostic build profile are unchanged. No timing
comparison uses this development build.

The [report](ORIENTED_RESULT.md) separates full-base failures from the planted
fixed-slot integration test and semantic witness checks. These commands do
not submit an EKS job or start external delivery. The existing local capture
hook can buffer approved invocations while the receiving Cloudflare URL is
unavailable.
