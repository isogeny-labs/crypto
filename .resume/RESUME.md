# Resume: crypto @ claude/isogeny-class-ic-yield-20261102

## Goal
User request (2026-11-02): measure index-calculus relation yield, solve time,
degree of regularity (Dreg), solving degree and related parameters across the
isogeny classes of ECC2K-130 and ECCp-131, and report per-curve differences.

## Lead / hypothesis
Within an isogeny class only transport cost, Aut and model presentation can
differ (research/isogeny_class_difficulty_20261008). Prediction: null (no
curve effect beyond A/A and presentation). Prior ECC2K-130 work:
cryptanalysis/experiments/ecc2k130-isogeny-class/, research/isogeny_conductor_gap_ic_20261010,
koblitz-isogeny ICC study, cryptanalysis/ecc2k130/research/volcano-m83/.
ECCp-131 never measured. A real curve effect larger than the presentation
control would revive the lead; a null closes it.

## Spec
research/isogeny_class_ic_yield_20261102/DESIGN.md (class structure, parts
P1-P3). Per-part PROTOCOL.md in p1_eccp131/, p2_prime_analogues/,
p3_ecc2k130/ (written by Sonnet builder agents; to be frozen before runs).
ECCp-131: p=0x48e1d43f293469e33194c43186b3abc0b, conductor 1, Elkies l<=150:
5,7,13,17,29,37,61,73,83,89,107,131,149 (x2), 67 (x1).
ECC2K-130: conductor 263*146505763881528721, ecdb curve id 22.

## State (updated 2026-11-02 ~10:00Z)
- P1 tier 1 DONE (card -1 attempt 2, detached), masked analysis frozen (6199f52d57), key opened (26f3758222).
  Every arm (27 Elkies neighbours, iso, 27 random non-isogenous curves) identical to E0 in D_solve, rank, field_mults,
  per-degree profile; only AA_perm (variable reorder) moves mults (0.976-1.165). Instrument curve-independent; null
  uninformative about isogeny specifically. 1008 planted rows not formed (factor base < m abscissae).
- P2: cells done and committed: c24A_m2 c28A_m3 c24A_m3 c32A_m3 c24B_m2 c32A_m2 c28A_m2. c32A_m3 max|z| flag
  (curve 80, +7.25%, p=0.0005) -> preregistered follow-up 1 (FOLLOWUP_c32A_m3.md, note F1.1, card p2fu1) gated to run
  after the P2 queue. Deviation to report: executed specs used R=200 (generator default), protocol text says min(100,0.6N).
- P3: done and committed: setup, a-ctrl-k3, a-main-k3, a-m2, a-main-k5, a-ctrl-k4-s0/s1, a-main-k4-s0/s1, b-ctrl,
  b-main-k34. b-badapt -1 timed out (untimed solution count, TIMEOUT_NOTE.md), reissued -2; b-bsparse -2.
- Report pipeline: report/build.sh (generator gen/gen_report.rs, typst via scratch venv), rerun when all data is in.
- Infra: executor Bash kills background commands at 30 min -> all runs launched detached (LAUNCH_NOTE.md) via
  /tmp/.../scratchpad/card_queue.sh (also copied below). Outputs copied into main with gzip >1MiB (repo 10MiB limit).

## Next steps
1. Collect builder outputs; review protocols; freeze them (commit).
2. experiment-director compiles task cards (route: isolab-eks unless director finds it unavailable).
3. executor-mechanical runs cards; director assesses; main session analyses.
4. Report + graphs + PDF (research-visuals), PR ready for review.

## Runs in flight
- PR #3306 MERGED 2026-11-02T17:58Z (P1, P3, P2 11/12). Owner decision: c28B-m2 + curve-80 follow-up + report rebuild go in a NEW follow-up PR from this branch (restarted from main), auto-merge once green.
Detached queues (logs in /tmp/claude-0/-home-user/fafe457a-a14a-53c5-a937-e53817fc07eb/scratchpad/queue_*.log; card
outputs in each worktree's cards/<id>.launch.log + receipt; copy finished outputs into main and commit):
- P2 queue in /home/user/crypto-p2run (pin b2d892912): cells c28B-m3 (running), c24B-m3, c32B-m3, c32B-m2, c28B-m2.
- P2 follow-up gate (gate_p2fu1.log): runs CARD-isogeny-ic-yield-p2fu1-c32A-m3-20261102-1 in /home/user/crypto-p2fu
  (pin ef9d986d75) when queue_p2.log prints "queue done".
- P3 lane A2 in /home/user/crypto-p3run (pin d97f5953b2): b-badapt-2 (running), b-bsparse-2, b-main-k5-s1-1.
- P3 lane B: b-stdv-s0 (running), b-stdv-s1, b-main-k5-s0.
If a fresh session finds a queue dead without "queue done": read the last launch.log, have experiment-director assess,
move partials to _failed/, relaunch the remaining cards with card_queue.sh (detached).

## Stop and success conditions
Budget: about one day of cluster CPU. Success = complete, controlled per-curve
measurements for P1-P3 with a verdict per metric (curve effect vs A/A and
presentation bands). Not an IC-baseline improvement claim either way.

## Open questions
- Is the isolab-eks route reachable from this cloud session (no kubectl; AWS user `claude` creds present)?
