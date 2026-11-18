# Binary relation baseline: target subtraction with batch inversion

Use `subtract_batched` as the native comparison baseline for complete two-point relation finding in the orientation/y-x fixtures. It replaces pair enumeration with target subtraction and full-point membership, and shares a single inversion across the ordinary and doubling denominators. This is an explicit control implementation of mechanisms already used in the repo. It is not an established wall-time record. [pcc:unverified]

Claim `PCC-20261115-c73d58` is **unverified**. All test outcomes and operation counts below are producer observations, with no signed independent verdict. [pcc:unverified]

## Contract and recommendation

Given a signed/labelled factor-base array F and supplied target T, return **every ordered index pair** `(i,j)` satisfying `F[i]+F[j]=T`. Distinct labels on equal points remain distinct. Results are in index order. No sign choice, relation, repeated label or exceptional branch is discarded. The index is rebuilt for every target: there is no pair-sum table or reusable cross-target setup.

For base size B and R output rows:

- `pair_scan`: B² group-addition calls, no membership index.
- `subtract`: B group-addition calls, B membership inserts and B lookups; one inversion per ordinary/doubling addition.
- `subtract_batched`: B group-addition calls, the same membership work and outputs, and at most one inversion per target. Identity/inverse/two-torsion cases require no inversion.

The derivation is `P+Q=T iff Q=T-P`. Full-point keys retain orientation. A prefix/suffix product computes inverses of every nonzero denominator with one field inversion and three multiplications per admitted denominator; zero entries are handled by their explicit group-law branch. Expected query work including index construction is O(B+R), versus O(B²+R) for pair enumeration; returned rows can themselves be quadratic with repeated labels. Hash/index and allocation costs are not field arithmetic. These are algorithmic statements and producer derivations, not wall-time measurements. [pcc:unverified]

For the prior `P1+P2+P3=O` polynomial convention, supply `T=-P3`. Full-point membership handles T=O and x=0 directly; a y/x chart is not needed for this baseline. Higher-arity decomposition and a whole IC pipeline remain outside this API.

## Native checks and exact operation accounting

```
cargo test --release --manifest-path research/orientation_baseline_20261115/Cargo.toml -- --nocapture
```

Nine tests pass: the six previous orientation/hybrid controls, fixed-field/batch-inverse controls, complete-output equivalence over all targets on 18 small curves (528 cases), and 32 GF(256) cases. The original pinned test source/log are unchanged. [pcc:unverified]

Small-curve fields have degrees 2,3,4; coefficients a in {0,1,z}, b in {1,z}. Bases include the whole group, non-negation-closed subsets with repeated labels, and the empty base. Targets include O and every finite point. GF(256) uses modulus `z^8+z^4+z^3+z+1`, a=0,b=1, base sizes 8,16,32,64 from lexicographic complete point enumeration, and eight deterministic targets including O and x=0. The fixed GF(32) field also has every inverse checked. Exact fixtures are in `baseline.rs`; no random sampling is used. [pcc:unverified]

The following raw counters include input point validation and the multiplication/squaring internals of exponentiation-based inversions. **Do not add the inversion column to the multiplication/squaring columns:** it would count those internals twice. Index insertion, lookup, output size and allocated scratch are separate from field arithmetic. XORs, hashing instructions, allocation bytes and machine cycles are not counted; this is not a calibrated total cost. [pcc:unverified]

| B | Arm | Group-add calls | Field mul | Field square | Inversion calls | Inserts | Lookups | Rows |
|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 8 | pair scan | 512 | 3213 | 2814 | 336 | 0 | 0 | 16 |
| 8 | scalar subtract | 64 | 612 | 502 | 47 | 64 | 64 | 16 |
| 8 | batched subtract | 64 | 473 | 222 | 7 | 64 | 64 | 16 |
| 16 | pair scan | 2048 | 15501 | 13694 | 1680 | 0 | 0 | 28 |
| 16 | scalar subtract | 128 | 1308 | 1078 | 103 | 128 | 128 | 28 |
| 16 | batched subtract | 128 | 945 | 406 | 7 | 128 | 128 | 28 |
| 32 | pair scan | 8192 | 67725 | 60030 | 7440 | 0 | 0 | 55 |
| 32 | scalar subtract | 256 | 2700 | 2230 | 215 | 256 | 256 | 55 |
| 32 | batched subtract | 256 | 1889 | 774 | 7 | 256 | 256 | 55 |
| 64 | pair scan | 32768 | 282765 | 251006 | 31248 | 0 | 0 | 165 |
| 64 | scalar subtract | 512 | 5475 | 4526 | 438 | 512 | 512 | 165 |
| 64 | batched subtract | 512 | 3772 | 1509 | 7 | 512 | 512 | 165 |

Each row aggregates eight targets; outcomes/counts are producer observations. [pcc:unverified]

At B=64 the subtraction arms use 64 times fewer group-add calls than quadratic enumeration; batching further removes 431 inversion calls while reducing both recorded field multiplication and squaring counts relative to scalar subtraction. None of these ratios is a wall-time speedup. [pcc:unverified]

Public code: [baseline.rs](baseline.rs). Raw native log: [unit-tests.log](unit-tests.log). Frozen protocol: [PROTOCOL.md](PROTOCOL.md). Diagram: [model.svg](model.svg). Report: [report.pdf](report.pdf), [source](report.tex).

The native [timing driver](bench.rs) and [configuration](TIMING_DRIVER.md) are ready. Its `--check` mode compares all four arms without opening a timing window; [driver-check.log](driver-check.log) records equal complete outputs for all four base sizes. Its unrun `--measure` mode has a warmup, five alternating rounds, 16 stream repetitions and a scalar A/A arm; it requires a build-time source pin. Source strings alone do not establish executable provenance or L2 admission. [pcc:unverified]

## Timing status and admission

**Timing is pending.** No timed benchmark or cloud job was submitted. No EKS MCP is loaded; after installing AWS CLI outside the repo, the actual `aws sts get-caller-identity` check failed with “Unable to locate credentials.” GitHub dispatch is available, but it does not supply current daily/monthly budget remainder and live pool-capacity readback. `taskq/deploy/eks-karpenter/COST_CONTROLS.md` says: “If the remainder is unknown or exhausted, do not start a new build/job.” The user was asked for the existing connection while implementation continued. [pcc:unverified]

The frozen follow-up requires EKS L2, a warmup, five alternating rounds, matching A/A and identical complete outputs. Cold validation, index construction, buffers and output materialization stay inside the measured boundary. Proposed ceiling: USD 5 including build/failures, 4 CPU/4 GiB, 900 seconds, zero retries. No pool/provider changes or local-timing substitute. An external baseline is required before a fastest-implementation claim. Setup reuse is secondary, with its amortization stated explicitly.

The API is a tiny-field fixture implementation; do not transplant its shift-based arithmetic as an optimized ECC2K backend. The production binary `FastCurve::add_many` and integrated root-finding oracles are the existing native comparison targets. A follow-up that integrates this contract should reuse those backends.

No library or leaderboard change is made. Canonical IC graphs are unchanged because this is a development baseline with no admitted wall-time or pipeline result; `model.svg` explains the new contract instead.

## Sources checked

- Base `9acb741c2` (PR 4327 merged), `research/orientation_yx_validation_20261115/`: preserve the orientation, repeated-x, chart and identity conditions.
- `docs/ic/ECDLP_RESEARCH_TECHNIQUES.md`, root-finding section: use target subtraction and exact membership as the two-point baseline; do not assign a wall-time benefit to a coordinate change alone.
- `src/cryptanalysis/ic_framework/plugins.rs`, `SubtractOracle`: existing generic target subtraction. `RootfindSubtractOracle` is prime-only; its implementation is not treated as a binary benchmark.
- `src/cryptanalysis/ic_framework/rootfind.rs`, PairedAdd and binary FastCurve batching: batch inversion already exists in the production binary backend. No novelty claim.
- Kosters–Yeo, [Notes on summation polynomials](https://arxiv.org/html/1503.08001v2), sections 2,4,5, read for the preceding model validation: distinguish algebraic-root semantics, rational orientation and solving cost.
- Sources checked (KB): unavailable, no loaded MCP and `scripts/research-kb status` exits 127. Fallback is the source-level prior-art review above, not an absence claim.
- Execution policy: `.agents/skills/isolab-eks/SKILL.md`, submission/access references, `taskq/deploy/eks-karpenter/{HARNESS.md,COST_CONTROLS.md}`. Budget/capacity readback remains an actual pre-submission dependency.
