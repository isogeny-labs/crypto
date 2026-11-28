# Analysis addendum (written 2026-11-16, before any scale-study measurement was read)

Status: declared after the protocol froze, before analysis. Only line counts of
the data files had been looked at. Triggered by the validator's finding B13 on
`research/genus4_rho_refs_20261116` (see its ERRATA.md).

The frozen readings (Q1–Q4, Qcal; Q3 primary) are computed exactly as the
protocol says. In addition, each reading is also reported against two rho
walks that are **not run** but follow exactly from the P rows:

- **P0f**: plain walk with the short-cycle window off, canonicalisation floored at one per step.
  Counts are the P row's counts with `compares_uncharged = 0`. The validator showed that
  `cycle_window = 0` leaves the log, group operations and steps unchanged on the plain walk
  (7/7 runs).
- **P0u**: as P0f, with canonicalisation unfloored (0).

The protocol §7 decision ("below rho across the band") is reported against the
frozen cheapest rho, and also against min(R, P, P0f). Only the second counts
as "below the cheapest rho" in RESULTS, because B13 showed that P0f is a cheaper rho the
frozen set left out. P0u is reported as a sensitivity.

Not covered: a negation walk with periodic (rather than per-step) fruitless-cycle
handling (Bernstein–Lange–Schwabe 2011; Bos–Kleinjung–Lenstra 2010) is likely
cheaper still. It is not implemented here, so the rho side stays an upper bound on the
cheapest rho, and an IC "below rho" verdict at w = 1 remains optimistic.
