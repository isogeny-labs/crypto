# Source-block regression: complete physical sets, local rank comparison

`regression-v2` resumes only the three source-block unions whose original
launchers lost their process owners during recovery. It uses the frozen
`950e15f38c4a2646a14518ee89cc672687ac2d6a` runner copied from v1. The six
terminal global attempts were not repeated. All three resumed solves and
their independent audits have exit zero; `complete.txt` is retained.

![Actual local relation counts and independent ranks](../../figures/source-block-rank.svg)

Each union covers all allowed source values in three disjoint blocks
`U^12=T`, with `T=1,10,26`, on the predeclared positive
p37/seed0/query2 fixture. Each block reports complete roots within the
unchanged per-query budget. The verifier compares the exact signed
physical sets with the earlier independent group/BSGS oracle, checks
actual group sums and independently reduces the folded matrix.

| Arm | Distinct signed physical relations | Independent rank / columns | Signed-set audit |
|---|---:|---:|---|
| Pure L0, partial symmetry | 1 | 1 / 3 | pass, no missing or extra tuple |
| Phi image | 2 | 2 / 3 | pass, no missing or extra tuple |
| Doubling image | 1 | 1 / 3 | pass, no missing or extra tuple |

All numbers above are copied from the native
[retained audits](../../results/source-block-regression/). A distinct row
count is not substituted for independent rank. The phi relations occur
in the `T=10` block; the other two blocks are still fully solved and
charged in the actual execution. No arm has full column rank from this
single residual, so none constitutes a complete DLP run.

This establishes a local rank advantage for phi on one selected positive
query. It does not establish a uniform-residual yield advantage, a larger
prime result, a free twelvefold symmetry quotient, or an end-to-end
speedup. All 21 source partitions and map coordinates were checked
natively; full group-action equivalence of the partition remains open.

The run preserves one thread, 8 GiB address space, 1,180 s cooperative
and 1,200 s outer limits, and the busy reservation. Its component durations
are unadmitted diagnostics. They are not compared as a speed ranking.
The stronger pure four-factor symmetry control and mixed inverse-variable
controls have separate runs and must remain in any subsequent pricing.
