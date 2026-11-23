# Proposed next construction: selected weighted orbit modes

Status: proposal to derive and check, not implemented, tested or a recovery claim. Do the prior-art/KB check first. The aim is to avoid compiling a large collection of cubic/higher-order features and, if valid, remove this triple feature's even-d restriction.

For h=(r-1)/2, x=(k+c)/b, v=x^m, and e in0..m-1, investigate the selected weighted sums

    F_e = sum_j s_j*zeta^((e-h)j).

The proposed coefficient decomposition is

    F_e = m*chi_r(b)*x^e*H_e(v),
    H_e(V)=sum_(e+tm<=h) (-1)^(h-e-tm)*binom(h,e+tm)*V^t.

The weight e-h is intended to cancel the kernel character even when chi(zeta)=-1. Check that sign, cyclic indexing and all zero cases directly; do not import the unweighted triple formula outside its domain.

First candidate image constraint: m*chi(b)*H_0(z/b^m)-F_0=0.
For m>=3, also test the division-free combination

    F_1^2*H_2(z/b^m)
      -m*chi(b)*F_2*H_1(z/b^m)^2 = 0.

This is intended to cancel the unknown x phase without selecting a root or using a discrete logarithm. Retain cases with zero modes: the second constraint can become vacuous. Do not assume a complete invariant or a nonzero spectral coefficient. Avoid division by an observed mode.

Candidate scalar-block compiler: for g_n=(-1)^(h-n)binom(h,n),

    g_(n+m)/g_n = (-1)^m * product_(i=0)^(m-1)(h-n-i)/(n+i+1).

For e=0,1,2 the starting coefficient is explicit. Evaluate numerator/denominator block polynomials at n=e+jm only while n+m<=h; no characteristic crossing should occur. This could replace the3x3 cubic compiler with scalar product/remainder trees. Derive cost and check every boundary before implementation. No measured improvement is asserted.

Required checks: independent dense binomial expansion; all scalar residues for small primes with both parity classes of d; nonsquare radius/sign controls; mode zeros; comparisons on the retained reciprocal-collision examples; existing public source and full candidate-point verification. Keep field work, source calls, setup, modes and storage separate. The source still needs m signs per orbit unless a separate source construction improves it.
