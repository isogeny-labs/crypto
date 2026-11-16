# Lean proofs for the planner studies

Lean core only (`leanprover/lean4:v4.34.1`, no Mathlib), checked in CI by
[`.github/workflows/planner-proofs-lean.yml`](../../../.github/workflows/planner-proofs-lean.yml).
This package is item 3 of [`../PROTOCOL.md`](../PROTOCOL.md), "What is proven in Lean".

| file | what it proves |
|:--|:--|
| `Band.lean` | The AGENTS.md §5a band is affine in the weight. In integers, for every weight `p/q`: `order_at`/`order_at_le` (when one arm is cheaper), `crossing` (a reversal between w = 0 and w = 1 forces a crossing weight strictly inside), `sides` (one strict order below the crossing, the other above), and `optimal_native_nonincreasing` (if arm i is optimal at w and arm j at v > w, then j has no more native work than i). |
| `KeyedInvariance.lean` | A keyed-generic model. An algorithm sees elements only through `add`, `neg`, handle equality and a key per element. `exec_map`: on two carriers joined by an injective map that preserves `add`, `neg` and the key, the whole transcript is identical: every observation (equality answers and keys read), the operation count, and the answer. `run_transported_key` is the corollary for the residual collector: running it on E is the same as running it on Z/r with E's key pulled back along k ↦ [k]G. A concrete run on Z/7 (closed by `decide`) shows the hypotheses can be satisfied. |
| `Certificates.lean` | Generated, not hand-written: `../scripts/lean_certificates.sh` builds it from the committed band sums (see below). It holds 2428 integer verdicts from 39 band files, each closed by `decide`: 2010 pairwise orders, 58 exact crossing weights and 360 argmin facts behind every `m*(w)`. |
| `AxiomAudit.lean` | Collects every theorem in the three namespaces from the environment, so there is no hand-kept list. It fails on any axiom outside `propext`, `Classical.choice` and `Quot.sound`. |

## What the certificates restate

Each verdict is a point fact about the committed exact sums of `ecbench band`:
`charged_sum` (Σ (G − C) over verified runs) and `native_sum` (Σ (U + n_C)).

An arm's mean priced cost at `w = p/q` is `(q·c + p·n) / (q·v)`, where `v` is the
arm's number of verified runs. So "A cheaper than B at w" becomes
`v_B · scaled(c_A, n_A, p, q) < v_A · scaled(c_B, n_B, p, q)`.

Some charged sums are not integers (calibration-priced canonicalisation, on the
Koblitz rho arms). For those, the generator takes the exact dyadic value of the
f64 that `ecbench` printed, and scales every sum in that band file by the
common denominator.

`m*(w)` is restated as the argmin of the product over the family's curves,
with ties going to the smaller m. G is a geometric mean of ratios to the plan,
and the plan and √r factors are common to every m, so the two argmins agree.

Where a study reported an `m*(w)`, the generator stops unless its own argmin
matches. The checked reports are `derivation.json` of
`ic_planner_weight_rule_20261021` and of `ic_planner_weight_rule_binkob_20261026`.

The sources are listed, with their sha256, at the top of `Certificates.lean`:
- this study's `analysis/band-*.json`;
- the four earlier studies' sessions, re-banded by `../scripts/rebands.sh`
  into `analysis/rebands/`.

## What is not proven here

- **Bootstrap intervals.** They are statistics, from `ecbench compare`.
- **That the Rust code is the Lean model.** For `KeyedInvariance`, the claim
  that `ic_framework::residual` touches elements only through those four
  operations is checked by reading the code. For the certificates, the integers
  are whatever `ecbench band` reported, so the proof is only as good as the
  band's arithmetic. `ecbench`'s own test
  `exact_sums_reproduce_the_band_means` checks that arithmetic against the
  printed means.

## Checking it

```sh
bash ../scripts/lean_certificates.sh   # must leave Certificates.lean unchanged
lake build
lake env lean AxiomAudit.lean          # ends with AXIOM_AUDIT_COUNT|Certificates|2428
lake env leanchecker --fresh Band
lake env leanchecker --fresh KeyedInvariance
lake env leanchecker --fresh Certificates
```

## Local logs

These logs were made on 2026-10-26 with the release toolchain, running `lean -o`
directly; lake was not used locally, for lack of disk space.

| log | contents | result |
|:--|:--|:--|
| `axiom-audit.log` | `AxiomAudit.lean` | exit 0 |
| `print-axioms.log` | `#print axioms` for every theorem | 2428 certificates with no axioms; the main theorems use only the standard three |
| `leanchecker.log` | `leanchecker --fresh` for each module | exit 0 |

A deliberately flipped certificate fails `decide`, as it should; this was
checked during development.
