# A global supported-unit lattice alongside pole-bound enumeration

PROPOSED, not executed: compute an explicit full-rank subgroup of the units
supported at the ten closed degree2 cusps, then restrict to the four infinity
points used by the404/818cohorts. This can produce exact principal divisor
patterns directly instead of increasing a coefficient search bound.

## Explicit F2 units from the original nu divisor
R27 proves original nu has only cuspidal zeros and poles. On the original
F4 chart sigma(nu)=nu+b. Its norm q=nu*(nu+b) has no noncuspidal support
and is sigma-invariant. If the F2 descended coordinate is n=nu+a*b with
a²+a+1=0, the ACTUAL F2 formula is q=n²+b*n+b², not n*(n+b).
Native source and localseries must certify this coordinate bridge first.
The inherited R27 valuations give q, in cusp orderO,A,C,D,B:

| Point type | O | A | C | D | B |
|---|---:|---:|---:|---:|---:|
| e7 |53|-102|21|39|19|
| e1 |0|-24|0|0|-6|

These numbers are derived by adding nu's two sigma-conjugate orders; they
are predictions for the actual norm function, not newly measured values.
Their total is0 with equal degree2weights. T commutes with sigma and its
fifth power is sigma, so the five T translates of q descend and give five
explicit cuspidal units. Verify every whole-field iterate and all local
valuations; do not replace a descended function by an original one.

Add four explicit base elliptic Miller units with divisors5(P)-5(O) for
nonzero rational5torsionpoints. First bind cusp order to actual basepoints,
verify5P=O and complete Miller divisors, and pull them back using e7/e1.
This gives nine known units. Their expected independent valuation span has
rank9 on ten cusps; measure the rank and full integer certificates.

## Exact integer elimination and even divisors
Use HNF/SNF with exact integer transformation matrices. Cancel all six
unwanted cusp rows. The complete integer kernel gives all units IN THIS
GENERATED SUBGROUP supported on the four allowed points, with explicit
unit-exponent words. Add even-valuation constraints via four auxiliary
integer variables V_remaining*a=2e. Compute the complete lattice of signed
ramification patterns e, not a coefficient box. Retain unimodular transforms,
all source valuation products, lattice rank/index and changed-column controls.
Rustcheckedintegerarithmetic or native FLINT via Rust/Singular is permitted;
freeze sources/resources/overflow handling before data. Do not use Python
for the mathematical lattice computation.

Every F2 rational function supported on the four sigma-stable closed points
is sigma-fixed: sigma(f)/f is a constant and its square is1. Thus the exact
supported-unit problem descends to the genus31 quotient, potentially reducing
the reconstruction problem. The finite-index subgroup is not automatically
the full unit group; its lattice index/regulator bounds the saturation primes.
This unit-lattice saturation is DISTINCT from projective/incidence quotient
saturation, which remains a separate obligation.

## A direct link to the Cartier construction
For the2-saturation, products of known units are squares exactly when their
logarithmic derivative vanishes, over the perfect constant fieldF2. Hence
solve the small F2 kernel of the whole-source dlog(unit) columns and extract
actual square roots from the exact field square decomposition. Verify each
root in the full field and its complete divisor before updating the lattice.
Odd-prime saturation requires separate exact Kummer-root/divisor tests.

The resulting explicit unit generators can feed all31F32scalar andboth
ordinary-sign/trace-parity AS tests once polebounds are adequate. A principal
unit is not an elliptic map; preserve whole-source identities, all5conjugates,
F2descent and intended11action. Positivecontrols are the constructed units;
R59/R60 complete excluded cohorts provide consistency checks on any claimed
small supported divisor. The complete cubic/818andtrace-dual/Cartierlanes
remain active independently.

Sources checked: R27CM complete nu divisors/actualsigma/freequotient,
R26base-cusp divisors, R50-R52descendednormal coordinates, R60principal-
divisor exclusions. The next native step must bind q and T translates in
actual source coordinates before integer-lattice conclusions are reported.
