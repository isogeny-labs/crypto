# N83 five-byte index capacity boundary and next factor-base comparison

The current symmetric one-root index reaches the 75% worst-case load boundary
at K4405. Its 2³¹ five-byte slots occupy 10,737,418,240 bytes. At K4406 the
existing load75 sizing rule requests 2³² slots, or 21,474,836,480 bytes.
`capacity.py` reproduces the producer's state bound and slot sizing with exact
integer arithmetic. It also finds K4549 as the largest K whose two-root
upper load is at most 80% of the existing 2³¹-slot table. The K4405 measured
process reached 22,355,877,888 bytes peak RSS in the independently replayed
1,536-point Tier 1 screen; its online selector remained `BOUNDED_UNKNOWN`.

The next implementation will add an opt-in fixed 2³¹-slot five-byte layout
with a declared 80% upper-load limit. It must preserve exact-key reconstruction,
first-witness semantics, root collision handling, and the CPU fallback. The
initial comparison will include the exact K4300 and K4405 controls, plus
K4500 and K4549 under the new layout, with actual usable base counts, hashes,
source and binary identity, memory, setup, rank, one-target online intervals,
same-point rho, and independent arithmetic replay recorded per run. The new
layout is a different candidate even when K and base points match.

This directory is a capacity analysis and experiment preparation. New candidate
IDs and measured costs remain null until the implementation, exact manifests,
frozen public targets, resource envelope, dispatch, and replay gates are ready.
The previous RunPod campaign's conservative upper charge was $5.710216
against its $5 ceiling, so this preparation launches no paid instance. A new
bounded allocation needs its own reviewed budget and protocol.
