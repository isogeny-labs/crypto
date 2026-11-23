# C81: fixed startup-amortization ladder

Hypothesis: amortizing process startup over a fixed ladder reduces A/A variability enough to support later total-cost comparisons. Identical C71 reference code runs under both labels; this is not a new solver comparison.

The source, compact input generator, every materialized-input digest, pair count, deadlines and memory limits were frozen before execution. Six complete translated-root regressions and an independent binomial expansion of the full degree12 resultant passed, together with five input-digest checks. Base root completeness is inherited from C78; these translated fixtures are exposed calibration inputs, not independent hard instances.

All30 cold child processes completed and returned equal complete conditional root vectors, candidate shifts and resultant coefficients. Across4686 conditional solves,9372 roots were emitted and checked. Parsing, generation, hashing, extraction, verification, serialization, startup and full child lifetime are included. Each pair compares identical code with the same size/input sequence.

|Inputs/process|Median cold batch seconds|Pair spreads (%)|Failed pairs|
|---|---:|---|---:|
|1|0.032212133|3.815, 8.735, 5.570|0|
|5|0.035187183|8.852, 0.769, 6.868|0|
|25|0.054202753|0.021, 10.536, 0.685|1|
|125|0.156738721|0.347, 1.897, 0.142|0|
|625|0.704980658|14.875, 3.382, 3.457|1|

Two of15 pairs failed the fixed10% limit:25 inputs, repetition1,10.536%;625 inputs, repetition0,14.875%. Thus increasing batch size did not monotonically remove variability on this host. All sizes and failures are retained. No winner, stable size or hardness threshold is selected. Child peak RSS ranged11904–14848KiB; wait4 peaks and post-exec samples are also retained per process, with inherited-pre-exec scope caveat for wait4 and lower-bound scope for sampled RSS. One CPU and4096MiB address-space cap were enforced by the lab; exclusive-host isolation is unproved.

Both lab workers passed execution/correctness contracts; the A/A promotion criterion failed. No qualified algorithm speed ratio follows. The main original-family all-roots search, genuine matched F4/F5/F6 coverage,20hard-instance gate and user50x stretch goal remain open. No release gate was changed.

Next: implement exact semilinear Frobenius composition with coefficient twisting and compare it against sequential quotient squaring, including repeated-root moduli and all setup costs. A separate isolation stream must change its assumption (e.g., controlled host), not retry this same failed ladder to cherry-pick passes. Public novelty is not established: basic Frobenius composition and polynomial squaring are classical. The earlier specialized quintic discriminant already exists in C44 and is not a new discovery.
