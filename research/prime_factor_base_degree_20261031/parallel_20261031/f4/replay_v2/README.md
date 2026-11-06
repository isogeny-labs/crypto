# Exact offline replay, version 2

This verifier reads saved input, F4 basis, trace and original result files. It
never launches CAS, raises the v1 root-search budget, edits v1 source or raw
records, or discards a probabilistic branch. It accompanies the original
results, including their 26 root-extraction caps.

## Exact finite-field proof

Write I for the original complete system, G for the saved exported basis, and
R for the independently enumerated signed group-relation assignments.

1. Independently reconstruct and compare every coefficient of I. Check exact
   split-graph factorization for the candidate, complete rational membership
   for controls/dense systems, signed inventory and intended branch metadata.
   The established secant/group and S3 equivalence gives R=V_Fp(I).
2. Exact normal-form reductions of every original generator against G give
   explicit polynomial identities f_i=sum(a_ij*g_j), so V_Fp(G) is contained
   in V_Fp(I)=R. This implication does not require a probabilistic search or
   enumerating a coordinate cube.
3. Evaluate every point of the complete reference R in every saved g in G.
   When they all vanish, R is contained in V_Fp(G), proving equality.
4. Also check the Buchberger S-pairs. Check producer reference roots when
   available and compare every available v1 extracted-root list with R.

This proves **finite-field root-set equality**. It does not certify equality of
ideals/multiplicities, nor does it relabel the original root-extraction cap as
a successful extraction run. In particular, group-pair enumeration here is
verification work; it is not credited as F4 relation discovery or omitted from
an end-to-end claim.

## Source and raw-evidence binding

`build.rs` checks the immutable v1 `../main.rs` hash
`e201eda231b41820b1733eefd725841036075aadceb9d4cada92926893025842`.
It includes the unchanged pre-test helpers in a private module. Only the two
leading inner doc comments become ordinary comments and the original test
module is omitted to avoid relocating its relative fixture paths. The original
entry point remains an unused module function; no v1 algorithm is altered.

The companion checks original input/stdout/stderr hash linkages, records the
original result's hash/status/error, and emits its own source hash and raw
hashes. Original failed records remain failed/capped records. Per-case output
is `f4/.../cell-NNNNN/replay.json` under a **new companion root**; the final
`manifest.json` contains companion paths and exact hashes.

The implementation uses only deterministic algebraic replay. It reuses the
frozen model/field/group primitives, so it is a separately versioned proof
route, not an independent implementation of those primitives. Independent
review is recorded by the parent.

## Build and bounded analysis

```sh
cargo test --release --locked --manifest-path Cargo.toml
cargo clippy --locked --manifest-path Cargo.toml -- -D warnings
cargo build --release --locked --manifest-path Cargo.toml
timeout --signal=TERM --kill-after=5s 300s bash -c '
  ulimit -v 2097152
  exec target/release/secant-f4-offline-replay-v2 run \
    ORIGINAL_RUN_ROOT FRESH_COMPANION_ROOT 240
'
```

One process/CPU, 2 GiB address-space cap, 240-second internal analysis cap and
300-second outer cap. No new scientific experiment or CAS call is authorized
by this README. The parent explicitly requested this offline analysis after
the original scientific panels finished under their existing caps.

Four artificial tests check both inclusion directions, rejection of a
stricter wrong basis and a basis admitting an extra root, and a unit/empty
reference. A first build needed a larger Rust macro recursion limit for the
companion JSON literal; strict Clippy also found an unused import, which was
removed. No mathematical gate or frozen source was weakened.

## Observed offline replay

The bounded offline command completed with exit status 0. It verified all
**14,220 saved cases**, including the **26 originally capped cases**, with
zero failed or unattempted companion cases. The original panel manifests
independently reconcile 14,220 requested and attempted cases. No CAS was
launched and no coordinate cube was searched. The original 26 extraction
caps remain operational caps; the companion proves the saved bases' exact
finite-field root sets without changing those original results.

The immutable companion root is
`/workspace/scratch/secant-parallel-runs/replay-v2-20261031T201044Z`.
Its `manifest.json` is 4,767,211 bytes with SHA-256
`adca412a6d4adcd970989aa8342d58060db4bb472c9435dbadad0e20a41be8f0`.
Every companion record links its original input, solver output, trace and
result hashes. The sibling process stdout/stderr files are empty; no separate
elapsed-time measurement was recorded. Parent publication packages the raw
run and companion trees together; their individual files remain unchanged.

See [VERIFICATION.md](VERIFICATION.md) for the exact execution command and
checks, and [source-lock.json](source-lock.json) for source/binary hashes.
