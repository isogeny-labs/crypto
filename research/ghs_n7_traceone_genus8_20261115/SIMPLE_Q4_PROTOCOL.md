# Frozen q4 Weil-restriction simplicity check

Frozen 15 November 2026 UTC after the successful order-8,317 Magma map receipt, before the polynomial factorization run. The purpose is a rigorous genus floor for the exact q4 coefficient set, separate from any index-calculus cost claim.

## Prior theorem and predictions

The local `research/ghs_n7_base_change_20261115/GENUS_FLOOR.md` proves the corresponding Capelli/Honda–Tate criterion for the ±9 isogeny line and cites [Stevenhagen, Algebra III, §23](https://pub.math.leidenuniv.nl/~stevenhagenp/GaloisTheory.pdf) and [Oort, Abelian varieties over finite fields, §5](https://math.nyu.edu/~tschinke/books/finite-fields/submitted/oort.pdf). For an ordinary E/F_(q^7) of trace t, the Weil restriction to Fq has characteristic polynomial `X^14−tX^7+q^7`. Irreducibility over Q implies that this seven-dimensional restriction is simple. Any nonzero algebraic homomorphism from it to a Jacobian then requires target genus at least seven.

For the exact q4 representative of `MAP_Q4_PROTOCOL.md`, t=−249, so the polynomial is `X^14+249X^7+16384`. Its quadratic subfield discriminant is `t²−4·4^7=−3535=−5·7·101`. If a Frobenius root were a seventh power in this imaginary quadratic field, its seventh root would be an algebraic integer of norm four. The only norm-four integers in this field are `±2`, whose seventh powers have trace `±256`, not −249. The predicted polynomial is irreducible.

Factor the exact degree-14 polynomial for every q4 trace stratum in `strata.json` over Q. Record the factors and an `irreducible` flag per stratum; require irreducibility for every stratum with a full-absolute-field coefficient. Reconcile the number of such coefficients with 364 and preserve all factorization results, including proper-field strata. Stop on any disagreement. This finite check supports, but does not replace, the paper-level genus-floor argument and does not establish lower attack cost than rho.
