# EXP-R31-G — S4 triple-arm elimination into explicit rational S3 fibres (2026-11-15)

**Standing: [pcc:unverified].** This is an algebraic *component* on the inherited ordinary prime-order `p=2^31-1` curve. It is neither a newly extracted target-dependent IC relation nor a complete prime31 discrete-logarithm solver. The new Rust example and independent-language replay must be run at the exact final GitHub head before native execution is claimed.

## Why this was chosen (prior-lead check before editing)

The latest #4188 CI [43725436615](https://github.com/isogeny-labs/crypto/actions/runs/43725436615) reduced each fixed three-point S4 side to **only three depth-six Dickson towers plus one free coordinate**. Each independent S4 side nonetheless stopped at F4 degree ten and 14,129 partial multiply-adds under the registered 15,000-entry dense-reducer cap. Removing unrelated blocks is not the cure. Earlier EXP-R31-F eliminated the fixed pair S3 algebraically into 961 roots (930 admissible distinct-index roots), but the remaining target-dependent S4 still oversized. The new test asks whether **the S4 side itself admits a cheap exact finite-rational projection using S3**, without pretending that a fixed-stage projection recovers the target scalar.

Scope of prior work checked:

- [#4077](https://github.com/isogeny-labs/crypto/pull/4077): whole-call useful independent-rank gate; stage root count is not rank, a speedup or a new complexity exponent.
- [#4275](https://github.com/isogeny-labs/crypto/pull/4275) and [#4091](https://github.com/isogeny-labs/crypto/pull/4091): dense vs tower PKM F4, including previous planted-positive versus naturally sampled distinction.
- [#4228](https://github.com/isogeny-labs/crypto/pull/4228): certified quotient and subresultant construction, stronger prospective competitor than an unfinished S4 F4 basis.
- [#4188](https://github.com/isogeny-labs/crypto/pull/4188): full prime31 curve/cost baseline, m3 projective row collisions, all m8 negative/censored Rust F4 controls and recorded fixed-pair elimination.
- [#3877](https://github.com/isogeny-labs/crypto/pull/3877): auxiliary/chained DLP transfers still lack a certified scalar-free complete cheaper transfer.
- [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536): an existing optimized Rust auxiliary-map IC implementation with its own public-target and amortized controls; no qualified same-curve cold win transferred here.
- [crypto-autoresearcher #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305): cross-repo lead atlas and "do not reopen without a falsifier" restrictions.
- Petit–Kosters–Messeng, *Algebraic Approaches for the Elliptic Curve Discrete Logarithm Problem over Prime Fields*, PKC 2016, [DOI](https://doi.org/10.1007/978-3-662-49387-8_1); Diem, *On the discrete logarithm problem in elliptic curves*, Compositio 2011, [DOI](https://doi.org/10.1112/S0010437X10005075). Semaev polynomial root decomposition and batched inversion are established methods, **not claimed new**.

The research-KB MCP server and its repository wrapper, Conductor, EKS admission, and independent PCC signer were unavailable in this execution environment. This is a bounded primary-repo/literature review, not proof of novelty.

## Mathematical projection and finite-instance certificate

Same publicly fixed curve, without replacing it by a smooth-order toy:

`p=2147483647, a=847514846, b=2087444724, n=2147446831`,
`G=(625037715,2100627251)`.

The deterministic depth-six Dickson fibre has 64 rational x roots and **31** canonical curve points `P_i=(x_i,y_i)`. For all `i<j<k`, first solve

`S_3(x_i,x_j,w)=0`

by the exact quadratic in `w`, then for both of its roots solve

`S_3(w,x_k,u)=0`

by another quadratic. Every root `u` represents an x-coordinate of a signed three-point sum, hence satisfies `S_4(x_i,x_j,x_k,u)=0`. Conversely, for nonexceptional finite points the four sign classes modulo global negation exhaust the four `F_p` roots of this specialised S4; the code explicitly fails on a diagonal leading coefficient, repeated root, nonresidue, infinity or mismatched group semantic check. **This equivalence is restricted to the actual accepted base and these distinct-index triples.** It does not claim an isomorphism of complete polynomial ideals, particularly with nonrational or nonliftable points.

The exact producer-local Python arithmetic census returned:

| Domain/count | Result |
|---|---:|
| Canonical factor-base points | 31 |
| Distinct index triples | `C(31,3)=4495` |
| Initial two-point S3 quadratics | `C(31,2)=465` |
| Remaining two-root S3 calls | `2*C(31,3)=8990` |
| Total quadratic solves | **9,455** |
| Finite S4 abscissa outputs | **17,980** |
| Distinct finite abscissae | **17,980** |
| Across-triple collisions | **0** |
| Point-addition semantic comparisons | **17,980** |
| Sorted u32 little-endian roots SHA-256 | `a71a99940794aead48642690575a37be72ef9994a93c9bbace47b538307d126d` |
| Sorted roots FNV-1a (little-endian u32) | `cc15d719e3eaf8c2` |

Because all 17,980 `F_p` roots are distinct, the **squarefree rational triple-arm projection polynomial**

`K_3(U)=prod_{i<j<k} prod_{u in Roots_Fp(S4(x_i,x_j,x_k,U))}(U-u)`

has *exactly degree 17,980*. It is an explicit finite-instance result, and any materialized polynomial with this complete rational zero set (and no others) has degree at least 17,980. This **does not give** a lower bound of 17,980 on F4 matrix entries, on arbitrary succinct/circuit representations, or on target-dependent solving complexity. Coefficients of `K_3` are intentionally **not** constructed quadratically; the finite root representation is used.

## Concrete Rust optimization

[Native S4 example](examples/prime31_s4_triple_projection.rs) reuses the existing `prime31_algebraic_ic::{Arithmetic,Base,PRIME31}`. All **root discovery** uses only finite-field S3 coefficient/discriminant calculations, two batch inversions and rational square roots; group additions validate the finished roots afterward and are excluded from the discovery timer. Both S3 batches are guarded against diagonal/zero denominators, merged roots and nonsquares. The same Rust example separately performs **all 9,455** scalar-inversion quadratic calls and demands exact equality. Thus replacing per-call inverse exponentiation by two Montgomery batch inversions cuts the count of expensive field inversions from 9,455 to **2**, while introducing batch multiplications and storage. This is **not** a 4,727.5x runtime speedup: sqrt exponentiations, field operations, setup and memory remain.

[Standalone Python replay](replay_s4_triple_projection.py) independently implements the finite field and elliptic group law and fail-closed checks the sorted-root SHA-256, all group-semantic x-values and distinctness. It is a producer-local check, not a signed independent scientific replay. The focused existing GitHub Actions Rust workflow now invokes both on push.

## Experiment decision and next falsifiers

1. **Gate A: source/CI reproducibility.** Exact-head Rust compile, full roots, hash, all 17,980 post-hoc EC semantic checks, and stage operation counts. Retain failures. The new CI step is a development test, not an admitted cold benchmark. Record actual Rust stage diagnostic times separately for batched vs scalar inversions; run a matched A/A experiment before treating clock differences as an optimization.
2. **Gate B: prove the S4-coordinate quotient at larger depths.** Increase factor base to B=63/135 without materializing a giant polynomial; check x-collisions, zero/infinity exclusions, memory and `K_3` rational degree. Rank-useful relation count, not abscissa cardinality, is the correct lead metric. Higher B may create collisions, so the present uniqueness result must not be extrapolated.
3. **Gate C: target-dependent algebraic extraction.** Combine the two `K_3` side projections with the degree-930 admissible pair projection and the central `S4(u,v,w,x_R)` via a certified quotient/subresultant or polynomial modular method (#4228), instead of writing a generic hash join or enumerating `K_3×K_3×H_2`. Freeze a small preregistered panel of natural positives/misses; count *all* finite-field operation work, root-lifts, censored calls, projective-unique relations and independent ranks. Compare against mature CAS exact-output controls, not just our capped F4.
4. **Gate D: complete solver and cost.** Only after Gate C yields genuine extracted natural relations: Rust sparse relation accumulation and linear algebra, unknown public Q recovery (point check), setup, all failures, full cold wall/GAE/memory, and matched optimized rho and BSGS on the identical ordinary prime-order curve. Include heldouts, independent validation and growth checks. **Do not promote a leaderboard or claim a cheaper non-generic ECDLP solver from this component.**

**Classification:** The S4/S3 finite-field projection is algebraic preprocessing. A subsequent generic x-table join or group enumeration would belong to the repository's **generic decomposition** track, even if fed by this algebraic component. Only a true target-dependent polynomial solver with measured complete costs can satisfy the headline algebraic track. [pcc:unverified]


## Exact-source Rust development CI result (source `9074471be1ad`, 2026-11-15)

[Successful CI run 43730061804](https://github.com/isogeny-labs/crypto/actions/runs/43730061804), job `132860334673`, compiled and ran the original repository Rust engine adapter, the standalone Python replay, the new Rust triple projection and the focused strict lint. All 29 job steps succeeded. [Archived machine-readable original outputs](S4_TRIPLE_CI_EVIDENCE_20261115.json). Standing remains **[pcc:unverified]** (GitHub development CI is not independently signed PCC).

| Actual Rust development value | Result |
|---|---:|
| S4 projected rational roots | 17,980 (all distinct) |
| Nested S3 quadratics (scalar replayed) | 9,455 |
| Batched field inversions | 2 |
| Scalar reference inversions | 9,455 |
| Group additions for **post-discovery validation only** | 26,970 |
| Batched field-projection phase | 2.913431 ms |
| Scalar inversion reference phase | 4.773699 ms |

The two reported clocks come from a single unpaired **development** run, with batch first and scalar second and no A/A randomization, no full cold setup, memory pricing or exclusive host control. Therefore the measured ratio is *not* a qualified speedup, and the 26,970 validation group additions cannot be treated as relation discovery work or elided from an eventual end-to-end independent replay. The Python check matched the Rust FNV and independently recomputed the SHA-256 root corpus.

**New whole-pipeline design constraint:** naive enumeration of the two triple side projections and one admissible distinct-index pair projection requires `17980² × 930 = 300,650,772,000` tuple candidates per target before any central S4 predicate. This exact Cartesian-product count is not a group-operation count or a lower bound on algebraic elimination. It rules out *that particular unstructured enumeration* as the next algorithm; the target-dependent quotient/subresultant intersection and its complete rank-and-cost gate are the next experiment.
