# Bounds and frontiers

**What a method costs, as a record; which methods nobody has beaten, as a
page; how to beat one, as a procedure.**

`ecbench` measures (`run`), pairs (`compare`), tabulates (`table`) and checks
one IC-against-rho claim (`claim`). None of those records *the established
cost of a method* as an object another session can cite, re-derive or
challenge. The best known constant for a walk lived in prose and in the
registry's derived expectation; whether a change moved the exponent, the
constant, or only the accounting was left to the author. This directory adds
five sealed records and the commands that produce them:

| record | schema | id | what it says |
|:--|:--|:--|:--|
| **bound** | `ecbench.bound/v1` | `ECBND1h…` | the cost of one method on one domain: `S = gae / √r` at the declared exponent, `gae = C · r^α` fitted across sizes, both with 95 % intervals, per phase, with what it stored and what it left unpriced, read from named sessions and audit receipts |
| **frontier** | `ecbench.frontier/v2` (v1 still builds and checks) | `ECFR1h…` | per domain, the bounds no other method's bounds dominate on the curves they share; built from the records, never edited |
| **challenge** | `ecbench.challenge/v1` | `ECCH1h…` | what a candidate must do to replace an incumbent: domain, incumbent, curves, targets, rounds, acceptance rule, a nonce |
| **commitment** | `ecbench.commitment/v1` | `ECCM1h…` | a candidate's sealed commitment to a challenge, made before the spec exists; the epoch is derived from it and a later public beacon |
| **verdict** | `ecbench.verdict/v1` (procedure 1 or 2) | `ECVD1h…` | one paired session judged against a challenge: `advances`, `trade`, `matches`, `regresses` or `inadmissible`, with the axes and the level it moved |

**Versions.** The first records were written before several refusals existed
(§3a). They are not rewritten. A bound names the procedure it was fitted under
in `fit_options.fit_version` (absent means 1), a verdict in `procedure`
(absent means 1), a frontier in its `schema`; each re-derives under the
version it names, and a new record is written under the current one.
Everything below is in the harness's unit (`ecbench.gae`, group-addition
equivalents) on single-target workloads, under AGENTS.md's rules: the whole
method, cold, one target, operation counts as the metric, wall time as a
practicality note and never the basis of a bound.

## 1. What a bound is, and is not

A bound is a **measurement with its scope attached**: this method
configuration (`ECM1h…`), this problem, this curve family, these sizes, this
tier, this unit, these sessions, this many verified runs. It carries the
constant and the exponent *with intervals*, because rho's cost is a random
variable with a spread of the order of its mean, and a point without an
interval is a guess with a number on it.

A bound is **not** a lower bound on the problem, not a theorem, not a
statement about cryptographic sizes, and not a wall-clock figure. The generic
floor `√(π / 2A)` it is measured against is a model of collision search, not a
proof about implementations. **Toy stays toy**: a bound in the `toy` tier
(fields of at most 32 bits) says nothing about `medium` or `crypto`, and the
frontier never mixes tiers. This is `claim_tier` from
crypto-autoresearcher's `docs/claims-and-verification.md`, applied at the
record level rather than by convention.

## 2. Where a change can happen: four levels

Every claimed speed-up moves exactly one of these, and the protocol refuses
to let one masquerade as another.

| level | what moves | how it is read | unit |
|:--|:--|:--|:--|
| **exponent** `α` | the algorithmic class: `r^{1/2}` to `r^{1/3}` | the slope of `ln gae` against `ln r` over at least four sizes, with its interval disjoint from the incumbent's | any counted unit |
| **constant** `C` | walk design, automorphisms used, distinguished-point parameters, table shape | the paired ratio `Σ S_candidate / Σ S_incumbent` with its interval excluding 1, at the same `α` | `ecbench.gae` |
| **primitive weights** | formulas and coordinates: field multiplications, squarings and inversions per group operation | for the prime-field generic walks and tables (`rho.*`, `bsgs.*`, `kangaroo.vow` on `PrimeCurve`): the record's `field_ops` block, tallied inside the group law — every modular multiplication (`muls`, one by a small constant included), every squaring where the formula squares a value (`sqrs`), every modular inversion (`invs`) — read as the axes `field_muls`, `field_sqrs`, `field_invs` per `√r` (§5); **unknown**, absent and never zero, on binary and Koblitz curves and for the index-calculus pipeline | field operations |
| **machine** | cycles per primitive: micro-architecture, SIMD, memory layout | wall time, instructions retired, isolation level | seconds, Ir |

The example that motivates the table: *one fewer squaring in the point
addition formula*. It is a primitive-weight change. It cannot move `α`, and in
`ecbench.gae` — which charges a group addition as one unit whatever its field
cost — it cannot move `C` either. The `gae` figure of a bound is therefore
blind to it by design, and a report that turns it into "`r^0.9`" has confused
levels. It is real, and it is measured at its own level: on prime-field
curves the generic walks and tables count the field operations behind every
group operation they charge, the record carries them as `field_ops`, the
bound as the axes `field_muls`, `field_sqrs` and `field_invs` (§5), and a
verdict reads them paired. One fewer squaring in the addition formula is then
a move on `field_sqrs` at an unchanged `ops`, and the level it names is
`primitive`. Where nothing counts them — binary and Koblitz curves, and the
index-calculus pipeline, whose field arithmetic outside the group law (square
roots, Legendre symbols, oracle inversions, the elimination) is counted
natively and priced apart — the axes are unknown, not zero. The counts reach
the end-to-end figure only through composition (§7), as a *derived* number
until an end-to-end session in a field-operation unit confirms it.

The level a verdict names is a statement about operations: `exponent` when
both arms' fitted `α` intervals are disjoint over four or more sizes and the
candidate's is lower, `constant` otherwise when `ops` moved, and `primitive`
when `ops` did not move but a field-operation axis did. An advance on memory
alone names no level.

## 3. Records

Every record is pretty-printed JSON whose id is `prefix + 12 hex` of the
SHA-256 of **its own bytes with the id field empty**, the seal `Record::seal`
uses for runs. A changed byte is a different record; two sessions that fit the
same sessions write the same bytes; a tampered record fails `bound check`
before anything is read from it. Records are write-once: a correction is a new
record, and the old one stays. Timestamps are kept out of every record for the
same reason.

### 3.1 Domain

Bounds compare only inside one domain, and a domain's id (`ECDOM1h…`) hashes
every field of it:

```json
{"problem": "ecdlp.single_target", "family": "prime", "target_kind": "planted",
 "unit": "ecbench.gae", "tier": "toy",
 "envelope": {"targets": 1, "precomputation": "none", "threads": 1}}
```

`family` is `prime`, `koblitz` or `binary` (the automorphism group, and so the
floor, differs). `tier` is derived from the field size — `toy` ≤ 32 bits,
`medium` ≤ 96, `crypto` above — and a fit whose sizes span tiers is refused
(`--tier` keeps one). `envelope` is the resource shape every ecbench workload
has today; a multi-target or precomputation method is a different domain, not
a better entry in this one.

### 3.2 Bound

| field | meaning |
|:--|:--|
| `method` | the configuration: registry id, resolved parameters, `method_id` (`ECM1h…`). The code's hash is in the provenance, not the identity: a registered method never changes what it counts (CI replays every committed session), so the configuration names the counts. |
| `level` | `exponent` when four or more sizes verified (a scaling claim), `constant` otherwise |
| `sizes[]` | one row per curve, sorted by `r`: slug, `log2_r`, field bits, `A`, the floor, workloads, runs, verified runs, mean `gae`, mean `S` with its interval, ratio to the floor, the registry's derived `S` and its ratio to the floor, memory and unpriced work per `√r`, isolation levels earned; from fit version 2 also `axes` (memory, unpriced work and the field axes on this size alone, each with an interval over the size's workloads) and `band` (the mean `S` on this size with unpriced work charged at 0, 0.1 and 1 GAE per operation, AGENTS.md §5a), so two bounds can be compared on the sizes they share |
| `fit` | `gae = C · r^α`: `alpha` and `alpha_ci95`, `log2_c`, `r_squared`, the declared `α` (`0.5` for every method the registry derives an expectation for), whether the interval contains it, `scaling_claim` |
| `constant` | mean `S` and mean ratio to the floor with intervals; the derived ratio to the floor when it is one number across sizes (`1.0` for `rho.negation`; `1.5 / √(π/4) = 1.69` for `bsgs.textbook`) |
| `dimensions` | the axes (§5): `ops`, `memory`, `uncharged`, each with `known`, `value`, `ci95`, its statistic and source; and `field_muls`, `field_sqrs`, `field_invs` (mean per `√r`, the same two-stage interval) **only when every verified run of the arm carries `field_ops`** — otherwise the keys are absent, not unknown, so a bound fitted from records written before the block existed is byte for byte the bound it was |
| `stages[]` | per phase name: mean `gae`, share of the total, sizes with work, its own fitted `α` with interval when it has work on four or more sizes |
| `provenance` | sessions (directory, session id, spec id, status, arm, role, binary hash, commit, environment class, SHA-256 of `records.jsonl`), audit receipts (path, hash, `ok`, replays, replays reproduced), record counts by status |
| `admissibility` | `admissible` or `inadmissible` (§3a lists every refusal), `bounded` (some work was counted and not priced: every total is a floor), the unpriced counters, determinism, reasons |
| `fit_options` | the filters, bootstrap settings (`resamples`, `seed`) and `fit_version`, so `bound check` fits it again |
| `tail` | fit version 2: the ops figure's tail, `max_over_median`, `top_run_share`, the run, and `large` when a single run is ten times the median or holds a quarter of the total (§3a) |
| `improves_on`, `verdict_id` | set by the verdict that produced the bound; empty on a bound fitted from a session directly |

Every statistic carries a two-stage bootstrap interval — sizes resampled,
then runs within each — the interval `compare` and `table` already use, with
2 000 resamples from a fixed seed so a bound re-derives bit for bit.

The run records a bound reads (`ecbench.record/v1`) carry one optional block
of their own for the primitive level: `field_ops: {muls, sqrs, invs}`, the
modular multiplications, squarings and inversions behind every addition and
doubling of the solve, summed over its phases. It is written only when the
method's group counted it (prime-field curves through the generic walks and
tables), so an older record is unchanged and its seal still checks; it lives
outside `counters` and `phases`, which the replay of a committed record
compares exactly; and `verify` compares it on replay only when both the
record and the replay carry it. What is counted: every modular multiplication
the group law performs is a `mul` (the doubling's multiplications by the
constants 3, 2 and 2 included, since the code performs them as modular
multiplications), every squaring where the formula squares a value is a
`sqr` (so `sqrs` is a genuine subset, not a reading of equal operands), every
modular inversion is an `inv`; additions, subtractions, negations and
comparisons are not counted. In the code as written an affine addition is
`2M + 1S + 1I` and a doubling `5M + 2S + 1I` (`PrimeCurve::FIELD_OPS_PER_ADD`,
`FIELD_OPS_PER_DOUBLE`); the special cases `∞ + P` and `P + (−P)` do no field
arithmetic. Binary and Koblitz curves and the index-calculus pipeline write no
block: unknown, never zero.

### 3a. Fit versions and what a bound refuses

A review found that the first procedure (fit version 1) published more than
its data supported and could be gamed. Version 2 refuses each of these; a
refusal is an inadmissible bound or a fit error, never a changed figure. The
point estimates of a version-2 bound are the version-1 ones; what changes is
which bounds are admitted, how the exponent's interval is drawn, and what
else a record carries. Version 1 stays in the code: a record without
`fit_version` re-derives under it, byte for byte.

| refusal (fit version 2) | why |
|:--|:--|
| a session read twice, two directories holding the same `records.jsonl`, or two sessions that ran the same workload with the same algorithm seed (sessions of one spec share their draws) is a **fit error** | `--dir X X X X` counted one set of runs four times and narrowed the interval |
| a session that is not `complete` is inadmissible | an interrupted session was admitted |
| a session without at least one **passing audit receipt** is inadmissible | zero receipts were admissible |
| runs that did not verify are counted **before** `--curve` and `--tier`; a filter that drops them makes the bound inadmissible | keeping only the curves that verified left a "wholly verified" arm |
| `resamples` other than the default 2 000 is inadmissible; the seed is recorded | `--resamples 1` gave a point "interval" |
| an interval that does not contain its own point estimate is dropped and the bound is inadmissible | a failed bootstrap was read as perfect precision |
| the exponent `α` needs two distinct sizes (a **relative** tolerance, not `1e-12` on a quantity of order 10⁶), has **no interval below four sizes**, and its interval is the percentile interval of resamples that hold **at least three distinct sizes** (the others are discarded and redrawn); `alpha_agrees_with_declared` exists only with an interval | one-size resamples produced slopes that were rounding noise, and committed interval ends were exactly those values |
| a bound on **one size** is stratified by workload (a cluster bootstrap over its workloads), not by run | the rounds of one target counted as many targets |

A version-2 bound also records, per size, the other axes with their own
intervals and the band-priced `S` (§5), and its `tail`: the ops figure is a
mean, and a mean of a heavy-tailed cost is carried by its largest runs. The
tail reports the largest run over the median run and the largest run's share
of the sum; `large` is set at ten times the median, or at a quarter of the
total with at least eight runs. A large tail is not a defect, but it is the
first thing to read beside the figure: it says why the interval is wide and
how lucky a single session could be.

### 3.3 Frontier

Built by `frontier build` from a directory of bounds. Per domain: every
ranked entry with its axes, `is_frontier`, `dominated_by`, the reasons, its
tail diagnostics and pricing checks, and the ops and memory leaders (or, where
none can be named, the tie set); the admissible bounds that are not ranked,
each with its reason; and the inadmissible bounds, listed and never on a
frontier. The page `FRONTIER.md` and `frontier.json` are generated and
committed; CI rebuilds both and fails when they are stale. `frontier build
--check` rebuilds under the procedure the committed file's `schema` names, so
a v1 frontier still re-derives (`--procedure 1`).

### 3.4 Challenge

| field | meaning |
|:--|:--|
| `domain`, `domain_id` | the domain the challenge is in |
| `incumbent` | the method to beat, as a method spec, and its committed `bound_id` when it holds one |
| `workloads` | the curves (at least `acceptance.min_sizes`, all in the domain's family and tier — checked by building them), targets per curve, target kind, and a `nonce` |
| `measurement` | rounds, warm-up, required isolation level, timeout |
| `acceptance` | `axes` dominance reads (default `ops`, `memory`; `uncharged`, `field_muls`, `field_sqrs`, `field_invs` opt-in); `min_sizes` (4); `min_runs_per_size` (8); `require_audit`; `require_replay_all`; `uncharged_tolerance` (0.05) |

The spec a candidate runs is a function of the challenge, the candidate's
method spec and an **epoch**: the target seed and the algorithm seed derive
from the nonce and the epoch, and the spec has three arms — `incumbent`
(reference), `candidate`, `incumbent-aa` (the incumbent again with the same
seeds, a determinism check, §5) — interleaved on the same workloads. Anyone
can rebuild the spec and check its `ECS1h…` id.

**The epoch is not the candidate's to choose** (procedure 2). A candidate that
may pick any integer tries epochs until the targets suit it, and every
verdict would then be the best of many. So `challenge commit` writes a sealed
*commitment* of the candidate to the challenge *before* any spec exists, and
`challenge spec --commitment C --beacon B` derives the epoch from the
challenge's nonce, the commitment's id and a **beacon**: a public value of at
least 16 hex digits that did not exist when the commitment was written (the
git commit id of the first commit that contains the commitment file, or a
randomness-beacon value). The candidate can neither predict the beacon nor
choose the epoch, and the spec and the verdict record the commitment and the
beacon. What the tool can check is that the epoch is what the commitment and
beacon give, that the commitment names this challenge and this candidate, and
that its timestamp is not in the future; that the beacon is later than the
commitment is checked by the reviewer, from the repository's history. A
commitment can be ground by varying its `note`, so every commitment made for
a challenge belongs in the repository and a verdict names the one it used.
`--legacy-epoch N` writes the version-1 spec of the first two verdicts; a
verdict over it is procedure 1 only.

### 3.5 Verdict

| field | meaning |
|:--|:--|
| `session` | directory, session id, its spec id, the spec id the challenge yields for this epoch and candidate, whether they match, status, binary hash, commit, levels earned |
| `audit` | the audit run by the verdict itself: `ok`, problems, records, verified, replays and how many reproduced, whether every run was replayed, and the SHA-256 of every session file |
| `incumbent`, `candidate`, `control` | arm names and method ids; the control's ratio and interval. The control is the incumbent against itself with shared seeds, identical by construction, so it reads `1.000 [1.000, 1.000]` whatever the noise: a **determinism check** of the pairing (procedure 2 labels it so, in `control.purpose` and in the statement), not a noise gate for `ops`; the noise `ops` has is in the paired interval itself |
| `axes[]` | per axis: both means, `Σ candidate / Σ incumbent` over matched `(workload, round)` pairs, its interval, pairs, `better` / `worse` / `indistinguishable` / `unknown`, and whether the axis decides; `ops`, `memory` and `uncharged` always, and the three field axes when a measured run of either arm carries `field_ops` or the challenge names one — `unknown` unless every measured run of both arms carries it — so a verdict over a session that counted nothing re-derives as it always did |
| `per_curve[]` | the ratio per size, the rows a scaling claim reads |
| `fits` | both arms' `α` with intervals, and `exponent_moved` |
| `stages[]` | per phase: each arm's share and the paired ratio — which sub-algorithm moved (§7) |
| `accounting` | unpriced counters on each side, `bounded`, the unpriced-work ratio and `uncharged_shift` |
| `incumbent_drift` | the incumbent's recorded ops figure against what it measured in this session, when `--bounds` is given: a disagreement is a reason for inadmissibility, because the recorded bound and the session cannot both be right |
| `outcome`, `advances_on`, `regresses_on`, `level_moved`, `reasons`, `statement` | the decision and why; `level_moved` is `exponent` or `constant` when the advance includes `ops`, `primitive` when it does not and includes a field axis, `null` otherwise |
| procedure 2 | `procedure`, `resamples` and `seed` (recorded, never free: an admissible verdict uses 2 000), `commitment` (id, timestamp, beacon), `multiplicity` (Holm over the deciding axes) with each axis's `p_value`, `holm_threshold` and `unadjusted_verdict`, and, for a bounded comparison, `band` (§5) |

## 4. The fit

Points are every verified measured run of the arm, `(ln r, ln gae)`, one
stratum per size. Least squares gives `α` and `ln C`; `R²` says how much of the
spread the law explains. The interval is the two-stage bootstrap of the slope,
drawn as §3a says (version 2: four or more sizes, resamples with at least
three distinct sizes).
`declared_alpha` is `1/2` wherever the registry derives an expected `S`
(`methods::expected_s`), and `alpha_agrees_with_declared` says whether the
interval contains it.

Three things to know when reading `α`:

- **Fewer than four sizes is not a scaling claim** (AGENTS.md §5). The bound
  then has `level: constant`, and `α` is printed as description, with no
  interval below four sizes and none at all below two.
- **Fixed costs pull `α` below the law at small sizes.** A method whose
  set-up (jump table, lanes, factor base) is a large share of the total at
  `2^16` fits a flat line there. The stage rows carry the set-up's share and
  the search phase's own `α`; read those before reading the headline.
- **Rho's variance is the law's.** With 24 runs per size on four sizes,
  `rho.negation`'s `α` interval is `[0.27, 0.61]`. That is what the data
  supports, and the record says so instead of rounding to `0.5`.

The constant at the declared exponent is `S = gae / √r` averaged over verified
runs, and its ratio to the floor `√(π / 2A)` of each curve. On prime curves
`A = 2` and the two are proportional; on Koblitz curves `A = 2n` varies with
the size, so the ratio to the floor is the figure that compares across sizes
and across families. It is the `ops` axis.

## 5. Axes and dominance

| axis | statistic | source | default |
|:--|:--|:--|:--|
| `ops` | mean `S / √(π / 2A)` | `cost.total_gae`, `r`, `A` | decides |
| `memory` | mean table entries per `√r` | the first of `inserts_uncharged`, `table_inserts_uncharged`, `table_entries`, `distinguished_points` the record carries | decides |
| `uncharged` | mean `Σ *_uncharged` counters per `√r` | every counter the unit counts and does not price | reported; decides when a challenge names it |
| `field_muls` | mean modular multiplications per `√r` | `field_ops.muls` | on a bound only when every verified run carries the block; on a verdict when a run of either arm carries it or the challenge names it; decides when a challenge names it |
| `field_sqrs` | mean modular squarings per `√r` | `field_ops.sqrs` | as `field_muls` |
| `field_invs` | mean modular inversions per `√r` | `field_ops.invs` | as `field_muls` |

**Unknown is not zero.** A method that reports no table counter has `memory:
known = false`; the axis is left out of every comparison involving it and the
entry says so. The index-calculus pipeline is in that position today.

The three field axes are the primitive level (§2). A bound carries them only
when every verified run of its arm carries `field_ops` — today the generic
walks and tables on prime-field curves — and a bound without them has no such
key at all, so the committed records fitted before the axes existed are
unchanged. On a frontier an axis one entry lacks is left out of comparisons
with it, as `memory` is for index calculus, and the page shows the columns in
a domain only when some entry there has them. A verdict carries them when a
measured run of either arm counted them or the challenge names one, `unknown`
unless every measured run of both arms carries the block, and deciding only
when named; a session that counted nothing under a challenge that asked
nothing gives the verdict it always gave, byte for byte. They are not a unit:
`ops` still decides in `ecbench.gae`, and a candidate
that is clearly better on `field_sqrs` and indistinguishable on `ops` has
moved the primitive level, not the constant.

On a frontier, bounds were measured apart and cannot be paired, so dominance
is conservative, and it is read **only where the bounds can be compared**.
The ratio to the floor depends strongly on the curve (prime `rho.negation` is
`2.22 ×` at 16 bits and `1.06 ×` at 26), so two bounds fitted on different
curves are not comparable on their headline figures, and procedure 1, which
compared them, ranked a bound fitted on large curves above one fitted on
small ones. Procedure 2 therefore compares two bounds **per size, on the sizes
they share** (the same slug and subgroup order), each size with its own
interval. If they share none they are *not comparable*: neither dominates, and
the page says so. On the shared sizes `A` dominates `B` when, on some axis,
`A`'s interval lies wholly below `B`'s on at least one size, on no shared size
and no axis is `A` clearly worse, and `A`'s mean over the shared sizes is not
above `B`'s. A missing interval separates nothing (a point against anything is
a tie), and an axis a size does not carry is left out and named. Ties stand.

Four further rules close the ways a frontier was made to overstate:

- **Only whole fits are ranked.** A bound enters the frontier only when it was
  fitted on **four or more sizes** and on the session's **full curve set** (no
  `--curve`, no `--tier`) under fit version 2. The rest are listed under each
  table with the reason; a fit restricted to the curves that suit it cannot
  cut a frontier down.
- **A method is ranked on its worst bound.** A method that has several bounds
  in a domain dominates another entry only if *every* one of its bounds that
  shares a size with it does, so the luckiest session cannot decide; a single
  lucky bound cannot knock an entry off the frontier.
- **Unpriced work is ranked across the band.** When either bound has counted
  but unpriced work (`bounded`), a dominance must hold at **0, 0.1 and 1 GAE
  per unpriced operation** (AGENTS.md §5a, the band `ecbench band` uses), read
  from each size's `band`; if it fails at any weight, or a bound cannot be
  priced above zero because a counter has no count, it is not dominance. The
  page shows the **break-even price** per uncharged operation at which the two
  mean ops figures cross: the dominance flips there, inside the band or beyond
  it.
- **A leader is named only when it is separated.** The ops (and memory) leader
  is the method with the smallest *worst-bound* figure among the frontier
  entries, and it is named only when its intervals separate from every other
  method's on the sizes they share; otherwise no leader is named and the tie
  set is listed, as it is for a method with which it shares no size.

The point condition keeps a wide interval from pushing a precise, better entry
off the frontier through a secondary axis. Tail diagnostics (§3a) are shown
beside every ops figure.

In a verdict the arms *are* paired, so each axis reads the interval of
`Σ candidate / Σ incumbent` against 1: `better` below, `worse` above,
`indistinguishable` across. A ratio that its own interval excludes is a failed
bootstrap and reads `unknown`. Under procedure 2 the per-axis 95 % tests are
corrected for the several deciding axes: each carries a bootstrap two-sided
`p_value`, Holm's step-down procedure (family-wise 0.05, over the deciding
axes) decides which are rejected, and an axis whose interval said `better` or
`worse` and that Holm does not reject reads `indistinguishable` (its
interval-only verdict is kept in `unadjusted_verdict`). With unpriced work on
either side the `ops` axis is read at every weight of the band: `better` only
if better at all three, and the verdict records the ratios, the intervals and
the break-even price (`band`); a counter without a count makes the comparison
unrankable and the verdict inadmissible. The outcome is Pareto over the
deciding axes:

| outcome | meaning |
|:--|:--|
| `advances` | better on at least one deciding axis, worse on none |
| `trade` | better on some, worse on others (BSGS against rho: fewer operations, a `√r` table) |
| `matches` | better on none, worse on none |
| `regresses` | worse on some, better on none |
| `inadmissible` | the session is not the challenge's spec for the epoch (procedure 2: the epoch is not the one the commitment and beacon give); the session is not complete; the audit found a problem or did not replay every run when required; too few sizes or runs; a measured run did not verify; the bootstrap used other than 2 000 resamples; **the incumbent drift check was not run** (`--bounds`, and a challenge that names the incumbent's bound) or the recorded bound and this session disagree. The paired ratio is still reported, for information. |

## 6. Challenge to verdict

```sh
# 1. The challenge, from a draft (domain, incumbent, curves, measurement,
#    acceptance); `seal` builds the curves, checks family and tier, hashes.
ecbench challenge seal --draft draft.json --out docs/bounds/challenges/prime-toy-ops.json

# 2. Commit the candidate BEFORE any spec exists, and commit the file to the
#    repository.  The epoch is then derived, not chosen.
ecbench challenge commit --challenge docs/bounds/challenges/prime-toy-ops.json \
    --candidate '{"id":"kangaroo.vow2","params":{"cap_multiple":"64"}}' \
    --out research/<topic>/commitment.json

# 3. Take a beacon: a public value that did not exist when the commitment
#    was written (the git commit id of the commit that added it, a
#    randomness-beacon value), at least 16 hex digits.  The spec's epoch
#    derives from the challenge nonce, the commitment and the beacon.
ecbench challenge spec --challenge docs/bounds/challenges/prime-toy-ops.json \
    --candidate '{"id":"kangaroo.vow2","params":{"cap_multiple":"64"}}' \
    --commitment research/<topic>/commitment.json --beacon <hex> --out spec.json

# 4. Run it as any session is run (docs/ecbench/README.md; L2 for wall time,
#    irrelevant to the counts).
ecbench run --wait --spec spec.json --out research/<topic>/sessions/e1

# 5. The verdict: audits with every run replayed, pairs, fits, corrects for
#    the deciding axes, decides; writes the candidate's bound on an advance or
#    trade.  --bounds is required (the incumbent drift check).
ecbench challenge verdict --challenge docs/bounds/challenges/prime-toy-ops.json \
    --dir research/<topic>/sessions/e1 --commitment research/<topic>/commitment.json --beacon <hex> \
    --replay-all --bounds docs/bounds/records --out research/<topic>/verdict.json \
    --bound-out docs/bounds/records/<written by you>.json --audit-out research/<topic>/audit.json --exit-code

# 6. Rebuild the frontier; commit the session, the verdict, the receipt, the
#    bound and the page in one PR.
ecbench frontier build --bounds docs/bounds/records --out docs/bounds/frontier.json --markdown docs/bounds/FRONTIER.md
```

An `advances` verdict writes the candidate's bound with `improves_on` naming
the incumbent's. A `trade` writes the bound too, since it is a measurement,
but it improves on nothing (procedure 1 set `improves_on` on a trade as well;
that is the one thing a version-1 record of a trade gets wrong, and the
frontier ignores `improves_on` on legacy bounds). The first two verdicts are
procedure 1, re-derived with `--procedure 1 --epoch 1` (`refit.sh`).

A candidate is a registered method (`ecbench-extend`): a changed algorithm is
a new registry id, the old id keeps its evidence, and the challenge's spec
names both. A candidate binary against a baseline binary is what two sessions
of one spec and `compare --b-dir` are for; a verdict reads one session, so a
code change goes through the registry.

The verdict is **re-derivable**: run it again on the same session and you get
the same `ECVD1h…`. It embeds the SHA-256 of every session file, not the
receipt's own hash (which carries a timestamp).

## 7. Sub-algorithms: stages and composition

A method is phases — `setup`, `search`, `internal_verification` for the tuned
walks; `factor_base`, `oracle_setup`, `relations`, `linear_algebra`, `verify`
for index calculus — and every record already charges each phase separately.
A bound carries each phase's share of the total and, with work on four or more
sizes, the phase's own `α`; a verdict carries each phase's paired ratio. So a
reader sees *where* a change landed: a new jump-table construction shows in
`setup`; a better walk in `search`; a candidate whose `relations` share fell
while `linear_algebra` rose traded one sub-algorithm's cost for another's.

Composition across levels is a sum over phases of counts times weights:

```text
cost_u(method, r) = Σ_phase Σ_op  count(op, phase, r) · w_u(op)
```

`w_gae` is 1 for an addition or a doubling and 0 for everything else (the
unit's charging rule, `docs/ecbench/README.md` §7). A field-operation unit
would weight an addition by its multiplications, squarings and inversions on
the curve family at hand, and a time unit by pinned nanoseconds per native
operation (`docs/ic/calibration.json`, the `ICBCAL1h…` discipline of
`aburan28/cryptanalysis`). On prime-field curves the weights are measured
rather than assumed: every phase's `adds` and `doubles` and the solve's
`field_ops` (§3.2) are the raw material of `w_field`, since dividing the
solve's multiplications, squarings and inversions by its additions and
doublings gives the weight per group operation the code actually paid —
`2M + 1S + 1I` per affine addition and `5M + 2S + 1I` per doubling as written
today — and a candidate formula shows up as a different quotient on the same
group-operation counts. **A composed figure is derived, never measured, and
never enters a frontier**: it predicts what an end-to-end session in that
unit should find, and the session is what moves the frontier in that unit.
This is where "one fewer squaring" lives — as a weight change, now visible on
the `field_sqrs` axis of the records that count it, whose end-to-end
consequence in any other unit is a prediction until measured — and it is why
the four levels are kept apart.

## 8. Rules

1. **Records are write-once and content-addressed.** A changed bound is a new
   bound. Nothing overwrites; `frontier.json` and `FRONTIER.md` are generated
   from the records and checked in CI.
2. **A bound is re-derivable.** `bound check` fits it again from the sessions
   and receipts it names and requires the same id; CI does this for every
   committed record. A bound whose sessions are missing, edited, or re-count
   fails.
3. **One domain per bound, one tier per domain.** Mixed families or tiers are
   refused at fit time. A `toy` frontier is a statement about toy sizes.
4. **Wall time never enters.** No axis, no fit, no acceptance rule reads a
   clock. The isolation level is recorded for the reader of wall figures
   elsewhere.
5. **Unknown is not zero, and unpriced is not free.** A missing counter leaves
   an axis unknown; counted-but-unpriced work is reported on every record and
   verdict (`bounded`, `uncharged`), and a challenge may make it decide.
6. **A frontier is conservative; a verdict is paired.** Ties stand on the
   frontier, bounds are compared only on the sizes they share, and a leader is
   named only when its interval separates (§5). Only a paired session on the
   challenge's frozen workloads, audited with every run replayed, moves it.
9. **Nothing in a figure is free.** The bootstrap's resample count and seed
   are recorded in every bound and verdict, and an admissible one uses the
   default; the draws behind a figure are distinct; the incumbent's recorded
   bound is checked against the session (`--bounds`); the epoch is derived
   from a commitment and a beacon (§3.4); and the several deciding axes are
   corrected for (§5). These are conditions of admissibility, not options.
7. **Levels are named, never inferred from the headline.** `exponent` needs
   disjoint `α` intervals over four or more sizes. Everything else that moves
   `ops` at fixed `α` is `constant`. `primitive` needs a field-operation axis
   to move while `ops` does not, and exists only where the axes are known
   (prime-field generic walks and tables). Machine changes are not measured
   here and are not claimed.
8. **Everything cites.** A bound names its sessions by directory, session id
   and `records.jsonl` hash, and its receipts by hash; a verdict names the
   session's files by hash and the challenge by id; a new bound names the
   verdict and the bound it improves on.

## 9. Reading the seed frontier

`records/` holds 54 bounds fitted from five committed sessions and two
challenge sessions: `research/ecbench_all_candidates_20261003/sessions/{prime,koblitz}`
(four sizes each, eight targets, three rounds, audited with 12 replays);
`research/ecbench_calibration_20261002/sessions/{prime,koblitz}` (six sizes
each, same targets and rounds; the Koblitz session spans tiers — degrees 17,
19, 23 are toy, 37, 41, 43 medium — so it is fitted once per tier, three
sizes each, and those are listed, not ranked: three sizes and a tier subset);
`research/ecbench_pair_claw_20261003/sessions/koblitz` (six Koblitz degrees
41 to 61, all medium); and the two epoch-1 sessions of
`research/ecbench_bounds_challenges_20261006/`, whose `incumbent` and
`candidate` arms are fitted as ordinary bounds (`session-*`). Forty-eight are fit
version 2. Six are fit version 1 and unchanged: the four incumbents the
standing challenges name by id (`prime-rho-neg`, `prime-bsgs-neg`,
`koblitz-rho-strong`, `koblitz-bsgs-neg`; a committed challenge is never
rewritten, and each has a `-v2` twin beside it) and the two bounds the
epoch-1 verdicts wrote (`challenge-*`). `bound check` re-derives all of them,
each under the version it records; the version-1 bounds appear on the page as
*listed, not ranked*. The page has three domains. What changed from the first
page, and what they say, read honestly:

- **Prime, toy.** `bsgs.negation` is `1.100 ×` the floor `[0.978, 1.225]` over
  six sizes (16 to 26 bits) with a `0.5 √r` table; `rho.negation` is the
  memory-light walk at `1.453 ×` `[1.184, 1.772]` (six sizes) and `1.406 ×`,
  `1.466 ×` over four, storing `0.03 √r` points. The first page named
  `bsgs.negation` the ops leader on its point estimates; the intervals
  overlap (`[0.978, 1.225]` against `[1.184, 1.772]`), so **no leader is named**
  and the page lists the six-bound tie set. Both methods are on the frontier:
  the time–memory trade, as `docs/ecbench/README.md` §7 says, and not a
  finding. `rho.plain` (`1.920 ×`, `1.990 ×`) used to be dominated by
  `rho.negation` ("the negation map's `√2`"); it now stands, because
  `rho.negation` leaves `1.0` canonicalisation per `√r` unpriced and the order
  flips at about `0.34` to `0.55` GAE per canonicalisation (the page shows each
  break-even): inside the band of 0, 0.1 and 1, the claim is unresolved, not
  a win. `rho.frozen_reference` stands through memory alone. `bsgs.interleaved`,
  `bsgs.textbook` and `kangaroo.vow` are dominated, each by every comparable
  bound of the method that dominates it.
- **Koblitz, toy.** The first page had three `rho.signed_frobenius` entries on
  the frontier and `bsgs.negation` and `rho.negation` "dominated by the
  three-size walk on both axes". That comparison put a bound fitted on degrees
  17, 19, 23 (`3.306 ×`) against bounds fitted on other curves. Those three-size,
  tier-subset bounds are now listed and not ranked; on the four degrees the
  full session covers, `rho.signed_frobenius` reads `4.996 ×` `[2.985, 7.374]`
  and `9.455 ×` `[2.933, 19.898]` (both with a **large** tail: one run is
  `11.7 ×` and `54 ×` the median), `bsgs.negation` `5.877 ×` `[4.674, 7.188]`,
  `rho.negation` `9.041 ×`, `rho.plain` `10.31 ×`. All five stand: the walk
  leads on memory by a factor of about thirty, and its operations order
  against `bsgs.negation` changes with the size (disjoint intervals on one of
  four degrees, in the walk's favour); the page lists three bounds as an ops
  tie set and names no leader. In the same session, on the same targets, the
  first page ranked `bsgs.negation` below a `rho.signed_frobenius` bound
  fitted on other curves, though the order here is the reverse (`9.455 ×` for
  the walk against `5.877 ×` for `bsgs.negation`). The strong
  lockstep rho is dominated at these sizes because its set-up is most of its
  total and its `α` fits at `0.13`: a fixed cost at `2^15` to `2^21`, not a
  law.
- **Koblitz, medium** (degrees 37 to 61, `r` from `2^27.8` to `2^47.2`): four
  ranked, two on the frontier. `rho.signed_frobenius_strong` leads on
  operations at `1.089 ×` the floor `[0.927, 1.261]` over six degrees (41 to
  61, 144 runs, `α 0.451` `[0.399, 0.503]`), storing `0.008 √r`; here its
  set-up is 0.4 % of its total and the toy-tier reading above does not carry
  over. It is named the ops leader because its intervals separate from the
  other frontier method's on all six shared sizes. The pair claw at the shape of
  cryptanalysis#175 (`35.6 ×`, memory `0.004 √r`) is on the frontier through
  memory alone, with a large tail; the claw at `c = 1` (`2.390 ×`) and
  `bsgs.negation` (`7.669 ×`) are dominated. The three-degree `rho.signed_frobenius`
  bound (`1.188 ×` over 37, 41, 43) that held the memory lead is listed, not
  ranked.
- **Index calculus** is inadmissible on the Koblitz session (half its runs did
  not verify at the two largest sizes) and far from the floor on the prime
  session (`22.7 ×` and `1258 ×`). Both are listed; neither is on a frontier.
  Its memory is unknown (no table counter), which is a gap in the pipeline's
  reporting, not a strength.
- **Two verdicts** (`research/ecbench_bounds_challenges_20261006/`; epoch 1
  each, every run replayed, every run L0), written under procedure 1 and
  re-derived byte for byte by `refit.sh --procedure 1`. `ECVD1h0b0f5e0d7f67` on
  `prime-toy-reference` reads `trade`: `bsgs.negation / rho.negation = 0.8250
  [0.725, 0.945]` on operations and `15.79 [13.38, 19.06]` on memory. The bound
  it wrote, `ECBND1he0aea8671f10`, names `ECBND1h7b69b9787056` under
  `improves_on`: **that is wrong for a trade** (a procedure-1 defect, fixed in
  procedure 2), so the frontier ignores `improves_on` on version-1 bounds.
  `ECVD1h53a3dfe36118` on `koblitz-toy-reference` reads `advances` at the
  constant level: `rho.signed_frobenius / rho.signed_frobenius_strong = 0.2991
  [0.235, 0.402]` on operations and `0.560 [0.483, 0.656]` on memory; its
  `setup` stage carries the move. Both were one epoch, toy tier, no wall-clock
  figure, a candidate-chosen epoch (the commitment of §3.4 did not exist), and
  an A/A control that reads `1.0000 [1.000, 1.000]` by construction: a
  determinism check, not a noise estimate. Their arms stand as ordinary
  version-2 bounds (`session-*`).
- **`α` is a description here, not a result.** The first page quoted
  intervals such as `[-1.000, 0.573]`, `[0.179, 2.500]` and `[0.000, 0.383]`;
  the ends were degenerate resamples (a draw of one size, a slope of rounding
  noise), and `kangaroo.vow`'s "`α` agrees with declared" rested on
  `[0.18, 2.5]`. Version 2 reads `[0.401, 0.724]` for it, and gives the
  three-size bounds no interval at all. Where an interval excludes `0.5` it lies below it, and the stage rows say
  why: a fixed cost that is a large share of the total at toy sizes (the
  strong lockstep walk's set-up; the frozen reference walk; the toy
  signed-Frobenius fits), or a 16-bit curve in a six-size fit. The one
  interval above `0.5` is the fixed-base `ic.pipeline` with the `subtract`
  oracle (`1.005 [0.880, 1.077]`), a 32-element base at growing `r`. Nothing in
  these records is a scaling result; they are constants with their scope.

## 10. Commands

```text
ecbench bound fit --dir SESSION... --arm ARM [--tier T] [--curve SLUG...] [--audit RECEIPT...]
                  [--label L] [--notes N] [--root REPO] [--fit-version 2] --out BOUND.json
ecbench bound check --record BOUND.json... [--root REPO]
ecbench frontier build --bounds DIR|FILE... [--axes ops,memory] [--procedure 1|2]
                       --out F.json --markdown F.md [--check]
ecbench challenge seal --draft D.json --out C.json
ecbench challenge check --file C.json...
ecbench challenge commit --challenge C.json --candidate '{"id":...}' [--note N] --out commitment.json
ecbench challenge spec --challenge C.json --candidate '{"id":...}'
                       (--commitment commitment.json --beacon HEX | --legacy-epoch N) --out spec.json
ecbench challenge verdict --challenge C.json --dir SESSION --bounds DIR [--replay-all] [--root REPO]
                          (--commitment commitment.json --beacon HEX | --procedure 1 --epoch N)
                          --out V.json [--bound-out B.json] [--audit-out A.json] [--exit-code]
```

`--axes` and a challenge's `acceptance.axes` may name `ops`, `memory`,
`uncharged`, `field_muls`, `field_sqrs` and `field_invs`.

`refit.sh` regenerates every seed record (fit version 2) and the frontier
(procedure 2) from the committed sessions, and runs the two committed
verdicts again under procedure 1 so the version-1 records they wrote
re-derive byte for byte; CI runs `bound check` on every record, `challenge check` on every
challenge and `frontier build --check` on the page.

## 11. Layout

```text
docs/bounds/
  README.md          this protocol
  FRONTIER.md        generated page, checked in CI
  frontier.json      its machine-readable twin
  records/*.json     bound records, write-once: <session>-<arm>.json from a fit,
                     challenge-<name>-e<N>-<candidate>.json written by a verdict
  challenges/*.json  standing challenges, one per domain and axis leader
  refit.sh           regenerate records and frontier from committed sessions

research/ecbench_bounds_challenges_<date>/
  specs/<name>-e<N>.json     the spec `challenge spec` wrote for one epoch
  sessions/<name>-e<N>/      the session; evidence, replayed by CI, never edited
  verdict-<name>-e<N>.json   the verdict, re-derivable byte for byte
  audit-<name>-e<N>.json     the receipt the verdict wrote (carries a timestamp)
  table-<name>-e<N>.md       `ecbench table` over the session
  README.md                  the question, the host, the levels, each statement
```

## 12. Beyond this repository

- **crypto-autoresearcher** reads bounds into its ledger through an
  evidence record's `measured_bound` block, reviewed by the Coordinator like
  any evidence; a frontier move there is a decision, not a push
  (`docs/bounds-and-frontiers.md` in that repository). The tier rules are the
  same rules.
- **cairn** holds the same frontier as a ratchet objective per domain, with a
  pinned evaluator that rescores a bound record from its own per-size counts
  and refuses a wall-clock basis; the network's `frontier_status` and this
  page then agree or one of them is wrong.
- **aburan28/cryptanalysis** fits the same law (`ca_bench complexity`) in its
  own units; a bound record in this schema from its receipts is the way to
  put the two libraries on one page without pretending the units agree.
