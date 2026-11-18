# Exact covariance of a common reciprocal Richelot chart

Written algebraic argument, not a new Lean proof or an executed divisor map.
Let k have odd characteristic, q_i be monic quadratics over a finite extension,
Delta the determinant of their ascending coefficient rows, and
f=lambda*product(q_i), H=product(B_i)/(lambda*Delta^3), with nonzero lambda,
Delta. Indices in B_i=q'_(i+1)q_(i+2)-q_(i+1)q'_(i+2) are cyclic.
Assume f/H are squarefree sextics and a,b,c in k satisfy b*c nonzero,
b^2=f(a), c^2=H(a). [pcc:unverified]

Put x=a+1/w, r_i(w)=w^2 q_i(x), alpha_i=q_i(a), S=product(alpha_i).
Since lambda*S=b^2, every alpha_i is nonzero. Writing q_i=x^2+l_i*x+m_i,

    r_i=1+(2a+l_i)w+(a^2+l_i*a+m_i)w^2.

The linear transformation on ascending coefficient rows has matrix

    [0 0 1]
    [0 1 a]
    [1 2a a^2]

of determinant -1. Thus for the monic Q_i=r_i/alpha_i,
Delta'=det(Q_0,Q_1,Q_2)=-Delta/S. Also product(Q_i)=w^6*f(x)/b^2=F.
The roots of Q_i are the two reciprocal shifted roots of q_i. These claims
hold over the extension before any coefficient descent. [pcc:unverified]

The chain rule gives r'_i=2w*q_i(x)-q'_i(x), because dx/dw=-1/w^2.
The 2w terms cancel in the Wronskian, hence

    B'_i=Q'_(i+1)Q_(i+2)-Q_(i+1)Q'_(i+2)
        =-w^2*B_i(x)/(alpha_(i+1)*alpha_(i+2)).

Multiplying the three equalities and dividing by Delta'^3 gives

    product(B'_i)/Delta'^3
      =S*w^6*product(B_i)/Delta^3
      =b^2*w^6*H(x)
      =(bc)^2*G(w),     G=w^6*H(a+1/w)/c^2.

The leading coefficients of F/G are respectively f(a)/b^2=1 and
H(a)/c^2=1. Therefore the raw new Richelot target has the explicit square
leading coefficient (bc)^2, the acceptance condition of the existing monic
constructor. No twist is chosen by matching orders. [pcc:unverified]

If the q_i are cycled by Frobenius and a lies in k, the alpha_i and Q_i are
cycled in the same way. Their products and the normalized target coefficients
descend. A permutation of the factors multiplies Delta by its parity; cyclic
brackets acquire the same parity together with a permutation. Both cubed
signs cancel in the target recipe. All six reorderings are legitimate.
Changing either b or c changes a chosen ordinate normalization sign while
leaving F/G unchanged; the exact evaluator's dual[2] sign is a further check.
[pcc:unverified]

The affine substitutions z=y*w^3/b and v=Y*w^3/c have explicit inverses.
At the reciprocal pole use t=1/w=x-a and z/w^3=y/b (and the analogous
target formula); at old infinity use s=1/x and w=s/(1-a*s). These give
regular local inverses on the smooth complete curves. This also covers
nonrational old infinity points: the maps are defined over k and commute
with Frobenius. Galbraith corrected8.3.8/10 then gives isomorphisms of Pic^0.
The construction therefore supplies k-model charts into the conditional
classical Richelot correspondence. Actual evaluation of that correspondence,
exceptional fibres and its oriented dual remains task14. [pcc:unverified]

Inheritance: unchanged FpPoly supplies polynomial arithmetic/Euclidean
inversion; unchanged CubicField supplies the checked irreducible cubic field,
Frobenius, products, determinant and Richelot brackets. The new unit compares
their outputs with this coefficient-transform identity and the frozen CT49
target literals. It is the same author using a reused backend, not a trusted
independently authored proof verifier. No generic field instance is asserted.
[pcc:unverified]
