# Reproduce the external cofactor output controls

Use an isolated checkout of published source
`3176dae78f371c9b5f4357f74835c161ecf52ce1`. Materialize the complete repository
or every path in `evidence/separating_v1/production_source_manifest.json`.
The exact-source dependency closure deliberately refuses missing files or
stale pins. Install msolve 0.7.5 and its dependencies. The retained executable
hash is in `evidence/separating_v1/pre_call.txt`; rebuilding on another host
need not reproduce the binary bytes.

```sh
export CARGO_TARGET_DIR=/tmp/rr5-output-build
cargo +1.99.1 test --locked --release \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --features production-controls -- --test-threads=1
cargo +1.99.1 build --locked --release \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --features production-controls

# Run each panel as a separate invocation. Set MSOLVE to its installed path.
(ulimit -v 2097152; RAYON_NUM_THREADS=1 timeout --kill-after=5s 60 \
  "$CARGO_TARGET_DIR/release/dickson-four-summand" 127 4 "$MSOLVE" \
  cofactor-msolve refined)
(ulimit -v 2097152; RAYON_NUM_THREADS=1 timeout --kill-after=5s 60 \
  "$CARGO_TARGET_DIR/release/dickson-four-summand" 8191 4 "$MSOLVE" \
  cofactor-msolve)
(ulimit -v 2097152; RAYON_NUM_THREADS=1 timeout --kill-after=5s 60 \
  "$CARGO_TARGET_DIR/release/dickson-four-summand" 8191 5 "$MSOLVE" \
  cofactor-msolve refined)
```

This is a local correctness protocol. It records incomplete outcomes and
fails nonzero after emitting any exact-set mismatch. A zero exit is not a
completion certificate. Exact timing behavior can vary across hosts; retain
all outcomes instead of repeating until a desired status appears.

Each version-2 cofactor system contains `prepared`, `input_prefix`, solver
arguments and bounded `attempts`. Reconstruct an attempt's actual solver
input as `input_prefix + ",\n" + linear_equation + "\n"`. The input includes
the original equations and one additional coordinate equation. Raw RUR text
is retained up to the stated cumulative per-query cap. The raw-file manifest
`evidence/COFACTOR_OUTPUT_SHA256SUMS` covers the initial failed qualification,
the separate diagnostic and this new protocol's receipts.

The two small genericity-contract fixtures can be passed directly to msolve
with `-f INPUT -o OUTPUT -t 1 -c 0`, under a three-second outer timeout and
2 GiB address-space cap. The retained nonseparating fixture returns exit zero
and an empty output with its explicit diagnostic; the A=x fixture returns
a degree-two/dimension-two RUR. [pcc:unverified]

Keep the original failures separate: the first external source is
`5ea8e6940e`, and the diagnostic source is `b67f9770de`. Their protocols and
before-call records remain in `COFACTOR_MSOLVE.md`, `COFACTOR_DIAGNOSTIC.md`,
`evidence/cofactor_msolve_v1/` and `evidence/cofactor_diagnostic_v1/`.
The initial failing call did not retain its second RUR; the diagnostic added
failure-record emission before replaying. That evidence gap remains explicit.

No external delivery is started by these commands. To capture later approved
runs, use the existing local streaming wrapper with this feature-enabled
binary, or the generic `research-stream capture` command in the exact command
receipt. The receiving Cloudflare URL is still missing.
