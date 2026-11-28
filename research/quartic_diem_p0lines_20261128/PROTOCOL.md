# Protocol: lines through P0 in quartic Diem `2lp-sfb-sge-nofb`

Status: **FROZEN 2026-11-28**, by `frozen.sha256` (this file, `seeds.json`, `analyze.py` and `run_*.sh`). Committed before any grid run.
- **Binary:** `genus_ladder.p0`, built from the freeze commit after the freeze. Its sha256 goes in `binary.sha256` and the source hashes in `source.sha256`.
- **Development data:** only in `dev/`, on curves 4093 and 8191, which are outside the grid.
- Every number this study produces is `[pcc:unverified]`.

Parent: `research/quartic_diem_nofb_20261126` (merged in #5216; validated). Diagnostics: #5269.

## 1. Question and mechanism

In `2lp-sfb-sge-nofb`, a run whose first attempt leaves t undetermined pays a second attempt: 8 / 20, 2 / 20 and 5 / 20 runs at p = 16411, 65537 and 131101 in the parent. Such a run costs 1.5–1.9× a single-attempt run [pcc:unverified].

The diagnosis (`dev/diag/`, 57 attempt cores dumped on the dev curves, exact elimination mod r) found:
- The line rows of a core leave dL free directions. These are 1–2 dense directions (scale, and the base-point shift `v_deg`: every λ_P + 1, κ + 4) plus z ∈ {0, 1, 2} columns that occur only in decomposition rows.
- t is determined exactly when the linked decomposition rows add at least dL + 1 to the rank. Every failed first attempt falls short.
- `v_deg` is free in 45 of 57 cores, because no line through P0 survives into the core.
- Offline, adding line rows through P0 would rescue 7 of the 11 failing cores.

A line relation costs about 280 counted operations at p = 16411. One linked decomposition relation costs about 1.9·10⁵.

The question: does seeding the relation set with the lines through P0 cut the counted work of `2lp-sfb-sge-nofb`, paired on the same instances and solver seeds?

## 2. Arms (one binary)

- **`2lp-sfb-sge-nofb`** (`Q_diem_2lp_sfb_sge_nofb`), the baseline. Unchanged: W = 8, FILL = 1.5, EXCESS = 64, 4 linked decomposition rows before the first attempt.
- **`2lp-sfb-sge-p0`** (`Q_diem_2lp_sfb_sge_p0`), the primary arm. Identical, except that before the relation loop it adds the lines through P0 and each factor-base point, one per factor-base point. They are counted like every line (`rel_*`), and `p0_lines` / `p0_line_relations` record them.
- **`2lp-sfb-sge-p0i3`** (`Q_diem_2lp_sfb_sge_p0i3`), the secondary arm. Like `p0`, but with 3 linked decomposition rows before the first attempt instead of 4: with `v_deg` gone, 3 rows keep the margin of one row that 4 gave.

The P0 lines change the relation set and the moment of the first cycle check, so the arms are paired by workload (curve, target, solver seed), not by identical relation streams. The solver RNG stream is keyed by the solver seed alone.

## 3. Grid, seeds and commands (`seeds.json`)

- **Instances:** the parent's 20 prime-order plane quartics, by curve seed. One target per curve (target seed = curve seed). This is intentional corpus reuse.
  - 4099: 35, 205, 220, 257, 295
  - 16411: 93, 158, 252, 366, 382
  - 65537: 13, 57, 156, 166, 369
  - 131101: 53, 177, 193, 228, 288
- **Solver seeds 1–6.** The binary runs seeds 1..N.
  - Seeds 1–4 were run in the parent and anchor C0.
  - Seeds 5 and 6 are **held-out**, never run before, and form the primary panel.
- **Runs:** 4 primes × 5 curves × 6 seeds × 3 arms = 360, arm order nofb, p0, p0i3.
  - `run_main.sh` covers 4099, 16411 and 65537 with 2 threads.
  - `run_131101.sh` then covers 131101 with 1 thread.

## 4. Preregistered predictions

1. Second attempts: on the primary panel, the share of `p0` runs needing more than one attempt is at most half of the `nofb` share, pooled over primes.
2. Pooled paired GAE₁ ratio p0 / nofb on the primary panel lies in [0.75, 0.97].
3. `p0i3` / nofb lies in [0.75, 1.02], and `p0i3` needs more second attempts than `p0`.

## 5. Analysis and decision rule (`analyze.py`)

- **Cost.** GAE_w = `group_ops` + w × (`rel_mul + rel_add + la_mul + la_add + idx_lookups + idx_inserts`), for w ∈ {0, 0.1, 1} (AGENTS.md §5a).
- **Paired ratios.** arm / nofb per (curve, seed).
  - Curve level: the geometric mean over the panel's seeds.
  - Per prime: the geometric mean over curves, with a cluster bootstrap over curves (4000 replicates, seed 20261128).
  - Pooled: equal weight per prime.
  - Panels: primary (seeds 5, 6) and all seeds (1–6). p0i3 / p0 is also reported.
- **IC / ideal negation rho** per arm, prime and w, over all seeds, with the parent's verdict rule:
  - BELOW-RHO CANDIDATE only if all three upper bounds are below 1;
  - ABOVE RHO if all lower bounds are above 1;
  - otherwise UNRESOLVED.
- **Integrity.** Any failure makes the study VOID.
  - Every row is certified with `recovered == t_secret`, and all arms recover the same log.
  - Every (curve, seed) has all three arms.
  - **C0:** the `nofb` rows for seeds 1–4 equal the parent's `Q_diem_2lp_sfb_sge_nofb` rows on the 24 fields listed in `analyze.py`.
  - Every `p0`/`p0i3` row has `p0_line_relations > 0`, and every `nofb` row has `p0_lines == 0`.
- **Decision, on the primary panel, p0 / nofb.**
  - **IMPROVEMENT** if the pooled upper 95% is below 1 at w = 1 and at w = 0.1, and at most 1 at w = 0. That is an order holding across the band.
  - **KILL** if the pooled w = 1 lower 95% is at least 0.98.
  - **INCONCLUSIVE** otherwise.
  - The same rule is reported for p0i3 / nofb as a secondary verdict.
- **Scope.** An IMPROVEMENT is a component speed-up of the index-calculus baseline, not a rho claim.

## 6. Outputs

- `raw/`, `data.sha256`, `analysis.json`
- `RESULTS.md`, a figure (SVG and PNG) and a short PDF report
- an independent validator report, then the benchmarks row (validation gate)

## 7. Execution route

These are counted-operation measurements, deterministic for a given binary and seed. They run locally in this container with at most 2 threads, as the parent did. The default EKS route (CLAUDE.md, AGENTS.md §9a) is not available: the container's AWS key is rejected (InvalidClientTokenId), and there is no AWS CLI. Wall time is recorded but is not a metric.
