# Cross-track review of the prime-field IC prep program (2026-10-29)

This is a skeptical review of the six prep tracks in
`research/prime_ic_program_20261029/`: `ruling/`, `reference_arm/`,
`fall_degree_relaunch/`, `table_b_gate/` and `hashtable_prefetch/`, plus the
lead-atlas edits in crypto-autoresearcher `knowledge/leads/ecdlp/`. Nothing was
run apart from read-only checks. No number below is new: each one is either
quoted from a file, with its path, or re-derived by hand from quoted values.
Checkout: crypto `main` at `7ffaab06b`.

## 1. Verdict

The tracks are careful, and almost every number I checked matches its source
(§2). There are three substantive problems:

1. **Owner decisions went stale.** `DECISIONS.md` (program root, written after
   the tracks) records R1 = Option C, plus D1 yes, D2 yes, D3 allowed and
   D-EKS approved. Every track was still written against the 2026-10-24
   wording, where `prime-table` is reference only and `prime-rootfind` is
   unclassified, and treated D1–D4 as open. The ruling memo recommends
   Option B, which was not adopted. I added status notes to each track (§3).
2. **`DECISIONS.md` contradicts itself.** D2 says to rank the Table B incumbent
   against the published algebraic reference. Option C's adopted text
   (`ruling/README.md` §5.3) says the two tracks "are never ranked against each
   other". I did not edit `DECISIONS.md`, because it is the owner record and
   sits outside the track subdirectories. I also left the reference arm's
   decision rule C3 as "context only". This has to be settled before the
   reference-arm freeze.
3. **The atlas carried a stale published-arm plan.** The PKM lead told readers
   to build the "standard-base FGHR-symmetrised S_{m+1} + msolve" arm from
   `ic_sweeper` §2.5. The reference-arm track shows FGHR's 2-torsion invariants
   do not apply, because every Table B prime curve has odd order
   (`docs/curves/registry.json`: 823, 65003, 1046999, 10933753 checked). It
   recommends Semaev's naive small-abscissa system with msolve instead. Fixed
   in the lead.

## 2. Spot checks (verified against source)

| claim (track) | source | result |
|:--|:--|:--|
| rfsub2 579.571 vs sub2 579.547 (20 b); 1489.249 vs 1489.223 (24 b); rfsub3 = mitm3 at w = 0, n32 41.282 and n64 24.509 (ruling) | `research/prime_base_oracle_tuning_20261029/select/band-train-P20.md`, `-P24.md` | match |
| derived ~572× and ~1,150× rho-neg for rfsub2; mitm2 9.9× and 8.1× (ruling) | same files (rho-neg 1.013, 1.295) | match (572.1, 1150.0, 9.91, 8.13) |
| planner: rootfind-subtract 171 / 233 / 439× the plan (ruling) | `research/ic_planner_validation_20261020/README.md` | match |
| `RootfindSubtractOracle` docs, `batch_inv`, m = 2 / m = 3 dispatch; `rootfind.rs` lines 6–7; `build_ic_leaderboard.py` line 2380; `methods.rs` ~4171 `pkm-semaev` (ruling) | code at `7ffaab06b` | match |
| SWEEPER tracks table and "No algebraic prime space" (ruling, atlas) | `docs/ic/SWEEPER.md` 145–168 | match |
| Table B rows: all `mitm_m2_negfold_walk*`; 20 b N = 51, 6195.7 trials; IC/rho 5.47–13.46 (ruling, ref arm, atlas) | `docs/ic/leaderboard.json` | match |
| 0.818 [0.773, 0.866] / 0.726 at w = 1; gap 9.47 → 7.75; stride 0.809; random-base control absent (all) | tuning `README.md`, `analysis/table-holdout.md` | match |
| msolve 0.6.5-1build2 wrong above 2^16; fit exponents 3.80 / 4.41; medians 0.120 / 3.55 / 6.97 / 52.9 s; msolve pin a18d0de, 71 tests (ref arm) | crypto-autoresearcher `msolve.py`, `index_calculus/README.md`, math-readiness README | match |
| `protocol.draft.json` validates (ref arm) | `tools/experiment_protocol.py validate` | `OK ... (0 finding(s))` |
| ecbench image 147,172,601 B; request `seconds` max 3600; lab rustc 1.99.1; pfd Dockerfile `rust:1.88-bookworm` (ref arm, fall degree) | `taskq/deploy/eks-karpenter/*`, study `isolab/Dockerfile` | match |
| round-2 x_r and D at t = 6–11; ms 5848 / 57196 / 610517; cap-5 at t = 11 not refuting (fall degree) | `research/pkm_tower_round2_20260925/runs/*.jsonl` | match |
| medians 3,4,4,4,4,5,5,5,5,6 "not re-derived"; λ = 0.133 at t = 14, 0.71 for m = 4 at t = 8 (fall degree) | `RESTART.md`; λ formula by hand | match |
| m = 4 N = 16: 76,837 columns, 55.5 min, 8.06 GB; m = 4 N = 20 at 240–380 GB (fall degree, atlas) | `RESEARCH_PKM_TOWER_ORACLE.md` §12.7, line 3258 | match |
| `docs/ic/runs/VALIDATION.json` byte-identical to `research/ic_validation_gate_20261024/B/VALIDATION.docs-ic-runs.json`; `validation_gate.py evidence docs/ic/runs` gives `valid`; band 4.658–16.30 (gate) | `cmp`; `scripts/validation_gate.py`; `gate_status.out.txt` | match |
| PR #2811 for the tuning study (atlas) | `git log --merges` | match |
| 318.1 ns trial, touch 30.6 / 63.6 ns; guard 56.3 and oracle 80.9 ns (prefetch) | PR #2806 `research/ic_wide_ext_20261028/results.json` at `db02dd4f`, fetched read-only through the GitHub API | match. Guard and oracle are TSC cycles (118.2, 170.0) ÷ 2.1 GHz, which the README discloses |
| Coppersmith closure 0.3293 / 0.3536 vs 8/9; linear ladder slope 0.647, 36,440× (atlas) | `research/smallroot_optimal_reach_20261026/README.md`, `research/ic_m2_linear_ladder_20261024/REPORT.md` | match |

**The patch is plausible (prefetch track).** `patch.diff` still begins with
"UNCOMPILED AND UNTESTED". I fetched its four base files at `db02dd4f`
(read-only, into the scratchpad). `patch -p1 --dry-run` applies cleanly there,
and the identifiers it uses exist there: `Params::get -> Option<&str>`,
`u64_or`, `collect_and_solve_relations_touching`. Against `main` it fails, as
the patch header says: the `stages.rs` hunk is rejected. One known compile
risk remains, the if/else of slice literals of different lengths in
`parameters()`, which the track already lists.

**Atlas check (after my edits).** `python3 tools/lead_atlas.py --check`
prints `lead atlas OK: 148 leads {...}; coverage 2092/2092 (0 unmapped)`.
`--strict-paths` still reports the 7 errors that predate this review: paths
from the unmerged PRs #2783, #2806 and #2807.

## 3. Issues and fixes

| # | track | issue | fix |
|:--|:--|:--|:--|
| 1 | all | `DECISIONS.md` R1/D1–D4 not reflected; tracks still say "reference only / unclassified / D1–D4 open" | status notes added to `ruling/README.md`, `ruling/classification.json` (`adopted_option`), `reference_arm/README.md`, `reference_arm/PROTOCOL.md` (track column), `table_b_gate/README.md` (owner item 1), `hashtable_prefetch/README.md` (track status) and the atlas leads (rootfind, constant-engineering, generic-table) |
| 2 | DECISIONS | D2 ("rank") vs Option C ("never ranked") | flagged only; C3 left "context only"; owner or main session must reconcile before freeze |
| 3 | atlas: pkm-tower lead | next_action names the FGHR arm, which does not apply to odd-order curves; "only prime-field route" ignores D1 | rewritten to point to `reference_arm/` and D1 |
| 4 | atlas: rootfind lead | "default oracle in every Table A/B row" (flagged by the ruling memo, not fixed by the atlas track) | replaced with the recorded variants from `leaderboard.json` |
| 5 | atlas: constant-engineering lead | evidence note "all Table B rows unvalidated (pre-gate)" contradicts its own best_measured and the gate dossier (validated, with caveats) | note corrected; hash-table floor updated to ~168/188 ns including the touch (derived, prefetch track); "IC baseline" relabelled as pair-table / generic |
| 6 | ruling §1.4 | "ranked it below the selected base" read as "worse"; the source says `stride < sel` in S (stride no more expensive) | reworded, with 0.818 added beside 0.809 |
| 7 | ruling §1.3 | attributed the 0.004% rfsub2/sub2 gap at w = 0 to native counters, but native work is unpriced at w = 0 (`band.rs`: S = G − C) | marked untraced and unverified |
| 8 | ruling §7.4 | asks for the rootfind-lead correction | noted as done (#4); the `build_ic_leaderboard.py` line is still open, outside write scope |
| 9 | reference arm §6 | model trial count "about 10,270" vs table 10,280 vs 1046999/102 = 10,265 | text corrected, rounding explained |
| 10 | reference arm PROTOCOL | `inc` reads as if it were the Table B row, but Table B came from `ic boundary` (round-5 ledger), not ecbench `ic.pipeline` | note added: `inc` is a same-session re-measurement, not expected to equal Table B S exactly |
| 11 | fall degree §2.6(b) | "medians equal D at N = 2..20 (§17)" compares P1 medians with D recorded at p₀ and the torus primes | clarified. At P1 only t = 6–10 (round-2 rows) are committed, and they agree |
| 12 | prefetch §1.1 / §1.2 | "touch 30.6 to 63.6 ns" mixes the 28-bit 32-lane and 32-bit 1024-lane cells, while the floor (168/188) uses the 32-lane cells (30.6 / 47.9) | reworded; ns values marked as TSC cycles converted at 2.1 GHz |
| 13 | fall degree, gate, ref arm, prefetch | no plan or estimate is presented as measured; the derived budgets (49 CPU-h, 1.7 h / 17 h / 7 d, 240–380 GB) are all labelled derived | no change |

## 4. Still open (not fixable from here)

- Reconcile D2 with Option C, then edit `AGENTS.md`, `docs/ic/SWEEPER.md` and
  `src/bin/ic/sweep/space.rs` (`Track::Generic`). The board relabel goes
  through `scripts/build_ic_leaderboard.py`, and its line 2380 still says no
  algebraic prime oracle exists.
- Nothing is committed. The atlas edits (by the atlas track and by this
  review) and the program directory need one PR with a session-log entry and
  the research-visuals report and PDF. This container has no PDF engine.
- The blockers are unchanged: disk and cargo for the reference-arm code and
  the prefetch patch, an EKS identity for the second-host replay and the
  reference arm, the harness host for the fall-degree relaunch, and the Mac
  files.
