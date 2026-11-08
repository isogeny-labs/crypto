# P1 round 2: complete the capped cells, the first m = 5 cell with a tower, and arm F (protocol, frozen 2026-11-08)

Follows round 1 (`PROTOCOL.md`, `RESULTS.md`, red team `../red-team-p1-p2.md`). Same
instrument (`p1tool`, extended with `--system f` in Addendum A of this file), same
msolve 0.6.5, same curves, same seed rule (integers 1..50, first two admissible),
same instances (one seed gives the same instance to RR, T and F). Stage diagnostic,
algebraic track; `S`, end-to-end cost, rho comparison unset. User request
(2026-11-08): pursue the next steps of the round-1 report.

## Arms and limits

| part | cells | systems | memory (VLIMIT) | time (TLIMIT) |
|---|---|---|---|---|
| R2a, complete round 1's caps | T24 m3 t5, T31 m3 t5, p0 m3 t5, T24 m5 t3 | T only (RR completed in round 1) | 6 GiB (6,291,456 KiB; round 7's T limit) | 3,600 s |
| R2c, arm F | T24, T31, p0: m3 t3, t4, t5 | F | 6 GiB | 3,600 s |
| R2b, first m = 5 cell with a tower | T24 m5 t4 | RR, T | 13 GiB (13,631,488 KiB; round 7's m5 t4 limit) | 10,800 s |

R2a and R2c use round 1's admissible seeds per cell. R2b takes the first admissible
seed in 1..50 only (one target per system: each job may take three hours). R2b runs
alone on the host, after every other round-2 job, in its own card.

## Reading capped jobs (frozen now, before data)

The red team confirmed that a capped job's reported degree and widest matrix are lower
bounds on the completed values (the parser counts completed F4 steps only). Round 2
uses that reading: "RR below X" on a metric is **decided** when RR completed and its
value is below X's value or X's lower bound; **not below** when both completed and
RR ≥ X; **undecided** otherwise. Nothing is imputed.

## Decision rules (frozen)

1. **Control:** the R2a T jobs at m3 t5 on p0 reproduce round 7's completed T at m3 t5
   on its own curve within D ± 1 and widest ± 5 % (round 7: D 6, widest 23,720; the
   curve differs, so only the order of magnitude is checked); any completed job with
   quotient dimension 0 is a control failure.
2. **Round-1 rule 6 restated with the bound reading:** at T24 m5 t3, RR (round 1: D 5,
   widest 2,878) against T (R2a). Reported, and labelled as the tower-free cell.
3. **m = 5 with a tower (R2b):** RR below T on D and widest at T24 m5 t4 → "RR
   advantage holds at m = 5 with a tower" (the repository's round7_rr criterion);
   RR below on D only → "trade"; RR not below on D → "fails at m = 5"; otherwise
   "undecided".
4. **RR vs F at m = 3 (R2c):** per cell, "RR dominates" (below on D and widest),
   "trade" (below on one, not the other), "F dominates" (F below on both), or
   "undecided". Prediction (round 7, Kummer, other curve): trade at t4 and t5, F
   narrower.
5. **RR vs T at m3 t5 (R2a):** decided now that T completes at 6 GiB; prediction
   (red team): D lower for RR, widest larger (trade).

## Sources checked

KB unavailable. `research/ic_tree_gluing_20261103/{gen.sing, run.sh, rows.tsv,
round7/README.md, round7_rr/}`, round-1 files of this directory.

## Addendum A: instrument (pinned before any claim cell)

Pending the builder's report (arm F emitter and its controls).
