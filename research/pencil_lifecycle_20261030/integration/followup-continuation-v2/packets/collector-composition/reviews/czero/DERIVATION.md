# Blind derivation before producer inspection

Let u1=X^2+q1 X+r1, u2=X^2+q2 X+r2, v1=s1 X+t1, v2=s2 X+t2. Put dq=q1-q2, dr=r1-r2, ds=s1-s2, dt=t1-t2. Read literal native B=dq*(q2*r1-q1*r2)-dr^2, A=dq*(dt*q2-ds*r2)-dr*dt, C=dq*dt-dr*ds.

Define L=dq*X+dq*q2-dr. Expansion gives

B = L*u1 - (L+dq^2)*u2.

Thus B nonzero in a field provides a constant-unit Bezout certificate. Independent coefficient expansion yields

A*dq - B*ds = C*(dq*q2-dr),
A*dr - B*dt = C*dq*r2.

For C=0 and B nonzero, lambda=A/B satisfies lambda*dq=ds and lambda*dr=dt. Consequently the common interpolant is l=v1-lambda*u1=v2-lambda*u2. The MINUS sign is essential. In normalized unit-scale native C0 output, x=q1+q2+lambda^2 and y=lambda*((q1+x)*x+r1)-s1*x-t1=-l(x). This checks ordinate orientation only; output curve equation needs intersection residual factorization and source curve hypotheses, while divisor-class equality needs its own construction.

Weighted boundary: native qic/ric/sic/tic have different cross-weights. One cannot substitute them as normalized coefficients without tracking z1,z2,w1,w2. The normalized algebra establishes a reusable component, not by itself weighted payload or machine correspondence.

Derived by source inspection and symbolic hand expansion, before reading producer CZero.lean. No sampled inputs, execution measurements or Lean replay used for this blind derivation.
