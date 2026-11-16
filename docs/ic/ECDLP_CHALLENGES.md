# Verifier-backed ECDLP index-calculus challenges

## Purpose and evidence boundary

This package gives contributors six separable places to improve index-calculus
code while keeping acceptance mechanical. A candidate emits JSON certificates;
the native verifier recomputes the elliptic-curve, modular-linear-algebra,
descent, transfer, and orbit claims. The committed fixture is a public
known-answer software control on a 19-point subgroup, not evidence about a
standard curve or a performance result.

The six frontier coordinates are intentionally not collapsed into one score:

| Track | Verified object | Frontier coordinate |
| --- | --- | --- |
| `relation_forge` | EC relation rows and their modular independence | independent rank, maximize |
| `factor_base_lab` | factor-base membership and target decompositions | distinct decomposed targets, maximize |
| `matrix_smash` | `Ax=b`, rank, and a full left-kernel certificate | matrix columns solved, maximize |
| `descent_tree` | a closed EC addition tree with factor-base leaves | internal nodes, minimize |
| `transfer_audit` | an explicit map, subgroup action, and recovery | recovered supplied points, maximize |
| `orbit_relations` | signed orbit reconstruction and folded rows | folded columns, minimize at preserved rank |

`transfer_audit` requires the asymptotic-advantage obligation to remain
`unknown` in this fixture. A verified map is not a verified speedup.

## Requirement-to-evidence ledger

| Requirement | Planned durable evidence | Acceptance gate |
| --- | --- | --- |
| Six large IC problems can be worked independently | one typed submission section and one receipt section per track | every section independently rejects a mutation |
| Other contributors can submit faster implementations | stable JSON fixture/submission schemas and a native CLI | baseline generation and third-party submission verification both work |
| Results are verifiable | verifier recomputes group equations, rank, residuals, tree closure, maps, and orbit folding | no candidate-supplied metric is trusted |
| A frontier can be established | direction-tagged six-coordinate receipt | no aggregate score or cross-unit comparison |
| Harness works end to end | sibling autoresearcher invokes this binary through its existing `run` program | isolated integration test produces an immutable run record |
| Claims stay scoped | receipt claim-boundary fields and this document | performance/security/transfer advantage remain unmeasured or unknown |

## Source context

The design follows the repository records for Semaev summation polynomials
(`KN-LIT-001`), Gaudry-style index calculus (`KN-LIT-002`), symmetry use
(`KN-LIT-004`), Block Wiedemann (`KN-LIT-017`), large-prime variants
(`KN-LIT-164`), and cover/descent work (`KN-LIT-634555`, `KN-LIT-b4a89c`).
Those sources motivate the decomposition; they do not establish that the toy
frontier extrapolates.

## Intended use

Build the `ecdlp-challenges` binary, generate a baseline submission for the
committed fixture, then verify it. Output paths are create-new and are never
silently overwritten. A verifier receipt establishes only correctness on the
named fixture. Runtime, memory, preprocessing, failures, and complete ECDLP
costs need a separately frozen experiment before any performance claim.

```sh
cargo build --release --bin ecdlp-challenges
target/release/ecdlp-challenges baseline \
  --fixture fixtures/ecdlp_challenges/v1.json --out /tmp/submission.json
target/release/ecdlp-challenges verify \
  --fixture fixtures/ecdlp_challenges/v1.json \
  --submission /tmp/submission.json
```

The committed fixture file SHA-256 is
`0399b42507f2c0c410bf801935ece4c00d5d7a16dc9b7276cfebf77f0115eb65`.
