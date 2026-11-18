# Owned Euclidean row updates in half-GCD

T-1833 · 2026-11-07. Derived arithmetic/ownership transformation with native
producer checks; no timing measurement or independent signed theorem.

## Sources, domain and scope

Read predecessor feature-native-singleton-resultant/polynomial.rs and the
source-accounting ledger. Primary background:
[FLINT resultant and half-GCD documentation](https://flintlib.org/doc/nmod_poly/index.html#resultant-and-discriminant).
The Euclidean transformation and matrix specialization below are standard
algebra, not a novelty claim. KB unavailable: no MCP; repository wrapper reports
missing executable. Local exact kernels and primary documentation are fallback.

Retain caller-certified prime p in the native word-size domain and all source
prime/coprime subgroup assumptions. Internal matrix entries and quotient q are
canonical coefficient vectors: [0] for zero, otherwise nonzero leading entry
and coefficients in [0,p). This matches the existing half-GCD construction.
General matrix composition, polynomial division, transition leading factors,
degree/parity handling and resultant classes remain unchanged.

## Transformation

Let M=[[a,b],[c,d]] and S(q)=[[0,1],[1,-q]]. Then

    S(q) M = [[c,d],[a-qc,b-qd]].

Previous half-GCD code materialized S(q) and called the general matrix multiply,
whose eight polynomial-product calls include two zero products, four identity
products and two nontrivial products (-q)c and (-q)d. It then makes four sums.

step_left consumes M, computes qc and qd, forms a-qc and b-qd, and moves the
owned Vec buffers c,d into the first output row. It needs two polynomial-product
calls and two subtractions, no S(q) object and no multiplication to copy a row.
Rust ownership keeps c,d readable for these products and transfers them only
afterward; no unsafe code, aliases or in-place final-use assumptions are needed.

Both Euclidean update sites use this operation: the small-degree base-case loop
and the intermediate Euclidean step between recursive half-GCD calls. The
composition of two genuinely general recursive matrices still uses matrix_mul.

Because the replacement yields exactly the same matrix, induction through the
half-GCD recursion gives identical transformed polynomial pairs and transition
records. Therefore resultant leading-coefficient factors, parity changes, zero
termination and projected output are preserved. This is a local equivalence
argument; it does not introduce a new half-GCD complexity claim.

## Exact removed field products

Define L(v)=0 for the zero polynomial [0], otherwise the canonical vector length
(including internal zero coefficients). The old schoolbook multiplication of 1
by a nonzero v performs exactly L(v) field products; multiplying zero returns
without any products. The four identity terms therefore cost

    Delta(M)=L(a)+L(b)+L(c)+L(d)

field multiplications. The new operation removes exactly these terms. Product
counts for mul(-q,v) and mul(q,v) agree in this implementation: negation preserves
trimmed lengths and zero tests, and every recursive first-operand Karatsuba sum
is negated as a whole. Induction through the multiplication routine preserves
its recursion and product counts. Negation/addition/reduction costs are separate.

Thus the exact difference in Poly::fm counts per specialized step is Delta(M),
with no change in inversion count. No additional claim about cycle counts or
allocation counts is inferred from the number of top-level calls.

Matrices generated from the identity by Euclidean steps are unimodular:
det(S(q))=-1 and determinants multiply. Both rows must be nonzero, giving
Delta(M)>=2 for every internal step. For a complete resultant call, the saving
is the sum of Delta(M) over its executed specialized updates. The rest of its
polynomial work and all class-factor powers remain charged. Half-GCD's asymptotic
order is unchanged; this removes systematic identity work inside that algorithm.

## Storage and lifetime

The prior lower-row vectors become the new upper row with the same allocation,
capacity and address. The old upper row is dropped after its last subtraction.
The new lower row and multiplication/subtraction temporaries still allocate
through existing Poly routines. No constant-memory or total-peak-memory bound is
claimed. Avoided S(q) construction and two retained row copies are concrete
lifetime changes; the general composition and lifted applications are untouched.

## Exact validation

validation-01 compiles source_audit, resultant_audit and matrix_audit and passes:

- 288 canonical matrix/quotient cases across p=5,17,97,257, multiple degrees,
  zero q and zero entries. Direct dense S(q)M equality; identical moved-row
  addresses/capacities; exact Delta(M) difference in field products; equal
  inversion counts. General dense multiplication is retained as the reference.
- 432 complete resultant pairs at degrees9..96 and the same primes, all exercising
  half-GCD. Predecessor outputs agree, including47 zero resultants. Poly::fm
  count differences exactly equal accumulated Delta(M); inversion/root-product
  counts agree. Inputs include forced common factors, varied degree gaps and
  coefficient cancellation. This is deterministic software validation, not a
  benchmark or statistical speed estimate.
- Inherited8320singleton checks,128fallbacks,24empty errors,large-prime control;
  1108sourcepoints/24windows,160660division equalities,4100typed-lifetime plans,
  and8preparation controls.

recovery-01 preserves617targets/5929calls/zero unresolved and63public
comparisons/511calls. normal-build compiles seven native files without audit
cfg or reference/test modules. Original4563 cohort unchanged. No cloud run,
external benchmark, full Cargo rerun or numerical full-source cost profile.
The prior full-suite missing-fixture/deadline failure remains recorded.

Reproduce with `python3 generate.py`, then `python3 validate.py NEW_DIRECTORY`
and `python3 validate-recovery.py ANOTHER_NEW_DIRECTORY` from this directory.
Receipts bind compiler commands, source and output hashes; do not overwrite old
output directories. The normal build command is in its receipt. matrix-checks.rs
is a hand-written independent validation input pinned by the validation receipt.
The audit counter counts Poly::fm operations; class powers are unchanged and are
not counted by that counter. No unmeasured total is reported.

Next: general Euclidean division still materializes the full q*b product before
subtracting. Investigate a low-remainder construction with exact high-coefficient
cancellation proof, while retaining quotient availability for these row updates.
