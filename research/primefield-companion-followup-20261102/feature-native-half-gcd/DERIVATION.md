# Half-GCD batches with projected-resultant metadata

The backend now batches Euclidean transitions on truncated high coefficients, lifts a polynomial matrix to the full inputs, and applies resultant factors without replaying full-size Euclidean divisions. The field, quotient-class representation and geometry finalization are unchanged.

## Forward matrices and lifting

A division A=QB+R acts by N_Q=[[0,1],[1,-Q]] on the column (A,B). Its determinant is -1. A batch matrix is the left-ordered product of these forward matrices. For canonical inputs with deg(A)=d>deg(B), half_gcd returns consecutive remainders (C,D) satisfying deg(C)>=ceil(d/2)>deg(D), allowing D=0.

Below degree cutoff8, ordinary divisions build the matrix until this threshold is crossed. Otherwise, recurse on A div X^h,B div X^h with h=ceil(d/2), and apply the resulting forward matrix to the full inputs. If the lifted second remainder already crosses h, return. Otherwise perform one full bridge division. If needed, recurse again on the high parts of that pair, shifted by 2h-deg(B), lift, and compose the matrices in forward order. The implementation applies matrices to full polynomials directly; it does not reconstruct them by replaying each quotient at full size.

The standard lifting invariant bounds matrix-entry degrees by the degree lost by the leading remainder. Therefore low-part perturbations cannot change the leading degree/coefficient of the first returned remainder or any earlier divisor. The lifted pair still has strictly decreasing degrees. Polynomial quotient uniqueness then makes the batch the corresponding prefix of the full Euclidean sequence. This is the usual half-GCD lifting argument; the implementation checks the returned leading divisor against its metadata and checks the threshold/degree order.

## Correct resultant factors

Each transition stores (m,n,k,c): dividend degree, divisor degree, optional remainder degree, and divisor leading coefficient. Its factor is (-1)^(mn)c^(m-k) when the remainder is nonzero. Under a lift by sigma, add sigma to m,n and to every nonfinal k. All stored leading coefficients remain unchanged. The final k is replaced by the degree of the actual lifted second remainder; a truncated zero can become nonzero and vice versa. This final correction is mandatory before factor accumulation.

After all lifts, the records describe actual full-field Euclidean transitions. Their degree-dependent signs and leading factors are accumulated in ResultantClass, reducing exponents modulo q in {2,4}. A zero remainder at positive divisor degree returns the zero class. A constant divisor contributes c^m. Equal-degree and initially reversed inputs use the explicit full-division/swap branches. No sign or exponent is computed from an uncorrected final truncated degree.

Projection remains at the geometry boundary, after main/tail/correction composition. This preserves the single-projection identity established by feature-native-resultant-class. No full resultants or scalar labels enter the query path.

## Work and limitations

The recursive shape is two half-size calls plus polynomial matrix products/application and a bounded number of full divisions per level. The transition vector adds linear work per level; it is not a full-size division replay. This supplies the standard half-GCD batching mechanism, but elapsed-time superiority over ordinary Euclid or the earlier evaluation-tree backend is not measured. Cutoff8 is a development choice, not a tuned optimum. Polynomial multiplication/division and matrix allocations remain. The explicit geometry point-list and dense decoder-construction bottlenecks remain as recorded previously.

## Validation and sources

960 deterministic polynomial cases over F5,F7,F13,F29,F509, degrees2,3,7,8,9,15,16,17,31,32,33,64,65,127,128,129, three divisor-degree choices and four coefficient patterns per combination, compare every transition with a separate schoolbook division trace. They check full returned remainders, matrix action, determinant (-1)^steps, and projected resultants against the prior Euclidean implementation. The panel exercises1679 recursive calls and17 changed-final-degree lifts. Geometry exercises174 recursive calls. These are development coverage counts, not performance measurements. [pcc:unverified]

Read the preceding quotient-class/Euclidean implementations and FLINT's primary half-GCD documentation, including the lifting and resultant sections: https://flintlib.org/doc/algorithms/polynomials/half-gcd.html . Standard half-GCD is not claimed novel; this implementation carries the projected-resultant metadata through its lifts. KB unavailable: no loaded tools and wrapper executable absent. Prime fields only, caller-certified primality and inherited word bounds; projection orders above four rejected. Original cohort and historical outputs unchanged.
