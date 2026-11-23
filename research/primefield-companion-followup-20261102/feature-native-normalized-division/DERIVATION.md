# Parity-normalized division values without a field inverse

T-1825, 2026-11-07. Algebraic construction and native software checks.

## Representation and proof

Let z=2y on y²=x³+ax+b over a caller-certified prime field with p>3,
canonical coordinates, nonsingular curve and y!=0. Store

    f_n = W_n / z when n is even; f_n = W_n when n is odd.

This is a mathematical definition, not an implemented division. The initial
values are f_0=0, f_1=f_2=1, f_3=W_3 and

    f_4=2(x^6+5ax^4+20bx^3-5a²x²-4abx-8b²-a³).

Cache z and z^4. Standard division-polynomial recurrence then yields

    f_(2m) = f_m (f_(m+2) f_(m-1)^2 - f_(m-2) f_(m+1)^2).

For odd indices let L=f_(m+2) f_m^3 and R=f_(m-1) f_(m+1)^3. Then

    f_(2m+1) = z^4 L - R    if m is even,
               L - z^4 R    if m is odd.

Proof: substitute W_j=z^[j even] f_j into the standard even/odd
identities. In the even identity each side has the same z-power, whether
m is even or odd; it cancels since z!=0. In the odd identity exactly one
cubic product contains four factors z, with its position determined by m's
parity. The bases and induction on decreasing dependency indices prove
all returned values. Restore W_n=z f_n for even reads, f_n for odd reads.
No inverse, division or exponentiation is called by this cache.

The recursion uses the same dependency index DAG as the preserved value
cache, starting with indices0..4. The first nonbase odd index is5 and even
index6, so all decrements are valid. For maximum u64 n, n/2+2 fits u64.
Repeated reads reuse retained f-values; even reads still multiply by z.
The cache has no scalar-label, derivative, phase, group-enumeration or
point-table input. Index enumeration in checks is a reference oracle only.

## Exact expression costs, not a benchmark

Count field multiplication, including squaring and multiplication by small
constants, as one operation. Let E and O be counts of newly generated
nonbase even and odd DAG nodes, and A count all even public value() reads
(including repetitions and zero). The original recurrence uses 6E+6O
multiplications after dependencies; this one uses 5E+7O+A. The constructor
adds two squares for z^4 but removes one multiplication in f_4 relative to
W_4=4y*(...), a net one multiplication, and removes the single inverse of z.
Thus relative arithmetic change is

    delta multiplications = O - E + A + 1; delta inversions = -1.

This is derived directly from the expressions, not measured wall time.
Input validation, field reduction, additions, allocation, BTreeMap access,
compiler optimizations and the cost of an inverse all matter to elapsed time.
A model charging inverse I and multiplication M prefers this cache only when
I > (O-E+A+1) M, with other costs accounted separately. There is no unconditional
speed ordering. The representation and exact output equality enable a future
public-index schedule selector without inspecting a target scalar.

## Public source integration and scope

The previous general dyadic quotient source is copied with only its division
cache import changed. Seed, class resultant, normalization and window logic
are unchanged. Its prime/coprime subgroup, r!=p, u64 and lane-cap restrictions
remain. The normal metadata build has five dependencies and excludes the old
value cache and derivative sources. Source geometry/polynomial routines still
have their own inversions and square-root-sized data. The division cache's
p<=u64::MAX/2 bound keeps canonical additions safe; products use u128.
The retained Nonunit error variant preserves interface shape; prime/nonzero-y
inputs no longer require an inversion check. Composite fields remain outside
the contract.

## Checks and evidence

validation-01 compares all nonzero-y points on the eight declared source
curves over p=5,7,17,41,97,193,257,509. At each of1108 points it requests
indices0..127 and u64::MAX-1,u64::MAX: 144040 exact old-cache equalities,
144040 antipodal checks and144040 repeated-read/cache-size checks. Domain,
noncanonical, singular, off-curve and zero-y rejection controls pass.
Maximum-index tests cover index recursion, not cryptographic-size fields.

Inherited source checks retain114751classpairs,1792resultants,1108source
points/direct-halfproducts/covariance/signs,24windows and266wrong-exponent
point-function controls. Full dyadic and specialized source references remain
separate test modules. recovery-01 recovers617additionaltargets with5929source
calls andzero unresolved; the inherited63public comparisons retain511calls.
The original4563-case cohort is unchanged. All native builds/assertions pass;
warnings remain in raw logs. No timing study, cloud launch or independent
signed verdict. Full repository Cargo suite not rerun for standalone modules.

Run generate.py, then generate-recovery.py; validate.py NEW_DIRECTORY and
validate-recovery.py ANOTHER_DIRECTORY reproduce the native checks with overflow
checking. Receipts save compiler, exact commands, input hashes and stdout hash.
Do not overwrite saved directories. publication.json pins all deliverables.

## Sources checked and next work

KB MCP tools unavailable and scripts/research-kb status reports no executable.
Read ../feature-native-value-division/DERIVATION.md and current source code;
[Mathlib EDS documentation](https://leanprover-community.github.io/mathlib4_docs/Mathlib/NumberTheory/EllipticDivisibilitySequence.html)
states the standard even/odd recurrences. The parity substitution is derived
above; it is not claimed as novel or as a newly checked Lean theorem.
[Lauter–Stange2008](https://arxiv.org/pdf/0803.0728) supplies the inherited Ward
normalization context. No Lean environment digest or formal proof is asserted.
Next use the public request-index DAG to compute exact E,O,A and combine these
with full-source geometry/polynomial accounting before selecting either cache.
