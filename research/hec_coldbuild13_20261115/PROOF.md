# Recovering the construction from public inputs

For the source curve letL(T)=1+a1T+a2T²+a3T³+p*a2T4+p²*a1T5+p³T6.
N1,N2 determine a1=N1-p-1 and a2=(a1²-(p²+1-N2))/2. The elementary
symmetric Weil bound gives |a3|<=20p^(3/2). A nonzero point killed by the
verified primeell implies ell divides#Jac. If the resulting interval forL(1)
contains a unique multiple ofell, that multiple is the full Jacobian order
and givesa3. No storedL or extra subgroup order is needed in this fixture.
For quadratic field tau²=r0+r1tau, Norm(a+b*tau)=a²+ab*r1-b²*r0;
the quadratic character overFp² equals the Fpcharacter of that norm.

Squarefree Frobenius factorization mod3 into distinct dual irreducible cubics
of order13 gives two simple3-dimensional modules. Bezout idempotents project
the cofactored3torsion point onto them. Their nonselfduality, p=1mod3 and the
Frobenius-equivariant Weil pairing imply isotropy of each; size27 is maximal.
These checks and the recorded existing torsion13 proof justify the configured
kernel. No explicit kernel coordinates are read by the builder.

For the raw Weil set, isotropy makes its central extension split. A symmetric
splitting is unique: two differ by a character into mu3, and symmetry would
also make that character2-torsion, hence trivial. The order3and symmetry
constraints determine alpha²=A2,alpha³=A3. When consistent they give the
unique alpha=A3/A2 without extracting roots. Those constraints are checked
on every point; the previously proved divisors make the cocycles constants.
Only the necessary diagonal/opposite cocycles need evaluation, not the full
729pair table or four-sample control campaign.

The runtime8section functions lie inH0(2M), dimension8. Their full-rank36
quadratic evaluation matrix excludes any global quadratic relation, and thus
also excludes a linear dependence among them. They form the complete basis.
A decomposable ppav has Segre quadrics; a hyperelliptic genus3Kummer has its
nonzero quadric. Thus the quotient is neither. Oort–Ueno gives a smooth
nonhyperelliptic geometric genus3Jacobian. The unique rank7Gamma00line and
its nonzero smooth fourth jet identify its canonical quartic using the exact
characteristic0Fay/unit-minor specialization argument in geometry13. Functorial
reconstruction and pointcount sign supply same-field descent as before.
Cubic/Coble relation fitting is not logically required for this route.

The runtimeHC/QFand the first decoded source jet fix a common global sign.
The existing scalar-inversion transport and pair-line solve then apply.
Final source/destination scalar checks certify the executed result. This is a
fixed admitted fixture, not proof of a universal cold-construction success rate
or a speed gain. All constructions must be charged in a later benchmark.
