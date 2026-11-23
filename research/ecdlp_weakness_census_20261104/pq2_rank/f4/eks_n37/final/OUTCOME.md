# n = 37 msolve on isolab-eks: outcome

- Job `pq2-msolve-n37-e63bf8e5`, UID 334e45a5-4da2-4993-a97a-60b00603e205, pod d3d86828-dde3-4ced-9584-0b5c35aa8d9a.
- Created 04:45:29 UTC; the pod waited on `cpu-spot` NodePool limits until about 05:46 and msolve started about 06:42.
- Stopped by Kubernetes `DeadlineExceeded` at 10:45:29 UTC: `activeDeadlineSeconds` (21600 s) counts from Job start, so the
  queueing time consumed two of the six hours. msolve ran about 4 hours.
- msolve steps: degree 2, 2, 3, 3, 4, 4, 4, 4, then a degree-5 step on a 1,016,044 × 610,665 matrix, started about 07:42 and
  still in linear algebra at the last upload (10:42). Memory 53 GB (VmHWM), flat from 07:42; about 71 GB still available.
- No `exit.json` and an empty `gb.out`: the run did not finish, so the reduced-basis check (linear = k = 33) is not available.
- Established: msolve's maximal step degree on this instance is at least 5 (F4 selects the lowest-degree pairs first).
- Not established: msolve's final maximal step degree; a lower bound for every F4 run (would need the degree-4 closure).
- Cost: one 128 GiB / 8+ vCPU Spot node for about 5 hours (05:46–10:46); at typical memory-optimised Spot prices that is about
  USD 2, within the USD 5 ceiling. Estimate, not a billing readback.
- A rerun to completion needs a deadline that excludes queueing (a deadline of 10–12 h, or submitting when the pool is idle).
