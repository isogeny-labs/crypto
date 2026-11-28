# Order-four genus-four qualification and simplicity certificates

This is a proof/qualification screen, not an index-calculus speedup or a mathematical priority claim. For a smooth odd polynomial f of degree9 over Fp, p1mod4, choose i²=−1. The map sigma(x,y)=(-x,i*y) preserves y²=f(x), squares to the hyperelliptic involution and has order4. On a reduced degree-m Mumford divisor,

 u'(X)=(-1)^m*u(-X), v'(X)=i*v(-X).

This preserves monicity, reduction degree and u|f−v². It is induced by a curve automorphism fixing infinity, so it is a group endomorphism. The gate checks these identities, additivity, and a wrong-y negative control independently in the original group.

## Exact order from extension-field counts

Count C over F_(p^d), d1..4, by enumerating every field element and every square. Irreducible field moduli are independently checked by the existing polynomial Frobenius/GCD routine. Optimized extension multiplication is compared with native polynomial multiplication/remainder. Base-field point counts are also compared with native Legendre enumeration.

Set S_d=p^d+1−#C(F_(p^d)). Newton identities determine the first four Frobenius coefficients; the Weil functional equation determines the other four. Thus F(T)=T8+c1*T7+c2*T6+c3*T5+c4*T4+p*c3*T3+p²*c2*T²+p³*c1*T+p4. The whole Jacobian order is F(1). Exact divisibility and Weil trace bounds are checked. No old identity pi=P(sigma) is assumed.

## Simplicity over Fp

For a genuine ordinary Weil polynomial, every irreducible rational factor has even degree and is itself reciprocal with the appropriate p powers. A reducible degree8 polynomial therefore has a factor of degree2 or4. Exhaustively test monic candidates T²+aT+p with a²≤4p and T4+aT3+bT²+p*aT+p² with a²≤16p and |b|≤6p. The bounds follow from root absolute values sqrt(p). Exact integer division tests every candidate. Ordinarity is checked by p not dividing c4. For ordinary F, absence of these factors proves irreducibility; then the corresponding ordinary Jacobian is Fp-simple. A nonordinary case remains uncertified by this test. Finding a factor proves reducibility; geometric conclusions are not silently inferred in nonordinary cases.

## Absolute simplicity: finite modular certificates

Use the Howe–Zhu sufficient criterion: for a simple variety, Q(pi^m)=Q(pi) for every m>0 implies absolute simplicity. Primary source: Howe–Zhu, Proposition3 and Lemma4, https://arxiv.org/pdf/math/0002205. We give an explicit sufficient certificate for the needed power fields, rather than assuming them from an irreducible F.

If two distinct conjugates alpha,beta have alpha^m=beta^m, then alpha/beta is a root of unity of some order n. Since [Q(alpha,beta):Q]≤8²=64, Euler phi(n)≤64. Also phi(n)²/n≥1/2: each odd prime-power factor contributes at least1 to that ratio, and the only smaller factor is2^1, contributing1/2. Hence n≤8192. It suffices to rule out collisions for all n1..8192 with phi(n)≤64.

For each such n, compute the characteristic polynomial F_n of the n-th power of the integer companion matrix of F, modulo a fixed prime r>8. Newton identities modulo r compute it exactly. If gcd(F_n,F_n')=1 modulo any one r, then the integer F_n has no repeated root. A genuine collision would make its discriminant identically zero and repeated modulo every prime. A witness for every candidate n therefore excludes every root-of-unity ratio. With irreducible F this implies [Q(pi^m):Q]=8 for every m, hence the required power-field equality and absolute simplicity.

The certificate is sufficient, not complete for the finite witness list: failure to find a modular witness is UNCERTIFIED, not proof of decomposition. All witness primes are independently checked prime. A known collision polynomial T8−1 at exponent2 is the negative control; a product of eight distinct integer linear factors is the positive control, with its squared-root characteristic polynomial checked directly. No arbitrary extension cutoff is substituted for the finite root-of-unity bound.

## Prime-subgroup compatibility and orbit map

Factor the exact F(1), choose its largest prime ell, and require ell odd with v_ell(F(1))=1. The cofactor h=F(1)/ell then projects rational points into the unique rational subgroup of order ell. A nonzero projected point supplies D; no accessible projected point means the input is not group-qualified. Recover lambda from the two public roots of lambda²=−1 by checking sigma(D)=[lambda]D, not by solving a general DLP. Verify the same action on every projected catalogue point and [ell]P=0. Branch-point classes have order2 and project to zero. Signed nonbranch orbits have length2 after identifying negation; after two steps the point and weight are both negated. Their original-group reconstruction is checked.

Group qualification, Fp simplicity and absolute simplicity are separate output flags. Small fields and cofactors are reported; no generic, strongest-external, independent-admission or tenfold performance claim is made. This may establish a valid replacement test class, but it does not establish an index-calculus improvement on it.
