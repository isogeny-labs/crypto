# PR #4979 integration audit: exact support and cancellation

Standing: independent written mathematical and integration review, **[pcc:unverified]**. No cryptographic computation, symbolic execution, native test, benchmark or new subgroup certification was performed. Only read-only GitHub access and local administrative file/hash operations were used. This review does not promote either PR's scientific standing.

## Source-bound decision

The mathematical identities and the stated 338,272 support cap are valid under their explicit assumptions. The count is conservative: cancellation-equivalent signed multisets give the stronger cap **309,442**. The factorization adds a useful support representation and explicit multiplicity bookkeeping to the already established resultant degree, without improving an asymptotic exponent. Integrate this as a separately attributed review and retain the native-validation gap.

| Source | Immutable identity |
| --- | --- |
| [PR #4979](https://github.com/isogeny-labs/crypto/pull/4979), open draft and unmerged at observation | head `1a9f3e3c6966258f1a94068d95ec3f3e337b7196` |
| Common base | `816f9c3a942efed569049618c056d12a0c56d693` |
| [#4979 mathematical note](https://github.com/isogeny-labs/crypto/blob/1a9f3e3c6966258f1a94068d95ec3f3e337b7196/research/prime31_summation_compression_20261122/iteration10/DOUBLE_RESULTANT_AND_SUPPORT.md) | blob `db62fd8a58435c9b103d767dc6a86d9da58e5ef7` |
| [#4979 STATE](https://github.com/isogeny-labs/crypto/blob/1a9f3e3c6966258f1a94068d95ec3f3e337b7196/research/prime31_summation_compression_20261122/iteration10/STATE.json) | blob `4562f2591fecff7d89739d3c831e262d5f8d2bdc` |
| [#4979 validation provenance](https://github.com/isogeny-labs/crypto/blob/1a9f3e3c6966258f1a94068d95ec3f3e337b7196/research/prime31_summation_compression_20261122/iteration10/VALIDATION.md) | blob `000456f3e6052eb1d3681a468f8e3e379443b89e` |
| [#4977 frozen S3 pencil note](https://github.com/isogeny-labs/crypto/blob/d6e1d03aff64ff168bd1d1d565a08c8f5032ef62/research/prime31_summation_compression_20261122/iteration10/S3_PENCIL_CONTRACTION.md) | blob `da12c7a459405f4fdea44118cc03ade721d5eb90`; SHA-256 `9e9e64e76208202d05957794e7bd9371f9e427b2a5dded9f88aceb3ca40588ed` |

The compared #4977 head was `d6e1d03aff64ff168bd1d1d565a08c8f5032ef62`. Exact fetched source bytes were retained under `sources/` and checked against all five Git blob identities. #4979's API `merge_commit_sha=87c8dd4eb681ecc71ffde73377756b57f2d37cf6` is a synthetic merge identity while `merged=false` and `merged_at=null`; it is not evidence of a completed merge. Complete PR metadata and changed-file identities are in `metadata.json`.

## Overlap and integration conflicts

#4979 changes seven files: it appends `docs/ACTIVE.md` and `docs/session-logs/2026-11-23.md`, and adds `iteration10/DOUBLE_RESULTANT_AND_SUPPORT.md`, `REPORT.md`, `RESULTANT_GATES.svg`, `STATE.json` and `VALIDATION.md`. No Rust, workflow, PCC or root research STATE file is changed by that PR.

There are three shared changed paths with #4977. The ACTIVE and dated log entries carry distinct work and should both be retained. Both PRs add **different contents at `iteration10/STATE.json`**, so the work is not path-disjoint as advertised. Individual mergeability against the current main does not resolve that add/add collision.

The parent has selected a safe integration: rename **only #4977's unfrozen metadata** to `iteration10/PENCIL_FAMILY_STATE.json`, update its metadata references, and leave #4979's source-bound STATE path available. Link both reviewed immutable heads from the parent state. Preserve every frozen mathematical source and its recorded hash. This review performs none of those mutations.

## What the double resultant adds

| Claim in #4979 | Relationship to the frozen pencil note | Review |
| --- | --- | --- |
| The diagonal S3 factor is linear with leading coefficient `-4f(u)` | Already used in pencil Section 4 | Correct; excludes zero ordinates and characteristic 2 |
| `deg Delta = 2D²-D` with nonzero leading coefficient | Already explicit in pencil Section 4 | Same exact formal degree, no new complexity result |
| `Delta = diag * off²` by symmetry | Explicit multiplicity bookkeeping absent from the pencil note | Correct; nonzero leading scalars retained |
| `Q = H * diag * off`, degree `D²+D`, exact nonidentity four-sum support | A shorter scalar support representative | Correct; includes the zero-pair branch through H; labels and replay still required |
| Rank at most three | Already pencil Sections 2–3 | Correct; low ordinary rank alone does not compute the componentwise product |
| `det M = 0` for every target once `D >= 4` | Explicit negative control for a shortcut the pencil note did not propose | Correct; a missed target makes every entry nonzero nevertheless |

The factorization follows directly from `S3(u,v,r)=S3(v,u,r)` and the product over all ordered pairs of distinct roots of monic H. Replacing each forced off-diagonal square by one copy preserves vanishing support. It does **not** make Q squarefree: different group sums, H factors and diagonal factors can have repeated roots. Thus `D²+D+1` is the allocation for that exact dense polynomial, not a minimal support-polynomial degree or a lower bound on arbitrary indexes. All of these refinements remain quadratic in D in the stated materialized formats. [pcc:unverified]

The support proof handles a zero two-leaf subtotal by H and nonzero subtotals by the actual sign-closed rational pair spectrum. For `i=j`, the affine diagonal factor gives the doubling point; opposite equal subtotals sum to O and belong to the separate identity branch. For `i != j`, distinct x-values and nonzero ordinates make both rational addition roots well-defined. Global sign closure recovers the requested R rather than only its x-coordinate. These are set-support statements, not assertions of complete ordered outputs or relation multiplicity. [pcc:unverified]

Under the inherited B=31 assumptions, injectivity of doubling in odd-order G gives D>=31. The support bound below is much smaller than the inherited nonidentity x-domain, so missed targets exist. On any such target, every matrix entry is nonzero while the determinant vanishes. This is a proof of existence of a false-positive input, not an executed native fixture.

## Audit of the published 338,272 cap

Let `A={+P_i,-P_i : 1<=i<=B}` consist of exactly 2B distinct nonzero points in an odd-order subgroup G. Repeated leaves are allowed. An unordered four-leaf multiset is specified by nonnegative multiplicities `m_i+` and `m_i-` summing to four, so there are

\[
M=\binom{2B+3}{4}
\]

such multisets. Negation swaps the two multiplicities at every index. Its fixed multisets satisfy `m_i+=m_i-` for every i, and hence have group sum O. They are exactly multisets of two cancelling pairs, with count

\[
F=\binom{B+1}{2}.
\]

All other negation orbits have two elements and yield at most one nonidentity x-value. Some can still sum to O because of group relations, and distinct orbits can collide; neither phenomenon invalidates an upper bound. Therefore `(M-F)/2` is valid. Parity is guaranteed by the size-two orbit partition. Repetitions such as three copies of one signed point are included in M. At B=31, M=677,040 and F=496, yielding 338,272. [pcc:unverified]

## Stronger bound after exact cancellation

Set `c_i=m_i+-m_i-`. The group sum is `sum_i c_i P_i`, and

\[
4-\lVert c\rVert_1=2\sum_i\min(m_i^+,m_i^-).
\]

Consequently the only possible reduced norms are 0, 2 and 4. Norm zero contributes O. Every norm-two integer vector can be realized by four leaves by adding one cancelling pair, and every norm-four vector already uses four leaves. Thus counting nonzero coefficient vectors of norms two and four, up to `c -> -c`, removes cancellation duplicates before any group collisions. This is a combinatorial reduction, not a cryptographic execution. [pcc:unverified]

For norm two, the magnitude partitions `2` and `1+1` give

\[
N_2=2B+4\binom B2=2B^2.
\]

For norm four the complete partition count is:

| Absolute-value partition | Signed coefficient vectors |
| --- | ---: |
| `4` | `2B` |
| `3+1` | `4B(B-1)` |
| `2+2` | `4 binom(B,2)` |
| `2+1+1` | `8B binom(B-1,2)` |
| `1+1+1+1` | `16 binom(B,4)` |

Signs are chosen independently on each nonzero coordinate. The coefficient 3 distinguishes the two coordinates in `3+1`; the coefficient 2 distinguishes one coordinate in `2+1+1`. Repeated leaf labels therefore remain fully represented. Summing and simplifying gives

\[
N_4=2B+12\binom B2+24\binom B3+16\binom B4
    =\frac{2B^4+4B^2}{3}.
\]

Every nonzero integer vector has a distinct negative. They map to opposite group points; any group identity images or additional collisions only decrease nonidentity support. Hence

\[
|x((A+A+A+A)\setminus\{O\})|
\leq \min\!\left(\frac{n-1}{2},\frac{N_2+N_4}{2}\right)
=\min\!\left(\frac{n-1}{2},\frac{B^4+5B^2}{3}\right).
\]

For B=31, `N2=1,922`, `N4=616,962`, and the cap is **309,442**. The published cap exceeds this by `B²(B-1)=28,830`: each norm-two vector has B distinct four-multiset realizations obtained by adding one cancelling pair, but only one reduced coefficient vector. No assumption about genericity, statistical independence or useful relation rank enters this count. [pcc:unverified]

Using the **inherited, unrecounted** order `n=2,147,446,831`, the nonidentity x-domain has `1,073,723,415` elements. At least **1,073,413,973** are outside the four-sum support. For uniform nonidentity point targets, equivalently uniform nonidentity x-pairs in this odd-order group, the miss probability is at least

\[
1-\frac{309442}{1073723415}.
\]

These are exact upper/lower bounds conditional on the specified base and inherited order. They are neither a measured support census nor a factor-base density, runtime, useful-row-rank or cold-ECDLP result. The improvement removes lower-order terms from a Theta(B^4) count and does not change the exponent barrier. [pcc:unverified]

## Remaining gaps and safe continuation

1. #4979's three residuals are the producer's same-session SymPy outputs. The source text and failed first attempt are retained as provenance; they were not rerun here and are not native Rust validation, independent executable replay, formal proof or signed PCC evidence.
2. The actual Prime31 source group order, actual pair-spectrum D, actual four-sum support and all signed ordered-output behavior remain inherited or unmeasured in this audit. The support proof does not certify any of them computationally.
3. Full native handling of O, cancellations, repeated leaves, collision buckets, rational-root recovery, all misses and signed replay remains open. A frozen Rust counterexample should distinguish a zero determinant from zero entries; no such executed fixture is asserted.
4. The 2/3 and 4/7 exponents remain architecture-specific conditional budgets. Eligible asymptotic factor-base families and independent-row yield must still be established. Neither the support refinement nor the removed square multiplicity supplies a faster exact query index. [pcc:unverified]
5. Integrate the mathematical overlap and this stronger combinatorial bound by reference. Preserve both original notes and producer validation provenance. Use separate metadata paths, retain both ACTIVE/session entries and link immutable reviewed heads. Do not overwrite PCC, substitute producer symbolic checks for native evidence, or infer novelty or asymptotic success.

The parent independently rederived the `N2` and `N4` formulas before this review was finalized. This is additional written checking only; the standing remains **[pcc:unverified]**.
