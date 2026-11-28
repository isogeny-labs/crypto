# Direct sign-predicate source packing screen

1. Structure: actual packet-padding endpoint of one Unit-valued predicate, natural source parameters a,u,groups,Delta,M; number of sign ports signs.
2. Laws as stated: real_endpoint_degree165, Fintype.card_subtype_true; Nat.lt_two_pow_self, Nat.pow_le_pow_left/right, Nat.le_mul_self, Nat.le_div_iff_mul_le. Existing arithmetic and finite-instance laws only.
3. Object: all original real ports meet the one predicate vertex; new degree-one dummy endpoints remain distinct. Darr=groups^(2*a), source D.6 uses2*Darr^2*Delta<=M.
4. Chain: actual endpoint degree=signs; signs<=Delta and u<=a imply signs*u<=M via u<=Darr<=Darr^2. Positive u gives signs<=M/u; fieldBits<=signs transfers this bound.
5. Missing endpoints only. Conditional on this direct representation and actual source packing. Not a runtime lower bound, not a universal obstruction, no claim every ECDLP reduction has high degree or that total description bits equal structural size.
6. Primary D.1/D.4.2 and NEXT_FINDER.md read. Full squared packing contract retained. New low-degree public reduction or actual counted branch selection may avoid this direct high-arity cost screen. [pcc:unverified]
