# Native recovery stage accounting

Seven generated modules instrument the exact existing recovery, source, geometry,
jet phase, value division, polynomial and sign-mask code. generate.py adds only
hooks and dependency/fixture routing. Its recorded inverse transformation recovers
every original source byte. Original sources remain immutable. The static source
map binds each original/generated SHA and every replacement, including occurrence
counts. Build and fixture identity remain separately recorded.

## Event model and coverage

Stages are setup (including loading/compiling the saved packet and calibration),
reusable preparation, query and independent point verification. Each field helper
records modular products or inversion calls; field add/sub/neg primitives are
recorded at their implementations or inline expressions. Jet operations count
their underlying field work, not an extra jet product. Decoder arithmetic is
modulo r, whereas source arithmetic is modulo p. Scalar exponent products and
phase inverses modulo p-1 are distinct events. Euclidean loop iterations cover
both scalar and field inverse helper work. Unit-operand field inversions are a
subset of inversion calls, never an additional cost to sum.

Polynomial hooks cover schoolbook accumulations, coefficient sums/negations,
root signs, field products and inverse helpers. Source hooks cover seed/inner
recurrences, scaling, normalization and anchor preparation. Verifier hooks cover
all independent affine formulas. Mask term visits, intersections, popcounts and
coefficient additions/negations are separate decoder events. Constant construction,
canonicalizing input reductions, index arithmetic, bit shifts, branches, comparisons,
parsing and memory traffic are not exhaustively priced. Thus this is arithmetic-
event accounting, not complete machine work or an elapsed-time model.

A fixed-array thread-local meter separates nested stages and rejects observed
borrow-loss/overflow at snapshot. Counter code allocates no heap state. The global
allocator forwards exact arguments to System and counts observed calls and requested
bytes per active stage. Requested bytes include full requested realloc sizes;
freed bytes reflect explicit dealloc calls, not the old block implicitly replaced
by realloc. Neither is net, live or peak memory. Counts describe this compiled
binary and can change with optimizer/toolchain. No field weights or allocator
latency estimates are assigned.

## Checks and observations

One build and one invocation pass three methods. All eight original recovery
receipts agree exactly with the unchanged native reference. Query geometry uses
one inversion; phase uses two field inversions and one scalar inverse per direct
initialization; value division uses one per finite point; source inversions equal
finite raw quotients plus seed recurrences. Every polynomial inverse in these
checks has operand one. Decoder scans477 terms and performs zero field products.
Setup/preparation/query/verification counters remain separated. Thread isolation,
nested restoration and explicit alloc/zero/realloc/free forwarding contracts pass.
The allocator check observes104 requested bytes and72 explicit freed bytes for
its controlled operations. Fifty-six earlier tests are filtered.

For k173, query products are geometry7740, phase2250, value3904, polynomial14624,
source4264; verification adds54 products and11 inversions. Polynomial records320
inversions, all operand1. Query allocator requests599560 bytes in this build.
For k0, query polynomial products13807 and300 unit inversions reflect15 finite
source points. These are exact recorded event counts, not speed estimates.
counts.json projects356 nonzero rows from test.log, without rerunning the source.

## Next optimization and remaining work

The remainder tree repeatedly calls general reversed division, even for monic
linear leaves, and constructs quotient temporaries that evaluation discards.
Next add a checked monic/linear remainder path and avoid unnecessary temporary
polynomial vectors while retaining the same product/remainder-tree algorithm.
Treat eliminating operand1 inversion calls as removal of cheap helper overhead,
not as saving the same cost as a general inversion. Compare complete recorded
stage events with identical public inputs and verify candidate points again.

Coefficient synthesis, full integer work, retained/peak memory and isolated timing
remain separate obligations. The original4563-case cohort is unchanged and did not
run here. No new cloud submission or alternative role was used.

## References and failures

The frozen native components determine arithmetic semantics. Rust GlobalAlloc
contract: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html . It documents
optimizer freedom and allocator reentrancy constraints; the forwarding adapter
uses allocation-free thread-local storage and does not unwind. Two packaging-only
generator attempts failed closed on duplicate fixture routing and a source layout
mismatch; both records remain. The sole native build/test attempt passed with two
inherited unused-field warnings retained. No speed result is claimed.
