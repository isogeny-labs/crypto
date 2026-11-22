# Lean instance gate and proof map

Full proposed average-energy theorem: OPEN. No Lean theorem below proves the
cover construction, independence of its square classes, morphism degree,
collision count, or an attack exponent. Concrete arithmetic is one dependency.

Six gate fields for the finite arithmetic endpoint:

1. Structure: Lean Core Nat, its ordered semiring operations, and Decidable
   propositions. No study-specific arithmetic syntax or checker is introduced.
2. Existing theorem: Core `of_decide_eq_true`; use its decidable-reflection
   rule as stated. Kernel `decide` checks the finite instance.
3. Instance: p=2^255-19; M=60824497*213156431; L=4M;
   W0=16L(L-1)(L-2)(L-3); K=2^28. All are concrete Nat values.
4. Assembly probe: `example (P : Prop) [Decidable P] (h : decide P = true) : P :=
   of_decide_eq_true h`. The instance is discharged by `by decide`, then
   `#print axioms` is inspected. This is arithmetic, not a geometric probe.
5. Missing components: none for these finite inequalities. For the geometric
   theorem, the cover/morphism-degree bridge remains unformalized; do not
   assume that theorem as an axiom. Existing CryptoLean.Polynomial.EllipticMaps
   proves equations only and explicitly excludes morphism/degree conclusions.
6. Inheritance: arithmetic, comparison and subtraction inherit Core Nat;
   decision procedures inherit Core's Decidable instances; reflection uses
   of_decide_eq_true; no custom axiom, native_decide or kernel bypass.

| Report dependency | Lean owner / existing result | This artifact |
|---|---|---|
| Concrete M divides p+1 | Core Nat arithmetic | Arithmetic.divisor |
| Admissible-parameter denominator positive | Core Nat arithmetic | Arithmetic.admissible_positive |
| Exact energy-correction arithmetic | Core Nat arithmetic | Arithmetic.correction_lt_one |
| Finite support/coverage inequality | CryptoLean.PartiteCoverage.support_cauchy_schwarz; Mathlib Finset.sum_mul_sq_le_sq_mul_sq | Identified, no actual torus instance assembled |
| Elliptic quotient polynomial equations | Existing generic polynomial structures; source identity certificates/exact Rust expansion | No new Lean law claimed |
| Multiquadratic cover and nonconstant map | Actual smooth projective covers and elliptic morphisms | Missing definitions/bridge here |
| Map-degree parallelogram and collision bound | Milne AV Corollaries 5.3–5.4, then degree positivity | Written draft; Lean bridge missing |
| Average energy and planted/uniform transfer for this base | Needs the preceding actual-object bridge | OPEN |
| Whole index-calculus cost/exponent | Needs solver and complete phase bounds | OPEN |

Reference shared library: aburan28/crypto-lean at
5f2ca251cb34806f69fd5455175434cac3567dfa. Its toolchain is
leanprover/lean4:v4.35.0; Mathlib pin cfc55fc58714a59dc06e6e79edc6d1d95140d3c2.
The arithmetic file imports only Core, so Mathlib/CryptoLean are references,
not hidden runtime dependencies. No change to the shared library is needed
for an instance of an existing Core rule. Initial isogeny-labs/crypto-lean
lookups were 404; installed-repository discovery located aburan28/crypto-lean.

| Statement | Kind | Owner | Structure and hypotheses |
|---|---|---|---|
| divisor | Instance obligation | This study | Core Nat; fixed p and M |
| admissible_positive | Instance obligation | This study | Core Nat; fixed p and M |
| correction_lt_one | Instance obligation | This study | Core Nat; fixed p,M,L,W0,K |
| class_lower_bound | Instance obligation | This study | Core Nat; fixed M; arithmetic only |

Final gate receipt: [gate-v1](gate-v1/), four theorems and zero trusted-native
exceptions. `#print axioms` reports no axioms for the four declarations.
PCC-20261115-2deca4 records this narrow producer check; signed independent
standing is separate. No geometric theorem is assumed as a Lean axiom.

## Birthday diagnostic arithmetic instance, 2026-11-16

Six gate fields: (1) same Core Nat ordered semiring and Decidable structure;
(2) same Core of_decide_eq_true, used as stated; (3) birthdaySamples is the
explicit native-planner integer 268951465398242860562463204936310 and W0 is
the existing instance, not a copied definition; (4) the earlier Core assembly
probe applies unchanged, and by decide checks the three actual inequalities;
(5) no missing arithmetic component, but the finite-IID moment/second-moment
and binomial monotonicity instances are not formalized here; (6) operations,
order and reflection inherit Core exactly as before. No limit is raised.

| Statement | Kind | Owner | Structure and hypotheses |
|---|---|---|---|
| birthday_minimum | Instance obligation | This study | Core Nat; fixed block length |
| birthday_completeness | Instance obligation | This study | Core Nat; fixed block length and existing W0 |
| birthday_soundness | Instance obligation | This study | Core Nat; fixed block length and existing W0 |

These endpoints prove only the finite integer thresholds. Existing generic
finite-support Cauchy–Schwarz belongs to CryptoLean.Group.PartiteCoverage;
new reusable probability/counting laws would belong in crypto-lean. This
study introduces no law of probability and assumes no probability axiom.
