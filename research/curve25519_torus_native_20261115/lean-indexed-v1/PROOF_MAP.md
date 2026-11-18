# Existing primitive-root laws for indexed extension

1. Structure above: Mathlib Field, IsDomain, IsPrimitiveRoot, ordinary
   multiplication/division/natural powers and NeZero natural order.
   Module Mathlib.RingTheory.RootsOfUnity.PrimitiveRoots.
2. Laws used as stated: IsPrimitiveRoot.pow_div_gcd,
   eq_pow_of_pow_eq_one, pow_inj; div_pow, div_self, pow_ne_zero,
   mul_div_cancel₀ and mul_left_cancel₀. Read the pinned primary Mathlib
   source at cfc55fc58714a59dc06e6e79edc6d1d95140d3c2 before editing.
3. Objects as instances: an arbitrary coefficient field K, an element h of
   primitive order60824497*213156431, and its order-q power h^60824497.
   These are hypotheses, NOT proved identifications with native Fp2 or a
   primality/exact-order certificate for the working Rust parameter object.
4. Assembly probe: symbolic q with NeZero q, primitive g and r^q=1;
   apply eq_pow_of_pow_eq_one directly. Then instantiate actual q.
5. Missing components: native Fp/Fp2 semantics and primality/irreducibility
   bridge, Rust exact-order checker soundness/parameter binding, Boolean
   range/borrow and constant-product compiler semantics, full original
   circuit roots and curve/group replay. The present endpoints prove only
   the CONDITIONAL algebraic existence/uniqueness step. No solver cost,
   full root bijection of the native compiler or useful-yield result.
6. Inheritance: powers/division and cancellation from Field; primitive
   order from explicit IsPrimitiveRoot hypothesis; index existence from
   eq_pow_of_pow_eq_one, uniqueness from pow_inj. gcd/division of fixed
   naturals is normalized with existing arithmetic tactics, not a new
   generic law. No custom axiom, native exception or increased limit.

## Proof chain before Lean source

Subgroup: pow_div_gcd at exponenta gives primitive orderM/gcd(M,a).
Normalize the fixed naturals toq. This is an existing-law endpoint.

Existence: (z/h^j)^q = z^q/(h^j)^q by div_pow;
replace numerator by the membership hypothesis, then div_self gives1
because pow_ne_zero preserves h!=0. The existing primitive-root theorem
gives t<q and (h^a)^t=z/h^j. Replace that component and use
mul_div_cancel₀ to get h^j*(h^a)^t=z.

Uniqueness: cancel the nonzero h^j with mul_left_cancel₀. Then apply
primitive-root pow_inj at the two strict range bounds. No root bound or
group law is re-proved locally.

| Statement | Kind | Owner | Exact scope |
|---|---|---|---|
| actual_subgroup_primitive | Endpoint | Study | primitive h^a under primitive h of declaredM |
| actual_coset_index_exists | Endpoint | Study | t existence under field, nonzero h, primitive h^a, original power equality |
| actual_coset_index_unique | Endpoint | Study | range-restricted uniqueness under the same field/primitive contracts |

CryptoLean5f2ca251cb34806f69fd5455175434cac3567dfa,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2, Lean4.35.0.
Before the focused gate, every statement is source-only and unverified.
The generic laws already exist; there is no upstream candidate or study-local
replacement theorem. Any failed source/build stays in the versioned study.

## Local qualification

The focused maintained gate audited3 named theorems,0 failing and0 trusted
native exceptions. Axiom output is propext/Quot.sound for the subgroup
endpoint and additionally Classical.choice for existence/uniqueness.
Signed PCC standing remains unverified. Full native compiler/Fp2 binding
is still missing; these are the conditional algebraic endpoints above.

The first source type-checked but had a redundant trailing norm_num after
convert. Its source archive/log are retained in indexed-circuit-v1/development;
the final source removes that unused tactic without changing the statement.
Final logs retain informational module-system suggestions and a pre-existing
String.mk deprecation in the maintained auditor. No linter/gate was disabled,
no limit was raised, and no old source or proof binding was edited.
