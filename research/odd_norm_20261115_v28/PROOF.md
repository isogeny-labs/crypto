# Prepared odd-characteristic quintic norm compiler

Let K have odd characteristic and let the finite target T=(z,y) satisfy y^2=z^3+a z+b. Put F(X)=X^3+rX^2+sX+t, G(X)=hX+k and t=-z^3-rz^2-sz-(hz+k)y. Then F(z)+G(z)y=0, so the monic degree-six norm F(X)^2-(X^3+aX+b)G(X)^2 is divisible by X-z. Its quotient H is monic of degree five, including repeated roots and arbitrary h or k.

Writing H=X^5-e1 X^4+e2 X^3-..., coefficient comparison gives

r=(h^2-z-e1)/2,
s=(e2-r^2+2hk+z e1)/2.

Thus for every fixed h,k, prescribing e1,e2 determines r,s uniquely, and conversely every r,s yields one e1,e2. No division by h or k occurs, so their zero branches are retained. This differs from binary formulas: two is invertible here, whereas binary squaring is additive and permits different linear-kernel eliminations. The compiler requires a finite on-curve target and excludes characteristic two explicitly via two=1+1. This is not a claim that one field family is universally faster.

The prepared source constructs the norm by literal polynomial convolution and exact synthetic division; its displayed moment assertions are consistency checks. Status: syntax-checked, unexecuted and unregistered. First freeze an independent worker and input hashes, then compare all h,k,r,s over F3,F5,GF9 at a fixed valid target, bounded GF25 cases and full-width NIST curve/generator cases. Use a separately implemented coefficient convolution/oracle and test h=0,k=0, repeated H, characteristic three and invalid-target refusal. Budget90seconds/4GiB/onecore; explicit state/output caps before materialization.

This is a divisor-norm coefficient representation, not a supported-source search or complete polynomial-root solver. Curve/source/ghost and all outer-chart coverage remain separate. Divisor norms and coefficient matching are established prior art; neither novelty nor an end-to-end speedup is asserted. The new signed field backend's tests do not automatically validate this nonlinear compiler.
