# Protocol: the factor-base line search on the JV sieve route (frozen 2026-11-02)

Frozen before any arm-H, arm-T or arm-U run. `results/FROZEN.sha` records the SHA-256 of this file,
`run.sh`, `results/rule.txt` and the binary, and a UTC timestamp taken from the clock (no git). The only
runs that exist when it is written: none. If a threshold below proves unsuitable the outcome is
reported as failed, not changed.

**Scope.** Weak class only: full rational 2-torsion, `#E = 4l`, the genus-3 cover over `F_{p^2}`, never
prime order. **Class of claim:** a measured, paired comparison of two settings of one existing algorithm
(the JV12 §3.2 sieve route, `jv_sieve::run_cover_sieve_dlp_on`), on the same instances, in the same
unit. **Labels:** *measured* = read from a record; *derived* = computed from records in Rust; *model* = an
extrapolation or a stated cost model.

## Question

#3286 cut the Nagao-only cover arm by x1.71 by choosing the F_p-line `a·F_p + b` (`a = t²`) with the
largest factor base `B`. The review found that arm dominated by the sieve route (83-100x cheaper) and an
exploratory red-team pilot (not frozen, p = 1511, seeds 17-24) found x1.42 [1.18, 1.73] net at n = 4096 and
a net loss at n = 65536. This study asks, with the search fully charged: **does the line search lower the
total cost of the sieve route**, and does a cheaper search (a `chi(h)` table over `F_q`, then additions
and lookups per line) lower it further?

## Code (what the binary does; opt-in, default path unchanged)

* `JV_FB_LINE=n` (unset or 0: off) makes `run_cover_sieve_dlp_on` search `n` candidate lines before any
  context is built and move the instance (`G`, `Q` and the cover) to the best one. Every context the route
  builds (`Ctx::new` / `with_trace`, in the line enumeration, the sieve batches, the descent and the
  verification) is built from the moved spec, and `Ctx` reads `spec.line`. The search's F_p
  multiplications are added to `setup_muls` and `total_muls`; its F_p additions and table lookups to
  `total_plus`. The record carries a `fb_line` object (absent when off, so the plain record is unchanged).
* `JV_FB_LINE_MODE=horner` (default): per candidate, `h(a·s + b)` by Horner at every `s ∈ F_p` and the norm
  (`2·7 + 3 = 17` F_p mults per point, `15` additions and one lookup, emitted).
* `JV_FB_LINE_MODE=table`: build `chi(h(x))` for all `x ∈ F_q` once (`38·q` F_p mults, `29·q` additions,
  `q` lookups, emitted) and then count `B` per line by walking `x = a·s + b` (2 additions and 1 lookup per
  point, emitted, no multiplication). Candidates are the same as Horner's (plain line first, then seeded
  random `(t, b)`, `a = t²`), so for equal `n` the two modes choose the same line. If `n` exceeds the
  number of square-direction lines up to equivalence, `p(p+1)/2`, it enumerates all of them
  (`(p+1)/2` directions by `p` offsets, plain line first) and picks the first maximum.
* Not counted anywhere in the search (negligible or non-F_p): the index arithmetic `x0·p + x1` of a table
  lookup, the `p+1` direction inversions of the exhaustive enumeration beyond a 17-mult charge each,
  loop control.
* Unit-tested: table and Horner agree on every `B`, on the chosen line and on the candidate count; the
  exhaustive maximum is at least the sampled one; the moved model's factor base has exactly `b_best`
  points.

## Instances and options (V2's sieve arm, exactly)

`p ∈ {1009, 1511, 1823}`, seeds 17-32 (16 per arm and prime, 48 instances), `jv_cover --exp sieve --stop 64
--sizes p --first-seed 17 --seeds 16 --rho-runs 0` with `JV_LIN=sparse` (see `run.sh`; V2's flags).
The sieve runs on all four cores; counted metrics are host-independent. No rho is run.

## Arms

| arm | environment | role |
|:--|:--|:--|
| P | `JV_FB_LINE` unset | plain control; must reproduce V2's sparse records |
| H | `JV_FB_LINE=4096 JV_FB_LINE_MODE=horner` | Horner search, `n_H = 4096` |
| T | `JV_FB_LINE=4294967296 JV_FB_LINE_MODE=table` | table search, exhaustive (`n_T` = all lines) |
| U | `JV_FB_LINE=32768 JV_FB_LINE_MODE=table` | table search at the w = 0.1 rule's n (secondary, declared before data) |
| C2h, C2t | `JV_FB_LINE=1`, horner / table; seed 17, one instance per prime | machinery null |
| C3 | `JV_FB_LINE=4096 JV_FB_LINE_MODE=table`; seed 17, one instance per prime | table = Horner choice |

## Choosing n (rule written before data; its output is `results/rule.txt`, produced by `fit rule`)

* **Gain model (model).** The route cost falls with the base as `(B_plain/B_best)^γ`. With
  `B ≈ p/2 · (1 + z/√p)` (the red team's census: `sd(B) = √p/2` within 0.2%, Gaussian), the best of `n`
  lines has `z = k(n)`, the expected maximum of `n` standard normals (numerical quadrature), so the route
  scales by `(1 + k(n)/√p)^(-γ)`.
* **γ = 5.** It is the Nagao arm's measured exponent (#3286, B^-5, rate `(2B/p)^6`, `B+1` relations) and it
  lies inside the range the red team's exploratory pilot implies for the sieve route at p = 1511: its
  route-only x1.49 at n = 4096 and x1.75 at n = 65536 give γ ≈ 4.6 and 5.5 with `k = 3.5` and `4.2`. It is
  therefore *taken from the pilot's range and the Nagao arm, not derived* for the sieve route. (The
  descent alone scales as B^-6; the relation phase as B times a cost per relation the pilot does not
  separate.) The whole route cost, muls and native operations, is scaled by the same factor.
* **Search cost, counted.** Horner: `n·p·(17 + w·331·16)`. Table: `38 q + w·331·(30 q + 3·min(n, L)·p)`,
  `q = p²`, `L = p(p+1)/2`. `w` is the §5a weight in GAE per native operation (331 F_p mults per GAE).
* **Route cost.** `C_P(w) = M_P + w·331·N_P`, with `M_P`, `N_P` the means over V2's 16 sparse records at
  that prime of `total_muls` and `sieve_adds + sieve_lookups + enum_adds` (F4 and descent additions are
  not emitted by either arm).
* **Rule.** Minimise the geometric mean over the three primes of `J(n; w) = (route + search)/C_P`, over the
  grid `n ∈ {1, 2, 4, …, 2^20}` (plus *exhaustive* for the table), one `n` per mode for all primes.
* **Result at w = 0 (frozen, used):** `n_H = 4096` (model J = 0.655), `n_T = exhaustive` (J = 0.564; at
  w = 0 the table's lookups and additions are free, so only the `38 q` table build is charged).
* **At w = 0.1 (reported, arm U runs the table's):** `n_H = 8192` (J = 0.634), `n_T = 32768` (J = 0.611),
  the same at w = 1 (J = 0.634 and 0.611). Arm U is `n = 32768`; there is no arm at `n_H = 8192`.
  The model predicts that at w ≥ 0.1 the exhaustive table is at about parity with P (J = 0.989 and 0.993
  at w = 0.1 and 1), because its `3·p·L` native operations are priced.

## Metrics, per run (all from the record)

* `S = total_muls / (c_add_e·√l)`; `S⁺ = total_plus / (c_add_e·√l)`, `total_plus` being V2's definition
  (muls + `sieve_adds` + `sieve_lookups` + `enum_adds`) plus the search's emitted adds and lookups. `c_add_e`
  is read from the record.
* Phase shares of `total_muls`: enumeration (`enum_muls`), sieve (`sieve_muls`), descent (`descent_muls`),
  linear algebra (`la_muls`), search (`fb_line.muls`), other (setup, base, table, extract, verify). Shares of
  `total_plus` too.
* §5a band for arm/P: cost(w) = `total_muls + w·c_add_e·(sieve_adds + sieve_lookups + enum_adds +
  search adds + search lookups)` at `w = 0, 0.1, 1`, applied identically to both arms. The F4 and descent
  additions are unemitted on both arms, so every band cell is a lower bound on both.
* B model check, per run: `route-only ratio = (total_muls − search muls)/P's total_muls` against
  `(B_plain/B_best)^5` (`B_plain` = the P record's `base`, `B_best` = the arm's `base`; both equal the
  `fb_line` fields), and the fitted exponent `γ̂` of `ln(route ratio)` on `ln(B_plain/B_best)` through the
  origin with a bootstrap interval.

## Primary and confirmatory statistics

Paired by seed. The statistic is the geometric mean of arm/P over seeds, per prime (16 pairs) and pooled
(48 pairs, resampling seeds within each prime), with a 95% percentile bootstrap (20000 resamples, fixed
seed in the analyzer, Rust only).

* **H1 (confirmatory, arms H and T separately):** the upper end of the pooled `S` ratio is below 1
  (net, search charged). Supported or not, per arm; per-prime results are reported.
* **H2 (S⁺):** the same for `S⁺`. **H3 (band):** an arm "improves across the band" iff the upper end of its
  pooled ratio is below 1 at all of `w = 0, 0.1, 1`; otherwise the band order is unresolved and nothing is
  claimed. Arm U is descriptive (it is not a confirmatory test), as is the model check.

## Controls and acceptance (all must pass or every H-decision is withheld, and the failure is reported)

| id | requirement |
|:--|:--|
| C1 | arm P equals V2's sparse record on all 48 instances in every field but `wall_ms` (in particular `total_muls`, `total_plus`, `relations`, `descent_digest`) |
| C2h, C2t | on seed 17 of each prime: `relations`, `descent_digest`, `descent_residuals`, `base`, `hits`, `lines` equal P's; `total_muls − P's = fb_line.muls` exactly; `total_plus − P's = muls + adds + lookups` of the search exactly; `best_index = 0` |
| C3 | table at `n = 4096` has the same `b_plain`, `b_best`, `best_index`, `relations`, `descent_digest` as Horner's record on seed 17 of each prime, and route multiplications (total minus search) equal |
| C4 | every run of every arm is solved, correct, 0 failed verifications, not exhausted |
| C5 | `fb_line.b_plain` equals P's `base`, and `fb_line.b_best` equals the arm's `base`, on every pair |

## Cut rule and budget

At most 2 h wall on the 4-core box (another agent runs rho concurrently; counted metrics do not depend on
the host). Order: p = 1009, then 1511, then 1823, each prime's arms P, H, T, U and controls in turn.
If the budget runs out the arms of p = 1823 are dropped, last arm U first; the cut is reported. A cut
removes a cell, never a seed.

## What is reported regardless

Every record, log and the versions; the controls; all three primes and the pooled ratios for S and S⁺ and the
band; phase shares; the B model check; deviations. No leaderboard or `benchmarks/` row: there is no
`VALIDATION.json`. Wall time is an observation only.

## Limits declared in advance

* `γ = 5` is not derived for the sieve route; if the fitted `γ̂` differs the rule's `n` was chosen on a
  slightly wrong model (the cost is flat near the optimum, see `results/rule.txt`).
* The instances are V2's; the pilot used seeds 17-24 at p = 1511, so those seeds are not fresh with
  respect to the pilot (the rule did not use the pilot's records, only its exponent range).
* Local run, not isolab-eks (owner's default route for experiments; this is a counted, host-independent
  measurement of the same deterministic binary, and the rho runs of the sister study share the box).
