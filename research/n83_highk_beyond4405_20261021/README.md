# N83 five-byte index capacity boundary and next factor-base comparison

The current symmetric one-root index reaches the 75% worst-case load boundary
at K4405. Its 2³¹ five-byte slots occupy 10,737,418,240 bytes. At K4406 the
existing load75 sizing rule requests 2³² slots, or 21,474,836,480 bytes.
`capacity.py` reproduces the producer's state bound and slot sizing with exact
integer arithmetic. It also finds K4549 as the largest K whose two-root
upper load is at most 80% of the existing 2³¹-slot table. The K4405 measured
process reached 22,355,877,888 bytes peak RSS in the independently replayed
1,536-point Tier 1 screen; its online selector remained `BOUNDED_UNKNOWN`.

The capacity proof is checked in `Capacity.lean` with the pinned Lean 4.35.0
toolchain. `maxStates` is the producer's symmetric N83 state bound:
`83 * (K * (K - 1) / 2) + 42 * K`. `maxStates_mono` proves this bound is
monotone on natural K. `max_k_load75` proves any K with worst-case two-root
load at most 75% of 2³¹ slots is at most 4405; `load75_4405` proves the
boundary is attained. `max_k_load80` and `load80_4549` give the corresponding
80% boundary. `lean-check.log` records a successful module build and
`#print axioms` reports only `propext` and `Quot.sound`. `lean4checker --fresh`
was unavailable on this host.

The opt-in `KIC_INDEXED5_FIXED31=1` mode in
`examples/koblitz_orbit_dlp_fast.rs` fixes the five-byte table at 2³¹ slots
and rejects a worst-case two-root load above 80% before allocation. It
requires the exact N83 one-root, ID30 and five-byte index modes, and excludes
the existing load75 and fixed30 sizing modes. The slot encoder and exact-key
reconstruction are shared with the checked ID30 path. The new mode has a
separate result flag; the earlier source and candidate identities are
unchanged. The initial comparison will include exact K4300 and K4405 controls,
K4405 under both sizing flags as a same-layout control, then K4500 and K4549
under fixed31. Record actual usable base counts, hashes, source and binary
identity, memory, setup, rank, one-target online intervals, same-point rho,
and independent arithmetic replay per run.

This directory is a capacity analysis and experiment preparation. New candidate
IDs and measured costs remain null until exact manifests, frozen public
targets, resource envelope, dispatch, and replay gates are ready. The local
release example suite passed 22/22 tests with Rust 1.93.1; its command and
output are retained in `rust-example-tests.log`. The capacity model, Lean
module, `rustfmt --check`, and `actionlint` passed. The branch workflow
repeats the capacity, Lean, and Rust example checks with Rust 1.99.1 on its
exact head. A full K4500 or K4549 process has not yet been measured.
The previous RunPod campaign's conservative upper charge was $5.710216
against its $5 ceiling, so this preparation launches no paid instance. A new
bounded allocation needs its own reviewed budget and protocol.
