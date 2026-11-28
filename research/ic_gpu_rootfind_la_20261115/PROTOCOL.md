# m = 3 rootfind on the GPU with incremental-gauss-echelon (protocol, frozen before the run)

## Question
`research/ic_la_m3_20261115` found `incremental-gauss-echelon` doing the same job as `incremental-gauss`
(same trials, relations, logarithm, S) with 0.10–0.15× the row operations, in a local count study.
Round 4 (`research/ic_gpu_rootfind_pairs_20261115`) left linear algebra as most of the GPU arm's solve
at |F_x| = 4096 (1.2–1.5 s of a 1.4–1.7 s solve). What is the whole-solve effect on the GPU arm?

## Arms (`specs/rf3la.json`)
The prime_explicit curves of 38 and 40 bits (research/ic_gpu_rootfind_big_20261115), |F_x| 2048 and
4096, 3 targets each, 2 rounds: `rootfind-subtract:m=3` (CPU) and `rootfind-subtract-gpu:m=3`
(`probe_build=pairs`, daemon, `backend=cuda`), each with `linalg=incremental-gauss` and
`incremental-gauss-echelon`; an A/A copy of the GPU echelon arm at 4096; `rho.negation` and its A/A.

## Predictions
1. Per workload, trials, relations, recovered logarithm and S are identical across all IC arms.
2. At |F_x| = 4096 the GPU arm's median solve wall with echelon is at most 0.5× its wall with gauss.
3. The linear-algebra `row_ops` ratio echelon/gauss is 0.10–0.20 (as in the local study).

## Reading
L0 on a shared pod: a wall difference is reported only beyond the A/A copy's spread. §5a band per arm
against the matched rho. Results compared within this session only.

## Bound
One A100 pod, about 1 GPU-hour, $3. S3: the same waiver as earlier rounds (session AWS keys rejected).
