# Public source phase accounting and lazy preparation

T-1831, 2026-11-07. Derived implementation accounting and producer checks;
no timing measurement or independently signed theorem. [pcc:unverified]

## Scope and sources

Prime p>3 and odd prime subgroup order r>=7 are caller-certified, r!=p,
gcd(r,p-1)=1, p,r<=u64::MAX/2. Points belong to the intended order-r subgroup.
The implementation also checks the curve, point, division value W_r=0 and
nonzero factors. Failed/degenerate paths terminate early and are excluded from
the accepted-path formulas below. No new curve domain or cohort is introduced.

Read the predecessor split-lifetime derivation, the joint-window geometry,
and the source, division and polynomial kernels. Primary formula reference:
[EFD short Weierstrass Jacobian](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html).
EFD separates squares and constants; the counts here come from this source,
where every Fp product, including squares and constant products, costs one M.
An extended-Euclid field inverse is one I. Integer arithmetic, reductions,
comparisons, additions and allocation remain separate costs, not free runtime.
KB unavailable in the saved construction round; local sources are the fallback.

## Public setup computed once

Source stores e=(r²)^(-1) modulo q, q=2^v2(p-1), during construction.
This is integer modular inversion modulo q, not an Fp inversion. Previously
raw_grid recomputed it, including calibration. For N raw_grid evaluations the
integer Euclid count changes from N to one. Public n² modulo q is still formed
for each scaled term.

A OnceLock<Result<Prepared,Error>> lazily owns the public base-stride geometry.
Prepared now owns a cloned immutable Curve context; no self-reference or unsafe
code is needed. (The copied geometry module's older borrow-oriented doc comment
is historical; its actual type owns Curve.) Zero-length windows return before
initialization. sign/raw do not initialize it. All nonempty windows on one
Source share the preparation, including concurrent first callers. Deterministic
preparation errors are cached and mapped to the same source error.

The transformation preserves geometry and arithmetic inputs: the immutable
curve, base, subgroup and partition never change during Source's lifetime.
Only public, target-independent data is retained. The initialization counter is
compiled only under source_audit; normal builds contain no audit field/method.

## Geometry

Write h=(r-1)/2=2wk+2s+epsilon, epsilon in {0,1}, 0<=s<w;
nu=(r+1)/2, delta=[s>0], g=1+w+k+delta lanes.
For 0<n<r define

    G(n)=15(bit_length(n)-1)+17(popcount(n)-1).

The binary scalar routine doubles after each nonfinal bit, costs 15M per
double, and the first accumulation from infinity is free. Later partial
accumulations have smaller positive index than the next power of two, so
neither equality nor opposite-point branches occur in this accepted domain.
The grid's w+k-1 additions likewise have distinct, nonopposite nonzero
indices: its labels from nu upward stay below r under the partition bound.
Thus its ordinary addition count is 17(w+k-1)M.

Batch affine conversion of N>0 points costs (N-1) prefix products,
2(N-1) reverse products and 4N coordinate products: (7N-3)M+I.
Point membership costs 4M. Therefore standalone geometry costs

    H_raw,M = 4 + G(nu)+G(2w)+delta G(s+epsilon)
                +17(w+k-1)+7g-3
    H_raw,I = 1.

Public preparation adds a second base membership check:
H_prepare=H_raw+4M. Curve construction separately costs 5M for the discriminant;
cloning the already validated curve incurs no field product.

## Division schedule

Let E/O be even/odd recurrence node counts, S/C distinct prepared squares/cubes,
and A the number of even output reads. These are public schedule statistics.
The common constructor validates curve and point (9M), forms 2y (1M), five
coordinate/parameter powers (5M), W3 (5M), and the W4 body (12M).
Full W4 uses two final products; normalized f4 uses one and z^4 two squares.
Consequently the typed-lifetime evaluator costs

    D_full,M = 34+S+C+4E+2O;        D_full,I = 1
    D_norm,M = 35+S+C+3E+3O+A;    D_norm,I = 0.

Arena payload is 16P bytes for P typed field slots. Public compilation,
logical/physical schedules, slot map, output array, vector headers and
constructor temporaries are additional. The node cap is not a total RAM cap.

## Multiplicative class layer

Let B(t)=bit_length(t)+popcount(t), B(0)=0, and b(t)=B(t mod q).
Class power and final projection include a final square, so these are the
actual nonzero-representative power counts of the implementation. Set d=(p-1)/q.
Outside polynomial root products/resultants the accepted raw_grid path costs

    C_M = 2+2b(e)                          # seed
        + (1+2b(nu²))+(1+2b((2w)²))      # scaled u and step
        + 4b(2)                           # seed/step squared pairs
        + 7(w+k-1)                        # recurrence updates
        + 2(w+s+k)                        # accumulated pairs
        + 4+2b(2w)+2b(2k)                 # main product
        + delta(7+2b((s+epsilon)²)+2b(2s)+2b(2)) # tail
        + 2epsilon                        # leftover
        + 1+B(q-1)+B(d).                  # inverse class, combine, project
    C_I = 0.

Each recurrence update has four numerator and three denominator products.
Pair multiplication has two. Coordinate subtraction occurs in Fp before
conversion to a multiplicative class; no additive quotient operation is used.
The class inverse is a power, not an Fp inverse. Negations are not M.

## Polynomial trace term and complete symbolic composition

Let Pi_M(T),Pi_I(T) be the actual polynomial-operation trace for constructing
root products on w babies and k centers and their half-GCD resultant_class,
plus the optional size-s prefix/single-center root resultant. This term includes
all polynomial products, reverse division/Newton steps, half-GCD matrix products,
leading-coefficient powers and scalar inversions. It is not zero or a measured
number in this round. Degree drops and leading coefficients depend on T.

Schoolbook multiplication counts a*b products for nonzero trimmed lengths a,b;
Karatsuba recursively counts its actual child products. Truncated multiplication
counts only retained pairs, or its recursive retained children. inv_series
charges its scalar inverse when the constant is not one; half-GCD charges every
matrix and remainder operation. class_factor charges its power and final product.
Thus this term can be obtained from a future actual call trace without imposing
a fictitious generic degree sequence.

    M_raw(T)=H_raw,M+D_mode,M+C_M+Pi_M(T)
    I_raw(T)=1+D_mode,I+Pi_I(T).

Construction costs 5M plus one raw(base) calibration, public schedule compilation,
partition/cost selection and the integer inversion above. Sign normalization
uses comparisons, no M or I. Identity sign returns zero without raw evaluation.
No numerical total, dominant phase or wall-time speedup follows from this ledger.

## Window composition and retained storage

For length n>0 and t finite shifts, joint conversion has N=t(g+1) points.
Initialization at identity is free; at P it checks membership (4M) and copies
prepared lanes; at other Q it costs 8M+G(nu)+G(2w)+delta G(s+epsilon)
+17(w+k-1). For each of n-1 updates charge one Jacobian addition plus g mixed
additions at their actual branches:

| Branch | Jacobian M | Mixed M |
|---|---:|---:|
| ordinary |17|12|
| identical stored representative, direct double |15|15|
| same point detected after comparisons |23|19|
| opposite point |8|4|
| an identity operand |0|0|

The order-r odd subgroup excludes finite y=0. Joint affine conversion charges
(7N-3)M+I if N>0, otherwise zero. Add D+C+Pi for each finite shift, not H_raw:
its geometry was already generated jointly. Identity shifts incur no raw_grid.

First nonempty window pays H_prepare; subsequent windows reuse it. For J>=1
nonempty calls the removed preparation work is (J-1)H_prepare, including J-1
field inversions. The first-window arithmetic is unchanged. This is a derived
operation saving with additional persistent O(g) public preparation storage,
not a measured speedup. Labels, projective and affine strides, base/context and
OnceLock state remain retained. Window snapshots, flat points, mappings,
converted slots, Source's n*g copied grids and output arrays remain additional.
No memory peak or latency dominance is asserted.

## Saved checks and reproduction

`validation-01/receipt.json`: compile/check exit 0, source_audit enabled.
Eight source fixtures check raw-only and zero-window laziness, repeated reuse,
concurrent reuse and concurrent first initialization. Counter equals one after
nonempty calls. Existing 1,108 signs/direct products/covariances, 24 identity-
crossing windows and 266 detected wrong-exponent controls are retained.
The division suite retains 160,660 output equalities and 4,100 lifetime plans.
`recovery-01/receipt.json`: compile/check exit 0; 617 targets, 5,929 source calls,
zero unresolved; inherited 63 public comparisons/511 calls. These panels are
separate from the unchanged original 4,563-case cohort.
`normal-build/receipt.json`: exit 0, seven source pins, no audit cfg.

Reproduce with `python3 generate.py`, `python3 generate-recovery.py`, then
`python3 validate.py NEW_DIRECTORY` and `python3 validate-recovery.py NEW_DIRECTORY`
from this directory (consult each wrapper's argparse before invocation).
Saved receipts bind exact rustc commands, compiler, source/output hashes;
check warnings are retained. No cloud run, timing panel or full Cargo rerun.
Next: instrument the polynomial trace under the same phase convention to
identify which remaining operations merit compression.
