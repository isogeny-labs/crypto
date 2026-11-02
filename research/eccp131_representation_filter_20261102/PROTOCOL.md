# ECCp-131 representation-filter sweep

## Scope

This prospective experiment runs on the exact Certicom ECCp-131 prime-field
curve.  It asks whether any simple coordinate-visible tag supplies the missing
Wagner property for four-summand relation search: enrichment of true
factor-base relations over an independent null, with a held-out effect that
survives a matched random-base control and a path to enumerating qualifying
pairs without first computing every elliptic-curve pair sum.

It is not a claim of key recovery.  The published target is used only by a
bounded exact meet-in-the-middle null probe.  The known-answer target is a
four-summand point constructed from the factor base and must be recovered by
the same root path.

## Requirements and evidence

| Requested item | Prospective evidence |
| --- | --- |
| Exact “ECC p 131” | `CurveParams::eccp131()`, subgroup/order checks, canonical curve digest in every result |
| Hyperparameter search | Exhaustive native-Rust sweep over tag source, combiner, bit width, and bit offset on the training cohort only |
| Baseline end to end | Exact pair-table construction, planted root recovery, published-target probe, relation/rank probe, phase/resource ledger |
| Relation finding | Small-x factor base at `B = 64, 128, 256`; exact four-sum planted and independent-null cohorts |
| Relation solving | Exact scalar-derived relation/rank run through `ic prime`; zero-row outcomes remain evidence; a planted modular-solver control checks the solver path |
| Root finding | Exact signed pair-table MITM positive control plus the unchanged published target |
| F4/F5/F6 | Each named separately in the phase ledger. F6 means the repository's exact geometric pair index. No unavailable exact-width F4/F5 backend may be relabelled as measured. |
| Reproducibility | Frozen `experiment.protocol/v1`, explicit seeds, raw JSON, unified export, SHA-256 inventory, SVG visual, exact command and source revision |

## Candidate family

For each signed split `R = (P1 + P2) + (P3 + P4)`, tags are drawn from:

1. affine `x(Pi + Pj)`;
2. affine `y(Pi + Pj)`;
3. canonical-field `x(Pi) + x(Pj)`;
4. canonical-bitwise `x(Pi) xor x(Pj)`;
5. canonical-field `x(Pi) * x(Pj)`.

The first two require the pair group operation and therefore fail the
subquadratic-enumeration condition unless the data show an additional way to
generate the bucket.  The last three can reject before a group addition and
are the non-generic-coordinate candidates.

For widths `w = 1..8`, offsets `0, 8, 16, …` plus the final legal high-bit
offset `131-w` are swept.  The two
pair tags are combined by XOR, addition modulo `2^w`, either-direction
subtraction modulo `2^w`, or equality.  Candidate output is compared with the
same slice of `x(R)` except for equality.  Jeffreys-smoothed log2 enrichment is
used only to rank candidates.  Candidates with fewer than 32 training null
hits are ineligible.  The selected configuration is evaluated once on the
held-out cohort.

## Controls and decision rule

- Positive: exact planted four-sum root recovery must pass.
- Negative: pair features are matched to a target from an independent sample.
- Random-base: the same sweep and held-out evaluation is repeated on distinct
  scalar-derived points of the same exact subgroup.
- A/A: the fixed cohort is evaluated twice by the same winner; bit-identical
  counts are required. Timing is descriptive locally and admits no speed claim.

The representation-filter lead is supported only if the held-out lower
confidence bound for enrichment exceeds 1, the matched random-base comparison
does not explain the effect, and the candidate has a concrete
`O(B + output)` or otherwise subquadratic qualifying-pair enumerator.  A
post-add coordinate tag cannot pass the last condition.  Otherwise the verdict
is `no_qualifying_filter_in_frozen_family`; this does not rule out other
coordinate predicates.

## Frozen run shape

- Factor-base sizes: 64, 128, 256 abscissae.
- Training: 4,096 true and 4,096 null samples per base design and size.
- Held-out: 8,192 true and 8,192 null samples.
- Seeds: fixed by `protocol.json` before execution.
- MITM root probe: `B = 128`, one planted target and the published ECCp-131
  target.
- Exact relation/rank probe: `B = 256`, arities 2 and 3, 256 setup and 256
  target candidates per arity.
- Fixed-N stopping; no favourable-seed reruns.

The preferred execution backend is Iso Lab EKS Spot. New cloud execution is
excluded while the USD 25 daily cost gate has unknown or exhausted headroom.
Local runs are development evidence with invalid clocks; counted results and
exact checks remain usable. The frozen cloud workload is not reduced to fit.

