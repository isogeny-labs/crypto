# Actual residual fractional unit and invariant class

1. **Actual objects:** same W.CoordinateRing/functionfield, actual nonzero h and actual nonsingular target point ideal unit u.
2. **Construction:** public toPrincipalIdeal applied to actual mapped nonzero h gives principal unit; define actual residual unit principal(h)*u^-4.
3. **Integral binding:** actual exactspanh=I^4J derives residual unit coercion=actual J by public coeIdeal product/power/span and actual unit cancellation. No J-unit/invertibility/class witness supplied.
4. **Class:** derive principalclass1 using public actual ClassGroup.mk_eq_one_of_coe_ideal, then residual class=(targetclass^4)^-1, independent of chosen nonzero h. Nonsplit J allowed.
5. **Limit:** no residual field splitting/point extraction/log labels, varying h does not change this class but no universal algorithm barrier/existence claim. ActualJ binding conditional on actualparentfactor already constructed; no scalar/publicexpectedcost result.
6. **Sources:** Mathlib ClassGroup.Basic toPrincipalIdeal/coe_toPrincipalIdeal/mk_eq_one_of_coe_ideal, FractionalIdeal.Operations coeIdeal_span_singleton, actual public XYIdealprimeunit; exactsource+compiled pins. These generic laws already exist; this is actual coordinate-ring application, not a missing reusable owner lemma. Defaultsingle readonly check; KB unavailable asrecorded.
