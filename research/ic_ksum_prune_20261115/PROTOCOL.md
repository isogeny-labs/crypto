---
title: "Protocol: ksum prune=1 against prune=0 on Table B's eight curves"
date: 2026-11-15
status: registered before any evidence run (hashes in REGISTRATION.sha256, registered-at.txt)
---

# Protocol: ksum `prune=1` against `prune=0`, paired, on the curves of the two Table B k-sum studies `[pcc:unverified]`

**Track: generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS).** Nothing here moves the algebraic track.

## 1. Lever and question

Commit 6d6aa93b8f added `prune=1` to the `ksum` oracle (`KSumTable::build_negation_folded_opts`, `src/cryptanalysis/ic_boundary.rs`;
parameter `prune` of `KSumOracle` in `src/cryptanalysis/ic_framework/plugins.rs`). With it the table build does not add the lower-level
partial sums that can never be completed to m summands. Claimed by the commit: same table, keys, inserts and collisions; only the build adds
drop (by count about 10-12% of the build adds at m = 5, about 3% at m = 4). Default off. Prediction (director, before data): about 5% lower S
for `ksum5` at w = 0, less at w = 1 (native work unchanged), about 1-2% for `ksum4`. **Question: is the pruned arm cheaper than the
unpruned arm in the counted unit, row by row, at every weight of the AGENTS.md section 5a band, with identical outputs?** An accounting-only
change is not an improvement (success criterion); here the counted adds are real group additions removed from the build, which the
correctness check below tests (no other counter may move).

*Sources checked (KB review unavailable: the research-kb MCP server is not loaded in this container; the lead atlas is encrypted/unread):*
`research/ic_tableb_ksum_20261115/` and `research/ic_tableb_binary_ksum_20261115/` (PROTOCOL, specs, analysis, band), commit 6d6aa93b8f
(message and diff), `src/cryptanalysis/ic_framework/plugins.rs` (`KSumOracle::parameters`, `params.flag("prune")`).

## 2. Design

**Rows.** The eight curves of the two sibling studies (`rows.mjs`; slugs checked by `ecbench plan`): prime P20, P22, P24 (ICV1
`icv1-fp20-t1293-3c2999ba`, `icv1-fp22-tm1267-a5acaf6d`, `icv1-fp24-t1577-77336155`), binary B15, B18, B21, B24, B27
(`icv1-f2m15-t131-ed7256f0`, `icv1-f2m18-t393-d2c04152`, `icv1-f2m21-t2591-de54d72e`, `icv1-f2m24-tm3047-2d0c0e8c`,
`icv1-f2m27-t12543-569dca8b`). 3 replicate sessions per row: 24 sessions, `specs/<row>-r<1..3>.json`, written by `make_specs.mjs` from the
sibling spec of the same name: same curve, `target_seed`, measurement `seed`, `targets_per_curve` = 48, 3 measured rounds, 1 warm-up,
order `alternate`, isolation L0, same factor-base sizes for the k-sum arms (not re-tuned).

**Arms** (every arm sees the same curve, targets and algorithm seed per round): `rho-neg` (reference), `rho-neg-aa` (control),
`ksum4`, `ksum4-p`, `ksum5`, `ksum5-p`. The `-p` arms differ from their twin only by `prune=1` in the oracle string
(`ksum:m=5,prune=1`; `max_trials` 13528399621 as in the siblings). The label of every spec starts `ic_ksum_prune_20261115`.

**Binary.** `target/release/ecbench`, `cargo build --release --locked --bin ecbench` at commit 3a9e85e644 (clean tree; `src/` identical to
71d56d2af0); sha256 in `BINARY.sha256` (`6c60b4c931294bf770039a5b47800e39c424eb2e7f4876b426cd084f7c364d77`).
**Route.** Local, `--cpus none`, isolation L0, counted cost only; one session at a time, with the CPU lock held. This deviates from the
default isolab-eks route of CLAUDE.md, as both sibling studies did, at the experiment director's instruction; counted cost does not depend on
the host; wall time is not claimed. **Verification.** `ecbench verify --dir <session> --replay-all --exit-code` on every session, same
binary, same host (weaker than a second host class). `run_all.sh` runs and verifies.

**Pilot.** None. The lever was not sized on any data of this study; no value of any run enters this protocol.

## 3. Registered tests (AGENTS.md section 5a band: w = 0, 0.1, 1 GAE per unpriced native operation)

For k in {5 (primary), 4 (secondary)} and a row, `R_w = cost(ksum<k>-p) / cost(ksum<k>)`, paired by target and round:
`ecbench compare --dir <the 3 sessions> --a ksum<k> --b ksum<k>-p --weight w --resamples 4000 --seed 20261002` (the printed
`ratio_b_over_a`), and the same per replicate session. The tested upper end is the larger of the linearised-t and the bootstrap 95% upper ends.

**Holds on a row** if the upper end of `R_w` is below 1 at w = 0, 0.1 and 1, pooled **and** in each of the 3 replicate sessions, **and** the
gain exceeds the A/A noise. Noise is defined two ways, both reported, the first being the verdict:
1. *paired A/A gate (verdict, `holds`):* `1 - max_w upper(R_w) > max_w |R_aa(w) - 1|`, with `R_aa(w)` the same paired ratio of `rho-neg-aa`
   over `rho-neg` (identical code, identical seeds: expected exactly 1, so this gate reduces to the upper end lying below 1; reported, not
   hidden). Rationale: the two arms compared have identical outputs and share every seed, so the sampling error of the pairing is what
   `compare` prints; the replicate seed spread does not enter a paired ratio.
2. *sibling delta gate (`holds_sibling_delta`):* the siblings' rule, `1 - max_w upper(R_w) > delta - 1` with delta the largest max/min ratio
   of one arm's mean S (w = 0) across the 3 replicate sessions over `rho-neg`, `ksum<k>`, `ksum<k>-p` (a different-seed spread, so
   expected to be far larger than a ~5% gain; reported as the conservative reading).

**Secondary** (`ksum4`): same, reported separately, never used to rescue `ksum5`.

**Correctness (per target, every round including warm-up, every session; `analysis/correctness.py`).** `ksum<k>-p` must equal `ksum<k>` on:
outcome status and recovered log, `counters` (relations found, targets tried, matrix rank/rows/work), `factor_base`, every phase's `native`
and `work_native` (relation count, trials, lookups, target-guard probes, `table_insert_attempts`, ...), every phase except `oracle_setup` in adds,
doubles, scalar multiplications and GAE, the `oracle_setup` doubles and scalar multiplications, and the work-model primitives of every phase
except `oracle_setup` `group_add`; and `oracle_setup` adds of `-p` must not exceed the unpruned arm's. Any other difference is a **failure**,
reported as such. Table entry count (`ksum_table_entries`) is a `setup_native` value; if the records expose it, it is compared as part of
`native`, otherwise insert attempts and hash probes are the proxy and the report says so.

**Distance remaining (reported, not a gate):** `S(ksum<k>-p) / S(rho-neg)` at each weight on each row, next to the unpruned value.

**Predictions** (director's, for the record): `R_0` of `ksum5` about 0.95 (0.93-0.98), of `ksum4` 0.98-0.99; `R_1` closer to 1. A prediction is
held if the observed `R_0` of `ksum5` is within +/-0.03.

**Verdict booleans (summary.json):** `ksum5_p_holds_all_rows`, `ksum4_p_holds_all_rows` (paired A/A gate), the row lists that hold, the
sibling-delta-gate lists, `correctness_ok`, `all_sessions_verified`.

**Not claimed:** wall time, scaling beyond these curves, an algebraic-track result, a security statement, any leaderboard row (no
`VALIDATION.json`, second-host replay certificate or independent validator; the validation gate is not met by this study), any ranking against
tables A and C. All numbers are `[pcc:unverified]`.

## 4. Landing

Study directory with README.md, results.json, summary.json, graph (SVG + PNG), report.pdf; `benchmarks/records/` record
(`tools/benchrec.py`) and regenerated `benchmarks/progress/`; one commit of these paths only. No push, no PR.
