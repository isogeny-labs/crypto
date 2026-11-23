# Independent novelty and evidence-selection review

2026-10-29. Read-only review for `/root`; no solver, performance experiment, private-data publication, or source edit. This note assesses a research bet, not a breakthrough claim.

## Verdict

The selected **shared divisor-coefficient membership family** is a defensible provisional allocation of effort among the inspected prime-field leads. It is not an evidence-backed novel sub-exponential algorithm. The concrete norm/Graeffe equations already exist at m=5 in `IDEA-20261029-3d0e71`; the extension from m=5 to general odd m uses standard Riemann-Roch spaces and a standard root-squaring identity. Growing arity is necessary for sub-exponential index calculus, and has been discussed extensively in prior decomposition work. The fresh research question is a uniform *solver bound* as arity and depth grow, not a fresh decomposition identity or attack.

The report's current cautious novelty wording is appropriate. It should not call the growing-arity extension itself a new algorithm. A positive correctness prototype would establish implementation of the construction, not algorithmic novelty or sub-exponentiality.

I recommend making **norm + shared quotient-remainder membership** the primary candidate and retaining Graeffe membership as a matched comparator. There is a concrete scheme-multiplicity defect in the terminal Graeffe formulation, and this defect was already diagnosed in the nearest actual-P-256 predecessor.

## Most important missing predecessor

Read and cite:

- `crypto-autoresearcher/research/p256_factor_base_parallel_20261025/tracks/f1/report.md`
- `.../tracks/f1/derivations.md`
- `.../tracks/f1/cubic-divisor/report.md`
- `.../tracks/f1/cubic-divisor/derivation.md`
- `.../tracks/f1/cubic-divisor/problem.md`

This package is more directly relevant than the positive small-field symmetric PKM stage example alone. It works over the actual P-256 prime, uses fully split norm-one torus/Dickson fibres, distinguishes symmetric terminal membership from radical divisor membership, and contains both successful exact solves and explicitly retained unknowns.

Recorded source outcomes (not newly rerun here):

- Original scalar-map symmetric terminal has length S^3 but C(S+2,3) distinct points; local multiplicities are 1,3,6 for the three repetition types.
- Radical cubic-divisor membership h | (D_S-c) has dimension C(S,3). It automatically enforces distinct coordinate roots for separable D_S-c; separate repeated-coordinate strata complete the domain.
- Original k=3 positive, several repeated-coordinate controls, a base-member cancellation control, and the published generator empty control were solved completely in the cubic-divisor continuation.
- k=4 distinct-coordinate positive remained unknown after three fresh bounded attempts (one Buchberger, two F5B), each with a 30-second watchdog and 2 GiB process cap. These are implementation/watchdog outcomes, not mathematical negatives or isolated timing results.
- Expanded remainder degree is S-2 for S>=4: degrees 2,6,14,30 at k=2,3,4,5. Keeping a short circuit is essential; dense expansion already loses the desired depth scaling.
- Deep witnesses through k=96 verify representation only, not complete relation solving.
- P-256 has v_2(p+1)=96, and suitable norm-one torus fibres are already constructed. Thus a Dickson shared-remainder variant gives a concrete prime-field setting beyond p-1 Kummer membership, though no attack is established.

The current report links the summary problem but should foreground these actual constructions, positive/empty controls, and unknowns. This materially improves its evidence selection and identifies the correct baseline.

## Terminal Graeffe membership is set-theoretic, not scheme-theoretic, even on squarefree U

The proposed equations are correct as a condition on root sets under their field hypotheses. They do not define the same finite reduced algebra as U | (X^d-c). Adding squarefreeness of U does not cure this.

An exact odd-arity counterexample to scheme equality uses F_17[eps]/(eps^2), m=3, d=4, c=1. Take roots

    alpha_1=1+eps, alpha_2=-1+eps, alpha_3=4.

Their fourth powers are 1+4eps, 1-4eps, 1. Consequently

    product_i (T-alpha_i^4) = (T-1)^3 mod eps^2.

The initial divisor polynomial is

    U_eps=(X-1-eps)(X+1-eps)(X-4)
         = U_0 + eps(-2X^2+8X),
    U_0=(X-1)(X+1)(X-4).

U_eps is squarefree in the sense that its discriminant is a unit (its reduction has three distinct roots). It is a nonconstant infinitesimal deformation admitted by terminal Graeffe equations. It does not divide X^4-1, since substituting the root 1+eps gives 4eps != 0.

The general explanation: the scalar power map is unramified at nonzero roots because p does not divide d, but the terminal symmetric coefficients identify m images all equal to c. The map to the symmetric quotient is ramified along that diagonal. At an input divisor with m distinct roots, local multiplicity is m! for p>m. This is the same symmetric-diagonal mechanism explicitly derived for m=3 in the P-256 F1 source. It is not a newly discovered general obstruction.

This does not refute the set-correctness proof, or prove a time lower bound. It does invalidate any assumption that squarefree U alone makes the Graeffe membership algebra radical, and it can enlarge quotient algebras and complicate extraction. At m~sqrt(n), m! itself remains sub-exponential, so its existence alone does not rule out the conditional complexity target. A claimed saving from eliminating labels must be assessed against the multiplicity that the scheme reintroduces.

## Cleaner primary encoding

For Kummer membership, use a monic degree-m divisor U and auxiliary remainder and quotient polynomials satisfying

    R_0 = X mod U,
    R_{j-1}^2 = Q_j U + R_j,  deg R_j < m, deg Q_j <= m-2,
    R_t = c.

For Dickson membership replace the first squared expression with R_{j-1}^2-2. Coefficient equations are quadratic if quotient coefficients are retained, with O(mt) unknowns and equations; eliminating quotient witnesses is not free. They express U | F exactly for the relevant separable F. The monic-divisor scheme of a separable polynomial is finite etale: at F=Uq, differentiating gives delta R = -q delta U mod U, whose Jacobian is invertible since gcd(U,q)=1. This proof appears already for cubics in the P-256 predecessor and extends directly to m.

Combining this radical shared membership with a retained norm A^2-f C^2=(X-r_x)U and growing m avoids dense S_{m+1} expansion and terminal symmetric multiplicity. It remains an unproved solver proposal with known ingredients, not a new ECDLP method yet.

## Prior-art map

1. Petit-Kosters-Messeng (PKC 2016): prime-field rational-map tower bases, per-root auxiliary membership, unknown solver complexity, and dedicated generalized root-finding as an open problem.
2. Nagao 2013/548 and function-first predecessor `IDEA-20260918-5a9a51`: Riemann-Roch coefficient objects before root extraction. The hyperelliptic/general-plane framing does not make an elliptic specialization new.
3. `research/pkm_symmetric_tower_20261026`: shared polynomial/divisor coefficients for Kummer membership, using quotient remainders, already measured at m=2 and m=3. Replacing S_{m+1} by retained norm coefficients is the recent composition delta.
4. P-256 F1 and cubic-divisor continuation (2026-10-25): radical shared remainder membership, actual-field correctness and depth-limited complete solver evidence.
5. `IDEA-20261029-3d0e71`: exact five-point norm + Graeffe composition. It explicitly claims adaptation and no exponent result.
6. Earlier growing-arity discussions appear in `IDEA-20260801-021` (outside the bounded-degree/bounded-arity obstruction), `IDEA-20260922-8278db`, and related DREG work. Their presence prevents claiming that letting arity grow is itself novel.

A full external novelty search is still absent. Existing internal source records sometimes rest on abstracts or search extracts. This review does not upgrade those records to full-paper verification.

## Strongest alternatives considered

- **Full PKM fall-degree relaunch:** the most mature mathematical/empirical continuation, but directly existing work, not a novel mechanism. Its resource-censored cells and missing Mac raw data justify precise replication. It tests fixed-small-arity behavior and cannot by itself establish growing-arity sub-exponentiality.
- **Joint multi-equation lattices:** the per-block convex/equal-bound result does not exclude these. This is a genuine scope opening, but no explicit lattice meeting the required reach or independent positive scaling evidence was found. Do not present the whole lattice family as disproved.
- **Partial multigraded Macaulay (`IDEA-20261029-e4edce`):** a concrete solver-engineering partner to the chosen representation. It needs valid multihomogenization, controls, and complete matrix/work measurements. It is not by itself a degree or exponent reduction.
- **Certified finite-field CDCL (`IDEA-20261029-8d82e6`) and strong algebraic backdoors (`...-2e292c`):** conceivable direct solver routes, but no demonstrated shallow strong backdoor or amortized conflict bound. Without that ingredient their sub-exponential claim is as conditional as the selected route.
- **Chart-local saturation (`...-ec08bc`):** valuable for avoiding spurious components, but the radical divisor comparator should be used first. Component removal can improve finite cost; it does not supply the uniform bound.
- **Coincidence harvesting:** the newest independently reviewed toy census supplies a stronger null than old atlas wording suggests, but scope remains limited to relation-free log-GAP half-boxes and measured coincidence statistics.
- **Same-field covers, lifts, and coordinate leaks:** no inspected candidate comes with an explicit costed non-generic relation source plus positive evidence that dominates this choice. Several atlas closures overstate generality; they should not be imported as universal impossibility theorems.

## Recommended report corrections

1. Promote norm + radical shared remainder membership to the primary implementation, Graeffe to a comparator.
2. Add the P-256 F1/cubic-divisor predecessor with exact statuses and limits.
3. Explain nonreduced terminal membership and require scheme dimension/Jacobian/radical checks, not only equal finite model sets.
4. Keep novelty explicitly unestablished; the output is a new prototype/conditional complexity research program, not a novel algorithm.
5. Do not claim the selection is uniquely best or mathematically probable. It is a defensible provisional bet because it has an explicit compact non-generic interface, recent exact predecessor implementations, and cheap discriminating correctness checks.
6. Retain the complete end-to-end gap: uniform SAT/refutation procedure, rational coefficient selection, base construction/applicability, coverage, useful relation rank, cold costs, and no hidden backward enumeration.
