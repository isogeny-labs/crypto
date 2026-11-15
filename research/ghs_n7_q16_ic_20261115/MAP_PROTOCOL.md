# Frozen q16 n7 selected-subgroup map gate

15 November 2026. Freeze after the [128-row order screen](SCREEN_REPORT.md), before the public Magma map call. This is a direct GHS-map check in Teske's published genus-eight selector, not an attack-cost measurement. [pcc:unverified]

Use `K=F2[z]/(z^28+z^3+1)`, its unique F16 subfield, and `E:y²+xy=x³+a x²+b`, where polynomial masks are `a=65712646`, `b=406166`. The selected source point has coordinate masks `x(P)=179462209`, `y(P)=218074333`; the order-screen receipt predicts `#E(K)=268413186=6·44735531` and the Sage point check gives exact order `r=44735531`. Let the public validation target be `Q=1234567P`. [pcc:unverified]

One Magma 2.30-1 script checks coefficient subfield and full-absolute-field properties, source order and primality, `WeilDescentGenus(E,F16,1)=8`, a genus-eight hyperelliptic target, nonzero `G=h(P)` of exact order r, and `h(Q)=1234567G` plus homomorphism checks at two and three. Print the exact source point, target curve, Mumford image and timing, preserving input SHA-256 and the raw service response. If any check fails or the 180-second client cap is hit, retain the failed receipt and stop before relation collection. The validation scalar is a fixture; any later attack script may use it only in final verification. [pcc:unverified]

Prior work: [Teske's family](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf), [Magma Weil-descent API](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurvesOverFiniteFields/ec_weil_desc.html), and KB revision `9141e386254340a9bdba3036f2bb706a` (source `source:8618328474ca57d3a61b1a8832d41daf`). The bounded exact-parameter search in the screen report found no priority proof.
