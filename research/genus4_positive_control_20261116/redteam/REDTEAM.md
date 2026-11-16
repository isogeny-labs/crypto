# Red-team review: genus-4 positive control, arm w against rho at p = 8209

Reviewer: Red Team (2026-11-16). This review challenges the claim and does not
decide it. The Coordinator and the Validator decide and record the outcome.
Every number below is `[pcc:unverified]`, the producer's and this review's alike.

## What was reviewed

- Commit `b87f8ad56d` on `claude/hyperelliptic-index-calculus-td1fdn`. The
  protocol was frozen at `bce0eb2148` (15:41:00 UTC), `analyze.py` was
  committed at `16b409c747` (15:41:54) and the results at `77d5750f9a`.
- `run.jsonl` comes from the artifact store: sha256 `e0f7050d…a2efa32f`, 78,049
  bytes, 144 runs. The manifest `.artifacts.json` at HEAD pins the same hash.
- `frozen.sha256` matches the four frozen sources at HEAD and at
  `bce0eb2148`, and `ic_boundary.rs` is unchanged between those commits.
- The binary (`binary.sha256`, in a session scratchpad) was not rebuilt or
  rerun, per the dispatch.
- The claim under review: arm w is below the matched negation-map rho at
  w = 0, 0.1 and 1, with the bootstrap upper bound below 1 at each price.
  At w = 1 the reading is 0.528 [0.382, 0.754].

Method:
- I read the code paths: `run_ic_ext`, `run_rho`, `rho_canon` and the tuned
  walk, plus `build_rows`, `peel`, `mat_vec`, `wiedemann_coord` and
  `berlekamp_massey`.
- I reimplemented the analysis independently in `redteam/check.rs`, a
  standalone Rust file with no crates and its own RNG. To run it:
  `rustc -O --edition 2021 check.rs -o check && ./check ../run.jsonl`
  (`check.rs` sha256 `693e4509…`).
- I checked gate G1 at p = 8209 with `jq` against
  `../genus_ladder_bound_r_20261114/bound_r.jsonl`.

The reimplementation reproduces the producer's cell: w/R at w = 1, p = 8209 is
0.528 [0.382, 0.751]. Across ten bootstrap seeds (B = 2000) the upper bound
ranges from 0.738 to 0.764, so the bootstrap seed and RNG do not matter.

## Findings, most severe first

### F1. The cheapest-rho rule was not checked at w = 1. Major; unsupported (unchecked).

**Evidence**
- `docs/ic/SUCCESS_CRITERIA.md:56-67` defines the rho reference "at each weight
  of the band" as "the **cheapest** measured configuration among the registered
  rho variants … plain, negation, …". Line 327 lists "a reference other than
  the cheapest registered rho at each weight" under what does not count.
- The study measures only `RhoWalk::negation()`
  (`src/cryptanalysis/genus_ladder.rs:1994-2005`,
  `src/bin/genus_ladder.rs:386-406`). `RhoWalk::plain()` exists at
  `src/cryptanalysis/ic_boundary.rs:1738-1746`.
- Under the band, the negation walk pays one priced native operation per step
  (`native` equals `group_ops` to within 1e-5). At w = 1, half of rho's total is
  that operation.
- Measured constants: negation rho takes 0.989·√(πr/4) steps with 1.034 group
  operations per step. Plain rho has E = √(πr/2), 1.41 times as many steps, and
  no canonicalisation: `rho_canon` returns before counting when the classes are
  trivial (`ic_boundary.rs:2430-2432`).

**Which walk is cheapest at w = 1 depends on how plain rho's per-step native
work is priced:**

| pricing of the per-step native work | cheapest at w = 1 | IC w / cheapest at w = 1 |
|---|---|---|
| (a) counters as emitted: plain has 0 per step, negation 1 (canonicalisation) | plain (wins for every w > 0.389) | **0.761**, analytic; no measured interval |
| (b) this instrument's floor, `native = max(canonicalisations, steps)` (`genus_ladder.rs:2020-2025`): plain 1 per step, negation 1 per step | negation | 0.528 [0.382, 0.751], as reported |
| (c) ecbench style, hash priced on both walks: plain 1 (hash), negation 2 (canonicalisation + hash), estimated | plain | 0.380, analytic |

- The w = 0 and w = 0.1 cells are unaffected: negation is cheapest there under
  (a), (b) and (c), at 0.004 and 0.099.
- Floor (b) gives plain rho the same one-per-step charge as negation rho's
  canonicalisation. Negation therefore pays nothing for its canonicalisation
  beyond what any walk pays, and that is the only reading in which negation
  stays the cheapest walk at w = 1. The floor is not documented as a pricing
  decision anywhere in the protocol.
- Under (a), suppose a measured plain-rho mean landed on its expectation.
  Rayleigh-like run lengths have a CV of about 0.52, so roughly 18 or more
  plain-rho runs would be needed before the 97.5% bound falls below 1, and that
  ignores the between-curve term. A 12-run plain arm would most likely leave
  the cell **unresolved**.

**Classification.** The claim is not shown wrong; the cheapest-rho check is
missing. Under (b) the reported cell stands. Under (a) it is unsupported
(0.76, unmeasured). Under (c) it gets stronger. The frozen decision rule
(`PROTOCOL.md:78-87`) names "R" and is met as written. The gap is in the
wording "below rho across the §5a band", which in the lab's vocabulary means
the cheapest registered rho under one stated pricing.

**Cheapest test.** Add an `R_rho_plain` arm, `rho_reference_walk(...,
RhoWalk::plain())`. Run it on the three p = 8209 instances with at least 8
seeds per curve: about 150 s per run single-threaded, so about 20 minutes on 3
threads. Emit its partition-hash count, state which per-step pricing applies to
both walks, and recompute min(negation, plain) at each weight. Theory gives the
point estimate without the run: 0.761 under (a).

### F2. Unpriced work is listed incompletely and priced asymmetrically; the w = 1 reading depends on the convention. Major; unpriced.

**Evidence**
- `PROTOCOL.md:59-65` and `analyze.py:52-56` price IC as `group_ops + w·(rel_mul
  + la_mul)` and rho as `group_ops + w·native`.
- Arm w at p = 8209 also emits `rel_add` = 3.905e7 and `la_add` = 6.006e7.
  These 9.91e7 modular additions are 1.92 times its priced native
  multiplications (5.147e7). `tables.md` lists none of them.
- AGENTS.md §5a, line 1218: "List every unpriced counter for every arm,
  references included, with its count."
- The instrument's own convention, `ic_control.rs:13-19`: "`add` is reported
  beside it as a sensitivity". The genus-3 control's frozen protocol required
  that sensitivity (`research/ic_positive_control_20261112/PROTOCOL.md:46`,
  "sensitivity: mul + add"). This protocol dropped it without comment.
- Rho's priced `native` is canonicalisations (`genus_ladder.rs:2020-2025`).
  They contain no field multiplication: `rho_canon`,
  `ic_boundary.rs:2433-2448`, says "no field work is tallied either way". A
  multiplication-only rule therefore charges rho for operations that are not
  multiplications.
- Rho also performs unpriced work that `run_rho` never emits (`RhoOut`,
  `genus_ladder.rs:1971-1984`):
  - a partition hash per step (`ic_boundary.rs:2864-2865`); §5a, lines
    1207-1209, names partition hashes as rho's unpriced work;
  - two `addmod`s per step (`ic_boundary.rs:2858-2859`);
  - two mod-r negations, up to four mod-p negations and two keys per
    canonicalisation;
  - about 18 ring compares per step (`window_word_ops`,
    `ic_boundary.rs:2893-2949`; counted in the tally but not emitted here).

**Readings at w = 1, p = 8209** (`check.rs`; rho counters it does not emit are
estimated per step)

| convention | IC / negation rho | IC / cheapest rho (plain priced by counters, F1 (a)) |
|---|---|---|
| producer: IC multiplications; rho canonicalisations | 0.528 [0.382, 0.751] | 0.761 (plain, analytic); 0.528 under the F1 (b) floor |
| multiplications only on both arms (rho native = 0) | 1.057 [0.765, 1.499] | 1.057 |
| IC multiplications + additions; rho as emitted (genus-3 sensitivity) | 1.543 [1.117, 2.192] | 2.221 |
| IC multiplications + additions; rho canonicalisation + hash + 2 addmods per step (est.) | 0.630 | 0.630 |
| producer + rho partition hash per step (§5a's own list; est.) | 0.356 | 0.380 |

- Break-even: pricing IC additions at 0.465 of a multiplication, with rho
  unchanged, puts the w = 1 cell at 1.
- At w = 0.1 every convention stays below 1. The largest upper bound is 0.403,
  for multiplications plus additions against rho as emitted.
- At w = 0 every convention gives 0.004.

**Interpretation, labelled as such.** The w = 1 reading moves between 0.38 and
2.2 depending on which non-multiplicative native operations each side
charges. The two readings above 1 are:
- one that charges IC's additions but not rho's (asymmetric, so not a fair
  reading, though it is the genus-3 control's sensitivity convention);
- the strict multiplication-only reading. It is symmetric but breaks §5a,
  which names rho's canonicalisations as work to price. It gives 1.057, an
  interval that straddles 1.

Every reading that is both symmetric and §5a-compliant is below 1 (0.38 to
0.76 as point estimates). So the w = 1 margin is real under the lab's rules,
but it is not shown by the committed evidence:
- the IC additions are not listed;
- rho's additions and hashes are not emitted, so the matched add-inclusive
  reading rests on per-step estimates here;
- the margin over a multiplication-only reading comes entirely from rho's
  priced per-step canonicalisation.

§5a: an arm that names a counter but does not emit it "cannot be ranked";
report it as a lower bound and record the gap.

**Cheapest test**
1. Emit counters from `run_rho`: partition hashes, walk `addmod`s, and the
   canonicalisation's negations and keys. Most of these have a closed form per
   step and per look-ahead attempt, so one run at p = 2053 can confirm the
   formula.
2. Report the multiplication + addition sensitivity for both arms, as the
   genus-3 control did.
3. Restate the w = 1 cell under one symmetric convention.

No rerun of the IC arms is needed: `rel_add` and `la_add` are already in the
JSONL.

### F3. The calibrated reading is missing. Major (a required reading is absent); expected to favour IC.

**Evidence**
- AGENTS.md:1226-1242 (owner, 2026-11-12) says field-arithmetic counters "as
  well as the band" are priced at a per-host `w_cal`. The order is to be
  reported at w = 0, w_cal and 10·w_cal, with w_cal recorded together with its
  method and host.
- The study reports no `w_cal`. FPM is a different reading: it converts to
  field multiplications and does not measure time per mulmod against time per
  group addition.
- Borrowing the genus-3 control's weights at p = 8209 (w_rel 0.0047, w_la
  0.0130 per genus-3 Cantor addition) gives IC / rho of 0.0098 at w_cal and
  0.067 at 10·w_cal, with rho's canonicalisations priced at 0. Informational
  only: different curve and genus, and nothing pinned.

**Note for whoever measures it.** At p = 8209 every r is above 2^50
(2.26e15, 4.52e15). `mulmod` therefore takes the u128 `%` path
(`ic_control.rs:600-615`), not the f64 fast path that the genus-3 calibration
timed, so a mod-r mulmod here is slower than the borrowed w_la assumes.
Measure it at these r.

**Cheapest test.** Run a best-of-N timing on this host:
- one `mulmod` mod p;
- one `mulmod` mod r with r > 2^50;
- one genus-4 `JCurve::cantor` addition.

Then report w = 0, w_cal and 10·w_cal against the cheapest rho.

### F4. Statistics: robust under the producer's pricing, with three targets and no A/A control. Minor.

What holds, against measured negation rho:
- Leave-one-curve-out upper bounds are 0.879 (drop 363), 0.832 (drop 393) and
  0.737 (drop 407).
- With the Holm-level one-sided quantiles of the full data, q99.17 = 0.815 and
  q99.5 = 0.850.
- Replacing measured rho by its theoretical expectation removes rho's sampling
  noise and gives 0.520.
- No curve multiset exceeds 0.590.

The interval excluding 1 is not a sampling accident.

Limits:
- **Three targets, not twelve.** The target is fixed per curve
  (`src/bin/genus_ladder.rs:357-358`: `instance_from(…, *curve_seed)`), so the
  12 runs at p = 8209 are 3 targets × 4 solver seeds. The bench record's
  `"n": 12` overstates independent targets. SUCCESS_CRITERIA T1 item 1 asks for
  at least 32 targets.
- **The curve-level bootstrap is degenerate.** With 3 clusters there are only
  10 distinct multisets; the curves-only interval is [0.444, 0.590]. The width
  comes almost entirely from rho's within-curve seed variance, and the
  between-curve component cannot be estimated.
- **Per-target losses.** In 2 of the 12 paired runs IC costs more than rho
  (paired ratios range from 0.234 to 2.248), because rho got lucky on curve 407
  seed 1 and curve 393 seed 1 (0.26 and 0.33 of E). The claim is about expected
  cost, and the write-up should say so.
- **No A/A rho control** (SUCCESS_CRITERIA:137). A pseudo-A/A built from the
  data, rho seeds 1-2 against seeds 3-4, gives 0.912 [0.354, 1.802] at
  p = 8209. It contains 1, but the interval is ±2×, which shows how little
  four seeds per curve resolve.

**Cheapest test.** Add independent targets per curve (a target seed distinct
from the curve seed), at least 8 per curve, for IC and for both rho walks.

### F5. Component table: the unit-T linear-algebra saving is mostly an accounting effect, and the restart cap never engaged. Minor; relabelling hazard.

**Evidence**
- `mat_vec` (`ic_control.rs:1186-1203`) charges ±1 entries as additions only.
- In arm w every raw relation has T coefficient r − 1 (`genus_ladder.rs:1473-1477`),
  so the T column of the core is mostly ±1 and moves from the priced to the
  unpriced counter.
- At p = 8209:
  - w/v LA is 0.705 counting multiplications only and 0.933 counting
    multiplications and additions;
  - w/c LA is 0.233 counting multiplications and 0.317 counting both;
  - v/c is 0.331 against 0.340, so coordinate Wiedemann's gain is real under
    both readings.
- "Capped restarts": the cap is `min(restart, max(64, √r/16))`
  (`genus_ladder.rs:1367-1371`). At p = 8209 that is
  max(64, √r/16) = 2.97e6 to 4.20e6, and it is at least 1.9e5 at p = 2053, so
  the restart is 4096 in every run. In this study arm w is v plus the unit-T
  start and jumps; the cap was inert.

**Cheapest test.** Add the LA multiplication + addition column to the
components table and mark the cap as inactive at these r. No rerun is needed.

### F6. Provenance and process. Minor.

- **The analysis script was not frozen before data.** The bench record says the
  run started at 15:41:14 UTC (`benchmarks/records/20261116T1540Z-genus4-positive-control.json`);
  `analyze.py` ("analysis script (frozen rules)") landed 40 s later. Its
  formulas match `PROTOCOL.md` §5, and F4 shows the independent
  reimplementation and other bootstrap seeds agree, so I found no impact.
- **Python generates the results.** `analyze.py` produces the research
  results, against AGENTS.md "Implementation language: no Python" (research
  result generation).
- **The binary is not durable.** It sits in a session scratchpad, so a
  validator must rebuild from `2ca7ece552` (the bench record says
  `commit_dirty: false`).
- **Local run on a shared 4-vCPU host** (declared deviation). Counts are
  seed-deterministic, so wall time is a practicality note only.
- **`run.log` is 0 bytes.** The driver writes only JSONL on success.
- **RESULTS.md G2 wording is wrong for rho.** "`[t]G = T` twice … all four
  arms": rho certifies once (`genus_ladder.rs:2007-2010`) and is checked
  against the planted value by the driver (`src/bin/genus_ladder.rs:398`).
  The certificate cannot be replayed from the data, because the recovered `t`
  is not in the JSONL (the r-bound validator flagged the same).
- **No §15 external comparison is named** for genus-4 Jacobian DLP (rho or IC
  implementations, published records), and no "external comparison: not run"
  line.
- **Misleading problem tag.** The bench record files the run under
  `prime-field-chained-transfer-no-advantage`. A genus-4 positive control is
  no evidence about that transfer; the tag should not be read as such.

### F7. Framing and scope: what may and may not be claimed.

**May be claimed, after validator replay and with F1 and F2 resolved or
caveated:**

> On three synthetic genus-4 hyperelliptic Jacobians over F_8209 (h ≤ 2,
> #J ≈ 2^52, 3 targets × 4 solver seeds), double-large-prime index calculus
> with the parity prefilter, coordinate Wiedemann and the unit-T walk cost less
> than the matched negation-map rho:
> - 0.004 at w = 0 and 0.099 [0.072, 0.141] at w = 0.1;
> - 0.528 [0.382, 0.754] at w = 1, under multiplication-only native pricing of
>   the IC arm and canonicalisation pricing of rho;
> - 0.011 in field-multiplication equivalents;
> - about 0.01 in wall time on a shared host.
>
> This is a positive control of the lab's instrument. It reproduces the known
> asymptotic advantage of index calculus on genus ≥ 3 hyperelliptic Jacobians
> (Õ(p^{2−2/g}) against rho's p^{g/2}; Thériault 2003, Gaudry–Thomé–Thériault–
> Diem 2007).

**May not be claimed:**
- "below rho at all three prices of the §5a band" without naming negation rho
  and the pricing convention (F1, F2);
- any ECDLP statement, or any T0–T3 tier. It is a Jacobian DLP, not a transfer
  from an elliptic curve; SUCCESS_CRITERIA:147-152 says the genus-3 control
  "would be T1 at neither reading", and the same applies here;
- generalisation to larger p, other genera, or curves with larger cofactors;
- that the instrument "detects an IC win under the band" in general. At
  p = 2053 and 4099 the w = 1 cells straddle 1.

The w = 1 trend (1.018 → 0.708 → 0.528) is an observation on three primes
against negation rho only.

## Checks that found nothing

- **No leak of the planted secret.**
  - `t_secret` appears only in the driver's `ok` flags
    (`src/bin/genus_ladder.rs:398, 449`) and in tests.
  - IC relations use `rhs = h·ca` and `tcoef = −1` for the unknown h·log T
    (`genus_ladder.rs:1472-1480`).
  - Restarts draw `a` uniformly from [1, r) (`:1390`).
  - The answer is h⁻¹·X (`:1539-1545`).
  - It is certified by two independent scalar multiplications against
    `inst.target` (`:1550-1559`).
  - 144 of 144 runs are certified and equal to the planted value.
- **The rho reference is efficient.**
  - 0.989·√(πr/4) steps, averaged over 12 runs.
  - Group operations are 1.034 per step, with look-ahead additions, cycle
    escapes and set-up charged as group operations.
  - The negation map works, and fruitless 2-cycles are handled
    (`ic_boundary.rs:2870-2877, 2893-2945`).
  - For random genus-4 curves over a prime field there is no further
    automorphism, so no faster generic walk exists beyond the plain/negation
    choice in F1.
- **Both arms count group operations by the same rule.** Both use the same
  Cantor routine: 393.7 field multiplications per group operation in IC
  against 393.0 in rho.
- **IC multiplications look completely counted.** I checked:
  - the character table, factor-base evaluation, `ModU::mul` and `mul_xpa`,
    `disc_character`, `roots_of`, `vrem`, `vgcd`, `vdiv` and the v evaluation;
  - `mulmod` for the right-hand side, BM, `mat_vec`, `wiedemann_coord`, and the
    h-inverse and BM inversions (2·bits(r) each).
- **IC's uncounted word work is small.**
  - Per trial: key, mix64 and 2 `addm`.
  - Elsewhere: Vec allocations, the large-prime HashMap, row sorts, the subset
    clone and Fisher–Yates.
  - Total: about 2e6 operations. Rho's uncounted per-step word work is of order
    1e9, so the producer's convention is conservative toward IC on word work.
- **Gate G1 holds at p = 8209.** The c and R rows for seeds 1-2 are identical
  to `bound_r.jsonl` (`jq`, 12/12). These are rows the r-bound validator did
  not replay, so this is reproducibility within one lineage, not independent
  validation.
- **No overflow.** Lesson `L-61e5ea1f9e0d` applied. At p = 8209 the mod-p
  products are below 2^28. Mod-r products go through u128 because r > 2^50.
- **No selection effect.**
  - The instances are the r-bound study's first admitted seeds under h ≤ 2,
    not chosen by outcome.
  - P4 named p = 8209 and arm w before the data.
  - The added rho seeds 3-4 raised rho's mean slightly (seeds 1-2: 46.6e6;
    seeds 3-4: 51.1e6), against IC's interest.

## Verdict

**Survives:** the arm-w IC is below rho at p = 8209 for these three instances
at w = 0 (0.004) and w = 0.1 (0.099 [0.072, 0.141]).
- This holds under every pricing convention examined (largest upper bound
  0.403) and against the cheapest rho, which at these weights is the negation
  walk.
- The positive-control conclusion is robust: IC on these genus-4 Jacobians
  costs about 1% of rho in field multiplications and in wall time.

**Survives only with qualifiers:** "below rho at all three prices". The
statement holds as "below the negation-map rho at w = 0, 0.1 and 1 under the
frozen pricing". At w = 1 that means:
- the IC is priced by its multiplications only;
- rho is priced at one native operation per step;
- the reading is 0.528 [0.382, 0.754].

The reimplementation reproduces it. It is statistically robust: leave one
curve out gives an upper bound of at most 0.879, the Holm-level q99.5 is 0.850,
and against rho's theoretical expectation it reads 0.520.

It is not robust as the unqualified lab statement "below rho across the §5a
band":
1. The cheapest-rho rule was not checked. Plain rho is the cheapest walk at
   w = 1 unless this instrument's `max(canonicalisations, steps)` floor is the
   pricing. Analytically IC/plain is 0.76 and unmeasured; with ecbench-style
   hash pricing it is 0.38 (F1).
2. The w = 1 value depends on which non-multiplicative native operations each
   arm is charged. Symmetric, §5a-compliant readings put it at 0.38 to 0.76.
   The strict multiplication-only reading gives 1.057 [0.765, 1.499],
   unresolved. Charging IC's additions alone gives 1.54. The frozen data cannot
   settle the matched reading: rho emits no addition or hash counters, and the
   protocol dropped the instrument's own mul + add sensitivity (F2).

There is no blocking finding. Nothing shows the cell wrong, but the w = 1 part
rests on pricing choices that the committed evidence neither lists in full nor
tests.

**Required caveats on any citation:**
- "against negation-map rho; the cheapest-rho check (plain rho) not run";
- "IC native work priced by multiplications only (9.9e7 IC additions and
  rho's partition hashes and additions unpriced); rho priced at one native
  operation per step";
- "the w = 1 margin is convention-dependent: 1.057 [0.765, 1.499] under a
  multiplication-only reading of both arms";
- "3 synthetic curves and 3 targets, p ≤ 8209, one shared host, no calibrated
  reading, no independent replay";
- "positive control of a known genus-3+ effect; not an ECDLP result and not a
  tier claim".

**To make the w = 1 statement hold without qualifiers:**
- measure plain rho and state one per-step pricing for both walks (F1);
- emit rho's addition and hash counters, and report the symmetric mul + add
  sensitivity (F2);
- add w_cal (F3);
- add independent targets (F4).

Status: `[pcc:unverified]`. This review does not change any status.
