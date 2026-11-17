# Polynomial image inversion over the base field

The prototype now converts newly constructed quotient images into actual
Jacobian divisor classes and completes two small ECDLP reductions. It uses no
published numerical isogeny map and does not enumerate the target group.
The sampled setup also avoids enumerating curve points over F_(p^3).

The construction follows the normal-Weil-function and symmetric-section
framework of [Tian](https://arxiv.org/abs/2012.07173). The practical refinement
tested here is the use of a rational **odd** theta characteristic and a rational
tangent chart, which puts the final polynomial system over F_p. Novelty of this
specialization has not been established.

## Why change the reference theta characteristic?

The natural even theta characteristic delta_e supplied by the gluing is usually
not F_p-rational. For the prime-order cubic cases considered here, there is a
unique F_p-rational bitangent. Its contact divisor delta_o is an odd rational
theta characteristic.

Choose a rational curve point O away from that bitangent. Then

    j_o(D) = [D-delta_o-O]

is a rational degree-three Abel-Jacobi parametrization. A generic rational
Jacobian class has its effective degree-three representative defined over F_p.
With delta_e as reference, that representative would generally be defined over
an extension even when the Jacobian class itself is rational.

The change is an explicit translation by

    tau = delta_e-delta_o in J[2].

The bitangent's characteristic label identifies the corresponding two-torsion
point on the source Weil restriction, so the source evaluation at R+tau is
computable directly from the existing theta tensor.

## Normalization constants cancel

For a two-torsion class D_i, obtain a raw Jacobian Weil function f_i on triples
of curve points. It differs from the corresponding quotient function only by
a constant. Evaluate it at the known calibration class tau. For a source point R,

    f_i(j_o^(-1)(pi(R))) / f_i(j_o^(-1)(tau))
       = h_i(R+tau) / h_i(0).

Thus the separate normalization constants are unnecessary for image inversion.
The calibration divisor represents delta_e+O, the same degree-three divisor used
at the earlier even-theta origin.

A zero-sum triple of odd characteristic labels gives

    delta_e = L_9 + L_19 + L_26 - K_C

in divisor class. For each required two-torsion label i, choose odd labels a,b
with a+b=i and use D_i=L_a-L_b. The function l_a/l_b has divisor 2D_i, where
l_a,l_b are the bitangent linear forms. These small divisor expressions replace
the larger unreduced expressions used in initial development.

Since 2delta_e is equivalent to K_C, the determinant spaces can be written

    L(O+K_C),  L(D_i+O+K_C).

The first has basis (1,x,y). The raw functions are

    f_i(P1,P2,P3)
      = (det(psi_j(P_l))/det(1,x(P_l),y(P_l)))^2
        * product_l (l_a/l_b)(P_l).

Seven labels, together with the constant function, give the eight coordinates
used by the prototype.

## Eight symmetric sections with rational coefficients

Put O=(1:0:0) and its tangent line at Z=0 using a projective change over F_p.
The tested charts have ordinary tangency. Write

    C(X,Y,0) = Y^2*q(X,Y),

with q a quadratic binary form. An eight-element basis for L(2O+2K_C) is

    1, x, y, x^2, xy, y^2, x*q(x,y), y*q(x,y).

The 120 symmetric triple products of this basis have an eight-dimensional
subspace vanishing twice on the big diagonal. The code finds it by sampling
diagonal triples, then verifies all eight resulting diagonal identities exactly
by polynomial reduction. Symmetry and characteristic different from two make
the first normal derivatives vanish as well.

All 120 coefficients are sought over F_p. The sampling uses a fixed number of
random curve points over F_(p^3), rather than a scan through that field. The
constraint rank must be exactly 112. The resulting eight sections zeta_i have
coefficients in F_p; their observed maximum total degree is eight.

An invertible eight-sample evaluation matrix identifies these sections with
omega^2*f_i. Twelve additional samples verify the change of basis. The known
finite-dimensional section-space identities justify interpolation; the exact
diagonal tests are additional checks, not the sole justification.

Applying this change of basis to the source quotient-function values gives
lambda_1,...,lambda_8. The code checks that **each** lambda_i lies in F_p.

## Polynomial inversion and signs

For six affine point coordinates and a saturation variable t, solve

    C(x_i,y_i,1)=0, i=1,2,3;
    lambda_j*zeta_i-lambda_i*zeta_j=0, i!=j;
    t*zeta_j-1=0.

The saturation index j is chosen with lambda_j nonzero. The solver uses a
base-field Groebner basis, FGLM conversion, and triangular solution extraction.
The test cases have 6 or 12 ordered affine solutions. Permuting the three points
does not change the divisor. One or two opposite divisor classes are recovered;
if only one sign has an affine representative, Jacobian negation supplies the
other.

To align the images of P and Q, also invert the image of P+Q. Choose signs S,T,U
so S+T=U. A common global sign remains immaterial to the logarithm. The final
answer is checked on the original elliptic curve.

The source/target pair is multiplied by a common nonzero scalar when needed to
leave the evaluation chart. The tested driver searches a bounded set of such
multipliers. Full handling of every nongeneric divisor chart is not claimed.

## What is measured

The end-to-end driver uses a basic rational-point factor-base index-calculus
solver on the actual target Jacobian. Every smooth decomposition is verified as
a divisor-class equality; a full-rank linear system produces the logarithm; the
answer is checked in both groups. The solver receives no known source scalar.

This is not Diem's optimized line/large-prime implementation. The small-field
end-to-end runs are much slower than source BSGS. No cryptographic-size ECDLP or
new attack exponent is demonstrated. The completed stage is generic polynomial
image conversion in a research prototype, tested on two characteristics.
The source BSGS runs are separate controls performed after the transfer; their
answers are not inputs to inversion or target solving.

## Reproduce

From the suite root with SageMath 10.6:

```sh
sage -python research_lead/polynomial_inverse.py --solve
sage -python research_lead/polynomial_inverse.py --solve --p 13
```

The raw records are [the p=7 run](results/polynomial_inverse.json) and
[the p=13 run](results/polynomial_inverse_p13.json). The older
`complete_toy.py` uses an explicitly exhaustive inverse oracle and is retained
only as development evidence. It is not used by the polynomial driver.
