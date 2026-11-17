# Independent cost review of the pinned Boolean tensor-contraction proof

2026-10-30 UTC. This review concerns general finite-ring mathematics and algorithm accounting, not an ECDLP implementation. No solver, scientific experiment or timing run was performed. I read Theorem D.1 and the full decisive proof in Appendix D.4, including its algebraic identities, rather than only the theorem statement or abstract.

Source: the supplied public commit `7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a`, file `preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/paper.pdf`, retained as `/workspace/audit/tensor-primary/openai-oct25.pdf`. SHA256: `5b1612dc2eebb2cad5b4cbe62d1fed2982f01ab1908c26f70010a651487e8123`. Page references below are the paper's printed pages. The repository namespace and PDF metadata establish neither institutional endorsement nor mathematical validity.

## Verdict and precise scope

I found no concrete unjustified implication in the reviewed D.4 contraction-cost argument. Its recursion, edge counts, mode enumeration, precision growth, entry-evaluation costs and depth-first space bound fit together as stated. This is a substantive independent check of that derivation, not a claim that every result or application elsewhere in the paper has been audited, nor a formal verification of the theorem.

The actual theorem is restricted (Theorem D.1, p53): a bipartite Boolean-edge tensor network with at most Cs edges, maximum degree at most ceil(s^(9/10)), polynomially many vertices, entries in (Z/2^P0 Z)[V]/(V^ell), P0,ell<=Cs, and uniformly computable entries in time (s+L_in)^b 2^(A s^(9/10)) and polynomial space. Here L_in denotes the total input-description length; I distinguish it from the proof's packet length, also called L. The claimed contraction cost is deterministic (s+L_in)^O(1) 2^O(s/sqrt(log s)) bit work and polynomial space, with fixed constants depending on the stated fixed bounds. Contraction reductions into that exact model require their own separate verification.

## 1. Shrinking children and valid parameter range

The global parameters (D.4.2, p58) use lambda=ceil(log2 s), a leaf threshold H=ceil(s/floor(sqrt(lambda))), r=64, and a fixed explicit epsilon. At active depth j,

    u_j=floor(epsilon lambda/2^j),
    a_j=2 sum_(k=0)^j u_k,
    L_j=2^a_j,
    Darr_j=r^(2a_j).

The superscript in Darr is material: it is the number of ordered pairs of vertex labels. Equation (D.6) requires M>=2 Darr² Delta, M>=L Darr, and 2L<=Delta. The first inequality includes Darr squared. These follow eventually from the printed small epsilon and H much larger than s^(9/10+2/1000); no assumption that an asymptotically nonconstant quantity is constant is needed.

Lemma D.3 (pp58-59) is supported by its stated second moment. Distinct-edge pairs with disjoint endpoints have independent label pairs; at most2M Delta ordered pairs share an endpoint. Conditional expectation can be computed exactly by trying polynomially many labels and examining edge pairs. The resulting individual class load is at most M/Darr+sqrt(2M Delta). Under (D.6), rounding to a multiple of L gives Hpad<=3M/Darr. The packet count is at most3M/L, and each side has at most3M/r real ports of a given coordinate symbol.

A group child retains both current row and current column ports. Thus its edge count is at most3M/r+3M/r=6M/64=3M/32<M/8 (D.4.6, p63). It is not necessary to pretend that the sum of all children's edge counts is M: that sum can be2M. The constant r children per mode tuple is charged as branching. Old degrees are unchanged and new packet vertices have at most2L ports, preserving Delta.

Hence M_j<=Cs 8^-j. For an active call M_j>H, giving j<=(log2 lambda)/6+O_C(1). In particular u_j is eventually at least lambda^(4/5), all retained valuation coordinates i<u_j exist among the a_j coordinates, and floor effects do not create an active call with u_j=0. Small sizes or failed parameter inequalities can be handled by direct summation; the thresholds are effective because the inequalities are explicit. The fixed finite exceptional range affects constants, rather than the asymptotic exponent.

## 2. Mode bound and exact factorization

The Cauchy kernel (D.7), deformation (D.8), and Lemma D.4 (pp59-60) preserve the original contraction at T=0. At a retained node t with i=v2(t), its coordinate-i orientation makes W(t) local. For a crossing row and tile, v2(t-y0+x)<=i, so the denominator in the geometric expansion (D.12, p62) has that same valuation after adding alpha zeta. It cannot become zero there.

The b-th expansion coefficient has valuation at least b+1, and the number of modes at each weight is at most6M/D, where D=L/2^i (D.13). The omitted weighted monomials really vanish modulo2^pt: the substituted mode sums are integral, so they cannot undo that divisibility. For N=6M/D, the partition generating-function bound gives

    #mode tuples <=exp(theta pt+(N/theta) sum_(h>=1)1/h²).

Choosing theta=sqrt(M/(D pt)) yields exp(O(sqrt(M pt/D))). Together with pt=O_C(MD/u), this is exp(O_C(M/sqrt(u))) (D.15, pp62-63). The square root in the printed choice of theta is easy to lose in text extraction; the displayed derivation uses that root. Finite-prefix enumeration has polynomial delay/space: each visited feasible prefix has an all-zero continuation and is a prefix of an actual tuple.

The child identity (D.16, p63) is also structurally consistent. Each formal mode occurs in row factors of one target group and is substituted by a linear form involving one source group. Coefficient extraction is local to a packet and target group; source powers are local to its source group. Independent port assignments can then be summed within each child. This avoids an uncharged coupling between different children.

## 3. Precision and bit work

D.4.1 (pp57-58) uses a degree e=4lambda unramified coefficient extension. Enumerating 2^e candidates and testing irreducibility by the stated exhaustive divisibility test costs a fixed polynomial in s. The minimum coordinate two-adic valuation is multiplicative because normalized nonzero residues multiply nontrivially in F_(2^e). Unit inversion and all truncated-V coordinate arithmetic therefore have polynomial bit cost in s,p,ell; no floating-point or numerical approximation is present.

I checked the precision mechanism in D.4.3-D.4.4 (pp59-61):

* The complete residue grid has uniform derivative valuations. Canceling known powers of two before evaluating gives coefficientwise integral W(t+alpha z), including evaluation at roots where the uncanceled rational expression would look undefined.
* The Hermite identity (D.9) is the usual product/jet Chinese-remainder construction over K[V]/(V^ell). Node differences are nonzero scalar units in K; the nilpotent V variable causes no interpolation obstruction.
* For v2(t)=i, D=L/2^i, there are D/2^h points contributing each higher valuation i+h. Pairing their positive and negative contributions gives (D.10). The taper changes by at most h_steps times the slope h_precision, yielding the stated valuation lower bound -6D h_precision from sum h_steps²/2^h_steps=6.
* Jet degree is below q_t<=4M. Evaluation at -t/alpha loses at most4M(a+1) additional powers of two. The supplied precision p_t=p+8h_precision D+8M(a+1) exceeds both losses with room to spare. The inverse local Hermite factor has integral coefficients and constant term1, so it contributes no further precision loss.

The inherited-precision estimate (D.11, p61) is consistent. Previous increments give p<=s lambda^O(1) 2^(a_(j-1)). For an active node, D>=2^(a_j-u_j)=2^(a_(j-1)+u_j). Since u_j>=lambda^(4/5), the extra2^u_j absorbs every fixed polynomial in lambda and gives p_t=O_C(MD/u_j). All p_t remain polynomial in s because a_j<=4epsilon lambda. This is the crucial premise used in the mode count, not an omitted cost.

The full local polynomial degree is at most M(2L-1). Fewer than s^4 residue-distinct evaluation points suffice, and the residue field has at least s^4 elements. Their differences are units, so coefficient recovery has no extra two-adic loss. Rational Hermite-combination multipliers have polynomial-bit numerators/denominators: there are polynomially many factors, with polynomial multiplicities and O(a)-bit scalar factors. A product of their denominators is still polynomial-bit. Exact cancellation of powers of two is consequently charged, rather than treated as unit-cost rational arithmetic.

## 4. Entry routines and descriptions do not hide exponential recursion

A new packet entry contains at most L affine row factors, each having polynomially many terms. Exhaustively choosing their terms costs exp(O(L log s)), with polynomial space (D.4.6, p63). Since L<=s^(1/1000), this is eventually at most exp(O(s^(9/10))). Powers of a numerically evaluated source mode use ordinary repeated squaring, not expansion into exponentially many monomials.

This entry computation does not call the old tensor entries. Those entries remain at their own retained vertices. A packet entry is determined by its stored local kernel, interpolation value, mode tuple and creation precision. Therefore retaining an old packet does not wrap its routine inside another packet routine. The immutable/shared descriptions have only polynomial growth along a path.

Creation-precision canonical lifts are entrywise choices, not alleged ring homomorphisms (D.4.1, p58). Evaluating an old entry modulo its creation precision and reusing those integer representatives supplies the same chosen lift at every descendant precision. This is sufficient for the deformation identities; it would be wrong to replace them by independently recomputed higher-precision formulas for an earlier packet, but the stated protocol does not do so.

Original entry routines retain their global cost 2^(A s^(9/10)); the proof does not need them to speed up with the child edge count. Each leaf has at most H edges and uses at most2^H assignments and polynomially many entry evaluations per assignment. Thus its cost is

    (s+L_in)^O(1) exp(O(H)+O_A(s^(9/10)))
      =(s+L_in)^O(1) exp(O_A(s/sqrt(lambda))).

Polynomially many isolated vertices are included in that entry-evaluation cost. Their scalar factors are not dropped.

## 5. Summing the recursion and space

At active depth j, the per-node branching bound (p64) is

    s^O(1) exp(O_C((s/sqrt(lambda))(sqrt(2)/8)^j)),

because M_j<=Cs 8^-j and u_j=Theta_C(lambda/2^j). The exponent sum is geometric. The polynomial factors over O(log lambda) levels add only O(lambda log lambda) to the logarithm of the call count. This is dominated by s/sqrt(lambda). Thus the total call count, not merely a single root-to-leaf path, is exp(O_C(s/sqrt(lambda))). Multiplying by the leaf cost and polynomial arithmetic/description costs preserves that exponent class and the fixed polynomial factor in s+L_in.

Depth-first traversal need retain only the current mode tuple, group/product accumulator, interpolation data, graph and shared descriptions for each level. Each is polynomial-bit; there are O(log lambda) levels. The explicit common-denominator calculation is likewise polynomial-bit. This supports the stated polynomial-space bound (D.4.7, p64).

## Remaining review boundaries

No concrete counterexample or missing cost factor was found in this Appendix D.4 review. I did not independently audit the paper's other reductions, its broader ETH consequences, or any separate ECDLP circuit encoding; those cannot be inferred solely from the contraction theorem's statement. Practical constants are not a mathematical objection to this asymptotic proof. Conversely, a scoped independent reading is not a substitute for further expert or formal checking of a claim of this strength. The appropriate recorded status is that the decisive contraction recurrence and its displayed supporting accounting survived this review, with its exact hypotheses retained.
