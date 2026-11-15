# VERIFY: closure-falls-crosscheck (independent skeptic), 2026-11-12

Every statement here is `[pcc:unverified]`. One skeptic agent ran these checks on the shared 4-vCPU container,
single-threaded. There is no pcc claim and no replay certificate.

## Verdict: the key claim is not refuted. **Agree** `[pcc:unverified]`

I tried to break the claim that 1 is in W_4 for T35 k17 seeds 1 and 2 and R36 k18 seeds 1 and 2. I used
a third closure (`verify/vclose.c`) and a third Weil descent (`verify/mydescent.py`). I wrote both myself,
without reading the internals of `iclose.c` or `fallclose.c`.

- **Falls.** My closure finds 1 in W_4 on all four fall instances.
- **Stall controls.** It reproduces both stall controls exactly. This includes the product count, which
  matches iclose and fallclose.
- **Unsatisfiability.** I counted solutions exactly with my own method. All four fall instances are
  unsatisfiable. This is a necessary condition for a fall, and here it does not depend on `emin`.
- **Producer's arithmetic.** I re-derived every number from the producer's raw records, and all are
  consistent.
- **Pod.** No pod from this task is still running.

## Checks and results

### 1. Instances and descent

Code: `verify/mydescent.py`. It does symbolic ANF from S3 = (X1X2 + z(X1+X2))^2 + zX1X2 + a6 with its own
carry-less GF(2^n) arithmetic. It checks itself against direct evaluation of S at 300 random points.

- **Emitters.** All 8 instance lines were regenerated from the committed emitters. That is the 6 claimed
  instances plus the 2 satisfiable negative controls T35s4 and R34s1. All 8 match
  `../out/instances.jsonl` and `sys/*.line` byte for byte.
- **System files.** My system equals the producer's `sys/*.sys` polynomial by polynomial on all 7 shared
  instances. The files are byte-identical; my copies are in `verify/out/*.mysys`. So `gen_system.py` is
  correct on these instances.
- **Exact solution counts.** These are independent of `emin`. For each a in GF(2)^k (Gray code), I solve
  the GF(2)-linear system in b exactly. This works because S is quadratic in X2, so it is GF(2)-linear in
  X2 for fixed X1.

| instance | sols (mine) | sols (instances.jsonl / emin) |
|:--|--:|--:|
| T35k17s1, T35k17s2, R36k18s1, R36k18s2, T33k17s2, R35k18s6, T32k16s3 | 0 each | 0 each |
| T35k17s4 (positive control for the counter) | 4 | 4 |
| R34k17s1 (positive control for the counter) | 2 | 2 |

Raw output: `verify/out/descent.txt`.

### 2. Own closure, validated before use

`verify/vclose.c` is about 110 lines and uses libm4ri. It works in rounds:

- Each round stacks the current RREF basis with a block of products x_i·f for low-degree rows whose pivot
  is new.
- It then runs a full `mzd_echelonize_m4ri(.., full=1)`.

How it differs from the producer's tools:

- Re-echelonization is round-based. It does not use addmul/Gray-table batch reduction.
- Within each degree, columns are ordered by a seeded xorshift-multiply hash, with column seed 5. The
  producers used colex, seed 1 and seed 7.
- Low rows are taken from my own system files.

M4RI is shared with fallclose, but none of fallclose's code is.

**Validation.** I ran vclose on all 72 reference rows of `../out/crosscheck.jsonl`, with column seeds 3
and 11, using my own descent: 144 runs, **0 disagreements**. In those runs:

- 60 runs are falls, and every fall is on an unsatisfiable instance.
- 84 runs are stalls, and every stall matches every cumulative dimension.
- All 60 satisfiable cases stall.

Raw output: `verify/out/small_validate.jsonl`.

**Results at D = 4.** Settings: 1 thread, column seed 5, block 16384, stop at fall. Raw output:
`verify/out/vclose.jsonl`, plus per-round logs in `verify/out/log_*.txt`.

| instance | vclose | W_dims at stop | rank / ncols | products | sec | RSS | iclose / fallclose |
|:--|:--|:--|:--|--:|--:|--:|:--|
| T35k17s1 | **fall** | (1,35,596,6578,52954) | 52954 / 52956 | 115634 | 148 | 0.47 GB | fall / fall |
| T35k17s2 | **fall** | (1,35,596,6579,52955) | 52955 / 52956 | 115634 | 149 | 0.47 GB | fall / fall |
| R36k18s1 | **fall** | (1,37,667,7801,66705) | 66705 / 66712 | 115956 | 279 | 0.70 GB | fall / fall |
| R36k18s2 | **fall** | (1,37,667,7801,66705) | 66705 / 66712 | 115956 | 299 | 0.70 GB | fall / fall |
| T33k17s2 (stall control) | stall | (0,0,64,2968,48068), codim 4888 | 48068 / 52956 | 100912 | 151 | 0.47 GB | identical |
| R35k18s6 (stall control) | stall | (0,1,105,3885,61425), codim 5287 | 61425 / 66712 | 139860 | 340 | 0.70 GB | identical |

Reading the table:

- The fall rows are snapshots taken when 1 first appears in the RREF. Their degree-3 and degree-4 counts
  depend on the order, so they differ from iclose's.
- In a fall, 1 is in W, so W_4 = B_{<=4}. That is consistent with the full dimension vectors that the pod
  and fallclose recorded.
- The stall controls match iclose (local and pod) and fallclose exactly, including the product count. They
  also match the treatise records T33a/b and R35a/b in
  `papers/ecdlp-index-calculus-treatise/figures/data-bin-fd-instances.json`.
- The generator ranks are 34, 34, 36, 36, 32 and 35, the same as both producers.

### 3. Re-derivation of the producer's raw numbers (own script, inline Python)

I checked each of the 12 records in `out/local.jsonl` and `out/pod.jsonl`:

- ncols = C(N,<=4): 52956 at N=34 and 66712 at N=36.
- codim = ncols - rank, and the last entry of W_dims equals rank.
- For falls, the low part of W_dims is (1, N+1, C(N,<=2), C(N,<=3)).
- one_in, W_dims and gen_rank equal fallclose's `../out/cells.jsonl` on every instance.

All checks pass. `out/pod/exec_8.log` contains the same JSON lines as `out/pod.jsonl`, and `lscpu` there
shows a Threadripper 7960X.

### 4. Re-run of the producer's tool (cheapest key measurement)

`iclose.c` rebuilt locally (binary sha256 `147c53c7…`; source sha256 `e8183ccd…`, which matches RESULT.md).
The run used T35k17s1 with an unused column seed 13, FIFO and 1 thread:

- Result: **fall**, (1,35,596,6580,52956), 116416 products, 100.5 s.
- The product count equals fallclose's 116416 for this instance.

Raw output: `verify/out/iclose_rerun_T35k17s1.json`.

### 5. Thresholds, source changes and RunPod

- **Preregistered thresholds.** This cross-check task had no new numeric threshold. The v-law prediction
  it supports was frozen in `../RESULT.md` and the lane note before the runs. The crosscheck did not
  change cells, instances or the definition of D.
- **Validation order.** The validation-before-use step (144/144) was done and recorded before the large
  runs; the file timestamps order this way.
- **Source changes.** `git status --porcelain` shows no modified tracked file. `fallclose.c` and the
  emitters are unchanged, and the crosscheck directory is untracked and new. No default behaviour
  changed, because no existing code changed.
- **RunPod.** I listed the pods at 22:40Z and 23:08Z. Neither listing contains a
  `bdr-closure-falls-crosscheck` pod or pod id `3s0i2d2itxl75h`. The one `bdr-*` pod present,
  `bdr-coverfb-r5`, belongs to another task and was not touched. The reported cost (~0.2 h at
  $1.04/h, ~$0.21) is plausible from the log timestamps. I cannot verify the billed amount.

## Issues found (none refute the claim)

1. **Wording.** RESULT.md says the stall controls are "identical to fallclose and the treatise". For the
   treatise, only the dimension vectors are identical; the treatise instances are different seeds. The
   producer states this elsewhere, so it is a wording issue only.
2. **Independence limits remain.**
   - All three closures implement the same definition of W_D and read systems derived from the same
     emitters.
   - vclose and fallclose both link libm4ri. My descent is the same symbolic expansion that fallclose is
     described as using, though written separately.
   - There is still no exported certificate of 1 in W_4, and no pcc claim.
3. **Satisfiability reliance on `emin`.** RESULT.md takes solution counts from `emin`. My exact counter
   now confirms them independently (0 for the six claimed instances, plus 4 and 2 on the controls).

## Commands (from `crosscheck/`)

```
gcc -O3 -march=native -Wall -o /tmp/claude-0/vclose verify/vclose.c $(pkg-config --cflags --libs m4ri)
python3 verify/mydescent.py verify/lines/<t>.line sys/<t>.sys verify/out/<t>.mysys   # -> verify/out/descent.txt
python3 verify/small_validate.py /tmp/claude-0/vclose /tmp/claude-0/vrf/x > verify/out/small_validate.jsonl
gcc -O3 -march=native -fopenmp -Wall -o /tmp/claude-0/iclose_v iclose.c
OMP_NUM_THREADS=1 verify/run_verify.sh /tmp/claude-0/vclose /tmp/claude-0/iclose_v   # -> verify/out/vclose.jsonl, iclose_rerun_*.json
curl -sS https://rest.runpod.io/v1/pods -H "Authorization: Bearer proxy-injected"    # listing only, no pod created
```

`verify/lines/*.line` are copies of the `line` fields of `../out/instances.jsonl` (checked against the emitters).

## Hashes

| file | sha256 |
|:--|:--|
| verify/vclose.c | de589a1c2a674765537a5f24d5b713046062a966eeccdd7a048a1340c7d2f51b |
| /tmp/claude-0/vclose (binary) | 279b7a8692d0e59a068eb6e74f23b0d794b7ff3c00affdc9b73ff466d1170f9f |
| verify/mydescent.py | 77a89c5adc52753c8a75f5f1f0e6b26d39d3335a54081f63b01adcae0670bd83 |
| verify/small_validate.py | d2c43211debbb7655e104ff77044c35b726cfbae9387906ac346005ec5bffbd3 |
| verify/out/vclose.jsonl | 5837484f16763b296df1f81985b464685b3b0e79ac9acff0efb7df80c39b8647 |
| verify/out/small_validate.jsonl | d78eee3ed40748578cc00222fcf5294016f5c058bbb978bacd435171114a8e2c |

Repository HEAD: `2003c4bc85a6db91e74bfe19522a517b44995ff4`. The `verify/` files are new and untracked.

## Next step

- File a pcc claim for "1 in W_4 at T35 k17 s1,s2 and R36 k18 s1,s2". It should cite
  `out/local.jsonl`, `out/pod.jsonl`, `../out/cells.jsonl` and `verify/out/vclose.jsonl`.
- If full independence is wanted, build an exported, checkable certificate. Example: the round-by-round
  row provenance of vclose, where each round's products are re-verified by a separate checker.
