# Speeding up hyperelliptic index calculus at genus 2, 3 and 4: plan

2026-11-13. Branch `research/hec-native-ic-20261113`, from `main` at `8839290e5f`.

This is a programme plan with a frozen protocol for its first slice (S1, at the end). Each later
track gets its own protocol, written before its measurement, under `AGENTS.md` "The rule: boundary,
table, ratio" and the R10 ledger rules.

## 1. Goal and success criterion

Lower the cost of index calculus (IC) on Jacobians of imaginary hyperelliptic curves
`y² = f(x)`, `deg f = 2g+1`, of genus 2, 3 and 4 over prime fields `F_p`. Each track measures
against a named baseline on the same instances, with every answer verified.

The success criterion is `AGENTS.md` "Success criterion: iterative improvement of the IC baseline".
Beating rho is not the goal. Every row still carries its gap to matched rho (ecbench `rho.negation`,
adopted as the reference in round sixteen).

**Exponent targets (heuristic, literature).** Double-large-prime IC costs `Õ(q^{2−2/g})`.

| genus | rho `q^{g/2}` | full-base IC (relation `O(g!·q)`, solve `O(q²)`) | single LP `q^{2−4/(2g+1)}` | double LP `q^{2−2/g}` |
|--:|--:|--:|--:|--:|
| 2 | `q^1` | `q^2` | `q^{1.2}` | `q^1` |
| 3 | `q^{1.5}` | `q^2` | `q^{1.43}` | `q^{4/3}` |
| 4 | `q^2` | `q^2` | `q^{1.56}` | `q^{1.5}` |

So:

- **Genus 2 can only gain constants.** Its best IC exponent equals rho's.
- **The exponent targets are 4/3 at genus 3 and 3/2 at genus 4.**
- The full-base table reflects round seven: with a full base, the solve outgrows rho below genus 4.

## 2. Sources checked (literature first)

| source | what it gives | how it was checked |
|:--|:--|:--|
| Gaudry, EUROCRYPT 2000 | full-base IC, `O(g!·q)` relation stage | recalled; the repo's engine implements it (`src/cryptanalysis/hyperelliptic_index_calculus.rs`) |
| Thériault, ASIACRYPT 2003 | single large prime, `Õ(q^{2−4/(2g+1)})` | via `RESEARCH_HYPERELLIPTIC_IC_RHO.md` §round six and KB `IDEA-20261009-275a14` ("recalled") |
| Gaudry–Thomé–Thériault–Diem, Math. Comp. 76 (2007), eprint 2004/153 | double large prime, `Õ(q^{2−2/g})`; graph-based cycle combination | KB `KN-LIT-92caf4` (abstract-level, "not re-extracted"); exponent cross-checked in KB notes on Diem (`KN-LIT-a45b7b`) and Chao et al. (`KN-LIT-a4d0f1`) |
| Nagao, JJIAM 24(3) 2007 | an alternative two-large-prime IC for small-genus hyperelliptic Jacobians | cited in Nagao 2013/2015 full texts in the KB; paper itself not in the corpus |
| Diem, plane curves of small degree | `Õ(q^{2−2/(d−2)})` for degree-`d` plane models; `Õ(q)` for non-hyperelliptic genus 3 | KB `KN-LIT-a45b7b` (read from `downloads/diem.pdf`) |
| Laine–Lauter, eprint 2014/346 | time–memory trade-offs for Diem's genus-3 algorithm; an implementation that makes constants explicit | KB `KN-LIT-446` (abstract only); non-hyperelliptic, so it is a reference for constants, not a baseline here |

**Evidence gaps.**

- The GTTD and Thériault full texts are not in the local corpus. eprint was behind a Cloudflare
  challenge on 2026-10-24 (memory `high-genus-dlp-records`).
- Their published timings and constants are not yet extracted.
- Before a track claims anything against "the published 2LP", it must extract their cycle-finding
  rule, their factor-base size rule and their record timings from the primary text. Until then they
  are labelled "published, not reproduced".

## 3. Where the repository stands (read 2026-11-13)

| asset | genus | arithmetic | relation search | large primes | solve | sizes reached | record |
|:--|:--|:--|:--|:--|:--|:--|:--|
| `src/cryptanalysis/hyperelliptic_index_calculus.rs` | 2, 3, 4 | `BigUint` Mumford/Cantor | factor-base walk (no step precomputation) | single LP (round eight) | Markowitz sparse | g2 `p ≤ 4001`, g3 `p ≤ 401`, g4 `p ≤ 61` | `RESEARCH_HYPERELLIPTIC_IC_RHO.md` rounds 1–16 |
| `examples/ic_improve4_01_genus3_rel.rs` (+ rounds 2–4) | 3 (GHS cubic cover only) | native `u64`/`u128` | walk with C1 (Lucas cubic split) and C2 (batched inversion, `W = 32`) | double LP (graph) | SGE + block Wiedemann (`lpsgebw2`) | `q = 2^12 … 2^22` | `research/ic_improve4_20261015/01-genus3-relations/README.md`; pinned `g3ghs-v1` |
| `src/cryptanalysis/hyperelliptic_word.rs` + ecbench `ic.hyperelliptic` | ≤ 4 | word-size, `p < 256` | thread's walk | single LP | thread's | smoke only | round sixteen (rho reference adopted) |
| PR #2654 `ic_acct3` (open) | 2, 3, 4 | — | — | — | — | — | prices thread's IC and rho as ecbench arms in `bitop/v1` |

**Measured facts that steer this plan.**

1. **Full base, `S_ic/S_rho` at the largest sizes measured** (round seven, in-situ unit): g2 0.55,
   g3 0.41, g4 0.15. Only genus 4 has a falling exponent (−0.29, consistent with −1/4). Genus 3 is
   flat once the solve is priced.
2. **Round eight: single large primes gave no gain at `p ≤ 4001`.** There the solve is only 1–4% of
   the total at genus 3–4. Its conclusion was "at reachable sizes the lever is the oracle and the
   per-trial cost of the walk, not the factor base".
3. **The `BigUint` engine is what caps the sizes.** At `p ≤ 401` (genus 3), the regime where the
   double-LP exponent shows cannot be reached. The genus-3 GHS engine shows that native arithmetic
   reaches `q = 2^22`, and that 2LP plus SGE/BW gets within 0.035 of 4/3 there.
4. **Oracle work is under-priced in counted units.** Round six measured wall clock 1.5–2.2× for a
   counted 60% cut. Wall clock (and `bitop/v1`, PR #2654) must sit beside counted units.

**Gaps.**

- There is no native engine for generic genus-2 or genus-4 curves, and none for genus-3 curves
  outside the GHS family.
- There is no double-large-prime IC at genus 2 or 4.
- There is no pinned IC baseline for genus 2 or 4.

## 4. Tracks

Each track is tagged with its `AGENTS.md` §3 class *as expected*. The measured class is what gets
reported.

### T0. Native engine (infrastructure; enables everything else)

A crate `crates/hec-native-ic`: genus 2, 3, 4 over `F_p`, `p < 2^62`.

- **Field.** `u64` residues, `u128` products.
- **Divisors.** Fixed-array Mumford `(u, v)`, Cantor composition and reduction. A fast path for
  adding a degree-1 place (the factor-base walk's step).
- **Oracle per degree.**
  - Degree 2: one Legendre symbol.
  - Degree 3 and 4: the discriminant-parity filter (round six), then distinct-degree splitting
    through `x^p mod u`.
  - Roots by equal-degree splitting.
- **Walk.** The factor-base walk, with `D₁`/`D₂` in the step set (round five).
- **Solve.** Sparse elimination mod `ℓ` (`u64`, `ℓ < 2^62`).
- **Verification.** Every answer is verified (`k·D₁ = D₂`) before it is returned.

This moves wall clock, not the counted unit. It is not headlined as an improvement. Its job is to
reach the sizes where T1–T4 can pay. Slice S1 builds and gates it.

### T1. State-of-the-art baselines per genus (accounting)

- Port the genus-3 double-LP machinery (graph of partial relations, cycle combination, SGE + block
  Wiedemann) onto T0, generic in `g`.
- Pin `hec2-v1` (genus 2) and `hec4-v1` (genus 4) in `research/ic_tool_program/baselines.json` on
  the thread's curve families, at a size ladder reaching at least `p ≈ 2^16` (g4) to `2^24` (g2).
- At genus 3, run the GHS instances of `g3ghs-v1` through T0 as a reproduction gate (same relations,
  same `k`). Pin a generic-curve genus-3 entry beside it.
- External reference (§15): no open hyperelliptic-IC implementation is known. Published GTTD and
  Nagao timings are "published, not reproduced" until extracted.

### T2. Relation phase: cost per trial (engineering)

The phase that dominates at every reachable size.

- (a) **Batched inversion across `W` lockstep walks** (Montgomery's trick). Generalise genus-3 C2
  to all genera.
- (b) **Degree-1-place addition formulas.** Use them instead of generic Cantor: weight-`g` plus
  weight-1, then reduction.
- (c) **Quartic splitting test for genus 4,** analogous to genus-3 C1. Use the resolvent cubic (it
  splits iff the quartic's Galois group is in `V₄`), then a cubic Lucas test. This replaces
  `x^p mod u` on the half of candidates that survive the parity filter.
- (d) **An early-abort root rule for the large-prime arms.** A candidate whose split has more off-base
  roots than the arm keeps (one or two) is rejected before computing every root.
- (e) **Threads.** Independent walks per core, partials merged centrally. Report wall clock per core.

### T3. Linear algebra (engineering)

- Generalise `lpsgebw2` (large primes as columns, SGE, then block Wiedemann).
- Compare against `la-engine-v1` (CADO-style filter).
- External (§15): FLINT `nmod_sparse`/dense and CADO-NFS `bwc` on exported matrices.

### T4. Balance: factor-base size and large-prime bound (engineering; possibly advance)

- A per-genus `κ` schedule (factor-base fraction against `q`), tuned on training curves and frozen
  before held-out curves, as rounds 2–4 did at genus 3.
- Rounds 2–4 found that a falling `κ` raises the fitted exponent. So both the exponent and the
  constant are reported, over a fixed range.

### T5. Structure (deferred)

- Curves with extra automorphisms. For example `y² = x⁵ + a` or `y² = x⁷ + ax`: the factor base
  shrinks by `|Aut|`.
- Genus 3 through Smith's isogeny to a plane quartic. It gives `Õ(q)` for a positive proportion of
  curves. Diem's algorithm was reproduced in the plane-quartic programme (closed; memory
  `plane-quartic-program-state`).
- Both are special-curve tracks. They report against their own baselines, not T1's.

## 5. Order of work and stopping rules

1. **S1 (now).** Build T0 and gate it (protocol below).
2. **T1 genus 4 first.** It has the only falling exponent and no double-LP baseline. Then genus 2,
   then the genus-3 reproduction.
3. **T2 (a)–(d).** Each lever runs as a paired arm against the pinned T1 entry, under the round-4
   style design: held-out curves, A/A floor, 2,000-resample paired bootstrap.
4. **T3, then T4.** Only once the solve exceeds 10% of the total at the top of the ladder.
5. **Stop rule.** A lever that fails its pre-registered test twice is recorded as a negative result
   and dropped.

## 6. Slice S1: native engine, protocol frozen before measuring

**What is built.** T0 as above, in `crates/hec-native-ic`: a standalone crate with no dependency on
the `crypto` crate. It has an example binary `hec_native_bench` that runs the full-base
factor-base-walk IC on round eight's curve families and instance rule:

- genus 2: `y² = x⁵ + 3x³ + 2x² + x + c`
- genus 3: `y² = x⁷ + x³ + cx + 1`
- genus 4: `y² = x⁹ + x³ + cx + 1`

For each, `c` is the first value giving a prime subgroup `ℓ > 1000` with `4ℓ ≥` the Hasse–Weil lower
bound. The target is `k = ⌊ℓ/3⌋ + 7`, as in round eight.

**Gates (each must hold before any timing is reported).**

- **G1, group law.** The following hold on random divisors at genus 2, 3 and 4, for several primes:
  - associativity and commutativity;
  - `D + (−D) = 0`;
  - doubling equals self-addition;
  - the degree-1-place fast path equals generic Cantor.
- **G2, oracle.** Over all monic polynomials of degree `≤ 4` at `p ∈ {13, 17}` (both residues
  mod 4), the oracle's split decision and root multiset equal brute-force evaluation.
- **G3, instances.** The instance rule picks the same `(c, ℓ)` as round eight's example on every
  round-eight row. `ℓ·D₁ = 0` holds.
- **G4, answers.** Every IC run returns a `k` with `k·D₁ = D₂`, checked inside the crate.

**Measurement (descriptive, not a ranked claim).**

- Instances: round eight's `θ = 1` rows (genus 2 `p ∈ {101, 251, 503, 1009, 2003, 4001}`, genus 3
  `{61, 211, 401}`, genus 4 `{31, 61}`), then larger `p` that only the native engine reaches.
- Five seeds per row. Report:
  - wall-clock milliseconds per solve;
  - trials, group operations, smoothness rate;
  - oracle and solve wall time as a share of the total.
- Compare against round eight's frozen `wall_ic_ms` and against a re-run of
  `examples/hyperelliptic_ic_large_prime.rs` (`LP_ONLY=g:p`, `θ = 1`) on the same host.

**Pre-registered expectations.**

- **E1.** The native smoothness rate lies within the BigUint engine's range at the same row: round
  five's 0.49–0.56 (g2), 0.17–0.27 (g3), 0.05–0.08 (g4). The two walks use different random streams,
  so equality is not expected.
- **E2.** Wall clock per solve is at least 10× below the BigUint engine on every shared row.
  Falsified by any row below 10×.

**Class.** Infrastructure. The algorithm is unchanged (full base, factor-base walk). Counted group
operations are not expected to move, so no `S` improvement is claimed. The deliverable is the
ability to run T1–T4 at sizes the BigUint engine cannot reach.

**Host.** Apple-silicon laptop (arm64), single thread. Results land in `S1-RESULTS.md` beside this
file, with the command, the binary hash and the raw output. Records go to the artifact store per
`AGENTS.md` §13 if they exceed the in-Git limit.
