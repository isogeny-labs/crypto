# Next candidate: specialize full-cache affine subtraction

Source review during V21 identified avoidable general-coordinate work in the
full-cache query. This is a candidate, not an implemented or measured change.
V21's pinned source and frozen workload remain unchanged.

`device.cuh` converts the cached affine right sum to Jacobian coordinates,
negates it, then calls `Curve::mixed` with the affine target. The cached point
has Z=1 (Montgomery one) when finite. `curve.cuh::mixed` nevertheless computes
Z squared, scales the other input by Z, and computes the general output Z.

The primary Explicit-Formulas Database lists Bernstein–Lange's
`madd-2007-bl` at7M+4S for Z2=1 and `mmadd-2007-bl` at4M+2S for
Z1=Z2=1:
https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html
Read2026-10-30 alongside the actual mixed implementation. These are formula
operation counts, excluding coordinate conversion, normalization, lookup,
memory traffic, exceptional cases and kernel overhead; they are not GPU times.

For P=-R and Q=T in Montgomery coordinates, the specialization uses:
H=Q.x-P.x; HH=H^2; I=4HH; J=HI; r=2(Q.y-P.y); V=P.x I;
X=r^2-J-2V; Y=r(V-X)-2P.y J; Z=2H.
It must separately handle either input at infinity, H=0 with unequal y
(infinity), H=0 with equal y (doubling), and zero-y doubling. Preserve the
existing exact minimum witness, batched normalization and lookup semantics.

A second, independently testable possibility is retaining cache coordinates
in Montgomery representation and converting the target once per query. Today
`jac` converts two cached coordinates and `mixed` converts two target
coordinates per tuple. A representation change needs an explicit type and
round-trip fixtures; `Point` must continue to mean ordinary coordinates.
Charge cache construction and keep the16-byte capacity accounting exact.

Before any deployment: add host/device differentials including infinity,
inverses, doubling and63-bit arithmetic; run all sanitizer paths; freeze
matched52-bit controls with identical bases/targets and separate formula and
representation factors. Only measured complete recoveries can establish a
speed improvement. Do not alter the active V21 pod or compare clocks across
its source boundary as a causal result.
