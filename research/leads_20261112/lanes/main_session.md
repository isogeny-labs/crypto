> Agent report, verbatim (2026-11-12). Proposals only; nothing was run; every statement in this file is `[pcc:unverified]`.

# Main-session (frontier tier) notes, 2026-11-12

## The bar for prime-field Gröbner routes: entropy threshold against enumeration
Setting: m summands, factor base a coset of mu_N, N = 2^k, chain presentation with m k variables of weight 1.
Brute-force decomposition of one target: enumerate m-1 summands, solve the last one: 2^((m-1)k) field-op units.
The Gröbner/XL route must at least write the Macaulay columns of weight <= LFD = s * (m k): about 2^(m k H(s)).
Computed (python, this session):
  m=2: columns < enumeration needs LFD/k < 0.220; linear algebra at omega=2 (sparse) needs LFD/k < 0.083; omega=2.81: < 0.054
  m=3: columns needs LFD/k < 0.522; omega=2: < 0.184; omega=2.81: < 0.117
  m=4: columns needs LFD/k < 0.858; omega=2: < 0.290; omega=2.81: < 0.182
Measured m=2 prime slope 1/4 = 0.25 > 0.220: the matrix alone already exceeds brute force (2^(1.087k) vs 2^k), and
the proved lower bound (slope >= 1/14 = 0.071, written) is above the omega=2.81 threshold 0.054, so for m = 2 the
Gröbner route on family 0.1 can never beat enumeration asymptotically even if the slope dropped to the proved floor,
unless the linear algebra is sub-quadratic in the column count. For m = 3 the bar is LFD/k < 0.184 (sparse) and the
value of LFD(k) for 3-summand chains is NOT in any record I have; this is the single cheapest decisive measurement.
Caveats: equal yield per attempt in both arms (same base, same target distribution); the Macaulay count is a lower
bound on F4 work (structure could make the effective matrix smaller); enumeration is priced at one unit per summand
tuple (a quadratic solve plus an N-th power test, about k multiplications, is a constant factor).

## Mechanism-level ideas of my own (to be merged with the lanes)
1. Arity-vs-slope grid (prime, m = 3): compute certified LFD(k) for m = 3 chains at k = 3..7 (3k <= 21 variables,
   weights <= 6) with the existing closure engines; preregister: if LFD/k at k = 7 exceeds 0.184 the m = 3 Gröbner
   route is dominated by enumeration at omega = 2 and family 0.1 closes for m <= 3; if below, size the F4 at m = 3.
   Nearest: LEAD-ICP-full-width-arity-grid (body unknown, vault); difference: the entropy threshold as the pass line.
2. Radix of the chain: the locality proof is radix-agnostic (far exponents cannot connect); the constant may not be.
   Radix-3 chains (p = 1 mod 3^k) or radix-4 (u^4 = next) change the monomial count per weight. Test: LFD(k) for
   radix 3 at small k against radix 2 at equal log N. Nearest: LEAD-ICP-digit-presentation-solving-degree.
3. Evaluation picture: with 2^k | p - 1 the ring K[x1,x2]/(x_i^N - 1) is K^(N x N) by evaluation on mu_N^2; the
   closure is a compressed evaluation table and LFD measures the compressibility; the certificate "codim V_D = |Z|"
   is this isomorphism. Consequence: for m = 2 nothing beats O(N) per target; for m = 3 the question is whether a
   batched 2-summand solve over N shifted targets can be done in o(N^2) by exploiting the smooth group mu_N (FFT-type
   multipoint evaluation on a subgroup of smooth order). Nearest: LEAD-ICP-generic-table-decomposition,
   LEAD-EXO-four-list-ktree-join; difference: subgroup smoothness is the lever, not a table.
4. Free-extension solver: below LFD the closure at depth k+1 is a direct sum of four shifted copies of depth k
   (measured profile recursion; proved for the ideal spaces S_D from depth D + Lambda(D)); so F4 work at depth k+1
   for degrees < LFD is replay, and only the top degree is new. A speedup only if the top-degree repair is small:
   measure the repair count when extending V_6(12) -> V_6(13) by free extension plus sweep on the binary GPU engine's
   repair mode. Not a route to beat enumeration (see the bar), but a cheaper way to run the degree-law experiments.
5. Reframing of 0.1 for the atlas: "open" -> "open only for m >= 3 with LFD/k < 0.184, or slope < 0.083 at m = 2".
