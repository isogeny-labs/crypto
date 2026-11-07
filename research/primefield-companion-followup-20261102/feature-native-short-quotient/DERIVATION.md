# Direct constant and linear quotient construction

T-1837 · 2026-11-07. Derived exact coefficient and operation formulas with
native producer checks. No independently signed theorem or timing measurement.

## Sources and domain

Primary source: [FLINT division documentation](https://flintlib.org/doc/nmod_poly/index.html#division).
It describes basecase and Newton routes and specifically identifies a dedicated
quotient-length-two Euclidean remainder routine. Read predecessor low-remainder
quotient/inv_series/mul_low and monic-remainder kernels. KB unavailable (no MCP;
wrapper reports missing executable); primary documentation and local exact code
are fallback. Standard division specialization, no novelty assertion.

Retain caller-certified prime p with3<=p<=u64::MAX/2 and source prime/coprime
subgroup assumptions. quotient_canonical is private: caller supplies canonical
nonzero divisor b and len(a)>=len(b). Public divrem still trims inputs, rejects
zero divisors and handles lower-degree dividends before calling it. Zero a
retains the prior Newton path, including its operation behavior.

## Direct coefficient construction

For nonzero a,b, set m=deg(a),d=deg(b),n=m-d+1. When n=1, the quotient is

    q0 = a_m / b_d.

When n=2, the quotient q0+q1 X is

    q1 = a_m / b_d,
    q0 = (a_(m-1) - q1 b_(d-1)) / b_d,

with b_(d-1)=0 for a constant divisor. These cancel the highest n coefficients
of a-bq, leaving degree below d. The high quotient coefficient is nonzero,
so the returned length is canonical. One leading-coefficient inverse is reused;
when b_d=1 both scaling products and the inverse are skipped. A zero/missing
b_(d-1) skips the cross product. All sums/subtractions are reduced safely under
the retained word bound; products use u128.

For n>2 and for a=[0], the helper runs the original reversal/Newton code.
Both general divrem and the monic remainder-tree path now call this helper.
Their low residual product and all other error semantics remain unchanged.
Larger quotient arrays and monic inverse-series behavior are preserved.

## Exact cost comparison and dispatch justification

Count every field multiplication (including squares/constant products) as M,
leading-coefficient Euclid as I. Let

    u=[b_d != 1],
    h=[d>0 and b_(d-1) != 0],
    ell=[a_(m-1) != 0] for n=2.

The quotient-only costs for nonzero a are:

| Quotient length | Prior Newton M | Direct M | I in either path |
|---|---:|---:|---:|
|1|1|u|u|
|2|3+3h+ell|2u+h|u|

For n=1 the prior final multiplication scales its nonzero high coefficient
by the series inverse, even when that inverse is1. The direct monic case
copies it; the nonmonic case retains that product.

For n=2, the old single Newton step forms rev(b)*v and v*(2-rev(b)*v), each
with1+h products because v is a nonzero scalar and the corrected series has
nonzero constant1. The final truncated rev(a)*inverse-series product has
1+h+ell retained coefficient pairs. Total3+3h+ell. This accounts for zero
subleading coefficients and for constant divisors; no dense generic assumption.
The direct code scales two coefficients iff u=1 and forms one cross product
iff h=1, hence2u+h.

Thus n=1 never increases M and saves1M precisely in the monic case. For n=2,
the saving is3+2h+ell-2u>=1M, with the same I. These statements concern the
quotient stage and this exact predecessor. The remainder receives the same
q,b, so its arithmetic is unchanged. Since exact q/r output preserves subsequent
Euclidean steps, no later algebraic branch changes are introduced.

Dispatch is simply a nonzero check and n<=2; it does not scan coefficients,
benchmark a threshold or allocate a cost model. Length/index/branch/modular
reduction costs are additional, so the product bound is not a latency claim.
No optimal cutoff beyond two is claimed.

## Storage

The direct path returns a one- or two-element quotient vector. It avoids
rev_a, the full rev_b copy, the inverse-series vector, Newton temporaries and
reverse_q construction for those calls. Input canonicalization copies, output
q and low-remainder arrays remain. In particular, it does not make the whole
division or source constant-space. The monic tree path shares this specialization.

## Validation and reproducibility

validation-01 checks:
- 224 constant-quotient cases over p=3,5,7 with every nonzero leading pair,
  embedded at divisor degrees0,1,8,33. Exact q/r equality and the M/I formulas.
- 4,744 linear-quotient cases: every nonzero leading pair and every subleading
  pair over p=3,5,7, embedded at degrees1 and8; plus all corresponding constant-
  divisor cases. Exact old/new quotient costs and strict M reduction are checked.
- 640 quotient-length3,4,8,13,32 cases over p=5,17,97,257; same quotient and
  identical arithmetic audit counts on the unchanged fallback.
- 256 remainder-tree evaluation comparisons and8zero-dividend count controls.
- Inherited2592general division/reconstruction/high-cancellation checks,
  288matrix and432complete resultant comparisons; all source/singleton/division-
  value/lifetime/preparation controls remain passing.

The old quotient construction is recreated verbatim in the test using the
predecessor's public inv_series and mul_low; full divrem and tree outputs also
compare against that unchanged predecessor. No test scalar enters runtime.
The fixed broader division panel still reports1792lower800equal0higher product
counts relative to its older full-product reference; this classification is not
an additional measurement of the latest quotient-only saving.

recovery-01 retains617targets/5929calls/zero unresolved and63public comparisons
with511calls. normal-build pins seven native files without audit cfg or test
references. Original4563cohort unchanged. No cloud run, external benchmark,
timing comparison or full Cargo rerun; prior full-suite failure remains logged.

Reproduce with `python3 generate.py`, `python3 validate.py NEW_DIRECTORY` and
`python3 validate-recovery.py ANOTHER_NEW_DIRECTORY` in this directory. Use fresh
output paths. Receipts bind compiler commands, source/output hashes; the normal
build command is in its receipt. quotient-checks.rs is a hand-written pinned
validation input. Preserve compiler warnings and historical evidence.

Next: derive a degree- and sparsity-aware cost envelope for wider triangular
quotient recurrences versus the existing Newton routine. Any wider dispatch
must charge coefficient inspection, inverse reuse and temporary storage, rather
than extrapolate the proved length-two bound.
