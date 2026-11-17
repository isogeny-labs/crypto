# Decision rule (written 2026-10-22, before any Arm64 data)

Hosts: GitHub `ubuntu-24.04-arm` (Linux, Arm Neoverse) and `macos-latest`
(Apple silicon). One binary per host and tool; arms differ only by
environment; arms rotate inside every round.

**Exactness gate (all three questions).** Every arm must give the same
per-target decisions (`hit`, `indices`, `exhausted`, `reductions`, `splits`,
`propagations`, `infeasible`, …) in every round, and the same basis
fingerprints / row-space fingerprints and word counts for the GF(2) benches.
A failed gate stops the question; no default changes on it.

Ratios below are medians over rounds of recorded CPU time (inherited F4:
`cpu_ms` of the probe's total line; benches: `wall_ms`), arm ÷ default.

1. **Eager rewrite through shift runs (`on` vs `noeager`).** The default on
   Arm64 since #2043. It stays if `noeager / on ≥ 1.00` on at least 6 of the
   9 cells for each engine (`f4`, `f6p`) on each host. If a host fails that,
   the follow-up is a proposal to restore the lazy rewrite on that
   architecture, not an edit in this PR.
2. **Gate-first F6-IC order (`on` vs `nogate`).** The default since #2043,
   measured before only on aarch64 with the lazy rewrite. It stays if
   `nogate / on ≥ 1.00` on at least 7 of 9 cells for `f6p` on each host.
   `f4` has no node oracle and is reported only as an A/A check
   (expected 0.95–1.05).
3. **GF(2) Gray-code tables (`KIC_GF2_TABLES`, default 4) and table reuse
   (`KIC_GF2_REUSE_TABLE`, default off).** An Arm64-specific default is
   proposed only if one arm is at least 5 % faster than `t4` in the
   geometric mean over cases, at 1 thread and at all cores, on both hosts,
   with no case slower than 0.97×. Otherwise the x86-tuned defaults stand.

Anything else reported is description, not a decision.
