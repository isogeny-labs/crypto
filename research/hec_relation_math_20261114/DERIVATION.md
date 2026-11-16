# Principal-divisor relations: mathematical screen for the 10x goal

Date: 2026-11-14. The objective remains a mathematical breakthrough giving a verified >=10x
complete-cost reduction on genus 2 and above; the existing engineering gains do not establish it.

## Exact identity

Let C: y^2=f(x), with f monic squarefree of degree 2g+1 over an odd prime field. Its unique
infinite place has ord_inf(x)=-2 and ord_inf(y)=-(2g+1). Choose g+1 rational places
P_i=(x_i,y_i) with distinct x_i, and interpolate h, deg h<=g, satisfying h(x_i)=y_i.
Put A=product_i(x-x_i). Then A divides f-h^2 and Q=(f-h^2)/A is monic of degree g.

If Q=product_j(x-z_j)^{e_j} splits completely, then

    sum_i [P_i-infinity] + sum_j e_j[(z_j,h(z_j))-infinity] = 0.

Proof: y-h has pole order exactly 2g+1 at infinity, since h has pole order <=2g. Its norm under
(y -> -y) equals h^2-f. At a nonbranch root z, only the place with y=h(z) is a zero, with
multiplicity equal to the root multiplicity of f-h^2. At a branch root, squarefreeness of f
makes the root simple and y-h has valuation one. These account for all 2g+1 zeros. Thus the
left side is the class of div(y-h). This also handles residual roots overlapping anchors:
their multiplicities add at the same oriented place.

Consequences: each accepted row has at most 2g+1 nonzero columns, independent of the number
of relation trials and of the 64 walk-step columns. The split probability of Q is measured,
not assumed uniform; g! is a heuristic reference, not a bound.

## Target connection, with no omitted descent

For a target divisor D, add factor-base places to obtain D+S with split Mumford u. Then
D=sum c_i F_i - S exactly. This costs a separately charged smoothness search; no known target
logarithm is used. Express D1 by row a and D2 by row b. Principal rows are homogeneous in
factor-base logarithms L. Append a L=1 and b L-k=0. Elimination determines k if enough relevant
principal rows span. Always verify k D1=D2. Rank failure triggers more rows, counted in full.

## Prior art and scope

This identity is standard Riemann-Roch/intersection machinery, not a novelty claim.
Sources checked: Magma handbook /magma/handbook/text/1473 (principal divisors, Riemann-Roch and
geometric index calculus); Galbraith ecc2015.pdf (Sarkar-Singh incidence relations, special
families); KB IDEA-20260918-5a9a51 (coefficient-first relation proposal, primary-source prerequisite).
The special-subfield constructions there cannot be transferred blindly to these prime-field
instances. No special-subfield assumption is made here. This is a screen for whether short
principal rows remove enough measured cost to justify a further mathematical lead.

## Frozen falsification protocol

Code correctness before timing: generic interpolation vs point evaluations, exact division,
Cantor sum of every accepted relation equals identity, each D descent identity, independent
planted k recovery, and rank failure/cap retention. Test g=2,3,4 on current curve families.

Pilot: eleven existing instances x five fixed seeds, then g2 p=8009,16001 and g3 p=1009 as scaling
checks if budgets allow. Candidate includes catalogue construction, interpolation, oracle,
descent, row assembly, all solve retries, and final verification. Cap 1000000 trials, 16 solve
attempts, 10000 descent steps. Base is the full rational-place catalogue. No precomputation is
amortized away. Unit-check group sums separately from timing after those gates pass; count
that gate-only work explicitly.

Baseline: run four native arms on the same input/target: Fast and Hybrid solvers with reference
and direct oracle. Also include a one-step-place walk control, since its row weight may explain
any gain without requiring a new identity. Score the candidate against the lowest complete
cost of these controls on each instance, and against reduced-base/2LP once a competitive
end-to-end implementation exists. A gain over the original BigUint engine alone cannot meet
the goal. The current validation-heavy LP baseline is not a competitive comparator.

Success target remains >=10x at every genus 2,3,4 row in a held-out growing-size ladder with
independent replay and a novelty review. Pilot gains, rank behavior and component ratios do
not establish that conclusion. Freeze hold-out curves/statistics before measuring that ladder.
Record failed or capped attempts and preserve the active goal if this mechanism misses 10x.

### Normalization certificate (before pilot)

The logarithm map on all base places must land in the target's unique ell-primary component.
We certify ell^2 > #J using the conservative integer Hasse upper bound
(ceil(sqrt(p))+1)^(2g) < ell^2. Since D1 has prime order ell, this makes the rational ell-primary
component cyclic of order ell. Projection by the prime-to-ell cofactor then defines the map
L with aL=1. The implementation rejects inputs without this certificate. This condition is
separate from the principal-divisor identity, which is unconditional under the curve hypotheses.

The q exponent in genus2's published double-large-prime family is not a universal lower bound
for all non-generic algorithms; no such impossibility statement is assumed in this search.
