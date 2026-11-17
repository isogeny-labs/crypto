# emin-exact-large-n: exact selector degree e_min above n = 29 (RunPod), 2026-11-12

All result statements here are `[pcc:unverified]`.

## 1. Preregistered prediction (written before any pod run)

Model A (`out/lead_A2/RESULT.md`, lead A2): `rho_A(k,n,e) = (k+1) M(k,e+1) / (n M(k,e))`,
`M(k,e) = sum_{i<=e} C(k,i)`. Selector of degree `e` predicted **feasible** if
`rho_A <= 1.2194`, **infeasible** if `rho_A >= 1.2698`, ambiguous in between.
Diagonal instances, `k = (n+1)/2`, random `V` (R-type, `emit_instance.py N SEED`).

| n | k | e | cols = n M(k,e) | rho_A | Model A |
|--:|--:|--:|--:|--:|:--|
| 31 | 16 | 4 | 78,027 | 1.5001 | infeasible (already observed, out/c) |
| 31 | 16 | 5 | 213,435 | 1.1862 | **feasible** |
| 33 | 17 | 4 | 106,062 | 1.5956 | infeasible (already observed) |
| 33 | 17 | 5 | 310,266 | 1.2634 | ambiguous |
| 33 | 17 | 6 | 718,674 | 1.0326 | feasible |
| 35 | 18 | 5 | 441,560 | 1.3417 | **infeasible** |
| 35 | 18 | 6 | 1,091,300 | 1.0969 | feasible |

**Predicted e_min (Model A):** n = 31 -> **5**; n = 33 -> **5 or 6** (ambiguous at e = 5);
n = 35 -> **6**.

Falsifiers: n = 31 e = 5 infeasible on an instance falsifies A's n = 31 prediction;
n = 35 e = 5 feasible falsifies A's n = 35 prediction. n = 33 e = 5 is not scored.

Cost forecast (single-threaded M4RI: the Ubuntu libm4ri is built without OpenMP;
cubic scaling from n = 29 e = 5, 143,376 cols, 1,057 s locally):
n = 31 e = 5 ~ 1 h, peak RAM ~ 12 GB; n = 33 e = 5 ~ 3 h, ~ 27 GB;
n = 35 e = 5 ~ 8.6 h, ~ 55 GB (over the 8 h pod budget); any e = 6 (>= 462k cols) is out of budget.

Instances: same first-unsatisfiable seeds as `out/c/`: n = 31 seeds 2, 3; n = 33 seeds 1, 5;
n = 35 seeds 6, 7.

Source: repo HEAD `2003c4bc85a6db91e74bfe19522a517b44995ff4`; `emin.c` sha256
`c5ee59f60742a0bf22efc23761e6903d01639e5c0092bb52b9854d26946eb22c` (last changed in
`370e85e2`).

## 2. Runs (RunPod, after the prediction above)

Pod `jozvsiy9t8avys` (`bdr-emin-exact-large-n`), cpu5m, 16 vCPU, 128 GB (cgroup
`memory.max` = 128,000,000,000), AMD Ryzen Threadripper 7960X, `costPerHr` 1.04 USD.
Created 2026-11-12 16:07:58 UTC. Image `runpod/base:1.0.2-ubuntu2404`.

Binary: `emin` built locally from `emin.c` (sha256 above, repo HEAD `2003c4bc`) with
`gcc -O2 -o emin emin.c -static -lm4ri -lpng -lz -lm` (Ubuntu libm4ri 20200125, SSE2,
no OpenMP); sha256 `5ce4b70870f260dc46799c00a1b384e0caa67f57c0c22849bbf6532d16ae5cf1`
(verified on the pod by `rp.sh put`). Instance lines from `python3 emit_instance.py N SEED`
locally (`emin count` = 0 for all six, both locally and on the pod).

Per job, on the pod (`job.sh N SEED 5 500000`, kept in `pod/job.sh`):
```
./emin solve 5 500000 SEED cert/nN_sSEED < inst_nN_sSEED.txt
./emin verify E cert/nN_sSEED/points_eE.txt < inst   # every infeasible E >= 3 (PLUQ, independent row build)
```
Feasible `lambda` certificates were downloaded and checked **locally** (independent
machine, `lfd_small` field arithmetic):
`python3 -I verify_lambda.py N SEED 5 cert/nN_sSEED/lambda_e5.bin`.
(The pod-side `verify_lambda.py` call failed: no numpy on the base image; local check replaces it.)

Jobs launched 16:09 UTC: n31 s2, n31 s3, n33 s1, n33 s5, n35 s6 (all concurrent);
n35 s7 added at 16:49 once the n = 31 jobs had freed memory.
Extra seeds added at 18:18 (spare cores): n31 s5, s6 and n33 s8 (the next
unsatisfiable seeds after the out/c ones; `emin count` = 0 locally).

## 3. Results `[pcc:unverified]`

| n | k | seed | e = 4 (78k / 106k cols) | e = 5 solve | e = 5 lambda check (local) | **e_min** | solve wall | peak RSS |
|--:|--:|--:|:--|:--|:--|--:|--:|--:|
| 31 | 16 | 2 | infeasible; `emin verify`: 5,228 pts, rank 77,451 -> 77,452, inconsistent | feasible, 3 rounds, 15,819 pts | violated 0, ANF degree 5, ok | **5** | 2,415 s | 11.3 GB |
| 31 | 16 | 3 | same (5,228 pts, inconsistent) | feasible, 3 rounds, 15,819 pts | violated 0, degree 5, ok | **5** | 2,410 s | 11.3 GB |
| 31 | 16 | 5 | same (inconsistent) | feasible, 3 rounds | violated 0, degree 5, ok | **5** | 2,458 s | 11.3 GB |
| 31 | 16 | 6 | same (inconsistent) | feasible, 3 rounds | violated 0, degree 5, ok | **5** | 2,434 s | 11.3 GB |
| 33 | 17 | 1 | infeasible; `emin verify`: 6,690 pts, rank 105,365 -> 105,366, inconsistent | feasible, 4 rounds, 23,877 pts | violated 0, degree 5, ok | **5** | 7,408 s | 24.6 GB |
| 33 | 17 | 5 | same (6,690 pts, inconsistent) | feasible, 4 rounds, 23,877 pts | violated 0, degree 5, ok | **5** | 7,421 s | 24.6 GB |
| 33 | 17 | 8 | same (6,690 pts, inconsistent) | feasible, 4 rounds, 23,877 pts | violated 0, degree 5, ok | **5** | 7,245 s | n.r. (same size as s1) |

e = 3 infeasibility was also re-verified with `emin verify` for every instance (inconsistent).
All lower degrees e = 0..2 are infeasible in the solve (implied by e = 3 infeasibility,
since the degree-<=e selectors are nested).

Exact selector degree, all instances run: **e_min(n=31) = 5** (4 / 4 instances),
**e_min(n=33) = 5** (3 / 3 instances). `[pcc:unverified]`

### n = 35 (seeds 6, 7): e = 5 NOT decided — tool limit hit

- e = 4 infeasible again on both (8,448 points; `emin verify`: rank 140,846 -> 140,847,
  inconsistent); e = 3 likewise. So `e_min(n=35) > 4`, unchanged from out/c.
- e = 5 (441,560 cols): the round-1 M4RI echelonisation ran to completion
  (~5 h 15 min), then both processes aborted with
  `mzd.h:306: mzd_row: Assertion 'result == M->rows[row]' failed` (signal 6;
  s6 at 5:20:26, s7 at 5:14:41 wall; peak RSS 23.9 GB).
- **Cause (diagnosed from the header, not yet tested):** M4RI 20200125's inline `mzd_row`
  computes `wi_t big_vector = offset_vector + row * rowstride` in `int` (`wi_t`, `rci_t` are
  `int`). For 441k rows x 6,900 words per row = 3.0e9 words > 2^31 this overflows.
  `emin.c`'s `first_bit` calls `mzd_row`. A square system of N columns overflows once N^2/64 > 2^31,
  i.e. N > ~370k. n = 33 e = 5 (310k cols, 343k rows x 4,848 words = 1.7e9) was under the limit.
  **Compiling with `-DNDEBUG` is NOT a fix:** without the assert, `mzd_row` returns the
  overflowed pointer. Library-internal paths that use `mzd_row` past 2^31 words may also be wrong
  even where they do not assert, so any >2^31-word result from this M4RI build needs an
  independent check.
- Consequence: with this binary, n = 35 e = 5, n = 37+ e >= 5, and every e = 6 system
  (>= 462k cols) are out of range regardless of budget.

## 4. Verdict against the preregistered prediction `[pcc:unverified]`

| n | Model A predicted e_min | observed | verdict |
|--:|:--|:--|:--|
| 31 | 5 (rho_A(5) = 1.186 < 1.2194) | **5** on 4/4 instances | **holds** |
| 33 | 5 or 6 (rho_A(5) = 1.263, ambiguous band) | **5** on 3/3 instances | consistent; not scored. Narrows the feasibility threshold: theta_A > 1.2634 (was > 1.2194) |
| 35 | 6 (rho_A(5) = 1.342 >= 1.2698) | e_min > 4 only; e = 5 undecided | **not tested** (tool limit) |

The n = 33 result moves A's feasible/infeasible boundary to theta_A in (1.2634, 1.2698).
That is a narrow band, so the n = 35 cell (rho_A(5) = 1.342) remains a sharp falsifier for A.
The diagonal sequence now reads e_min = 3, 3, 4, 4, 4, 4, 5, 5, 5, 5 for n = 15, 17, ..., 33.

## 5. RunPod accounting

| pod id | name | flavor | vCPU / RAM | created (UTC) | deleted (UTC) | hours | costPerHr | cost |
|:--|:--|:--|:--|:--|:--|--:|--:|--:|
| `jozvsiy9t8avys` | `bdr-emin-exact-large-n` | cpu5m | 16 / 128 GB | 16:07:58 | 22:37:46 | 6.50 | 1.04 USD | ~6.8 USD |

The first attempt at 32 vCPU cpu5m was refused (no capacity), so no pod was created.
After the delete, the account pod listing shows no pod named `bdr-emin-exact-large-n`
(checked 22:37:46 UTC). No other pod was touched. Total pod time was 6.5 h of the 8 h allowance.

## 6. Files

- `cert/n{31_s2,31_s3,31_s5,31_s6,33_s1,33_s5,33_s8}/`: `line.json` (as out/c's jsonl rows),
  `solve.json`, `solve.log` (with `/usr/bin/time -v`), `verify_e3/e4.{json,log}` (pod `emin verify`),
  `lambda_e5.bin` (feasibility certificate), `verify_lambda_e5_local.txt` (local check),
  `points_e4.txt` (the e = 4 infeasibility point set).
- `cert/n35_s{6,7}/`: e <= 4 results, the aborted e = 5 `solve.log`, and verify e3/e4.
- `pod/`: `job.sh`, the instance lines, and `final_listing.json` (pod /work listing before the delete).
- `SHA256SUMS`.

## 7. Blocked / next step

- **Blocked:** n = 35 e = 5 (and every system above ~370k columns) is blocked by the
  `int` row-offset overflow in M4RI 20200125, not by budget. The emin.c round loop also
  needs ~2x the matrix in RAM (`E` and `A` together): ~50 GB at n = 35 e = 5.
- **Next:**
  1. Build against a 64-bit-safe M4RI, or patch `emin.c` opt-in (for example an
     `EMIN_ROWPTR=1` build flag) so that it reads `M->rows[r]` instead of `mzd_row`. Then
     validate on a synthetic >2^31-word matrix against a known rank before trusting it.
  2. Rerun n = 35 e = 5 (~5.5 h solve on a Threadripper 7960X with 2 jobs sharing the pod)
     plus `emin verify` (PLUQ; ~cubic, scaled from n = 33 e = 4 = 5 min, about 6 h). That fits
     one 8 h pod only if the two verify eliminations (B and A|b) run as separate processes.
  3. Model A's n = 35 prediction (e = 5 infeasible) is the sharp test. A feasible e = 5 at
     n = 35 would falsify it.
- Pod-side `verify_lambda.py` needs numpy (`apt-get install -y python3-numpy`). Here it was
  run locally instead.
