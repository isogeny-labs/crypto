# A specific explanation for the 496 cubic false positives

This is a new falsifiable hypothesis, not an executed fourth experiment.
Let K=GF(32), fix quintic coefficient h4=t, and let Q=X^2+uX+v. Then

    (X+t)Q^2 = X^5+tX^4+u^2X^3+t*u^2X^2+v^2X+t*v^2.

Hence H belongs to this squared-quadratic family iff h2=t*h3 and h0=t*h1.
The unique Q is recovered by u=sqrt(h3),v=sqrt(h1); square roots are unique
and linear over F2. These are exact identities for every characteristic-two
finite field, retaining all repeated roots. No analogous omission of cross
terms is valid in odd characteristic.

If u=0, Q splits with a double root. If u!=0, Q splits over K iff
Tr(v/u^2)=0, equivalently Tr(sqrt(h1)/h3)=0. This follows by substituting
X=uZ into Q and solving Z^2+Z=v/u^2. The trace-zero image has q/2 elements.
Thus there are q(q-1)/2=496 irreducible monic quadratics over GF(32), and
H=(X+t)Q^2 is an injective map from these to 496 nonsplit quintics.

The measured cubic zero set has exactly496 false positives for each of the
six fixed-trace GF(32) tables. All eight recorded examples per field satisfy
the displayed coefficient pattern by inspection. Neither count nor sampled
pattern proves that every false positive has this form. The next experiment
must compare the complete masks and check both inclusions. Do not claim the
guarded predicate exact until that certificate passes.

Conditional theorem: if the measured false-positive mask equals precisely
these496 H values, then the old full splitting predicate equals

    all_cubic_relations_zero AND
    ((h2!=t*h3 OR h0!=t*h1) OR Q_splits).

No inversion is evaluated in a branch with h3=0; it uses the proved split
case directly. A 10-bit lookup for Q splitting or a linear-square-root,
inversion and trace circuit can be measured separately. Both retain the
same global76 parameters. Full prefix support and all original source/U
recovery remain separate requirements. A local cubic+guard replacement
does not establish the whole339-variable solve or an end-to-end speed gain.

The next compilation should reuse degree2/3 monomials and compare total XOR,
AND, construction time and process RSS with C101. Dense basis coefficients
may offset the reduced algebraic degree. Use Frobenius/trace normalization
and weight-reduced relation bases only after independent equivalence checks.
