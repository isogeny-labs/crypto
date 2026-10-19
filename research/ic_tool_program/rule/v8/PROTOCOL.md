# The rule's comparison at baseline v8, with a matched rho

**Declared 2026-10-19, before any row below.** Two arms of one binary, run
on the six sizes of ledger §23, 64 targets a size. This protocol, `run.sh`,
`history.json` and the binaries' hashes are committed before the first row.

**Everything rule v7 declared holds here** ([`../v7/PROTOCOL.md`](../v7/PROTOCOL.md),
and through it v6's, v3's, v2's): §23's six sizes, 64 targets and four A/A
repeats a size (408 rows a run), the arms' order inside one process, the
retries and stop rule, the claims, the pin against §23's `T01-R1`, each
row's logarithms against §23's, the reference check against the strong rho,
the accounting, class and non-claims. The rows under "What changes" are the
only differences.

## Question

#1978 narrowed the index calculus's cold distance to rho from 5.2–21.0×
(rule v3) to 3.43–11.51× (rule v7). Two things were left open by review:

1. **Is the rho arm matched?** The index calculus got GFNI and carry-less
   kernels; rho's canonicalisation did not. Rho cost 9.4–10.6 units a step
   against a canonical step of 4.2–4.9 (v7's `analysis.json`). At the
   canonical step the model gap is 6.1–22.4×.
2. **Does the gap still grow with the group?** v3 fitted `r^0.176` and v7
   `r^0.144`; the intervals overlap.

## What changes from rule v7

- **The binary** is v8's: `ic` built from local commit
  `370e19193f6f30bf04ca2e82a2c3626932dc069e` (the R06/R08/R11/R12 kernels of
  #2279 plus the matched rho), SHA-256
  `3500e56bcb8d62caaa733e35deb0e7cb5d622ab19058dc1cffac00770c582982`.
  `icprog` `86d9f0b76046de380e49f4c9cfc5d431d33f97ceb0a9cf969b1e8b056797883d`,
  `isolated_bench` `1bc76f0a599ceaa7a379255d838307a55229085574244534e1ebcffa4f09b410`,
  `koblitz_rho_fixture`
  `22617a9971c1548f1f82c719aff512e981aa60b20eae98b2dc6af1561c8760cd`.
  The index calculus code is R12's, unchanged by the matched rho.
- **The matched rho.** `ParallelRho`'s per-step canonicalisation (basis
  change, least rotation, the Frobenius power of `x` and `y`) is done
  sixteen lanes at a time: GFNI affine maps take `x` and `y` to normal-basis
  coordinates, the least rotation and its smallest shift `t` come from
  funnel shifts compared under the window's top `n` bits, `y` is rotated by
  `t`, GFNI takes both back (`FrobeniusCanon::frobenius_reps`). The
  group additions already use `pclmulqdq` field multiplication and one
  batched inversion a round; the walk is the strong signed-Frobenius walk
  with look-ahead, short-cycle escape and distinguished points.
  **It is the same walk:** a unit test compares every counter and the
  logarithm with the per-point path for 8–64 lanes, and another compares the
  triples for every `n` from 2 to 63 including tied rotations.
- **Two runs of the same binary, same targets, same host:**
  - `v8`: rho as above (the default);
  - `v8c`: `KIC_RHO_CANON=scalar`, the per-point tables, which is rho as at
    rule v7. This is the same-binary control.
  `v8` runs first, then `v8c`. The environment is set by `run.sh` and is in
  each comparison's `process` string, so it is in every claim's envelope.
- **Read-out.** `icprog rule matched --comparison v8` pairs the two runs by
  target (see "Analysis").
- **The reference check** is run for each, against `koblitz_rho_fixture strong`
  built from the same commit.
- **The host** is the Emerald Rapids container of rule v7 (family 6 model
  207), kernel build `6.18.44-fc-v139`, 4 vCPUs here (rule v7 recorded 3);
  `runs/host.json` of each run says.

### What was seen before this protocol

A microbenchmark (`examples/rho_step_profile.rs`, one `taskset` pair of
cores, 7 repetitions, median; not a rule row) at the six sizes:
`walk_batched / walk_scalar` ns a step = 0.61, 0.58, 0.63, 0.59, 0.62,
0.59 (n = 47, 57, 41, 53, 59, 61). The two walks took the same number of
steps at every size. Nothing else was timed with the matched rho.

## The ≥ 50-bit curve (not run, and why)

The request was for at least one new Koblitz curve with `log2 r ≥ 50`,
registered before timing.
- **Over `n ≤ 63` there is none.** `examples/koblitz_roster_scan.rs` lists
  every `K_a` over `GF(2^n)`, `31 ≤ n ≤ 63`: the largest prime subgroup is
  47.21 bits (`icv1-f2m61-t158598901-ab42b6c5`, already §23's largest).
- **Over `64 ≤ n ≤ 81` four reach 50 bits**, registered here as the
  candidates, in the order of preference: `icv1-f2m71-t48653080717-2cafaea3`
  (K_0, 52.29 bits), `icv1-f2m73-tm184271214331-9d25cc67` (K_0, 56.26),
  `icv1-f2m73-t184271214331-83f9f5e9` (K_1, 62.22) and
  `icv1-f2m79-t420247971347-dc997574` (K_1, 69.69). Also listed:
  `icv1-f2m65-t12115575461-d3d0406c` (48.57), `icv1-f2m71-tm48653080717-f25c4638`
  (49.06). (`roster_n64_n81.txt`; the scan was stopped after `n = 81`, and `roster_n31_n63.txt` is the `n ≤ 63` list.)
- **None can be timed by this pipeline.** `ic price --single-target` on a
  `K_0` over `GF(2^71)` stops with "no single-word Koblitz instance for this
  curve": the pipeline's field, its rho and its pair table are one 64-bit
  word. The wide path (`FastPoint128`, the N83 programme) has no
  single-target `price`, no wide `ParallelRho`, and a different cost
  account. Building them is not part of this comparison.

So the **first** kill condition below cannot be tested by a measurement at
50 bits. It is answered by the fit's extrapolation, labelled a prediction.

## Prediction (from v7's analysis and the microbenchmark; not from a rule row)

If matched rho's online interval is `k` times the control's (`k` = 0.55–0.65
from the microbenchmark) and nothing else moves, cold IC/rho at the six
sizes (log₂ r = 36.6, 38.0, 39.0, 44.3, 44.5, 47.2) is

| k | 36.6 | 38.0 | 39.0 | 44.3 | 44.5 | 47.2 |
|:--|--:|--:|--:|--:|--:|--:|
| 0.65 | 4.68 | 5.37 | 6.93 | 10.59 | 9.54 | 17.52 |
| 0.60 | 4.94 | 5.65 | 7.42 | 11.41 | 10.27 | 18.93 |
| 0.55 | 5.23 | 5.97 | 7.98 | 12.38 | 11.12 | 20.59 |

(v7: 3.43, 3.97, 4.74, 7.03, 6.37, 11.51.) The OLS slope of ln(cold ratio)
on ln r is then 0.160–0.167 (v7's figures give 0.144). **The matched gap is
predicted larger than v7's at every size, and the slope predicted below
v3's 0.1756 by less than the fit's own interval** (v3's 95% interval is
0.137–0.214; v7's 0.092–0.196). The index calculus's own time is predicted
the same in `v8` and `v8c` to within the A/A band.

## Decisions, fixed now

**What "the narrowing survives" means.** The cold ratio is
`(IC set-up + IC online) / (rho set-up + rho online)`. v3 and v7 used the
same rho, so their ratio of ratios is the index calculus's own speed-up,
whatever rho is: 1.52–1.89×. That narrowing is therefore not at issue here.
What a matched rho changes is the *level* of the gap and, if rho's gain
depends on size, its slope. The report states both:
- the level: cold ratio of `v8` and of `v8c`, with 95% target-bootstrap
  intervals, and their paired ratio;
- the distance remaining by the success criterion is the `v8` figure.

**Kill conditions** (a result that makes the claim "#1978 closed the gap's
growth" false). Any of:
1. *Gap at ≥ 50 bits at or above the fit's ~15×.* Not measurable (above).
   Recorded as: not tested; the fit's extrapolation to 50 bits, with the
   band through the centroid from the slope's interval, is printed.
2. *The slope does not fall below v3's 0.1756.* **Triggered if the matched
   run's six-point slope is ≥ 0.1756.** If it is below, it is called
   "fell below, within the interval" unless the matched slope's 95%
   interval (t over the six sizes) lies wholly under 0.1756, which is
   called "fell significantly".
3. *The matched cold ratio is not above the control's at every size.* A
   matched rho that is not faster would mean the kernel did nothing.

If 2 triggers: **record that #1978 lowered the constant only.** If it does
not, the slope claim is limited to what the interval says.

## Analysis (`icprog rule matched`, `icprog rule analyse` on each run)

Every figure is on checked rows only; a row that fails its check stops the
read-out. Intervals are 95% percentile bootstraps over targets, 10,000
resamples, seeded.
- Per size: walks agree (same rho steps, same logarithms, same target hash,
  row by row); cold ratio `v8`, `v8c`; paired `v8/v8c`; rho online time
  `v8c/v8`; the index calculus's online time `v8/v8c` (identical code: its
  distance from 1 is the drift and noise of the measurement).
- Across sizes, for each run: the slope of ln(cold ratio) on ln r with the
  t interval over six sizes (the rule's), a bootstrap over targets within
  each size, each size left out in turn, and a fit on ln r and n together
  (six points, three parameters: descriptive). **n and log₂ r are not
  independent in these six curves** (n = 41 has log₂ r = 39.0, n = 57 has
  38.0); the index calculus's cost depends on the field and rho's on the
  group, so a slope against log₂ r is partly a slope against n. These
  curves cannot separate them. The report says so.
- The fit's extrapolation to 50 and 60 bits is a prediction, labelled so.
- `analyse` of each run gives the rule's own figures (online speed-up,
  break-even, the Bernstein–Lange model, rho at the canonical step), so the
  v7 tables are reproduced like for like.

## The pricing band (AGENTS.md §5a)

Both arms are timed in one process on one isolated core: every native
operation either arm performs is inside its time, so no counter is
unpriced on either arm and the band at 0, 0.1 and 1 GAE per native
operation gives one figure. The model readings (rho at the canonical step,
Bernstein–Lange) are priced in counted units and are reported as
lower-bound readings; they rank nothing.

## Class (labels for every claim in the report)

- the batched canonicalisation, the kernel and its cost: **engineering**;
- IC cold time unchanged by it: **no change**;
- the narrowing of 1.52–1.89× since v3: **engineering** (index calculus
  only), already established by #1978;
- the widening of the distance to a matched rho: **accounting** (the same
  index calculus against a cheaper reference), never an index-calculus loss;
- the slope: **measurement**, neither an improvement nor a regression.
Nothing here is an **advance**.

## Stop

Stopped if a row fails its check, a logarithm differs from §23's, the pin
fails, or the reference check finds the comparison's rho slower a step than
the strong walk (that size is reported as not run). A contended row re-runs
under the runner's rule; only the clean attempt counts.

## Cost

About 90 minutes a run (rule v7: 21:04–22:34 UTC), two runs, plus the
reference checks.

## Non-claims

No statement about ECC2K-130 or any curve past `n = 61`; nothing about
precomputation rho; one host class; the six curves' recipes were tuned
at v3 and are not retuned; no claim that the index calculus beats rho.

## Commands

[`run.sh`](run.sh) runs, in order, for `v8` then `v8c` (with
`KIC_RHO_CANON=scalar`): `manifest`, `all`, `reference`, `claims`,
`analyse`, then `icprog rule matched --comparison v8`.

## Amendment 1 (2026-10-19, before any timed row): the pin

The first attempt (card `CARD-RULEV8-20261019-1`) stopped at the pin, as
"Stop" declares, with no timed row run (the pin is untimed;
`runs/attempt-1-stopped-at-pin/`). At all six sizes the pin differed in one
output, `counts`, and in no other: `status`, both recovered logarithms,
`rho_counts`, `all_verified` and `ic_and_rho_agree` equal §23's `T01-R1`.
The difference is two counters, `row_ops` (498 and 852 at `n = 47`) in the
index calculus's pass counts, which the 2026-09-30 reference output predates:
main's drift added the counter after §23. It is a change of accounting
schema, not of what the binary decides.

**Change** (the harness only; the `ic` binary is unchanged): for rule v8 and
v8c the pin requires every counter the earlier output has to match exactly
and lists the counters only the new output has
(`counters_added_since_the_earlier_output` in `pin.json`). Frozen
comparisons keep the strict test. `icprog` is rebuilt from the commit that
makes this change: SHA-256
`15e4a84352563b59efe2eda2aa28a997ba5e3034f22699ce218de0888d1c16e7`. The
other hashes above stand. The cards are recompiled for the new commit.
