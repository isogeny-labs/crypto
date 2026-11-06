# Current chord proof task - source only
## Goal
Prove the exact chord encoding needed by the user's twist useful-yield/source-DLP program. The user explicitly approved checkpoint uploads for continued work; routine start/finish/plan checkpoints are authorized, with no paid resources or arbitrary result uploads.
## Lead / hypothesis
The accepted target-aware chord proposal, merged PR3450, needs faithful distinct-root and diagonal algebra before any native solver admission. Sources and prior work are linked in research/quadratic_twist_chord_preflight_20261105/ALGORITHM.md.
## Spec
research/quadratic_twist_chord_proof_20261105/PROTOCOL.md and Chord.lean. Mathlib d13f23b723b8a846827a245b89c10fc7d3f11612; Lean4.34.1. Existing LOCAL resources only, two workers max,600s stage caps,3600s iteration.
## State
New branch codex/twist-chord-proof from current main4268660fa364601cbb6c42f4099677209aa35213. Five proposed algebra declarations written; ZERO proof builds/scientific runs on this branch so far. Prior merged worktrees/evidence untouched. No new measured rank/degree/cost or DLP result.
## Next steps
Commit exact source, pass current official checkpoint gate, then run pinned lake env lean on Chord.lean with600s cap; retain raw output and failed sources. Review axioms and declaration fidelity. Complete group coverage/native census only in later explicitly frozen stages.
## Runs in flight
None. Next proof command has not launched. Logs will live in /workspace/twist-chord-proof-20261105 outside the checkout and be preserved for publication.
## Stop and success conditions
Stop each stage at600s and the focused scientific window at3600s. No paid compute or token fallback. Kernel errors are retained failures. All five declarations plus allowed-axiom audit complete this algebra component only; point-group coverage, native solver census, useful-yield/source-scalar/regularity objectives remain open.

# Preserved incoming resume (historical task state)
# Resume: crypto @ main e87fa18a73 (IC line-layout tournament v6, V6-AMENDMENT-1 merged in #3531)

## Goal
Run IC-LINE-LAYOUT-E2E-FP28-V6-20261106. It tests whether `layout=lines` lowers the fp28 IC wall time against the walk incumbent at equal counted cost.

## Lead / hypothesis
Candidate: `mitm:negation_folded=1,layout=lines` against the incumbent `prime-abscissa:size=407`, `mitm:negation_folded=1`, `targets=walk`, `max_trials=26479085`.
- Confirm: both wall sessions pass analyze-v6.sh.
- Kill: per the PROTOCOL-V6.md stopping rule.

## Spec
- Protocol: research/ic_line_layout_tournament_20261031/protocol-v6.json, frozen (sha256 12415faa…, commit 7c173e5a1d, merged in PR 3527 as f352040f06).
- Decision: DEC-20261106-662e33.
- Specs: spec-fp28-v6{,-qual,-counted}.json. Launch requests: launch-v6/.
- Pinned image: ea8e9ef7 / ecbench 73362eed.

## State
- Qualification passed (§5.1). Wall s1 and s2 exit 0, both on node ip-10-84-67-216, data SEALED (S3 runs/ic-line-v6-s{1,2}-20261106/).
- Counted companion ic-line-v6-counted-20261106: exit 42 (accepted by §7), audit OK, node ip-10-84-72-222.
- V6-AMENDMENT-1 frozen (protocol_sha256 2aedaaf8…) and merged in #3531 at 05:33Z. The owner approved launching s3 after the merge.
- A worker restart at about 05:50Z wiped /dev/shm: the old cards, clones and downloads are gone. Work now lives in the session scratchpad v6/ on the main disk.

## Next steps
1. Director rewrites card-s3 for the new paths; launch ic-line-v6-s3-20261106 via executor-mechanical.
2. After s3 has written job-exit.json: node check, then analyze-v6.sh on (s1,s2) and (s3,s1), validator, report and results PR.

## Runs in flight
- None at checkpoint time; s3 is about to launch.

## Stop and success conditions
- Budget: USD 5 ceiling, rolling hard-bound check (§14.1). Spend so far USD 0.155.
- The qualification requires all executions at L2 and a minimum solve of at least 5 ms.
- Never redispatch a consumed registration, and never auto-retry an ambiguous submission.

## Open questions
- Work is in the session scratchpad on the main disk (about 7 GB free); /dev/shm does not survive restarts.
- /home/user/crypto-autoresearcher is half-switched, pending the owner.

# Preserved previous branch resume (historical task state)
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
Multi-curve re-baseline done (PCC-20261105-37a78d): plateau 1.57-1.91x rho across five family curves per size; next: owner decision or new structural idea.
Specs: research/curve25519_autolab_20261104/multicurve/spec-k*.json (frozen before data); siblings from `curve25519_family siblings`. Binaries rebuilt from main.
After it: add-raw, metric per size (mean S/rho over curves and its curve-to-curve spread), close the record, pcc claim, README row, commit, PR. If the incumbents' ratios move materially, update the ledger incumbents and OBSTRUCTION.md.

## Stop and success conditions
Stop: per-cell caps in the protocols; Part A stops at a given m after two
consecutive n cap; t = 4 prime cells do not run if t = 3 caps. Success is any
decided cell with a verified planted control; the preregistered kill criteria
close the respective m >= 5 lines. No cap is raised after the freezes.

# Round 5 (2026-11-04, written before any round-5 run): solver side

## Derived first (no runs; awk on recorded logs)
- Cost model with LA exponent ω from 1.0 to 2.0: beating rho at 2^256 needs ω ≤ ~1.3 and β = 0.
- Effective sparse-LA exponent of the recorded msolve matrices: 2.41–2.72. Sparsity does not get there.

## Test H: character-hybrid on the separators
Branch on the quadratic character of each separator, adding X_c = n_j·z_c² with n_j ∈ {1, a non-square}:
2 branches per separator.
- Cells: T at m3 t4 (D = 6, 3.7 s), m3 t5 (D = 6, 512 s) and m4 t4 (D = 7, 697 s). Run the planted (true) branch and one false branch each.
- Prediction (Elekes–Szabó reading): D_H = D_T in the true branch (no degree drop), and the false branch costs about the same time.
- Win condition: D drops by ≥ 1 per guessed bit, or total time over all branches < T's time.
- Runs: msolve 1 thread, 1800 s cap, 6 GiB.

## Round 5, test H outcome (recorded before test L)
- At m3 t4, Htrue and Hfalse match T exactly (D = 6, widest 7,077), each with quotient dimension 6 (z ↦ −z).
- Reason: over F̄_p every X is c·z²; F_p-rationality is not a low-degree algebraic constraint.
- m3 t5 was stopped at 42 s, still on T's matrices; m4 t4 was not run.

## Test L (fat-leaf tree, written before any L run)
- m = 4 as one triple (S4sym in e1, e2, e3 with output X1, degree-3 remainder chain) plus one leaf (Kummer chain), root S3(X1, x4, xR).
- That is 1 separator against T's 2 separators (X1, X2) with a symmetric root.
- Prediction (excess ∝ number of separators): D_L(4,4) ≤ 6 < D_T = 7.
- Falsified if D_L ≥ 7. Cells: m4 at t4 and t5, k0.

## Test Y (function coupling, written before any Y run)
- m = 3, first cherry by chord: per-root Kummer chains for w1 and w2; y1, y2 with y_i² = f(g·w_i); slope lam with lam·(x2 − x1) = y2 − y1; X1 = lam² − x1 − x2.
- Then the x-only root S3(X1, g·w3, xR) with leaf 3 on a Kummer chain.
- The coupling is a function of the inputs (no 2-to-2 correspondence). Solutions are not quotiented: ordered pair × signs.
- Prediction: D_Y ≥ D_T (6 at t = 4), since lost symmetry and degree-3 curve equations offset the determinism. Win condition: D_Y ≤ 5 at t = 4 or 5.
- Cells: m3 at t4 and t5, k0.

## Round 6 (2026-11-04): local long run of the P2 cell m5 t4 (written before the run)
- The big host is blocked: AWS STS rejects the keys, the lab routes run only ecbench specs (≤ 3,600 s), and no other route is authorized.
- Locally: T at m5 t4 (round 3; cut off at 2,400 s while reducing degree 9, memory not exhausted), rerun alone with TLIMIT = 18,000 s and VLIMIT = 14.5 GiB.
- Registered reading (from round 3): if the run completes with D = 9 then E(5) = 5 ≤ 5 and P2 holds at m = 5; D ≥ 10 means P2 fails at m = 5; another cut-off is still undecided.

## Round 7d: decisive Riemann–Roch cell (written before the run)
- RR at m5 t4 (round7_rr/genrr.sing, input in scratchpad rr/logs/m5_t4_k0_RR.ms), alone on the host after the memory-lean F4 run exits: 13 GiB cap, 10,800 s.
- Reading, from round7_rr's kill criterion:
  - D_RR ≤ 8 (below T's ≥ 9) with a finished run: RR beats T in D at m = 5.
  - Improvement only if the widest matrix is also below T's (≥ 999,374 cut off).
  - D_RR ≥ 9 means no gain at m = 5.
  - Cut off again: undecided.

## Round 7e: E1Q at m4 t5 (written before the run)
- Edwards 2-torsion-quotient tree, input round7_edwards m4_t5_E1Q.ms (sha256 5592ad57…f099).
- Runs alone after RR m5 t4 exits, with a 13 GiB cap and 7,200 s.
- Reading:
  - D_E1Q(4,5) finishes ≤ 6 (T and E1 are ≥ 7 there): the excess stays at least 1 below T at m = 4 as t grows.
  - D = 7: the quotient's gain at m = 4 vanishes at t = 5.
  - Cut off again: undecided.


# Current twist proof continuation, 2026-11-06
All preceding resume content is preserved; no unrelated worker is started by this task. The following task-specific sections describe this branch and its actual next run.

## Goal
Prove the exact chord encoding needed by the user's twist useful-yield/source-DLP program. The user explicitly approved checkpoint uploads for continued work; routine start/finish/plan checkpoints are authorized, with no paid resources or arbitrary result uploads.
## Lead / hypothesis
The accepted target-aware chord proposal, merged PR3450, needs faithful distinct-root and diagonal algebra before any native solver admission. Sources and prior work are linked in research/quadratic_twist_chord_preflight_20261105/ALGORITHM.md.
## Spec
research/quadratic_twist_chord_proof_20261105/PROTOCOL.md and Chord.lean. Mathlib d13f23b723b8a846827a245b89c10fc7d3f11612; Lean4.34.1. Existing LOCAL resources only, two workers max,600s stage caps,3600s iteration.
## State
New branch codex/twist-chord-proof from current main4268660fa364601cbb6c42f4099677209aa35213. Five proposed algebra declarations written; ZERO proof builds/scientific runs on this branch so far. Exact-branch publication/checkpoint authorization was confirmed by the user on2026-11-06. Prior merged worktrees/evidence untouched. No new measured rank/degree/cost or DLP result.
## Next steps
Commit exact source, pass current official checkpoint gate, then run pinned lake env lean on Chord.lean with600s cap; retain raw output and failed sources. Review axioms and declaration fidelity. Complete group coverage/native census only in later explicitly frozen stages.
## Runs in flight
None. Next proof command has not launched. Logs will live in /workspace/twist-chord-proof-20261105 outside the checkout and be preserved for publication.
## Stop and success conditions
Stop each stage at600s and the focused scientific window at3600s. No paid compute or token fallback. Kernel errors are retained failures. All five declarations plus allowed-axiom audit complete this algebra component only; point-group coverage, native solver census, useful-yield/source-scalar/regularity objectives remain open.
