# Koblitz47 m4 F6 and enumeration: run report

All 16 predeclared native cells at `n=47,m=4` have terminal custody. The fixed
population crosses F6 and enumeration, both twists, two generators, and two
keys per generator. Every cell produced a verified method artifact; none
promoted an independently recovered ECDLP scalar. The official
[16-row export](../experiment-export.json) passed the pinned checker with zero
findings and has 16 planned, 16 attempted, 16 completed, and zero exclusions.

| Method | Untwisted verified | Twisted verified | Scalar labels |
|---|---:|---:|---:|
| F6 geometric four-sum | 4 | 4 | 0 |
| Bounded enumeration | 4 | 4 | 0 |

The [independent audit](../AUDIT.json) replayed all 16 restricted archive
hashes, 512 native artifact hashes, method/twist/generator/key identities, and
the Koblitz field metadata. Each public record has degree 47 over F2, a
trace-one twist parameter of 1, coefficient-field degree 1, and a registered
Frobenius action. These metadata establish the model identity of this fixed
population; relation transport and rank would require separate evidence.

The [producer checkpoints](../checkpoints/) record S3 archive and pointer
PUT/fresh GET byte matches for every cell. An [independent fresh readback
sample](../S3-READBACK-SAMPLE.json) checked archives 0 and 15. The protocol
and plan were committed before data in `9d827a90bb44`; the pinned controller
was committed in `a8914c7a6628`. Native source is
`c41bec150e16e4c4f9a00634002b633b821de262` and binary SHA-256 is
`b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e`.

The [cost observations](../cost-observations.jsonl) retain all 64 phase
telemetry records, each bound to its cell and native artifact hash. All 16
solver phases have a raw calling-thread instruction observation. None has
verified whole-worker coverage or `training_cost_eligible=true`, so this run
provides no complete instruction-to-solve label or controlled timing
comparison. F6's saved pair table is an independently reconstructed bounded
domain; its private lookup sequence is a declared trace gap. Enumeration's
finite work limit does not prove a full-domain negative. See the
[outcome chart](outcomes.svg) and [PDF report](run.pdf).
