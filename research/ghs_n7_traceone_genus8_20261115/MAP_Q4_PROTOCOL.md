# Frozen q4 genus-eight subgroup-map check

Frozen 15 November 2026 UTC after `STRATA_PROTOCOL.md` and `strata.json`, before the public Magma call. The selected equation is inside Teske's published classical n=7 genus-eight family. This check asks whether its 8,317-order subgroup survives the explicit GHS map; it is not an index-calculus cost run or a new-family claim.

## Exact source and prior work

Let `K=F2[z]/(z^14+z^5+1)`, and embed `k=F4` into K. Use `E:y²+xy=x³+a x²+b` with `a` polynomial mask 5107 (`1+z+z⁴+z⁵+z⁶+z⁷+z⁸+z⁹+z¹²`) and `b` mask 483 (`1+z+z⁵+z⁶+z⁷+z⁸`). The Sage stratum receipt predicts `a∈k`, absolute trace of a equal to one, full absolute F2 field of definition for b, source trace −249, and `#E(K)=16,634=2·8,317` with 8,317 prime. Its trace is outside the two seventh-power traces `{87,251}` of the trace-one base-field isogeny classes.

[Teske's presentation](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf) supplies the known component family. [Magma's Weil-descent handbook](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurvesOverFiniteFields/ec_weil_desc.html) documents `Embed(k,K)`, `WeilDescentGenus`, and the divisor-class homomorphism on a hyperelliptic target. KB searches for these exact masks, source order, and trace found no prior exact equation, but this bounded search does not establish publication priority.

## Frozen checks and limits

Submit one public Magma 2.30-1 script with only the displayed curve and a 120-second client timeout. Verify the field and coefficient identities, prime factor, source cardinality, `WeilDescentGenus(E,k,1)=8`, a hyperelliptic target of genus eight, and a nonzero image of an exact order-8,317 source point. Verify `8,317*h(P)=0` and `h(7P)=7h(P)` and recover scalar seven by scanning target multiples zero through ten. Preserve the exact script hash and full response or failure. Stop on any failed assertion or timeout. Service time measures this construction/check only; it cannot be compared with rho as a complete attack cost.
