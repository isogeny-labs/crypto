# F2 protocol: L06 re-freeze, odd-l rational Lagrangian census with an exact chi

Frozen before any decision data, 2026-11-17, by the F2 coordinator. Hashes of PILOT.md,
this file and SPEC.md are in `frozen.sha256`. SPEC.md section 1 lists the sha256 of every
source file of the instrument, so the code is frozen with it. Everything here has the
standing `[pcc:unverified]`. This is not a benchmark: no leaderboard row, no
`benchmarks/` row, no speed claim, and no wall-time metric. All metrics are exact
counts and fractions with recorded seeds.

Paths are relative to this directory,
`research/g2plus_followups_20261117/F2_L06_refreeze_rerun/`. Yesterday's lead is
`research/g2plus_leads_20261116/L06_odd_ell_isogeny_census/` ("L06"). It is read only and
never edited. Its mathematics (L06 `DERIVATION.md` sections 1-4, re-checked by the L06
assessor in `ASSESSMENT_RERUN.md` section 3) is used unchanged and is not repeated here.

## 0. What changed from L06, and corrections to the brief

1. **Gate G1 is redesigned, not relaxed.** L06's G1 required at most 2% "ambiguous"
   draws and failed at 3.75% (9/240; 8 at p = 101, 1 at p = 257). L06's 2% was a rate
   gate calibrated at p = 101, below the smallest census prime. I chose brief option
   (ii): ambiguity is resolved exactly, so that every chi-requested draw ends in one of
   two outcomes. Either it gets a unique chi, or it is an instrument `error`. The
   ambiguity limit is removed because the state no longer exists. It is not raised.
   The new G1 is zero-tolerance (section 5). Why (ii) and not (i), "gate only on
   disagreement, report the ambiguous fraction": under (i) the population would be
   defined by the instrument's success. In L06, J-side ambiguity cannot hit a
   prime-order curve, but twist-side ambiguity can. The twist search window holds about
   `15p` congruent candidates, so a twist with a non-cyclic part of order of about
   `p/30` or more (at p = 257, for example, `(Z/3)^2`) gives several pairs even when
   `#J` is prime. That correlates with `chi mod 3` and so could bias `F_3` on PRIME.
   L06 PROTOCOL G8 claimed that ambiguous draws "cannot be prime order". That holds only
   for the J side, and the claim is corrected here. Under (ii) the bias cannot arise
   (section 5, proof under G8).
2. **The cascade (SPEC.md 4.6).** Stage A is L06's BSGS on J and its twist, unchanged.
   If stage A does not return exactly one pair, stage B computes `N_2 = #C(F_{p^2})` by
   enumeration, which fixes `s2`. One unknown is then left (`s3`). Stage B finds every
   `n` with `nD = 0` in the Weil window `|s3| <= 20 p^{3/2}`, then filters them with 8
   fresh classes on J and 8 on the twist (twist order `= chi(1) + chi(-1) - n` stays
   fixed once `s2` is known). If more than one candidate is left, stage C computes
   `N_3 = #C(F_{p^3})` by enumeration, which fixes chi outright. Stage C runs only for
   `p <= 1031`, where it costs at most about 3 CPU-minutes per draw. At p = 2053 and
   p = 4099 a draw that stage B leaves undetermined is `unresolved`. It is proved below
   (G8) that an unresolved draw is never prime order. In the pilot at the full study
   size, stage B was needed on 13 of 17,031 chi-requested draws (12 at p = 257, 1 at
   p = 521) and stage C on none (PILOT.md).
3. **Fresh sample.** The new seed tags are `0x4C26` (decision census), `0x4C46`
   (G1 gate draws) and `0x4C36` (pilot). L06 used `0x4C06`. Bits 48-63 of
   `curve_seed = tag ^ (p << 24) ^ i` equal the tag, because `p < 2^13` and
   `i < 2^24` (asserted in code). So no new seed equals any L06 seed. Gate G10 also
   checks it explicitly against the L06 index ranges read from L06 `raw/`, and checks
   that no new curve `f` equals any L06 curve `f` at the same `p`.
4. **Seed-stream independence is now checked, not assumed** (assessor concern). Each
   stream is SplitMix64 from `seed ^ X`, with `X` one of coefficient (0), order
   `0xD1B5...`, resolution `0x2545...` and G2 `0x8CB9...`. SplitMix64 state after `j`
   calls is `seed + j*gamma` (mod 2^64, gamma odd), so a stream occupies the positions
   `pos(seed) + 1 .. pos(seed) + n` with `pos(s) = s * gamma^{-1} mod 2^64`. The
   instrument records `n` per stream per curve. Gate G9 proves that no two streams in
   the run share a state, and so share no output. In particular the G2 annihilation
   classes are never drawn from the order-search or resolution streams of any curve.
5. **Replay rebuilds from source** in a fresh target directory and must reproduce every
   output byte for byte (section 8, R-commands).
6. **ext_deg, corrected.** The brief calls ext_deg "the degree of the field of
   definition of the kernel points (13 or 26)". It is the least `k` with
   `T^k = 1 mod h`. The points of `ker h(pi)` are defined over `F_{p^k}`. `k` is exactly
   their field degree when the minimal polynomial of `pi` on `V_h = ker h(pi)` equals
   `h`, for example when `chi mod 3` is squarefree. Otherwise it is an upper bound.
   At `l = 3` the possible values follow from algebra (derivation in S5 below): for
   `p = 1 mod 3`, `{13, 26}`, and 13 iff `h(0) = 2`; for `p = 2 mod 3`, 13, except 3
   when `chi = (T-1)^3 (T-2)^3 mod 3`. So "13 or 26" is not the full list.

## 1. Hypothesis and exact claim

H1 (unchanged from L06). For odd `l in {3, 5, 7}`, a non-negligible share of genus-3
hyperelliptic Jacobians `J/F_p` (`y^2 = f(x)`, `f` monic of degree 7) with `#J(F_p)`
prime satisfy the criterion: there is a monic cubic `h in F_l[T]` with
`h h^* = chi mod l` and `gcd(h, h^*) = 1`, where `h^*(T) = T^3 h(p/T)/h(0)`. By L06
DERIVATION section 1, `ker h(pi)` is then an `F_p`-rational Lagrangian of `J[l]`, the
kernel of an `F_p`-rational `(l,l,l)`-isogeny.

Kill criterion (main session, unchanged): H1 is killed if fewer than 1% of prime-order
curves fire, for every `l in {3, 5, 7}`.

**What "supported" would mean.** In the fresh sample, at least the stated share of
prime-order genus-3 hyperelliptic Jacobians over these five primes satisfy a
*sufficient* condition for an `F_p`-rational `(l,l,l)` kernel. **What it would not
mean.** The isogeny is not computed. Its codomain is not computed and could be a
hyperelliptic Jacobian, a plane-quartic Jacobian, or a decomposable or non-Jacobian
principally polarised abelian threefold. Descent of a codomain curve to `F_p` is not
examined. The cost of evaluating the isogeny is not measured. Nothing moves the
hyperelliptic index-calculus exponent (`q^{4/3}`, `[pcc:unverified]`) or the distance to
rho (`q^{3/2}`). A "supported" is not evidence that a transfer to a quartic Jacobian
exists. The census also does not find all rational Lagrangians: the criterion is only
sufficient (`undetermined` class, section 2).

## 2. Quantities counted

For a curve with chi determined (`status = ok`) and `l in {2, 3, 5, 7}`, these are the L06
definitions, computed by the same code (SPEC.md 6): `fire_l`, `class_l` in
`{fire, none, undetermined}`, `n_h_l`, and `ext_deg_l` (least `k` with `T^k = 1 mod h`,
minimum over valid `h`).

Populations, per `p` and pooled over the five `p`. A curve is a draw `(tag, p, i)`.
Rule against double counting: a curve enters a population at most once. If two draws
at the same `p` give the same `f` (probability about `n^2/p^7`, which is negligible),
only the one with the smaller `i` counts (G10 reports the duplicate count).
- ALL: `status = ok` draws in the U block.
- PRIME: `status = ok` draws anywhere in the scanned range with `#J` prime.
- C2: U-block `ok` draws with `#J = q` or `#J = 2q`, `q` prime (secondary).
- ODD: U-block `ok` draws with `#J` odd (for A2).

There is no linear algebra and no relation collection, so no rank condition applies.
The stop rule is a count of PRIME draws, and it does not depend on any criterion
outcome.

**Primary metric:** `F_l = #(PRIME with fire_l) / #PRIME`, pooled over the five `p`,
with a Wilson 95% interval (`z = 1.959964`).

**Unit.** No cost is measured, so FPM is not used. Every metric is a count or a
fraction of counts. Instrument work (stage B and C invocations, candidate counts, RNG
draws per stream) is recorded per draw as counts.

## 3. Instances

- Primes `p in {257, 521, 1031, 2053, 4099}`, as in L06, with
  `p mod 3 = 2, 2, 2, 1, 1`, `p mod 5 = 2, 1, 1, 3, 4` and `p mod 7 = 5, 3, 2, 2, 4`.
- Curve rule: `f = x^7 + c6 x^6 + ... + c0`, with `c0, ..., c6` drawn in that order by
  `uniform(p)` from `SplitMix64(curve_seed(tag, p, i))`,
  `curve_seed = tag ^ (p << 24) ^ i`. Decision census: `tag = 0x4C26_0000_0000_0000`.
  G1 gate draws: `tag = 0x4C46_0000_0000_0000`. Pilot: `tag = 0x4C36_0000_0000_0000`
  (pilot data are never decision data). Non-squarefree `f` are recorded and skipped.
- U block: indices from 0 until `U_p` squarefree draws, with `U_p = 3000` for
  p = 257, 521, 1031 and `U_p = 1000` for p = 2053, 4099. P stream: the following
  indices, with chi computed only for irreducible `f` (`#J` odd iff `f` irreducible, so
  no prime-order curve is skipped; G5 checks it on every ok row). Stop when PRIME for
  that `p` reaches 120, or at draw index 40,000. The pilot at exactly these sizes
  stopped at `prime_target` at every `p` (PILOT.md), so about 600 PRIME are expected.
- Fallback (the only one): if a census command hits `timeout 900`, rerun that `p` once
  with half its `--u-count` and `--prime-target 60`, and record the timeout verbatim in
  DEVIATIONS.md.

## 4. Arms and controls

- **A3, A5, A7** (test arms): the criterion at `l = 3, 5, 7` on ALL, PRIME and C2.
- **R (baseline / null):** the GSp_6(F_l) multiplier-coset reference, unchanged from L06
  (200,000 samples per `(l, m)`, seed `0x4C06_5500... ^ (g << 32) ^ (l << 16) ^ m`). The
  sampler is deterministic, so it reproduces L06's reference, which is not census data.
  It is the rate that equidistribution of Frobenius predicts. It is a comparison only
  (S4), not a threshold.
- **Positive controls.** (a) G7: the genus-1 run of the same sampler must match the
  exact `(l - 1 - s) / (2(l - 1))` within 4 standard errors in every cell. (b) A2,
  `l = 2`: on ODD, the derivation predicts `fire_2` on 100%, no pair-type stable
  kernel, and exactly 2 stable "other" Lagrangians among the 135. These are theorems
  (L06 DERIVATION section 4, checked by the L06 assessor). A miss means the instrument
  or the derivation is wrong. (c) S5: the `l = 3` ext_deg predictions.
- **A/A and negative controls.** (a) G1 computes chi twice, by the cascade and by
  table-based enumeration (a separate implementation, `enumchi.rs`), and requires
  identical results on every draw and on every resolution path (stages A+, B and C
  forced). (b) Replay: a fresh build from the frozen sources must reproduce every raw
  output byte for byte. (c) G4: two independent tests of the criterion (brute force
  over `l^3` cubics, and factorisation) must agree on every curve and every reference
  sample.
- **A2 (`l = 2` arm, kept).** On ALL and ODD: `fire_2`, whether a pair-type stable
  kernel exists, and the count of stable Lagrangians among the explicit 135, by type,
  from the `J[2]` table (G6).

## 5. Correctness gates (all are computed by `analyze.py` before any metric is read)

Each threshold below passed on the pilot (PILOT.md), at the study's instance sizes where
a rate is involved. No gate is a rate gate any more.

- **G0** `cargo test --release` in a fresh target directory: `test result: ok. 15 passed`,
  with no `FAILED` or `panicked` line. The 15 tests are SPEC.md 11.
- **G1 (two chi methods, all three resolution paths)**, gate tag, `p = 257`, the first
  squarefree draws. `--force auto`: 200 draws. `--force B` (stage A skipped): 40 draws.
  `--force C` (stages A and B skipped): 20 draws. Required on every file: every draw
  `status = ok`, `chi_census == chi_enum` (table enumeration of `N_1, N_2, N_3`), G2
  annihilation true, every `C` draw `resolved_by = n3`, and no `B` draw resolved by
  `bsgs`. Required: 260 of 260.
- **G2 (annihilation)**: on every `ok` census row, 3 nonzero classes on J (from the G2
  stream) are killed by `nJ`, and 3 on the twist by `nJtw`. Required: 100%.
- **G3 (Weil)**: `s1^2 <= 36p`, `|s2| <= 15p`, `s3^2 <= 400 p^3` on every ok row.
  Required: 100%.
- **G4 (criterion tests agree)**: as L06, on every census `(curve, l)` and every
  reference sample. Required: 100%.
- **G5 (mod-2 structure)**: as L06, on every ok row. Required: 100%.
- **G6**: the `J[2]` table has 135 Lagrangians, 105 pair type, and the pair rule holds
  on all 15 cycle types.
- **G7**: as L06 (similitude exact on every sample; genus-1 `|z| <= 4` in all 12 cells).
- **G8 (completeness)**: across all five census runs, `error = 0`; `unresolved = 0` at
  `p <= 1031` (where stage C makes `unresolved` impossible by construction); every run
  writes its `run_summary`. At `p in {2053, 4099}`, an `unresolved` count above 0 does
  not fail G8. It is reported beside the ALL, C2 and ODD tables of that `p`, because
  it cannot touch PRIME. *Proof:* if `N = #J` is prime, every nonzero class `D` has
  order `N`. Stage A's J window has width `2B + 1 < (sqrt(p) - 1)^6 <= N` (checked:
  2,154,021 < 11,533,611 at p = 257, and the gap grows with p), so its only candidate
  is `N`. If stage A fails on the twist side, stage B's window is narrower still
  (`2 floor(20 p^{3/2}) + 1 < 2B + 1`), so `n = N` is again its only candidate. The
  true twist order always passes the twist filter, so stage B returns exactly one
  candidate. A prime-order draw therefore never reaches stage C or `unresolved`.
- **G9 (stream disjointness)**: the SplitMix64 position intervals of every coefficient,
  order, resolution and G2 stream used in the five census runs and the three G1 files
  are pairwise disjoint (the same curve's stream in two G1 files counts once).
  Every recorded `f` is regenerated from its seed. Required: 0 overlaps and 0
  mismatches.
- **G10 (freshness)**: no decision or G1 seed equals an L06 seed or a pilot seed, and
  no decision or G1 `(p, f)` equals an L06 or pilot `(p, f)`. Covered: L06 census
  indices `0 .. draws-1` at each p, L06 gate-enum indices at p = 101 and 257, and the
  pilot census ranges (draws 9453, 9446, 11863, 10582, 11794 at the five p, which also
  cover the pilot gate-enum indices 0..199 at p = 257), all regenerated from their seed
  rules. Required: 0 and 0. The duplicate-`f` count within the run is reported.

If any of G0-G10 fails, the verdict is **void (instrument)**. Every gate is applied by
`analyze.py`, and the verdict string is computed, not typed.

## 6. Decision rule (frozen; on the pooled PRIME population, all five `p`)

`n = #PRIME` pooled. `F_l` and its Wilson 95% interval `[lo_l, hi_l]` are as in section 2.

1. Any of G0-G10 failed: **void (instrument)**.
2. `n < 300`: **inconclusive (sample)**.
3. `F_l < 0.01` for every `l in {3, 5, 7}`: **killed**.
4. `lo_l >= 0.01` for at least one `l in {3, 5, 7}`: **supported**.
5. Otherwise: **inconclusive**.

The thresholds are those of L06 and are not moved. L06's census numbers are not
decision data, and nothing here is pooled with them.

## 7. Secondary readings (reported; they never change the verdict)

- S1-S3: the A2 predictions on ODD (section 4).
- S4: per `(p, l)`, `z = (F_obs - F_ref) / sqrt(se_obs^2 + se_ref^2)` against R (ALL
  against the unconditional coset, PRIME against `charpoly(1) != 0`). The count of
  cells with `|z| > 3` is reported.
- **S5 (new): field degree of the `l = 3` kernel on firing PRIME curves.** Histogram of
  `ext_deg_3`, per `p`, pooled, split by `p mod 3` and by whether `chi mod 3` is
  squarefree. The valid `h` are recomputed independently in `analyze.py`, and
  mismatches with the recorded `n_h`/`ext_deg` are counted (required 0; a mismatch
  voids S5 only). Frozen predictions, derived: over `F_3` with `q = p mod 3`, a valid
  `h` that is an irreducible cubic has `ord(T)` dividing 26 and not 1 or 2, so 13 or
  26, and 13 iff `T` is a square in `F_27`, iff `N(T) = -h(0)` is a square in `F_3`,
  iff `h(0) = 2`. Since `h^*(0) = q^3/h(0)`, for `q = 2` exactly one of `h, h^*` has
  constant term 2, so the minimum is 13. For `q = 1`, `h(0) = h^*(0)`. A reducible
  valid `h` needs a non-self-reciprocal factor and a coprime partner. For `q = 1` every
  linear factor is self-reciprocal (`a = 1/a` in `F_3`), so `h` is an irreducible cubic.
  For `q = 2` all three monic irreducible quadratics are self-reciprocal, so the only
  reducible case is `h = (T-1)^3` or `(T-2)^3`, that is `chi = (T-1)^3 (T-2)^3`, with
  `ext_deg = 3` (an upper bound there, since `h` is not squarefree). Predictions: at
  p = 2053 and 4099, `ext_deg_3 in {13, 26}`; at p = 257, 521 and 1031, `ext_deg_3 = 13`
  except in that special case, where it is 3. Violations are counted. The table sets
  the extension `F_{p^13}` or `F_{p^26}` over which the kernel points must be handled
  to compute the isogeny. That cost is not evaluated here.
- S6: per `p`, counts of `resolved_by` (`bsgs`, `n2`, `n3`) and of `unresolved`, and
  per-`p` `F_l`, class shares, `n_h` and `ext_deg` histograms for `l = 3, 5, 7` on PRIME
  and C2 (as L06).

## 8. Confounders and how each is handled

- Instrument-defined population (the L06 failure): removed by exact resolution (G1, G8).
  The PRIME population is every prime-order curve in the scanned range (proof in G8).
- Twist-side ambiguity correlated with `chi mod l`: impossible on PRIME (G8 proof).
- Seed reuse or stream overlap between search and verification: G9 and G10.
- Small-`p` equidistribution: S4 only, never the verdict.
- Multiplier mix (`p mod l`): reported per `p`. The pooled rate is a mixture over the
  five residues and is labelled as such.
- Model choice: only odd-degree monic models. Even-degree models and twists are outside
  the population, except that each twist is used for its order.
- Sufficient-only criterion: `undetermined` shares are reported.
- Analyst freedom: the verdict is computed by `analyze.py`, frozen with SPEC.md.
- Coordinator has seen L06 rates: the thresholds are L06's, unchanged, and the
  decision data are fresh draws.

## 9. Exact commands (run from this directory, one thread each)

Before any run, verify `sha256sum -c frozen.sha256` and the source hashes in SPEC.md 1
(`sha256sum -c SOURCES.sha256`, a file whose content is quoted verbatim in SPEC.md 1).

```
mkdir -p raw
(cd l06r && cargo build --release --offline --target-dir target 2>&1) | tee raw/build.log
(cd l06r && cargo test --release --offline --target-dir target 2>&1) | tee raw/test.log
B=./l06r/target/release/l06r
$B lagr2 --out raw/l2_lagrangians.json
$B gsp --samples 200000 --out raw/gsp_reference.json 2> raw/gsp.log
$B gate-enum --tag gate --p 257 --count 200 --force auto --out raw/gate_enum_p257_auto.jsonl 2> raw/gate_enum_p257_auto.log
$B gate-enum --tag gate --p 257 --count 40 --force B --out raw/gate_enum_p257_B.jsonl 2> raw/gate_enum_p257_B.log
$B gate-enum --tag gate --p 257 --count 20 --force C --out raw/gate_enum_p257_C.jsonl 2> raw/gate_enum_p257_C.log
for pu in 257:3000 521:3000 1031:3000 2053:1000 4099:1000; do
  p=${pu%%:*}; u=${pu##*:}
  timeout 900 $B census --tag decision --p $p --u-count $u --prime-target 120 \
    --draw-cap 40000 --out raw/census_p$p.jsonl 2> raw/census_p$p.log
  echo "p=$p exit=$?" >> raw/exit_codes.txt
done
python3 -I analyze.py raw > raw/summary.json
sha256sum raw/* > data.sha256
```

The gate files and the census may run concurrently as separate single-thread
processes (at most 2 at once). Outputs do not depend on order. `cargo build` must start
from an empty `l06r/target` (delete it first if present; the coordinator's pilot build
is not reused).

Replay (assessor). Fresh build, then byte comparison:

```
mkdir -p replay
(cd l06r && cargo build --release --offline --target-dir ../replay/target_fresh 2>&1) | tee replay/build.log
(cd l06r && cargo test --release --offline --target-dir ../replay/target_fresh 2>&1) | tee replay/test.log
R=./replay/target_fresh/release/l06r
# the same commands as above with $R in place of $B and replay/ in place of raw/
for f in l2_lagrangians.json gsp_reference.json gate_enum_p257_auto.jsonl gate_enum_p257_B.jsonl \
         gate_enum_p257_C.jsonl census_p257.jsonl census_p521.jsonl census_p1031.jsonl \
         census_p2053.jsonl census_p4099.jsonl; do cmp raw/$f replay/$f && echo "$f identical"; done
```

The replay passes if every listed file is identical. Expected runtime (pilot): census
runs of 21 s to 183 s; G1 `auto` 676 s; `B` 128 s; `C` 104 s; `gsp` 103 s.
Memory was not measured in the pilot; the largest allocation is the 17 MB
square table of `enumchi.rs` at p = 257 (gate-enum only). If a
repository hook refuses a command, record the refusal verbatim in DEVIATIONS.md and do
not work around it.

## 10. Outputs

`raw/` holds the files above (formats in SPEC.md 9). The five census JSONL files are
about 3 to 5 MB each (pilot sizes), above the 50 KB guideline. They are per-curve data,
needed for the replay and the audit. The executor keeps them in `raw/` and does not
commit them; `data.sha256` records their hashes. Where they are kept long term is
for the main session to decide. The committed summaries are `raw/summary.json`
(about 50 to 100 KB) and RESULTS.md. The executor writes RESULTS.md
with the gate table first, then the verdict string from `raw/summary.json`, then S1-S6.
Any departure from this file goes in DEVIATIONS.md. Per-relation rows are not dumped
into markdown. The census JSONL files are raw data, and RESULTS.md summarises them.
