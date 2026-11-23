# Actual curve formal branch binding

1. FieldK, actual f in K[X], actual x,y with y²=f.evalx,2nonzero,ynonzero; no cubic degree required for this local branch.
2. Existing laws as stated: committed/audited owner b3ce082 PowerSeriesSquareRoot existence/linear-tail from actual constant square/unit2y; Polynomial.hom_eval₂, constantCoeff_comp_C/map_add/constantCoeff_C/X, eval₂_id, isUnit_iff_ne_zero and mul_ne_zero.
3. Objects: actual translatedCurve f x=f.eval₂(PowerSeries.C)(PowerSeries.Cx+PowerSeries.X). Its actual constant coefficient and actual s/u, not assumed local branch witness.
4. Chain: hom_eval₂ derives constantCoeff(translatedCurve)=f(x); actual point gives y², actual nonzero2y gives unit. Apply actual generic owner endpoints to derive s²=translatedCurve with constanty and actual(Cy+T*u)²=translatedCurve.
5. No actual helper substitution/full4->order4, norm root/divisor/scalar/efficiency or growing-finder cost conclusion inferred.
6. Only actual curve instance of owner generic laws; declared2new public imports, inherited051body exact after removing only these imports. New50fileoverlay preserves old45bytes; no private override/trust/resource change. [pcc:unverified]
