# PROTOCOL (frozen before any panel data): elliptic-code list decoding as a relation finder

Status: FROZEN 2026-11-07, before any `--tier full` run. No panel data exists. Only development checks (`results/dev_*`, tiny p, explicitly not evidence) were run. Source of the instrument: `ecld_toy.rs`, sha256 `de901e230d2558cb488dee653e8bdfe44ff44b928a611322a014670c5fa8769d` at freeze (the executor must compare the hash of the file it builds; any difference voids the freeze). Every number below is `[pcc:unverified]` (derived or planned, not yet measured).

Lead: `LEAD-ICP-elliptic-code-list-decoding` (twin `LEAD-EXO-elliptic-code-list-decoding`). This protocol is a prospective `experiment.protocol/v1` design written in prose; the machine-readable record and the `experiment.export/v1` binding are for the experiment director to produce from it (not done here).

## 1. Claims being tested (classified)

C1 (exact, theorem-class, tested for correctness only). For the code C = ev_D(L((k-1)O)) and received word R = ev_D(f'), f' = (y + y_Q)/(x - x_Q), the codewords with agreement exactly k are in bijection with the k-subsets of D that sum to Q (Cheng, arXiv cs/0507026, Lemma 3), and no codeword has agreement > k.
C2 (comparative, measured). An exact list decoder at agreement k (arm D1) finds the same relations as plain enumeration, and costs more than enumeration in counted work.
C3 (scaling, measured). The counted work of D1 grows as B^(k-1) in the factor-base size B at fixed k, like enumeration with a one-lookup last step.
C4 (derived, no experiment). The Guruswami-Sudan guarantee radius t_GS = floor(sqrt(B (k-1))) + 1 exceeds k, so a GS list on this received word is empty. Printed by `ecld_toy bound`; the toy only checks, by brute force at tiny size (`check`), that no codeword exceeds agreement k.

Not tested and not claimed: anything about the Zhang-Liu paper's own procedure (unread). The toy tests the mechanism class "exact or GS-style list decoding of a received word built from the target".

## 2. Instrument

Binary `ecld_toy` (std-only Rust, `rustc -O`, single thread). Field F_p with p prime drawn deterministically from [61000, 65521) (so p < 2^16, about 2^16). Curve y^2 = x^3 + ax + b with prime group order N (checked by point counting, N != p, N >= 50, G*N = O). Factor base D: B distinct points e_i G with e_i uniform in [1, N-1] (a random subgroup sample; the toy does not use factor-base structure). Target Q: either a uniform multiple of G ("random target") or the sum of a random k-subset of D ("planted", positive control); x_Q must not occur among x(D) and y_Q != 0 (redrawn otherwise).

Arms. All three are exact and must return the same set of k-subsets of D (as sets of indices) summing to Q; any difference voids the instance.

- E1 (enumeration with one lookup): all (k-1)-subsets i_1<...<i_{k-1}, incremental partial sums (1 group addition per node at depth >= 2), final T = Q - S (1 group addition) looked up in a hash of D, accepted if the index exceeds i_{k-1}. This is the lead's "plain enumeration of k-subsets" with the last element found by lookup.
- E2 (meet in the middle): table of all floor(k/2)-subset sums, enumerate all ceil(k/2)-subsets, look up Q - S, merge disjoint pairs, dedupe as sets. This is the strongest simple enumeration and is the reference the decoder must beat.
- D1 (exact list decoding at agreement k, information-set form): basis of L((k-1)O) is 1, x, y, x^2, xy, ... (pole orders 0,2,3,4,5,...). For each (k-1)-subset S of positions, solve the (k-1)x(k-1) system for the unique codeword agreeing with R on S (Gauss-Jordan mod p), then compare it with R at every later position j > max S; each match gives an agreement set of size k. Subsets with a singular system (points of S sum to O) cannot lie in a relation unless some point equals Q, which the construction excludes, so skipping them is exact; their number is logged (`singular_S`).
- D0 (check mode only): brute force over all p^(k-1) codewords. Used only at p < 1100 to confirm the bijection of C1 and that no agreement exceeds k.
- Guruswami-Sudan itself is NOT implemented. Reason: on this received word the guarantee radius exceeds the maximum possible agreement (C4), so a correct GS decoder returns the empty list by the theorem and an implementation could add nothing. The exact list decoder D1 stands for "any decoder that outputs all agreement-k codewords", which is the strongest thing a list decoder could do on this word.

## 3. Counted unit and pricing (AGENTS.md section 5a)

Priced unit: one affine group addition (GAE) on the toy curve (E1, E2: every addition including doublings and subtractions; D1: zero). Unpriced native counters, listed with their counts for every arm: `fmul` (field multiplications, decoder side), `finv` (field inversions), `fadd` (field additions in the decoder), `tins` (hash inserts), `tlook` (hash lookups), `cmp` (comparisons). The setup of D1 (basis table at all B positions, B inversions for R) is charged to D1. Setup of D (scalar multiplications) and verification are common to all arms and not charged. Each reported cost is gae + w * (fmul+finv+fadd+tins+tlook+cmp) at w = 0, 0.1, 1. Arms are ranked only where the order holds at every w in {0.1, 1}; w = 0 prices D1 at exactly zero by construction (it performs no group addition), so it is reported but a "win" at w = 0 alone is a pricing artifact and is declared unresolved, not a win. Crossing weight w* = E_gae / D1_native is reported per cell. Memory (hash tables) is listed in bytes by the runner's notes and not priced (owner rule 2026-11-01). Wall time is a practicality note only.

Verification. Every reported relation is re-summed with a separate, uncounted group-law call and must equal Q; "verified relations" counts only those. Logs of the secrets e_i are never given to an arm.

Relation-phase distance to rho (reported as distance remaining, not pass/fail): relation-phase cost per verified relation, and cost to collect B+1 relations (extrapolated, labelled), against the nominal matched rho figure sqrt(pi N / 4) group additions. This is a relation-phase-only quantity on a toy, with known logs of the factor base; it is not a leaderboard row (no replay certificate, validator report or measured rho) and must not be presented as one.

## 4. Panel (frozen)

Master seed 20261107; all curve, factor-base and target seeds are `derive(master, tags)` as in the source. Cells (k, B, curves, targets per curve; the last target of each curve is the planted control):

- k = 2, 3, 4: B in {64, 128, 256}, 4 curves, 4 targets (3 random + 1 planted).
- k = 5: B in {64, 128} with 4 curves, B = 256 with 2 curves; 4 targets each.
- k = 6: B in {32, 48, 64}, 4 curves, 4 targets. (k = 6 at B = 256 is out of reach for D1: about 2e12 native operations per instance; the lead's B ~ 2^8 is therefore covered for k = 2..5 only, and k = 6 enters as a scaling cell at smaller B, with the B = 256 value an extrapolation using the B^(k-1) exponent fitted in section 6.)
- No k = 6, B = 256 cell: its enumeration side (E2, about 8.3e6 GAE per instance) is cheap but D1 is not run there, so it is a listed gap, not a cell.

Run order of arms alternates per (curve+target) parity: (E1,E2,D1) or (D1,E2,E1). Arm labels cannot be masked (the counters identify the arm); because every count is deterministic given the seeds, order and labelling cannot move the result, and the interleaving is recorded for completeness.

Sample size rationale. Counted cost per instance is deterministic, so there is no sampling noise in the cost comparison; the instance count controls only the relation-count statistics (which vary by Poisson-like noise around C(B,k)/N). Relation yields at k = 2, B <= 128 are below 1 per target by the model, so those cells are dominated by the planted control and report cost per relation with a wide interval; this is stated, not hidden.

## 5. Controls

- Positive control: every curve has a planted target; each arm must recover the planted k-set. Failure of any arm on any planted instance voids that arm's panel.
- Negative control: cells with expected relations < 1 per random target (k = 2, B = 64..256; k = 3, B = 64) must show arms agreeing on the empty set where the exact enumeration finds none.
- A/A: for k <= 4, E1 is run twice on every instance; sets and every counter must be identical (`aa_e1_identical`). Determinism check of the instrument.
- Cross-arm exactness: `sets_consistent_across_arms` must be true on every instance.
- Bijection control (development check, done): `check` compares D0 (all p^(k-1) codewords) against E1, E2 and D1 at p in [200, 1100) for k = 2, 3, 4.
- Random-word control (deferred, not implemented): a decoder run on a random received word would show the same list-size model C(B,k)/p; it is not run in this version. This is a stated gap, not a result.

## 6. Predictions committed before data

P1. All instances: E1 = E2 = D1 as sets, all relations verify, all planted sets recovered, A/A identical. A failure is an instrument bug, not a finding.
P2. D1 `gae` = 0 and its native count within 25% of the plan in `results/panel_cost_plan.txt` (formula: C(B,k-1) x [(B-k+1)(2k-1)/k + (k-1)^2 k + 3(k-1)]).
P3. At w = 0.1 and w = 1, D1's priced cost exceeds both E1's and E2's in every cell with k >= 3, by a factor of at least 10 at k >= 4. Crossing weight w* < 0.1 in every such cell (plan values: 4.5e-3 at k=3, B=256).
P4. log-log slope of D1 native count against B at fixed k within 0.25 of k-1 (between B = 64 and 256 for k = 3, 4, 5; between 32 and 64 for k = 6, allowing 0.4 because the range is narrow).
P5. Relations per random target within a factor 2 of C(B,k)/N (pooled over a cell's instances once the cell expects >= 10 relations in total); below that, only the direction is checked.

## 7. Decision rule (the lead's discriminator, made operational)

- ENUMERATION IN DISGUISE for the mechanism class tested (exact or GS-style list decoding of the target-built received word): P1 holds, and in every cell with k >= 3 and expected relations >= 1 per target, D1's cost per verified relation is higher than E2's at both w = 0.1 and w = 1, and the slope of P4 holds. Consequence: the lead goes to a cost-model closure with the scope of LEAD-ICP-generic-table-decomposition, for this mechanism class only.
- BELOW ENUMERATION: in any cell with k >= 3, D1's cost per verified relation is lower than that of both E1 and E2 at both w = 0.1 and w = 1, with P1 holding. Consequence: the next step is a charged comparison on a size ladder against matched rho; the finding would be reported with the instance list.
- UNRESOLVED: any other pattern, including order reversals inside the w in {0.1, 1} band, a violated P1, or a failed control. Report as unresolved at the boundary; no win for either side.
- Neither outcome speaks to the Zhang-Liu procedure beyond "its received word, if of the target-built kind, lies in this class". If its text shows a different mechanism (for example a heuristic decoder not tied to the GS radius, or a different code), the lead stays open at the reading step.

## 8. Variable ledger

Instrument: one binary, hash recorded; one rustc and optimization level (`-O`). Host: Iso Lab EKS job, one thread, no co-running panel jobs on the same pod. Seeds: derived, listed above. Layout: single process per (cell, curve, target) filter; no shared state between jobs. Order: interleaved by parity. Spot preemption: counts are deterministic, so a preempted job is re-run from scratch with the same filter and its stale output discarded; wall times of re-run jobs are practicality notes only. Analyst: the director reads `results/` rows; arm labels are not maskable (above).

## 9. Outputs and stop condition

One JSON line per (instance, arm) as in `results/dev_smoke_panel.jsonl` (fields: k, B, p, N, curve, target, planted, arm, relations, verified, planted_recovered, sets_consistent_across_arms, aa_e1_identical, set_hash, singular_S, t_gs, counters{gae,fmul,finv,fadd,tins,tlook,cmp,native}, priced_w0, priced_w0.1, priced_w1, wall_s). Rows from preempted or failed jobs are kept in a separate file. Stop when all frozen cells are complete or a control fails. No cell is added, dropped or re-parameterised after seeing data.
