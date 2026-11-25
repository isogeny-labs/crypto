# Frozen EC graph and coefficient rank check (#35–39)

**Status:** prepared, not executed. Launch waits for the parent to confirm the S3 checkpoint gate passes. Source: [`EC_GRAPH_RANK_GP.gp`](EC_GRAPH_RANK_GP.gp). Conductor task `T-12177` reserved these two paths at preparation time.

## Question and controls

Can a graph cycle or a topology derived solely from relation support determine the rank of a prime-field **elliptic-curve** index-calculus relation matrix? The support-only control is the identical triangle graph in both arms. The coefficient-aware baseline is exact rank over the subgroup order, with every row independently checked in the curve group.

Public synthetic curve: `E/F_1009: y²=x³+x+14`, group order `r=1013`, generator candidate `P=(0,425)`. The script checks the group cardinality, primality, and point equation before evaluating rows. Factor-base points are `F1=P`, `F2=2P`, `F3=3P`. They are generated from known logs to make the row witnesses inspectable; this is a rank-mechanism test, not a relation-collection benchmark.

The frozen matrices and right-hand sides are

```text
A = [1 1  0]   b_A = [ 3]
    [0 1  1]         [ 5]
    [1 0  1]         [ 4]

B = [1 1  0]   b_B = [ 3]
    [0 1  1]         [ 5]
    [1 0 -1]         [-2]
```

Each row asserts `Σ_j C_ij F_j = b_i P`; the GP source checks all six group equations. The last row sign is the only changed coefficient, so the support graph and its single cycle are the same. `det(A)=2` is nonzero modulo 1013, while `det(B)=0` and its first two rows have a unit 2×2 minor. The prediction is ranks **3 and 2**. A support-only rank rule necessarily gives the same answer for both and fails on one; the exact modular baseline distinguishes them.

## Launch and acceptance

After the checkpoint gate passes, run only:

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_GRAPH_RANK_GP.gp
```

Capture stdout and stderr separately in a new, scope-checked raw-output file. Reject any `***` GP error line or nonzero exit code. Require the printed curve/order, six checked rows, equal support, and the two rank lines exactly as stated. Do not select another curve, coefficient, or control after viewing output. Budget: one local GP process, under 60 seconds and 100 MiB expected; stop rather than retry automatically if it exceeds either limit. No external target or paid resource is involved.

The check can falsify the literal topology-only rank claim. It cannot establish relation yield, independent-rank accumulation rate, a successful target descent, or a complete-solver cost advantage. An affirmative path for ideas #35–39 would require a coefficient-aware topology statistic that predicts a useful outcome on held-out, independently collected EC relations, followed by a full-cost comparison with the existing large-prime filter or exact rank baseline.
