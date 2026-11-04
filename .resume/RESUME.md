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
Round 35 (k = 24, 28, 32; seed=run wwalkG grid) running under benchrec record in scratchpad rec35, branch claude/curve25519-autolab-r35 (stacked on PR #3416).
Panel: ksum4 at extrapolated N = 37 and neighbours 19/26/52/74; ksum3 transfer N = 176/246; mitm-prime-batch / blocked exploration.
Branch claude/curve25519-ic-autolab-round4 (PR #3392). After it: add-raw, metric (incumbent S at k = 32, matched rho), close the record, claim, commit.

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
