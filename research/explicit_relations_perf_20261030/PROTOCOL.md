# Block normalization optimization

This software panel compares the existing dual inclusive scan (u), a
product-tree batch inverse (v), NVIDIA CUB BlockScan (w), and a second
invocation of the exact u binary (x). The only implementation factor is the
normalizer; tuple enumeration, table sorting, target search, transfers,
witness ordering, and CPU verification remain identical.

The product tree multiplies its 256 leaves upward, inverts the root once,
then propagates each parent inverse multiplied by the sibling product down.
Its source-level inversion preparation uses 3*(256-1)=765 multiplications;
the old two scans use 2*256*8+2*256=4608. Both additionally pay the same one
field inversion and affine-coordinate conversions. These are algorithmic
counts, not measured machine instructions, total field operations, or GAE.
Infinity and inactive lanes contribute the multiplicative identity.

Sources checked before implementation:

- [NVIDIA GPU Gems, parallel scans](https://developer.nvidia.com/gpugems/gpugems3/part-vi-gpu-computing/chapter-39-parallel-prefix-sum-scan-cuda): work-efficient tree traversal and memory-layout limitations.
- [CUDA asynchronous execution](https://docs.nvidia.com/cuda/cuda-programming-guide/02-basics/asynchronous-execution.html): event timing and explicit completion boundaries.
- [CUB BlockScan API](https://nvidia.github.io/cccl/unstable/cub/api/classcub_1_1BlockScan.html): custom multiplication functor, exclusive-scan aggregate and barriers before storage reuse.

The fixed design is 4 arms x 3 instances x (1 warmup + 9 measured rounds).
Inputs use p=2^61-1, a=0, b=7, base sizes16/64/256, four slots and seeds
111/222/333. Each execution checks two planted positives and two exhaustive
negatives against the independent host backend. Normalization fixtures have
257/65537/262145 lanes, varying projective scales and infinity every17 lanes.
Separate correctness tests cover 1/31/32/255/256/257/4097 lanes over three
prime fields, including 61-bit arithmetic. Memcheck covers service self-tests;
racecheck and synccheck cover the new normalization fixtures.

All arms run in fresh processes on the same full RunPod RTX PRO6000 with a
Latin rotation and alternating reversal. Round0 supplies the declared
preflight/warmup observations before rounds1-9; no performance data existed
at freeze. The 1800-second planning estimate is an unmeasured allocation
allowance, not a measured preflight estimate. Preserve the actual preflight
rows and report this limitation rather than inventing prior timings.

CUDA-event normalization timing excludes input generation and readback;
prepare timing includes allocation/build/sort; four-query timing includes
transfers, synchronizations and native witness checks. Cold timing includes
fixture construction, plan checks, context startup, preparation and queries.
The separate independent host comparison is validation overhead and excluded
from these intervals. Source, binaries, host identity and all stage values
are retained. Timings earn only L0 and remain descriptive diagnostics with
wall_valid=false; they cannot establish a certified speedup, fastest-tool
claim or full ECDLP advantage. The exact multiplication-count comparison is
the primary result; CUB's implementation-dependent count is left unknown.

The frozen native Rust analysis consumes only neutral arm labels and raw
rows, checks the complete design and equal cross-arm digests, and emits
medians, full ranges and a quantitative SVG. Numeric signatures may reveal
implementations; no human-blinded inference is claimed. A follow-up report
decodes the labels after analysis. No rows may be discarded for their timing.

One bounded non-Spot RunPod pod, no resumption, worker timeout2400s and
controller deadline2700s. Preserve any failed/partial panel. Do not retry a
failed measurement or silently replace fixtures. Build failures before the
first row can be corrected before a fresh, recorded launch. Delete the pod
after collecting evidence. Root crypto compilation remains off this local
disk-constrained machine.
