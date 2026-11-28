# Strict quadratic homogeneous rational interpolation test

1. Structure: arbitrary quadratic auxiliary q0(X)Z^2+q1(X)YZ+q2(X)Y^2 over FieldK, char!=2; actual support |S|=2a,a>=2 and nonzero paired branches ±z.
2. Existing laws: degree_eq_natDegree/degree_eq_bot and finite root/nodal divisibility from NormSignedHankel.nodalDivides and Polynomial.natDegree_le_of_dvd. Field arithmetic only; no new generic root-count or multiplicity induction.
3. Exact budget: every coefficient's degree plus weight for (1,a,a-2) strictly below2a, with zero coefficient degree bottom. This is exact coefficientwise weighted-degree condition for this displayed homogeneous quadratic, not assumed output q2=0.
4. Chain: actual budget forces q2=0, degq1<2,degq0<4. Opposite branch vanishing with2!=0,z!=0 forces each q0,q1 evaluation0. At least4distinct abscissas then root bound forces q0=q1=0.
5. Only quadratic homogeneous auxiliary, equal multiplicity1, natural weights and both branches. No all-degree/all-multiplicity decoder or general ECDLP obstruction; not a finder.
6. Literature Theorem6strict condition motivated checking an independent auxiliary instead of the failed fixed curve equation. Curve polynomial has equality and is excluded by strict budget; all other quadratic choices are explicitly tested here. [pcc:unverified]
