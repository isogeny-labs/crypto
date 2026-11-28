# Native modular solver scans and residual labels

After the published25diagnostic, the next patch counts data scans/comparisons
in the shared u64 Wiedemann and Berlekamp–Massey kernels and hooks the cover
stream's two scalar additions per residual. It does not change a filter,
select a new scientific comparison or close complete accounting.
[pcc:unverified]

Sources checked(KB): no MCP/server executable; exact/negative queries and
status are in /tmp/ic-wiedemann-native-kb-{status,search1,search2}.log.
Fallback sources: docs/units/BITOPS.md, jv_bitops.rs rules, the actual
gaudry_cubic.rs solver/BM implementation and jv_sieve.rs residual stream.
The existing convention prices a native compare/scan step as one word_op;
allocation, pure copies, loop control and memory sizes remain excluded by
this patch. No new unit/formula or novelty assertion. [pcc:unverified]

Counted work: sparse-entry scan plus indirect vector index per term; RHS and
Krylov/Horner vector scans; BM sequence/recurrence visits and discrepancy,
bound, length and span comparisons; terminal-coefficient access/tests; and
the answer-vector comparison's length and visited coefficient pairs. Fixed
full scans aggregate their known iteration count; conditional BM/equality
paths charge only executed work. Existing arithmetic hooks and legacy ops
are preserved. Instrumentation-only nonzero counting is outside algorithm
work. [pcc:unverified]

For BM input[1,2] modulo101 the recurrence is[1,99]. The hand-checkable
contract is5modr multiplications,3modr additions,2inversions and15word events
(6in the first iteration,9in the second), with3legacy arithmetic updates.
An equality control covers empty, length-mismatched, unequal and equal
vectors, including short-circuit comparison counts. Existing randomized
sparse solve and full group endpoint checks remain required.
[pcc:unverified]

The residual labels are already reduced modulo l<2^63. Replacing
(a+al)%l and(b+be)%l with the shared modular-add helper preserves their values
and charges2modr_add events per generated residual; the sum is below2^64.
RNG streams are unchanged. New diagnostic markers name these specific
counted items. Generic graph/container and RNG/rejection/internal work
remain explicit gaps; complete totals remain null. [pcc:unverified]
