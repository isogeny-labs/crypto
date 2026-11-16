# L7-d5-stalled: degree-5 closure on the stalled n=31 instances

Measurement on toy sizes. No security claim. All numbers come from RESULTS.json (made by src/analyze.py from raw/).

## Answer
**n=31 C1 s=1 (UNSAT): 1 is in W_5.** The closure finished with dim = ncols = 242825, low = 41449, one = 1, CPU 2818 s, wall 5011 s.
N1 and L1 (two independent engines) found 1 not in W_4 for this instance. So d_F = 5 exactly for this
definition of W_d (shared N1 generator, Macaulay closure under multiplication by x_v of the low pivots, x^2 = x).
- L2's run on this instance had hit its 2400 s cap at qh 21215/37979. That record was inconclusive. It was not a stall: completing the run needed about 18% more CPU.
- D1 s=0 at n=31 is SAT (nsol=5, from N1 raw), so 1 is in no W_d for it, and it was not run. Its stalled UNSAT substitute, D1 s=25, already reached 1 in W_5 in L2 (CPU 2393 s; imported unchanged into RESULTS.json).
- n=29 C1 s=0 calibration (SAT): one=0 and dim = ncols - 2 = 174435, as expected for a SAT system. CPU 2677 s.

## Validation
- I copied L2's gf2close5 unchanged (sha256 in src/SHA256).
- I re-ran it against the Python reference (tensor_closure) at d=5 on n=9 and 11, s=0..2, arms A, C1 and D1: 18/18 matched.
- Earlier checks of the same engine: L2 V1 matched 81/81 at n<=13, and the L2 audit matched 11/11 against the non-triangular engine at n=19/21.
- The cap path was not exercised: neither run hit its cap.

## Predictions (PROTOCOL.md, written before the runs)
- P1 (1 in W_5 within the cap): held.
- P2 (calibration gives one=0): held.
- P2 (CPU ratio n29/n31 of 0.3-0.6): failed. The ratio was 0.95, because the SAT calibration has to run its whole queue.

## Caveats
- This is one instance per arm.
- The result is about this W_d definition and generator only. Nothing is said about other n or about the treatise's named instances.
- Resources: 2 single-thread processes, 3.7 GB RAM at N=32. No checkpointing was needed.
