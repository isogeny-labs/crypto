# CPU residual reciprocal candidate

The batched target-minus-prefix kernel still used integer remainder for each
field product. Reuse one ceiling reciprocal per batch for moduli below 2^32,
with the same proved correction bound documented in reciprocal-candidate.md.
Wide moduli retain independent u128 remainder. The inverse-point branch tests
whether the canonical coordinate sum is zero or p instead of dividing by p.
Batch sizes, exceptional points, prefix order, witness replay and memory counters
are unchanged. Setup remains charged, including reciprocal construction.

Literature: [HAC Chapter14 section14.3.3](https://cacr.uwaterloo.ca/hac/about/chap14.pdf),
previously read for the same reciprocal method; this is not a new arithmetic
method. The shared module is reserved by T-906. A small private helper in the
standalone CPU crate avoids changing that owner's interface; it can be unified
when the shared scope is available.

All 29 release tests and strict Clippy pass. New independent u128 remainder
checks cover all residue pairs for moduli2..127, larger boundary/composite
moduli and operands near the modulus. Existing exhaustive group-pair batch
subtraction, wide-field witnesses and complete frozen certificates pass.
The next frozen panel retains baseline2bd4588fba60, all three fixtures and the
complete setup/replay/serialization interval. No timing follows from tests.
