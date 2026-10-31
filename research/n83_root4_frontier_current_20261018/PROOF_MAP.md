# Root4 state-ID arithmetic proof map

The source's `stateBound` formula is
`83 * (K * (K - 1) / 2) + 42 * K`. The existing
[`RootFour.lean`](../n83_root4_20261014/RootFour.lean) formalization defines
the same expression as `N83RootFour.stateBound`. Its theorem
`k3596_state_id_fits_29_bits` proves
`stateBound 3596 + 1 < 2^29`; theorem
`k3597_state_id_exceeds_29_bits` proves
`2^29 ≤ stateBound 3597 + 1`. These are the exact adjacent-boundary
statements used in [PROTOCOL.md](PROTOCOL.md). They do not assume a curve,
point, factor-base yield, or timing result.

The source bridge is the four-byte slot encoder in
`examples/koblitz_orbit_dlp_fast.rs`, which stores `state + 1` in a
29-bit payload. The protocol's current-source hash pins that implementation.

The pinned toolchain is
[`lean-toolchain`](../n83_root4_20261014/lean-toolchain),
`leanprover/lean4:v4.34.1`. On 2026-10-18, from
`research/n83_root4_20261014`, `lean RootFour.lean` exited 0. Its
`#print axioms` output reports that both boundary theorems depend on no
axioms. The full command, module hash, stdout, stderr, and exit status are
retained in [lean_check.json](lean_check.json). `lean4checker --fresh` was
unavailable on this host. The module
contains no `sorry`, `admit`, custom claim axioms, or native-evaluation
proof bypasses.
