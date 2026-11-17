# H114 multiradix carry-cohomology experiment

H114 tests one frozen, bounded ECDLP digit hypothesis: whether the three
first-formal-layer multiplication windings for the faces `(2,3)`, `(2,5)`,
and `(3,5)`, passed through one shared generator-aware channel, determine the
canonical scalar modulo 30 after fixed residue-alphabet permutations.

This directory currently contains the preregistration and generic Lean proof
layer. It contains no candidate scores, selected IDs, decoder outcomes,
runtime measurements, or claim of a generic ECDLP speedup.

## Frozen mechanism

For a coordinate-fixed section `s:E(F_p)->E(Z/p^2 Z)`, multiplication carries
produce bounded integers

```text
w_23 in [-3,2],  w_25 in [-5,2],  w_35 in [-5,3].
```

The six multiplication orders of `(2,3,5)` form a permutohedron. Their lifted
paths all telescope to `[30]s(Q/30)-s(Q)`, and the two paths around its braid
hexagon impose one exact integer identity. H114 treats that identity only as a
structural check. Its candidate digit combines

```text
channel(w_35) mod 2,
channel(w_25) mod 3,
channel(w_23) mod 5
```

by CRT. Eight shared channels and every fixed permutation of the three
residue alphabets give exactly `8 * 2! * 3! * 5! = 11,520` public predicate
IDs. One unchanged ID must be exact on every one of 28 rows and decode all
9,096 targets; an empty intersection rejects the frozen family.

## Evidence index

| Artifact | Role | SHA-256 at freeze |
| --- | --- | --- |
| [`PROTOCOL.md`](PROTOCOL.md) | Human-readable formulas, controls, accounting and acceptance | `92a970c1392183ceb5c84d76082af7b977ebf52145c6b938704cfca24d708c3b` |
| [`PROTOCOL.json`](PROTOCOL.json) | Machine-readable family and run contract | `8d89092331a64a00d51ce7caa1602b035ff582835f8b6f7287912d07590ed51c` |
| [`CURVES.json`](CURVES.json) | Seven-curve cohort and coordinate-isomorphism controls | `5dff248152f4cfb3a8b5f9f39542110767d60779d4b917e4b817a935b6170385` |
| [`mechanism.svg`](mechanism.svg) | Editable six-path/CRT mechanism diagram | `a93c1cc5cd193d0ff6d9321732968cbdc8fd8e8aa06575315c2a5b6fd4988d57` |
| [`mechanism.png`](mechanism.png) | Compact raster rendering | `9d9cf5d3b39e01ec00a0c5785de65f7d48479b71c5feef65752cb3a018e72585` |
| [`H114MultiradixCarryCohomology.lean`](H114MultiradixCarryCohomology.lean) | Generic paths, ranges, braid, CRT and conditional decoder | `6bbc556a0cebeafcd493d829180f2c41ad5fdcb9657faed5ae7aaa70c2734eba` |
| [`proof-gate-v1/`](proof-gate-v1/) | 46 declarations, zero failing, zero trusted-native | retained raw receipt |
| [`h114_multiradix_lab.rs`](h114_multiradix_lab.rs) | Native producer, controls, structural tests and conditional decoder | `795a80d7cd527ac878e54087664aaa1ccf6c46322f7952c7dd213df0a8b90d57` |

The authoritative freeze is the Git tree recorded by the first commit that
adds this directory; the table hashes are convenience receipts, not a
substitute for the tree object.

## Required controls

The evaluation must retain random global sections, histogram-matched labels,
coordinate isomorphisms, the prime-to-`p` torsion section, all path and braid
checks, single-limb ablations, and verifier tamper rejection. Candidate and
control families receive the same 11,520-way selection pressure.

## Execution gate

Implementation follows the freeze. Measurement additionally requires a
pushed source revision, immutable image digest, uploaded checkpoint, and the
frozen EKS Spot budget/deadline. If that infrastructure gate is unavailable,
the absence of data is reported as a blocked measurement rather than silently
replaced with local results.

The source can be checked without scoring the frozen candidate:

```text
rustc --edition=2021 -D warnings h114_multiradix_lab.rs -o h114_multiradix_lab
./h114_multiradix_lab --self-test
./h114_multiradix_lab --structural-test
```

`--self-test` checks SHA-256, all 11,520 unique predicate IDs, CRT residues,
online/cached winding agreement, every canonical path on the first curve, and
the first-curve torsion-section zero control. `--structural-test` checks all
2,274 nonidentity points across the seven curves, including 13,644 lifted path
evaluations and 2,274 exact braid residuals. Neither mode scores a candidate.
The `--quick` and full modes do score candidates and remain behind the
measurement gate.

## Interpretation boundary

Even an exact toy-cohort result would remain a hypothesis until a uniform
proof establishes the digit relation. The planned Lean work separates generic
path/range/CRT/decoder theorems from the empirical oracle premise. Conversely,
a null result rejects these 11,520 predicates, not every use of local-ring
carry cohomology.

