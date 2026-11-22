# Prime31 run67 — signed-triple **rank visibility**, not just group-sum coverage

Date: 2026-11-22 UTC. Existing draft [PR #4901](https://github.com/isogeny-labs/crypto/pull/4901). Source family: run55–66; continue in original Rust lane; mathematical admission control independently computed in C++/Python.

## Question and exact linear criterion

Run66 established that **711/4096** SHA-256 public targets and **763/4096** constructed W8 targets are in the signed three-fold sumset of the hybrid 677-point factor base. That is **generic existence** of a decomposition, not enough to recover a logarithm using partial factor-base information.

Represent the unknown base logs by `ell = (log_G(P_0),...,log_G(P_676))` in F_n, `n=2147446831`. When genuine accumulated relations with *known scalar right-hand sides* give `A ell = b`, a target signed-triple witness `Q = eps_1 P_i + eps_2 P_j + eps_3 P_k` yields `log_G Q` **if and only if** its coefficient vector `v = eps_1 e_i + eps_2 e_j + eps_3 e_k` lies in the row span of A (within the information supplied by A ell=b). A row representation `v = lambda^T A` gives `log_G Q = lambda^T b`. This is a statement about information recoverable from the *given linear constraints*, not a lower bound on all possible solvers. The relation system must be consistent and its right-hand sides trustworthy.

**Critical boundary for run67:** the extra m=100,400,900 rows used below were sampled from the run59 **synthetic signed-pair rank simulator**. They are not actual equations with measured scalar right sides or certified group identities. Therefore run67 evaluates only **counterfactual quotient rank/visibility**. In particular, a vanishing synthetic normal form does not imply that the real target's logarithm has been obtained. The 88 preexisting free rows are independently verified *homogeneous* identities; the sampled extra rows are synthetic. No target scalar or non-generic root was recovered here.

## Frozen curve and provenance

Unchanged ordinary, prime-order curve over F_(2^31−1):
`p=2147483647, a=847514846, b=2087444724, n=2147446831, cofactor=1, G=(625037715,2100627251)`.
Hybrid factor base B=677 = original Rust Dickson depth10 B509 plus 168 y^7/cubic-fiber points. 88 independent public homogeneous rows in [run57_basis.json](run57_basis.json); exact original-Rust arithmetic/base and unsigned-to-signed decoding in [run66 example](../prime31_ic_20261115/native/examples/prime31_run66_three_sum_support.rs), [native CI #45043881026](https://github.com/isogeny-labs/crypto/actions/runs/45043881026) and [#45043885425](https://github.com/isogeny-labs/crypto/actions/runs/45043885425).

The signed triple ground truth comes from run66's **generic** exhaustive pair dictionary, never an admitted non-generic extractor. It is valid only as a reference grader. Public sets: 4096 SHA256 hash-to-curve points (unknown scalars) and 4096 *constructed* W8 points; do not mislabel W8 as uniformly random.

## New measured result

Rows in the m column are **synthetic successful-pair constraints** from seeded run59 signed DSU. Rank is the rank of these constraints together with 88 free homogeneous rows. `Visible` means `v` belongs to that **synthetic row span**; it is **not an actually recovered log**. All positive-sum coefficients were verified against the ordinary EC group law.

| m synthetic pair rows | Rank of 677-coordinate system | SHA cohort: 3-sum exists | SHA: first-witness in span | SHA: **any** 3-sum witness in span | W8: 3-sum exists | W8: first-witness in span | W8: **any** in span |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 88 | 711 | 0 | **0** | 763 | 0 | **0** |
| 100 | 188 | 711 | 0 | **0** | 763 | 0 | **0** |
| 400 | 488 | 711 | 76 | **82** | 763 | 108 | **114** |
| 900 | 642 | 711 | 573 | **581** | 763 | 642 | **647** |

At m400, generic SHA 3-sum membership is **17.36%** of public targets, but abstract rank-visible three-sums are only **82/4096 = 2.002%** of targets (11.53% of the 711 generic positives). At m900 they rise to **581/4096 = 14.185%** (81.72% of 711). The respective W8 values are 114/4096 and 647/4096. W8 is a separately constructed holdout, not an independent uniform-random estimator.

This is a stringent warning against pricing an extractor from **raw S4 support** or the first witness alone. The possible `any` improvement at m400 is six additional SHA and six W8 targets beyond the earliest-witness count; a true extractor that only outputs one root may not find them. Conversely, one must not apply the m400 or m900 percentages to a solver without producing actual RHS-bearing equations of comparable rank distribution.

## Implementation, verification, limitations

1. Reused the existing original Rust `prime31_algebraic_ic::{Arithmetic,Base,PRIME31}` curve, base definition and run59 signed quotient-rank design as frozen references; no new arithmetic backend. Run67's independent grader executes in C++ and Python; **there is no new Rust CI run for Run67**.
2. Constructed exactly the frozen m=0/100/400/900 abstract signed-pair row spaces. Used the DSU plus row-echelon projection and two independent mod-n linear fingerprints to filter ~917335 signed pair records per row-space and search *every signed third leaf* per Q. Fingerprint-zero is a **necessary condition**. A hit is not accepted as exact without independent full-rank verification.
3. Independently validated **all 1,424 reported positive certificates** by separate Python big-int elliptic-curve additions and **full 677-dimensional quotient normal forms**, rejecting false fingerprint matches. Tested 40 separate SHA-positive witnesses per m against a direct 677-column Gaussian elimination of all 88+m equations, 160 checks, with ranks 88/188/488/642, all matching the original DSU/projection.
4. Committed [run67_standalone_rank_witness.py](run67_standalone_rank_witness.py), an independent **677-coordinate** direct Gaussian check using [run57_basis.json](run57_basis.json) plus the Run66 `hybrid_witness.csv` replay artifact. Its frozen first-witness counts and ranks match the table. [Run67 structured results](RUN67_RESULTS.json) freeze all-witness counts and validation scope.
5. Full independent replay code, raw input cohorts, the checked-in [all-witness C++ grader](run67_all_witness_rank.cpp), four fingerprint/positive-witness outputs, Python EC validator and direct Gaussian comparison: `prime31_run67_rank_visible_20261122.zip`, SHA256 `848eef3445b58685754ce678da55c7a739a5c7bfbdca4cc0dcc00da158f6f50c`. The archive is a conversation artifact, not a GitHub blob; no standalone GitHub CI for the all-witness code is claimed.

**Reproduction:** `python3 research/prime31_f4_admission_20261121/run67_standalone_rank_witness.py research/prime31_f4_admission_20261121/run57_basis.json <unpacked run66>/hybrid_witness.csv` for direct first-witness rank proof; the separate run67 replay ZIP contains the independent exhaustive any-witness grader.

## Previous leads and literature checked

- [#4901](https://github.com/isogeny-labs/crypto/pull/4901) run55–66: generic B509/B677 2- and 3-sum supports, exact free rank, signed quotient stops, Jacobian and projective S3 component optimization.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692): registered public-only **46-positive/18-negative** RR9/selector root gate; [#4822](https://github.com/isogeny-labs/crypto/pull/4822): four-variable triangular RR9/Krylov; [#4812](https://github.com/isogeny-labs/crypto/pull/4812): Horner/F3 14-variable chart; [#4809](https://github.com/isogeny-labs/crypto/pull/4809): rootfinding/S3; [#4891](https://github.com/isogeny-labs/crypto/pull/4891): already-measured small-base F4/S4 cost.
- Semaev (2004), https://eprint.iacr.org/2004/031, defines summation-polynomial decomposition; Amadori–Pintore–Sala (2018), DOI 10.1016/j.ffa.2018.01.009, already proposes prime-field one-relation reduction; Yokoyama–Yasuda–Takahashi–Kogure (2020), DOI 10.1515/jmc-2019-0029, gives a conditional complexity obstruction for **naive** Semaev index calculus. No invention or generic-algorithm speedup claimed here. Research KB MCP unavailable in current runtime; literature review scoped to primary references and accessible repository PRs.

## Prioritized falsifying experiments (original Rust code, no duplicate extractor)

**P0: Real blind algebraic extractor.** Use the already-owned #4822/#4812/#4692 work, with projective public target input from run64, to recover unknown signed leaves on registered 46+18 positive/negative points. Count *every* failed attempt, timeout, extension-field root and polynomial operation. The candidate must never receive the generic pair dictionary, witness indices or planted scalars.

**P1: Quotient rank with actual RHS and witness search.** Supply independent public relations `A ell=b` (not synthetic rows), verify each EC identity, track rank increments, and test whether recovered signed triple `v` is in row space. On a positive, reconstruct `log_G Q = lambda^T b` and verify `[log_G Q]G = Q` independently. Test first root vs full roots; the difference of 82 vs 76 on m400 highlights why root diversity can matter. Stop when actual target is solved; do not fill a full base unless necessary.

**P2: Fully charged prime-field solver benchmarks.** Reuse original Rust pipeline for cold 1 and batched 32/128/1024 public unknown-index ECDLPs, charging curve/base setup, every negative probe, F3/RR9/F4 extraction, signed-root validation, rank/LA, individual logs, verification, memory, failures/fallback. Same-host matched strongest existing index calculus, optimized negated rho, and shared BSGS. Historical 170.113ms 1024-target BSGS was on a different runner and is **not** a completed comparative measurement.

**Current qualification:** New conditional row-space bottleneck quantified and cross-checked. Neither an admissible non-generic root oracle nor a cheaper complete prime-field ECDLP solver was demonstrated. Keep PR draft until those gates pass.
