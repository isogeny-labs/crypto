# Streaming character-product traces

Validation02 reproduces7965 coefficients across45 polynomial powers,90 character
traces and60 precision-loss steps. The largest retained coefficient window in
these kernel controls is8; their largest modulus is6234839521. A Frobenius
counterexample demonstrates why the first-order equation directly modulo r is
insufficient with only constant/leading coefficient and degree data.
[pcc:unverified]

Of36 word systems,27 return exact moments matching the explicit endpoint-corrected
projector;14 singleton phases are checked. Nine refuse: six at subset preflight,
three after499728 steps each at the next polynomial's step preflight. Successful
cases use1265016 steps/15580180 modular products; refused cases add1499184 steps/
20402004 products. Combined:2764200 steps and35982184 modular products. All nine
refusal rows are retained in validation02/check.stdout and receipt.json.
The retained recurrence window reaches12 entries in this panel. [pcc:unverified]

Public29 fixtures return12 extracted and verified scalars,11 identity/endpoints
and6 explicit subset-cap refusals. Nominal source calls332,guard calls27,total359
excluding setup. They use2654208 recurrence steps,38436864 modular products,
49152 attempted subsets and19844 endpoint-correction field multiplications.
These counts include capped public cases' source and completed correction work.
The modular products can be modulo powers of r; they are not the old field
monomial-pair metric. [pcc:unverified]

The new extractor retains a degree-sized rolling window and two trace sums,
without dense phase-quotient coefficient vectors. The source, input subset
polynomial, scalar phase circuit and endpoint map remain. Modulus precision is
also storage: general bit memory scales with degree times precision, not just
number of field elements. The exact caps are500000 recurrence steps,4096 subsets,
modulus<=i64MAX,100000 circuit nodes and100000 correction multiplications.

Large r998244353 controls reject the step budget and precision modulus before
recurrence work; no large-instance unknown image is extracted. A nonunit inverse
is refused. No native compile/assertion failures. Validation01 and its altered
checks snapshot are retained; validation02 adds failed-case operation rows only.
Twelve copied sources/16 compiler dependencies are hash-bound. Original4563-case
cohort and preceding adapters unchanged. [pcc:unverified]

This is a state-representation construction, with linear coefficient stepping
and an explicit exponential subset expansion. No timing or total-work ranking.
The fixed-observation information bound2^N>=L-B is derived for universal
word-only image identification after at most B endpoint images are excluded;
it does not bound other observations or adaptive algorithms. [pcc:unverified]

Next: structured matrix products for the coefficient steps, and a complete-word
trace representation avoiding subset expansion. Accelerating one subset alone
does not remove2^N subsets. See DERIVATION.md for the established recurrence,
precision invariant, proof, exact scope and primary literature. Independent signed
review pending; no novelty claim or cloud work.

Claim: PCC-20261108-d2f2f7, unsigned producer evidence. [pcc:unverified]
