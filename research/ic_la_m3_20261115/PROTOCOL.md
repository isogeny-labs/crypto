# Linear algebra for m = 3 rootfind-subtract at |F_x| = 2048 and 4096 (protocol, frozen before the run)

## Question
Round 4 of the GPU rootfind work (`research/ic_gpu_rootfind_pairs_20261115`) left linear algebra as the
largest part of an m = 3 solve at |F_x| = 4096: about 316M `row_ops` (unpriced in S, priced in the
§5a band at 0.1 and 1 GAE per operation, where it dominates) and 0.5–1.5 s of wall. Do the pipeline's
existing sparse incremental solvers do the same job with fewer `row_ops`?

## Sources checked
- research-kb: not reachable in this container (no `crypto-autoresearcher` checkout, no catalog);
  recorded as a gap. Repository notes instead:
- `research/ic_gauss_echelon_20261027/README.md`: `incremental-gauss-echelon` gives the same pins,
  rank and logarithm as `incremental-gauss` with 0.59–0.61× its `row_ops` at |F| = 30–42.
- `research/ic_linalg_pipeline_20261019/README.md`: `sparse-cado` costs about 1.5× dense `row_ops`
  but runs 2–4× faster; it needs the full-rank completion rule (changes trials and S), so it is out of
  scope here.

## Arms (spec `specs/la.json`)
`rootfind-subtract:m=3`, prime-abscissa base of 2048 or 4096 abscissae, `walk-prime-batch-1024`
targets, on the prime_explicit curves of 34, 36 and 38 bits, 2 targets each, 1 round, with:
- `incremental-gauss` (baseline, the default every earlier round used);
- `incremental-gauss-echelon` (sparse forward elimination);
- `structured-gauss` (sparse reduced echelon, lightest-column pivot);
- `rho.negation` as the matched reference.

## Predictions
1. Per workload, trials, relations, recovered logarithm and S are identical across the three solvers.
   Any difference voids that comparison.
2. `incremental-gauss-echelon` uses at most 0.7× the `row_ops` of `incremental-gauss` at |F_x| = 4096.
3. `structured-gauss`: no prediction (no prior measurement at these sizes).

## Measurement and reading
`row_ops` are deterministic counts, valid at L0 (this container, `--cpus none --allow-busy`). LA wall is
reported as indicative only. The §5a band is computed per arm at w = 0, 0.1 and 1 GAE per unpriced
native operation against the matched rho. A solver is reported better only where its `row_ops` are
lower on every workload and the per-workload identity check holds.

## Not in scope
The full-rank sparse solvers (`sparse-cado`, Wiedemann, Lanczos); the GPU arms (the linear algebra is
the same code on every arm); any wall-time claim.
