# PKM Kummer chain: F4 degree at m = 8, 9, 10 and N-step ladders at m = 6, 7

Standing: **every number here is [pcc:unverified]**. No pcc claim filed. Local fallback run on a shared
4 vCPU / 15 GB container (L0), not isolab-eks. Not committed or pushed (scratch mirror only).

Follow-up to `research/notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md` section 4 (A4-PKM).

## Route and what blocked EKS

- isolab-eks: no `aws`/`kubectl`; ambient AWS keys rejected by STS (`InvalidClientTokenId`); the route
  also requires the source/spec committed and pushed (forbidden for this task).
- `scripts/checkpoint.sh push`: no upload token and no valid AWS identity. RESUME.md written only in this mirror
  (`.resume/RESUME.md`).
- Director/executor relay: not available to this subagent (no Agent tool).
- `tools/benchrec.py`: run against a scratch git repo in this mirror (record
  `benchmarks/records/20261115T1919Z-pkm-kummer-m8-f4-degree.json`). It was initialised at 19:19Z, about a minute
  after the runner started (19:17:58Z). It was not uploaded to S3.

## Protocol

`PRESET.txt` (sha256 `fd02b3a4…2fcd`) was frozen at 2026-11-15T19:16:49Z, before any system was generated.
- Generator: the parent's `pkmgen.py`, wrapped so that each summand gets its own tower depth (`gen_mixed.py`).
  On uniform depth it is byte-identical to the parent generator, and the new enumerator matches the parent's
  (`selftest.txt`, PASS).
- Ladders: "more N steps" means mixed tower depths. Rung k makes the first k summands one level deeper, so N rises
  one tower bit per rung.
- Arm B (main): P1 = 2013265921, the repository curve, refutation regime. Two random draws per rung.
- Arm A: p = 40961, uniform t = 1, with 2 random draws and 1 planted draw per arity m.
- Instrument: msolve 0.6.5 `-v 2 -g 2`.
  - Memory cap: AS 6e9 bytes.
  - Wall cap: 300 s for nvars ≤ 16, 1200 s otherwise.
  - Total budget: 3 h. Each ladder stops at its first censored r0.

## Results (D = msolve max F4 round degree; min over 2 draws; ">=" = censored)

| m | exact rungs (N: D) | first censored rung | sigma_hat (OLS) | verdict (preset) |
|---|---|---|---|---|
| 6 | 6–10: 6; 11–14: 7 | N=15: ≥8 (memory, 4.6 GB, 883 s) | 0.1667 | KILL (α*₁ = 0.110) |
| 7 | 7: 6; 8–11: 7 | N=12: ≥8 (wall 1200 s) | 0.200 | KILL (α*₁ = 0.135) |
| 8 | 8–11: 7 | N=12: ≥8 (memory) | n/a (ΔN = 3 < 4) | INCONCLUSIVE |
| 9 | 9–12: 7 | N=13: ≥8 (memory) | n/a (ΔN = 3 < 5) | INCONCLUSIVE |
| 10 | 10–11: 7 | N=12: ≥7 (wall 1200 s) | n/a | INCONCLUSIVE |

**Item verdict for m ≥ 8: INCONCLUSIVE** under the frozen rule.

Repeats: m = 6 and m = 7 read KILL by OLS slope.
- m = 6 endpoint slope is 0.125. Under the w = 1 cost model that implies exponent 0.535.
- The m = 7 OLS slope is driven entirely by the N = 7→8 step (the t = 1, |V| = 2 cell). Over N = 8..11, D is flat.

Details per rung: `tables.md`. Full analysis: `analysis.json`. Graph: `D_vs_N.{svg,png,pdf}`.

### Arm A (p = 40961)

| m | random r0 / r1 | planted |
|---|---|---|
| 8 | 7 / 7, unit | 8, non-unit |
| 9 | 7 / 7, unit | ≥7 (censored, 300 s) |
| 10 | 7 / 7, unit | ≥7 (censored, 1200 s) |

At t = 1, D is the same on both primes.

### Checks

55 finished runs were checked; none failed.
- Unit iff the exhaustive grid enumeration finds 0 zeros: 55/55.
- Planted m = 8: the GB vanishes at the planted point.
- The 7 censored runs were not checkable.

### Out-of-preset extension (`EXTENSION.txt`, frozen before its data)

- Run: m = 8, N = 12, r0, with AS 10e9 and a 1800 s wall cap.
- Outcome: censored again at 1800 s, peak RSS 6.4 GB, still inside a degree-7 round (153291 × 202495 matrix, 15641 pairs).
- Max printed degree: 8.
- The verdict does not change.

## Observations (not part of the rule)

- **The two draws gave identical traces in every finished cell.** Round degree sequences and W were identical: the draws
  differ only in the constant x_R. Two draws therefore add no independent information about D.
- **Every censored run reached a degree-8 round and then returned to degree 7.** msolve's degree sequence is not
  monotone. "≥ 8" means "msolve entered a degree-8 round". It is not a certified degree-7 closure failure.
- **Where D first leaves 7:** at m = 7, 8, 9 the first rung with D ≥ 8 had 4–5 summands at depth 2. At m = 6 it had
  3 summands at depth 3. At m ≥ 8, D stayed at 7 for 3–4 consecutive N steps.
- **Exponent reading:** the w = 1 exponent from a flat slope is below 0.5, but a 3-step plateau is not a slope.
  The censored-rung bound gives σ ≥ 0.25 at m = 8 and 9, implied exponent ≥ 0.80 / 0.74. Those bounds rest on
  censored values, so they are not values.
- **Load:** the shared VM's load average reached about 4.8 during the run. Wall-capped runs (m = 7 N = 12,
  m = 10 N = 12) are load-sensitive. Memory-capped runs (m = 6 N = 15, m = 8 N = 12, m = 9 N = 13) are not.

## Open

- The decisive cells are m = 8 N ≥ 12, m = 9 N ≥ 13 and m = 10 N ≥ 12. They need more than 6–10 GB and more than
  30 minutes per run.
- They belong on isolab-eks (memory-family node, ≥ 64 GiB) once a pushed commit and a valid AWS identity exist.
- A degree-7 closure certificate (the parent's `closure.py`) at those cells would turn "≥ 8" into a value.
