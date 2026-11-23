# Arbitrary-length quotients from a sparse leading band

T-1839 · 2026-11-07. Derived recurrence/operation bound and native producer
checks. No independently signed theorem, timing or novelty claim.

## Sources and assumptions

Primary background: [FLINT series inversion and division](https://flintlib.org/doc/nmod_poly/index.html#power-series-inversion-and-division),
including the binomial inverse/geometric series and coefficient recurrence.
Read the exact predecessor short-quotient/Newton/low-remainder kernels. KB
unavailable (no MCP; repository wrapper lacks executable), so primary and local
sources are the fallback. This is a standard sparse series specialization.

Retain caller-certified prime p with3<=p<=u64::MAX/2 and all public source
prime/coprime subgroup bounds. Input canonicalization and zero-divisor handling
are unchanged. The new route applies only to nonzero a and quotient length n>2.
Constant/linear quotient formulas and the zero-dividend path remain unchanged.

## Relevant divisor band and recurrence

Let m=deg(a),d=deg(b),n=m-d+1, A=rev_m(a),B=rev_d(b). Quotient reversal C satisfies

    B C = A modulo X^n.

Only B coefficients0..n-1 matter. Inspect b_(d-j) for1<=j<=min(d,n-1).
If at most one is nonzero, then modulo X^n the divisor reversal is

    B = alpha + beta X^j,

or just alpha when the band has no nonzero subleading coefficient. Lower
coefficients of the original divisor may be dense: this requirement concerns
only its relevant leading band, not the whole divisor.

With C_i=0 for i<0, the unique quotient coefficients satisfy

    C_i = alpha^(-1) (A_i - beta C_(i-j)), 0<=i<n,

omitting the second term when absent or i<j. Induction on i proves B C=A modX^n;
alpha is nonzero, so each step is defined and unique. Reversing C in place gives
the same quotient as Newton. C_0=A_0/alpha is nonzero, so its output vector has
canonical length n after reversal. The existing low remainder construction uses
the full divisor, preserving the contribution of coefficients outside the band.

The scan stores at most one (lag,coefficient) pair. Encountering a second nonzero
coefficient stops the scan and runs the previous Newton code, without forming
any partial quotient. General division and monic tree remainder both share this
helper; dense fallback arithmetic remains unchanged.

## Exact field-operation count

Let u=[alpha!=1], Z(C) count nonzero entries of C, and Z_j(C) count nonzero
entries C_0..C_(n-j-1). The implementation skips cross products with a zero
previous coefficient and skips scaling a zero numerator. Because alpha is a
unit, the numerator is nonzero exactly when the output C_i is nonzero.
Therefore quotient construction uses exactly

    M = u Z(C) + Z_j(C),    I = u,

with Z_j=0 when there is no subleading band term. In particular

    M <= u n + max(0,n-j) <= 2n-1

for a present lag j>=1; with no term M<=u n. This is a linear operation bound
for arbitrary n, independent of the lower divisor coefficients. It does not
assert a universal numerical advantage over Newton on every coefficient array.
The finite panel below compares actual counts and preserves all outcomes.

The scan charges up to min(d,n-1) coefficient comparisons, stopping after the
second nonzero on fallback. The recurrence makes n iterations, optional
subtractions, reductions and zero checks, plus an O(n) in-place reversal.
Those are real costs additional to M/I. Dense fallback pays this inspection
before its unchanged field arithmetic; no claim of unchanged latency is made.

## Storage

The recurrence allocates one n-entry output vector and O(1) scalar/lag state.
It uses the already produced output coefficients as its delay history, requiring
no separate ring buffer, reversed-input arrays or inverse-series polynomial.
No full reversed divisor copy is made on this path. Input normalization and
remainder/matrix/source arrays remain separate; no total-memory bound is claimed.
The dense fallback allocates as before after the scan, with only O(1) scan state.

## Validation

validation-01 passes:
- 1,536 sparse-leading-band cases over p=5,17,97,257; quotient lengths
  3,4,7,8,13,32,65,129; leading coefficients1/2; varied dividend zero patterns;
  absent lag and lags1,floor(n/2),n-1. Lower divisor coefficients are populated.
  Quotients and complete q/r pairs match the predecessor. Exact M= uZ+Z_j,
  linear bound, inversion count, zero Newton steps and scan length are asserted.
- All1,536 have lower quotient-stage product counts than the retained Newton
  reference in this fixed panel;0equal,0higher. These are deterministic algorithm
  checks, not an external benchmark, timing or universal superiority statement.
- 384dense-band cases with two relevant nonzero coefficients preserve output
  and all field-operation counts, and assert stopping after two inspections.
- 1,536lower-coefficient controls change a divisor coefficient outside the band;
  quotient stays equal while remainder changes. This checks that the source
  does not incorrectly treat the whole divisor as binomial.
- Inherited224constant/4744linear quotient formula cases,640wider quotient
  comparisons,256tree evaluations and8zero-dividend controls pass. The wider
  comparison now asserts identical field counts only for dense fallback;
  sparse eligible calls assert output/inversion agreement. Historical stronger
  count assertions remain unchanged in their original modules.
- General2592division cases,288matrix updates,432complete resultants,8320singleton
  controls and1108sourcepoints/24windows pass, with prior division-value/lifetime/
  preparation checks retained.

recovery-01 retains617targets/5929calls/zero unresolved and63public comparisons
with511calls. normal-build pins seven native inputs and excludes audit/reference
modules. Original4563cohort unchanged. No cloud run, timing benchmark, external
comparison or full Cargo rerun; historical full-suite failure remains logged.

Reproduce with `python3 generate.py`, `python3 validate.py NEW_DIRECTORY` and
`python3 validate-recovery.py ANOTHER_NEW_DIRECTORY` from this directory. Use new
output paths. Receipts bind commands/compiler/source/output hashes; normal-build
contains its command. sparse-checks.rs is a separate pinned validation input.
Compiler warnings and predecessor artifacts are retained.

Next: trace support sizes and quotient lengths encountered by the actual public
source/recovery paths before selecting a larger sparse-support cap. A general
support set J has a triangular recurrence cost bounded by u n+sum_(j in J)(n-j),
but support inspection, storage and fallback costs must be charged before any
broader dispatch is justified. This round makes no claim about sparse-band
frequency in the complete source workload.
