# L10: C_{3,4} / Picard curves as index-calculus targets, with a correctly matched rho (frozen protocol, 2026-11-16)

Status: protocol only. No data from the frozen instrument existed when this file was
written. Section 0.3 discloses one pre-freeze feasibility probe (two curves, 19 operations
each), run to make sure the lead is not void. Every number this study produces is
`[pcc:unverified]`: one session, a shared 4-core container, no independent replay. Lab
numbers quoted here are `[pcc:unverified]` assertions. This is not a benchmark. It adds no
leaderboard row, no `benchmarks/` row and no speed claim, and it edits no existing row.
Costs are exact counts, never wall time.

Implementation contract: [`SPEC.md`](SPEC.md). Freeze: `frozen.sha256` holds the sha256
of this file and of SPEC.md, written before any code or data.

## 0. Prior art and pre-freeze checks

### 0.1 Paths read (grep; the research-kb tool is not installed in this session)

| path | what it shows |
|---|---|
| `src/cryptanalysis/quartic_ladder.rs:1-50, 1336-1418` | genus-3 plane-quartic rung: Diem line relations on a **generic** smooth plane quartic, and `run_rho` = `rho_reference_walk(..., RhoWalk::negation())`. `QGroup::neg` is "uncharged" in `group_ops`, but `pic_mul`/`pic_inv` include "the uncharged negations of the negation map's canonicalisation" (doc comment at 1381-1382) |
| `src/cryptanalysis/ic_boundary.rs:2001-2016` | `NegationClasses::canon` calls `g.neg(p)` and compares keys at **every** canonicalisation (every walk step) |
| `src/cryptanalysis/ic_boundary.rs:2025-2060` | `CubicClasses`: the order-6 class walk exists for elliptic `j = 0` curves only |
| `src/bin/genus_ladder.rs:496-509` | quartic rho record `Q_rho_neg`: `cantor_mul = o.pic_mul`, `cantor_inv = o.pic_inv`, so the FPM of the rows includes the negation work |
| `src/cryptanalysis/plane_quartic_pic.rs:1-45, 1095-1160, 1284-1340` | the lab representation: `add` reduces a degree-6 divisor through cubics; `neg` takes the conic through `D_x + 2 P0` and computes the residual by an ideal quotient. Negation is a full linear-algebra operation, not a sign flip |
| `research/genus_ladder_20261114/RESULTS.md:41, 60, 73, 95-99, 165` | quartic row at `p = 1031`: IC/rho 0.027 [0.019, 0.037]; rho exponent 1.36; "about 8,900 field multiplications per `Pic` addition" (that is, per group operation of the negation walk); H8 "quartic IC below quartic rho, FPM band: held" |
| `research/genus_ladder_bound_r_20261114/RESULTS.md:29-35, 41-46, 98` and `bound_r.jsonl` | quartic 2LP/rho = 0.0089, 0.0086, 0.013 at `p` = 2053, 4099, 8209. In the raw rows, `cantor_mul / group_ops` = 8,836 to 8,924 and `cantor_inv / group_ops` is about 78 |
| `research/plane_quartic_program_20261018/CLOSING.md:19-37`, `docs/ic/LEADERBOARD.md:759, 843, 918` | GHS-weak quartic rows (N4, N4 v2, DL1). Their matched rho runs on the **elliptic** `E0(F_{q^3})`, where negation costs nothing, so this lead's accounting question does not reach them |
| `research/diem_quartic_relations_20261018/PROTOCOL.md`, `research/quartic_pic_canonical_20261018/` | Diem relations and W1 canonical `Pic^0` (generic `add`: 3,523 multiplications and 44 inversions) |
| `research/leads_20261111/EXISTING_LEADS.txt` | `LEAD-RHO-negation-map-fruitless-cycles`, `LEAD-RHO-automorphism-class-walks`, `LEAD-ICB-orbit-linear-algebra`: names only, all framed for elliptic or Koblitz curves |
| `research/g2plus_leads_20261116/L02_automorphism_orbit_bases/PROTOCOL.md` | sibling lead: orbit factor bases on CM **hyperelliptic** `y^2 = x^q + a`. Same mechanism (Duursma-Gaudry-Morain), different curve family and representation; not a duplicate |
| grep `picard curve|C_{3,4}|c34|superelliptic` over `crypto/` and `../cryptanalysis` | no Picard or C_{3,4} index calculus, and no non-hyperelliptic automorphism fold, in either repository |

Verdict: **not a duplicate**. Diem relations on generic plane quartics and the negation-map
quartic rho exist. Picard and C_{3,4} targets, the orbit quotient on a non-hyperelliptic
Jacobian, and an audit of what the quartic rho's negation costs do not. The orbit mechanism
is literature prior art (Duursma, Gaudry and Morain 1999), and no novelty is claimed for it.

### 0.2 Mathematics of the brief, checked and corrected

Coordinates follow the lab: a point is `[X0, X1, X2]`, a degree-4 form is
`sum f[i] X0^a X1^b X2^c` over `mons_of_degree(4)` (a descending, then b), and the affine
chart is `X0 = 1` with `x = X1`, `y = X2`.

1. **C_{3,4} curves have a hyperflex at infinity, not just a flex (correction).** A C_{3,4}
   curve is `y^3 + x^4 + sum c_ij x^i y^j` with `3i + 4j < 12`. Homogenised with `Z = X0`,
   the line `X0 = 0` meets it in `X1^4 = 0`. That is the single point `P_inf = [0,0,1]`
   with contact order 4, a total inflection point (hyperflex). It is smooth because
   `dF/dX0 = X2^3 = 1` there. Picard curves `y^3 = x^4 + a2 x^2 + a1 x + a0` (`p != 2, 3`,
   depressed, `f` squarefree) are the special case. Consequences:
   - With base point `P0 = P_inf`, the lab's `kappa = [K - 4 P0] = 0` exactly, because the
     tangent line section is `4 P0`. The `kappa` unknown disappears, and every line section
     gives `sum c(P_i) = 0`.
   - `D_inf = 3 P0` is invariant under every automorphism that fixes `P0`.
   - Curves with a rational hyperflex form a codimension-1 family of quartics. "Generic
     quartic" (lab family, `kappa` unknown) and "generic C_{3,4}" are therefore separate
     controls.
2. **Line relations: probability about 1/2 is right for a full base.** For distinct rational
   points `P, Q`, the line `PQ` cuts `C` in `P + Q + R1 + R2`. The residual pair consists of
   the roots of a binary quadratic, so it is rational when the discriminant is a square:
   probability about 1/2, plus about `1/p` for tangency. With the factor base equal to all
   rational points, every rational residual is in the base. Lines through `P_inf` are the
   verticals `x = x0`.
3. **The automorphism and its orbits (refined).** For `p = 1 (mod 3)` and `zeta` of order 3
   in `F_p`, `rho: [X0,X1,X2] -> [X0, X1, zeta X2]` is an `F_p`-automorphism of every Picard
   curve, and not of a generic C_{3,4} curve. Orbits on rational points: size 3 for affine
   points with `y != 0`; fixed points are `P_inf` and the at most 4 points `(x0, 0)` with
   `f(x0) = 0`. Since `div(x - x0) = 3(x0,0) - 3 P_inf`, each `c((x0,0))` is 3-torsion and has
   log 0 in any prime-order subgroup of order `r > 3`. `rho` maps lines to lines, so it maps
   relations to relations.
4. **Eigenvalue (corrected statement).** The quotient `C/<rho>` is `P^1` (the `x`-map), so
   `1 + rho + rho^2 = 0` on `J`. Let `r` be a prime with `r > 3` and `r^2` not dividing
   `#J`, and let `G` generate the order-`r` subgroup. Then `rho` acts on `<G>` as `[lambda]`
   with `lambda^2 + lambda + 1 = 0 (mod r)`. Hence `r = 1 (mod 3)` necessarily, and
   `l(rho^k P) = lambda^k l(P)` for the projected logs `l(P) = log_G([h] c(P))`,
   `h = #J / r`. If `r^2` divides `#J`, `rho` need not act as a scalar on the `r`-part, so
   the tiny-tier instances require `r^2` not to divide `#J`.
5. **Orbit quotient (brief's prediction, refined).** Columns fall from `n_F = N - 1` (all
   points but `P0`) to `n_O = (number of affine points with y != 0) / 3`. A quotient row
   carries coefficients `lambda^k`, combined as a triple `(m0, m1, m2)` modulo `(1,1,1)`.
   - A vertical line through an orbit `{P, rho P, rho^2 P}` gives `1 + lambda + lambda^2 = 0`,
     a trivial row, and `rho L` gives the same row times `lambda`, a duplicate. Both are
     wasted trials in the orbit arm.
   - Expected effect: relations needed fall by about 3 and matrix weight by about 3.
   - Counted linear algebra falls by about 9 under the Wiedemann model the lab uses
     (cost proportional to `n x (n + w)`), and by about 27 under dense elimination. The brief's
     "9" holds for Wiedemann only, and SPEC fixes Wiedemann.
   - Point enumeration is common to both arms and does not shrink.
6. **The matched rho for Picard targets is not the negation walk (addition to the brief).**
   A correctly matched rho on a Picard Jacobian may use the class of `x` under
   `<-1, rho>` (order 6) or under `<rho>` alone (order 3). Ideal expected steps are
   `sqrt(pi r / (2 |A|))` with `|A|` in {1, 2, 3, 6}, and each canonicalisation charges the
   negation or `rho` applications it needs. An orbit-base IC therefore has to be compared
   with the cheapest of these walks, not with the lab's negation walk.

### 0.3 Pre-freeze feasibility probe (disclosed; not data of this study)

To check that question (1) is not void, a throwaway crate in the session scratchpad
included `plane_quartic_pic.rs` by `#[path]`. On one generic quartic at each of `p` = 2053
and 8209 (lab generator, rng seed 10), it measured 19 additions and 19 negations:

- `add`: 3,521 multiplications and 44 inversions per operation;
- `neg`: 5,400 multiplications and 35 inversions per operation.

The sum, 8,921 multiplications and 79 inversions, agrees with the observed
`cantor_mul / group_ops` of about 8,900 and `cantor_inv / group_ops` of about 78 in
`bound_r.jsonl`. One Picard curve at `p = 2053` gave `add` 2,833/42 and `neg` 2,442/34.

These numbers are `[pcc:unverified]`, are **not** used by the decision rule, and are
re-measured by the frozen `rhocost` arm (Section 3, R). They are disclosed because they
shaped the hypothesis H1 below. No probe of the relation or linear-algebra instrument was
run.

## 1. Hypotheses and exact claims

- **H1 (accounting, existing rows).** In the lab representation a negation costs at least
  as much as an addition (`N >= A` in FPM). The lab's matched quartic rho (negation walk,
  per-step cost about `A + N`) is therefore costlier in ideal expectation than the plain
  walk on the same group law, by `F_Q = (A + N) / (sqrt(2) A)`. Claim form: "the existing
  generic-quartic rows' matched rho is overpriced by `F_Q` in ideal expectation; their
  IC/rho ratios would be multiplied by `F_Q` if the matched rho were the cheaper walk."
  This claims nothing about a better negation formula, and no row is edited.
- **H2 (Picard matched rho).** On Picard Jacobians in the lab representation, the cheapest
  ideal class walk among plain, negation, `<rho>` and `<-1, rho>` is identified, with its
  cost relative to the negation walk.
- **H3 (orbit IC, primary).** On the same Picard curve and the same pair stream, the orbit
  arm `P_orbit` lowers the counted total `T = enum + line + 9 x LA` of the full-base arm
  `P_full` by at least 2x at the largest primes.
- **H3' (prediction check, secondary).** Line trials fall by a factor in [2.5, 3.5] and
  weighted LA by a factor in [7, 11] at `p = 16411`.
- **H4 (descriptive).** Relation yield per line trial (`accepted / trials`) on Picard, generic
  C_{3,4} and generic quartic curves at the same `p`.

## 2. Unit of cost

`FPM = field multiplications + 10 x inversions`. Squarings and multiplications by constants
count as multiplications, and additions are free.

- Relation-phase and enumeration counts come from the instrument's own counted `F_p` layer
  (SPEC section 3).
- `Pic` counts (A, N, D, R) are deltas of the lab's `PlaneQuartic::counter()`, plus the
  instrument's own count of the `rho` coefficient scaling.
- Linear algebra is counted in mulmods and inversions modulo the LA modulus, and weighted
  by `g^2 = 9` as in the lab ladder: `FPM_la_w = 9 (la_mul + 10 la_inv)`, matching
  `research/genus_ladder_20261114/RESULTS.md:41`. The unweighted value is reported too.

## 3. Arms

| arm | curve family | factor base / columns | role |
|---|---|---|---|
| `Q_full` | generic smooth plane quartic (lab generator `next_smooth_quartic_with_point`), `P0` from the generator | all rational points minus `P0`, plus `kappa` | lab baseline mechanism (brief's "generic quartic control") |
| `C_full` | generic C_{3,4}, `P0 = [0,0,1]` | all rational points minus `P0` (`kappa = 0`) | isolates the hyperflex from the automorphism |
| `P_full` | Picard, `P0 = [0,0,1]` | all rational points minus `P0` | **baseline** for H3 |
| `P_orbit` | Picard (same curve, same pair stream as `P_full`) | one column per size-3 orbit; fixed points and `P0` dropped (log 0) | **treatment** for H3 |
| `R` (rhocost) | Q, C, P at `p` in {2053, 4099, 8209} | n/a | per-operation `Pic` costs A (add), D (double), N (neg), R (`rho`, P only) for H1/H2 |

Controls:

- **Positive control (PC).** `C_full` at `p_small` in {349, 1381, 5479} against
  `p_big` in {1033, 4099, 16411}. The column count falls by about 3 by a known mechanism
  (field size). The instrument must see it in line trials. PC passes at a pair if the
  geomean ratio `trials(p_big) / trials(p_small)` lies within ±15% of the geomean ratio
  `(n_cols(p_big) + 10) / (n_cols(p_small) + 10)`.
- **A/A control.** `P_full` with solver seeds 1 and 2 on the same curves. It passes at a
  prime if the geomean of `T(seed 1) / T(seed 2)` lies in [0.9, 1.1]. This is checked at
  every `p >= 1033`.
- **Replay control.** `measure --p 1033` is run twice, and the two outputs must be
  byte-identical (the instrument is deterministic; records carry no wall time).
- **Negative control (wrong eigenvalue).** At the tiny tier, the orbit matrix built with
  the other root `lambda' = lambda^2 (mod r)` must have trivial kernel, while the matrix
  with the correct `lambda` has a 1-dimensional kernel proportional to the brute-force logs.
- **Identity-quotient control (unit test U7).** `P_orbit` run with the trivial orbit map must
  reproduce `P_full` counts exactly.

## 4. Variables held fixed

- Same counted `F_p` layer, F-evaluation cost rule, line algebra, sqrt table and stopping
  rule (`n_cols + 10` accepted rows) in every arm.
- `P_full` and `P_orbit` use the same curve, the same point order and the same pair stream
  (same RNG state). `P_orbit` stops earlier.
- LA: counted Wiedemann (Krylov `2n` terms, Berlekamp-Massey, reconstruction) on the square
  system of the first `n_cols` accepted rows, in every arm. At the measurement tier it is
  executed modulo the proxy prime `ell = 2^61 - 1`, and for `P_orbit` with
  `lambda_proxy = 1669582390241348315`, a primitive cube root of 1 mod `ell`. Krylov counts
  are data-independent, which unit test U5 and tiny gate T8 check.
- One thread, release build, the `plane_quartic_pic.rs` hash recorded in
  `raw/lab_source.sha256`.

## 5. Instances (explicit)

- **Tiny tier (correctness only):** `p` in {13, 19, 31, 37, 43} (all `= 1 mod 3`).
  - Picard: curve seeds 1..200 in order. Accept the first 2 seeds whose curve is smooth and
    whose `#J`, computed from point counts over `F_p`, `F_{p^2}` and `F_{p^3}`, has largest
    prime factor `r > p`, `r != 3`, `r^2` not dividing `#J`.
  - C and Q: curve seeds 1, 2.
- **Measurement tier:** `p` in {271, 523, 1033, 2053, 4099, 8209, 16411} (first prime
  `= 1 mod 3` above `2^k`, k = 8..14). Curve seeds {1, 2, 3} per family, solver seeds
  {1, 2}. Families: P (arms `P_full`, `P_orbit`), C (`C_full`), Q (`Q_full`).
- **PC primes:** {349, 1381, 5479}, family C, curve seeds {1, 2, 3}, solver seeds {1, 2}.
- **rhocost:** `p` in {2053, 4099, 8209} (the `bound_r` primes, all `= 1 mod 3`), families
  Q, C, P, curve seeds {1, 2, 3}. Per curve: 100 random classes, 99 adds, 99 negs,
  50 doubles, and 99 `rho` applications (P only).
- Curve generation rules, RNG seeds, `zeta` choice (smallest `g >= 2` with
  `g^((p-1)/3) != 1`) and the point ordering are fixed in SPEC section 4.

## 6. Correctness gates (all must pass before any metric is read)

Tiny tier:

- **T1** `#J` from point counts kills 8 random classes (`mul_scalar(x, #J) == 0`), and
  satisfies Hasse-Weil.
- **T2** The table of `<G>` has `r` distinct keys, and `[r]G = 0`.
- **T3** `lambda = log_G(rho(G))` satisfies `lambda^2 + lambda + 1 = 0 (mod r)`.
- **T4** `l(rho P) = lambda l(P)` for every non-fixed rational point, and `l = 0` at
  fixed points.
- **T5** `rho` on `Pic` (20 classes and 20 points): `rho^3 = id`,
  `x + rho x + rho^2 x = 0`, `rho(x + y) = rho x + rho y`,
  `rho(point_class P) = point_class(rho P)`.
- **T6** Every collected relation of every family verifies in `Pic`:
  `sum point_class(P_i) = kappa` (`= 0` for C and P; for Q, every row gives the same class).
- **T7** `P_full` and `P_orbit` rows annihilate the brute-force log vectors. The dense-GE
  kernel is 1-dimensional and proportional to the logs on at least 8 of 10 Picard curves
  (the rest "underdetermined" with the logs in the kernel). The wrong-`lambda` matrix has
  trivial kernel on every determined curve.
- **T8** Krylov counts modulo `r` and modulo `ell` are identical.

Measurement tier:

- **M1** For Picard, the counted enumeration's point set equals the uncounted brute-force
  set.
- **M2** The first 50 accepted rows of every run verify in `Pic` (as T6).
- **M3** `|N - (p + 1)| <= 6 sqrt(p)`.
- **M4** The Krylov counter equals `2n(n + w)` exactly.
- **M5** No run hits the trial cap (`60 x (n_cols + 10)`).

rhocost:

- **R1** `x + neg(x) = 0` for every measured `neg`, and `add` commutes on 10 samples.
- **R2** (reconciliation, Q family only) `A_mul + N_mul` lies within ±5% of the median
  `cantor_mul / group_ops` of the `Q_rho_neg` rows of
  `research/genus_ladder_bound_r_20261114/bound_r.jsonl` at the same `p`.

If a gate fails, the affected decision is reported as "gate failed" with the failing
records. Thresholds are not moved.

## 7. Decision rules (numeric, frozen)

**Part 1 (H1).** Let `A`, `N` be the mean FPM per add and per neg over all Q curves at a
prime, and `F_Q = (A + N) / (sqrt(2) A)`.

- `F_Q >= 1.10` at all three primes: **H1 supported**. The existing generic-quartic rows'
  matched rho is overpriced by `F_Q`. Rows affected:
  - `research/genus_ladder_20261114/RESULTS.md:60, 73, 95-99, 165`;
  - `research/genus_ladder_bound_r_20261114/RESULTS.md:29-35, 41-46`.

  Each row's IC/rho would be multiplied by `F_Q`, with the band verdict re-read against
  that.
- `0.90 < F_Q < 1.10` at all three primes: **no material move**.
- `F_Q <= 0.90` at all three primes: **negation walk legitimate**.
- Otherwise **inconclusive**.
- If R2 fails, Part 1 reports per-operation costs only, with "model not reconciled".

The GHS-weak rows (elliptic reference) are reported as unaffected.

**Part 1 (H2).** For Picard curves, with `A, N, R` the mean FPM per add, neg and `rho`
application:

- `S1 = sqrt(2) A`
- `S2 = A + N`
- `S3 = (A + 2R) sqrt(2/3)`
- `S6 = (A + N + 4R) / sqrt(3)`

Each is the ideal expected cost relative to the factor `sqrt(pi r / 4)`. Report
`argmin` and `min / S2` per prime. These are model numbers: ideal step counts times
measured step costs; fruitless cycles and walk overheads are not modelled.

**Part 2 (H3, primary).** `R_T(p)` is the geometric mean over the 6 (curve, solver seed)
pairs of `T(P_full) / T(P_orbit)`.

- **Supported:** `R_T >= 2.0` at each of `p` in {4099, 8209, 16411}.
- **Killed:** `R_T < 2.0` at each of `p` in {8209, 16411}.
- **Inconclusive:** otherwise, or if any of T1-T8, M1-M5, PC or A/A fails at a prime used.

**H3'**, at `p = 16411`:

- `trials(P_full) / trials(P_orbit)` in [2.5, 3.5]: "relation prediction held", else
  "missed".
- `LA_w(P_full) / LA_w(P_orbit)` in [7, 11]: "LA prediction held", else "missed".

**Distance to the matched rho (reported, not a gate).**

- Gap movement `G(p) = R_T(p) / (min(S1, S2) / min(S1, S2, S3, S6))` with the Picard
  rhocost values (prime 8209 values used for `p >= 8209`, otherwise the nearest rhocost
  prime). `G > 1` means the orbit IC closes distance to the correctly matched rho.
- Descriptive only: `T(P_orbit) / (sqrt(pi r_proxy / 4) min(S1, S2, S3, S6))` with
  `r_proxy = p^3`, labelled **proxy** (no `#J` at the measurement tier). It excludes
  the target's descent, so it is not a DLP cost.

## 8. Confounders and how each is isolated

| confounder | isolation |
|---|---|
| curves differ in point count `N` | H3 is paired on the same curve and pair stream |
| Picard F is sparse, so its F-evaluations are cheaper | affects only cross-family comparisons (H4), not H3. Reported per family |
| enumeration algorithm differs by family | only Picard enumeration is charged (table method, identical in both P arms; `P_orbit` pays 2 extra multiplications per orbit). C and Q enumeration is setup, uncharged, and their totals are not compared with P totals |
| LA modulus unknown at the measurement tier | proxy `ell`. Krylov counts are data-independent (U5, T8). BM counts are data-dependent and reported separately |
| LA weighting `g^2` | both weighted and unweighted ratios are reported. The decision uses weighted (lab convention) |
| duplicate and trivial rows | counted as trials in both arms, with separate counters |
| RNG / order effects | deterministic seeds; replay control; arms run in a fixed order with no shared mutable state |
| analyst | thresholds frozen here; `analyze.py` emits the verdict strings mechanically |
| rho model idealisation | S1-S6 are labelled model values. R2 reconciles the negation-walk step cost with the lab's executed rho |

## 9. Commands (executor runs these unchanged; lead dir `L = /home/user/crypto/research/g2plus_leads_20261116/L10_c34_picard_matched_rho`)

```
cd $L/code && CARGO_TARGET_DIR=$L/code/target cargo build --release --offline -j1 2>&1 | tee $L/raw/build.log
cd $L/code && CARGO_TARGET_DIR=$L/code/target cargo test --release --offline -j1 -- --test-threads=1 2>&1 | tee $L/raw/test.log
sha256sum /home/user/crypto/src/cryptanalysis/plane_quartic_pic.rs > $L/raw/lab_source.sha256
for p in 13 19 31 37 43; do $L/code/target/release/l10 tiny --p $p --out $L/raw/tiny.jsonl; done
for p in 2053 4099 8209; do for f in Q C P; do for s in 1 2 3; do $L/code/target/release/l10 rhocost --family $f --p $p --curve-seed $s --out $L/raw/rhocost.jsonl; done; done; done
for p in 271 523 1033 2053 4099 8209 16411; do $L/code/target/release/l10 measure --p $p --families P,C,Q --out $L/raw/measure.jsonl; done
for p in 349 1381 5479; do $L/code/target/release/l10 measure --p $p --families C --out $L/raw/pc.jsonl; done
$L/code/target/release/l10 measure --p 1033 --families P,C,Q --out $L/raw/replay_1033_a.jsonl
$L/code/target/release/l10 measure --p 1033 --families P,C,Q --out $L/raw/replay_1033_b.jsonl
cmp $L/raw/replay_1033_a.jsonl $L/raw/replay_1033_b.jsonl > $L/raw/replay_cmp.txt 2>&1; echo "exit=$?" >> $L/raw/replay_cmp.txt
python3 $L/code/analyze.py --raw $L/raw --bound-r /home/user/crypto/research/genus_ladder_bound_r_20261114/bound_r.jsonl --out $L/raw/analysis.json --tables $L/raw/tables.md
```

Each command must stay under 10 CPU-minutes and 2 GB (one thread). The largest,
`measure --p 16411`, is estimated at about 24 runs of 10-20 s each. If a repository hook
refuses a command, record the refusal verbatim in `$L/raw/refusals.txt` and continue with
the rest. Do not work around it.

## 10. What is reported

- Gate table.
- Per-operation `Pic` costs, `F_Q` and the H1 verdict with the affected file:line rows.
- S1/S2/S3/S6 and the H2 argmin.
- Per prime: `n_cols`, trials, yield, `w`, enumeration/line/LA FPM per arm; `R_T` with
  min/max and a percentile bootstrap 95% interval over the 6 pairs (seed 12345, 2000
  resamples, descriptive); the H3 verdict; the H3' checks; `G(p)`; the proxy distance.
- Negatives are reported plainly. A kill at 2x is a valid outcome.
