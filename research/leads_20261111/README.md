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
