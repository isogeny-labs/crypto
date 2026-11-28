# Retained K600 V5 composed-adapter admission

Frozen before the first binding check. The source is the commit that contains
this protocol and the `primary-v5-retained-*` exporter commands. This is one
exact adapter admission, not a relation search or a total-runtime comparison.

## Question and fixed input

Does V5 ordinal `26960859236` decode to the primary `a=0` K600
`public_x_hash`, seed `2026100801`, signed-Frobenius base with six summands,
zero split bits, SAT backend, binary enumeration, zero large primes, one
thread, modular BigUint linear algebra, and the composed
`signed_frobenius` plus `ordered_first_pair` mask `18`? Does the retained v3
store manifest bind the exact compressed object SHA-256
`3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`
(3,025,798 bytes) at `s3://setec-astronomy-artifacts/store`?

The read-only manifest is
[`k600-v3-s3-adoption-20261115/attempt-1/manifest.json`](../k600-v3-s3-adoption-20261115/attempt-1/manifest.json).
The checked CLI command is:

```sh
cargo run --release --locked --offline -j 1 --example koblitz_n83_factor_base_export -- \
  primary-v5-retained-binding-check 26960859236 \
  research/koblitz_n83_factor_base_sweep_20261008/verification/k600-v3-s3-adoption-20261115/attempt-1
```

Run the exact ordinal and one adjacent-ordinal rejection under the focused
release example tests. Record the command, source commit, host, toolchain,
exit, and raw receipt. The exact binding passes only if the native V5 decoder
verifies all three pinned parent hashes and the saved V5 design bytes, the
selected axes match, and the manifest matches the object identity. A failure
stops admission and is retained. The binding check makes no S3 request and
performs no solver work. A later cold run needs its own frozen source/input,
positive controls, cgroup and outer wall limits, cumulative setup accounting,
matched rho, and independent replay.

The `primary-v5-retained-chain-cold` route is implemented for this exact
ordinal and invokes the existing ordered-orbit chained SAT path only after
the same binding check. Its output sidecar records the binding separately
from the worker's run directory. The route is not launched by this admission
protocol. Other V5 ordinals remain without a composed execution adapter.
