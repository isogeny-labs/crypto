# Protocol addendum: a 45-bit replica of the prime-field class (P2)

Version 1, frozen 2026-10-18 before any measurement of the run it governs.
This is a new experiment directory; the committed protocol and run in
`research/isogeny-class-rho/` are not changed. Instrument: `isoclass-rho`
(`src/bin/isoclass_rho/`, pull request #2174), the same binary and the same
rules R1 to R6 as `research/isogeny-class-rho/PROTOCOL.md` (sha256
`81c3232bdab481244324101d10e2d50199da1990ceba10c8dc66cc206286409e`), which this addendum
repeats where it is needed and otherwise inherits unchanged.

## 1. Question and claims

**Question.** The first run's four classes have `n` of 33 to 40 bits, small
enough that whole classes could be measured. Does the same null (no effect
of curve, endomorphism-ring stratum, volcano level, isogeny degree, kernel
field or generator on generic rho cost) hold when `n` is about 64 times
larger in iteration count? This replica carries one prime-field class to a
45-bit group order and repeats the class-mode measurement and a reduced
generator sweep.

**Null (theorem).** As in the first protocol: an isogeny of degree prime to
`n` is an isomorphism on `<P>`, so the expected cost is `sqrt(pi n / 2)`
(`x 1/sqrt(1 - 1/r)` for the `r = 32` adding walk) on every curve.

**Claims this run can license.** "No curve, stratum, level, degree, kernel
field or generator of P2 left the preregistered band at n of about 2^45",
and the size scaling of the pooled mean. Nothing about index calculus, any
deployed curve, binary or Koblitz classes at this size, or a second host
(see section 6).

## 2. Target

One class, one root curve, identity from `isoclass-rho search --bits 46
--want 3,5,7 --seed 31 --count 3` (third draw; the record is
`runs/<run>/search/prime46-want-3-5-7-seed31.jsonl`), chosen before any
measurement because its conductor `315 = 3^2 * 5 * 7` equals the first run's
class P1 and so gives the same twelve endomorphism-ring strata:

| id | field | root curve | n | class |
|:--|:--|:--|:--|:--|
| P2 | `F_p`, `p = ROOT_P` | `a = ROOT_A`, `b = ROOT_B` | `ROOT_N` (ROOT_NBITS bits), `#E = ROOT_COF * n` | `D = 315^2 * (-416214539)`, `h(D_K) = 8332`, class size 2,624,580 |

## 3. Design (inherited, scaled)

Walk, from the repository root, executed once:

```sh
R=research/isogeny-class-rho-n45/runs/RUN
target/release/isoclass-rho walk --curve ROOT_P,ROOT_A,ROOT_B --ells 2,3,5,7,11,13,17,19 --hard-ells 23,29,31,37,41,43,47,53,59,61 --hard-from 3 --radius 3 --max-vertices 5000 --per-stratum 3 --hard 4 --sources 2 --seed 20261018 --out $R/P2
```

Measurements (seed 20261018 as in the first run):

```sh
B=target/release/isoclass-rho
$B measure --walk $R/P2/walk.json --mode class      --trials 100 --controls 3 --plants --folds --seed 20261018 --out $R/P2/class
$B measure --walk $R/P2/walk.json --mode generators --generators 20 --max-vertices 2 --trials 10 --controls 0 --seed 20261018 --out $R/P2/generators
$B measure --walk $R/P2/walk.json --mode class      --trials 5 --controls 3 --plants --folds --seed 20261018 --out $R/P2/replay
```

The class mode has three isomorphic or representation controls, an A/A
replica, a planted negation fold (expected 0.707) and a planted 4-adding walk
(expected 1.137), exactly as in the first run. There is no shift-matrix
mode in this replica (budget).

## 4. Decision rules (inherited, fixed before data)

R1: per-curve two-sided z against the pooled rest, Bonferroni over all arms
of the family; R2: Kruskal-Wallis and one-way ANOVA across the measured
curves at alpha = 0.05, with the variance ratio of curve means against the
single-solve prediction reported beside them; R3: pooled mean against
`sqrt(pi n / 2) * 1.016`; R4: controls, A/A and plants against their expected
values; R5: stratum, level, distance, edge degree and kernel field pooled as
in the first run; R6: replay of the first five seeds must reproduce the
recorded iteration counts exactly. Verdict wording is fixed: "within the
control distribution" or "more extreme than the family".

Added, descriptive only (not a decision rule): the ratio of this class's
pooled mean iteration count to the first run's P1 pooled mean, compared with
`sqrt(n_P2 / n_P1)`.

Power (unchanged, since the single-solve coefficient of variation is
`0.52` at any size): about 18 % per curve, 6.5 % per stratum, 2 % class-wide
at 100 seeds.

## 5. Budget and stopping

Estimated 4 to 5 thousand solves at about 3 to 4 seconds each on four
threads: about one hour for the class mode. If total wall time passes four
hours, the run stops and is reported as partial with what finished.

## 6. What this run does not do

It is one class at one larger size, on the same host as the first run
(Intel Xeon @ 2.10 GHz, four threads, no isolation: level L0). It is not a
second-host measurement; that is a separate re-run of the first run's class
mode on another VM (`second-host/`, a data-only branch), whose wall-time rows
are L0 as well.
