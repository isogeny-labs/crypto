# Additional unprojected addition ANF component

Frozen prospectively, before `unprojected-anf` trials. This adds a narrower
component of item4 and preserves the original four-presentation comparison.

On the same K0/GF(2^7) polynomial basis and exact `(m,ell)=(3,4),(4,3)`
factor bases, construct Boolean ANFs for finite generic addition and finite
doubling. Each field coordinate has seven Boolean variables. Include curve
membership equations, slope equations, output equations and an inverse
witness equation `denominator * witness = 1`; this excludes spurious roots
on the zero-denominator stratum.

Generic variables: x1,y1,x2,y2,x3,y3,lambda,w (56 Boolean variables).
Doubling variables: x1,y1,x3,y3,lambda,w (42 variables). Identity, inverse
and 2-torsion cases retain separate certificates from the first component.

For every applicable ordered factor-base pair, fix its coordinates and
the uniquely defined nonzero-denominator slope/inverse witness. Enumerate
every finite point of the complete curve for the output. Require that the
zero set of the Boolean ANF equations equals the independently computed
group-law output, exactly one solution. Retain checked pair/output counts,
input maximum degree, monomial counts, complete ANFs and certificate hash.
Abort on discrepancy and preserve unsuccessful output. This is an exhaustive
toy graph-equivalence check, not a measured F4 solving degree or a complete
four-presentation m-sum comparison.
