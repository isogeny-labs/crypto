# Prime31 public-only exterior-4 RR9 candidate and B8 cost falsifier

2026-11-19 UTC. Draft [PR #4776](https://github.com/isogeny-labs/crypto/pull/4776). Preregistered [protocol](RR9_EXTERIOR4_PUBLIC_PROTOCOL_20261119.md), new [original-crate Rust candidate](../native/examples/prime31_rr9_exterior4.rs), [independent Python BigInt grader](../native/verify_prime31_rr9_exterior4.py) and [source-specific CI](../../../.github/workflows/prime31-rr9-exterior4.yml). **Source-specific CI PASS:** [run #44445676750](https://github.com/isogeny-labs/crypto/actions/runs/44445676750), source ef037d6c641ff69351f2714fee8ad2cbfc54ec7d, original Rust regression tests, focused algebraic identity tests and independent Python BigInt replay passed. [pcc:unverified]. Independent signed PCC pending.

## Algebraic method and exact identity

On the unchanged ordinary cofactor-one curve p=2147483647, E:y²=x³+847514846x+2087444724, prime order n=2147446831, G=(625037715,2100627251), reuse the original Prime31 Rust Arithmetic at pinned source 1df714645aa683cf975bc76fcacb9842cb4780fd. The public R=(r,s) gives for any signed support leaf Q=(x,y) the row

    v_R(Q) = [(x-r)*x^i : i=0..3] || [(y+s)*x^i : i=0..3]

For four selected leaves from slots 0–3, take the 70 4x4 minors indexed by four-subsets I of the eight columns. For slots 4–7, take the complementary minors with sign (-1)^sum(I). By generalized Laplace expansion the contraction of those 70-coordinate exterior-four vectors equals **exactly** the target-conditioned eight-row determinant. Given pairwise distinct finite leaves and -R, the existing RR9 divisor criterion says that determinant is zero exactly when the eight signed points sum to R. Duplicate rows and leaves equal to -R are rejected before emitting a candidate.

The new Rust extractor takes public R and publicly constructed signed supports, and performs **no elliptic-curve addition and no generic EC half-sum lookup inside its extraction function**. A separate grader uses ordinary Rust EC arithmetic to replay the signed witness. This is an algebraic non-generic *candidate interface*, but its exhaustive pair search is **not** a fast nonenumerative solver. The mathematical Laplace identity is classical, not a novelty claim.

## Exact cost obstruction for this specific algorithm

| Signed base B, eight slots | Half tuples | All pair contractions | Coordinate-product Fp multiplications |
|---|---:|---:|---:|
| B4 | 4,096 | 16,777,216 | 1,174,405,120 |
| B8 | 65,536 | 4,294,967,296 | 300,647,710,720 |

Each tested pair contracts 70 field-coordinate products, plus modular additions. These counts **exclude** the 70 half-minor constructions per tuple, inversions inside their Gaussian eliminations, point setup and final verification. An early successful stop reduces the actual pair count; a complete B8 no-hit query must check all 4,294,967,296 pairs.

1,024 full B8 queries would require **307,863,255,777,280 coordinate products**, about **283,988 times** the 1,084,071,298 total field multiplications in the 1,024-log original-affine signed shared BSGS control from [#4774](https://github.com/isogeny-labs/crypto/pull/4774). These are different work categories, not a measured elapsed-time ratio, and not a lower bound on other algebraic/F4 algorithms. This exact-work barrier means do **not** launch a naive exhaustive B8 exterior contraction.

## Registered B4 verification

- Positive fixture: the sum of the first publicly available signed support in each of the eight slots. The extractor receives only the resultant public target, no planted labels.
- Unplanted target: original G; its positive/negative truth must be certified independently and the complete 16,777,216-pair space traversed if it has no witness.
- Unit tests: field Mersenne multiplication/inversion and 70-minor generalized Laplace identity against independent 8x8 Gaussian determinant on synthetic matrices.
- Separate Python BigInt implementation: reconstruct canonical x=0,1,2,3 base, signed weighted supports, exact EC group truth and replay, all emitted witness identities and exact coordinate-product counts. Do not include the independent group grader in candidate timing.
- Exact-source GitHub workflow overlays only the candidate example on the pinned original Rust crate, runs original library tests and focused tests, then produces two-case JSON and independently replays it.

Source-specific measurements are now included below; this original-curve B4 diagnostic is **not** a complete ECDLP benchmark.

## Prior leads, novelty scope and next experiments

Reviewed [#4466](https://github.com/isogeny-labs/crypto/pull/4466) (RR9 and Hermite duplicate handling), [#4728](https://github.com/isogeny-labs/crypto/pull/4728) (supplied-index determinant/Fourier/zeta, signed B8 oracle and four B8 windowed rank31 with generic tables), [#4692](https://github.com/isogeny-labs/crypto/pull/4692) (owner of unknown-index F4/XL extractor), [#4772](https://github.com/isogeny-labs/crypto/pull/4772) (public norm-Jacobian and fixed-support bound), [#4773](https://github.com/isogeny-labs/crypto/pull/4773) (conditional prefix2), [#4770](https://github.com/isogeny-labs/crypto/pull/4770) (ternary generic grader) and [#4774](https://github.com/isogeny-labs/crypto/pull/4774) (signed matched shared BSGS 3,342.793ms versus ternary W8 generic 7,834.933ms for 1,024 original-curve logs). Autoresearcher SA-04 RR norm and SA-18 determinant experiments were negative under their criteria; cryptanalysis has separate binary SAT and prime-field Semaev reference engines. Bounded exact GitHub code searches for exterior4/Laplace RR9/orthogonal wedge found no duplicate; the research KB MCP/CLI was unavailable, so this is not an exhaustive novelty claim.

Published prime-field index calculus: Petit–Kosters–Messeng (PKC 2016), DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala (FFA 2018), DOI 10.1016/j.ffa.2018.01.009. The asymptotic 2026 Kaarelson candidate is separately being audited in [#4704](https://github.com/isogeny-labs/crypto/pull/4704), and does not provide a practical replacement here.

Next mathematical target: a genuinely **subquadratic zero-inner-product search** over the 70-coordinate signed half tables, or a bounded F4/SAT elimination that outputs signed indices without examining most pairs. If that succeeds on sealed 46-positive/18-negative B8 natural targets, replace only the relation oracle in the existing Rust B8-window rank31/1,024-log system, charging setup, failed shifts, LA, extraction and verification. Compare cold-one-target, 64 and 1,024 public logs on a single host against optimized index calculus, shared signed BSGS and optimized negation-map rho. Extend to ordinary prime-field ladder before any scaling claims.

**Standing:** mathematical factorization, reproducible source-bound candidate and a strong work-budget falsifier. **No cheaper complete non-generic prime-field ECDLP solver has been established.**

## Local independent Python arithmetic preflight (not source-exact native CI)

Frozen [preflight JSON](RR9_EXTERIOR4_B4_PREFLIGHT_20261119.json). A separately implemented Python BigInt EC group enumerator reconstructed base points (0,309380359), (1,749117756), (2,51658786), (3,166645833), and independently counted 3,921 unique sums in each 4-slot B4 half. With the original eight weights:

- Planted B4 target R=(1991076510,718644930) admits all-zero slot labels; both direct eight-row determinant and 70-term exterior contraction are exactly zero.
- Unplanted target R=G=(625037715,2100627251) admits **no** B4 signed eight-slot word, by exhaustive independent 4+4 *grader* enumeration. The all-zero word's determinant and exterior contraction both equal 735,647,190, nonzero.
- A different nonmatching signed word (0,1,2,3,4,5,6,7) at R=G gives determinant and contraction both 468,030,532.

This confirms exact planted/negative fixture labels and the Laplace identity on three original-curve examples. It does **not** establish a native Rust result or wall-time: the new Rust public-only enumerator must still complete the source-bound CI. The Python truth grader is excluded from the native candidate interface.

## Source-bound original-curve Rust result — CI PASS, 2026-11-19 UTC

[Passing source-specific GitHub Actions #44445676750](https://github.com/isogeny-labs/crypto/actions/runs/44445676750) ran original Rust arithmetic pinned to commit 1df714645aa683cf975bc76fcacb9842cb4780fd, candidate source ef037d6c641ff69351f2714fee8ad2cbfc54ec7d, Rust 1.100.0, and an independent Python BigInt witness/truth checker. All CI steps passed, including original 40 Rust regression tests and focused modular-field/Laplace determinant unit tests. Signed independent PCC remains [pcc:unverified].

| Original B4 public-only algebraic candidate | Planted positive | Unplanted natural R=G |
|---|---:|---:|
| Correct EC truth (independent grader) | witness exists | no witness |
| Candidate outcome | witness [0,0,0,0,0,0,0,0] | no witness after entire scan |
| Algebraic half-pair contractions | 1 | 16,777,216 |
| Field coordinate products in contractions | 70 | 1,174,405,120 |
| Candidate elapsed including 4x4 minor table build | **175.178 ms** | **1,295.600 ms** |
| Public-only; generic EC sum tables inside candidate | yes; **none** | yes; **none** |

Public support setup was separately 0.034 ms, and the independent grading was excluded from the candidate elapsed times. The positive is a **planted first-pair control**, so 175 ms is not a natural-hit median. The negative is an actual complete exhaustive no-hit B4 scan. The observed outcomes exactly match independent original-curve Python BigInt truth enumeration. These are **one-run diagnostic times**, not paired cost comparisons or full ECDLP timing.

The frozen [durable exact-source JSON summary](RR9_EXTERIOR4_B4_CI_SUMMARY_20261119.json) preserves the source bound and counters. [Complete 758-byte raw GitHub artifact #14493406334](https://github.com/isogeny-labs/crypto/actions/runs/44445676750/artifacts/14493406334) has GitHub SHA-256 digest 8399b320128ebe7e8e2f931111d25c3abd72f4af03b48c23bcca5d3eea745640 and is retained until approximately 2026-12-19.

**Decision:** This public-only exterior contraction is a correctly checked algebraic relation recognizer at B4 but is an uncompetitive *exhaustive* architecture for B8 without an independently validated subquadratic orthogonal-vector or F4/SAT extraction breakthrough. No complete B8 extraction, rank-31 non-generic learning, 1,024 non-generic logs or advantage over shared BSGS/rho has been shown.
