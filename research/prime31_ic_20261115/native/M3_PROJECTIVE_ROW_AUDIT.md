# Prime31 m=3: raw collision counts overstate useful relation yield (2026-11-15)

**Status:** independently written producer-local C++ point enumeration + Python exact-rank/normalization replay; Rust-native instrumentation has been checked in but has **not been compiled or executed at the GitHub head**. All findings here are **[pcc:unverified]**. This is an index-calculus relation-collection diagnostic on the existing *ordinary, full-prime-order* p=2^31−1 curve, not a faster complete cold ECDLP claim.

## Frozen input and method

Use the already registered curve
`p=2147483647, E:y²=x³+847514846*x+2087444724`,
`n=2147446831` prime, cofactor one, and the same Dickson fibre construction in `native/lib.rs`. Two factor-base arms are compared without target selection: (i) first fully split depth-8 Dickson fibre with **B=135 liftable, nonzero-y points**, and (ii) the first **B=135** liftable points in increasing x order, on the *same curve*. The existing depth-9 **B=256** Dickson arm is truncated at its previously recorded first full-rank point, signed template 4,051,363. In each arm, enumerate the eight signed sums of each distinct-index triple `i<j<k`, keeping one first representative per EC group point; for each later representative of the same group point, subtract signed incidence vectors to get an exact homogeneous relation over `F_n`.

A nonzero row `r` is canonically represented by its sparse nonzero coordinates after scaling its first nonzero entry to one. Consequently, distinct rows differing only by a nonzero scalar are in **one projective class**. Count classes *before* modular Gaussian insertion; separately compute **actual independent row rank**. If a point has several three-sum witnesses, its first-versus-each-other star relations span all pairwise differences, so retaining the first representative is rank-complete for that sum point.

The producer-local reference used independent standard C++ affine point addition and two Python modular-elimination implementations, not the checked-in Rust F4 oracle or the historical C++ three-summand **solver**. That arithmetic was rerun on the fixed curve and gave the pre-existing counts for depth-8 and early depth-9 exactly. Its raw transient collision transcripts had SHA-256:
- Dickson B135: `cc04a93488292fd4af59509b6adb4135ff47b5ad55731bf7ea6b695c4bfbe3e8`.
- Plain-x B135: `1059dff96fe3da171f01ee6f8214c7f7520bfc113ae7850ddd86c71a37c52932`.
- Dickson B256 early: `a0db101100439f6fd79e1aa789878e0b0448900790a6da7203d563779e2385b9`.

These are local producer transcript hashes, **not independent signed replay certificates**; the transient transcripts are not checked in. The Rust program below is the exact-head reproducible version of the high-level counts.

**Additional group-law replay:** a separate Python implementation reconstructed each factor base and recomputed **both** signed triple group sums for **every 8,379 recorded collision pair** (2,370 + 2,244 + 3,765). All 8,379 equalities passed. This is still an in-session, producer-operated cross-check, not independent PCC certification. It guards against treating modular-index coincidences as actual elliptic-curve relations.

## Exact finite-instance results [pcc:unverified]

| Arm | Signed templates processed | Raw same-sum collisions | Distinct projective relation rows | Independent rank | Rank needed B−1 | Raw collisions per projective class |
|---|---:|---:|---:|---:|---:|---:|
| Dickson depth 8, B135 | 3,207,960 | 2,370 | **127** | 126 | 134 | 18.66 |
| First liftable x, B135 | 3,207,960 | 2,244 | **122** | 122 | 134 | 18.39 |
| Dickson depth 9, B256, early | 4,051,363 | 3,765 | **305** | **255** | 255 | 12.34 |

Each row had its associated EC-point equality replayed by construction and its linear-algebra row computed from exact signed indices. Both B135 arms reach *every* admissible distinct-index triple and **remain rank-deficient**: Dickson by 8 beyond the inevitable one-dimensional log kernel; plain-x by 12. Neither missing rank can be manufactured by further deduplicating the existing rows. At B256 the early bank has 305 projective classes, only 255 of which contribute a new pivot. No performance advantage over rho is inferred. The B256 arm is a truncated prefix, not an exhaustive B256 census.

### Mechanism: spectator and partition multiplicity

For a primitive homogeneous signed relation with **six distinct** basis points, `sum_{i=1..6} eps_i P_i=O`, any choice of three of its signed terms on the left and the three negated terms on the right makes a valid triple-sum collision. There are `C(6,3)=20` such ordered 3-versus-3 partitions, all producing **the same homogeneous equation up to sign**. Thus one algebraic six-term zero relation can inflate a raw collision counter by 20 without generating 20 new independent rows. Patterns with a repeated basis index and resulting coefficient ±2 have different multiplicities. Lexicographic truncation can reveal only a subset of the partitions.

For the *fully enumerated* Dickson B135 arm the histogram of (nonzero row support, multiplicity per projective class) is:
- `(6,20): 114` classes; `(6,18): 1` class; `(5,6): 12` classes.
For the fully enumerated first-liftable-x B135 control:
- `(6,20): 108` classes; `(5,6): 14` classes.
For the *early* B256 prefix:
- `(4,200): 1`, `(4,2): 1`, `(5,4): 7`, `(5,6): 12`, `(6,1): 1`, `(6,6): 12`, `(6,10): 12`, `(6,12): 231`, `(6,16): 3`, `(6,18): 25` classes.

The `(4,200)` class is a particularly clear example of reusing one relation with many spectator configurations; each duplicate is a genuine curve equality but **not a new independent rank increment**. This is a finite-instance combinatorial observation, not a proof of subgeneric or generic scaling for algebraic oracles.

## Rust change and reproduction

The existing `examples/prime31_m3_collision_ic.rs` was extended **in place**, preserving `Arithmetic`, `Base`, `check_curve`, the frozen public Q, and the full collision/anchor/target recovery path. It now performs sparse field-normalized projective deduplication before calling `HomogeneousRank::insert` and fails closed unless the depth-9 early prefix has exactly **305 projective classes and 3,460 rejected duplicates**, alongside its prior 255-rank, 3,765-collision and 4,051,363-template assertions.

Run the same existing Rust solver and (optionally) its two complete B135 controls:

```sh
cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m3_collision_ic
cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m3_collision_ic
PRIME31_COMPARE_BASES=1 cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m3_collision_ic
```

The optional plain-x arm is a **negative structural control**; it uses no auxiliary map and is not a proposed alternate cryptanalytic attack. It runs after the public target solve, outside the cold-solve counter. The normal path still builds its collision bank by **generic group enumeration**, not algebraic F4; the dedup change improves rank-insertion accounting, not asymptotic relation-finding cost. No benchmark performance numbers are presented for this Rust change.

## Lead assessment and highest-value experiments

1. **Rank-yield accounting is mandatory:** in every native algebraic PKM m3/m8 and dense-vs-tower oracle experiment, log *raw witnesses*, projective-distinct relation classes, and **incremental independent rank**, all three separately. Our measured 12–19× duplicate inflation shows why raw-output `delta` is unsafe in `gamma < 3 + delta/2`. Classify primitive support size and spectator multiplicity.
2. **Natural-positive dense-versus-tower pair:** use the existing Rust `src/cryptanalysis/pkm_smooth_tower.rs` and its `algebraic_decompose_dense` (#4275) versus `algebraic_decompose` (#4091) on identical naturally sampled public prime31 queries, including failed/censored ones. Hold out planted fixtures. The current focused native crate imports `f4_fp_tower.rs` and `sig_fp_tower.rs` but not yet the generic dense-oracle module; do not claim it ran here.
3. **Direct m=6 collision rows or sparse m=8 positive witnesses:** compare an algebraically extracted zero relation against these group-enumeration controls. First certify that its independent useful rank grows, then measure **whole-call** arithmetic and memory with two held-out size rungs; the frozen #4077 target selection and the rank-adjusted `gamma/delta` gate remain mandatory. The packed Rust native adapter has only two-summand S3; six-free-variable support is not implemented, so this lane is a prerequisite, not a finished experiment.
4. **Cold solve gate:** every candidate must solve public unknown-scalar Q with independently replayed factor-base logs (or collision/anchor equivalent) and final `[d]G=Q` on this same prime-order curve, charging setup, failures, row elimination, target logs and comparison to matched optimized negation-map rho. No generic fallback may supply the claimed IC win.

### Prior art and previous lead areas reviewed

- [PKM, PKC 2016, primary PDF](https://www.iacr.org/archive/pkc2016/96140156/96140156.pdf): composed low-degree factor bases and Semaev relations on prime-field elliptic curves; no known speed conclusion follows just from low polynomial degree. Their §2–3 define the relevant relation model. Not a new factor-base invention.
- [crypto #4077](https://github.com/isogeny-labs/crypto/pull/4077): preregistered sparse m8 full-call gate.
- [crypto #4091](https://github.com/isogeny-labs/crypto/pull/4091) and [#4275](https://github.com/isogeny-labs/crypto/pull/4275): existing Rust tower and dense algebraic oracles; the dense advantage was measured on small **planted-positive** systems, not cold ordinary-curve prime31 solves.
- [crypto #4228](https://github.com/isogeny-labs/crypto/pull/4228): paired S3 preprocessing, a component improvement not yet a complete cost reduction.
- [crypto-autoresearcher #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305): recent Semaev alternative campaign and lead dispositions; do not relabel previously closed axes as new.
- [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536): auxiliary-map Rust implementation and graph/proof-DAG row combination, but no new complete cold advantage verified.
- [cryptanalysis ECDLP attack matrix](https://github.com/isogeny-labs/cryptanalysis/blob/main/suite/docs/ECDLP_ATTACK_MATRIX.md): residual/collision hybrid negative at its measured scope.
- [crypto #4188](https://github.com/isogeny-labs/crypto/pull/4188) and the previous `M3_RANK_EARLY.md`: ordinary prime31 control, existing early-stopping model and frozen results.

KB access: the research-KB MCP server and repository CLI wrapper were not available in this runtime. A narrow GitHub search in all three named repositories found adjacent matrix/solver work but no exact already published instance of this particular finite-prime31 projective histogram; **this is not an exhaustive novelty search**. Conductor, Cargo, authorized EKS and PCC independent signer also remained unavailable. No paid run, merge, or leaderboard promotion. [pcc:unverified]
