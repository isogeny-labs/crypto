# Resume: crypto @ claude/arity-completion-20261104 (2026-11-04)

## Goal
Complete the arity dimension of elliptic-curve index calculus with the
preregistered cells no record covers: PKM prime towers at m = 5 (N = 10, 15,
20); chained-S3 SAT with a Frobenius-union base at m = 5 and 6 on Koblitz
n = 23..41; the same at n = 83 with the cell cap lifted to 1800 s. Study
directory `research/arity_completion_20261104/` (README there), PR #3378.

## Lead / hypothesis
Prime towers: D(m=5, N=10) in {6, 7}, D(m=5, N=15) in {7, 8}, D(m=5, N=20)
>= 8 or capped; tower kind does not change D; N = 15 finishes in 7200 s and
8 GiB. Kill: D >= 8 at N <= 15 closes m = 5 on prime towers.
Koblitz: m = 5, 6 decide at n = 23 within 600 s for S = 2; cost grows so that
n = 37, 41 cap at m = 6; planted controls always recover; n = 83 stays
undecided at 1800 s; no decided query at n >= 31 under the 188 s m = 4
root-table parity point. Kill: K4 and K5 both supported.

## Spec
- `research/arity_completion_20261104/prime_tower_m5/protocol.json`, frozen at
  commit 1fd6ece55f (sha 2b82eecd…); commands in arms[*].instrument.flags.
- `research/arity_completion_20261104/koblitz_m5_m6/protocol.json` + grid.tsv
  + run_shard.sh (freeze block records its commit once frozen).
- Cards: `research/arity_completion_20261104/cards/CARD-arity-prime-m5-*.yaml`
  (lint CARD OK); Koblitz cards follow the same pattern after its freeze.
- Instruments: `examples/pkm_tower_pilot.rs` (engine tower, p 2013265921),
  `examples/n83_msweep_cell.rs` (chain-s3 and chain-s3-orbit, union:S base).

## State
- Protocols validated with crypto-autoresearcher tools/experiment_protocol.py
  (main commit, files fetched raw); prime-tower frozen before data.
- No cell has run. Development release builds in /dev/shm only.
- Launch gates fail in the authoring container: STS InvalidClientTokenId, no
  checkpoint upload token (problems/arity-completion-launch-gates.md).

## Next steps
1. From a session with account 104186811093 (or an upload token): push this
   checkpoint, run the build cards, re-dispatch the director with the binary
   sha256 for the cell cards, submit through isolab-eks per AGENTS.md §9a.
2. Run controls first; a failed control voids the study (both protocols).
3. analyze.py per study; export with experiment_protocol.py from-rows;
   report, figure, PDF; no leaderboard row (stage diagnostics).
4. dreg v4 refreeze steps a–h in research/dreg_high_arity_20261101/README.md.

## Runs in flight
None. Nothing has been launched anywhere.

## Stop and success conditions
Stop: per-cell caps in the protocols; Part A stops at a given m after two
consecutive n cap; t = 4 prime cells do not run if t = 3 caps. Success is any
decided cell with a verified planted control; the preregistered kill criteria
close the respective m >= 5 lines. No cap is raised after the freezes.
