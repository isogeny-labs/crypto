# Actual translation pullback: local orders, residues and divisors

Lean proves the compatibility missing from the
[function-field translation construction](../elliptic_translation_audit_20261115/README.md).
For every nonsingular general Weierstrass model over an algebraically closed
field, in every characteristic including two and three, the constructed
function-field pullback satisfies

`v_P(φ_Q(f)) = v_(P+Q)(f)`.

These are the actual normalized valuations of the actual finite-point local
DVRs and infinity chart. Local preservation, valuation invariance, residue
compatibility and ramification index one are derived. None is an additional
hypothesis. [pcc:unverified]

The proof constructs an actual isomorphism from the regular ring at `P+Q`
to the regular ring at `P`, with inverse given on the actual function field
by translation through `−Q`. Actual residue evaluation commutes with that
map. All points are covered, including infinity, inverse pairs, doubling
and two-torsion. [pcc:unverified]

The divisor sign is now proved:

`div(φ_Q(f)) = (τ_(-Q))_* div(f)`.

For functions unit on the relevant supports, actual divisor evaluation obeys
`φ_Q(f)(D) = f((τ_Q)_*D)`. The corresponding disjoint-support conditions
are proved equivalent. The product helper has a default outside its local-unit
domain; that default is not claimed to be a residue of a meromorphic function.
Actual evaluations retain their support guards. [pcc:unverified]

## Construction and skeptical control

For a finite translation point `(b,c)`, the actual shifted curve equation
and actual generic-point addition formulas give coordinate differences
`dx=(Av−tw)/v²`, `dy=−(w/v+a1)dx−t`, where `v=x−b`, `w=y−c`,
`t=2c+a1b+a3` and `A=3b²+2a2b+a4−a1c`. Cancellation using the actual
curve equation shows both differences lie strictly in the infinity maximal
ideal. The actual translated point uniformizer has value `exp(−1)`: use
`dx` if `t≠0`, and `dy` if `t=0`, when actual nonsingularity forces `A≠0`.
[pcc:unverified]

Actual coordinate substitution lands in the actual infinity regular ring,
and its actual residue is evaluation at `(b,c)`. This proves the localization
denominators become units. The actual localization map commutes with both
the full function field and residue maps. Actual units have value one;
official DVR uniformizer-times-unit factorization gives the normalized
valuation identity for every actual function at infinity. The already proved
function-field composition law then gives the identity at every point.
[pcc:unverified]

The counterexample uses the actual scalar extension of the credited F5 curve
`y²=x³+1` to the actual algebraic closure of F5. Lean proves actual
characteristic five and nonzero discriminant, constructs a nonzero point `P`
with `3P=O` and `P≠−P`, and proves the actual nonzero tangent function has
divisor `3(P)−3(O)`. Its order at `P` is three, at `−P` zero, and the order
of its pullback through `P` at infinity is three. Consequently positive-`P`
divisor pushforward gives the wrong answer. The negative sign above passes.
The finite control and established methods are credited; this is not a new
curve population, novelty claim or refutation of a qualified source.
[pcc:unverified]

## Checks and retained failures

The [main certificate](lean/LocalTranslationCertificate.lean) explicitly
bundles the universal valuation/order, regular-ring isomorphism and inverse,
residue, principal-divisor and guarded evaluation statements, together with
the concrete counterexample's characteristic, curve, point and function guards.
The whole tracked-source gate audits 78 theorems across nine new concrete
modules, with zero failures and zero trusted-native exemptions. The dependency
has 96 frozen custom modules, 105 formal files and 124 source/artifact pins;
none was copied or edited. [pcc:unverified]

All 21 build states are preserved: fourteen failures and seven successes.
Missing evaluation imports produced default 200,000-heartbeat failures;
actual ring-wrapper and implicit projection mismatches required explicit
interfaces. An early configuration registered only the latest Lake module,
allowing earlier compiled files to be reused. State 17 corrected the roots
and rebuilt all seven then-current modules; an exact source/root coverage
assertion runs before every build from state 18. Final state 21 includes all
nine modules, and the whole source gate explicitly builds every tracked
source. Earlier target-only successes do not stand in for this closure.
All 124 predecessor pins matched before and after every state and the gate.
[pcc:unverified]

Final build and gate source warnings are zero; the frozen auditor retains one
`String.mk` deprecation. No holes, custom axioms, native proof evaluation,
raised limits, new global elaboration flags, linter waivers or frozen edits
were used. Accepted foundations are `propext`, `Classical.choice`, `Quot.sound`.
Python handles metadata, documents and archives only. No native cryptographic
experiment, paid compute or agent upload was run. [pcc:unverified]

See the [protocol](PROTOCOL.md), [source review](evidence/source-review.json),
[preflight](preflight.json), [proof map](proof-map.json),
[full written-header check](evidence/binding-check.json) and
[report](reports/report.pdf). Build snapshots are losslessly archived;
`restore_attempts.py DEST` checks every restored byte. The full written-header
check is a frozen same-host/prover procedure. It does not establish independent
parser, compiler or physical-host assurance. Signed standing remains unverified.
[pcc:unverified]

## Sources and remaining scope

Fresh Galbraith corrected [chapter 7](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch7.pdf),
PDF pages 10–11, printed 132–133: full text and individual full-page visual
inspection, including Lemmas 7.4.7, 7.4.10–14 and 7.4.16. Only the beginning
of 7.5.1 was read there; no reading of the whole 7.5.2 proof is claimed.
Corrected chapter 9 PDF pages 3–4, printed 167–168, was reread in full text;
prior individual visuals are credited. Definition 9.2.2 and Example 9.2.4
retain their actual point/affine-domain distinctions. Official DVR factorization
and adic proofs, and prior actual local-ring/residue/translation proofs, were
read and reused. Copyright primary files remain local and outside the archive.
[pcc:unverified]

Sources checked (KB): no relevant MCP tool was loaded. Four correct wrapper
calls—status, two distinct mechanism/negative searches, and theorem search—
returned 127 before approach or code. Conductor was also unavailable (127).
The bounded fallback used the primary, official and prior sources above;
unavailability is not evidence of absence. [pcc:unverified]

This closes the actual point-local translation compatibility obligation.
Separate projective `Scheme.localRing`/line-bundle comparisons, general
base-field descent, Weil reciprocity, isogeny/Lang/Jacobian bridges and
canonical Weil–Tate/MOV–FR remain open. The fourteen-item request remains
active. [pcc:unverified]

Exact local **PCC-20261115-67cf17** replay PASS at 2026-11-15T21:07:29Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: ready [PR #4343](https://github.com/isogeny-labs/crypto/pull/4343), stacked on #3739 after #3743 merged and its branch was deleted. Remote proof `178c18f5111fae22c20e2d121bfd36fc129be139` and local receipt `0db8d3008db9f24d6953cae14bab3a76a5a355a5` share tree `0eb01895d5255ca53433d67913944795c51d8706`. Anonymous fetch completed; fresh head/body/open/ready matched. See [integration context](integration-context.json) for the recorded publication-destination deviation; formal criteria and pins are unchanged. Publication metadata is outside the frozen pinset. [pcc:unverified]
