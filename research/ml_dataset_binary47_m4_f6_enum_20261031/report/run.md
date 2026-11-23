# Ordinary binary n47 m4 F6 and enumeration: run report

The four frozen native cells have terminal restricted S3 custody. Both F6
geometric four-sum cells and the untwisted bounded-enumeration cell reached
native `verified` status. Twisted enumeration reached the frozen 30-second
native timeout after retaining a verified transition artifact. Each method pair
has byte-identical public input within its twist; the opposite twists differ.
No cell promoted an independently recovered ECDLP scalar.

| Method | Untwisted | Twisted | Scalar labels |
|---|---|---|---:|
| F6 geometric four-sum | verified | verified | 0 |
| Bounded enumeration | verified | timeout | 0 |

The [official export](../experiment-export.json) passed the pinned checker
with zero findings: four planned and attempted, three completed, one
not completed because of timeout, zero exclusions or resumes. The
[independent audit](../AUDIT.json) replayed all four local restricted archive
hashes and 128 native artifact hashes. It checked method/twist/instance
identities, shared public-input digests, the exact Hasse point-count and
quadratic-twist certificate types, and the ordinary-binary model. The public
record has field degree 47 over F2, coefficient-field degree 47, and
`frobenius_nontrivial_on_model=false`. The trace-one twist changes `a` by XOR
with 1 and retains `b`; the actual curve and group orders are in the public
records. The [outcome chart](outcomes.svg) plots all four native statuses.

[Producer checkpoints](../checkpoints/) bind each restricted archive and
pointer to a content-addressed S3 PUT and fresh GET/hash comparison. A
[separate fresh readback](../S3-READBACK-SAMPLE.json) checked archive indices
0 and 3. The four archives total 1,015,585 bytes. The protocol and plan were
committed before data at `d1eebc768e3e`; the protocol file SHA-256 is
`e3888fe123a49692f56784611814425080a9d694e38574f09e15f5d9e9f47d9f`.
The pinned clean native source is `c41bec150e16e4c4f9a00634002b633b821de262`
and binary SHA-256 is
`b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e`.

[Cost observations](../cost-observations.jsonl) retain 16 phase-telemetry
records. The four solver phases have raw calling-thread instruction counts,
but none verifies whole-worker coverage or qualifies as a complete
instruction-to-solve training label. The timed-out enumeration artifact is a
censored bounded trace; it does not prove no decomposition. F6's private
lookup sequence remains a declared trace gap. Shared-host wrapper clocks are
diagnostic, so this fixed pilot makes no timing-effect comparison.

The protocol followed the repository's native capability and construction
contracts. For the algebraic context, [Shantz and Teske's characteristic-two
study](https://eprint.iacr.org/2013/596.pdf) distinguishes experimental
Weil-descent/Gröbner behavior from proved complexity. These four cells
measure the native geometric and bounded enumeration adapters only.
