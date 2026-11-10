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

## Round 2, card A: capped cells completed and arm F (run-r2a-20261108T053528Z)

Protocol `PROTOCOL-R2.md` (frozen before data, merged in #3693, Addendum A pinned before the
run). Card `CARD-p1-r2-a`: 18 arm-F jobs and 8 T jobs at 6 GiB / 3,600 s; 24 completed, the
two T jobs at m5 t3 capped again. Analysis: `analysis/p1r2analyze` (Rust; merges round 1 and
round 2, applies the bound reading frozen in PROTOCOL-R2) →
`analysis/derived_r2/{p1_r2_cells.tsv, decision_r2.json}`. [pcc:unverified]

| cell (all three primes alike) | RR | T | F | RR vs T | RR vs F |
|---|---|---|---|---|---|
| m3 t3 | D 4, widest 632 | D 5, 883 | D 5, 231 | RR dominates | trade |
| m3 t4 | D 4, 2,801 | D 6, 7,129 (Kummer 7,077) | D 5, 1,224 | RR dominates | trade |
| m3 t5 | D 5, 31,911 (Kummer 31,870) | D 6, 23,978 (Kummer 23,720) | D 6, 10,493 | **trade** | trade |
| m5 t3 (T24, no tower) | D 5, 2,878 | D ≥ 8, widest ≥ 82,927 (capped at 6 GiB) | n/a | RR below on both (bound reading) | n/a |

- **Rule 1, control: met exactly.** T at m3 t5 on the prime-order Kummer curve gives D 6,
  widest 23,720, round 7's values on a different curve; every completed job finds the planted
  relation.
- **Rule 4, RR vs arm F: trade in all 9 cells.** RR reaches one degree lower, but F's widest
  matrix is 2.3 to 3.0 times narrower (231 vs 632, 1,224 vs 2,801, 10,493 vs 31,911). Against
  the lab's strongest m = 3 system, RR is not a gain on these measures.
- **Rule 5, RR vs T: RR dominates at t3 and t4, trade at t5** (RR wider, 31,911 vs 23,978), as
  the red team predicted from round 7.
- **Rule 2, m5 t3:** RR below T on both metrics even against T's lower bounds; this is the
  tower-free cell (`../red-team-p1-p2.md`), so it says little about towers.
- F's values are identical on the Dickson and Kummer primes and equal round 7's, as T's and RR's
  are: at these sizes the F4 trace depends on the system's shape, not the prime or the tower.

**Reading.** Round 1's positive result (RR below the cherry tree T at m = 3) holds at t3 and t4
and turns into a trade at t5. Against arm F it is a trade everywhere. So the Riemann–Roch
system transfers to the NIST field shape but is not the lab's best m = 3 system on matrix
width. The m = 5 cell with a tower (card B, T24 m5 t4, 13 GiB) decides rule 3.

## Round 2, card B: the first m = 5 cell with a tower (run-r2b-20261108T053603Z)

Card `CARD-p1-r2-b`: T24 m5 t4, seed 2 (seed 1 skipped: base too small for m = 5), RR then T,
13 GiB and 10,800 s, alone on the host (`provenance.txt` records the idle start). Analysis:
`analysis/p1r2analyze` over rounds 1, 2a and 2b → `analysis/derived_r2b/`. [pcc:unverified]

| system | status | D reached | widest matrix reached |
|---|---|---|---|
| RR | timeout at 10,800 s (exit 124) | ≥ 6 (working in degree 7 when stopped) | ≥ 146,277 |
| T | memory cap at 13 GiB after 2,619 s (exit 139) | ≥ 9 | ≥ 662,876 |

**Rule 3: undecided under the frozen text**, because RR did not complete (the frozen bound reading
decides "RR below" only for a completed RR). Descriptively, at the point each job was stopped, RR
was working at a lower degree and with matrices 4.5 times narrower than T, and it was limited by
time rather than memory; neither number is an outcome. Round 7 found the same cell beyond a
15 GB host for both systems.

**What would decide it:** RR at T24 m5 t4 run to completion, on a host with more memory and a
time limit of about a day, against T on the same host. That is a resource limit, not a
mathematical obstruction; it is recorded in the follow-up index.

## Round 3: T24 m5 t4 on Iso Lab EKS, 56–60 GiB and 24 h (PROTOCOL-R3.md)

Same instrument, msolve, instance (seed 2; each pod re-emitted the system and its sha256 matched
R2b's) and frozen rule 3; only the limits and the host changed: RR at `ulimit -v` 60 GiB in a
64Gi pod, T at 56 GiB in a 60Gi pod, both `timeout` 86,400 s, one msolve process per pod on the
lab's `cpu-spot` pool. Launch files, image receipt and the attempt log: `launch/`. Analysis:
`analysis/p1r3analyze` over R2b's `parsed.jsonl` and round 3's parse records →
`analysis/derived_r3/{decision_r3.json, p1_r3_bounds.tsv}`. Director assessment:
`cards/p1-run-r3.assessment.md`. [pcc:unverified]

| attempt | host | outcome | D | widest |
|---|---|---|---|---|
| T a1 | m7i.4xlarge Spot | lost: Spot interruption after 1 h 30 min, in the degree-9 step | — | — |
| **T a2** | m7i.4xlarge Spot | **memory cap at 56 GiB after 21,267 s** (exit 139), in the degree-9 step | **≥ 9** | **≥ 1,305,345** (847,295 × 1,305,345) |
| RR a1 | r7a.4xlarge Spot | lost: Spot interruption after 15 min, in the degree-7 step | — | — |
| RR a2 | r7a.4xlarge Spot | lost: Spot interruption after 2 h 33 min, in the degree-7 step (preceded by 10 h Pending: pool at its node limit) | — | — |
| RR a3 | r6a.4xlarge Spot (pinned; Spot Advisor < 5 % band) | lost: Spot interruption after 52 min, in the degree-7 step | — | — |

Deviations noted by the director (`cards/p1-run-r3.assessment.md`): the two systems did not run at
the same time as the protocol says (RR a1 and T a1 overlapped about one minute; T a2 ended before
RR a2 started; RR a3 ran alone), RR a3's request pins an instance type the protocol does not name,
and the owner's lifting of the USD 60 target is recorded in `launch/README.md`, not in the protocol.
None of these touches a frozen value of the instrument or the reading.

Every loss is a CloudTrail `BidEvictedEvent` (`launch/receipts/*-spot-eviction.cloudtrail.json`);
the partial records are kept under `launch/receipts/*-results-partial/` and, per PROTOCOL-R3,
supply no bound. The protocol allowed two resubmissions per system; RR used both.

**Bounds after rounds 2 and 3** (max over attempts; nothing completed):

| system | D | widest | from |
|---|---|---|---|
| RR | ≥ 6 | ≥ 146,277 | R2b (timeout at 10,800 s) |
| T | ≥ 9 | ≥ 1,305,345 | R3 T a2 (cap at 56 GiB); R2b gave ≥ 662,876 at 13 GiB |

**Rule 3: still undecided.** The frozen reading decides "RR below T" only for a completed RR, and
no RR attempt ran longer than 2 h 33 min. T's widest bound doubled, so what RR must do to score
"holds" moved up: complete with D ≤ 8 and widest < 1,305,345; "trade" is D ≤ 8 with widest at or
above that; "fails" is D ≥ 9.

Descriptive, not an outcome: in all three RR attempts and in R2b, msolve was building the same
degree-7 matrix, 464,562 rows × 390,983 columns, when stopped; that step never finished in any
run (the longest, RR a2, gave it about 2 h 20 min). T's degree-8 step completed at 262,201 columns
and its degree-9 matrix reached 1,305,345 columns before the cap. If the degree-7 step completes,
RR's widest bound becomes at least 390,983, still below T's.

**What happened operationally:** five pods, four Spot interruptions within 15 min to 2 h 33 min,
and one 10 h capacity wait. With no On-Demand pool and no checkpointing in msolve, a 24 h
single-process run on this pool is unlikely to survive; the partial records show the computation
is not the obstacle. The cost of all attempts was small (Spot nodes for about 12 node-hours in
total; the owner lifted the USD 60 target, `launch/README.md`).

**Cheapest next steps, if pursued (not preregistered):** (a) RR alone on an On-Demand or
capacity-reserved 128 GiB host (outside the lab's Spot-only pool), one 24 h job; (b) the lab's
CRIU/EBS resumable command mode, once it is qualified for memory-family instances (today it is
qualified on c8a only, 16 GiB, too small); (c) msolve with `-t 8` on the same host, which changes
the instrument (the frozen protocol pins `-t 1`) and would need a new preregistration with its own
calibration.
