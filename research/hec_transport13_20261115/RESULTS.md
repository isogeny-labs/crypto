# Reused destination Jacobian arithmetic passed

Source f72b5fb5f004 plus lockfile6c82945ab581 was uploaded before execution:
s3://setec-astronomy-artifacts/checkpoints/crypto/research__hec-principal-relations-20261114/20261115T155620Z-6c82945ab581/.
One registered release run exited0, stderr empty. Local correctness checks; no timing
comparison, independent implementation or complete isogeny evaluation is asserted.

| Curve | measured N1 | measured N2 | registered annihilator | prime-order controls |
|---|---:|---:|---:|---|
| Q0 |127|16043|2027974|not requested|
| Q1 |129|16043|2057786|2 nonzero classes of order1028893|

Both curves passed two identity, inverse, doubling and annihilator checks each,
one commutativity and one associativity check each. Four class encodings and both
prime-order encodings are retained in RUN.txt. Counts use the pinned existing
extension-field root-gcd enumerator. The Jacobian law uses its existing
section-space product/colon/flip implementation. All tests are on fixed triples;
there was no retry, replacement, adaptive fixture search, timeout or cap.
The doubling check compares two uses of add_raw and is a wrapper control only.
The cumulative field-multiplication diagnostics include setup/counting/prior arms;
they are not separate group-operation costs or performance measurements.

## Explicit post-run protocol erratum

PROTOCOL.md mistakenly writes a1=128-N1. In the L-polynomial convention with
positive coefficient a1, the correct identity is **a1=N1-128**. The sign error is
only in the explanatory paragraph: frozen expected counts, annihilators and all
executable checks are unchanged. The raw protocol is preserved and not rewritten.
For both curves a2=(a1²-(16130-N2))/2=-43. For Q1, a1=1. Thus
L(1)=2048384+16130a1+128a2+a3=2059010+a3.
The registered order2057786 corresponds to a3=-1224. The interval
|a3|<28625 has width<57250<1028893. Prime checking and two nonzero
classes annihilated by1028893 prove that this is the unique possible order
inside the interval, conditional on the reused arithmetic being correct.
It also determines the derived L-polynomial
[1,1,-43,-1224,-5461,16129,2048383], matching the prior geometric prediction.
For Q0 the selected classes pass the registered annihilator check; this run
alone does not establish its full group order independently.

## Remaining transport construction

A correct destination group law and matching prime subgroup do not identify the
images of source public divisors. Existing quotient coordinates land in the
Kummer variety in its theta-section basis. Next identify the target Jacobian's
Kummer coordinates in that basis and lift to divisor classes, allowing a global
sign with final source verification (Tian §4.2, Remark4.3). Do not assign images
by matching discrete logarithms or public-multiple labels. Required controls:
kernel annihilation, addition up to consistent sign, source public-input images,
and rationality over F127. Charge alignment, lifting, exceptional charts and
verification before making a complete-cost comparison. Genus2/4 work remains
within the broader goal; this gate is specific to the genus3 construction.

No existing closed record, default, canonical plot or leaderboard changed.
QUARTICS.svg in geometry13 remains the correct geometric trace plot; this study
adds COUNTS.svg for the two newly counted finite-field extensions. This is a
correctness gate, so no performance benchmark record is created.
