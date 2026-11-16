# Degree-seven cover at s=7: frozen transfer and cost protocol

16 November 2026. Reuse the exact mask-30 degree-seven cover of
`research/ghs_odd5_carrier_20261115/degree7_quadratic_square_30.json`.
Set `K=F3[a]/(a^5+2a+1)`, `L=F_(3^35)`, `k=F_(3^7)`,
`E/L: v^2=u^3+2u^2+a+a^3`, and
`Y/k: y^2=x^11+x^9+2x^8+x^7+2x^5+x^3+x^2+x+1`.
The recurrence from the base trace 26 gives
`N=#E(L)=50031545359078046=2*29*109*7393*1070455751`.
Select `r=1070455751` and `h=N/r`. Use a fixed Sage random seed to
construct `P=hR` of order r and `Q=123456789P`.

First independently evaluate the corrected pullback and five-term
`Frob_(3^7)` trace for P, Q, and 37P. Require nonzero order-r images,
Frobenius fixedness, and both scalar identities. Save full field moduli,
embeddings, point and divisor coordinates, source hash, and the exact
cover identity. Preserve any failed/censored attempt without replacing it.

The complete benchmark, if feasible, charges small-field setup, cover
transfer, factor-base construction, relation generation, modular linear
algebra, target descent and final verification. Compare with an order-seven
F_(3^5)-Frobenius-quotiented rho on the same saved P,Q; charge source setup
and at least 30 independently seeded, verified rho solves. Record CPU and
wall time separately. A partial relation count or transfer timing is not
an attack total. Include caps and resource failures as censored evidence.
The existing s=3 result is a separate scale and cannot be pooled with s=7.

Prior-work boundary: the cover is the same one-orbit exact construction
already recorded in `DEGREE7_FINDING.md`, and odd-characteristic GHS,
weak-covering and isogeny extensions predate it. This run cannot establish
a new curve family or method. An attack advantage requires a measured full
same-target total below the matched rho total.
