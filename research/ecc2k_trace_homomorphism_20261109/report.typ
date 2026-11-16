#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: "1.")
= Binary trace constraints in point-decomposition solvers

2026-11-09 · Isogeny-labs/crypto · producer report, unsigned PCC standing.

The supplied screenshot is from Kosters and Yeo, _Notes on summation
polynomials_, arXiv:1503.08001v3. The native SAT path already supplied its
aggregate trace equation. This patch shares that construction with the
Gröbner, F6 and Crossbred group-decomposition frontends. Explicit Gröbner
and equation-export on/off controls retain a matched reference.

#image("flow.svg", width: 100%)

== Mathematical contract

For the binary model y² + xy = x³ + a x² + b, define delta(O) = 0 and
delta(P) = Tr(x(P) + a). Proposition 4.2 of the paper gives a group
homomorphism with kernel 2E. Thus R = P₁ + … + Pₘ implies
Σ Tr(xᵢ) = Tr(xR) + (m+1)Tr(a).

The equation is inserted before variable permutation and involves only
summand coordinates. Zero equations are skipped; contradictions are kept;
duplicate equations are not added. Full rational-point lifting remains
mandatory. Raw summation-polynomial builders keep their original contract:
Remark 4.8 warns that higher-polynomial algebraic roots need not satisfy
the rational-point relation. These are implementation assertions awaiting
independent signed verification.

== ECC2K-130 applicability

The exact challenge is E₀: y² + xy = x³ + 1 over GF(2¹³¹), represented
by z¹³¹ + z¹³ + z² + z + 1. It is different from the random ECC2-131
challenge. This study uses the published generator and target through
BinaryCurve.certicom_challenge and the existing degree-83 E₀ fixture.

The fixed campaign configuration selects a subgroup normal-basis base
of weight at most two. Trace is parity in every normal basis; subgroup
points have trace zero on E₀. Hence its admitted nonzero coordinates have
weight two, and the aggregate trace row is 0 = 0 on that domain. This is
an applicability analysis, not a campaign runtime measurement.

With ambient SAT coordinates the row is implied by membership, rather than
a literal zero polynomial. An explicit redundant XOR could still help
propagation; that effect is unmeasured. Literal zero-row omission applies
to a trace-zero linear coordinate basis.

The generic polynomial FieldStructure is exact only through field degree
64. That guard is preserved. Arithmetic checks at degree 83 and 131 do
not establish a wide-field Gröbner solver or a full-DLP comparison.

== Validation and remaining measurements

Native tests cover parity signs for a = 0/1 and m = 2/3/4, intermediate
variable independence, zero and contradictory rows, duplicate suppression,
trace-on/off agreement with exhaustive search, group lifting and exported
variable order. Further checks use the published challenge points and
rank-check a normal basis in the actual challenge field, checking trace
parity. Consult VALIDATION.md for the actual command outcomes.

No timing or speedup is reported. The required S3 experiment checkpoint
cannot be pushed with the current session access; the matched EKS study,
external solver comparisons, degree-83 full-DLP comparison and challenge
campaign measurement remain pending. The planned controls and admission
requirements are retained in PROTOCOL.md. No measurement changes the
leaderboards or performance graphs.

== Sources and reproducibility

Baseline source: e472218c8b14a770c38a1198bc2073edbc03d480.

Primary source: #link("https://arxiv.org/pdf/1503.08001")[Kosters–Yeo,
Proposition 4.2, Proposition 4.7 and Remark 4.8].

Repository sources: koblitz_index_calculus.rs (SAT and group-decomposition
frontends); koblitz_groebner.rs (exactness guard); binary_ecc/curve.rs
(challenge); docs/ic/params/ecc2k130-fixed.json and
codegen/indexcalc_fixed.py (configured subgroup normal-basis recipe).
README.md records precise paths, requirements and all pending deliverables.

Knowledge-base fallback: no MCP tools loaded; scripts/research-kb status
exited 127 because no executable was installed. No novelty claim is made.

== Final local checks

Five focused trace tests and six existing decomposition/status regressions
pass in the corrected diagnostic build: release profile, package crypto
opt-level=0. These are unsigned producer checks, not admitted timing or
independent verification. The original optimized run exposed the retained
two-torsion case: three copies of (0,1) sum to (0,1), but an intermediate
sum is infinity. Chained frontends now report such targets unsupported
rather than refuted. No missing infinity branch is claimed implemented.

The earlier whole-library run was interrupted after about 13 minutes and
was already red; isolated failures identify missing GPU source and a
historical session record. Other failures are unclassified. Ordinary
optimization rerun of the final source, full-suite validation, signed
verification and performance measurements remain pending. Exact command
outputs are retained in checks/ and VALIDATION.md.
