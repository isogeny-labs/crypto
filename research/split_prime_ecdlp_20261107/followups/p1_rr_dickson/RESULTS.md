# P1 results: Riemann–Roch system with a Dickson-torus tower (run-20261107T214621Z)

**Outcome under the frozen rules: rule 5, stage-level positive at m = 3;
rule 6 undecided** [pcc:unverified]. On the NIST-shape primes T24 and T31
(p ≡ 3 mod 4, 2^t | p + 1), the separator-free Riemann–Roch system (RR) carries
the Dickson-torus membership tower with exactly the F4 solving degree it has
with the Kummer tower, and it beats the cherry-tree chain system (T) on both
solving degree and widest matrix at m3 t3 and m3 t4 on both primes. The
proposal's predictions held exactly (D_RR = 4 at m3 t4, 5 at m3 t5; D_T = 6 at
m3 t4). This is a transfer of round7_rr's Kummer result to the field shape of
the NIST prime curves, not a new mechanism, and a stage diagnostic: `S`,
end-to-end cost and any rho comparison are unset.

- Protocol: [`PROTOCOL.md`](PROTOCOL.md) (frozen 535d9da826; Addendum A before any
  claim cell; merged in #3657).
- Card `CARD-p1-run-01` (sha256 `e4828616…9694`), receipt `cards/p1-run-01.receipt.json`,
  director assessment [`cards/p1-run-01.assessment.md`](cards/p1-run-01.assessment.md):
  **executed as specified** (40 jobs = 10 cells × 2 seeds × 2 systems; seed skip
  rule as frozen; 32 completed, 8 capped at 1.5 GiB, none imputed).
- Raw: `runs/run-20261107T214621Z/` (`parsed.jsonl`, `jobs.tsv`, `cells.tsv`,
  `skips.tsv`, per-cell systems, plants and msolve logs).
- Analysis: `analysis/p1analyze` (Rust) → `analysis/derived/{p1_cells.tsv, decision.json}`.
  Figure: `figures/p1_cells.svg`. Red-team review: `../red-team-p1-p2.md`.

## Measurements (exact counts; max over the 2 seeds per cell)

| cell | D RR | D T | widest RR | widest T | status |
|---|---:|---:|---:|---:|---|
| T24 m3 t3 | 4 | 5 | 632 | 883 | both completed |
| T24 m3 t4 | 4 | 6 | 2,801 | 7,129 | both completed |
| T24 m3 t5 | 5 | ≥ 6 | 31,911 | ≥ 23,852 | T capped |
| T24 m5 t3 | 5 | ≥ 8 | 2,878 | ≥ 39,898 | T capped |
| T31 m3 t3 | 4 | 5 | 632 | 883 | both completed |
| T31 m3 t4 | 4 | 6 | 2,801 | 7,129 | both completed |
| T31 m3 t5 | 5 | ≥ 6 | 31,911 | ≥ 23,852 | T capped |
| p0 (Kummer) m3 t3 | 4 | 5 | 632 | 883 | both completed |
| p0 (Kummer) m3 t4 | 4 | 6 | 2,801 | 7,077 | both completed |
| p0 (Kummer) m3 t5 | 5 | ≥ 6 | 31,870 | ≥ 23,720 | T capped |

Every completed job has quotient dimension ≥ 1 (the planted relation is found).

## Frozen rules, in order

1. **Control:** RR-Kummer D at m3 t4 = 4 (round7_rr: 4); no completed job lost the
   planted relation. Not triggered. (The analyzer first read the capped jobs' missing
   quotient dimension as 0; fixed to read completed jobs only, before any verdict was
   reported.)
2. **Killed:** no; RR is below T on D in every cell where the comparison is decided.
3. **Transfer fails:** no; D_RR(Dickson) = D_RR(Kummer) at every shared (m, t)
   (4, 4, 5), and the widest matrices agree to within 0.2 %.
4. **Trade:** no, in the cells where both arms completed.
5. **Stage-level positive at m = 3:** **yes**, at m3 t3 and m3 t4 on both T24 and
   T31 (D and widest both lower for RR).
6. **Repository criterion (m = 5 cell):** **undecided under the frozen text.** At
   m5 t3, RR completed (D = 5, widest 2,878) and T was capped at 1.5 GiB after
   reaching D = 8 and widest 39,898. The frozen protocol says capped jobs are
   recorded and not imputed, and calls rule 6 undecided when neither m = 5 arm
   completes; it does not say how to decide when one arm is capped. A post-data
   reading, which reads the capped values as lower bounds (sound: the red team
   confirmed that the parser counts only completed F4 steps, and the capped T log
   had already built a 60,855 × 82,927 matrix), would give "met". It is labelled as
   such and is **not** the reported outcome, for the second reason below.

## Why rule 6 decides little even under the post-data reading (red team, high)

- **At m5 t3 the RR system has no tower.** With t = 3 the membership is a single
  division U·Q = D_8(X) − c (`runs/.../D24_m5_t3/rr_s2.ms`, 16 equations); the base
  holds 8 abscissae (6 or 7 on E) and the relation uses 5 of them. The cell meets the
  letter of round7_rr's "at least one m ≥ 5 cell" but does not show RR beating T in a
  cell with a tower. Round 7's deciding cell was m5 t4, where RR was 25 to 60 times
  wider than T at equal degree.
- **At m3 t5 the width advantage most likely reverses.** RR's widest (31,911) exceeds
  T's capped lower bound (23,852), and round 7's completed T run at m3 t5 (6 GiB) had
  widest 23,720.
- **T is not the lab's strongest m = 3 system.** Round 7's arm F has widest 1,224 at
  m3 t4 and 10,493 at m3 t5, against RR's 2,801 and 31,870; against F, RR is a trade
  (lower D, wider matrices). That comparison is across different curves and memory
  limits and is not combined into a claim here.

## Limits and caveats

- Memory: the protocol froze round7_rr's 1.5 GiB for both systems; round 7's T runs
  used 6 GiB (`research/ic_tree_gluing_20261103/run.sh`), so T caps here where it
  completed there. With sound lower bounds a cap can only leave a comparison
  undecided, not bias it toward RR (red team: rejected as a bias).
- The predictions are not blind: tiny-prime smoke values (D 4 vs 6 at m3 t4) were
  seen before Addendum A (disclosed there).
- The relay added only the `BENCHREC=` prefix; the receipt's argv is the card's.

## Next discriminating steps (cheapest first)

1. Rerun T at m5 t3 with ≥ 6 GiB, then RR and T at m5 t4 (the first m = 5 cell
   with a tower), on a ≥ 16 GiB host.
2. Add round 7's arm F at m3 t3 to t5 on the same instances, so RR is compared with
   the lab's best m = 3 system.
