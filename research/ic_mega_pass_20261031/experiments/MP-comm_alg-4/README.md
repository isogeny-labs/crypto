# MP-comm_alg-4: block law vs count law for the chained m=3 descent. Outcome: REFUTES (toy scope)

Run: `python3 run.py` (msolve 0.9.5, python 3.9.10). Raw data: raw_output.jsonl; summary: RESULTS.json; pre-registration: PROTOCOL.md.
Setup: F_{2^13}, 26 Boolean equations from S3(x1,x2,y), S3(y,x3,xR). Blocks are random subspaces. 4 draws per cell.
Metric: the maximum F4 round degree from msolve (median over draws). Every chain instance was unsat except one at (4,4,4,13).

Median chain D:
- C2 (same minimum block, N grows): l=3 gives 2, 3, 4 at N=12, 15, 22; l=4 gives 3, 3, 4 at N=16, 19, 25. Both contrasts rise with N, which is the count-law direction.
- C1 (same N, different minimum block): at N=14, (3,3,3,5)=2.5, (4,4,4,2)=2 and (2,4,4,4)=3, a mixed result. At N=18, (4,4,4,6)=3 and (5,5,5,3)=3, a tie.
- y-series l=3, dy=13,6,5,3,2: 4, 3, 2.5, 2, 2. The degree already drops at dy=l+3, which matches prediction_if_false.
- y-series l=4, dy=13,7,6,4,3: 4, 3, 3, 3, 2.
- Null arm (random_matched, run only for N<=19): null D is 3 to 5 and also rises with N. Chain D is below null D in every cell (Delta -1 to -2).
  The null timed out at 60 s in 7 runs at N=18-19, and was not run for N>=20 because the pilot timed out.
- Exact XL refutation degree D* (N<=14): the chain gives 2-3 (one outlier of 5 at (2,4,4,4) seed 11000) and the null gives 4.
  This matches the msolve ordering.

Under the pre-registered rule, the count law wins. C2 rises in 2/2 contrasts and C1 gives a tie and a mixed result.
The minimum-block law is not supported: at fixed field and equation count, the degree tracks total unknowns and over-determination.

Caveats:
- The regime is heavily over-determined (26 equations, N<=25), so D only spans 2-4 and the resolution is coarse.
- The msolve step degree is used only for comparisons.
- Blocks are independent random subspaces, not one shared factor base V.
- Only m=3 and n=13 were tested.
- None of this supports or refutes any growth law at n=131. The class is theory/accounting, and B*D^2 is unchanged.

Next step at larger size: n=17-19 with N close to 2n, so the degree has room to separate the two laws. Use l=5,6, a faster null (exact D* via M4RI or the prebuilt macaulay_dense), and a closure-degree check, not the F4 step degree.
