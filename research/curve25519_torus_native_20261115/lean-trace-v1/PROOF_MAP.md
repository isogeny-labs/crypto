# Existing Chebyshev laws at the actual exponent

1. Structure above: Mathlib CommRing, Polynomial, Field/IsDomain and NeZero2;
   module Mathlib.RingTheory.Polynomial.Chebyshev.
2. Theorems used as stated: Polynomial.Chebyshev.natDegree_T,
   natDegree_U_natCast and T_derivative_eq_U. No new generic law is proved.
3. Object: the existing polynomials at the actual fixed integer q213156431
   and q-1, over an arbitrary coefficient field with2 nonzero. Instantiating
   a particular field is conditional on its field/nonzero2 contracts.
4. Assembly probe: the first three examples in TraceDegree.lean apply the
   named laws with q left symbolic. It type-checks before the fixed numeric
   endpoints, which are only those applications at the declared integers.
5. Missing components: native AD semantics, the Pell-to-coordinate lift,
   full Fp2/group/compiler semantics and solving-degree/runtime/yield bounds.
   Their generic laws would belong in crypto-lean; no study-local law is
   copied or new axiom added here. Source derivation/native certificates
   do not fill these formal gaps.
6. Inheritance: polynomial operations, derivatives, degrees and coefficient
   field are Mathlib's existing objects/instances. Only integer index
   normalization uses simplification; no recurrence is unfolded, limit
   raised or native exception introduced.

| Statement | Kind | Owner | Existing law |
|---|---|---|---|
| actual_trace_degree | Endpoint | Study | natDegree_T |
| actual_lift_u_degree | Endpoint | Study | natDegree_U_natCast |
| actual_derivative_identity | Endpoint | Study | T_derivative_eq_U |

Pinned context: CryptoLean5f2ca251cb34806f69fd5455175434cac3567dfa,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2, Lean4.35.0. This is not
a primality certificate or a formal proof of coordinate elimination.

The derivative endpoint retains the existing law's explicit integer cast
and index q-1. Converting its large integer coefficient to a natural numeral
cast exhausted kernel recursion in an earlier presentation. No limit is
raised; the final endpoint uses the theorem literally. The first failed
sources and diagnostics are archived separately and are not claimed proofs.
