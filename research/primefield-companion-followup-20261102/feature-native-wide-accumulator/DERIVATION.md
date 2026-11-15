# One-reduction bitplane accumulation — 2026-11-05

## Derived range proof

The existing decoder produces B signed digits d_b from T terms, with B<=63 and
|d_b|<=T. Its supported target has usize::BITS<=64, enforced by a compile-time
assertion, so T<=2^64-1. The constructor still accepts only
3<=r<=u64::MAX/2 and canonical coefficients; its largest coefficient has at most
63 bits. This proof does not require r to be prime.

Let v_h be the exact signed Horner accumulator after h digits, most significant
first, with v_0=0. Induction gives

    |v_h| <= T(2^h-1).

Indeed, |2v_h+d| <= 2T(2^h-1)+T = T(2^(h+1)-1). For h<=63 the largest bound is

    K = (2^64-1)(2^63-1)
      = 2^127 - 2^64 - 2^63 + 1
      < 2^127-1 = i128::MAX.

Both positive and negative values lie inside i128. The intermediate multiplication
2v_h before adding the next digit also fits: before the final step h<=62, so its
magnitude is at most T(2^63-2)<K. The digit accumulation across word blocks has
magnitude at most the number of represented terms processed, hence at most T.
Zero coefficient padding contributes nothing. These bounds cover the actual
operations, not only the final mathematical sum.

Consequently the private fold_digits helper may perform every doubling/addition
as ordinary exact i128 arithmetic and apply rem_euclid(r) only once at the end.
Reduction commutes with integer addition and doubling, so the result equals the
previous per-digit modular recurrence for every valid input. The divisor remains
positive and at least3. The all-zero/empty coefficient case retains its existing
early return after oracle validation and needs no reduction.

## Implementation and scope

Constructor, support/parity/coefficient planes, digit generation and oracle
behavior are byte-identical to the preceding bitplane decoder. The recovery
source is byte-identical and imports the same factored elliptic source. The only
production changes are the target-width assertion and the final accumulation
helper/call. The helper is private; its documented preconditions are established
by the Program constructor and digit-generation loop.

For B>0 the accumulation changes from B reductions to1. The saved packet has
B=9, giving9 to1 reductions, while retaining nine signed Horner steps,144
population-count calls and328 stored u64 plane entries. These are derived
source-operation counts, not timing or complete-cost measurements. Constructor
transposition, word logic, oracle/source work and coefficient-program construction
are unchanged. No larger field-width domain is claimed by the i128 proof.

## Native validation

One optimized test build with explicit -C overflow-checks=yes and one30-second
bounded invocation pass all283 methods, none filtered. The new extreme test uses
T in {0,1,63,2^64-1}, every digit count0..63, and four sign patterns: positive,
negative, alternating and sparse negative. That gives1024 digit vectors. Each
exact Horner result equals an independently ordered, checked low-to-high weighted
sum; four moduli give4096 comparisons with the previous modular fold. The full
positive and negative63-digit extrema also equal +/-K without overflow.

Synthetic maximum-T vectors exercise the arithmetic bound without allocating an
impossible number of terms. They are not large elliptic-curve recovery cases.
The retained tests check the actual repaired polynomial at all479 field inputs,
word boundaries, repeated offsets and zero behavior, oracle error order, maximum
supported modulus and all8 complete recovery receipts against the old bitplane
implementation.36 source/data hashes were locked before build.19 compiler
warnings are retained. No comparative panel or cloud job ran.

## Sources and next step

Read the preceding bitplane derivation and decoder before the change. Rust's
primitive i128 documentation gives the signed type limits and Euclidean remainder
semantics: https://doc.rust-lang.org/std/primitive.i128.html (read2026-11-03 before
interruption). The range proof above is specific to these constructor/digit
bounds; postponing reduction is a standard exact-arithmetic technique.

Next include this final-reduction variant in complete decoder/setup accounting
against the preserved bitplane implementation. Retain prior binaries/protocols
and original4563-case cohort. The supplied coefficient program remains unchanged.
