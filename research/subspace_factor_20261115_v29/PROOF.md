# Prepared implicit-subspace root extraction with multiplicities

Let K have prime characteristic p and V_i=span_Fp(v_0,...,v_{i-1}), with an independent basis. Its monic root polynomial L_i is separable and Fp-linear. Start L_0=X and use

L_{i+1}=L_i^p-L_i(v_i)^(p-1)L_i.

Independence means each L_i(v_i) is nonzero. This stores only i+1 linearized coefficients instead of materializing p^i roots or polynomial coefficients.

For monic H of degree n<=5, every root (with multiplicity) lies in offset+V_i iff (L_i(X)-L_i(offset))^n is zero modulo H. Each root inside the coset is a simple root of its L-polynomial, so exponent n covers every H multiplicity; outside the coset the value is nonzero. This handles inseparability of H without derivative or squarefree assumptions.

At a recursive node, the p children are offset+a*v_{i-1}+V_{i-1}, a in Fp. The full-multiplicity factor for child a is

gcd(H,(L_{i-1}(X)-L_{i-1}(offset)-a*L_{i-1}(v_{i-1}))^n).

Compute remainders modulo H before powering. Distinct children are disjoint, and the product of nonconstant child factors is exactly H. At dimension zero the factor must be (X-offset)^multiplicity. Check these products and the final reconstructed H. A negative initial support test returns not-fully-supported, never an empty root set. Unsupported characteristic returns unresolved status, never rejection.

Each nonempty node carries at least one of at most n distinct roots, giving at most1+n*dvisited nodes and at most p*(1+n*d)small-degree gcd calls; the stored linearized coefficient tables use O(d^2)field words. This avoids p^d-domain enumeration for small p. The prepared implementation caps p<=31, degree H<=5 and basis length<=83. Large-characteristic prime fields need a different factorization/interval route; do not enumerate their pcosets.

Status: syntax-checked, unexecuted and unregistered. Next freeze an independent worker, exhaust complete tiny monic-polynomial/root-multiplicity oracles including unsupported factors, then test planted full/repeated supported quintics in the actual GF(2^83) 18-dimensional factor-base subspace. Include small odd extensions, dependent-basis refusal and exact canonical root reconstruction. Budget90seconds/4GiB/onecore and explicit case/output/node caps. Compare with the strongest matching specialized factorizer before any speed claim, charging support and setup. This is not the outer search for H, nor a proof of original polynomial/source/ghost coverage. Additive-polynomial gcd splitting is prior art; novelty is unestablished.
