# 63-bit field boundary — GPU validation pending

This follow-up admits prime fields p<2^63 while retaining r<2^63. It is separate
from frozen v11 and v13, which remain pinned to their original 62-bit sources.
No new frontier protocol or GPU run has been launched for this extension.

The shared Montgomery operations are unchanged. For R=2^64 and p<R/2,
T<pR and m<R imply T+mp<2pR<R^2; the post-shift sum is below2p<R.
Canonical addition and subtraction intermediates also fit u64. HAC14.32/14.33
state the Montgomery input and reduction bounds; applying them to this word
width justifies the wider arithmetic domain. The mirrored Rust prime_fast
engine keeps its existing guard. Boundary host fixtures compare multiplication,
addition and subtraction with u128, and affine operations with a separate
canonical implementation. New device fixtures perform the same comparisons.

Hash primes in[2p,4p] may now use the entire u64 range. Overflow-safe subtraction
implements modular add/double multiplication above2^63; the existing narrower
path stays unchanged. CPU products useu128; full-width hash inversion uses
Fermat exponentiation, preserving the signed EEA bound for field/scalar work.
Hash addition, interval endpoints, affine doubling/RHS and Hasse radicands use
bounded modular additions oru128. An empty representable prime interval is a
limit, not a wrapped search. Fields near2^63 can have no u64 hash prime above2p;
future fixtures must retain this constraint and the existing order bound.

Finite compressed point keys still fit64bits. The conceptual terminal prefix
key atx=2^63 does not; its offset is written directly as table size. Device tests
cover62/63-bit index domains, empty buckets, duplicate minima, endpoint queries,
exact compact/full lookup agreement, and arithmetic near both boundaries.

Host C++ syntax and standalone arithmetic fixtures pass. Native Rust tests,
CUDA compilation, device differential checks and sanitizer runs remain pending.
This is a width extension of the generic decomposition pipeline, not an
asymptotic algorithm, tensor backend, certified speedup or hardware maximum.

Literature checked2026-10-30 before implementation: Handbook of Applied
Cryptography, Chapter14, algorithms14.32/14.36 and Note14.33:
https://cacr.uwaterloo.ca/hac/about/chap14.pdf
Existing contracts reviewed: ../rootfind/fp64.cuh, WIDE-HASH.md, point-key and
prefix-index tests, shared hashed_ecdl.rs and the supplied inverse-hash paper.
