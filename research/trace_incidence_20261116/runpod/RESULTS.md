# Native Runpod results — 2026-11-16

The native source-coordinate root finder reduced median CPU time on the supplied
paper's degree10 F83 example from306.75µs (dense) to68.78µs, a4.46× ratio of
medians. Complete degree256 trace translations were6.84× faster on collisions,
10.55–10.59× with exceptional points and12.23× on generic support. The fixed
root input has a supplied datum; validation is charged each call. [pcc:unverified]

These component gains did not remove the incidence cost. At B128, median
complete bounded target work was0.571220s for pullback,0.563907s for the
known-coordinate control and0.007883s for batched pairs. The median paired
pullback/rho wall ratio was1079.96 across60 jointly completed target-round
units (12 public targets, five rounds). The paired pairs/rho ratio was14.49.
These are shared-host descriptive observations for the declared small-field
workload and explicit rho configuration. [pcc:unverified]

| Component, median CPU per call | Time |
| --- | ---: |
| Paper dense, supplied datum | 306.75µs |
| Paper trace, supplied datum | 331.16µs |
| Source-coordinate pullback, supplied datum | 68.78µs |
| Pullback plus complete toy isogeny-datum construction | 405.03µs |
| External FLINT nmod_poly_roots, same polynomial | 10.03µs |

Isogeny construction made this complete fixed-input call slower; the external
randomized FLINT root finder was faster than the curve-based methods on this
input. No global fastest-tool, asymptotic or deterministic/randomized equivalence
claim follows from that comparison. [pcc:unverified]

## Complete target recovery

The public curves and P,Q are the original60 natural fixtures. Fresh native
labelled pools use public alpha*P+beta*Q with rejected draws charged, r approximately
8B³, up tofour attempts, and the same attempt seeds for every arm. Time includes
public input/validation, generation, prior misses, lazy auxiliary construction
and validation, norm/GCD, roots, witness recovery and original-curve scalar checks.
Process startup/output is retained separately. [pcc:unverified]

| B | Pullback wall seconds | Pairs wall seconds | Rho wall seconds | IC completed / target-round units |
| --- | ---: | ---: | ---: | ---: |
| 8 | 0.000662 | 0.000250 | 0.000173 | 50/60 |
| 16 | 0.002331 | 0.001055 | 0.000190 | 55/60 |
| 32 | 0.006812 | 0.000724 | 0.000237 | 55/60 |
| 64 | 0.036481 | 0.001607 | 0.000387 | 55/60 |
| 128 | 0.571220 | 0.007883 | 0.000582 | 60/60 |

IC times in this table include bounded work on censored targets; they are not
all times to successful solves. Each IC arm completed275/300 measured units and
censored25; rho completed300/300. Conditional paired ratios use only jointly
completed units. There are60 distinct public targets, not300 independent targets.
Fields range from4099 through16777259. [pcc:unverified]

The norm retains O(B²) degree-B quotient products. The known-coordinate control
shows that removing auxiliary-root machinery leaves the large-B cost nearly
unchanged. Reducing that norm is the next technical target. Structured factor
bases and random mixed-labelled pools remain separate; the auxiliary curve is
for field-root extraction and supplies no target-DLP labels. [pcc:unverified]

## Execution and evidence

- Sourcec0176f942c03a549a98cb680b816e864b8820333; digest-pinned Debian image,
  cpu5c8vCPU/16GB, one pinned measurement CPU. Shared host, no L2.
- One warmup andfive measured rounds:6066 raw rows,1011 warmup rows retained,
  5055 measured rows, no incomplete JSON tail or failed measurement row.
- Native analysis checked2610 reported scalar consistencies and3630 per-attempt
  existence agreements without disagreement. Repeats are included in those counts.
- Native and rho immutable qualifier pre/post hashes match. Rho was built from
  actual repository source: prime_fast::rho_parallel, one thread, negation,
  table_bits4, walkers8, dp_bits0, max_steps10000000. Optimal tuning and exclusive
  rho phase instrumentation are not asserted.
- B128 pullback parent RSS peaked at6224KiB; maximum auxiliary-child RSS was
  11640KiB. They are separate high-water readings, not a measured concurrent sum.
- The first setup attempt failed before timing and remains in [attempt-1](attempt-1/README.md).
  Both pods were deleted with HTTP204. Estimated compute fees total USD0.07638;
  storage/transfer/API charges and the provider invoice are separate. [pcc:unverified]

[Report PDF](RUNPOD_REPORT.pdf), [native summary](attempt-2/summary.json),
[protocol](PROTOCOL.json), [setup amendment](AMENDMENT_A2.json),
[credit-admission correction](AMENDMENT_A3.json), [raw-format note](ROW_FORMAT.md),
[deployment/custody](attempt-2/custody.json), [cost estimates](costs.json).
The raw second archive is3298986bytes, SHA-256
`0b27d27910807fc9e5b047e4f64999ba71d603178537c78ac7b7f1cb06be81fa`.
All scientific assertions remain unsigned (PCC-20261116-5d145d); no signed replay,
canonical-schema admission, benchmark-index or security-tier promotion.
[pcc:unverified]
