# Complete native wiring and paired recovery controls

The native runner now wires four stages together: canonical coverage tracking,
codomain moment assembly, complete point evaluation, and known-answer ECDLP
recovery on both endpoints. Complete ordinary-curve fixtures pass independent
PARI comparison. These fixtures validate software, not the requested 80-bit
conductor crossing or its performance.

| Control | Field p | Isogeny degree | Subgroup q | Compared point inputs | Paired scalars |
|:--|--:|--:|--:|--:|--:|
| Small ordinary curve | 71 | 19 | 3 | 6 | 2 |
| Larger ordinary control | 6,666,679 | 10,009 | 37 | 36 | 32 |

The source coefficients are `[0,0,0,5,5]`. Destination short coefficients
are `[52,17]` and `[5344836,3950273]` respectively. All 42 evaluated inputs
match PARI's independent `ellisogeny` / `ellisogenyapply`; counts include
repeated base inputs and six kernel/infinity controls. The runner checks
ordinary trace, exact kernel order, equal endpoint group order, nonzero
transported generator, prime-subgroup preservation and phi([s]G)=[s]phi(G).
Both endpoint logarithms recover each of 34 known answers: 68 recoveries,
with raw scalar/target receipts retained. The subgroups are tiny and no
performance or security conclusion follows from them.

## Execution and evidence

```sh
sh native/build.sh
native/build/complete_pipeline --fixtures
native/build/complete_pipeline --target80-check
```

The last command deliberately returns 3 and emits missing values as null.
It is a check of the fixed current prefix inventory (4,096 baby roots),
not a general S3 inventory crawler or a failed full-construction run.
`target80-gate.json` and its exit receipt preserve the actual outcome.

`fixture-receipts.jsonl` retains six negative coverage controls, each known
scalar and both answers/targets, and the two final successful summaries.
`checks.json` pins source/binary/raw hashes and the derived counts.
`pre-scalar-detail-receipts.jsonl` retains the earlier successful aggregate
run. `initial-subgroup-assumption.*` preserves an unsuccessful development
control: q=2 was incorrectly assumed for p=71. Independent PARI found
57=3*19, so the input was corrected to q=3 and the complete checks replayed.

This was native development validation. No construction/evaluation/DLP
runtime measurements were taken, and no EKS performance job was launched.
`runtime_measurements` is null. The earlier request for an index-calculus
arm remains unimplemented for this prime-field workload; the controls use
PARI `elllog` only and do not claim a cross-method benchmark.

## Coverage and formulas

The canonical controller accounts for every baby/giant rectangle exactly
once and every leftover odd kernel index. It rejects incomplete coverage,
gaps, duplicates, wrong rectangle extents, missing leftovers and incorrect
leftover ordinals. Codomain and image assembly occur only after completion.
The fixture half-kernel counts are 9 and 5,004, fully covered.

The runner uses bounded native polynomial jets for the existing square-root
Vélu biquadratic, calls the checked native log-norm helper, and accumulates
half-kernel moments and point logarithmic derivatives. Leftovers are handled
directly. It then applies the existing normalized Vélu quotient and point
formulas. Kernel collisions are detected by a polynomial gcd; infinity and
kernel inputs map to infinity. Ordinary non-kernel inputs are verified on
the destination and against PARI. These implement the existing mathematical
contracts; no new algebraic theorem or formal proof is claimed.

Sources: archived `blocked_velu.py` full-map contract and Sage 10.9
square-root Vélu source; the related CM study's `CONSTRUCTOR.md`; FLINT
polynomial APIs; PARI elliptic-isogeny and elliptic-logarithm APIs. Native
block traversal uses arbitrary-size degree/count arithmetic and bounded
polynomial tiles; no floating-point square root selects the window.

## What remains for the requested target

The 80-bit degree is unchanged: `729845429276318409687041`.
It needs 427,154,957,034 baby roots and the full corresponding giant/leftover
schedule; the fixed stored prefix is not complete. Its explicit quotient,
full point images and matched endpoint timings remain unavailable. Isogeny
degree in the fixture table is not an endomorphism-conductor gap.

The complete-fixture runner regenerates its bounded blocks from the kernel.
The production S3 manifest-bound, resumable coverage controller is **not**
implemented; stored-block validation and durable restart still need to be
integrated with these complete formulas. The code must not silently certify
an arbitrary stored prefix as complete. Full-work feasibility and setup/I/O/
verification costs are also unestablished. The unchanged
`MEASUREMENT_PROTOCOL.md` governs any future EKS timing run.

See the [report](../reports/complete-pipeline.pdf) and
[existing problem record](../../../problems/conductor-gap-upper-floor-and-cost.md).
