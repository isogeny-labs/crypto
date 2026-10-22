# Quadratic twists: full native replay and checked algebraic implications

The full finite workload now runs in C++17. All **434 deterministic fields**
match the frozen historical results: 942 nonsingular models, 4,188,162 ordered
S3 triples, 113,342 ordered S4 quadruples, the subfield and trace-zero fixtures,
all 81 toy IC logarithms, and the four transported relation collections.
The historical Python source and records remain unchanged in their archive.

The [2026-10-22 mixed-relation experiment](reports/mixed-20261022T0718Z/README.md)
adds **74,693,760 exhaustive signed words** on 274 shared odd-prime subgroup
cases, independently checked with Rust tuple arithmetic and a multiplicity DP.
No mixed-component counterexample or relation-rank gain appears. Nonzero
two-torsion exceptions remain visible. Ten new Lean statements construct the
explicit short-model twist image, prove its Frobenius negation without an
assumed hB, and derive the single-point and finite-sum odd-order obstructions.
There are now 28 named research theorems; full geometric formalization remains
partial. [New raw records](native/mixed-20261022T0718Z/),
[protocol](MIXED_RELATIONS_20261022_PROTOCOL.md),
[proof map](lean/README.md), and [validation](validation/mixed-20261022T0718Z/README.md).

Reproduce the new bounded experiment and independent controls with
`bash research/quadratic_twist_evidence_20261021/run-mixed.sh /tmp/twist-mixed-fresh`.
The directory must be fresh. The workflow runs this replay, and the existing
proof gate includes the new Lean module. No general hardness or speedup follows.

- [Full native replay receipt](native/runs/run-20261021T150125Z/parity.json),
  [raw output](native/runs/run-20261021T150125Z/results.json), and
  [execution transcript](native/runs/run-20261021T150125Z/run.log).
- [Independent audit of 84 IC certificates](native/runs/run-20261021T150125Z/independent-ic-audit.json):
  three headline matrices plus the 81 complete-log runs, recomputed with the
  separate tuple-based arithmetic in [audit.cpp](audit.cpp).
- [Tower-to-polynomial-basis certificate](native/runs/run-20261021T150125Z/basis.json).
- [False relation, missing coverage, and false reference controls](native/controls-20261021T150300Z/run.log).
- [Fourteen group/ring theorems](lean/TwistProof.lean) and
  [four actual elliptic-curve Frobenius theorems](lean/FrobeniusBridge.lean), their
  [proof map](lean/README.md), and [earlier 18-theorem proof-gate receipt](validation/lean-bridge-gate/).
- [Three IDC1 identities](certificates/identity-index.json), each linked to the
  exact checked Lean source. The native checker uses 32 derived points and
  16 extra points and rejects the deliberately false identities.
- [Exact model catalog](../../docs/curves/sources/quadratic-twist-native-20261021.json):
  1,921 ICV1 models and 27 selected subgroup representations with full EC1 UIDs.
  Census models with no selected cyclic generator retain that unknown explicitly.
- [Native workload report, graph, and PDF](reports/native-20261021T150125Z/README.md).
- [Cover catalog CI follow-up](reports/catalog-20261022/README.md): native replay
  and assessment of all 4,493 current models; earlier catalog records and dated
  reports are retained.
- [Current metadata follow-up and inspected PDF](reports/metadata-20261022T0500Z/README.md):
  canonical native import layout, all-model roster, preserved catalog values and
  exact predecessor/current source pins; inherited CI and review blockers retained.
- [Requirement status](REQUIREMENTS.md) and [remaining formal/coordination obligations](../../problems/quadratic-twist-evidence-native-parity.md).

The Lean proofs establish the stated group/ring implications, with explicit
hypotheses. The actual Frobenius point map and rational-point lifts are now
formalized. The twist-to-minus-eigenspace identification, summation-polynomial
lifting and Weil-pairing bridges remain open.
This study establishes finite correctness and specific obstructions; it supplies
neither a general ECDLP hardness proof nor a general GHS attack or speedup result.
Invalid-curve attacks are outside its scope; every experiment uses valid points.

## Reproduce

Requires C++17, OpenSSL development headers/library, Bash, tar/xz, sha256sum,
jq, and Rust 1.99.1. The isolated Rust metadata crate reuses the repository's
native ICV1/IDC1 implementations; it does not build a substitute Python path.

```sh
study=research/quadratic_twist_evidence_20261021
bash "$study/run-native.sh" /tmp/twist-native-fresh
bash "$study/test-native.sh" /tmp/twist-native-fresh /tmp/twist-controls-fresh
cargo test --release --locked --manifest-path "$study/native/metadata/Cargo.toml"
cargo run --release --locked --manifest-path "$study/native/metadata/Cargo.toml" -- \
  "$PWD" /tmp/twist-native-fresh/results.json /tmp/twist-metadata-fresh
cmp /tmp/twist-metadata-fresh/curve-models.json docs/curves/sources/quadratic-twist-native-20261021.json
```

Each output directory must be fresh. `run-native.sh` compiles snapshots of its
actual sources and records their hashes, compiler, input/output digests, exit
status, and UTC boundaries. Its snapshot duplicates `source.cpp` as `full.cpp`
so the separate basis checker can include the same arithmetic implementation.
The script verifies the archive and all historical manifest entries before
accepting the separately audited new certificates.

The parity comparison excludes only process timing, source provenance, and
SymPy version fields. It compares every historical mathematical field, including
randomly collected relation matrices and attempt counts. The C++ integer-seeded
MT19937 reproduces the original rejection-based integer draws. Exact sparse
polynomial expansion over Z replaces SymPy. New output also records per-model
S3 relation-set hashes and all 81 newly collected IC certificates, which the
original archive did not retain individually. No timing is admitted as a benchmark.

For proof checks, install the pinned Lean toolchain, then run:

```sh
tools/lean/proof-gate.sh --out /tmp/twist-proof-receipts "$study/lean"
```

The scoped Actions workflow reproduces the native run, controls, certificates,
and model catalog; the repository-wide Lean gate builds and audits the project.

## Frozen provenance

[Historical transcript](historical/run.log), [raw results](historical/results.json),
[metadata](historical/run-metadata.json), [source archive](historical/source-and-evidence.tar.xz),
and [checksum](historical/source-and-evidence.tar.xz.sha256) retain the original
Python provenance. The earlier [import report](reports/README.md), saved audit
sources/receipts, and failed native port/proof attempts are retained. Earlier
statements that the full native port was missing describe that earlier record.

The original certificate-only path remains available:

```sh
bash "$study/run-audit.sh"
bash "$study/test-audit.sh"
```

No benchmark or leaderboard row is promoted. Canonical IC/rho cost graphs remain
unchanged because there is no new admissible cost comparison. The new study graph
shows the expanded native coverage. Research records stay loose where review,
source, digest, and replay links name their individual paths.
