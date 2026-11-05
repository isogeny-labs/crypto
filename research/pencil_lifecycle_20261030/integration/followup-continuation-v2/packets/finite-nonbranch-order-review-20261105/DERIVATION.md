# Statement-based independent derivation (context-informed)

This was written before reading the producer source. The coordinator already disclosed the conjugate-point strategy; this is not blind.

Let A=K[x,y]/(y²-f(x)) be the actual integral coordinate ring, P=(x0,y0), Q=(x0,-y0), g=n(x)-b*y and h=n(x)+b*y. At Q, g evaluates to n(x0)+b*y0=2*b*y0≠0; thus g≠0 in A. At P, h has that same nonzero residue, hence is a unit of A_P. Injectivity A→A_P transports g≠0. For the nonsingular point's normalized discrete order, every nonzero local element has finite nonnegative order. Therefore ord_P(g*h)=ord_P(g)+ord_P(h)=ord_P(g). If the actual norm identity gives g*h=u1*u2*U (or a nonzero scalar times that product), the product is nonzero, each factor is nonzero, and additivity proves the desired finite Nat sum. Each use of actual ring, actual maximal ideal, injective map and finiteness must be found in the source binding, not postulated as the conclusion.

## Concrete boundary attacks

- b=0: over Q take f=x^5+1, P=(0,1), n=x. The curve is smooth and nonbranch at P. Here g=h=x and the norm is x². Since x is a local parameter, ord(g)=1 but ord(norm)=2. More severely, n=0 makes g=0 with infinite order. Thus b≠0 is load-bearing.
- y0=0: over Q take f=x^5+x, P=(0,0), n=0, b=1. This separable quintic is smooth at P. Locally y is a parameter and x has order 2 because y²=x(1+x^4). Then g=-y has order 1 while norm=-f has order 2. The conjugate factor is not a unit.
- Characteristic 2: over F2 take f=x^5+x+1, P=(0,1), n=1, b=1. f is separable: f'=x^4+1=(x+1)^4 and f(1)=1. At P set t=y+1; then t²=x(1+x^4), so ord(t)=1 and ord(x)=2. Both conjugate factors equal t; ord(g)=1 and ord(norm)=2. Nonzero b and y0 alone do not suffice.
- Zero U: with the valid nonzero/unit premises, norm=g*h≠0 in the domain. An identity norm=u1*u2*0 would contradict this. Check the formal theorem derives incompatibility rather than accepting a Nat order convention ord(0)=0 and thereby manufacturing a finite equality.
- Kernel map: evaluation at P is a ring homomorphism A→K that sends the nonzero g to zero under the selection premise. It cannot replace localization in the nonzero transport. Localization injectivity must be established from the actual domain/denominator set.

These examples are algebraic attacks on the boundary, not verified Lean counterexample artifacts. Fresh formal controls remain to be prospectively authorized and bound.
