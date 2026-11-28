# Actual anchored full-jet linear operator and finite index sets

1. Structure: actual Polynomial.lsum/lcoeff/monomial and Taylor linear maps over FieldK. Target-position type Unit+(Fin3*Bool) includes one target sign and three opposite pairs, repeated abscissas explicitly. Full row index adds triangular mixed jets t+k<4.
2. Laws AS STATED: taylor_apply, lcoeff_apply, lsum_apply/sum_over_range, ofFn_eq_sum_monomial, finite LinearMap.pi/comp, earlier full mixed multiplicity. No new generic linearity induction.
3. Objects: coefficientJet=(lcoefft).comp(taylorx); xJetLinear assembled bylsum of monomial maps; mixedJetLinear applies yTaylor then kthcoefficient. Actual 7-position anchored map is a linear map from actual bivariate polynomials to70rows. Column index is j<8,i<16 withi+2j<16,72slots; coefficient injection not yet assumed.
4. Chain: prove map output equals exact previous xJet/full mixed coefficients. Operatorzero iff full multiplicity4 at target and both signs on all3other positions. Kernel-check finite cardinalities7,10,70,72.
5. Actual72coefficient-to-polynomial injection and finite72-to70matrix/kernel construction remain next; index counts alone are not the whole generator or an ECDLP relation.
6. Reuses actual library maps rather than informal linearity or a favorable supplied operator. SourceTaylor input pinned; no factorial or ordinary-derivative approximation. [pcc:unverified]
