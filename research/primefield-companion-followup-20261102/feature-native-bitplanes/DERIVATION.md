# Transposed exact character-polynomial evaluation — 2026-11-03

## Derived construction

Let the supplied polynomial be a sum of T monomials with canonical coefficients
c_j modulo r and q character inputs x_a in {-1,0,1}. For each term, support S_j
records whether an offset appears; odd set O_j records its multiplicity modulo2.
The monomial is zero if any supported input is zero. Otherwise its sign is
(-1) raised to the number of negative inputs in O_j. Repeated even powers retain
their zero sensitivity even though they do not affect the sign.

Transpose these two incidence matrices into q support rows and q parity rows,
with term j stored at bit j. Also transpose coefficient bits into B rows C_b,
where B is the bit length of the largest coefficient. Empty/all-zero coefficient
sets have B=0. Every row consists of W=ceil(T/64) machine words; padding bits in
coefficient rows are zero by construction.

For each word, compute Z as OR of support rows for zero inputs, and M as XOR of
parity rows for negative inputs. In coefficient plane b let A_b=C_b AND NOT Z.
Its signed contribution is

    d_b = popcount(A_b) - 2*popcount(A_b AND M).

Sum d_b over the words, then evaluate sum_b 2^b*d_b modulo r by descending Horner
steps v=(2v+d_b) mod r. Expanding each coefficient into its binary digits and
interchanging these finite sums proves equality to the original polynomial for
every ternary input vector, not only elliptic-source inputs. Padded terms cannot
contribute because all their coefficient bits are zero. No scalar label or
point-to-scalar lookup is used by this decoder.

## Representation and arithmetic

Supported domain is the previous decoder domain: 3<=r<=u64::MAX/2, width<=64,
canonical coefficients and validated offsets. Primality is not needed for this
finite-ring evaluation identity. The ECDLP source remains prime-field-only.
B<=63. On the supported64-bit platform each signed digit has magnitude at most
T<=usize::MAX. Therefore digit accumulation and 2v+d_b fit comfortably in i128;
rem_euclid handles negative sums. Offset63, width64 and modulus boundaries are checked.
A checked layout multiplication rejects index-space overflow; allocation failure
remains the ordinary allocator failure boundary.

The compiled plane payload is (2q+B)*ceil(T/64) u64 entries. Evaluation calls the
oracle once per offset in ascending order, preserves failure at the first invalid
sign, and uses no decoder-owned heap allocation. Scratch includes63 signed128-bit
digits (1008 bytes) and scalar masks. Empty/all-zero programs still validate every
oracle sign before returning0. Queries use O((q+B)*ceil(T/64)+B) word/accumulator
work and B modular Horner steps. This bound includes conditional support/parity
processing and excludes the character oracle itself.

For the saved T=477,q=16,B=9 packet: W=8, plane payload328 words=2624 bytes,
2BW=144 count_ones calls and9 modular Horner steps. These are derived code/layout
counts, with dimensions asserted by a native test, not timing measurements.
The old term representation has three u64 fields per term, totaling1431 field
slots; the plane layout uses328 slots. Container metadata, transient constructor
inputs, allocator capacity, query stack, source memory and RSS are not included
in that payload comparison. Setup now transposes incidences and coefficients.
A full cost comparison must include this work, word logic, population counts,
signed accumulation and modular reduction; nine Horner steps alone are not total
cost. The polynomial still has477 supplied terms: this changes its evaluator,
not the cost of finding the coefficient program or its term-count growth.

## Exact checks

One optimized native test build and one30-second-capped invocation pass all237
methods, none filtered. The retained independent direct-multiplication reference
checks all479 inputs of the actual repaired polynomial and confirms output X.
All27 ternary vectors for the repeated-offset monomial fixture pass. New checks
cover9 term counts (0,1,63,64,65,127,128,129,477) times81 ternary vectors, or729
word-boundary cases, including padding and zero/odd interactions. Invalid-oracle
call order, constant/empty/all-zero programs, offset63, maximum supported modulus
and existing rejection controls pass. Full recovery on all8 original integration
fixtures matches the prior factored-source implementation; labels remain checker-only.

The source module is imported unchanged. Recovery differs only in two module
routes, verified by reversing those edits.33 source/data hashes were locked before
build.15 compiler warnings remain in raw logs. No comparative panel/cloud job ran.
Previous panels and original4563-case cohort remain unchanged.

## Prior work and next step

Read the existing signed-mask decoder and compiled recovery source before design.
The binary-plane/AND/popcount technique is established: Umuroglu et al.,
Optimizing Bit-Serial Matrix Multiplication for Reconfigurable Computing (2019),
sections2.1–2.2 and the high-to-low accumulation discussion, read2026-11-03:
https://www.sjalander.com/research/pdf/sjalander-trets2019.pdf.
That work supplies bit-serial arithmetic context; its FPGA performance results
are not evidence about this CPU decoder. The adaptation here adds exact monomial
support-zero and multiplicity-parity masks and modular signed accumulation.
No novelty claim for bit slicing or popcount dot products is made.

Next instrument the complete decoder including bit operations, popcounts,
integer/modular work and setup, then compare it within the unchanged recovery
pipeline under a separately frozen protocol. Preserve the new candidate even if
its complete-cost tradeoff is adverse.
