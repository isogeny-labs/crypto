# Exact ±9 W1 divisor-map pilot

Frozen 15 November 2026 UTC before the external Magma calculator call. The sole public input is the already committed F128 curve `E: y²+xy=x³+x²+z³`, `z⁷+z+1=0`. The goal is to test whether Magma's documented `WeilDescent(E,k,c)` constructs an evaluable homomorphism and what target genus its chosen c yields. This is a map-construction control inside the known W1 family, not a new-family or attack-cost claim. No private point, scalar, repository source, or benchmark dataset is sent.

## Prior work checked

- [Magma Weil Descent handbook](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurvesOverFiniteFields/ec_weil_desc.html) documents `WeilDescentGenus` and `WeilDescent`, including the divisor-class map when the image is hyperelliptic. The parameter c can change genus.
- [Hess, *The GHS Attack Revisited*, Theorem 7 and §5.4](https://www.iacr.org/archive/eurocrypt2003/26560374/26560374.pdf) bounds the conorm/norm kernel and explicitly closes translated degree-two x-coordinate subfields as a new genus lever. Higher-index rational subfields remain a different construction with genus-growth bounds.
- The exact ±9 source and genus-seven isogeny-class target are in `../ghs_n7_base_change_20261115/`. The natural quotient study shows that its nonclassical W1 coefficient is outside the natural hyperelliptic-quotient locus. Equal Weil polynomials do not identify the endpoint of the Magma divisor map.
- KB revision `9141e386254340a9bdba3036f2bb706a` (last refresh 2026-11-14T21:44:48Z) returned the published W1 and Weil-restriction carrier notes; it predates the present q2/q4 study.

## Pilot and stopping rules

Submit one small Magma program to the public calculator endpoint, retaining the exact input, response and input SHA-256. Construct K with the specified polynomial, verify `#E=138` and ordinary j-invariant, then request `WeilDescentGenus(E,GF(2),1)`. Stop on field/model/order mismatch, transport failure, timeout at 120 seconds, or calculator error. If the pilot succeeds, search the 127 nonzero c values in a second separately frozen amendment before asking for a divisor map. A genus-seven result would be an explicit-map lead, not proof that it reaches the previously found Y; a genus-14 result is the published W1 baseline.

## Amendment A: local CA bundle

The first request did not reach Magma: the host Python SSL trust store rejected the server certificate chain. The input and transport error are preserved in `pilot_receipt_ssl_failure.json`. The earlier repository Magma runner uses the installed `certifi` CA bundle; retry the identical input with `ssl.create_default_context(cafile=certifi.where())`. Stop if this still fails; do not disable certificate verification.

## Amendment B: complete q2 c-parameter genus scan

The pilot matched the exact field and source order 138; `c=1` gives genus 63. Before the full scan, enumerate every nonzero `c=z^i`, `0≤i≤126`, because `K×` has prime order 127 and the nonidentity z has order 127. For each c, call the documented `WeilDescentGenus(E,GF(2),c)` and print `(i,g)`. Expected bounds from the prior W1 certificate and the genus-floor theorem: the minimum should be at most 14 and every nonzero algebraic target must have genus at least 7. A genus-7 row would be an explicit-map lead; genus 14 as the minimum would mean this documented parameter family does not reach the separate genus-seven isogeny-class carrier. Stop on missing rows, errors or a 120-second timeout. Only the public curve equation and this 127-row parameter loop are submitted.

## Amendment C: ineligible c values

The first scan stopped at i=1: Magma reports `GHS descent trace conditions are not satisfied by the data`. This is an expected parameter-level rejection, not a field/model mismatch. Preserve `scan_receipt_unhandled_c.json`. Continue the frozen 127-element scan with Magma `try/catch`, recording `ERR` for rejected c and checking that every exponent appears once. Do not count rejected c as genus measurements. If every parameter is rejected except c=1, report that exact outcome; do not infer a genus minimum for untested constructions.

## Amendment D: minimum-genus map evaluation

The corrected complete c scan returned 127 rows: 98 eligible, 29 trace-condition rejections, minimum genus 14 at exponents 94 and 98, and no genus-seven output. Before the map call, select `c=z^94`, the first minimum exponent. Construct `C, h := WeilDescent(E,GF(2),c)`, confirm `Genus(C)=14`, enumerate E(F128) to select the first point of exact order 23, evaluate `h(P)` and `h(O)`, and record output types/degrees. If C is hyperelliptic and h returns a Jacobian class, test exact order 23 by multiplication; otherwise retain the map evaluation and apply Hess's published kernel theorem only after checking its hypotheses. Stop at 120 seconds or on any mismatch. This does not test index calculus or rho.

## Amendment E: independent kernel-bound input check

The minimum-genus Magma map constructed a genus-14 plane curve and evaluated its divisor image at the first exact order-23 source point `(1:z^105:1)`. Its printed divisors do not establish divisor-class order. Before applying Hess Theorem 7, independently compute over the frozen polynomial basis the sigma-annihilator polynomials of `c=z^94` and `r/c=z^98`, where `r=sqrt(1/j(E))=z^65`. Verify that the lcm has degree four, that Magma's c is eligible, and that `j=z^124` is outside F2, so the source is not a subfield curve. If the kernel exponent is `2^4−1=15`, the 23-primary subgroup injects under the published conorm/norm map. Preserve the exact arithmetic output. This is a theorem-based subgroup check; it does not create Jacobian arithmetic for the genus-14 plane model.

## Amendment F: correct the Hess kernel exponent

The frozen Amendment E predicted `2^m−1=15` for m=4; that exponent was a mistaken reading of the number of nonzero Artin–Schreier equations. [Hess, *Generalising the GHS Attack*, Theorems 7 and 14](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf) instead gives `2^(m−1)=8` torsion for the kernel when the source is not a subfield curve and the rational-function base has trivial class group. The chosen source has `j=z^124∉F2`, so its `s(h)=7`; the order-23 source subgroup remains injective because gcd(23,8)=1. The correction is reflected in the recomputed kernel receipt and report. Preserve the original predicted number in Amendment E as part of the protocol record.
