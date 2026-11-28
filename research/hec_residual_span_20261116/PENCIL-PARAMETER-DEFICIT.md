# Anchor degree, interpolation parameters, and residual support

[pcc:unverified]. Author-derived algebraic screening, not an independent proof or a performance result. No new scientific execution. Applies to smooth monic odd-characteristic models y²=f(x), deg f=2g+1, and valid nonidentity reduced Mumford anchors (u,v) with d=deg u in [1,g]. It does not require a binomial model or an automorphism.

## Exact degree and interpolation calculation

Choose w=v+u*t with deg t<=g-d. There are r=g-d+1 scalar coefficients of t. Since deg w<=g, H=(f-w²)/u is monic of exact degree N=2g+1-d. Divisibility follows from u | f-v². To leave a residual polynomial of degree at most s, forcing distinct known roots requires k>=N-s. This is a sufficient construction by known-root division, not a necessary condition for every possible relation algorithm.

At prescribed distinct points (xi,yi) of the small factor base with u(xi)!=0, each condition is

    t(xi)=(yi-v(xi))/u(xi).

The evaluation matrix has Vandermonde rank min(k,r). For k<=r every right-hand side has q^(r-k) solutions. For k>r the first r values determine at most one t and the remaining values are consistency conditions. There is no assumed independence or uniform distribution of those remaining conditions on curve points.

At the minimum k=N-s, the excess number of conditions is exactly

    k-r = g-s.

Consequently reducing anchor degree adds parameters and required known roots equally. For a quadratic residual (s=2), genus 2 has zero excess conditions, genus 3 has one, genus 4 has two, for every d. The present full-degree scalar pencil is d=g, r=1, k=g-1. A lower-degree anchor does not remove this particular interpolation obstruction. This is not a runtime lower bound, a proof of equal yield, or an exclusion of constant-factor improvements from processing skipped anchors.

Increasing the permitted degree of w does not improve this count. For exact deg w=L>=g+1, N=2L-d (leading term is minus the square of the leading coefficient of w), r=L-d+1, and k-r=L-s-1. Exact-degree restrictions remove the lower-degree stratum; no monic normalization is assumed in this case. For s=2, L=g+1 is already one condition worse than deg w<=g.

The principal relation is retained: the affine zeros of y-w include the anchor with its specified multiplicity, and the remaining divisor is encoded by H and y=w. Their sum is zero in the Jacobian after subtracting the total degree times infinity. Shared support and repeated roots require multiplicity-aware handling; distinct-point interpolation does not cover jet constraints.

## Existing-record screen

Parsing native-pencil-2lp/CONTROL.txt at parent commit 5e34480aba7 gives 22 cases, 1,344 anchors, and 21 degree skips, with at most eight skips in a case. These are already recorded prefixes ending at recovery, not new measurements or a counterfactual timing bound. A few early useful relations could still change completion disproportionately; the skip fraction alone cannot bound speedup. The degree argument removes the proposed automatic parameter advantage, so a lower-degree implementation is not the next experiment on that rationale.

## Cubic residual: concrete next question and falsifier

Taking s=3 instead gives excess g-3. For full-degree anchors, genus 3 needs one known point rather than two and genus 4 needs two rather than three. In genus 2, k=0 and a scalar parameter remains free, so scanning only occupied small-point buckets is not a complete schedule for this case. All cubic factors must split over the base field, with signed multiplicities retained; degree-three residuals are not automatically three usable large-prime columns. Bucket enumeration can generate the same principal relation repeatedly, so one row per anchor/parameter is essential.

This is a classical higher-large-prime extension, not a novelty claim. The KB screen found an existing arbitrary-hyperedge filter in src/cryptanalysis/large_prime_filter.rs via docs/ic/ECDLP_RESEARCH_TECHNIQUES.md (source:4548c0e85d47f9299ddc900c047c266d, indexed origin d390e84ab2973372534120fc4207fb65e52d4f3f, KB revision 9b2119e8fbac482b98b50521c0d604bb). Inspect the current implementation before proposing a duplicate. A search hit is not correctness evidence for a new adapter.

Before any execution: freeze a deterministic cubic-tail schedule, exact public-target row identity, and common validation/preparation/control costs. Retain unary, binary, and ternary rows; independently verify elimination against dense row spaces on bounded prefixes. The falsifier is failure to lower complete charged cost at equal verified recovery against the shared-preparation controls, including relation generation, cubic splitting, hypergraph elimination/fill, final solve and verification. Raw admitted-row growth without useful rank does not pass. A genus-specific success cannot establish the genus-2+ objective; all three genera and complete runtime remain required.

## Sources and status

The identities above are derived here, not attributed as a theorem in the references. Classical context: Falcone–Filippone, *Mumford representation and Riemann Roch space of a divisor on a hyperelliptic curve*, https://arxiv.org/abs/2303.08441 (abstract checked); Gaudry–Thériault–Thomé, *A double large prime variation for small genus hyperelliptic index calculus*, February 2005, sections 2–3, https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf. Existing multiroot-bucket/THEORY.md already excludes interpreting root-pair multiplicity inside one fixed pencil as independent rank. No asymptotic improvement or 10x result is established by this note.
