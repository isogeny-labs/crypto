# Two-dimensional affine-plane control — 2026-11-23

Producer-only [pcc:unverified]. Semaev constraint only; no changes to L. Follow-up to VERONESE_FINDINGS.md.

## Rank-shared quadratic lift

For fixed original affine forms x_i = c_i t + d_i u + e_i, replace all separate x_i^2 auxiliaries by three shared coordinates T=t^2, U=tu, W=u^2. Substitute x_i^2 = c_i^2 T + 2c_i d_i U + d_i^2 W + 2c_i e_i t + 2d_i e_i u + e_i^2. This leaves each projective S3 tensor cubic and all definitions quadratic. More generally, for d free parameters choose a basis of the span of the homogeneous quadratic forms (c_i dot t)^2; its rank r is at most min(number of nonconstant leaves, d(d+1)/2). Only r lift variables and equations are necessary. For a fixed target leaf, fold its square as a constant in a fair baseline. Thus the potential savings for m nonconstant leaves are m-r, not m+1-r. The transformation is definitional and preserves projected sets, not an unconditional complexity advantage. Balanced tree uses identical local charts, pairing leaves (0,1) and (2,3) for m=4; all internal points are projective and geometric, and path signs glue on a tree. Preinsert (1-z)(r-1) at each chart.

## Executed local diagnostic

SymPy 1.14.0, Python exact Fp arithmetic, seed 2026112302. 50 rank-two affine planes per cell, target fixed within each plane. Fixed-degree numeric Sylvester resultant with interpolation for S5 versus independent projective Fp2 root-set propagation on sequential and balanced trees: 100/100 plane sets, 3700 input decisions, 788 positive tuples. This is a projected-set oracle gate, not 50 GB runs; the oracle is deliberately exhaustive in the small field. Shared definitions and local closure are algebraic identities. p=5,m=3 and p=7,m=4. After gates, first plane of each cell, common t^p-t,u^p-u, exact lex Buchberger, 3-second subprocess cap including generation, solve, projected root extraction. Completed GB results all agreed with the oracle. No F4, no solving-degree claim.

```
GATE 5 3 50 1250 230
GATE 7 4 50 2450 558
BENCH 5 3 0 mono ok 0.049652963000028194 114512 2 1 8 4
BENCH 5 3 0 seq ok 0.10875417700003709 114776 9 9 3 4
BENCH 5 3 0 seq_shared_closed ok 0.11922876599999199 115032 8 9 3 4
BENCH 5 3 0 balanced_shared_closed ok 0.13484062600002744 115036 8 9 3 4
BENCH 7 4 0 mono timeout None None None None None 23
BENCH 7 4 0 seq timeout None None None None None 23
BENCH 7 4 0 seq_shared_closed timeout None None None None None 23
BENCH 7 4 0 balanced_shared_closed timeout None None None None None 23
```

BENCH columns p,m,case,arm,status,wall_seconds,peak_process_RSS_KiB,variables,generator_slots_without_field_equations,maximum_raw_degree_without_field_equations,expected_projected_roots. Single sample only; timeout is censored. For m=3 both tree shapes are the same. No performance advantage demonstrated. An initial run evaluating a symbolic resultant on all 50 planes timed out during m=4; the final gate instead used fixed-degree numeric Sylvester interpolation. Sage/Singular/msolve unavailable. Local harness retained in conversation artifact, not claimed present on branch. Next: split generation from solving, compare constant-folded baseline, exact instrumented F4 and matched term order; 50 actual projected GB gates if feasible. No new first-fall or F4 data. Keep draft; do not merge.

Prior-art scope: existing PR #5087; Kosters–Yeo https://arxiv.org/abs/1503.08001 warns about first-fall heuristics; Kousidis–Wiemers https://arxiv.org/abs/1906.05594 concerns Weil descent, not this plane specialization. KB and conductor CLI unavailable in this runtime; no novelty priority claim.
