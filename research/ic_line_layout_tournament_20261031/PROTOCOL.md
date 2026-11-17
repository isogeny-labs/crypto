# Prime IC line-layout tournament

Status: frozen before candidate data. The machine-readable authority is
`protocol.json`, protocol SHA-256
`a770471ff7aeac2c69f0292934a1d85fdf00ed66bc752741c2d2cb6a4b9895e6`.

## Claim boundary

This is a complete cold `ic.pipeline` comparison on a fresh 20-bit prime-order
curve. The decisive clock includes factor-base construction, oracle setup,
relation collection, linear algebra, logarithm recovery, and verification. A
relation-phase or setup-only improvement cannot pass. The candidate must retain
the incumbent's exact complete GAE on every pair.

The result, if admitted, is a native constant-factor engineering improvement in
the generic track on the measured AMD `c8a.2xlarge` class. It is not an
asymptotic result, not algebraic-track progress, and not a claim to beat matched
Pollard rho.

## Fixed design

- Incumbent: `prime-abscissa:size=45` with
  `mitm:negation_folded=1`.
- Candidate: the same recipe with `layout=lines` only.
- Controls: identical incumbent A/A; `rho.plain` versus `rho.negation` as a
  positive timing control; matched `rho.negation` retained as the ECDLP
  reference.
- Workloads: one fresh public fp20 curve, eight fresh public targets, eight
  measured rounds plus one warmup.
- Replication: two concurrent non-resumable EKS Jobs on distinct L2 nodes.

Both jobs must independently pass the exact rule in `protocol.json` and the
frozen `analyze.sh`. No pooling or favorable-host selection is allowed.

## Evidence handling

The GitHub EKS route publishes only the source-bound success receipt; complete
measurements remain in private S3 by design. Job success is not the performance
verdict. The authenticated session directories must be downloaded, hash-checked,
exported as `experiment.export/v1`, and passed to `analyze.sh` before the report
can say that a speedup was achieved.
