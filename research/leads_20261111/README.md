# New ECDLP index-calculus leads, 2026-11-11

Owner's request: "go and find more leads". Three idea-generator agents each took one
lane and wrote the files below. I read every file, re-checked the claims each top
lead depends on against the code and data on disk, and ranked the leads. All
statements are `[pcc:unverified]`. Nothing in this directory has been run, apart
from the re-checks listed in "Checks made for this report". Nothing concerns a
deployed curve or key recovery.

## Status update, 2026-11-12: leads 1 and 2 tested at the owner's request

All results below are `[pcc:unverified]`.

- **Lead 1 (`LEAD-C-jv-nagao-hilbert-at-scale`): prediction held.**
  - Setup: 9 instances at p = 1301–1601, plus a drift control that reproduced the
    frozen record exactly.
  - Counted cost: S_B/S_A = 0.5276–0.5280 at equal relations and equal digest. All
    four preregistered falsifiers passed.
  - Distance to matched negation rho, U2 at w = 0: from 1.900 / 1.585 / 1.318
    (above) to **1.003 (not resolved) / 0.837 (below) / 0.695 (below)**.
  - The order holds across the §5a band at p = 1451 and 1601.
  - Local and unvalidated: no leaderboard row.
  - Write-up: `research/jv_nagao_hilbert_20261112/README.md`.
- **Lead 2 (`LEAD-A-selector-eta-kernel`): conjecture C1 falsified as stated.**
  - The nullity equals the trivial kernel M(k−1, e−1) in every infeasible cell
    tested: n = 23–27 at e = 3, n = 27 at e = 4, and the 9 certificates at
    n = 33–41.
  - It exceeds the kernel at the feasible cells, inside the lead's claimed region:
    - n = 23, e = 4: 1143–1145 against 232;
    - n = 25, e = 4: 740–741 against 299.
  - So non-trivial c-only syzygies appear exactly at `e_min`.
  - Write-up: `research/lfd_lower_bound_20261111/out/lead_A1/RESULT.md`.

- **Lead 4 (`LEAD-C-jv-sieve-hilbert-descent`): killed by its own falsifier.**
  - Setup: opt-in `JV_SIEVE_HILBERT=16`; 16 paired seeds at p = 1511. The plain
    arm reproduces the committed records exactly.
  - Gains: descent F4 multiplications 0.407×, counted total 0.856×, digests
    identical.
  - The kill: descent native F4 operations rose 1.139×, against a preregistered
    limit of 1.10×.
  - §5a band: cheaper at w = 0 and 0.1, dearer at w = 1, so unresolved.
  - Write-up: `research/jv_sieve_hilbert_descent_20261112/README.md`.

- **Lead 5 (`LEAD-ICB-d4bin-chain-variable-sweep`): closed.**
  - The current chain unknown already gives k = 12 chains, not the 16 the lead
    assumed.
  - The only alternative (s4) gives 28 chains and 2.34× the cost per trial.
  - Write-up: `probes/CHAIN_VARIABLE.md`.

- **Lead 6 (`LEAD-A-selector-counting-law`): Model A wins the preregistered grid.**
  - Score: A 4 of 5, B 2 of 8; all controls as expected.
  - The plain ratio `(k+1)M(k,e+1)/(n M(k,e))` predicts the random-`V` cells.
  - `V` inside `ker Tr` is harder: (13, 25, 4) is infeasible there.
  - Write-up: `research/lfd_lower_bound_20261111/out/lead_A2/RESULT.md`.

## Status update, 2026-11-12 (evening): follow-ups on RunPod, with independent checks

A second workflow ran six follow-ups, using RunPod CPU pods through
`research/runpod_kit_20261112/`. Each result had an independent check: the
`validator` agent for the Nagao result, a skeptic for the others. All results are
`[pcc:unverified]`.

| follow-up | outcome | check |
|:--|:--|:--|
| **Nagao Hilbert stop, validation** (`research/jv_nagao_hilbert_20261112/validation/`) | See below. | validator: **insufficient_evidence** |
| **Exact e_min above n = 29** (`research/lfd_lower_bound_20261111/out/c_runpod/`) | **e_min = 5 at n = 31** (4 of 4 instances) and **e_min = 5 at n = 33** (3 of 3). Model A's n = 31 prediction holds; n = 33 narrows θ_A to (1.2634, 1.2698). n = 35 is undecided: a 32-bit index overflow in M4RI 20200125 at about 442k columns. | skeptic agrees |
| **Quartic up to k = 26** (`runs/C_quartic_k26/runpod/`) | **Not killed** on any frozen falsifier, but both numeric predictions are missed. DL exponent k = 22..26: 1.068, against a predicted 1.00 [0.95, 1.06]. DL/rho-neg model at k = 26: 0.0016 (w = 0) / 0.0009 (w = 1). | skeptic agrees. k = 22..25 alone gives 1.11, and the rho side is a model. |
| **Syzygy structure** (`out/lead_A1_syzygies/structure/`) | The extra syzygies are not products of lower-degree solutions, and not an orbit of one element (H_a, H_b refuted). H_c2 held in 6 of 6 cells. | skeptic agrees |
| **Rank-2 elimination, char-2 D4** (`runs/B_d4bin_probes/rank2/`) | **0.689× counted F_q multiplications** in the `frob_roots` arm at m = 12 (0.727× if the 7,260 Newton node inverses are charged), identical solutions 300/300. No gain in the baseline-roots arm (1.007). At m = 8 the current solver misses 3 solutions with s4 = 0. | skeptic agrees. Per-target rows were not saved. |
| **Closure falls cross-check** (`runs/A_fall_effective_variable/crosscheck/`) | **Confirmed** by an independently written closure (`iclose.c`, with its own descent). T35 and R36 fall at D = 4 on 4 of 4 instances; the T33 and R35 stall controls match. | skeptic agrees. There is still no exported certificate. |

**Nagao Hilbert stop: what changed after validation.** [pcc:unverified]
- **Held.**
  - A replay on RunPod, an independent host, reproduces all 10 records in every counted field.
  - A fresh same-binary matched-rho sample (runs 14–27, 126 walks) gives U2 at w = 0 of 1.050 / 0.876 / 0.728. The combined 252 runs give 1.026 / 0.856 / 0.711.
  - In counted F_p multiplications, the Nagao arm is below matched rho at p = 1451 and 1601.
- **Does not hold yet.** The validator's blocking findings:
  1. **No independent certificates** for 8 of the 9 cover solves or for the rho solves. `correct: true` is the solver checking itself. Re-running with `JV_DUMP_INSTANCE` fixes this; 1 of 9 was certified independently.
  2. **§5a.** The cover's native-operation count is a lower bound: F_p additions, F4 pair selection and matrix assembly are not emitted. Priced with an addition proxy, U2 at p = 1451 and 1601 would be *above* rho at w = 0.1 and 1. So "below rho across the band" is **not established**. `VALIDATION.json` now records `order_holds_across_band: false`.
  3. **The preregistered band was missed.** The prediction put Hilbert/plain in [0.50, 0.62] at w = 0.1 and 1. Measured, it is 0.821 and 0.959. The gain is 1.90× at w = 0 but only 1.04× at w = 1.
  4. **No S3 `bench.record/v1`**, because the AWS identity is rejected.
- **A recording defect.** `hilbert_check_muls` in the committed records is inflated by the thread count (6.4e10 recorded; 8.9e9 true, single-threaded). Do not quote it.

**RunPod spend:**

| pod | flavor | hours | cost |
|:--|:--|--:|--:|
| Nagao validation | 32 vCPU cpu5c | 1.56 | about $1.74 |
| exact e_min | 16 vCPU / 128 GB cpu5m | 6.5 | about $6.76 |
| quartic | 16 vCPU cpu5c | 1.95 | about $1.09 |
| closure cross-check | 16 vCPU cpu5m | 0.2 | about $0.21 |

All four were deleted, and a final listing shows none of them.

## Status update, 2026-11-12: the remaining open leads, run in parallel (workflow)

At the owner's request, a workflow ran one agent per open lead against its
preregistered test, plus an independent skeptic for each result. Write-ups are in
`runs/<lead>/RESULT.md` and the skeptics' notes in `runs/<lead>/VERIFY.md`. All
results are `[pcc:unverified]`. Where the skeptic disagreed, the skeptic's reading
is the one given here.

| lead | verdict | key numbers | skeptic |
|:--|:--|:--|:--|
| 3 `LEAD-C-jv-sieve-emit-price` | **partial** (accounting) | Counted F4, descent and LA additions / proxy: ratio of means 0.816, inside [0.80, 1.05]. Per seed 0.44–0.93, geometric mean 0.715. Layer-A priced cells move 0.976×; U1 at w = 1 is 0.1185, still below rho; distance ×8.2 → ×8.4. | agrees. The 0.816 depends on pricing an LA operation as 1 addition, not 16; with ×16 it is about 0.99. |
| 7 `LEAD-A-fall-effective-variable-law` | **falsified** | The rule "1 ∈ W_4 iff v ≤ 33" fails. T-type n = 35 (v = 34) and R-type n = 36 (v = 35) both *fall* at D = 4, 2 of 2 instances each. Treatise stalls reproduced (R35 codim 5287, T33 codim 4888). New native closure: `runs/A_fall_effective_variable/fallclose.c`. | agrees. The falls are checked by one tool only, with no exported certificate. |
| 8 `LEAD-A-closure-dimension-laws` | **held** | dim(W_3 ∩ B_≤3) = n(5n+1)/2 at n = 23–33, and dim(W_4 ∩ B_≤3) = 3n(n+2) at n = 35–43, on fresh instances. T-type differs. | agrees. Protocol deviation: n = 37–43 used the memory-lean `fallclose_q.c`, not the frozen tool. |
| 9 `LEAD-A-bilinear-lift-size-degree` | **partial** | The bilinear lift gives d_F = 3 at n = 11, 13, 15 (original d_F at n = 15 is 4). At n = 17 the full lift at D = 3 did not finish. The negative control (random bilinear forms) *also* drops 4 → 3, so the drop is not specific to the descent. | agrees. The control was built differently from the preregistration. |
| 10 `LEAD-ICB-tableB-binary-known-log-claw` | **prediction failed** (BSGS boundary result, outside the IC ratchet) | Under the preregistered band, known-log / matched rho-neg at w = 0 is 0.88, 1.03, 1.19, 1.27, 1.44 at f2m15–27. The prediction [1.4, 2.4] fails on 4 of 5. The low ratios come from rho-neg's high S at small r. | **disagrees with the producer's "held"**: the producer charged base generation, which the preregistration listed as unpriced. |
| 11 `LEAD-C-quartic-n4v2-k26` | **inconclusive, borderline kill** | 128-bit arithmetic fork reached k = 25. DL / rho-neg 0.0021 at k = 25 (w = 0) and 0.0012 at w = 1; k = 26 projected 0.0016. The k22–25 exponent is about 1.10, and its lower bound sits on the 1.10 kill line. | **disagrees with "partial / not triggered"**: four fresh calibrations put the lower bound at 1.1000–1.1034, at or above the kill line. Host calibration noise is about ±0.005. |
| 12 + 13 `LEAD-ICB-d4bin-…` | **closed / falsified as written** | Coset: the 21 carrying monomials have F2-rank of differences 4 on 300 of 300 instances, so there is no index-2 coset. Elimination: the quadratic part has rank 2, not 3, on 300 of 300 targets. | agrees. The rank-2 variant (two equations linear in S2, S3) is untested. |
| follow-up `FOLLOWUP-selector-extra-syzygies` | **held** | Extra c-only syzygies appear **only at e = e_min**. Excess over the trivial family: n = 15: 117–118; 17: 50–51; 19: 1034–1035; 21: 1034–1035; 23: 911–913; 25: 441–442; 27 (e = 5): 9307–9308. The excess is 0 at e_min − 1 on all 18 instances. Tool: `out/lead_A1_syzygies/syz.c`. | agrees. The preregistration sits inside RESULT.md; 12 of 37 cells were seen before. |

**Code changes, all opt-in with defaults unchanged:**
- `src/cryptanalysis/f4_fp.rs` and `examples/jv_cover.rs`: `JV_SIEVE_EMIT`.
- `src/cryptanalysis/gaudry_quartic_bin.rs`: `pub fn macaulay_staircase`, additive and not called by any solver path.
- New example `examples/d4bin_coset_quadratic_probe.rs`.

| Lane | File | Leads |
|:--|:--|--:|
| A — last fall degree and the selector proof system | `lane_A_falldegree.md` | 5 |
| B — binary-field IC engineering | `lane_B_binary_ic.md` | 4 |
| C — prime and extension fields, covers | `lane_C_prime_ext_cover.md` | 4 |

**Novelty check.** Every lead was compared against the 173 lead ids of the
2026-11-10 sweep (`EXISTING_LEADS.txt`, from `research/lead_sweep_20261110/`).
Each lane file names the nearest existing lead and the difference.

Two limits on that check:
- The crypto-autoresearcher lead atlas is vault-encrypted (`VLT1`) and was not
  read.
- The `research-kb` MCP server was not available.

So "new" means new with respect to this repository and the sweep only.

## Ranked list

| # | Lead | Lane | What it would change | Cheapest test | Prior / status |
|--:|:--|:--|:--|:--|:--|
| 1 | `LEAD-C-jv-nagao-hilbert-at-scale` | C | Nagao arm of the Joux–Vitse cover at p = 1301–1601 | 9 counted runs, about 5 min each, against the frozen rho reference | Measured 1.85–1.88× lower counted S at equal relations, p = 53–503, classified engineering (`research/ic_cover_weakclass_20261023/REPORT.md`); never run at scale |
| 2 | `LEAD-A-selector-eta-kernel` | A | Explains the selector system's rank, the k = 13 anomaly, and a reduction to the trace hyperplane | nullity mode in `emin.c`, under 15 min | Proved core; rank prediction already matches 9 of 9 certificates (below) |
| 3 | `LEAD-C-jv-sieve-emit-price` | C | Turns the one validated below-rho row (JV sieve, p = 1511, 0.0065×) from accounting into a priced result (treatise problem 11) | 16 cover seeds, under 5 min | Engineering; predicts emitted/proxy 0.80–1.05 |
| 4 | `LEAD-C-jv-sieve-hilbert-descent` | C | JV sieve total S × about 0.87 at p = 1511 | desk check on the 16 records, then a code change | Wiring gap confirmed (below) |
| 5 | `LEAD-ICB-d4bin-chain-variable-sweep` | B | k = 4 characteristic-2 per-trial cost (charpoly 45.8 % of the profile) | seconds for step 1 | Prior about 10 % |
| 6 | `LEAD-A-selector-counting-law` | A | Predicts where `e_min` dips; the Koszul-plus-hyperplane model puts the next dips at k = 16 and 19 | small-k grid, under 10 min | Conjecture |
| 7 | `LEAD-A-fall-effective-variable-law` | A | Rule `1 ∈ W_4` iff v ≤ 33; explains R33 falls versus T33 stalls | needs the native closure (vaulted; not on disk here) | Fitted rule; may point *against* a growing bound |
| 8 | `LEAD-A-closure-dimension-laws` | A | Closed forms for dim(W_3 ∩ B_≤3) and dim(W_4 ∩ B_≤3) | existing closure data | Fitted |
| 9 | `LEAD-A-bilinear-lift-size-degree` | A | k² extension variables y_ij = a_i c_j; claims d_F 4 → 3 at n = 15 | small closure, minutes | Instance of the size–degree trade-off of NOTE.md Lemma 2 |
| 10 | `LEAD-ICB-tableB-binary-known-log-claw` | B | Table B random-binary rows: predicts 1.4–2.4× matched rho, against 10.6–38.2× today | existing `ic.known_log_mitm2_v1` arm, under 5 min | **Needs an owner ruling first** (below) |
| 11 | `LEAD-C-quartic-n4v2-k26` | C | Plane-quartic double-large-prime run from k = 22 to k = 26 | needs 128-bit group-order arithmetic | Extends an existing row |
| 12 | `LEAD-ICB-d4bin-constant-quadratic-elimination` | B | k = 4 elimination of the weight-2 unknowns | about a day to implement | The agent's own count predicts *worse*; null registered |
| 13 | `LEAD-ICB-d4bin-exponent-coset` | B | Factor 2–4 on k = 4 if the 21 carrying monomials lie in an index-2 coset | one-minute probe | Prior about 5 % |

**Why the top two.**
- **Lead 1** is the cheapest test with the largest predicted *measured*
  improvement of an existing IC row [pcc:unverified]. Its mechanism (Hilbert early stop of F4) is
  already validated at smaller p, and the scale runs simply never used it.
- **Lead 2** has a proved core and a sharp falsifier: a non-trivial `c`-only
  syzygy. Data from this morning's runs already agree with it.

Leads 1, 3 and 4 move IC rows. Leads 2 and 6–9 bear on treatise problems 12 and 13,
not on any row.

## Checks made for this report

- **Lead 1, scale runs never used the Hilbert stop. Holds.**
  `research/jv_cover_crossover_20261101/run.sh`, `jv_cover_fbline_20261102/run.sh`
  and `jv_cover_p_scaling_20261024/run.sh` call `--exp dlp` with no `--stop`. The
  Hilbert rule was measured only in `research/f4_early_stop_20261018/`.
- **Lead 1, the 1.85× figure. Holds.** It comes from
  `research/ic_cover_weakclass_20261023/REPORT.md`: arm B (`hilbert:16`) lowers
  counted S by 1.848–1.875 per cell at p = 53–503, with equal relations in 25 of 25
  cells. That report classifies it as engineering, not an advance.
- **Lead 4, the sieve's descent cannot use the Hilbert stop. Holds.**
  `src/cryptanalysis/jv_sieve.rs:2086` builds its F4 options with
  `opts_for(24, 600.0, stop)`. In `src/cryptanalysis/jv_cover.rs:2734`, `opts_for`
  takes `Option<usize>` and only selects the staircase stop. The Hilbert rule is
  reachable only through `opts_for_mode` (`jv_cover.rs:2726`), which the sieve does
  not call.
- **Lead 5, the chain variable is fixed. Holds in substance.**
  `src/cryptanalysis/gaudry_quartic.rs:2235` takes `mvar` as the first variable of
  weight 1, so the choice is fixed by the weight vector and never searched.
- **Lead 2, rank deficit equals `M(k−1, e−1)`. Matches 9 of 9.**
  - Claim: the rank deficit of the selector system equals
    `M(k−1, e−1) = sum_{d<=e−1} C(k−1, d)`, the trivial homogeneous kernel.
  - Data: the coefficient-matrix ranks of every independently re-checked
    infeasibility certificate in `research/lfd_lower_bound_20261111/out/c/verify_*.txt`.
  - Result: deficit and prediction agree exactly in all 9 cases, at n = 33, 35,
    37, 39, 41 and e = 3, 4 (for example n = 35, e = 4: 141,680 − 140,846 = 834 =
    M(17, 3)). Figure: `figures/deficit_vs_kernel.svg`.
  - Caveat: the agent formed the claim from these same logs, so the match is a
    post-hoc consistency check, not the preregistered test. The test is the
    nullity mode the lead names.
- **Lead 10 needs an owner ruling before any work.** `ic.known_log_mitm2_v1` is a
  BSGS-type (known-log table) arm. The owner ruled on 2026-11-08 that BSGS-class
  results are labelled boundary results outside the index-calculus ratchet
  (`research/curve25519_autolab_20261104/bsgs/README.md`). As things stand, a win
  for lead 10 would not count as an IC improvement [pcc:unverified].

## What none of these is

None is a growing lower bound on `d_F`, and none is a sub-√p claim.

No lead is ready for an IC leaderboard row. Each would first have to pass its
test, then the validation gate (replay certificate, validator report, matched rho
with the §5a band).

Leads 1, 3, 4 and 11 should run on Iso Lab EKS once an AWS identity works. Until
then, local runs are development checks only.
