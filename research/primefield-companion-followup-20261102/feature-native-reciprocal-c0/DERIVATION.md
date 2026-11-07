# Reciprocal compression of the first sign-mode constraint

T-1862, 2026-11-07. Producer algebra and native correctness evidence; unsigned [pcc:unverified].

## Exact transfer within the prime scalar field

Assume caller-certified prime r, native r≤u64::MAX/2, m≥2 with 4m dividing r−1. Put h=(r−1)/2=2mn and n=(r−1)/(4m). The mode F0(v)=sum_{j=0}^{2n} a_j v^j has a_j=(-1)^(h-jm) binom(h,jm). Since h is even, binomial symmetry gives a_j=a_{2n-j}, and a_0=a_{2n}=1. Let D_j(W) be the Dickson polynomial with parameter1: D_0=2, D_1=W, D_j=W D_{j−1}−D_{j−2}. Its defining identity is D_j(v+v^−1)=v^j+v^−j.

Consequently, for v≠0,

F0(v)=v^n H(v+v^−1),
H(W)=a_n+sum_{j=1}^n a_{n+j} D_j(W).

H has degree n and leading coefficient1: half the degree of F0. This is a rational transfer with an explicit two-point fiber, entirely inside the prime scalar field. It is not a characteristic-changing map.

The actual first equation is A F0(tZ)=f0 with A=m b^h, t=b^(−m). For a nonzero m-th-power image Z, v=tZ satisfies v^(4n)=1. Thus u=v^n is one of four quartic phases {1,−1,i,−i}, where i²=−1. Write c=f0/A and W=v+v^−1. For each u form

G_u(W)=gcd(D_n(W)−u−u^−1, u H(W)−c).

Both gcd inputs have degree n. The forward implication follows directly from the identities. Conversely, lifting a root W of G_u through v²−Wv+1 and imposing v^n=u gives F0(v)=c and v^(4n)=1. All roots of v^n−u lie in F_r: n divides r−1 and u^(4m)=1, so u belongs to the n-th-power image. No extension field or generic collision search is used.

## Orientation, ramification and zero

The quotient coordinate merges v and v^−1. For u=±i their phases are different; dropping the phase filter gives false roots, as the controls explicitly demonstrate. For a monic G of degree d, construct L(V)=V^d G(V+V^−1). Then obtain the correctly oriented factor gcd(L(V), V^n−u). The power is computed modulo L by binary powering; V^n−u is never expanded. Since n is nonzero modulo r, V^n−u is squarefree. Its gcd removes repeated roots introduced by the ramified fibers W=±2 (v=±1), even when G itself has multiplicity.

The four oriented factors have disjoint root sets because each v has one phase. Their product gives exactly the nonzero first-constraint image roots. Add the factor V precisely when f0=A, since F0(0)=1. Substitute V=tZ and normalize to monic form. This equals the original gcd with Z^((r−1)/m+1)−Z, not merely its evaluation on sampled points.

## Streamed balanced construction

Only the upper n+1 coefficients are consumed. The reader buffers at most B values from the existing shared coefficient stream; it does not build F0. For each coefficient a, the Clenshaw transition is (b,c) -> (Wb−c+a,b), with matrix M=[[W,−1],[1,0]]. Balanced composition of n affine transitions produces (b1,b2) and M^n. Then H=Wb1−2b2+a_n and D_n=trace(M^n). The latter follows from M's characteristic polynomial T²−WT+1 and the same Dickson recurrence. The final stream chunk may advance recurrence state one step beyond the last consumed coefficient; that arithmetic remains charged.

Each composition uses eight polynomial products for the matrix and four for the offset, plus additions. A balanced node of size k has matrix degree at most k and offset degree at most k−1. Therefore the conservative construction bound is O(M(n) log n) polynomial work, plus the upper coefficient stream and O(n) additions at each level. A conservative temporary-space bound is O(n log n+B) field elements; H and D_n retain 2(n+1) live elements. Vec capacities, metadata and allocator overhead remain charged. Half-degree equations do not imply half total memory or half total work.

## Bounded integration and costs

Only RecoveryModes' first constraint uses the new route. Unsupported divisibility uses the old route. A bounded search of at most64 public constants computes z^((r−1)/4) and checks i²=−1; every exponentiation is charged. Failure to find i falls back, without claiming a universal bound on the least nonresidue. The standalone builder has an explicit maximum degree check.

Each branch lift requires 2 deg(G_u)≤B before allocation; the union degree must also stay≤B. The implementation never expands V^n; modular powering and gcd preserve phase. Lifting via homogeneous Horner costs O(d²) scalar work with the current sparse quadratic multiplier (or the actual kernel cost of those products); d is capped by B/2. Modular powering adds O(log n) capped polynomial products/divisions. Four degree-n gcds, normalization inversions, all setup and all failed-attempt work are charged. On a lift/union limit, the entire original first constraint is rebuilt and used. There is no claim that every eligible target benefits.

Successful compressed roots are intersected with the original implicit image domain; the rest of recovery is unchanged. No target/scalar table, root enumeration, cross-target reuse or changed source oracle is introduced. The reader/compiler retain caller-certified primality and mode-order assumptions.

## Native checks and observed limits

validation01 records optimized Rust, overflow checks, compile/check exit0, 54 transitive input hashes and stdout digest.

- 27 configurations across prime scalar fields17,41,97,193,257,281 and eligible modes2..16. All4,496 nonzero input values satisfy reciprocal F0 and Dickson identities against independently generated full coefficients.
- 308,844 branch-point checks and305,632 lifted-point checks.328 candidate orientations rejected;31 ramified-fiber incidences checked;183 just-below-required lift caps rejected.
- 2,409 full image polynomials agree coefficient-for-coefficient with the original dense-constraint/implicit-domain gcd. Includes every c for r≤97, selected c otherwise, radii1,2,r−1, zero cases, and phase controls. Enumeration occurs only in these fixtures.
- Integrated F257 curve y²=x³+x+7, base(1,3), prime subgroup order281, mode5: all281 targets agree with isolated recovery in scalar, route and all PullStats.2,881source calls, zero unresolved. Of275compressed attempts,10 complete and265 hit the existing seven-element lift/union cap and fall back. The fallback work is additional, not a saving.
- The separate inherited mode56 panel remains281targets/14,421calls; the five-curve617panel remains5,929calls. Prior cached-shift, context, stream, residue and arithmetic checks pass. Earlier simple63casebaseline retains17unresolved; later adapters'63cases resolve with511calls.

No compile/assertion failure. No timing/cloud trial or fullCargo rerun; historical missing-fixture/deadline limitation remains. Original4,563-case cohort and prospective trace protocol are unchanged. Independent signed review remains open.

Replay in a new directory:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-reciprocal-c0/validate-recovery.py /tmp/reciprocal-c0-replay-unique
```

## Sources and next construction

Sources checked (KB): MCP unavailable, scripts/research-kb status reports no executable. Read exact recovery-context firstC0 and implicit-domain code. Primary source: Bhargava and Zieve, Factoring Dickson polynomials over finite fields, definition and reciprocal fiber argument, https://websites.umich.edu/~zieve/papers/manjul.pdf. Dickson substitution and reciprocal pairing are established techniques; no novelty claim.

The cap failures identify the next representation task: keep phase-tagged compressed states through subsequent constraints instead of immediately lifting them. That requires transporting the remaining sign modes and histories through the quadratic algebra while preserving orientation, zero and final public verification. Current flow/PDF document this construction; historical and canonical graphs are unchanged.
