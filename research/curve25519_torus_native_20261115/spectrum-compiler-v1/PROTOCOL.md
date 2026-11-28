# Compact spectrum compiler v1 — prospective development contract

This version implements the written coset-spectrum-v1 proposal, preserving its
source, evidence and original point/compiler comparators. It does not extend
the old seven-input compiler's validation contract. A separate typed gate
program admits the five coefficient inputs, three inverse guards and bounded
label/slack bits. Only public Spec, pretarget R, target Q=[8]R and a node cap
enter compilation. Supplied coefficients and labels enter a separate extension
control. No novelty is claimed for the algebraic or solver primitives.

Use the fixed p101/A95/q3 and original Curve25519/A486662/q213156431
parameter contracts already admitted by the native quotient query. Keep the
normalization/chart and nonidentity restrictions. Repeated coset labels are
permitted; repeated raw indexed columns are refused by original replay.

Implementation: build the monic sextic, guarded map inverse, rational-torsion
unit resultant and squarefree resultant; compute T_q(lambda)^2 in its rank-six
quotient; obtain its characteristic polynomial with Newton traces; equate it
to the product of six bounded-index eigenvalues. Label bits and slack bits
obey explicit Boolean and no-wrap range constraints. Retain every equation in
the native check and SMT export. Squarefreeness is required; rank six here is
not the dimension of a coefficient solution ideal.

Development controls: existing fixed known-answer sections only, native full
assignment checks, numeric quotient comparison, original index/group replay,
one-variable corruption and malformed/cap/guard/range refusals. Preserve
initial failures. No timing, fitted scaling, random cohort or paid job. Builds,
unit tests and static checks are local development under AGENTS.md §9a. Any
new search campaign or benchmark requires its separate admitted EKS protocol.
One final bounded correctness invocation may use at most 2GiB/600seconds,
with source committed first, distinct write-once BUILD/PRECALL/POSTCALL and
immutable executable/qualifier. A cap is incomplete, never target absence.

Backend admission only: official cvc5 1.4.2 native GPL/CoCoA release; verify its
published archive digest, version and finite-field capability. Native small
polynomial/bit-sum interface tests establish parser/model compatibility only.
No full original-base solver success, negative certificate or attack cost is
inferred. A SAT model requires independent native checking and original replay;
UNSAT/unknown without an independently checked certificate is an observation.

Prior art: Ozdemir et al., *Split Gröbner Bases for Satisfiability Modulo Finite
Fields*, CAV2024, §§3–5, https://theory.stanford.edu/~barrett/pubs/OPB%2B24.pdf.
The two bases separate bit sums from bit constraints, propagate equalities and
refine conflicts. Completeness still includes finite-field search; worst-case
GB complexity remains. The reported exponential gains concern verification,
particularly unsatisfiable determinism queries, not ECDLP decomposition.
cvc5 finite-field syntax and solver switch:
https://cvc5.github.io/docs/cvc5-1.4.2/theories/finite_field.html.
SMT export uses standard field add/mul plus experimental ff.bitsum under the
pinned release. Keep explicit bit equations and no-wrap assumptions.

KB unavailable: no loaded KB tools; scripts/research-kb status exits127.
Intended retrieval: “coset spectrum characteristic polynomial Curve25519”;
“Split Gröbner bitsum finite field satisfiable decomposition”;
“unknown section coefficients original indexed witness”; “spectral resultant
nilpotent reduced quotient”. Repository assessment/compiler/query and primary
CAV paper/docs were read instead. No exhaustive novelty review is claimed.
