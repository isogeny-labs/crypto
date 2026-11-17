# Protocol: the PKM Kummer tower at P-224's 2-adic shape, priced against matched rho

Lane `pkm-p224-shape`, 2026-10-22. Branch `claude/p224-pkm-kummer-10gpep`.
Lead: claim 2d of
[`research/p224_hypothesis_ranking_20261008/README.md`](../p224_hypothesis_ranking_20261008/README.md)
(P-224 is the only NIST prime with `v2(p - 1) = 96`, the prerequisite of PKM's
Kummer factor base `V = g * mu_{2^t}`). Background:
[`RESEARCH_PKM_TOWER_ORACLE.md`](../notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md)
(the *PKM note*) §2.1, §3.4, §10-§17;
[`research/pkm_supersingular_aux_20261015/`](../pkm_supersingular_aux_20261015/README.md);
[`research/f4_generic_field_20261015/`](../f4_generic_field_20261015/README.md)
(generic-field F4); [`research/pkm_tower_ecbench_20261013/`](../pkm_tower_ecbench_20261013/README.md)
(the end-to-end ladder, Proth primes `c*2^12 + 1`).

This file is committed before any measured run. The runs it lists, and nothing
else, make the results. A change after the first measured run is a dated,
labelled amendment, and both versions are reported. Every number in the report
carries one label: *recorded* (a committed earlier row), *measured* (this
directory's runs), *derived* (a calculation shown) or *predicted* (stated here).

## 0. What is new relative to the prior rounds

- The prior pilot primes are all Solinas-shaped by accident:
  `p0 = 3*2^18 + 1 = 2^20 - 2^18 + 1`, `p1 = 15*2^27 + 1 = 2^31 - 2^27 + 1`,
  `p2 = 3*2^30 + 1 = 2^32 - 2^30 + 1`. In all three `k/n` is 0.87-0.94 and the
  odd part of `p - 1`, `2^(n-k) - 1`, is 3 or 15 (2 or 4 bits). P-224 is
  `2^224 - 2^96 + 1`: `k/n = 3/7 = 0.43`, odd part `2^128 - 1` (128 bits). So the
  prior rounds never tested the *proportions* of P-224's 2-adic structure
  (a small 2-power part sitting in a large, smooth odd part).
- This round uses primes `p = 2^n - 2^k + 1` with `k/n` from 0.42 to 0.70 and
  odd parts of 6 to 35 bits (and P-224 itself), at sizes from 20 to 224 bits.
- It goes one size beyond the recorded `m = 2` frontier (`N = 22`, PKM note
  §11.7): `N = 24` at `t = 12`.
- It prices the whole method (not the solver stage only) against matched rho
  under AGENTS.md §§1-8 and the §5a band.

## 1. Questions

- **Q1 (shape).** At a fixed `(m, N)`, does the F4 solving degree `D` of the Kummer
  tower at a P-224-shaped Solinas prime equal the recorded `D` at `p0, p1, p2`?
- **Q2 (frontier).** What is `D` at `m = 2, N = 24` (`t = 12`, the first size past
  the recorded `N = 22`)? Does the `m = 2` line rise?
- **Q3 (null).** Is tower `D` below the shape-matched null at the same `(m, N)`?
- **Q4 (size).** Is `D` the same, field-for-field (width, steps, multiply-adds),
  at 48, 61 and 224 bits as at 31-32 bits, at the same `(m, N)`?
- **Q5 (price).** What does the whole PKM method cost in `S = ops / sqrt(r)`
  against matched rho (negation map), on Solinas curves and on the
  prior ladder's Proth curves of the same bit size, with the §5a band?

## 2. Frozen inputs

### 2.1 Primes (all checked prime with `factor` before this file)

| id | `p` | form | `n` | `k = v2(p-1)` | `n-k` | `k/n` | arithmetic |
|:--|--:|:--|--:|--:|--:|--:|:--|
| SL20 | 1,032,193 | `2^20 - 2^14 + 1` | 20 | 14 | 6 | 0.70 | `Fp32` |
| SL24 | 16,760,833 | `2^24 - 2^14 + 1` | 24 | 14 | 10 | 0.58 | `Fp32` |
| SL26 | 67,104,769 | `2^26 - 2^12 + 1` | 26 | 12 | 14 | 0.46 | `Fp32` |
| SL28 | 268,369,921 | `2^28 - 2^16 + 1` | 28 | 16 | 12 | 0.57 | `Fp32` |
| SL31 | 2,147,352,577 | `2^31 - 2^17 + 1` | 31 | 17 | 14 | 0.55 | `Fp32`, under `2^31` |
| SL32 | 4,293,918,721 | `2^32 - 2^20 + 1` | 32 | 20 | 12 | 0.63 | `Fp32`, above `2^31` |
| SL48 | 281,474,975,662,081 | `2^48 - 2^20 + 1` | 48 | 20 | 28 | 0.42 | `auto` (`Fp52` if AVX-512 IFMA, else `Fp64`) |
| SL61 | 2,305,843,009,146,585,089 | `2^61 - 2^26 + 1` | 61 | 26 | 35 | 0.43 | `Fp64` |
| P224 | `2^224 - 2^96 + 1` | NIST P-224 | 224 | 96 | 128 | 0.43 | `Fp224` |
| p0 | 786,433 | `2^20 - 2^18 + 1` | 20 | 18 | 2 | 0.90 | recorded comparator, positive control |
| p1 | 2,013,265,921 | `2^31 - 2^27 + 1` | 31 | 27 | 4 | 0.87 | recorded comparator |
| p2 | 3,221,225,473 | `2^32 - 2^30 + 1` | 32 | 30 | 2 | 0.94 | recorded comparator |

`n - k` is the bit length of the odd part plus 0 or 1; the Kummer tower's
depth `t` is bounded by `k`, not by `n - k` (the task's phrase "`n - k` at least
the tower depth" is read as: `k >= t` for every cell, and `n - k` is reported
as the shape coordinate). Every cell below has `t <= k`.

All Solinas primes with `n` in {20, 22, ..., 32}, `8 <= k <= 20` were enumerated;
the ones with `k/n` closest to P-224's are SL26 (0.46) and SL48/SL61/P224
(0.42-0.43). Primes with `k/n` near 3/7 are rare at small `n`; that is why
SL20-SL28 and SL31-SL32 span 0.46-0.70.

### 2.2 Instrument

- **Builder and engine.** `examples/pkm_tower_pilot.rs` unmodified, `--engine
  generic` (`f4_tower_generic`, which at `Fp32` reproduces `f4_fp_tower` field
  for field, and at `Fp52`/`Fp64`/`Fp224` gives the same report,
  `research/f4_generic_field_20261015/`). Kummer kind, reduced presentation,
  round-6 full-rank exit on, degree bound `n + d + 6`.
- **Targets.** `--planted 0 --random 2 --ladder-t none`, seed `0x504B4D54`
  (1347112276, the seed of every earlier round), except where a cell says
  `--targets 0` (one target). Controls: `tower` and `null` (the shape-matched
  null of §5.3 of the PKM note; `--null-chain` at `m >= 3`).
- **Positive control (R0).** The isogeny kind at `p0`, `m = 2`, `N = 4...10`,
  must give `D = 5` on both targets (the recorded value; the Kummer tower gives
  4). If it does not, Q1-Q4 read invalid.
- **Checker.** `examples/pkm_tower_check.rs verify` recounts every finished
  `Fp32`/`u64` row by exhaustive search over `V^m`; the wide builder's own
  `--verify` does the same in `BigUint` arithmetic for rows above `2^63`. Every
  finished tower row must agree. `analyze` tabulates.
- **Resources.** Small cells: `nice -n 10`, `RAYON_NUM_THREADS=1`, `ulimit -v`
  4 GB, one cell at a time. The `N = 24` cell: 2 threads, `ulimit -v` 9 GB,
  `--max-nnz 1,800,000,000`. A system that runs out of memory, time or
  `--max-nnz` reports no `D` (at most a lower bound) and is recorded as out of
  budget; it is not re-run. The host is a 4-vCPU Xeon, 15 GB, shared with other
  agents, so no wall time here is an isolated measurement (stage diagnostic,
  descriptive only).

### 2.3 Cells (run in this order; `R` numbers are the run ids)

`m = 2` is `full`, `m >= 3` is `chain`. `t = N/m`. `T` = both targets unless
noted.

| run | prime | `m` | `N` | controls | notes |
|:--|:--|--:|:--|:--|:--|
| R0 | p0 | 2 | 4, 6, ..., 10 | isogeny tower | positive control |
| R1 | SL26 | 2 | 2, 4, ..., 20 | tower, null | `t <= 10 <= 12` |
| R2 | SL31 | 2 | 2, 4, ..., 20 | tower, null | |
| R3 | SL32 | 2 | 2, 4, ..., 20 | tower, null | |
| R4 | SL20 | 2 | 16, 18, 20 | tower | the 20-bit priced curve's prime |
| R5 | SL26 | 3 | 6, 9, 12 | tower, null-chain | |
| R6 | SL31 | 3 | 6, 9, 12 | tower, null-chain | |
| R7 | SL32 | 3 | 6, 9, 12 | tower, null-chain | |
| R8 | SL26, SL31 | 4 | 8, 12 | tower, null-chain | |
| R9 | SL48 | 2 | 2, ..., 18 | tower; null to 14 | |
| R10 | SL48 | 3, 4 | 6, 9, 12; 8 | tower | |
| R11 | SL61 | 2 | 2, ..., 18 | tower; null to 14 | |
| R12 | SL61 | 3, 4 | 6, 9, 12; 8 | tower | |
| R13 | P224 | 2 | 2, ..., 16 | tower; null to 14 | |
| R14 | P224 | 3, 4 | 6, 9, 12; 8 | tower | |
| R15 | P224 | 2 | 18 | tower | target 0 only; ~11 min predicted |
| R16 | SL24 | 2 | 20, 22 | tower | target 0 only each; 22 also null |
| R17 | SL31 | 3 | 15 | tower | target 0 only; recorded m=3 line is 6,6,7,7 at N=9..18 |
| R18 | SL24 | 2 | **24** | tower | target 0 only; **the frontier cell** (`t = 12`, past recorded `N = 22`); 2 threads |
| R19 | SL26 | 2 | 22 | tower | target 0 only; `t = 11` (second prime past `N = 20`) |

- **Time rule.** No run starts later than 7 hours after R0 started. A run not
  started is reported as not run, never as a result. R18 is started first among
  the long cells if the host's free memory at its start is at least 10 GB, else
  last; its start order is recorded in `progress.txt`.
- **Budgets.** `--budget 1800` s for every cell except R15-R17, R19 (`--budget
  7200`) and R18 (`--budget 14400`).

### 2.4 Recorded comparators (same engine, `f4_fp_tower`)

| `m` | `N` | recorded `D` (Kummer, `p0`/`p1`/`p2`; planted or random targets) | source |
|--:|:--|:--|:--|
| 2 | 2, 4, ..., 20 | 3, 4, 4, 4, 4, 5, 5, 5, 5, 6 | PKM note §10-§11, `pkm_dickson_torus_20261015/PROTOCOL.md` §5 |
| 2 | 22 | 6 | PKM note §11.7 (K1, `p1`) |
| 3 | 6, 9, 12, 15, 18 | 5, 6, 6, 7, 7 | PKM note §16 |
| 4 | 8, 12, 16 | 6, 7, 7 | PKM note §12 |

Recorded multiply-adds at `p1`, `m = 2`: `N = 16`: 6.18e9; `18`: 4.60e10;
`20`: 4.84e11; `22`: 4.88e12 (K1). Predicted `N = 24`: 4e13-5e13 (ratio of the
last two steps, 10.1x, applied once), and `max_nnz` 1.3e9-1.8e9 (ratios 6.7 and
3.9). *Predicted, not measured.* If `max_nnz` exceeds the cap, R18 reports
`D >= d` from its last productive step and the stop reason.

## 3. Hypotheses and decision rules (fixed now)

**Cell value.** A cell `(prime, m, N)` has a value when every target it measured
finished (refuted or solved; not timed out, not stopped for size) with the same
`D`. Targets that disagree make the cell *mixed*; mixed cells count toward
neither hypothesis and are listed.

**Q1.**
- **H_same:** at every `(m, N)` where a Solinas cell and the recorded comparator
  both have a value, they are equal.
- **H_diff:** at least one pair differs (listed with both values).
- *Predicted: H_same* (PKM note §3.4 and every earlier round: the tower
  equations' top forms are `y^2` whatever the prime, so the F4 computation is
  the same up to the numbers it computes with).

**Q2.**
- The `m = 2` line over `N = 2...24` is read by rule A1 (PKM note §11.6),
  computed by `pkm_tower_check analyze`: H0 needs an increase at least every 4 in
  `N` over the upper half; H1a needs a final plateau `L > 10` (impossible at
  `N <= 24` after a rise at `N = 20`, stated in advance); anything else is
  inconclusive.
- *Predicted:* `D = 6` or `7` at `N = 24`; both read *inconclusive* under A1
  (a rise to 7 gives one more level, not a rate). Stated so that neither
  outcome can later be called a surprise.
- If R18 stops for size, `D >= d` is all that is claimed.

**Q3.** Tower `D` against null `D` at each `(m, N)`: reported (H1b), gates
nothing.

**Q4.** H_same_size: for every `(m, N)` measured at two or more arithmetic
backends, `D`, width, step count and multiply-add count are all equal after the
thread count is held fixed. Differences listed.

**Positive control.** R0 passes if `D = 5` for `N = 4...10` on both targets.

**Invalid** if R0 fails or `verify` contradicts any finished row.

## 4. End-to-end pricing (Q5)

### 4.1 Boundary and reference (AGENTS.md §1)

- **Floor:** `sqrt(pi/4) = 0.886` in `S` (generic-group, ecbench records it).
- **Reference:** `rho.negation`, run by ecbench on the same curves, in the same
  session and binary. Matched rho for each IC arm is the rho arm of the same
  workload and round. Unit: `S = total GAE / sqrt(r)` (one table, one unit).

### 4.2 Curves

Four Solinas curves `y^2 = x^3 + ax + b` over `SL20, SL24, SL26, SL28`, found by
`prime_tower_curves --solinas 20:14,24:14,26:12,28:16 --seed 20261022
--max-cofactor 4` (a new `--solinas n:k,...` mode of the existing example: same
curve search, same seed rule, `p` fixed to `2^n - 2^k + 1` instead of drawn).
Their matched controls are the four prior-ladder Proth curves of the same bit
size (20, 24, 26, 28 bits; `research/pkm_tower_ecbench_20261013/curves.jsonl`
rows 1, 3, 4, 5, `p = c*2^12 + 1`), re-measured in the same sessions. Curves are
registered (AGENTS.md §11) before they are cited.

### 4.3 Arms (one spec per size, both curves in the spec)

| arm | role | method |
|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` |
| `ic-mitm` | baseline | `ic.pipeline`, `prime-kummer:t=T`, `mitm:m=3,negation_folded=1` |
| `ic-rootfind` | candidate | `ic.pipeline`, `prime-kummer:t=T`, `rootfind-mitm:m=3` |
| `ic-mitm-tf4` | factor-base record for §4.4 (20- and 24-bit specs only) | `ic.pipeline`, `prime-kummer:t=10` (20-bit) or `t=12` (24-bit), `mitm:m=3,negation_folded=1`; its record supplies the on-curve `columns` of the F4 cell's height; its `S` is reported but is not a comparison arm |
| `rho-neg-aa` | control | `rho.negation` (A/A) |

*Amendment 1 (2026-10-22, before any ecbench session, F4 runs may have started):*
the draft arm list had an `ic-abscissa` control (plain base of the tower base's
size). It is dropped: a single `prime-abscissa:size=S` parameter cannot match two
curves with different abscissa counts in one spec, and the prior ladder measured
tower against plain base within 1.2% (P3 there). `ic-mitm-tf4` is added as above.

`T = 8, 9, 10, 11` at 20, 24, 26, 28 bits (the prior ladder's rule
`t = ceil((bits + log2 6)/3)`, every `t <= k`). `max_trials = 200000`,
`linalg = incremental-gauss`, 8 one-target workloads per curve, 3 measured
rounds, `warmup = 1`, `order = alternate`, `isolation_required = L2`
(operation counts need no level; wall time is descriptive only, host is shared
and stays below L2). Audited with `ecbench verify --replay-all`.

### 4.4 The F4 arm is not an ecbench arm: derived, labelled, lower bound

`f4_fp_tower` is not an `ic.pipeline` oracle, and this round does not build
one (as in the prior ladder). Its priced cost is derived from *measured*
components, with the formula frozen now.

For a curve with `log2 r` and tower height `t = N/2` at `m = 2`:

```
GAE_F4  =  R * c_F4,   R = columns + 10,
c_F4(w) =  madds / M  +  w * U,
S_F4(w) =  GAE_F4(w) / sqrt(r)
```

- `columns` = number of base abscissae on the curve (from the ecbench arm's
  factor-base record); `R` relations are needed, each costs **at least one**
  F4 solve, so this is a **lower bound** (it omits failed targets, the factor
  base build and the linear algebra).
- `madds` = measured F4 multiply-adds for one system at `(prime, m = 2, N)`,
  the mean over the cell's measured random targets. Only cells R4 (SL20,
  `N = 20`, matches the 20-bit curve, `t = 10`) and R16/R18 (SL24 `N = 24`,
  matches the 24-bit curve, `t = 12`; if R18 completes) are priced; the 26- and
  28-bit curves need `N = 26, 28`, unmeasured, and get **no F4 row** (never
  extrapolated into a row).
- `M` = field multiply-adds per GAE, **measured**: `M = ns_GAE / ns_madd`,
  where `ns_GAE` is the wall time of one counted `PrimeCurve` addition at that
  prime (microbenchmark in `examples/pkm_p224_shape_report.rs`, 10^7
  additions, median of 5) and `ns_madd` is the engine's measured wall time per
  multiply-add on that cell (`ms / madds`; this includes the engine's own
  bookkeeping, so it is a full wall conversion, conservative against F4).
  Both on the same host, same session of the report.
- `U` = F4's unpriced native operations that its report emits: `critical_pairs_reduced +
  tower_pairs_reduced + reducer_rows + max_rows` per system (row and pair
  construction, symbolic preprocessing); priced at `w` in {0, 0.1, 1} GAE each,
  the same band the ecbench arms get. Counters F4 does not emit are named in
  the report as unpriced (so the F4 row stays a lower bound).
- The rho reference and the ecbench IC arms are banded by `ecbench band`
  (their `canonicalisations_uncharged`-style counters, all emitted).

An order between two arms is claimed only where it holds at every `w` in the
band (AGENTS.md §5a); otherwise "unresolved". A calibrated cost is never set
against an uncalibrated one: the band alone is used.

## 5. Success, stop and abandon conditions

- **Success criterion (owner, 2026-10-15).** A verified, measured improvement of
  the end-to-end index-calculus baseline (the prior ladder's best verified
  `ic-mitm`/`ic-rootfind` `S` on the matched Proth curves of the same bit size)
  on the same instances is a success. A Solinas-shape row is an improvement
  only if its `S_IC / S_rho` is below the Proth control's at the same bit size
  with the paired/cluster interval excluding no change, in more than one size.
  Relabelling and accounting are not improvements. A clean negative is a valid
  deliverable.
- **Distance to rho** is reported on every row as `S_arm / S_rho` with the band.
- **Abandon the solver axis** (stop adding cells) if H_same holds on every valued
  cell through the frontier cell: the shape adds nothing the prior rounds did not
  measure.
- **Stop a cell** at its budget or `--max-nnz`; record it as out of budget.
  Preserve every failure and timeout (rows, logs, `progress.txt`).
- **Stop the whole run** if R0 fails (instrument invalid) or `verify` contradicts
  a row: explain before reading anything else.

## 6. Verdict classes (AGENTS.md §3), fixed now

| outcome | class |
|:--|:--|
| H_same on all valued cells, F4 arm far above rho, table arms equal to the Proth control within 20% | **inconclusive on the solver axis; negative on price**; no improvement; shape adds nothing measurable |
| table arms of Solinas curves below Proth controls with excluded-1 intervals at >= 2 sizes | **engineering** (an improvement of the baseline), after a replay |
| `D` at some Solinas cell below the recorded and the null by a margin that H_diff lists | solver-axis **lead**, not a verdict; needs its own protocol |
| numbers differ only by arithmetic or accounting | **accounting** |
| the headline count falls while `S` rises | **relabelling** |

The F4 solver axis can never read as an advance here: it prices no pipeline
end to end (§4.4), and the PKM note §3.7 shows no oracle beats rho at `m <= 3`.

## 7. Predictions (stated before any run)

- **P1.** R0 gives `D = 5` at `m = 2, N = 4...10`.
- **P2.** H_same: every valued Solinas cell equals the recorded `D`; `D` rises
  from 5 to 6 at `N = 20` at every prime; width and step counts match the
  recorded ones at `p1`/`p2` where the same `(m, N)` ran.
- **P3.** `D(N = 24)` is 6 or 7 (A1 reads inconclusive either way).
- **P4.** Tower and null `D` agree at every cell (as in every earlier round).
- **P5.** Multiply-add counts at one thread are equal across backends at a given
  `(m, N)` (the engine is the same computation).
- **P6.** ecbench: `S_IC/S_rho` on Solinas curves is within 20% of the matched
  Proth control at each size (the tower base costs the same as a plain base,
  prior ladder P3), and grows with `r` at about `p^{1/6}`
  (0.17 bits of `log2(S_IC/S_rho)` per bit of `log2 r`).
- **P7.** The F4 arm's lower-bound `S` is at least `10^5` times rho's at
  `N = 20` and larger at `N = 24`, at every `w` in the band.

## 8. Inadmissible

Changing primes, cells, flags, seeds, arms, `t`, the pricing formula or the
band after the first measured run; dropping a target; re-running a system for
a different answer; counting `BudgetExceeded` as "does not decompose"; leaving
refutations out of a cost without saying the row is a lower bound; quoting a
per-call solver cost as a speed (it is a stage diagnostic, AGENTS.md §2);
extrapolating a missing cell into a table row; setting a calibrated cost
against an uncalibrated one; comparing a restricted-`p` row with a generic-`p`
row without saying so.

## 9. Scope, stated in advance

Nothing here attacks P-224. The wide cells use P-224's *prime* with a tower of
height `t <= 8`; an attack needs `|V|^m ~ r`, i.e. `N ~ 224`. The measured
`N <= 24` are 11% of that. Anything at `N ~ 224` is extrapolation (labelled).
Per AGENTS.md "Intermediate-cost and asymmetric-access hypotheses", a bounded
null is not nonexistence of structure.

## 10. Amendment 2 (2026-10-22, after R18 ended; committed before the run it adds)

**What happened (measured, from the kernel log).** R18 (SL24, `m = 2`,
`N = 24`, one target, 2 threads, `--max-nnz 1.8e9`, `ulimit -v` 9 GB) started
at 04:19 UTC and was killed by the kernel's memory-cgroup OOM killer at about
06:45 UTC (`Killed process ... pkm_tower_pilot, anon-rss 6,007,580 kB`),
invoked by another agent's process in the same container cgroup. It wrote no
row and no log line (the pilot prints a system's line when the system ends). So
R18 has **no `D` and no lower bound**, and per §2.2 it is recorded as out of
budget (memory) and is not re-run. `progress.txt` shows `exit 0` for it: that is
a bug in `run_f4.sh`'s `echo` (the `$?` it prints is the one of the `date`
command substitution, so every `end ... exit` line of the F4 runs reads 0 and
carries no information). Whether a run finished is read from its summary line in
`runs/*.log` instead; the script is fixed after the batch (`run_f4.sh` in the
final tree) and the old text is kept in this paragraph.

**Added cell R18b (sizing, not a replacement for R18).** Same system as R18
(`--p 16760833 --m 2 --controls tower --t-min 12 --max-t 12 --targets 0`,
same seed, same engine) with `--max-nnz 600000000 --trace --budget 7200`, 2
threads, `ulimit -v` 5 GB. It stops at the first step whose `B'` passes 6e8
entries and reports the stop step and what F4 knew at it. A stopped run gives
`D >= d` only from a step that added elements to the basis at degree `d` (PKM
note §16.1); otherwise it gives no bound. Read under A1 exactly as §3 Q2 says
for a stopped cell: `D >= 6` or `D >= 7` from a productive step leaves the
`m = 2` line inconclusive; nothing from R18b can read H0 or H1a. R18b is run
now, concurrently with the rest of the main batch (single thread each, 2 for
R18b), if free memory at its start is at least 6 GB; otherwise after the
batch. *(Edited before R18b started: the first draft said "after the batch".)*

## 11. Amendment 3 (2026-10-22, after the ecbench sessions, no measurement affected)

§4.2 said the four Solinas curves are registered (AGENTS.md §11) before they are
cited. They are not: adding them to `docs/curves/registry.json` changes the digest
that the cover, trait and transfer certificates bind, which needs those
regenerated (a separate, large change). The write-up therefore names the curves by
`(p, a, b)` and writes no slug in Markdown; the sealed sessions carry the slugs.
Registration is left as a follow-up.
