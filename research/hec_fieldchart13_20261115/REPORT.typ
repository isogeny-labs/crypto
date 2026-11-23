#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Exact quartic sections at runtime primes
Genus 3 · 15 November 2026
#image("flow.svg",width:100%)

Chart and section arithmetic now take a runtime prime. The fixed cohort uses X⁴+Y⁴−Z⁴=0 over97,257,509,1009. The curve is smooth in these characteristics: simultaneous partial zeros4X³=4Y³=−4Z³=0force the zero vector. These are reductions of one family, not generic random curves or source-isogeny targets.

== Preserved failure and amendment
V1source4695e8bfb65f/checkpoint20261116T004323Z stopped at97: no admitted origin among the first32affinepoints. Remaining fields were not run. The failure and all outputs remain in run/and the COHORT files.

V2source9d403f7de199/checkpoint20261116T004516Z separately freezes search over all affine origins. Exact no-supported-origin failures are retained as unsupported; other errors stop. All40affineorigins at97are rejected. Primes257,509,1009pass. Controller completion classifies all outcomes; it does not mean all charts succeeded.

#table(columns:4,inset:6pt,
 [Prime],[Affine / projective points],[Origin],[Outcome],
 [97],[40 / not chart-certified],[none],[Unsupported],
 [257],[248 / 252],[(2,25)],[Pass],
 [509],[520 / 520],[(3,63)],[Pass],
 [1009],[1096 / 1100],[(2,138)],[Pass])

== Exact validation on supported fields
Each passes RRdimension8, multiplicationrank18, diagonalrank112/nullity8, exact coefficient identities and corruption rejection,480determinant and480ordinary split tensor controls,120base-divisor derivative-limit checks, and canonical tangent/line checks. Point lists agree with the independent exhaustive enumerator. All successful new-prime cancellation cases are split; this does not test non-split cancellation at the new primes.

== Scope and next mathematical obstruction
Runtime modular arithmetic, inversion, Frobenius and signed control coefficients replace the fixed127constants. Intermediate u64products are safe on the tested cohort; arbitrary62bitprime arithmetic is not established.

The97failure disproves universal admission by the present nonvertical/squarefree/nonflex/Fy-unit origin policy in these coordinates. It does not prove that coordinate changes, infinity origins or repeated base divisors cannot supply a chart. The combined rejection reason leaves geometric cause classification open.

This removes a field restriction in the destination sections, but source kernels/isogenies, subgroup centering, algebraic inverses and new-field DLP remain unported. No timing, exponent, genericcurve or10×claim. Genus two/four and the full objective remain open; the prior failed charged coldcost comparison is unchanged.

All raw/stdout/stderr/status files, both protocols, source, locks and binary hashes are adjacent to RESULTS.md. No hidden curve substitution or silent scientific rerun occurred.
