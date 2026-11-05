# Centered coefficient planes with fused signs

For a canonical coefficient 0 <= c < r, define a=min(c,r-c) and e=1 if
c>floor(r/2), otherwise e=0. Then c is congruent to (-1)^e a modulo r.
For an even modulus the midpoint is positive. The identity requires no
primality assumption; the elliptic source remains prime-field-only.

Let term j have support S_j and odd-multiplicity set O_j. A zero input in
S_j kills the term, even when its multiplicity is even. Otherwise its sign
is the parity of negative inputs in O_j. Store a coefficient sign plane E
and magnitude planes A_b. For each machine word initialize the sign mask
M=E, then XOR the existing odd-incidence rows for negative inputs. Form Z
by OR of the support rows for zero inputs. With U_b=A_b AND NOT Z,

    d_b = popcount(U_b) - 2 popcount(U_b AND M).

Summing over words gives the signed magnitude digit. Expanding a in binary
shows sum_b 2^b d_b equals the polynomial with centered coefficients over
the integers. Its Euclidean remainder equals the original polynomial mod r.
Padding cannot contribute because all magnitude padding bits are zero.
Zero coefficients and zero monomials contribute zero irrespective of sign.

## Exact bounds and domain

Keep 3 <= r <= u64::MAX/2, width <= 64 and canonical coefficient/offset
validation. Every magnitude is <= floor(r/2) <= 2^62-1, so at most 62
planes are evaluated. The retained fold supports 63 digits and satisfies
|v_h| <= T(2^h-1), with T <= 2^64-1 enforced by the existing pointer-width
guard. Its proof covers intermediate doubling. Reuse the prior exact fold
and one final remainder. Query scratch remains the 63-entry i128 array
(1008 bytes). All oracle signs are validated, in order, even for an empty
or all-zero polynomial; invalid input behavior is unchanged.

## Derived cost and tradeoff

For W=ceil(T/64), storage is (2q+1+B_c)W u64 words, where B_c is the
largest magnitude's bit length. Query population counts are 2B_c W,
with B_c signed Horner steps and one final reduction when B_c>0.
The sign row supplies the initial mask; parity updates are unchanged.
Constructor work adds centering in validation and transposition, plus the
sign-plane writes. This is not a runtime or total-cost measurement.

The saved 477-term, width-16, modulus-479 packet has W=8 and B_c=8.
It needs 328 words (2624 payload bytes), 128 population counts and 8 Horner
steps. The previous packet needed 328 words, 144 population counts and
9 Horner steps. Both use one final remainder. The new sign row exactly
replaces the removed magnitude row for this packet.

Centering never increases coefficient magnitude, but need not lower the
actual bit length of an arbitrary program. If no magnitude plane disappears,
the sign row adds W payload words, including the all-zero case. For odd r
the worst-case magnitude bit length is one less than bit_length(r-1),
not necessarily one less than the previous program's actual bit length.
The 477-term coefficient program is unchanged, as is its construction cost.

## Native checks and provenance

One optimized build with overflow-checks=yes and one bounded test invocation
passed all 331 methods, none filtered. New single-term controls exhaust every
residue for moduli 3..65, four supports (constant and powers 1,2,3), and three
ternary signs: 25,704 comparisons with direct modular multiplication.
Seven boundary moduli times five midpoint/extreme coefficients times three
signs give 105 additional controls, including offset 63 and maximum modulus.
Existing all-479 identity, 729 word-boundary cases, repeated-offset cases,
1024 extreme digit vectors / 4096 modular folds and eight full recoveries pass.
All 39 source/data hashes verify. Routed recovery is byte-identical to the
one-reduction predecessor. Raw logs retain 23 compiler warnings. No cloud
job, timing panel, or replacement of the original 4563-case cohort occurred.

## Prior work and next dependency

Read feature-native-bitplanes/DERIVATION.md and the wide-accumulator source.
Bit-serial weighted sums with AND/popcount are established arithmetic:
Umuroglu et al. (2019), sections 2.1 and 2.2,
https://www.sjalander.com/research/pdf/sjalander-trets2019.pdf (read 2026-11-05).
Rust's documented rem_euclid returns the nonnegative canonical remainder:
https://doc.rust-lang.org/std/primitive.i128.html#method.rem_euclid
(read 2026-11-05). The construction adapts centered modular representatives
to the existing zero-sensitive monomial masks; no novelty claim for signed
representatives or bit slicing is made. FPGA timings are not CPU evidence.

Next account for constructor centering, sign-plane loads, bit logic, source
work and independent verification together. Compact coefficient-program
construction remains the separate structural task; these evaluator checks
do not supply that construction.
