# Formal scope and instance gate, before Lean edits

All results in this version are producer unverified. A passing Lean assembly
does not prove the whole coefficient reduction. [pcc:unverified]

## Six fields

1. Structure above: Mathlib `Polynomial R`, commutative semiring divisibility,
   `Squarefree`, and the existing nonunit polynomial `X-C x`. No new semantic
   object, syntax layer or certificate family.
2. Theorems used as stated: `Squarefree.squarefree_of_dvd` and the defining
   squarefree implication, with `Polynomial.not_isUnit_X_sub_C`. These give
   inheritance of squarefreeness and exclusion of squared linear factors.
   They do not supply split membership, norm identity or rational ordinates.
3. Object as instance: polynomials over `ZMod (2^255-19)`. The modulus is a
   numeric constant; these endpoints need no primality or curve assumptions.
4. Assembly probe: type-check these two compositions with S/E_B as parameters,
   then at the stated coefficient ring. The source itself is this probe and
   has no additional wrapper. Direct compile and maintained file audit pass;
   retained receipts are under lean-gate. [pcc:unverified]
5. Missing components: trace-coset split/separable polynomial equality;
   native denominator-clearing binding; norm/line-bundle quotient binding;
   nonsquare-ordinate/common-root multiplicity argument; raw-label injectivity;
   XZ denominator/group equivalence; literal degree leading-path induction.
   Generic new laws belong upstream in crypto-lean before study consumers.
6. Inheritance table below; no claim that missing rows have been implemented.

| Operation or statement | Existing law / owner | Status |
|---|---|---|
| divisor of squarefree E_B is squarefree | `Squarefree.squarefree_of_dvd` / Mathlib | direct application, local gate pass |
| squarefree S has no squared linear factor | squarefree predicate + `Polynomial.not_isUnit_X_sub_C` / Mathlib | direct application, local gate pass |
| square divided by square is square | `IsSquare.div`, `IsSquare.mul_self` / Mathlib | source reviewed, no curve binding claimed |
| full rational support equivalence | missing bridges listed above | written derivation and native controls only |
| generator degree 4qk-10 | missing generic leading-path induction | written derivation and finite exact pencils only |

## Statement/dependency/fidelity table

Both endpoints are applications of existing laws, not new generic laws.
First instantiate the ring, then squarefree inheritance, then the no-double-root
endpoint. Neither endpoint depends on the other; no renames or model changes.
No limits are raised and no composite requires opening beyond the squarefree
predicate's defining contract. No native evaluation or custom axiom is used.

| Report statement | Endpoint | Difference from report | Receipt |
|---|---|---|---|
| S inherits squarefreeness if S divides squarefree E_B | `SectionCoeffAssembly.actual_squarefree_of_divisibility` | E_B squarefree and divisibility are hypotheses, not proved native bindings | lean-gate/build.log and audit.log |
| squarefree S excludes a double linear factor | `SectionCoeffAssembly.actual_no_double_linear_factor` | squarefreeness is a hypothesis, not a compiler theorem | lean-gate/build.log and audit.log |
| norm support forces Fp ordinates | none | full formalization open | none |
| exact literal generator degree | none | full formalization open | none |

Toolchain/dependencies reuse the pinned v4.35.0 / Mathlib
cfc55fc58714a59dc06e6e79edc6d1d95140d3c2 and CryptoLean
5f2ca251cb34806f69fd5455175434cac3567dfa environment from section-search-v1.
The old source, manifest and gate receipts remain unchanged. Pin and audit the
new assembly separately. No broad root-library test rerun.

Local gate:2 named theorems,0 failing,0 trusted native; printed axioms are
propext, Classical.choice and Quot.sound. The initial Lake build unexpectedly
rebuilt dependencies and was stopped with143. After forced selected-cache
restoration, direct compilation and the unmodified pinned CryptoLean gate on
the exact source file passed. This file-target invocation inherits Lake's
dependency environment without rerunning Lake's project build. Failed source
and complete logs are retained in development/retained-attempts.tar.xz.
No independent signed standing or full-theorem proof is inferred.
[pcc:unverified]

Statement-fidelity checks: both statements quantify over every polynomial in
the stated ring, not a finite example; no primality/group hypotheses are
needed for these inherited divisibility laws. Their hypotheses are satisfiable
(S=E=1), and omission of squarefreeness would permit S=(X-C x)^2. Both
proofs use their divisibility/squarefreeness hypotheses; the missing E_B and
norm bindings are explicitly outside their conclusions.
