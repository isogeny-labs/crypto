# Exact development controls

Frozen before the first parent compilation on 2026-10-30 UTC. These are
fixed rational-arithmetic correctness controls, with no timings, fitted
exponents, search campaign or prime-field discrete-log instance.

Use C++20 and GMP rationals, strict warnings as errors, a separate output
directory, and the source in `exact_controls.cpp`. No random inputs or
downloaded answers are used.

1. Construct the full Boolean multilinear-extension kernels of the displayed
   two-by-two integer matrices. Their composed `(11,11)` entry must be4,
   whereas the kernel of the matrix product must have entry6. The difference
   is nonzero modulo4 and zero modulo2.
2. Check the genuine exterior composition law on all entries for dimensions
   1 through4 and12 deterministic integer-matrix pairs each. As a positive
   control, check Boolean composition for monomial left factors on the same
   inputs. There are4080 entries in each collection.
3. Independently multiply the seven rational factors defining the source's
   length-four Cauchy chart. Require the one-axis passive coefficient
   `-200/9` for the global-even-parity code witness. This is nonzero and
   rules out disappearance, without claiming that its valuation is exactly
   one or that it refutes every coded kernel.
4. For adjacent-swap exterior kernels with1 through5 swaps, check all1364
   assigned entries against the signed Walsh matrix. Verify the exact
   identity `H H^T=2^n I`, a rational full-rank certificate. This is a
   generic block-local matching witness, not an assertion about monotone
   Cauchy translations or rational rank over a truncated ring.
5. Preserve the one-particle `1 versus0` intermediate-state restriction
   witness. Full composition includes an intermediate dummy mode that
   separate restrictions would remove.
6. For8 fixed four-by-four base matrices, check all70 equal-cardinality
   minors at rank-one update values0,1,2,3. Require affine dependence, giving
   560 checked minors. This supports the finite instance of the written
   general multilinearity proof; it does not establish a faster enumeration.

Retain the first compiler output and exit status, failed attempts if any,
successful raw output, exact compiler/library versions and independent
replays. Freeze successful output only after comparing it to these specified
values. An assertion failure diagnoses this unit and is not a general
mathematical impossibility result. The Lean project checks its explicitly
listed declarations separately; passing native controls does not formalize
the full mathematical review or produce an ECDLP algorithm.
