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

The base-only path produced exact first-K signed-Frobenius bases on the local
correctness host without allocating the root table. K4405 reproduced the
archived BLAKE3 base hash `d64fcee7ba4975873859d256302872681c82b99fbe687ff4fba0bf3ba3a4c8d2`.
K4500 contains 747,000 usable points in 4,500 folded columns, with base hash
`6f3305328b265b70f7a2d53e97be2c4b70c3da291ef4cca4693dac6dae468f95`.
K4549 contains 755,134 usable points in 4,549 folded columns, with base hash
`65bcc74cd45d33881bcaad17216fe724d2666072230230ed6868eb7187b968af`.
The full ordered dumps and receipts are in the 36,005,831-byte
`base_stage.tar.gz` at
`s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_fixed31_base_stage_20261021/`.
Its SHA-256 is `92e3b0b463a8a8b1ca4591e03fc81fde631f3e4de5b4288fc63b1ba3b297b236`,
confirmed by a fresh S3 download in `base_stage_custody.json`.
`identities/catalog.json` records the four source-bound IC1 candidate
IDs for the K4405 load75/fixed31 controls and the two larger bases. These are
exact candidate identities, with run costs still unknown.

`generate_fresh.py` sealed 1,536 new public screen points and 16 one-target
holdout points. The respective public-stream SHA-256 values are
`e62e8ea5b77dbcbd80217b8f994ebb571fa03647355bfb5593077c58aedfdf11`
and `a0e1f76ddec6ae901d5e766a942ad0b35299e10f175243fe7abd0fc679040a96`.
`audit_fresh.py` found zero overlap within these streams or against 33,368
distinct earlier public points, including the prior high-K screen and both
current 2026-10-21 cold-base fixtures. Answer sidecars are excluded from timed
inputs. [PROTOCOL.md](PROTOCOL.md) fixes the comparison and promotion gates.

The local release example suite passed 22/22 tests with Rust 1.93.1; its
command and output are retained in `rust-example-tests.log`. The capacity
model, Lean module, `rustfmt --check`, and `actionlint` passed. The branch
workflow repeats the capacity, Lean, and Rust example checks with Rust 1.99.1
on its exact head. A full K4500 or K4549 process has not yet been measured.
The previous RunPod campaign's conservative upper charge was $5.710216
against its $5 ceiling, so this preparation launches no paid instance. A new
bounded allocation needs its own reviewed budget and protocol.
