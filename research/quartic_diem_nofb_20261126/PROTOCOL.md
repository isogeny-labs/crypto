# Protocol: skip the full-core fallback in quartic Diem `2lp-sfb-sge`

Status: **DRAFT, not frozen.** It is frozen by `frozen.sha256` (this file, `seeds.json`, `analyze.py` and `run_*.sh`), committed before any grid run. Every number this study produces is `[pcc:unverified]`.

Parent: `research/quartic_diem_la_20261123` (merged in #5081; validated with errata).

## 1. Question

When the square coordinate solve fails, `2lp-sfb-sge` spends a full-core scalar Wiedemann solve on the same attempt (the fallback) before it moves to the next attempt. In the parent's 12 observed fallbacks (4 development pairs, 8 grid pairs) that solve never certified, because t was undetermined in the attempt's system (parent ERRATA E6). Runs with a fallback carried 46–83% of the arm's linear algebra at p = 16411, 65537 and 131101 [pcc:unverified].

The question: on the same instances and solver seeds, how much counted work does a variant that skips the fallback save, and how often does the skip give up an attempt the fallback would have certified (a **rescue**)?

## 2. Arms (one binary)

Binary: `genus_ladder.nofb`, built from this branch. Its hash goes in `binary.sha256` and the source hashes in `source.sha256` at freeze.

- `2lp-sfb-sge` (`Q_diem_2lp_sfb_sge`), the baseline. Unchanged from the parent, with W = 8, FILL = 1.5, EXCESS = 64 (the parent's frozen setting). It now also reports `la_fallback_rescues`, the attempts its fallback certified.
- `2lp-sfb-sge-nofb` (`Q_diem_2lp_sfb_sge_nofb`): identical, except that a failed square solve skips the fallback and the relation loop goes on to the next attempt. It reports `la_fallbacks_skipped`.
  - The fallback draws only from a clone of the solver RNG (`wrng = rng.clone()`), and the main stream is advanced by `skip_wiedemann_draws` in both arms. Every later draw is therefore identical.
  - So without a rescue, the arms see the same relations and attempts, and `nofb` does the baseline's work minus the fallbacks: the full-core Wiedemann and its failed `[t]G = T` check.
  - With a rescue, `nofb` needs a later attempt, fully counted.
  - Test `sfb_sge_nofb_pairs_with_sge` checks these identities.

## 3. Grid, seeds and commands (`seeds.json`)

- **Instances.** The parent's 20 prime-order plane quartics, by curve seed:
  - 4099: 35, 205, 220, 257, 295
  - 16411: 93, 158, 252, 366, 382
  - 65537: 13, 57, 156, 166, 369
  - 131101: 53, 177, 193, 228, 288

  These are passed as `--quartic-seeds` (same admission path, no search). One target per curve (target seed = curve seed). **Intentional corpus reuse:** the instances are the ones whose rows suggested this lead.
- **Solver seeds.**
  - 1 and 2 are the parent's panel: they replicate the rows that motivated the change and anchor C0.
  - 3 and 4 are **held-out**, never run before. They form the primary panel for the decision.
- **Runs.** Both arms on every (curve, seed), baseline first: 4 primes × 5 curves × 4 seeds × 2 arms = 160 runs, at most 2 threads.
  - `run_main.sh` covers 4099, 16411 and 65537 with 2 threads.
  - Then `run_131101.sh` covers 131101 with 1 thread.
- **Seed manifest.** `seeds.json` gives the exact derivations (§3 of `docs/experiment-seeds.md`). Workload keys (curve, target, solver seed) exclude the arm, and the solver stream is shared by both arms: the pairing is exact.

## 4. Preregistered prediction

1. **Rescues.** No rescue on any of the 80 baseline runs.
2. **Single-attempt pairs.** On pairs where the baseline has no fallback, the two arms' rows are equal on every counted field.
3. **Fallback pairs.** On pairs with a fallback, the `nofb` linear algebra is 15–40% of the baseline's.
4. **Pooled ratio.** The pooled paired GAE₁ ratio on the held-out panel lies in [0.60, 0.95].

## 5. Analysis and decision rule (`analyze.py`)

- **Cost.** GAE_w = `group_ops` + w × (all native operations: `rel_mul + rel_add + la_mul + la_add + idx_lookups + idx_inserts`), for w ∈ {0, 0.1, 1} (AGENTS.md §5a).
- **Paired ratio.** GAE_w(nofb) / GAE_w(sge) per (curve, seed). Curve level is the geometric mean over the panel's seeds. Per prime, the geometric mean over curves, with a cluster bootstrap over curves (4000 replicates, seed 20261126). Pooled gives equal weight per prime. Reported for the primary panel (seeds 3, 4) and for all seeds.
- **IC / ideal negation rho.** Per arm and prime at each w, over all seeds. The verdict follows the parent: BELOW-RHO CANDIDATE only if all three upper bounds are below 1, ABOVE RHO if all lower bounds are above 1, otherwise UNRESOLVED.
- **Integrity.** Every row is certified with `recovered == t_secret`; both arms recover the same log; every pair is complete; C0 holds, meaning the baseline rows for seeds 1 and 2 equal the parent's `2lp-sfb-sge` rows on the 24 fields listed in `analyze.py`. Any failure makes the study VOID.
- **Decision, on the primary panel.**
  - **IMPROVEMENT** if the pooled paired ratio has upper 95% < 1 at w = 1 and at w = 0.1, and ≤ 1 at w = 0. This is an order that holds across the band, not a tolerance.
  - **KILL** if the pooled w = 1 lower 95% is ≥ 0.98.
  - **INCONCLUSIVE** otherwise.
- **Scope of the claim.** An IMPROVEMENT is a component speed-up of the index-calculus baseline, not a rho claim. Rescues are reported whatever the decision.

## 6. Outputs

- `raw/`, `data.sha256`, `analysis.json`
- `RESULTS.md`, `fig_nofb.svg` and `.png`, and a short PDF report
- an independent validator report, then the benchmarks row (validation gate)

## 7. Execution route

These are counted-operation measurements, deterministic for a given binary and seed. They run locally on this container (≤ 2 threads), as the parent did. The default EKS route (CLAUDE.md, AGENTS.md §9a) is not available here: the container has no AWS identity or CLI. Wall time is recorded but is not a metric.
