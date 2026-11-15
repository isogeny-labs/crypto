# Wave-6 leads 1 to 4, run in parallel: flat beats chain by 10 to 360x, cost depends on (m, D) only, a measured 3.3 % cut on the C = 2 arms, and the solving degree is bookkeeping (2026-11-15)

**Class.** Four frozen measurements on the Curve25519-family miniatures, one per lead of
[`../curve25519_leads_wave6_20261114/`](../curve25519_leads_wave6_20261114/README.md): three engine measurements in
counted field operations (L1, L3, L4, `curve25519_family f4sub`) and one paired ecbench session set in counted GAE
(L2). L2 is a measured improvement of the lab's C = 2 index-calculus arms under the owner's 2026-11-04 ruling; L3 is a
measured component improvement of the decomposition oracle for subgroup bases; L1 and L4 are rigor results. No
leaderboard, `benchmarks/` or atlas row changes here (the validation gate's VALIDATION.json, benchrec record and pcc
claim are not yet written; see §7). Every number is an unverified assertion [pcc:unverified] except where the
validator reports named in §6 say otherwise.

**Owner's request (2026-11-15).** "1-4 all parallel."

**How.** Protocols frozen in `c66e2b3f4f` before any data (`L*/PROTOCOL.md`); one pre-data amendment to L1
(§5). One release build of the branch's `curve25519_family` and `ecbench` (hashes in `README_PLAN.md`) served all
four; the frozen exponent-study binary dc2c30a4 remains the reference for the recorded chain cells, and the new
binary reproduces it exactly on two control cells (`smoke_equivalence.sh`) and on the three chain controls of L3.
Four cores, one lead each; the F4 leads under `ulimit -v 6000000`.

## 1. Answers

| lead | question | answer | predictions |
|:--|:--|:--|:--|
| L3 flat vs chain | Is one variable y^D = 1 per summand cheaper than the chain of power maps on the same base? | **Yes, by 10 to 363x at m = 3 and 2.7 to 17x at m = 2** at every D measured; flat D = 36 completes in 202 s under 6 GB where the chain ran out of 12 GB; flat SD = D + 1 everywhere | P1, P3, P4, P5 hold; P2 holds except that D = 27 (363x) lies above the predicted band [70, 260] |
| L1 faithful family | Do the exponent studies' costs depend on the family rule? | **No: T is identical to four digits at every shared (m, D)** between the faithful member (p = 2^24 − 75, A = 7842) and the k = 20, 22, 24 smooth-family records; the m = 3 estimator stays inapplicable on the faithful member because its divisor set is sparse | P1, P2, P4 hold; P3: pending the last m = 3 cells |
| L2 ladder checks | What does routing both [d]G = Q checks through the stored [2^i]G ladder save? | **3.3 % at k = 20 and 1.4 % at k = 24** of the tw2cj-lazy arm's counted cost at w = 0, the order holding at w = 0.1 and 1; walks identical, only check doublings change; distance to rho-neg 1.56 (k = 20) and 1.61 (k = 24) against 1.61 and 1.63 before | P1, P2, P3 hold; the prediction's basis was wrong in detail (validator E1) |
| L4 F4 step trace | Is the recorded solving degree bookkeeping of the generator degree? | **The top degree is reached at steps 3 to 4 of 22**, from pairs of the binomial, and 78 % of the field operations are spent afterwards on the way down; changing the variable order leaves SD unchanged and moves T by 1.24x and 0.91x | P1 fails as written (the last productive step is at degree 3), P2 vacuous, P3 holds |

**What changes for the programme [pcc:unverified].**
- The exponent studies' chain presentation was the wrong instrument: the flat presentation makes the same
  measurement 10 to 360x cheaper at m = 3 and reaches the cells the chain could not. The parent's section-5
  estimator, applied to the flat m = 3 cells D = 9 to 36 (8 cells, span 4.0x), gives s = 4.40 [4.10, 4.71]; at
  m = 2, s = 2.26 [2.21, 2.31] over D = 9 to 219 (9 cells, span 24x). These fits were not preregistered in L3 (reported as readings);
  under the parent's lever rule they would read "no lever at m = 3" instead of "undetermined".
- Cost is a function of (m, D) alone: the same chain cells on three different primes and two family rules give the
  same field_ops to four digits. The family-rule confounder the wave-6 README feared does not exist for this
  instrument. What the faithful family changes is which D exist.
- L2 is the first measured improvement over the C = 2 arms since the snc-panel: small, constant-factor,
  shrinking with k, and not an accounting change (fewer doublings are performed; nothing moved to an unpriced
  counter).
- The solving-degree column of the exponent studies measures where F4's degree climbs first, not where the work is;
  the corpus record for HEUR-FIRST-FALL-DEGREE should say so.

## 2. L3: flat against chain (`L3_flat/`)

`f4sub 20 subflat D 0 mont M sym 6 SEED 1800` on the k = 20 member, seeds equal to the recorded chain cells' so the
six targets agree. Binary control: the chain cells m = 3 D = 9, 21 and m = 2 D = 57 rerun with the new binary
reproduce the recorded per-target field_ops exactly (P5).

| m | D | flat T | flat SD | wall | chain T (recorded) | chain SD | chain / flat |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 3 | 9 | 7.59e6 | 10 | 0 s | 1.48e8 | 7 | 19.5 |
| 3 | 12 | 2.73e7 | 13 | 1 s | 7.75e8 | 7 | 28.4 |
| 3 | 14 | 4.69e7 | 15 | 2 s | 7.77e8 | 10 | 16.6 |
| 3 | 18 | 1.42e8 | 19 | 6 s | 2.43e10 | 7 | 170.5 |
| 3 | 21 | 2.65e8 | 22 | 12 s | 2.79e9 | 11 | 10.5 |
| 3 | 27 | 7.27e8 | 28 | 38 s | 2.63e11 | 8 | 362.6 |
| 3 | 28 | 1.00e9 | 29 | 47 s | 2.02e11 | 11 | 201.0 |
| 3 | 36 | 4.13e9 | 37 | 202 s | censored at 12 GB | | |
| 2 | 9 | 1.94e4 | 10 | 0 s | 5.22e4 | 5 | 2.7 |
| 2 | 21 | 1.20e5 | 22 | 0 s | 4.35e5 | 9 | 3.6 |
| 2 | 27 | 2.06e5 | 28 | 0 s | 1.79e6 | 5 | 8.7 |
| 2 | 57 | 1.07e6 | 58 | 2 s | 3.88e6 | 21 | 3.6 |
| 2 | 63 | 1.34e6 | 64 | 3 s | 2.23e7 | 9 | 16.7 |
| 2 | 133 | 7.65e6 | 134 | 143 s | 3.51e7 | 22 | 4.6 |
| 2 | 171 | 1.42e7 | 172 | 618 s | 3.83e8 | 21 | 27.1 |
| 2 | 189 | 1.81e7 | 190 | 973 s | 3.94e8 | 10 | 21.7 |
| 2 | 219 | 2.63e7 | 220 | 2,232 s | 6.18e7 | 75 | 2.4 |
| 2 | 399 | censored: memory, 6 GB, exit −6 after 3,851 s (no target finished) | | | | | |

Predictions: P1 (flat SD = D + 1) holds in every cell. P2: chain/flat in [70, 260] at D = 27 and 28 holds at
D = 28 (201) and fails upward at D = 27 (363); above 10 at D = 9 to 21 holds (the smallest is 10.5 at D = 21).
P3 (flat D = 36 completes under 6 GB and 1,800 s) holds. P4 (m = 2 ratio above 1 everywhere; flat on the prime-D
curve within 3x) holds: 9.7e4·(D/19)^2.2 predicts 1.1e6 at D = 57 and 7.0e6 at D = 133 against 1.07e6 and
7.65e6. P5 holds. Wall time per field operation grows with D at m = 2 (143 s for 7.7e6 field operations at D = 133,
2,232 s for 2.6e7 at D = 219), and the flat D = 399 cell exhausted 6 GB before its first target finished: the counted unit leaves out the symbolic work of the degree-D steps, as the theory lane
warned; wall time is recorded, not claimed. Full table: `L3_flat/analysis.json` (`analyse_l3.py`).

## 3. L1: the faithful member (`L1_faithful/`)

`f4sub p=16777141,A=7842 sub D 0 mont M sym 6 SEED 1800`, D over the divisors of (p − 1)/4 = 3·5·279619 up to 60.
At every (m, D) that also exists in the smooth-family records the mean field_ops agree to four significant digits
(ratio 1.000): m = 2 at D = 3 (three records, k = 20, 22, 24), 5, 15; m = 3 at D = 3 (two records), 5, 12, 15. The
solving degrees agree too. P1, P2 and P4 hold. P3 (the m = 3 chain censors at or before D = 60, so the estimator
stays inapplicable): the m = 3 cells D = 20, 30, 60 were still running at the time of writing; with D ≤ 15 complete
the span is 1.5x. Table: `L1_faithful/analysis.json` (`analyse_l1.py`).

The amendment (§5) is itself a result: under the full family rule (A ≡ 2 mod 4, cofactor 8 with prime r > 2^(k−3),
prime twist cofactor 4) no prime of Curve25519's shape below 2^20 among the seven searched admits a curve with
A < 40,000, while the k = 24 prime does at A = 7,842. The shape is restrictive at small k [pcc:unverified].

## 4. L2: both checks from the ladder (`L2_ladder_check/`)

ecbench sessions k = 20 and k = 24 (the snc-panel's five curves, 8 rounds, alternate order, seeds 20261130 and
20261134; 1,620 records each, all passing the audit, 8 replays identical per session, `verify-k*.json`). Arms: `rho-neg`
(reference), `tw2cj-lazy` (baseline, `wwalk:x=2.0,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`), `tw2cj-lazy-lcheck`
(the same with `lcheck=1`).

| k | ratio candidate / baseline, w = 0 | w = 0.1 | w = 1 | predicted (w = 0) | candidate / rho-neg | baseline / rho-neg |
|--:|--:|--:|--:|--:|--:|--:|
| 20 | 0.9669 [0.966, 0.968] | 0.9692 | 0.9809 | 0.959 [0.950, 0.970] | 1.557 [1.499, 1.618] | 1.611 [1.551, 1.673] |
| 24 | 0.9862 [0.986, 0.987] | 0.9873 | 0.9927 | 0.983 [0.975, 0.990] | 1.606 [1.523, 1.695] | 1.629 [1.544, 1.718] |

Paired ratios from `ecbench compare` (480 pairs, 60 workloads, 5 curves, linearised t, df 59; comparison files under
`sessions/k*/comparisons/`). P1 holds at both k (inside the frozen intervals, order holding across the band). P2
holds: in all 540 pairs per k the only differing fields are the check doublings (`tw_lazy_check_doubles`, the
set-up and verification doublings) and totals derived from them (validator). P3 holds. The saving per run is
exactly 2·(bitlen(d) − 1) doublings: 30.3 GAE (3.30 %) at k = 20 and 38.3 GAE (1.37 %) at k = 24. Wall time is
unresolved (1.009 [0.995, 1.023] at k = 20, loaded host, no A/A arm).

Classification: the 2026-11-04 ruling covers wwalk arms with g = 1, grow = 2, of which `tw2cj-lazy` is one; whether
`tw=2` arms count as index calculus remains the owner's call (wave-6 red team R8). Under either reading the
candidate performs fewer group doublings for the same walk, so this is not relabelling or accounting.

## 5. L4: the F4 step trace (`L4_trace/`)

`F4SUB_STEP_LOG=1 f4sub 20 sub 19 0 mont 2 sym 6 20263127 1800` (the recorded cell's seed; see E4) and the
`sym-efirst` form at D = 19 and 57. All six targets have the same log: degrees 2, 19, 20, 20, 19, 18, …, 3 over 21
logged steps (the 22nd, where 1 appears at degree 2, is not logged); pairs 1, 2, 4, 6 then 12; new polynomials 1,
2, 4, 4, …, 3. The top degree 20 is reached at steps 3 and 4; the descending steps carry 77.9 % of the logged field
operations. P1 as written fails on both clauses (the last productive step is at degree 3, and the degree-20 steps
have 4 and 6 pairs); P2 is vacuous; P3 holds (SD 20 and 21 unchanged under `sym-efirst`; T 1.203e5 = 1.24x and
3.534e6 = 0.91x of the recorded). The reading "SD is where the degree climbs, not where the work is" is what the
log shows; the frozen rule tied the bookkeeping verdict to P1 and P2, so it is a reading, not a verdict (E3).

## 6. Validation and errata

`validator-report-L2-L4.json` (independent code under `validation/`): status `pass_with_errata`, disposition
`validated`. It recomputed every L2 ratio from the records, re-checked all 3,240 certificates with its own curve
code, replayed every measured run (1,440 of 1,440 reproduced per session), confirmed that the new binary leaves the
baseline and reference arms' counted cost unchanged against the snc-panel (difference 0.0 on all 60 workloads at
both k), and replayed the L4 cells with both binaries. Errata, applied above:

- E1: the L2 protocol's basis ("about 4.7 additions each over the ladder") was wrong; the ladder does the same
  popcount(d) − 1 additions as double-and-add, and the whole saving is the doublings. The frozen interval still held.
- E2: the degree-20 steps have 4 and 6 pairs, not 4 each. E3: the bookkeeping reading is post hoc under the frozen
  rule. E4: the L4 protocol's literal seeds (20263019, 20263057) were miscomputed; the runs used the recorded cells'
  seeds 20263127 and 20263165, which is what the protocol says in words ("same seed as the record"). E5: the step
  log omits the final step. E6: `records.jsonl` (14 MB each) and the L2 run log exceed the 1 MB artifact policy
  and go to the artifact store on the PR. E7: `session.json` has no git commit; the binary-to-source link rests
  on `README_PLAN.md`. E8: the engine writes the old study label into the cells; `ext.study` is right. E9: "falls by
  the same factor" is true by construction.

L1 and L3 validation: pending (their last cells were running at the time of writing); the report will be
`validator-report-L1-L3.json`.

## 7. Not done and disclosures

- No Conductor task, EKS run, S3 checkpoint, benchrec record, VALIDATION.json or pcc claim: none of the services is
  reachable from this runtime, as in every study of this series. The L2 result therefore cannot enter the
  leaderboard or `benchmarks/` yet; the sessions, comparisons and verify reports are the evidence a gate entry
  would cite.
- Host: shared 4-core 15 GB container, the four leads running at once; wall times are recorded, not claimed.
- Lanes and validators ran on a different model tier from the agents' defaults; no model identifiers in files.
- The L2 arms are single-target runs (one DLP per run), as the owner requires.

## Files

| path | content |
|:--|:--|
| `README_PLAN.md` | plan, binary hashes, toolchain, host |
| `L1_faithful/`, `L3_flat/`, `L4_trace/` | PROTOCOL.md, cells/, run.log, analysis scripts and outputs |
| `L2_ladder_check/` | PROTOCOL.md, spec-k20/24.json, sessions/k20, k24 (records, comparisons), verify-k*.json, band-k*.json/txt |
| `run_cells.py`, `launch_all.sh`, `smoke_equivalence.sh` | runner, launch record, binary-equivalence smoke |
| `validator-report-L2-L4.json`, `validation/` | independent validation of L2 and L4 |
| `figure.py`, `wave6_runs.svg/.png` | figure: L3 flat vs chain, L2 ratios against the frozen bands, L1 equality |
| `REPORT.typ`, `REPORT.pdf` | this README as a report |
