# Monic remainder specialization

The candidate keeps the existing product tree, Karatsuba multiplication cutoff12,
Newton inversion and reversal-based quotient construction. It changes three paths:
(1) a series with constant coefficient1 starts from inverse1=1 directly;
(2) a linear tree leaf X-x evaluates its current remainder by Horner;
(3) internal monic division reconstructs only coefficients below the divisor degree.
General divrem remains available with its original algorithm and invariant check.

## Why the low remainder is sufficient

Let canonical monic B have degree d and A degree m>=d. Reversal and series inversion
modulo X^(m-d+1) determine Q exactly, as in the original division. Since deg(A-QB)<d,
the remainder is determined by (A-QB) modulo X^d. Only Q modulo X^d and B modulo X^d
can contribute. The candidate therefore multiplies these truncated inputs using
the existing fast multiplier, truncates that product to d coefficients, and
subtracts from the first d coefficients of A. It still constructs the Newton
quotient internally; it does not claim to eliminate every quotient temporary.
It avoids a full-length A-QB reconstruction and the unused returned quotient pair.
Constant monic B=1 yields zero. Inputs of degree below B return normalized A.

For a leaf, A mod(X-x)=A(x); Horner preserves the original output order, including
repeated points. It normalizes input coefficients modulo p. Empty A evaluates0.
All tree divisors are monic by construction, so their reversed constant is1.
The entire tree evaluation path therefore requires no field-inverse helper call:
leaf evaluation has none, and each internal series starts at the known unit.
This is a structural property of the code, not a measured timing reduction.

## Exact local checks

One build and one filtered native invocation pass4 methods.84 deterministic
monic-remainder cases across F7 and F509 match unchanged general division, covering
degrees0,1,2,5,14,27 and numerator lengths0,1,2,5,13,29,65. Ten multipoint cases
match both original tree evaluation and direct Horner, including repeats, empty
inputs, noncanonical coefficients and products spanning the Karatsuba threshold.
Unit/nonunit series and foreign-modulus errors pass. All8 saved public-point
recovery receipts equal the unchanged native pipeline and pass candidate point
verification. Sixty earlier/dependency methods were filtered out.

Two routed source/recovery copies change dependency and fixture paths only;
routing.json records reversible transformations and source/generated hashes.
Earlier sources and reports remain immutable. Ten pretest source/data hashes pass.
No candidate arithmetic/allocator comparison or timing was collected in this round.

## Comparison protocol and remaining work

comparison-protocol.draft.json passes canonical experiment.protocol/v1 validation
with zero findings. It plans baseline/candidate/A-A, eight fixed public inputs,
one-factor backend comparison, exact paired event differences, complete recorded
failures, separate setup/preparation, one thread and bounded isolated EKS execution.
Its freeze status is draft. Candidate instrumentation, matched runner, binary
identity, preflight, frozen analysis and live admission/reservation remain missing.
No launch authorization or capacity is inferred from schema validation.

Next extend the reversible arithmetic hooks to the three changed paths, build the
matched runner, freeze the complete comparison and reconcile admission. The
original4563-case validation cohort is unchanged; this small comparison will not
replace it. Other machine costs and coefficient construction remain separately priced.

## Sources and actual failures

The original polynomial source and native-accounting report identify the target.
FLINT modular-polynomial documentation provides context for standard division and
evaluation: https://flintlib.org/doc/nmod_poly/index.html . No novelty or external
backend comparison is claimed. The canonical protocol tool/schema were obtained
read-only from isogeny-labs/crypto-autoresearcher; hashes are retained. System Python
lacked Draft202012Validator, so validation used the existing research venv. The
first schema run found9 enum-value errors; corrected draft passes. Both failure
logs remain. Native build/tests did not fail; two inherited warnings are retained.
