# Independent derivation and failure modes for the h-block lead

This is a static mathematical review, not a scientific execution, novelty finding or timing result.

Let E/Fp have equation y²=X³+aX+b with p odd, with three fixed finite points (z,w),(x,u),(y,v) having distinct abscissae. For P=X³+rX²+sX+t and Q=hX+k, impose P+QY=0 at all three points. Subtract the target equation and divide by the nonzero abscissa differences. With alpha_i=(w−ordinate_i)/(abscissa_i−z) and beta_i=−(abscissa_i²+z*abscissa_i+z²)+h*(z*w−abscissa_i*ordinate_i)/(abscissa_i−z), solve
r=A*k+B0+B1*h,
z*r+s=C*k+D0+D1*h.
The constants A,C depend only on the fixed points. The target equation recovers t without division by Q.

The monic norm N=P²−(X³+aX+b)Q² has degree six and coefficient X5 equal to 2r−h². Division by the three fixed linear factors gives a monic cubic C=X³−S*X²+T*X−U, so
S=h²−2r−z−x−y.
When kappa=2A is nonzero,
k=(h²−2B0−2B1h−z−x−y−S)/kappa.
This sign convention agrees with the old convention kap=−2A and k=(S−delta)/kap.

Give h weight1 and S weight2. Then k,r,s,t have weighted degree at most2, so every coefficient of N and of its fixed linear-factor quotient has weighted degree at most4. The complete support is S²,S*h²,S*h,S,h⁴,h³,h²,h,1. There are eight variable monomials plus T and U: the obvious modular lattice has dimension10, rather than the previous fixed-h dimension4. Smaller feature notation does not establish a runtime improvement.

For a nonwrapping canonical interval h=h0+u, 0<=u<=H and h0+H<p, substitution retains the support with u replacing h. Let T=sum a_i*m_i+a0 and U=sum b_i*m_i+b0 modulo p. Any integer gamma,delta and integer lifts lambda_i congruent to gamma*a_i+delta*b_i, with c congruent to gamma*a0+delta*b0, imply
E=c+sum lambda_i*m_i−gamma*T−delta*U = 0 mod p
at every admissible residual-root tuple. An independently verified exact rational enclosure [L,R] for E over 0<=S<=3M,0<=u<=H,0<=T<=3M²,0<=U<=M³ proves the entire block empty if ceil(L/p)>floor(R/p). This is a sufficient rejection certificate only; failed search or overlap means UNKNOWN, never a nonempty or empty root set. Monomial boxing and exact Bernstein bounds are both sound if conservatively computed. Bernstein arithmetic must remain rational and handle H=0 separately. Untrusted lattice reduction proposes a vector; the checker verifies all congruences, domain bounds and exact enclosure.

A limitation is provable before running. For M>=1 and a full canonical h interval 0..p−1, every h-bearing monomial has independent-box width at least p−1. If any such monomial has nonzero integer coefficient, the resulting integer interval has width at least p−1 and contains p consecutive integers, hence every residue modulo p. Therefore a full-field monomial-box separator must make every h-bearing coefficient exactly zero. This is not an impossibility result for dependency-aware polynomial bounds, nor for small translated blocks, but it refutes an inference that the 9-term support alone compresses all h values tractably.

The kappa=0 branch is real. Over F5, E:y²=x³−x+1 is nonsingular, and (0,1),(1,1),(4,1) are distinct collinear points. Take the first two as anchors, last as target, and D=[0,1]; the target is outside D but A=0. The S relation is then independent of k. This example establishes the pole can occur; it does not itself assert a supported five-source solution. This branch must remain unresolved or use a different k/h parameterization. Repeated source abscissae, common P/Q zeros and rational torsion are not grounds for dropping branches: norm polynomial divisibility retains multiplicity and the certificate does not divide by Q. Recovering original point coordinates is a further obligation. Binary curves are outside this odd-characteristic proof.

A positive control with known public h must be tested in the unique deterministic block containing that canonical h; an exact certificate must not reject it. H=0 should reproduce the fixed-h algebra after constant folding. A small independent original-norm oracle should enumerate literal P/Q and residual multisets rather than compare two copies of the same feature interpolation. Global anchor/h coverage and original-root completeness remain unproved. The block count is ceil(p/(H+1)) for a full sweep, and anchor enumeration remains external work. No literature novelty or qualified comparative speed claim follows.
