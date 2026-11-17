# Exact reduction for a fixed oriented CM label

2026-11-08, T-1888, PR #3674. Status: algebraic derivation with independent integer replay of the existing five certificates. No new minimizing search, benchmark, or timing is reported here. This reduction supplies a finite search with an optimality certificate for the next construction; it does not assume that smaller degree means faster evaluation.

## Domain and completed square

Work in the Frobenius order Z[pi], with pi^2-t*pi+p=0, n=p+1-t, d=4p-t^2>0, and alpha=a+b*pi for integers a,b. The prime-order rational subgroup has size n, so pi acts as 1 and its label is mu=a+b modulo n. Put s=a+b, hence a=s-b. Then

```
N(alpha) = a^2 + t*a*b + p*b^2
         = s^2 + (t-2)*s*b + n*b^2,

4*n*N(alpha) = (2*n*b + (t-2)*s)^2 + d*s^2.             (1)
```

Proof: substitute a=s-b in the first line and collect coefficients. Completing the square leaves 4*n-(t-2)^2=4*p-t^2=d. Thus every representative with the prescribed label satisfies

```
N(alpha) >= d*s^2/(4*n),       s = mu + j*n, j in Z.    (2)
```

This holds independently of any class-number or L-value estimate. Its scope is the displayed Frobenius order. A larger endomorphism order can contain elements with denominators in this basis; no exclusion of those elements follows without a certified endomorphism-ring index.

## Keep the orientation

Let ell != p be an odd prime with distinct Frobenius roots rho and rho' modulo ell, and let K_rho be the already certified signed eigenline. Annihilating K_rho requires

```
a + b*rho = s + (rho-1)*b = 0 (mod ell).                (3)
```

To retain K_rho' require b != 0 modulo ell: under (3), the opposite value is b*(rho'-rho). For rho != 1, as in all five current certificates, set

```
b0 = -s/(rho-1) (mod ell).
```

If b0=0, omit this s when requiring exactly the selected eigenline at level ell. Otherwise every allowed coefficient is b=b0+ell*z. At fixed s, (1) is minimized by the one or two integers z closest to

```
(-(t-2)*s - 2*n*b0) / (2*n*ell).                      (4)
```

Use exact signed floor/ceiling division and retain both candidates on a tie. This follows because only the square in (1) depends on b. The rho=1 case needs a separate branch: s must vanish modulo ell, and b ranges over the integers not divisible by ell. Formula (4) must not be used with a nonexistent inverse.

## Finite global search and certificate

Start with any verified oriented candidate alpha0 of the same label, with N0=N(alpha0)>0. The existing certificates provide such a candidate. Every candidate of norm at most N0 has

```
|s| <= B,    B = floor(sqrt(floor(4*n*N0/d))).           (5)
```

Enumerate only s=mu+j*n in [-B,B], with j bounds obtained by exact ceiling/floor division. Apply (3)-(4) to each s and compare the exact integer norms. The original candidate is feasible, so this finite set contains a global minimum within the stated orientation and order. An independently checkable certificate consists of the input identity/hashes, N0, B and its two square inequalities, both j endpoints, every retained s and nearest pair of b values, and the selected a,b,norm. Its verifier must recompute completeness, not just check the winning row. There is no heuristic search radius or omitted tail.

This is a derived algorithm, not an executed five-curve minimum claim. Its admissible set fixes the selected eigenline at ell; it does not additionally certify separability of alpha or horizontal factorization at every other prime. Those are further checks for a proposed map route. Adding such restrictions to the minimization requires updating the admissible set and nearest-candidate completeness proof. A future comparative run must publish/freeze its exact source, inputs, tie key, finite resources and full relation/evaluation cost before taking affected data. It must preserve the current mu and orientation when comparing the lift.

## Point action and closure obligation

The globally defined straight-line endomorphism is alpha(P)=[a]P+[b]pi(P). On rational points it simplifies exactly to [s]P. Any evaluator must charge its actual scalar preparation and group work; a small norm is not a time measurement. The label and nonzero opposite-root check certify the selected kernel and hence a factorization alpha=psi*phi through the separable quotient phi. Its residual degree is N(alpha)/ell. This statement does not supply the expanded global polynomials or a cheaper evaluator for psi.

For an explicit map route, its endpoint isomorphism still has to be carried through the point action and checked. The existing full polynomial closure sigma*eta*phi=[ell] remains unchanged. Its charged rational-subgroup correction [mu/ell] does not become the global psi merely because the scalar label agrees. Thus the next lift certificate and the next executable closed route remain separately checkable objects.

## Sources and replay

The degree quadratic form and rational Frobenius fixed-point identity are standard; see Andrew Sutherland, [MIT 18.783 Fall 2023 Lecture 7, pp. 1-2](https://math.mit.edu/classes/18.783/2023/LectureNotes7.pdf). The endomorphism operations and dual cancellation used by the existing closure are in [Lecture 6, sections 6.2-6.3](https://math.mit.edu/classes/18.783/2023/LectureNotes6.pdf). Both primary PDFs were read. Equations (1)-(5) above are derived explicitly from those facts; no novelty claim is made. Internal inputs: CONSTRUCTION.md, fixtures-v2/published-curves.json, fixtures-v2/marked-relations.json and the existing native signed-kernel checker.

Sources checked (KB): no research-kb MCP tools are loaded; `scripts/research-kb status` failed with exit 127 because the migrated wrapper is absent. The intended searches were "fixed Frobenius scalar label norm closest lift" and "oriented CM relation Frobenius order norm lower bound". No successful catalog search or absence-of-prior-art claim is inferred. No ecdb tools are loaded; the replay uses the unchanged, already published exact curve/certificate inputs rather than a new curve-fact query.

`lift_identity.cpp` independently replays the five existing certificates with GMP integers, recomputing trace, norm, completed-square identity, rational label, selected/opposite eigenline and quotient degree. Its adversarial controls alter the saved norm, scalar label and orientation. This software qualification is not a minimizing search or a Lean formalization. The initial Boost-based build failed because its headers were absent; that attempt is retained, and the replacement uses the separately installed GMP development package.
