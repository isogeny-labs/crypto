# Exact finite buffers and modular inverse specification

The actual local inverse can now be specified from a finite coefficient vector,
with a proved modular encoding. These are quantified algebraic statements,
not a new ECDLP algorithm, native executable certificate or measured speed.
ClaimsPCC-20261108-ff4290/24cbda/ffec83 pin the central declarations. Producer
standing remains unsigned. [pcc:unverified]

Let A be the concrete rational two-adic integral ring already defined in
LocalIntegral. For bit precision p and jet length q, define the input buffer
as the function Fin q -> A/(2^p) taking each coefficient to its residue class.

## Coefficient precision is exactly jet-ideal precision

FiniteBuffers.jet_mem_iff_coeff_divisible proves, for every commutative ring R,
scalar c and series f, at unsigned producer scope. [pcc:unverified]

`f in (C(c),X^q) iff c divides coeff_n(f) for every n<q`.

The reverse implication constructs the coefficientwise witnesses, then factors
the remaining series by X^q. No completeness, domain or nonzero-c hypothesis is
used. buffer_eq_iff_jet instantiates c=2^p in A and proves that equality of the
length-q buffers is **equivalent** to the actual jet-ideal congruence. This
closes the previously missing coefficient/ideal bridge. [pcc:unverified]

## The finite constructor does not assume a supplied inverse

liftBuffer zero-extends its length-q input vector. invertBuffer takes only
that vector and its known constant unit, applies the standard coefficient
recurrence, and returns its first q inverse coefficients. For the actual
source denominator, the mapped rhoUnit is proved to be the correct constant
unit when q>0; q=0 is handled as the unique empty vector. [pcc:unverified]

actual_invertBuffer_correct proves that this constructed output equals the
actual source inverse buffer. It does not ask the caller to supply a full
approximation E or an inverse F satisfying E*F=1. Those full-series witnesses
are constructed inside the proof from the finite input. The generic
inverse_low_coefficients lemma proves why higher input coefficients cannot
change the first q inverse coefficients. [pcc:unverified]

This is a mathematical finite-buffer operator. The current Lean definitions
use noncomputable mathematical structures; a compiled native backend and its
operation/bit-cost correspondence are not certified by this theorem.

## The residue classes have an explicit ordinary modular encoding

ResidueEncoding.toPadic embeds A into the two-adic integers and composes the
standard residue map to ZMod(2^p). residue_kernel proves its kernel is exactly
the principal coefficient ideal(2^p) in A. Surjectivity and the first
isomorphism theorem produce a ring equivalence. [pcc:unverified]

`A/(2^p) ≃+* ZMod(2^p)`.

This is an actual ring equivalence, not an assumed finite representation.
residue_formula proves the ordinary encoding formula. [pcc:unverified]

`residue(r)=num(r) * den(r)^(-1) modulo 2^p`.

The denominator is proved to be a unit modulo2^p from the actual norm bound
on r. No p-adic approximation oracle is needed to use this formula.
[pcc:unverified]

modular_inverse_correct proves correctness of the same finite inverse operator
with this concrete modular encoding, for every scale a, actual source node m,
precision p and length q. modular_buffer_card proves exactly2^(p*q) possible
length-q modular vectors. This is an information/cardinality statement; it is
not a measurement of an implementation's memory footprint. [pcc:unverified]

## Verification and remaining obligations

The default Lake build includes FiniteBuffers and ResidueEncoding. Clean builds
are buffer-build-004, residue-build-007 and buffer-default-build-001; source
commit07cfb5586 contains all current mathematical declarations. The expanded
actual source gate reports96 declarations,0 failures and0 extra trusted axioms;
official import-inclusive fresh replay receipts are in
lean/FINITE_BUFFERS_VERIFICATION.md. All are producer evidence, no trusted
signature. [pcc:unverified]

No new panel, timing run, native benchmark or cloud job was performed. Prior
frozen data and PDFs remain immutable; this proof report is a new versioned
artifact. Exact failed elaborations are retained instead of being treated as
proofs. The merge with current main required keeping both appended session-log
parts; the PCC main-prefix check still passes without rewriting any event.

The remaining mathematical/algorithmic obligations are actual contraction
homogeneity, full finite-precision Hermite recovery and uniform all-level mode
bounds with construction/child-call bit costs. The finite inverse specification
neither supplies nor bounds that complete recursive algorithm. PLAN.md records
the narrower completed obligations and the outstanding ones separately.

Sources checked: pinned PowerSeries.Basic coefficient and X-divisibility APIs;
Ideal.mem_span_pair and quotient APIs; PadicIntegers.isUnit_den; PadicInt.RingHoms
residue kernel; ZMod unit inverse and cardinality APIs. KB query unavailable127;
primary-source fallback, no novelty or literature-exhaustion claim.

## Corrected claim metadata

CI detected missing declaration hashes in the branch's unsigned Lean claims.
Original JSON records and the CI log are preserved under pcc-pin-repair/.
Canonical declaration pins were then added, with actual/canonical source paths
and hashes recorded, and corrected registrations appended to the existing
ledger. No theorem statement/scope, checker or trust policy changed. Native
schema checks enforce the required fields. This does not create independent
standing or pretend that the original records passed verification.
