# LEAD-A-closure-dimension-laws (lane A, lead A4): result

Date 2026-11-12. Checkout `/home/user/crypto`, branch `claude/binary-decomposition-research-6cez5r`, HEAD `6b7fc452` at run time
(uncommitted working tree shared with other agents). Container: 4 vCPU / 15 GB, shared; every job single-threaded.
Every statement below is `[pcc:unverified]`: no pcc claim was filed, no replay certificate exists, and nothing here is a
measured improvement or concerns ECDLP cost, deployed curves or key recovery.

## 1. What was tested (frozen in `PREREG.md` before any run, copied unchanged from `lane_A_falldegree.md`)

- **L3**: `dim(W_3 ∩ B_{<=3}) = n(5n+1)/2` and `dim(W_3 ∩ B_{<=2}) = 3n` for every unsatisfiable random-V curve-z instance,
  k = ceil(n/2), odd n, 15 <= n <= 33. Predictions n = 23..33: 1334, 1575, 1836, 2117, 2418, 2739.
- **L4**: `dim(W_4 ∩ B_{<=3}) = 3n(n+2)` and `dim(W_4 ∩ B_{<=2}) = 3n` for every stalled random-V instance, n >= 35
  (n = 41: 5289; n = 43: 5805).
- Falsifier: one deviating dimension at any tested n kills that law (L3 and L4 independent).
- Controls: positive = logged values n = 15..21 (L3) and n = 35 total 61,425 (L4); negative = T-type (V in ker Tr) must not
  satisfy the formulas; A/A = two instances per n.

## 2. Tools and instance rule

- `fallclose.c`: byte-identical copy (sha256 `f8d33898…b52f`) of `../A_fall_effective_variable/fallclose.c` (lane A3 agent's
  native M4RI closure W_D; generators built symbolically, no 2^N enumeration). Not modified.
- `fallclose_q.c` (new, this directory only; sha256 `2262773d…c0b9`): the same closure with W stored in compressed RREF
  (only non-pivot columns kept), so memory is r·(ncols−r) bits instead of ncols². Same algorithm, same output format.
  **Validated bit-for-bit** against `fallclose` (W_dims, codim, one_in, gen_rank, stopped_early): 54 configurations
  in `out/validate_q.txt` (n = 15..25, D = 3/4, batch 256/512/8192, FULL 0/1, R and T types, falling and non-falling),
  the 29 D=3 cells of `out/cells.jsonl` (0 mismatches), and the L4 positive control R35 s6 at D=4
  (both tools: `[0, 1, 105, 3885, 61425]`, codim 5287; 226 s / 1.17 GB vs 99 s / 0.49 GB).
- No shared source file was edited (default behaviour of every committed tool is untouched; nothing to prove there).
- Instances: `select.sh` — R-type = `lfd_small.make_instance` via `../../../lfd_lower_bound_20261111/emit_instance.py`,
  T-type = `emit_instance_T.py`, k = (n+1)/2; satisfiability by `emin count` (exact solution count); the first two
  unsatisfiable seeds per n. Every scanned seed is in `out/scan.jsonl` / `out/extra_scan.jsonl`, chosen instances in
  `out/instances*.jsonl` (full instance line stored with every result).
- `research-kb` MCP was not available to this subagent (no such tool exposed); no KB query was made.

## 3. Exact commands

```
gcc -O3 -march=native -o fallclose   fallclose.c   -lm4ri -lm
gcc -O3 -march=native -o fallclose_q fallclose_q.c -lm4ri -lm
./select.sh R n 2            # n = 23..41 odd (and positive-control seeds added by hand to out/scan.jsonl)
INSTFILE=instances_T.jsonl ./select.sh T n 2 40     # n = 35, 37; T 23/27/31/33 via ./select.sh T n 2
./run_d3.sh                  # preregistered D=3 grid: posL3 (n=15..21 logged seeds), L3 (n=23..33 x2), negT (T 23,27,31,33 x2)
./run_d3_extra.sh            # ADDED, exploratory: 4 more seeds per odd n=23..33, odd n=35..43, even n=24..34 (D=3)
./run_d4.sh                  # fallclose D=4: posL4 R35 s6, L4 R35 s7 (driver stopped after these two, see run_d4q.sh)
./run_d4q.sh                 # fallclose_q D=4: posL4q R35 s6, L4 R37 s1,s3, R39 s3,s7, negT4 T35 s2, T37 s2, L4 R41 s1 (batch 4096)
./run_d4q_tail.sh            # fallclose_q D=4, batch 8192: L4 R41 s2, L4 R43 s3
./run_d4q_43b.sh             # fallclose_q D=4, batch 8192: L4 R43 s5 (A/A second instance at n=43)
python3 summarize.py > out/summary_table.md   # every cell, one row each (tables below are condensed from it)
```
`run_d4q43.sh` was killed before it ran any cell (superseded by `run_d4q_tail.sh`). `fallclose`, `fallclose_q` are
build outputs of the two `.c` files (not needed in Git).
Each cell: `echo "<instance line>" | RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 timeout 1200 ./fallclose[_q] D BATCH 1`
(FULL=1: run to the fixed point). Raw JSON per cell: `out/cells.jsonl`, `out/cells_extra.jsonl`; progress logs `out/log/`.

## 4. Results

W_dims = cumulative `dim(W_D ∩ B_{<=e})`, e = 0..D. N = 2k Boolean variables.

### 4a. L3 (D = 3), preregistered cells

| role | type | n | k | seeds (unsat) | W_3 dims | predicted (B<=2, B<=3) | verdict |
|:--|:--|--:|--:|:--|:--|:--|:--|
| positive | R | 15 | 8 | 1, 5, 6 | [0, 1, 45, 570] | (45, 570) | reproduces log |
| positive | R | 17 | 9 | 1, 2, 8 | [0, 1, 51, 731] | (51, 731) | reproduces log |
| positive | R | 19 | 10 | 4, 7 | [0, 1, 57, 912] | (57, 912) | reproduces log |
| positive | R | 21 | 11 | 3 | [0, 1, 63, 1113] | (63, 1113) | reproduces log |
| test | R | 23 | 12 | 4, 8 | [0, 1, 69, 1334] | (69, 1334) | match |
| test | R | 25 | 13 | 5, 6 | [0, 1, 75, 1575] | (75, 1575) | match |
| test | R | 27 | 14 | 3, 6 | [0, 1, 81, 1836] | (81, 1836) | match |
| test | R | 29 | 15 | 3, 4 | [0, 1, 87, 2117] | (87, 2117) | match |
| test | R | 31 | 16 | 2, 3 | [0, 1, 93, 2418] | (93, 2418) | match |
| test | R | 33 | 17 | 1, 5 | [0, 1, 99, 2739] | (99, 2739) | match |
| negative | T | 23 | 12 | 2, 35 | [0, 0, 44, 1078] | (69, 1334) | deviates (as required) |
| negative | T | 27 | 14 | 12, 17 | [0, 0, 52, 1482] | (81, 1836) | deviates |
| negative | T | 31 | 16 | 16, 17 | [0, 0, 60, 1950] | (93, 2418) | deviates |
| negative | T | 33 | 17 | 2, 18 | [0, 0, 64, 2208] | (99, 2739) | deviates |

All cells: one_in = 0 at D = 3; each run < 0.1 s.

### 4b. L3, exploratory (ADDED after 4a, not preregistered; `run_d3_extra.sh`, `out/cells_extra.jsonl`)

- 4 further unsatisfiable seeds per odd n = 23..33 (24 instances): all `[0, 1, 3n, n(5n+1)/2]`.
- Odd n outside the stated range, 2 seeds each: n = 35: 3080, 37: 3441, 39: 3822, 41: 4223, 43: 4644 — all equal
  n(5n+1)/2, B<=2 = 3n.
- Even n = 24..34 (k = n/2, so N = n, not n+1), 2 seeds each: 1404, 1651, 1918, 2205, 2512, 2839 — the odd-n formula
  does **not** apply (it is stated for odd n only); all six equal n(5n−3)/2 (post-hoc fit, B<=2 = 3n still holds).
  Both families fit `n(4N + n − 3)/2` (post-hoc, not a tested law).
- T-type at D = 3 (the four n of 4a) equals (n−1)(2n+3) = 2(n−1)(N+1) − (n−1) (post-hoc, 4 points).

### 4c. L4 (D = 4), stalled random-V instances

| role | tool | type | n | k | N | seed | one_in | W_4 dims | predicted (B<=2, B<=3) | codim | sec | max RSS |
|:--|:--|:--|--:|--:|--:|--:|--:|:--|:--|--:|--:|--:|
| positive | fallclose | R | 35 | 18 | 36 | 6 | 0 | [0, 1, 105, 3885, 61425] | (105, 3885) | 5287 | 226 | 1.17 GB |
| positive | fallclose_q | R | 35 | 18 | 36 | 6 | 0 | [0, 1, 105, 3885, 61425] | (105, 3885) | 5287 | 99 | 0.49 GB |
| fresh inst. (fit size) | fallclose | R | 35 | 18 | 36 | 7 | 0 | [0, 1, 105, 3885, 61425] | (105, 3885) match | 5287 | 229 | 1.20 GB |
| fresh inst. (fit size) | fallclose_q | R | 37 | 19 | 38 | 1 | 0 | [0, 1, 111, 4329, 72927] | (111, 4329) match | 10066 | 180 | 0.63 GB |
| fresh inst. (fit size) | fallclose_q | R | 37 | 19 | 38 | 3 | 0 | [0, 1, 111, 4329, 72927] | (111, 4329) match | 10066 | 177 | 0.63 GB |
| fresh inst. (fit size) | fallclose_q | R | 39 | 20 | 40 | 3 | 0 | [0, 1, 117, 4797, 85761] | (117, 4797) match | 16330 | 316 | 0.88 GB |
| fresh inst. (fit size) | fallclose_q | R | 39 | 20 | 40 | 7 | 0 | [0, 1, 117, 4797, 85761] | (117, 4797) match | 16330 | 313 | 0.89 GB |
| **test (new size)** | fallclose_q | R | 41 | 21 | 42 | 1 | 0 | [0, 1, 123, 5289, 99999] | (123, 5289) **match** | 24315 | 824 | 1.14 GB |
| **test (new size)** | fallclose_q | R | 41 | 21 | 42 | 2 | 0 | [0, 1, 123, 5289, 99999] | (123, 5289) **match** | 24315 | 580 | 1.23 GB |
| **test (new size)** | fallclose_q | R | 43 | 22 | 44 | 3 | 0 | [0, 1, 129, 5805, 115713] | (129, 5805) **match** | 34273 | 1033 | 1.63 GB |
| **test (new size)** | fallclose_q | R | 43 | 22 | 44 | 5 | 0 | [0, 1, 129, 5805, 115713] | (129, 5805) **match** | 34273 | 1037 | 1.67 GB |
| negative | fallclose_q | T | 35 | 18 | 36 | 2 | 0 | [0, 0, 68, 3323, 57503] | (105, 3885) | 9209 | 87 | 0.47 GB |
| negative | fallclose_q | T | 37 | 19 | 38 | 2 | 0 | [0, 0, 72, 3698, 68078] | (111, 4329) | 14915 | 154 | 0.65 GB |

All R-type cells stalled (one_in = 0), so all count for L4. The T-type negatives also stall and deviate, as required
(their cubic level exceeds the free-module count 2(n−1)(N+1): 3323 > 2516, 3698 > 2808).
Note: the n = 35..39 instances are fresh (lfd_small seeds) but those sizes were used for the fit (from the treatise's
instance file, not on disk here); only n = 41 (and 43) are new sizes.

## 5. Verdict against the preregistered falsifiers

- **L3: HELD** at every preregistered cell. Six new sizes n = 23, 25, 27, 29, 31, 33, two unsatisfiable instances each
  (12/12): `dim(W_3 ∩ B_{<=2}) = 3n` and `dim(W_3 ∩ B_{<=3}) = n(5n+1)/2` exactly, matching the six frozen predictions
  1334 … 2739. Positive control reproduced all 9 logged n = 15..21 values; the T-type negative control deviates at all
  8 cells (B<=2 = 2(n−1), B<=3 = (n−1)(2n+3)), so the formula is not satisfied trivially. Exploratory additions (not
  preregistered): 24 more odd-n instances in range and 10 instances at odd n = 35..43 also match; even n do not follow
  the odd-n formula (outside the stated scope). `[pcc:unverified]`
- **L4: HELD** at every preregistered cell. New sizes n = 41 (seeds 1, 2) and n = 43 (seeds 3, 5): all stalled, and
  `dim(W_4 ∩ B_{<=2}) = 3n`, `dim(W_4 ∩ B_{<=3}) = 3n(n+2)` = 5289 and 5805 exactly, as predicted. Positive control
  reproduced n = 35 total 61,425 with both tools; fresh instances at the fit sizes n = 35, 37, 39 (5 instances) also
  match; T-type stalled negatives at n = 35, 37 deviate. Within each n, both instances gave identical full filtrations
  (including the total dim W_4 and codim), so the A/A spread is zero at every n. `[pcc:unverified]`
- Neither falsifier fired. Scope of what this shows: the laws hold on 2 random lfd_small instances per size up to
  n = 43 (L4) and n = 33 (L3, preregistered; 43 exploratory). It is evidence for a structural statement, not a proof,
  and it says nothing about d_F growth or ECDLP cost (no gap to rho is affected).
- Observed but not preregistered: the total dim W_4 at the stall also has a regular pattern (61425, 72927, 85761, 99999,
  115713 for n = 35..43); not fitted here.

## 6. What is blocked or not done

- Iso Lab / EKS / S3, benchrec records and replay certificates: not available (AWS identity rejected). These are
  exact linear-algebra dimension counts, not timing claims; the `sec` column is a single-run wall time on a shared host.
- `research-kb` MCP not exposed to this subagent; `crypto-autoresearcher` lead atlas is vault-encrypted (not touched).
- The treatise's instance file `data-bin-fd-instances.json` (source of the L4 fit) is not on disk; the fit instances
  themselves were not re-run, only lfd_small instances at the same sizes.
- No pcc claim filed; every number is `[pcc:unverified]`.

## 7. Next steps

- A proof: L4 reads as "B_{<=1}·G_2 is free of rank (N+1)·3n and nothing else enters W_4 ∩ B_{<=3}". The first half is a
  linear-independence statement about x_i·g (g in a fixed 3n-dim space) that may be provable from Lemma `deg3`/Lemma 1;
  L3's deficit n(n+11)/2 = 3n(N+1) − n(5n+1)/2 (odd n) is the number of degree-3 relations at D = 3 and should be
  explained by the same basis (even n, k = n/2: 3n(N+1) − n(5n−3)/2 = n(n+9)/2 relations, post-hoc).
- `fallclose_q` makes D = 4 at n = 41–43 fit in ~1–2 GB; D = 5 at n ≤ 25 would test whether the same free-module
  pattern holds one level up (W_5 ∩ B_{<=4} vs (C(N,2)+N+1)·3n), a natural next law.
- Reconcile the "4n new cubic dimensions per round" phrase in prog_openproblems.tex (lane-file risk) with these counts.
