# Elliptic translation over every coefficient field

Lean removes the algebraically closed-field restriction from the actual
function-field and local translation constructions. For every field `K`,
every nonsingular general Weierstrass model over `K`, and every actual point
`Q`, it constructs a coefficient-fixing function-field automorphism with
composition, identity, explicit inverse through `−Q` and faithfulness.
Every characteristic is covered, including two and three. [pcc:unverified]

Actual normalized local valuations satisfy
`v_P(φ_Q(f)) = v_(P+Q)(f)` at every actual point, including infinity and all
exceptional configurations. Actual regular rings are isomorphic with the
explicit negative-point field inverse; actual residue evaluation commutes.
Transcendence, substitution injectivity, inverse, local preservation,
normalized valuation and residue compatibility are derived. [pcc:unverified]

The [main certificate](lean/GeneralTranslationCertificate.lean) also proves
the actual translation square commutes under every coefficient-field
extension. Actual normalized local values and orders are preserved at images
of `K`-rational points. The actual regular-ring map under coefficient
extension is injective and its residue diagram commutes. [pcc:unverified]

## Proof mechanism and scope

The actual infinity norm-degree valuation is trivial on actual coefficient
functions. For a finite translation point `(b,c)`, the actual translated
generic x-coordinate minus `b` has value `exp(−1)` when the y-derivative is
nonzero, and `exp(−2)` otherwise. Actual nonsingularity supplies the other
nonzero derivative in the latter case. The official Mathlib theorem
[`Valuation.transcendental_of_ne_one`](https://github.com/leanprover-community/mathlib4/blob/56a005855ea17e0b0958f73ec801ae140b0a1679/Mathlib/RingTheory/Valuation/IsTrivialOn.lean)
proves transcendence. The infinity point uses the actual x-function's value
`exp(2)`. This argument works over the original coefficient field.
[pcc:unverified]

The credited actual norm-product identity then proves coordinate substitution
injective, giving the actual fraction-field lift. Actual official `Point.map`,
generic-point addition and coefficient/x/y/basis/fraction generation prove
the group action and inverse. Over algebraically closed fields, the map
agrees with the [frozen construction](../elliptic_translation_audit_20261115/README.md).
[pcc:unverified]

Actual coordinate formulas and fraction generation prove the arbitrary
coefficient-extension square. The actual point-evaluation diagram makes
localization denominators units, producing an actual map of finite-point
local rings that commutes with function fields and residues. Actual derivative
branches preserve the chosen uniformizer; actual units and official DVR
factorization give normalized finite-point basechange on every fraction.
The existing infinity norm-degree basechange is credited. These bridges and
the algebraic-closure comparison derive the all-field local translation
identity. The algebraic closure occurs in that proof; the conclusion has no
algebraic-closure hypothesis. [pcc:unverified]

The actual principal point-divisor identity has the sign
`div(φ_Q(f)) = (τ_(-Q))_* div(f)`. For functions unit at the relevant support,
actual evaluation satisfies `φ_Q(f)(D) = f((τ_Q)_*D)`, with the disjoint-support
conditions proved equivalent. The existing guarded-product helper is used
only on its actual local-unit domain. [pcc:unverified]

Over a nonclosed field, this point-divisor API records `K`-rational points.
General geometric divisor/Picard/Galois descent remains open. The degree-zero
geometric divisor theorem continues to have its geometric scope.
[pcc:unverified]

## Concrete controls and counterexamples

- On the actual nonsingular F2 model `y²+y=x³`, Lean constructs a nonzero
  point and proves its actual function-field translation is nonidentity.
  It does the same on the credited nonsingular F3 curve `y²=x³+x+1`.
- On the credited source curve `y²=x³+x`, with `Q=(0,0)`, the actual
  general-field map has formula `x ↦ 1/x`, `y ↦ −y/x²`. Directly over F5,
  Lean proves it fails to preserve the actual affine coordinate-ring image.
  Source point/affine-domain distinctions and earlier exceptional/ordinary
  point controls remain intact.
- The credited F3 function `x²+1` has rational-point-only divisor degree
  `−4`. Lean proves this degree persists under every actual translation.
  This is an explicit boundary on using rational-point counts as geometric
  principal divisors.

Actual field characteristics, curve discriminants, nonzero points and function
guards are proved. Known models, source methods and earlier controls are
credited; no new curve population, qualified-source refutation or novelty is
claimed. [pcc:unverified]

## Checks and provenance

The whole tracked/current-source gate audits 92 theorems across twelve new
concrete modules: zero failures and zero trusted-native exemptions. The
dependency has 105 frozen custom modules, 117 formal files and 134 source/
artifact pins, with zero copied or edited modules. Every old pin matched
before and after each build state and the gate. [pcc:unverified]

All eleven states are retained: failed 01, 03, 06 and 07; successful 02, 04,
05, 08, 09, 10 and 11. Failures exposed an implicit valuation argument,
a missing opened namespace, a nonsingular witness hidden by an equation
projection, a repeated local-instance name and an insufficiently explicit
unit rewrite. Exact source/root coverage is asserted before every build
from state 01. Final and gate source warnings are zero; the frozen auditor
retains one `String.mk` deprecation. [pcc:unverified]

Accepted foundations are `propext`, `Classical.choice`, `Quot.sound`. No
holes, custom axioms, native proof evaluation, raised limits, new global
elaboration flags, linter waivers or frozen edits were used. Python handles
source/document/provenance metadata only. No native cryptographic experiment,
paid compute or agent upload was run. [pcc:unverified]

See the [protocol](PROTOCOL.md), [source review](evidence/source-review.json),
[preflight](preflight.json), [proof map](proof-map.json),
[written-header check](evidence/binding-check.json) and
[report](reports/report.pdf). The lossless archive retains failed source,
configuration, log and exit snapshots; `restore_attempts.py DEST` checks every
restored byte. The frozen written-header check uses the same host/prover and
does not establish independent parser, compiler or physical-host assurance.
Signed standing remains unverified. [pcc:unverified]

## Sources and remaining obligations

Fresh corrected Galbraith [chapter 7](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch7.pdf)
PDF pages 6–9, printed 128–131, and
[chapter 9](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch9.pdf)
PDF pages 3–4, printed 167–168: full text and individual full-page visual
inspection. The review includes Lemma 7.2.9 and proof, Example 7.3.4,
Lemma 7.3.5 and proof, and Lemma 7.3.6's statement; its exercise proof is
not claimed read. Definition 9.2.2 and Example 9.2.4 retain the actual
affine Q/O exclusions. No whole chapter or projective Scheme-morphism proof
is claimed read. Copyright primary files remain local and outside the archive.
[pcc:unverified]

Official trivial-valuation transcendence, point-map/generation, coordinate
norm and local DVR methods were read and reused. Sources checked (KB): no
relevant MCP tool; status, two mechanism/negative searches and theorem search
returned 127, followed by a fifth valuation-specific search returning 127
before selecting the method. Conductor returned 127 before code. Bounded
primary/official/prior fallback was used; unavailability is not evidence of
absence. [pcc:unverified]

Separate projective `Scheme.localRing`/line-bundle comparisons, general
geometric divisor/Picard/Galois descent, Weil reciprocity, isogeny/Lang/Jacobian
bridges and canonical Weil–Tate/MOV–FR remain open. The fourteen-item audit
remains active. [pcc:unverified]

Exact local **PCC-20261115-833af1** replay PASS at 2026-11-15T22:06:27Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: ready [PR #4355](https://github.com/isogeny-labs/crypto/pull/4355), stacked on #3739 after #4343 merged and its branch was deleted. Remote proof `34044dba931f705ce0336ccaddd2b3b118db770c` and local receipt `6a304f0e8455d5a761d422b836e146672756fdfa` share tree `f67d180e4112cc5e29254a06cc333063a7e8c4db`. Anonymous fetch completed; fresh head/body/open/ready matched. See [integration context](integration-context.json) for the recorded publication-destination deviation; formal criteria and pins are unchanged. Publication metadata is outside the frozen pinset. [pcc:unverified]
