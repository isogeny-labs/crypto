# EXP-R31-APS-JOINT64 — preregistered prime31 public-source Shamir/batch test

Date: 2026-11-16 UTC. This protocol is committed **before any implementation or benchmark** in the existing Rust APS example, with unchanged historical evidence. Development-only `[pcc:unverified]`. No leaderboard promotion, paid EKS, source-independent validation or security claim.

## Reuse / literature-first and negative lead review

- Existing open crypto #4391, `research/prime31_ic_20261115/native/examples/prime31_aps_one_relation.rs`: ordinary cofactor-one E over p = 2147483647, n = 2147446831, existing 64-jump labelled source, Jacobian mixed-add plus batched normalization, Mersenne S3 quadratic coordinate root oracle, [384,384,16384] partitions and packed x index. Preserve its original implementation as control.
- `research/ic_e222_m221_20261106/RESULTS_shamir.md`: **joint Jacobian Shamir** for generating `aG+bQ` is **already prior internal engineering work**. An initial affine Shamir variant lost; the Jacobian version won on its different small-field table collector. This experiment **does not claim novel Shamir arithmetic**; tests a different bottleneck: the 64 public walk jump points of the complete prime31 APS solver, normalized *together in one batch*.
- Existing `APS_PACKED_X_INDEX_RESULT_20261116.md`: 24 matched complete APS sources, packed 110.408323 ms vs fresh cold BSGS 108.326084 ms, 96/96 solves in CI #43890868003. The earlier 10% packed improvement gate failed; do not change it after observing this experiment.
- Existing `M8_SYMMETRIC_ROOT_RESULT_20261116.md`, #4228, #4349, #4373, `ANCHOR2_M6_F4_CI_REPORT_20261116.md`: given-f S9 signed-root correctness is not target-conditioned extraction. Anchored m6 has a 3.7407% natural-support ceiling; F4 calls censored. Nonenumerative S9 is a **separate** high-priority scientific lead, not a reason to reinterpret a generic S3 speedup.
- Primary published prior art: Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, FFA 2018, DOI 10.1016/j.ffa.2018.01.009. The tested m3 APS relation search remains generic pair-MITM-equivalent and is not a novel index-calculus asymptotic result.
- KB status/search unavailable: research-KB MCP tools and local repository wrapper are not exposed in the connector-only runtime. Above current GitHub files and publisher primary references are the bounded fallback. Conductor and local cargo unavailable here; exact-source Rust CI is mandatory before any execution claim.

## Mechanism / invariants

The existing public additive walk precomputes 64 jump points `J_i = a_i G + b_i Q` by **two independent affine scalar multiplications plus a group sum per jump**. New opt-in source computes each jump as a *joint left-to-right binary interleaved* Jacobian double-scalar multiplication, reusing public `G`, `Q` and `G+Q`, and then **normalizes all 64 projective jump points with a single Montgomery batch inversion**. It changes neither the PRNG draws, labels, accepted step points, subsequent walk, factorized S3 algebraic oracle nor signed scalar recovery. Degenerate `G+Q=O` must be handled and identity jumps discarded exactly as in the control. Compare the source's full `(x,y,a,b)` tuples, identity/duplicate counts, full ordered signed relation and S3 query counts; not just final recovered logarithms.

This is an optimization of a published APS-style one-relation complete Rust solver, **not** original non-generic elimination. The candidate must never use caller-known scalar or generic group join in algebraic candidate discovery.

## Frozen measurements / correctness gate

- Fixed **24 previously unused public targets** from SplitMix seed `0x4a53_3343_4f4c_4431`; draw k modulo n and construct public Q in the *caller only*. Reject k = 0,1 and duplicate k. The internal solvers receive Q, never k.
- Four fully cold in-process Rust release arms per Q, execution order rotated by case index: (0) current packed APS, (1) joint-Jacobian/batched jump APS, (2) joint APS independent A/A repeat, (3) fresh-baby-table batched-Jacobian BSGS. All under identical curve, profile [384,384,16384], max six complete APS rounds and one BSGS solve.
- Reuse source construction, S3 and independent BSGS Rust code, tests and locked Rust 1.100.0 CI. No refactoring of the old arm's costs. **All** field setup, jump precomputation, source generation, indexing, failed rounds, S3 root search and signed scalar verification are charged; BSGS has its own all-new baby table each time.
- **Mandatory correctness**: 96/96 complete recovered scalars match the caller-known values and independently pass [k]G=Q; original/new APS attempt counts, public-source point/label records in unit tests, S3 pair counts, signed solution witnesses, accepted/missed rounds and root match counts agree exactly. A discrepancy fails; preserve the first counterexample.
- **Predeclared win gate**: candidate aggregate full cold <= 0.90 × original packed, >= 18/24 paired wins, A/A aggregate drift <= 0.05. Compare candidate/BSGS and individual wins regardless of gate verdict. Explicitly mark a miss or a loss; do not cherry-pick.
- Store untouched original program JSON, source/workflow commit SHA, job/run URLs, compiler/tests, per-target raw wall times, attempts and complete witnesses. Do not treat CI output as PCC signature, make no external-CAS/rho-fastest claim and do not promote leaderboard.

## Decision tree

If correct and faster: retain the joint batch opt-in; independently replay, try a second disjoint panel and standard same-host external PKM/APS/rho comparators. If correct but slower: keep a negative report with overhead by phase. If any correctness mismatch: debug on frozen cases before any timing claim. **Separate non-generic line:** implement original-ideal-equivalent target-conditioned symmetric S9 K/S5 extraction on B4/B8, then B31 and full cold ordinary prime31 logs, not enumeration of candidate multisets.
