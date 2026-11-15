# Prime31 m=2 relation-yield preflight, native Rust continuation

**Date:** 2026-11-15 (UTC). **Standing:** local producer arithmetic [pcc:unverified]. This is an exact finite-instance *yield census*, **not** a timed Rust execution, complete prime31 logarithm recovery, normalized ecbench cost, or a cheaper-than-rho claim. The requested curve and full prime-order group are unchanged.

## Reproduction and implementation boundary

The new executable [examples/prime31_two_sum_gate.rs](examples/prime31_two_sum_gate.rs) imports `prime31_algebraic_ic::{check_curve,Arithmetic,Base,PRIME31}`. Those public items use the existing Rust native adapter, which itself directly references the repository's unchanged `src/cryptanalysis/f4_fp_tower.rs` and `sig_fp_tower.rs`; the yield census does **not** call F4 and must not be described as a polynomial-solving measurement. It also does not import the standalone C++ pair-table experiment.

    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_two_sum_gate

The narrow `.github/workflows/prime31-algebraic-ic.yml` now runs this exact preflight after the focused Rust tests. A separate independent Python integer-arithmetic implementation was executed locally before check-in to enumerate the support. **No exact-head Rust or workflow success is asserted by this document**: source compilation and CI outcome must be checked on the pushed head.

The curve is `p=2147483647, E:y²=x³+847514846*x+2087444724, n=2147446831, G=(625037715,2100627251)`. This is the existing ordinary, full prime-order fixture. `Base::build` deterministically chooses the first qualifying Dickson terminal from seeds 2..9999. The independent residue square-root replay and the Rust assertions use the same guards: full distinct x-fibre, nonzero liftable y, canonical ±y.

## Exact local arithmetic census [pcc:unverified]

| depth | first selected seed | Dickson terminal | full x-fibre | actual base B | distinct signed pair-sums `s` | ideal mean independent uniform calls to first positive `n/s` | ideal mean calls to collect B+1 rows, lower bound* |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 2 | 3 | 47 | 4 | 2 | 9 | 238,605,203.4 | 644,234,049.3 |
| 8 | 3 | 117813225 | 256 | 135 | 36,451 | 58,913.2 | 7,982,855.5 |
| 9 | 3 | 375193997 | 512 | 256 | 131,067 | 16,384.3 | 4,202,408.0 |
| 10 | 3 | 1616683971 | 1024 | 509 | 518,085 | 4,145.0 | 2,111,546.4 |

`s` is the exact number of *distinct group points* reachable by `±P_i±P_j`, including repeated indices and the identity. The maximum possible size for this signed two-summand set is `2B²+1`; depth 8 attains it, depth 9 misses it by 6, and depth 10 by 78. Thus the existing two-summand fibre's first-order limitation is the number of group targets it covers, **not** a widespread collision defect. The factor-base cardinality is **not** equal to the full x-fibre size.

* The last column is only a full `B+1` **linear-row-rank goal**, not a universal minimum for solving `Q=[d]G`: a shorter dependent cycle or direct identity row could isolate `d` earlier. It must never be presented as an unconditional ECDLP lower bound or an ecbench measurement.

## Mathematical accounting identity

For `B` distinct nonzero-y base points on a prime-order curve, canonicalize the two signed indices so that `i<=j`, with the repeated-index `(+,-)` and `(-,+)` treated as one row. There are

    M(B) = 4*C(B,2) + 3*B = 2*B²+B

canonical signed relation *templates*. For `i<j`, there are four signs; for `i=j` there are three canonical sign patterns (`++`, `--`, `+-`), the last yielding identity. Each template produces a definite group point.

Let `R` be uniformly distributed over this `n`-point prime-order group, independent of the fixed base. Since each template matches exactly one possible `R`, double-counting gives the **exact** expected number of distinct templates matching one `R`:

    E[matching relation templates per query] = M(B)/n.

Likewise, if `S_2` is the set of distinct reachable sums, then

    Pr[any positive relation on a uniform query] = |S_2|/n
    E[independent uniform queries until first positive] = n/|S_2|.

For an ideal oracle returning *all* matching templates, obtaining rank `B+1` requires at least `B+1` returned templates. Under i.i.d. uniform queries and a finite-mean stopping time, Wald's identity therefore yields the optimistic query lower bound `E[T] >= (B+1)n/M(B)`. **This does not apply to a target-adaptive generator of R, to a growing base, to a different m-summand relation mechanism, or to methods recovering the scalar before complete B+1 rank.** The native seeded SplitMix-derived `a,b` are deterministic modular reductions, not mathematical i.i.d. uniform draws; this idealized statement is a protocol model, not a claim about their exact empirical distribution.

Even a zero-cost algebraic solver cannot increase the probability that a fixed signed two-summand base contains a truly uniform target; it can only improve the cost of deciding membership and extracting every witness. A complete cold comparison still needs the stronger matched ecbench rho/BSGS baseline and actual end-to-end target recovery.

## Prior-lead and cross-repository check: do not retest an already settled axis

Checked the GitHub PR records in `isogeny-labs/crypto`, `crypto-autoresearcher` and `cryptanalysis`, the 304-row [prime IC heat map](../../prime_ic_lead_heatmap_20261114/TABLES.md), and the native README's prior-work table. Selected consequences:

| Prior work | Existing evidence at the cited scope | What to do instead |
|---|---|---|
| [crypto #4091](https://github.com/isogeny-labs/crypto/pull/4091), [#4257](https://github.com/isogeny-labs/crypto/pull/4257) | Rust PKM tower tooling, m2/m3 oracle runs and dense-membership observations | Reuse their actual `pkm_smooth_tower.rs` algebraic work, do not translate the C++ enumerator |
| [crypto #4275](https://github.com/isogeny-labs/crypto/pull/4275) | Dense membership was cheaper than tower on **planted positive** small cells, but enumeration remained cheaper | New **natural-positive** dense-vs-tower A/B on prime31, with full solved witnesses, misses and marginal independent rank |
| [crypto #4228](https://github.com/isogeny-labs/crypto/pull/4228) | Certified S3-pair cubic preprocessing changed native solver component costs; no complete cold ECDLP ranking | Test against exact same natural systems, charge preprocessing and replay |
| [crypto #4077](https://github.com/isogeny-labs/crypto/pull/4077) | Corrected m8 whole-call model and prospective positive sparse protocol | Preserve the unmodified m8 frozen selection, measure complete `gamma` and independent-rank `delta` |
| [crypto #4282](https://github.com/isogeny-labs/crypto/pull/4282) | Multi-target IC retuning near 2^32 was still 1.06–1.15× matched rho at its own measured target counts | Keep amortized multi-target separate from one-cold prime31 objective |
| [crypto #4259](https://github.com/isogeny-labs/crypto/pull/4259) | Smooth-order multiplicative subgroup factor bases are already PKM 2016 §3.3 | Make no novelty claim for this subgroup-factor-base selection |
| [crypto-autoresearcher #3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301) | Heat-map outcomes transferred to atlas with reopen triggers; not an exhaustive formal closure | Reopen only with a changed algebraic relation mechanism and actual evidence |
| [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877) | Chained transfers lack a costed complete scalar-blind cheaper map | Keep separate from the immediate fixed-field IC cost gate |

Primary source scope: Petit–Kosters–Messeng, *Algebraic Approaches for ECDLP over Prime Fields* (PKC 2016, DOI 10.1007/978-3-662-49387-8_1); Diem, *On the discrete logarithm problem in elliptic curves* (Compositio 2011), especially its extension-field qualifications; Shoup's generic lower bound (EUROCRYPT 1997) only rules out *generic* speedups. The shared research-KB MCP and repository wrapper were unavailable; GitHub source/PR tables and primary literature metadata were used as a documented fallback. No exhaustive search or novelty claim.

## Next proposed candidate experiments (not yet frozen)

1. **Algebraic dense-vs-tower paired oracle on the same prime31 curve.** Reuse the existing `pkm_smooth_tower.rs` dense relation equations and test them against this native Dickson baseline. At each matched liftable B, feed identical `R=[a]G+[b]Q` public target coordinates, some planted controls held separate from the primary random panel. Count every positive, failed, undetermined and time-capped call; independently replay witnesses; measure F4 wall/CPU/RSS, field operations, and incremental relation rank, not just degree. This is the first discriminating experiment given #4275, rather than relaunching the same tower-only probe.
2. **Sparse m8 over prime31.** On this field `v2(p-1)=1`, so *split binary Kummer* size 8/16/32 is invalid; `v2(p+1)=31` permits the already implemented nonsplit Dickson fibre. The current packed native two-summand F4 adapter does not represent six free abscissae for an m8 chain. Extend and audit this representation (or reuse the existing generic Rust exponent-vector F4 system) before any natural-positive m8 scaling claim. Keep #4077's `gamma<3+delta/2` necessary gate and at least two held-out larger sizes.
3. **Complete one-cold prime31 target recovery:** after admission and full ecbench phase accounting, run the best candidate against optimized negation-map rho and BSGS on identical public inputs and machine; count base generation, failed calls, root extraction, linear algebra, target recovery, and `[d]G=Q` verification. Run a real independent replay before cost/security promotion.

**Execution/admission:** `docs/session-logs/2026-11-15.md` and `native/EXPERIMENTS.md` document the missing EKS/checkpoint transport, incomplete development counters, and the earlier root Rust release-suite SIGKILL. The preflight requires no paid job. Do not promote these deterministic counts as an execution-certified performance comparison. [pcc:unverified]
