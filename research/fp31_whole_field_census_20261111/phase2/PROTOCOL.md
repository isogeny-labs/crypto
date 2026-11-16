# Phase 2 protocol: does Pollard rho cost depend on anything but r, across and within isogeny classes of F_{2^31−1}?

Status: **frozen before data** (2026-11-11T06:45Z), after the timing pilot of §8 and before any study data. Requested by the user on 2026-11-11 ("Yes run it all") after
the phase-1 census (`../REPORT.pdf`). Any later change is an amendment, appended at the end with its
reason and date. Data taken before an amendment stays in the record.

Classification (controlled-experiment skill): a **null-hypothesis characterization**, rho against rho on one
instrument. It is not an index-calculus result, makes no cross-method claim, and adds no leaderboard or
`benchmarks/README.md` row. All findings are [pcc:unverified] until registered.

## 1. Question

A generic algorithm sees only the group order and `Aut(E)`. Phase 1 gives the exact class invariants of every
trace. After normalizing by `√(πr/2)`, does Pollard rho's iteration count depend on any of the following?
- (a) the isogeny class, through class size `H`, conductor `f`, number of endomorphism rings or `|D_K|`;
- (b) the curve inside a class, at different volcano levels and reached through different `ℓ`;
- (c) the generator.

Preregistered expectation: no for (a), (b) and (c). A detected effect means either the instrument is
broken or the theory is wrong. Either is reported as found.

## 2. Instrument and metric

- `isoclass-rho` (`src/bin/isoclass_rho/`, repository commit pinned in the run manifest), unchanged.
- `walk` realises verified isogenies from a root curve to a stratified set of class members and carries one
  ECDLP instance. `measure --mode class` runs rho on those curves with controls. `measure --mode generators`
  runs rho on many generators.
- Rho configuration: the tool's default 32-adding walk with distinguished points.
- **Primary metric:** `c = iters / √(π n / 2)` per solve. Here `n` is the largest prime of #E, the subgroup
  rho runs in, which equals the census `r`. Iteration counts are exact counted units and are replayable from
  the recorded seeds. Wall time is descriptive only.

## 3. Population and selection (frozen rule)

- `fp31-class-census roots --seed 20261111 --draws 400000` draws `y² = x³ + ax + b` with `a, b` uniform in
  `[1, p−1]` from SplitMix64. It finds #E by BSGS and keeps a curve when exactly one multiple of a random
  point's order lies in the Hasse interval.
- A second stream, `roots --j0 --seed 20261112 --draws 20000`, gives the j = 0 control.
- `fp31-class-census select --per 16 --scale 8 --seed 20261111` walks the roots in draw order, one root per
  trace, and fills these strata first-come:
  - **S1**: prime order, one endomorphism ring (crater only).
  - **S2**: prime order, two rings.
  - **S3**: prime order, three or more rings.
  - **R** (r-scaling control): cofactor 4 to 64.
  - **J0**: the first `y² = x³ + b` root with cofactor at most 64 (Aut of order 6, since p ≡ 1 mod 3).
    For this p no j = 0 curve has prime order: the six twists have cofactors 13, 327, 5929, 45964, 80028
    and 342453 (counted from the j0 stream before freezing). So J0 is the t = 39115 class, with r = 165188041
    and D_K = −3, f = 48511.

  The result is 16 + 16 + 16 main classes, 8 R classes and 1 J0 class. Per-class seed: `20261111·1000 + index`.
- S1 classes have a single endomorphism ring, so their walk is horizontal only. That is the expected structure.

## 4. Per-class commands (frozen arguments, `run_phase2.sh`)

```
isoclass-rho walk    --curve p,a,b --ells 2,3,5,7 --radius 3 --max-vertices 400 --per-stratum 4 --hard 0 --seed S --out runs/<id>/walk
isoclass-rho measure --walk runs/<id>/walk/walk.json --mode class      --trials TC --max-vertices 6 --controls 2 --plants --folds --seed S --out runs/<id>/class
isoclass-rho measure --walk runs/<id>/walk/walk.json --mode generators --generators 10 --trials TG                    --seed S --out runs/<id>/gen
```
`TC` and `TG` are fixed in §8 from the timing pilot. A class whose walk fails is recorded as missing and is
not replaced.

## 5. Tests (computed by `fp31-class-census analyze`, frozen)

The primary family has 7 tests at Bonferroni α = 0.05/7, computed on the 48 main classes, `isogenous` arms:
- **T1** Kruskal–Wallis of `c` across classes, pooling all class-mode solves of each class.
- **T2–T5** Spearman correlation of the class mean `c` against `log2 H`, `log2 f`, number of rings and
  `log2 |D_K|`. The p-values come from 20,000 seeded permutations.
- **T6** Within class: each class's Kruskal–Wallis p across its measured curves. The 48 p-values are tested
  for uniformity on [0, 1] with one-sample KS.
- **T7** Generators: the same as T6, across the 10 generators of each class.

**Controls**, each reported as a ratio of pooled means to the root curve `v0` of the same classes, with a
delta-method SE:
- `plant-neg`: expect 0.707. This is the detector check: a planted effect of this size must be found.
- `plant-r4`: expect about 1.12 (Teske).
- `aa` replica: expect 1.
- `iso-control`, isomorphic models: expect 1.
- `J0:fold-aut`: expect `1/√6 ≈ 0.408`.
- **R** classes against the main classes, on normalised `c`: expect 1. The raw cost must scale as `√r`.

## 6. Decision rules

- **Null supported.** All of T1–T7 have p > 0.05/7, and `plant-neg` falls in [0.65, 0.76].
- **Detector failed.** `plant-neg` falls outside [0.65, 0.76]. No null is then claimed, and the instrument
  is investigated.
- **Effect suspected.** Any of T1–T7 has p ≤ 0.05/7. The effect is reported as suspected, not established.
  A preregistered follow-up on fresh seeds is then required, as in `isogeny_class_ic_yield_20261102` follow-up 1.
- **Power.** The per-solve coefficient of variation of rho is about 0.52. With `m` solves per class, the
  class mean has standard error about `0.52/√m`. The run reports the observed between-class SD against the
  SD expected from sampling alone.

## 7. Execution route and isolation (deviation from the default, recorded)

The lab default is isolab-eks (CLAUDE.md, AGENTS.md §9a). This session has no AWS identity. The GitHub
dispatch route runs only `ecbench.spec/v1` files from `main`, and `isoclass-rho` is not ecbench. So this
run executes locally on a 4-vCPU cloud container at isolation **L0**.

The decisive metric is the iteration count, which is exact and does not depend on host load, so L0 does not
weaken any test in §5. Wall times are descriptive. Resume checkpoints and S3 upload are unavailable without
AWS. Raw files are hashed into the benchmark record, and their upload is left to a session with AWS.

## 8. Trial counts (fixed from the timing pilot before any study data)

The pilot ran on a curve outside the study: the first prime-order root with draw > 100000 (draw 100006,
`a = 532875221`, `b = 504541723`, `t = −62813`). Its data are excluded from every analysis and are not
committed. Results:
- `isoclass-rho selftest`: 0 failures. Binary sha256 `4f0897306551c10bd3eebb065dc385aeae38101e81eedc76e5ec73a4917424d3`.
- `walk`: 16 measured vertices, 22 verified edges, 0.23 s.
- `measure --mode class --trials 10`: 21 arms (16 curves, 2 isomorphic models, A/A, 2 plants), 210 solves, 0.78 s.
- `measure --mode generators --generators 10 --trials 5`: 83 arms, 415 solves, 1.7 s.

Frozen: **TC = 200**, **TG = 40**. That is about 4,200 class-mode solves and about 3,300 generator solves per
class. With 16 curves × 200 solves, a class's mean `c` has standard error about 0.52/√3200 ≈ 0.9%. The
expected wall time is about 30 s per class and under an hour for all 57 classes.
