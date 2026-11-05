# The exact boundary defect block

LOCAL50 (`runs/local50-01`) terminates with native exit0, its exact PASS
marker and no Singular error lines. It checks the actual source equations,
not a scalar replacement of them. Write R=F2[t,u], e=u^3+u,
E=v^2+v+e and Q=A+Bv. The accepted finite refinement is
D=R[v]/(E,Q), and the plane ring maps into D through R.

Since E is monic, R[v]/E is free with basis 1,v. Multiplication by Q has
matrix

    [ A       eB   ]
    [ B       A+B  ].

The native run checks its second column by the complete polynomial identity
vQ+eB+(A+B)v=BE and checks that its determinant is the actual plane
polynomial F. Quotienting D by the image of the plane ring deletes the
first basis vector; the remaining relations are B and A+B. Therefore

    D / image(A_plane) = R/(A,B)       as R-modules.

This is a module quotient; the image need not be an ideal of D.
FIBER50 and LOCAL50 give B=hBprime with h=(u+1)^8. LOCAL50 exports
the full coefficients alpha,beta,gamma of

    1 = alpha A + beta h + gamma Bprime.

Thus K=(A,h) and L=(A,Bprime) are comaximal and
(A,B)=K intersect L. Constructive CRT gives a decomposition of the
actual defect module as R/K direct-sum R/L. In this module X=Bprime*v
has class Bprime: it is a unit on the K component and zero on L.
Its cyclic submodule is consequently exactly R/K.

The native standard-basis calculation returns dim_F2(R/K)=128. The
normal forms of (u+1)^j are nonzero for every j=0,...,7 and zero for j=8.
Consequently the class of X has exact parameter-saturation exponent8,
and generates a 128-dimensional F2 block. The radical support is
(u+1,t^2+t+1), consistently with the complete FIBER50 fiber and the
two separated refined degree2 points in BOUNDARY50. This is a block
of the plane-to-integral-v coordinate defect, not a calculation of the
remaining normalization defect of D.

Integrality is separately checked by the complete monic identity
X^2+Bprime X+e Bprime^2=Bprime^2 E. The source relation hX+A=Q
puts hX in the plane image. The unit lift, source matrix identity and
power normal forms are retained in `local-block50.sing`; corruption of
the unit coefficient is rejected. Local50.lean contains general scaling,
constructive CRT and nilpotency lemmas. `local-lean50-01` has compile.exit0
and audit.exit0: all3 named theorems pass, zero failing/trusted. The separate
Separation50 evaluation lemma passes in separation50-03. Together with the
six Rational50 theorems this gives10 audited R50 mathematical declarations.
These generic lemmas do not themselves formalize the native coefficient
arrays, the module presentation or the scheme-theoretic interpretation.
