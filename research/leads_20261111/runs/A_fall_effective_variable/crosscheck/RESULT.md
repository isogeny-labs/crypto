# closure-falls-crosscheck: independent closure confirms the degree-4 falls at T35 and R36, 2026-11-12

Task: cross-check the degree-4 falls (1 in W_4) that `../fallclose.c` reported for T-type n=35, k=17
and R-type n=36, k=18 (`../RESULT.md`, `../VERIFY.md`: one tool, no certificate), and re-confirm the
stall controls. Every result below is `[pcc:unverified]`: one agent, no replay certificate, no pcc claim.

## Verdict

**Agreement on every instance, on two hosts, under two column orders and two work-list orders**
`[pcc:unverified]`. The second, independently coded closure (route (b) of the task) finds 1 in W_4 for
both T35 instances and both R36 instances, and reproduces the T33 and R35 stall dimension vectors exactly,
which equal the treatise's T33a/b and R35a/b records. The preregistered v-law falsification in
`../RESULT.md` therefore no longer rests on a single tool.

Route (a), an exported certificate, was not built: see "What is blocked".

## The independent tool

| piece | `../fallclose.c` (producer) | this cross-check |
|:--|:--|:--|
| Weil descent | symbolic coefficient formulas, in C | `gen_system.py`: evaluates S(X1,X2) with its own GF(2^n) multiply at 0, e_i, e_i+e_j and recovers ANF coefficients by inclusion-exclusion; checks the ANF against direct evaluation at 400 random points |
| input | instance line | polynomial system file (`sys/*.sys`, SHA-256 in `sys/SHA256SUMS`) |
| linear algebra | M4RI (`mzd_echelonize_m4ri`, `mzd_addmul`) | own code, no M4RI: 8-bit Gray-code tables, 8 groups per pass, OpenMP over rows |
| basis form | reduced row-echelon (back-reduced) | semi-echelon (never back-reduced) |
| column order inside a degree | colex (combinatorial number system) | seeded random permutation (seed 1 local, seed 7 pod), hash-table lookup |
| work list | FIFO | LIFO (local) and FIFO (pod) |
| batch | 8192 | 8192 (local), 4096 (pod) |

Both tools compute the same object: the least subspace of B_{<=4} containing F and closed under
f -> x_i f for f in W ∩ B_{<4}. With a graded column order, the rows of a semi-echelon basis whose
pivot has degree <= e span W ∩ B_{<=e}, so `W_dims` are read from pivot degrees.

**Validation before use** (`out/small_check.jsonl`): all 72 reference rows of `../out/crosscheck.jsonl`
(where fallclose and the pure-Python `lfd_small.closure` agree), each run twice (column seed 0 with FIFO,
seed 7 with LIFO, full fixed point): **144 runs, 0 disagreements** in one_in and, for stalls, in every
cumulative dimension `[pcc:unverified]`. A first local run on cell (iv), T32 seed 3, also falls
(`out/local_T32k16s3.json`).

**Instances.** Every instance line was regenerated from the committed emitters
(`research/lfd_lower_bound_20261111/emit_instance{,_T}.py N SEED K`) and matched the line in
`../out/instances.jsonl` byte for byte (all 7). `emin` solution counts (0 for all) are taken from that file.

## Results (D = 4)

| instance | expected (fallclose) | local, 1 thread, seed 1, LIFO | RunPod, 16 threads, seed 7, FIFO | agree |
|:--|:--|:--|:--|:--|
| T n35 k17 seed 1 | fall | fall; low part (1,35,596,6580), rank 52910 at stop, 73.7 s | fall; (1,35,596,6580,52956), 67.5 s | yes |
| T n35 k17 seed 2 | fall | fall; (1,35,596,6580), rank 52908 at stop, 76.6 s | fall; (1,35,596,6580,52956), 68.9 s | yes |
| R n36 k18 seed 1 | fall | fall; (1,37,667,7807), rank 66596 at stop, 129.4 s | fall; (1,37,667,7807,66712), 152.1 s | yes |
| R n36 k18 seed 2 | fall | fall; (1,37,667,7807), rank 66596 at stop, 123.0 s | fall; (1,37,667,7807,66712), 137.2 s | yes |
| T n33 k17 seed 2 (stall control) | stall (0,0,64,2968,48068), codim 4888 = treatise T33a/b | stall (0,0,64,2968,48068), codim 4888, 100.9 s | identical, 56.5 s | yes |
| R n35 k18 seed 6 (stall control) | stall (0,1,105,3885,61425), codim 5287 = treatise R35a/b | stall (0,1,105,3885,61425), codim 5287, 231.4 s | identical, 116.0 s | yes |

- Generator ranks agree with fallclose: 34 (T35), 36 (R36), 32 (T33), 35 (R35).
- Fall runs stop as soon as 1 enters W, so the degree-4 count depends on the order (LIFO stopped before
  W_4 was full); in every fall run W already contains all of B_{<=3} (6580 = C(34,<=3), 7807 = C(36,<=3)).
- Peak RSS: 0.43 GB (N=34) and 0.66 GB (N=36) locally.
- Raw: `out/local.jsonl`, `out/pod.jsonl`, logs in `out/log/` and `out/pod/`.

## Commands (from this directory)

```
gcc -O3 -march=native -fopenmp -Wall -o /tmp/claude-0/iclose iclose.c          # local binary
OMP_NUM_THREADS=1 python3 small_check.py /tmp/claude-0/iclose > out/small_check.jsonl
# instance lines from the emitters, then the independent descent:
python3 ../../../../lfd_lower_bound_20261111/emit_instance_T.py 35 1 17 > sys/T35k17s1.line   # etc., 7 instances
python3 gen_system.py < sys/T35k17s1.line > sys/T35k17s1.sys
OMP_NUM_THREADS=1 /tmp/claude-0/iclose 4 sys/T32k16s3.sys 8192 0 1 0 > out/local_T32k16s3.json
./run_local.sh /tmp/claude-0/iclose                                            # -> out/local.jsonl
gcc -O3 -march=x86-64-v3 -fopenmp -static -o /tmp/claude-0/iclose_pod iclose.c  # pod binary
RP_STATE=/tmp/claude-0/rp_closure-falls-crosscheck rp.sh create bdr-closure-falls-crosscheck 16 cpu5m 20
rp.sh put ... iclose; rp.sh put ... sys/<inst>.sys (6 files, SHA-256 verified on the pod)
rp.sh exec ... "<run_pod.sh body>"                                              # -> out/pod/exec_8.log
rp.sh delete bdr-closure-falls-crosscheck
```

## Provenance

- Repository HEAD at run time: `2003c4bc85a6db91e74bfe19522a517b44995ff4`; this directory is new and
  untracked, so the source is identified by hash.
- `iclose.c` sha256 `e8183ccdb676bb13746f7c73e64d26af8dfe4b937dc413d195fbdc029dc00bb0`.
- `gen_system.py` sha256 `4a3531dd5d986985fd2b6d37390da3fe3a2ff5463e701a3fe15fecb5f65e94c7`.
- Pod binary sha256 `a6074818ad309f9c4a82db52a418933bc3fcb27bf361b5c66fc67a52dccbfcfe` (verified on the pod by `rp.sh put`).
- System files: `sys/SHA256SUMS`.
- Default behaviour unchanged: no tracked file was modified (`git status` shows only this new
  directory as untracked); `../fallclose.c` was not edited.

## RunPod

| pod id | name | flavor | vCPU | host CPU | created → deleted (UTC) | hours | costPerHr | cost |
|:--|:--|:--|--:|:--|:--|--:|--:|--:|
| 3s0i2d2itxl75h | bdr-closure-falls-crosscheck | cpu5m | 16 | AMD Ryzen Threadripper 7960X | ~21:27:00 → 21:38:33 | ~0.2 | $1.04 | ~$0.21 |

Deleted; a pod listing after deletion shows no `bdr-closure-falls-crosscheck` pod.

## What is blocked / not done

- **Route (a), an exported certificate.** Not built. An explicit expression of 1 needs the transformation
  history of every row (sources are the n generators plus ~10^5 products), which makes the export and its
  check far more expensive than a second closure; a cheaper certificate (the ordered list of low-degree
  members of W) still needs an elimination to check, i.e. it is another closure. Route (b) was chosen.
- **Independence limits.** Both tools were written by agents in this programme and run the same closure
  definition; the instance emitters and `lfd_small`-based reference rows are shared. Nothing is a pcc claim,
  no replay certificate or Iso Lab/EKS run exists, and no bench record was made (this is a correctness
  cross-check, not a benchmark).

## Next step

- If a certificate is still wanted for the record: instrument iclose to log, per low-degree row queued, the
  batch round it entered, and have a third checker re-verify round by round (membership by elimination
  restricted to the rows of earlier rounds).
- File a pcc claim for "1 in W_4 at T35 k17 and R36 k18 (two instances each)" citing `../out/cells.jsonl`,
  `out/local.jsonl` and `out/pod.jsonl`.
