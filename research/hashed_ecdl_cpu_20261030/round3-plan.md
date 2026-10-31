# Further 10x CPU complete-recovery objective

The user requested continued iteration until a further 10x improvement. The
baseline is the optimized CPU implementation at PR #2933 commit
`2bd4588fba603ba1787e1d8ad82f084ef461d7cf`, not the original Enumerate arm.
The goal remains active and unverified. Prior panel records remain immutable.

## Current evidence and candidate

The final prior panel attributes about 1.18 ms to curve/target construction,
1.68–1.74 ms to other reduction work and 0.45–0.51 ms to the separate replay
on the two 16-bit fixtures. These stage observations motivate work beyond the
MITM query loop; they are not measurements of the new code.

The first candidate adds three changes to the shared checked reduction:

- Extended-Euclid inversion replaces Fermat exponentiation for validated
  nonzero field, scalar and hash residues. Miller–Rabin and square-root
  exponentiation remain unchanged.
- Finite differences generate the tiny constructor's square-root table and
  cubic RHS values without repeated division. It still counts the full curve
  order and produces exactly the same canonical roots.
- Radix-16 fixed-base tables serve repeated `[u]P + [v]Q` calculations.
  Each solve and each certificate replay constructs its own tables. All
  setup remains inside the complete interval and every relation is checked.

The current 24-test release suite passes, as does strict all-target Clippy.
New controls compare inverses against Fermat, constructor tables against the
old exhaustive formula, fixed-base multiplication against binary multiplication
on small and 40-bit curves, and all three full certificates against archived
pre-change certificate bytes. These are software checks, not timing evidence.

## Literature checked before implementation

- Menezes, van Oorschot and Vanstone, Handbook of Applied Cryptography,
  [Chapter 2, Algorithms 2.107 and 2.142](https://cacr.uwaterloo.ca/hac/about/chap2.pdf):
  extended Euclid and modular inverses. The implementation uses signed i128
  coefficient intermediates for moduli below 2^62.
- Bernstein and Lange, [Analysis and optimization of elliptic-curve
  single-scalar multiplication](https://www.hyperelliptic.org/EFD/precomp.pdf):
  setup, inversion/multiplication tradeoffs and precomputation accounting.
  Our reusable radix tables are standard fixed-base precomputation, not a
  novelty or asymptotic claim.
- Existing reduction, CPU oracle, prior frozen protocol, stage summaries and
  archived certificates in this study.

## Next experiment and completion requirements

Before measuring, freeze a new protocol and native analysis script. Compare
the pinned optimized baseline and candidate on the same host, compiler flags,
thread count, inputs, seeds, certificates and complete timing interval.
Retain all three prior fixtures, A/A repeats and corruption controls. Include
constructor and scalar-table setup, failed trials, full solve, both certificate
checks and serialization. Do not omit replay or change inputs to meet 10x.
Use an L0 screen only to guide further iteration; an admitted performance
claim additionally needs the required isolation and independent review.

The original 20–40-bit ladder also remains outstanding. Its previous job
expired without executing; a fresh attempt must have a distinct identity and
preserve the earlier failure. A sub-millisecond small-fixture result cannot
establish the largest feasible CPU or GPU scale.

No new timing experiment has run for this candidate. No 10x improvement is
claimed. Stop conditions, resource caps and a complete matched protocol must
be frozen before execution; never reuse an old protocol for changed binaries.

## Handoff

Task T-892 owns the CPU crate, this study, the CPU handoff and the shared
`src/cryptanalysis/hashed_ecdl.rs` file. The shared file was unreserved at
admission. Continue on the existing CPU PR; keep the unchanged parent GPU
implementation separate. The previous goal turn only verified PR status;
this turn makes implementation and correctness-test progress.
