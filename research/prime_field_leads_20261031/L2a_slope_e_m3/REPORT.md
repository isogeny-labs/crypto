# L2a: slope e of counted decomposition work at m = 3 (partial ladder, s = 6..11)

**Lead.** L2a of `research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md`;
thresholds in `../PLAN.md` (interval of e entirely below 0.5 reopens; interval
containing 2 closes; null prediction e ≥ 1). **Status.** partial: the frozen
ladder is s = 6..12 and the run stopped during s = 12 (host load; the workflow
was stopped). **Decision.** does not reopen; the G arm closes under the frozen
rule (its interval contains 2); the S arm is above 2 and is reported as observed.

This report was written by the synthesis step (2026-11-01) from the files the
lead left: `results/fit.csv`, `results/per_s_means.csv`,
`results/L2a_work_vs_s.svg`, `raw/run.log`, `src/l2a.rs`, `src/stats.rs`, and
the per-attempt table `attempts.csv` (in the artifact store, `.artifacts.json`,
sha256 `f1520b5e…`). No number is new.

## 1. Design as run (deviations from the lead text named)

- Harness `src/l2a.rs` (standalone Rust, `rustc -O`, no crates). Prime
  p ≈ 2^{3s}, window [0, 2^s), B = 2^s; one fresh prime per s, 3 curves per
  prime, 32 targets per curve (96 attempts per s per arm). Question per
  attempt: do x₁ ≤ x₂, x₃ in the window, each liftable, exist with
  R = ±P₁ ± P₂ ± P₃?
- Arms: **G** group meet-in-the-middle over pairs (x₁ ≤ x₂), four sums
  R ∓ P₁ ∓ P₂ by x-only addition with batched Montgomery inversion, test
  x(sum) < B; **S** algebraic S₄: per pair build the quartic S₄(x₁, x₂, X, x_R)
  in X, find its F_p roots (gcd with X^p − X, Cantor–Zassenhaus), test root < B
  and liftable; **T** triple scan over all x₁ ≤ x₂ ≤ x₃ (s ≤ 8 only, an
  independent check of G and S).
- Deviation: the lead text named the repository's SAT/SMT decomposers as the
  algebraic arm. The agent used the S₄ root-finding arm instead (it builds and
  counts in the same unit); the SAT/SMT encodings were not run.
- Metric: Montgomery field multiplications (every one, inside inversions, root
  finding and window tests included) plus decisions, per attempt. No wall time
  is recorded; the clock is read only for the 20-minute stop guard, which
  never fired.
- Fit: ordinary least squares of log₂(mean work over the 96 attempts) on s,
  bootstrap over attempts (4000 resamples) for the interval. Since B = 2^s and
  p ≈ B³, the slope is the e of note §2 (work per attempt = p^{e/3} = B^e).

## 2. Cells attempted

| s | log₂ p | B | G | S | T |
|---|---|---|---|---|---|
| 6 | 18.66 | 64 | 96 ok | 96 ok | 96 ok |
| 7 | 21.54 | 128 | 96 ok | 96 ok | 96 ok |
| 8 | 24.86 | 256 | 96 ok | 96 ok | 96 ok |
| 9 | 27.19 | 512 | 96 ok | 96 ok | not run (s > 8 by design) |
| 10 | 30.42 | 1024 | 96 ok | 96 ok | — |
| 11 | 33.38 | 2048 | 96 ok | 96 ok | — |
| 12 | 36.68 | 4096 | 32 ok (curve 0 only) | not reached | — |

The s = 12 rows (one curve, G arm only) were removed from `attempts.csv` by
the resuming session so that a clean rerun of s = 12 could append both arms;
that rerun did not happen. Every cell that ran finished (no stop-guard hits);
the ladder is incomplete, not failed.

## 3. Fitted slopes (`results/fit.csv`; interval = bootstrap 2.5 % to 97.5 %)

| arm | class | points (s) | e | interval |
|---|---|---|---|---|
| G | all | 6 | 1.98 | [1.94, 2.01] |
| G | satisfiable | 6 | 2.03 | [1.89, 2.33] |
| G | unsatisfiable | 6 | 1.97 | [1.93, 2.00] |
| S | all | 6 | 2.21 | [2.16, 2.24] |
| S | satisfiable | 6 | 2.24 | [2.09, 2.59] |
| S | unsatisfiable | 6 | 2.20 | [2.15, 2.24] |
| T | all | 3 | 3.21 | [3.00, 3.40] |

Per-s means (`results/per_s_means.csv`): G grows from 1.0·10⁴ multiplications
at s = 6 to 1.04·10⁷ at s = 11 at 19.6 to 24.7 multiplications per pair; S from
8.6·10⁵ to 2.04·10⁹ at 2143 to 4037 multiplications per pair (the per-pair cost
of the gcd with X^p − X grows with log p, which is the excess of e over 2).
Satisfiable attempts are cheaper in level (early exit) and have the same slope.
Graph: `results/L2a_work_vs_s.svg` (log₂ mean work against s, one line per arm,
slope in the legend).

## 4. Decision against the frozen rule

- Reopen needs an interval entirely below e = 0.5. The lowest interval
  measured is [1.94, 2.01]. Not reopened, on any arm or class.
- Close needs an interval containing e = 2. G, all classes: yes (closes for the
  MITM encoding, which is the enumeration-over-pairs cost e = 2 the note
  predicts for MITM at m = 3 under §2's "enumeration gives e = m − 1" for the
  pair stage). S: the interval [2.16, 2.24] lies above 2, so the literal clause
  does not fire; it is recorded as observed: the algebraic arm costs more than
  the group arm by a factor that grows like log p, and nothing in it is below
  the MITM slope. T confirms e ≈ 3 for a full triple scan.
- Null prediction e ≥ 1: met on every arm.

## 5. Limits

- s = 12 is missing; the slopes rest on six points (G, S) and three (T).
- The SAT/SMT decomposers the lead text named were not measured; the S arm is
  an algebraic root-finding encoding, not a solver encoding.
- One prime per s and three curves per prime; curve-to-curve spread is inside
  the bootstrap over attempts, not estimated separately.
- No PDF renderer was available to the lead agent; `REPORT.pdf` was rendered
  by the synthesis step with Typst in a scratch virtualenv.

## 6. Reading

At m = 3 the counted work per decomposition attempt grows like B² for the
group meet-in-the-middle and a little faster than B² for the S₄ root-finding
arm. The threshold that would beat rho at m = 3 is e < 0.5. Nothing measured is
within a factor of four of it in the exponent; the lane stays closed for these
encodings, with the s = 12 point still to be taken.

## Rerun 2026-11-01: s = 6..12 complete (appended; the sections above are unchanged)

Plan `../rerun_20261101/RERUN_PLAN.md`, report `../rerun_20261101/REPORT.md`, data
`../rerun_20261101/L2a/`. The full ladder s = 6..12 was rerun with the same seeds on a
quiet host: every s = 6..11 per-s row reproduces the table above exactly, and all 192
s = 12 attempts (G and S) completed inside the 20-minute guard. Fit over seven points:

| arm | class | e | interval |
|---|---|---|---|
| G | all | 1.97 | [1.95, 2.00] |
| S | all | 2.19 | [2.16, 2.22] |
| T | all (s ≤ 8) | 3.21 | [2.99, 3.40] |

**Status:** complete. **Decision:** does not reopen; G closes (interval contains 2);
S above 2, recorded as observed. The SAT/SMT encodings remain unmeasured.
