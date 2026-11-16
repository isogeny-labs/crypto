# Native truncated product — T-1540

## What was done
Read prior monic polynomial implementation and FLINT nmod_poly/mul.html.
Construct low-product recursion preserving schoolbook cutoff12 and Karatsuba split.
Route Newton, reversed quotient and low remainder products through the new operation.
Retain full products for product trees. Copy source/recovery routing without changes.

## Decisions
Use exact full-product and independent schoolbook references. Preserve high-half
product needed for cross-term subtraction even when its shifted contribution vanishes.
Bounded software checks only; existing matched panel stays unchanged and unexecuted.

## Open threads
Run native correctness checks including complete8-target recovery; publish source,
proof and checked report. Counted integration/performance remain separate.

## Errors
None at construction start. No cloud submission.

## Checked result
One native build and one30-second-capped invocation pass all66 methods with no
filtered tests.936 low-product cases match full-product and independent schoolbook
references. Sparse/cancellation controls and all8 complete recoveries pass. Seven
prebuild source locks pass; source/recovery routed copies match monic bytes exactly.
Two inherited dead-code warnings retained. No comparative count/timing panel.
