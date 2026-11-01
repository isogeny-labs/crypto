# Direct cyclic companion generators

Derived reduction and unexecuted constructor, 2026-11-01. Source elliptic
curves and coefficient arithmetic remain over the existing prime fields.
This is a linear-system representation change, not a new logarithm extractor.

## Boundary identity before the Pascal transform

Over F_r, let (Tv)_k=v_(k+1 mod r). Let v_(P,a)(k) be the product of
quadratic characters chi(k+a+d) for d in a pattern P with minimum offset 0.
Write ell_P=q-max(P), gamma in F_r minus {0,1}, and define the same weighted
feature Gram matrix as the existing compiler:

    B = w_empty 11^t + sum_P w_P sum_(a=0)^(ell_P-1)
                                      gamma^a v_(P,a) v_(P,a)^t.

Since Tv_(P,a)=v_(P,a+1), telescoping gives

    H := B - gamma T B T^t
       = (1-gamma)w_empty 11^t
         + sum_P w_P(v_(P,0)v_(P,0)^t
                       - gamma^ell_P v_(P,ell_P)v_(P,ell_P)^t).

Let V collect 1 and both boundary vectors for every pattern, and let c
contain the displayed coefficients. Then H=V diag(c) V^t, using
s=3+q(q-1) columns. This is the public boundary data already used by the
existing compiler, before its inverse-Pascal transforms.

Multiplying the identity on the right by T gives

    gamma T B - B T = -H T.

All index shifts are cyclic on the coefficient field's r evaluation points.
No elliptic-point enumeration or unknown scalar enters this construction.

## Explicit standard companion form

Let J reverse the r coordinate positions, so J T J=T^t. Put
D=diag(1,gamma,...,gamma^(r-1)). Define

    A = D^-1 J B J,
    G = -D^-1 J V diag(c),
    K = J T^t V.

Then

    Z_(r,gamma) A - A Z_(r,1) = G K^t,

where Z_(r,a) has subdiagonal ones and top-right entry a. Indeed,
D^-1(gamma T^t)D=Z_(r,gamma), because gamma^r=gamma in F_r.
The right operator T^t is Z_(r,1). Their characteristic polynomials
X^r-gamma and X^r-1 are coprime: their difference is the nonzero constant
1-gamma. Repeated roots in characteristic r do not spoil this coprimality.

This Sylvester operator is injective directly: if gamma T B=B T, iterating
r times gives gamma^r B=B, hence (gamma-1)B=0 and B=0. Thus the generator
determines the matrix; no missing boundary row has been hidden.

For Bx=b, solve Ay=d with d=D^-1 Jb and return x=Jy. For a left witness
vA=0 with vd=1, return w=vD^-1 J; then wB=0 and wb=1. Here b_k=k is
the public target function used by the current compiler. A Gram null vector
still requires the existing separate public feature-exclusion check.

## Constructor and accounting

`generators.py` implements only G,K,d and feature labels/weights, from public
r,q,gamma,weights. It does not construct B, A, a Pascal matrix, an inverse,
or a solution. It meters character powers, boundary vectors, diagonal powers,
scalings and RHS formation. Time and storage include O(rs) boundary data;
this representation is not sublinear in r. Syntax checking is not finite
validation; no constructor experiment has run yet.

## Literature connection and exact applicability limit

Bostan, Jeannerod, Mouilleron and Schost, *On Matrices with Displacement
Structure: Generalized Operators and Faster Algorithms*, SIAM J. Matrix
Anal. Appl. 38(3), 733-775 (2017), studies companion displacement operators
and fast structured linear-system algorithms. Theorem 6.7 gives a sufficient
sampling condition |S|>2n(n+1) for its stated failure bound in the square
dimension-n case. Here n=r and S contained in F_r cannot meet that condition.
This does not prove the preconditioner fails; it means the quoted success
guarantee is unavailable for this base-field sampler.

Primary source, inspected 2026-11-01:
https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf
(operator setup and Theorems 1.1-1.2; solve guarantee Theorem 6.7, page 39
in the zero-based PDF numbering).

The formula above is our derived mapping of the existing compiler, not a
claim that the paper implements this ECDLP construction. Do not infer an
improved complete solver bound before the multiplication backend, singular
system behavior, base-field preconditioning and exact certificates exist.

## Next frozen check

Independently form the public feature matrix on the full declared small and
larger coefficient-field fixtures; verify the boundary identity, companion
displacement entrywise, coprimality and primal/dual transport. Charge all
constructor counts and compare the public boundary inputs to the old
compiler. Include zero/invalid gamma rejection and altered generator,
wrap-entry, scaling and transported-witness controls. Freeze this panel
before execution. The active full recovery remains unchanged.
