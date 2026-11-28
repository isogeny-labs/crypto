# Section-query v1: prospective native development controls

Implements the supplied-section query/restriction from section-search-v1.
The query receives public Spec, target preimage R, public Q=[8]R, and six
section coefficients. It receives no original column list or witness. A
coefficient constructor is outside this component. This is bounded native
correctness qualification, not an experiment campaign, benchmark or yield study.

## Algebra and implementation contract

Use Fp2[X]/S, monic 1<=degree(S)<=6, and its free quadratic extension
v²-(4u²-2)v+1. Retain repeated roots and zero divisors. Baby-step/giant-step
geometric products use homothety construction, chirp convolution and scalar
Karatsuba; invert only known nonzero field scalars. Karatsuba is O(n^log2(3));
the product bound is O(k^(log2(3)/2)), about k^0.7925, for fixed quotient degree.
This replaces the hypothetical quasi-linear backend, not coefficient search.
The homothety/geometric-evaluation methods are classical prior art.

Support tests retain sixth powers. Interval restriction uses gcd(S,H_J^6),
not squarefree truncation. Degree<=6 root extraction uses at most64 fixed
split shifts; failure to split is an incomplete/resource refusal. Recover
local ordinate multiplicities, trace sign and raw subgroup index; check the
unchanged member/target circuits and independent original signed-row replay.
Raw aliases remain distinct; repeated raw labels and identity images reject.
Finite two-torsion target charts and unsupported section leading coefficients
refuse, never certify target absence. Public target-fibre construction and
coefficient discovery are not implemented here.

## Fixed controls, before execution

1. Scalar Karatsuba versus schoolbook in p101 and actual Fp2 at lengths
0,1,3,8,9,17,33 using deterministic coefficients (14 comparisons).
2. Full geometric-product remainders versus direct multiplication: both fields,
S=(X-2)^d for d=1,2,6, n=0,1,2,3,8,9,17,31 and offsets0,3 (96 comparisons).
3. Explicit nonzero nilpotent and zero-divisor controls at rho=1,
S=(X-1)^6,t=1+(X-1): epsilon^5 !=0 and epsilon^6=0. This backend stress
case is not an original-base parameter claim. Plus full-length actual-field
degree1 remainder at n20633 against direct multiplication.
4. Supplied-section original-witness controls: p101/A95/h(17,12)/a17/q3/k8;
actual p/A486662/h from actual_four/a60824497/q213156431 with k31 and k20633.
Construct known sections from the first six retained public raw columns,
trying the next sixth column up to32 times if a section chart is unsupported;
record every attempt. Query receives only R,Q,c. These sections are known-answer
controls and reveal no coefficient discovery/yield claim. Original full-base
parameters are retained in the k20633 control, but its section is supplied.
5. Toy raw-alias control chooses two distinct raw columns with equal [8] image
from the24 declared toy columns and fills four others; keep the recorded
construction. Repeated-prepoint section tests support multiplicities and
must reject repeated original raw labels. Coefficient mutation is not assumed
necessarily useless: it must either reject or return an original checked row.
6. Malformed modulus/degree/canonical coefficients/nonunit chart, product n cap,
scalar-work cap, and create_new output collision controls. Resource refusal
is not absence. Any unavailable control is reported explicitly.

Public native API admits only p101 or p=2^255-19; primality is not inferred
from parsing. q and a must satisfy the existing fixed parameter contract.
Hard caps: product n<=65536, quotient degree<=6, p bits<=256, at most64
factor shifts, native scalar multiplication budget100000000 per engine,
polynomial scratch length<=2048, cumulative reserved coefficient slots
<=200000000. Counters cover only the new arithmetic/reservations; unchanged
circuit/group/index code, parsing, serialization, allocator overhead and RSS
are not fully priced. No timing, GAE or fastest-tool comparison is admitted.

## Evidence and failure handling

Use a shell-only launcher, create_new raw JSONL distinct from stdout/stderr;
flush each completed arm. Transport-only invocation/collision is checked with
the same launcher before substantive calls. Pin source before final execution,
copy immutable binary/qualifier, and keep distinct BUILD/PRECALL/POSTCALL
manifests. The final worker has a180second wall cap; largest original-k arm
runs last. A timeout retains completed rows and marks the missing completion
inconclusive; it does not reject the route. Retain failed source/log versions.
Focused release tests/fmt/strict Clippy qualify the new component only.

All evidence is producer unsigned pending maintained signed validation.
No new mathematical theorem is inferred from native controls. Existing
section-search identity records/partial unit-cancellation endpoint remain
unchanged and cover only their original declared statements.
