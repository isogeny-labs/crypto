# Protocol: the exact-count falsifier for lever 2 (frozen before the study rows)

Written 2026-11-01, before any row of `runs/`. A dry run on scratch output (seed 1, instances
discarded, disclosed in §9) preceded it.

## 1. Question and the claim each outcome licenses

`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §4 item 2 gives a falsifier for the lever "a factor base
with sub-`B`-degree membership": *count the solutions of the combined system over the closure for a toy
`p`; if the count is at least `B^{m−1}`, the lever is dead.* This study runs that count exactly.

For a base `X ⊂ F_p`, `|X| = B`, and a target `r ∈ F_p`, the closure count is

    c_m(r) = #{ (x_1, …, x_m) ∈ X^m, ordered : S_{m+1}(x_1, …, x_m, r) = 0 }.

`X` is split in `F_p`, so these are all the closure solutions. `r` ranges over every element of `F_p`,
twist targets included. `S_3(x,y,z) = (x−y)²z² − 2((x+y)(xy+a)+2b)z + ((xy−a)² − 4b(x+y))` and
`S_4(x_1,x_2,x_3,r) = Res_u(S_3(x_1,x_2,u), S_3(u,x_3,r))`, the resultant of the two quadratics.

| outcome | claim it licenses |
|:--|:--|
| a structured-family instance has `max_r c_m(r) ≥ B^{m−1}` | the literal falsifier fires for that family at that `(m, B)`, and nothing else |
| no structured instance fires, and the families match the null | the literal falsifier is not fired by any of the five candidate families at the tested sizes; it does not separate them from random bases |
| a family differs from the null on `Cmax` or coverage at `α_c` | that family has an excess or deficit of that metric at that `(m, B)`, with the effect size |
| the positive control does not fire or is not detected | the instrument cannot support a "nothing found" claim; the study is invalid |

Nothing here is a statement about the cost of finding a decomposition, or about the viability of the lever.

## 2. Claim classes

| claim | class |
|:--|:--|
| each `c_m(r)` of each instance, and each `Cmax`, coverage, firing flag | exact computation, verified by gates G1–G5 |
| a family differs from the null | comparative / statistical: controls in §5 |
| growth of `Cmax / B^{m−1}` with `p` | descriptive, no test |
| the lever's viability, any attack cost | out of scope |

## 3. Instrument, bases, sizes

- Instrument: `examples/exact_count_membership.rs`. `m = 2` solves the quadratic `S_3(x_i, x_j, ·) = 0` for every
  pair. `m = 3` finds the `F_p`-roots of the quartic `S_4(x_i, x_j, x_k, ·)` for every multiset (Cantor–Zassenhaus
  on degree ≤ 4), weighted by its orderings. Nothing is sampled. A tuple whose polynomial in `r` is identically
  zero counts for every `r` and is recorded (`all_add`).
- Gates, run on the study binary and recorded before the run: **G1** the fast count equals brute-force evaluation
  at every `r` (`m = 2, 3`, all seven families); **G2** `S_3` vanishes at `x(P_1 + P_2)` and `S_4` at
  `x(P_1 + P_2 + P_3)` for every point tuple of one curve; **G3** the polynomial form of `S_4` equals numeric
  evaluation and `S_4` is symmetric; **G4** the control reaches `B` at `m = 2`; **G5** at `m = 3` every `r ∈ X`
  carries at least the `3B − 2` trivial tuples (§4). A failed gate stops the study.
- Bases of size `B` (§4.2 candidates), each instance with its own random curve `y² = x³ + ax + b` over `F_p`:
  `subgroup` (a coset `g·μ_B`), `ap` (`a + jd`), `interval` (`0 … B−1`), `image` (a random cubic at
  `t = 0, 1, …`), `dickson` (`w + w⁻¹`, `w ∈ μ_{2B}`). Null `null`: a uniform `B`-subset of `F_p`. Positive control
  `smallmult`: `x(kP)`, `1 ≤ k ≤ B`, for a random `P` of order above `2B`.
- Sizes: `p` is the smallest prime `k·2B + 1` with `p ≥ B^m`, so `μ_{2B} ⊂ F_p^*` and the mean closure count is at
  most 1. `m = 2`: `B ∈ {128, 256, 512, 1024, 2048}`. `m = 3`: `B ∈ {16, 32, 64, 128}`.
- Instances per `(m, B)`: 40 per structured family, 40 for the control, 400 for the null. Seed `20261101`; the seed
  of an instance is a fixed hash of `(seed, m, B, family, index)` and is recorded.

## 4. Metrics

- **Primary: `Cmax`** = `max_r c_m(r)` over all `r ∈ F_p`, all solutions counted, and **coverage** = the fraction of
  `r` with `c_m(r) ≥ 1`.
- Threshold `B^{m−1}`; `fires` = `Cmax ≥ B^{m−1}`.
- Secondary, descriptive: `Cmax` over targets on the curve (`r³ + ar + b` a square); `Cmax_nt`, `Cmax` with the
  trivial solutions removed.
- Trivial solutions at `m = 3`: `P − P + R = R` makes `(x, x, r)` and its permutations solve `S_4 = 0` for every `r ∈ X`,
  `3B − 2` ordered tuples. They are solutions of the system as written, so the primary metric keeps them; `Cmax_nt`
  subtracts exactly `3B − 2` at each `r ∈ X`. No trivial solutions exist at `m = 2`.

## 5. Controls and decision rule

- **Null population:** a uniform `B`-subset of `F_p`, drawn on the same primes and curve distribution, matched on
  `B`, `p`, curve and the instrument.
- **Positive control `smallmult`:** at `m = 2` it must fire in every instance at every size (the count of
  `x(sP)` for `s = B + 1` is at least `B`); at `m = 3` it must reach `Cmax ≥ 0.5·B²` in every instance (the plain
  sums alone give `0.75·B²`); and its coverage must lie below the null's at `p < α_c`. Any miss invalidates the
  study.
- **A/A:** the null split by instance index into two halves of 200; the Mann–Whitney `p` of the halves on `Cmax` and
  coverage must be at least `10⁻³` at every size. A failure means the null is not stationary and the study is
  invalid.
- **Tests:** for each of the five structured families, each `(m, B)` and each of `Cmax` and coverage, a
  two-sided Mann–Whitney test of the 40 family instances against the 400 null instances (normal approximation with
  tie correction), with Cliff's `δ`. The number of tests is `2 × 5 × 9 = 90`; the family-wise level is `0.01`,
  so `α_c = 0.01/90 ≈ 1.1·10⁻⁴` (Bonferroni).
- **Verdict wording.** Per test: "within the null" (`p ≥ α_c`), "above the null" or "below the null" (`p < α_c`,
  by the sign of `δ`). Per family and `(m, B)`: the share of instances that fire.
- **Resolution and power.** With 40 against 400 the standard deviation of `U` is about 767 and `U − mean = δ·8000`,
  so the test reaches `α_c` at `|δ| ≈ 0.37` and detects `|δ| ≈ 0.46` with 80% power. Effects below that are not
  excluded. The test is attainable at `α_c`: the exact permutation `p`-value for fully separated samples is far
  below it.

## 6. Stopping rule and what does not count

Fixed instance counts and sizes; no run is extended or repeated after the data. Changing a size, `N`, a metric,
the null, a threshold, or whether trivial solutions are counted after the rows exist is a new, labelled protocol
version, and both are reported. The dry-run instances are not used.

## 7. Variable ledger

| factor | treatment |
|:--|:--|
| instrument (commit, binary SHA-256, `rustc -Vv`) | fixed, recorded in `commit.txt` |
| seeds | derived from `(seed, m, B, family, index)`; the same index maps to different curves per family |
| host, threads, order | the counts are exact integers and do not depend on them; `RAYON_NUM_THREADS` is recorded; family order is fixed by the job list and every instance is independent |
| curve | random per instance, from the instance seed |
| prime | fixed per `(m, B)`, shared by all families |

## 8. Route and runtime

AGENTS.md §9a sends x86 experiments to Iso Lab on EKS Spot. The container that runs this has no AWS access, so the
route is unavailable and the run is local. The claims are exact integer counts, do not use a clock and do not depend
on the host; `wall_seconds` in `summary.json` is operational only. Resume checkpoint: `.resume/RESUME.md`, pushed
to S3 before the run.

Pre-flight estimate (dry run, two threads, a four-thread job on the same four cores): `m = 2` takes under a second
per size up to `B = 256`; `m = 3` took 76 s for 120 instances at `B = 64`, about 0.6 s per instance, and the triples
grow eightfold per doubling of `B`. The `m = 3`, `B = 128` size is therefore about 5 s per instance, 640 instances,
roughly an hour on two threads.

## 9. Disclosures

- A dry run (seed 1, sizes 64–256 at `m = 2`, 16–64 at `m = 3`, 10 and 60 instances) preceded this protocol. It
  showed that structured families and the null have similar `Cmax` and coverage at those sizes, that the control
  fires, and that the `3B − 2` trivial solutions raise every `m = 3` maximum. The trivial-solution metric and gate
  G5 were added after it. The study instances use seed `20261101` and are new.
- The families differ in how much of the literature's construction they represent. `subgroup` is the Kummer tower's
  set `V`; `dickson` is the Dickson tower's `V` (`w + w⁻¹`), `ap` and `interval` the progression and small-`x`
  bases, `image` a random cubic image. The isogeny tower's `V` is not included.
- Source for the study's claims: `research/lever2_closure_20261031/REPORT.md` (the identity), `KN-FIND-007`,
  `EV-FBG-001`, `IDEA-20260801-021` (crypto-autoresearcher).

## 10. Outputs

`runs/m2/{rows.jsonl,summary.json}`, `runs/m3/…`; `REPORT.md`, `REPORT.pdf`, a figure of `Cmax / B^{m−1}` and coverage
against `B` for every family, null band and control; `commit.txt` and `gates.txt` (the gate output of the study
binary).
