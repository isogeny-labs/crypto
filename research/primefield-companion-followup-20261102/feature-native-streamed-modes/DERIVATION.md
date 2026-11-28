# Parameter-only modes with descending coefficient streams

T-1853, 2026-11-07. Producer derivation and deterministic native checks; no independent signed verdict [pcc:unverified].

## Problem and implementation

The prior residue path reduced mode polynomials modulo a small retained G, but first compiled and stored all selected coefficient vectors. `StreamModes` replaces these tables with r,m,zeta and a positive coefficient-chunk cap B. It validates the same mode domain and coefficient-count work limit; its constructor tests exact order in O(m) scalar steps without polynomial trees. Prime r remains caller-certified. All native arithmetic uses the existing u64/u128 range r<=u64::MAX/2.

The traced recovery control flow is copied mechanically with only the receiver changed to StreamModes. Public source/sign generation, endpoint handling, image intersection, bounded pullback, trace replay and final public-point scalar check are unchanged. Existing Modes and its compiled adapters remain reference controls. Runtime StreamModes neither enumerates scalars nor searches for zeta. Root searches and known scalars occur only in checker fixtures.

## Descending recurrence

Put h=(r-1)/2 and c_n=(-1)^(h-n) binom(h,n), so F_e(X)=sum_j c_(e+jm) X^j, 0<=e<min(m,3). Let L=floor((h-e)/m)+1, N=e+(L-1)m, q=h-N<m. Symmetry gives the initial highest coefficient

    c_N = (-1)^q * product_(i=0..q-1)(h-i) / product_(i=1..q)i.

For an index N>=e+m, descending one mode coefficient uses

    c_(N-m) / c_N = (-1)^m * product_(i=0..m-1)(N-i)
                                  / product_(i=1..m)(h-N+i).

These follow directly from the factorial formula. Every numerator and denominator factor is in 1..h, hence nonzero in the prime field. CoeffStream computes at most B transition ratios at a time, inverting their denominator products with one prefix/backward batch inversion, then emits descending coefficients. At a chunk boundary it retains the next coefficient and index. The terminal chunk skips the nonexistent final transition. No full coefficient vector is retained.

The constructor, transition formulas and induction imply every emitted coefficient equals the existing compiler coefficient, for every admitted prime/mode. Zero, over-cap and exhausted requests fail before changing the stream. The stream's ratio/prefix/output arrays are each bounded by B or B+1; the state between calls has constant size.

## Feeding the recovery operations

For the first C0, fill only its required output vector from the descending stream and scale in place. Drop that vector after intersection. No separate F0 table survives. First C0 remains dense with L0=floor(h/m)+1 coefficients.

For later mode residues, choose w=min(deg G,B). The top chunk has ((L-1) mod w)+1 coefficients and every lower chunk has w. Generate each in descending order, reverse only its iteration order and scale locally by t^i. Horner combines chunks using H=(tZ)^w mod G. If w<deg G this is a sparse monomial; if w=deg G its coefficients are -t^w G_i. Thus no transformed full mode vector is needed. Induction proves that the result is F_e(tZ) mod G. Residues have at most n=deg G coefficients and explicit products at most 2n-1; polynomial-kernel internal workspace is separate.

C0/C2 are then combined exactly as in the prior quotient-ring construction. If initial C0 is identically zero, m=r-1 and all modes are constant; the scalar fallback is retained. A unit state remains unit. Scalar phase evaluation uses the same coefficient stream in ordinary Horner form with chunks of size B; zero denominators and the original phase verification remain intact.

## Costs and limitations

Parameter storage is constant: the native 64-bit build checks a 32-byte StreamModes object. No coefficient table is allocated at construction. This does not remove the O(m) exact-order validation, the source grid, first C0, retained G or kernel workspace. First-constraint output storage is O(r/m); later coefficient-generation auxiliary storage is O(B), or O(min(n,B)) in a residue call, plus O(n) modular state.

One complete length-L stream performs exactly L-1 stride transitions, each evaluating m numerator and m denominator factors. Excluding the cost of inversion itself, each transition uses 2m+5 field multiplications: 2m for the two products, one prefix product, two backward inversion products and two coefficient-update products. Each nonempty transition batch adds one inverse-product check. Initialization uses at most 2q+2 multiplications and one inversion (zero when q=0). The number of transition-batch inversions is at most the number of coefficient chunks. In particular, n=1 residue evaluation can invert once per transition; this is a material arithmetic tradeoff.

The Horner residue stage additionally uses ceil(L/w)-1 modular products/reductions, local coefficient scaling and the existing square/combine/gcd work. Phase evaluation uses L scalar Horner multiplications. Generation repeats on each mode use/radius/target; no amortization over targets is claimed. Since m(L-1)<=h, direct factor work for a full mode is O(r), not polylogarithmic. A faster product/recurrence evaluator that preserves bounded storage is the next construction target. No wall-time dominance or measured speedup follows from this representation change.

## Exact native checks

`validation01/receipt.json` pins rustc, invocation, 45 transitive inputs and stdout. Overflow checks are enabled. `validation01/check.stdout` records:

- 476 coefficient sequences (5,632 individual coefficients) equal existing compiled modes. Small primes 7,11,13,17,31,97,257 use all m divisors and chunk caps 1,2,7,64. Additional r=65537 panels use m=64,128,256,512 and cap17.
- 4,176 scaled residue comparisons and 1,392 scalar evaluations, including x=0.
- 42,324 sequential state comparisons over all small-prime scalars, three radii and four chunk caps. Observations also equal the compiled implementation.
- Invalid order, coefficient limit, zero chunk cap, over-cap/exhausted stream controls.
- Parameter-only construction at prime r=2^61-1, m=2, zeta=-1 without coefficient generation. This is a representation check, not a large-order recovery.
- 617 public recoveries compared against the existing enveloped reference: same result, route and call/replay/lift/image statistics, 5,929 source calls, zero unresolved. Inherited arithmetic and 63 public comparisons also pass.

No native compilation/assertion failures. These are deterministic software checks; the prospective workload collector/protocol and original 4,563-case cohort remain unchanged. No cloud run, timing measurement, or full Cargo rerun; the historical unrelated fixture/deadline failure remains.

Replay into a fresh output directory:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-streamed-modes/validate-recovery.py /tmp/streamed-modes-replay-unique
```

## Sources checked

KB unavailable: MCP not loaded; scripts/research-kb status reported no executable. Used exact predecessor `feature-native-residue-constraints/{modes.rs,residue-constraints.rs,trace-adapter.rs,new-recovery.rs}` and scalar polynomial source. Primary NIST DLMF https://dlmf.nist.gov/26.3 (factorial formula and symmetry, equation26.3.1) justifies the coefficient ratio. FLINT https://flintlib.org/doc/nmod_poly/index.html supplies standard modular-polynomial arithmetic context. Binomial recurrence, batch inversion and blocked Horner are established methods; no novelty claim. The scoped new SVG/PDF illustrates their integration; historical and canonical research graphs remain unchanged.
