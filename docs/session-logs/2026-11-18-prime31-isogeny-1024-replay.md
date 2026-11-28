# 2026-11-18 — prime31 1024-target 3-isogeny replay (automation)

## 06:30–06:39 UTC — What was done
- Inspected AGENTS.md, docs/ACTIVE.md, latest prime31 native Rust examples and frozen rank-31 1024-log transcript in #4623. Checked source PRs #4604, #4635, #4647, #4652, #4660, #4671, concurrent #4678, #4681, #4682, #4675, #4668, and independent prime-field index-calculus publications. Shared research-KB service was unavailable in this connector execution; bounded GitHub/primary-source fallback, so no exhaustive novelty claim.
- Extended existing PR #4671 branch `automation/prime31-3isogeny-maxorder-20261118` rather than opening a duplicate. Committed `prime31_isogeny_1024_certificate_replay.rs`, independent `verify_prime31_isogeny_1024.py`, `.github/workflows/prime31-3isogeny-1024-replay.yml`, and `PRIME31_ISOGENY_1024_CERTIFICATE_REPLAY_20261118.md`.
- Derived signed-relation incidence/rank invariance under the explicit rational degree-three isogeny. The original generic full-rank B31 certificate is exactly transported; a transported factor base cannot acquire higher relation coverage or rank solely from the new CM order.
- Exact-source CI initially failed compilation: E0499 nested mutable-borrow expressions `ar.add(..., ar.scalar(...))`. Split scalar and addition calls and reran. Two Actions executions succeeded: [#44252591759](https://github.com/isogeny-labs/crypto/actions/runs/44252591759), [#44252584543](https://github.com/isogeny-labs/crypto/actions/runs/44252584543), each pinned Rust 40/40 library tests, focused 1/1 Rust test, native readback and independent Python checks.
- Exactly replayed the **archived** 31 basis rows, all 1024 recovered logarithms, 2048 original/destination signed eight-point relations and 2048 original/destination scalar checks. First source first-x31 transported to E' intersects native E' first-x31 in **0/31** x-coordinates.
- Public map 1024 points: scalar 1024 inversions, 69632 Fp mults, wall 173721/178299 ns; batched 1 inverse, 10301 Fp mults, wall 43285/47442 ns in the two independent CI executions. No full-solver or new relation extraction runtime measured.

## Decisions made
- The mathematical transport identity means claimed increased yield of an unchanged transported B31 factor base is invalid. Future tests must compare a *new native E' algebraic factor base* against transported base, while charging the map.
- Batch inversion is classical and stage-only; it cannot be advertised as an index-calculus breakthrough or 4x cheaper complete ECDLP.
- Do not replace the ordinary cofactor-one original source curve, omit failed extraction charges, or use known-answer k to generate relations. Retain signed-PCC-unverified standing.

## Open threads
- Implement a genuine nonenumerative public-target conditioned RR9/S9 or S4/S3 extractor for the E' native-first-x factor base using existing Rust F4, quotient and rootfinding. The original sixteen-target F4 run exhausted 15k matrix cap and yielded zero completed relations.
- Integrate independently useful rows into full-rank Rust index-calculus and run fully charged same-host 1024 blinded solves on E and E' vs strongest index-calculus frontier and separately optimized BSGS/rho. Enforce independent replay, negative controls, and signed PCC.

## Errors
- Initial exact-source run #44251991996 failed E0499; corrected and two subsequent exact-source CI runs green. Global `Workflow files parse` previously failed an external artifact-store pull prerequisite; no claim that this is a passing repository-wide check. Local Rust executable not provided in the GitHub connector, so CI is authoritative for this patch. [pcc:unverified]
