#set page(paper: "a4", margin: (x: 1.6cm, y: 1.6cm))
#set text(size: 9.5pt)
#show heading.where(level: 1): set text(size: 14pt)
#show heading.where(level: 2): set text(size: 11pt)

= F4 speed-up round 3: allocator per consumer, fewer per-term allocations (run-20261121T0100Z)

*Standing:* `[pcc:unverified]`. One shared 4-vCPU cloud VM, isolation L0, no PMU.

*Scope:* wall-clock, CPU time and callgrind instruction counts of the F_p F4 engine's index-calculus consumers. Counted operations are unchanged throughout (the oracle operations, F4 multiplications and native operations in each report), so no IC row or gap to rho moves.

*Follow-up to:* \#4851. *PR:* \#4859. *Protocol:* `CONFIRM_PROTOCOL.md`, frozen and pushed (`5a42abdbd2`) before the confirmation data.

== Result

#table(
  columns: (auto, auto, auto, auto),
  align: (left, left, left, left),
  table.header([*measurement*], [*arms*], [*result*], [*rounds*]),
  [jv_cover `--exp dlp`, pre-registered], [main + jemalloc vs f4_fp changes], [wall 1.090 (13.93 → 12.86 s), CPU 1.091], [11/12; A/A 1.0045],
  [jv_quartic `--exp cprime`, shipped], [main (glibc) vs jemalloc + f4_fp changes], [wall ≈ 1.45 (9.2 → 6.3 s)], [4/4],
  [allocator screen, jv_quartic], [glibc vs jemalloc preloaded], [wall ≈ 1.40 (9.1 → 6.5 s)], [4/4],
  [allocator screen, jv_quintic], [glibc vs jemalloc preloaded], [wall ≈ 0.94 (11.9 → 12.8 s): *slower*], [0/4],
)

Every run in every arm did identical work.
- jv_cover: 19,020 residuals, 31 relations, 17,873,891,895 counted oracle operations and a correct logarithm.
- jv_quartic: 126,594.6 mean F4 multiplications, 100/100 planted relations found, 7 decomposable residuals.
- jv_quintic: 1.1045e10 mean F4 multiplications.

The pre-registered predictions were met. Wall in [1.02, 1.10] with at least 10 of 12 rounds: 1.090 with 11. CPU in the same band: 1.091. A/A within ±2 %: 1.0045, with a median absolute deviation of 0.8 %.

#figure(image("confirm.svg", width: 100%), caption: [jv_cover per-round wall times and paired ratios; `data/confirm/runs.jsonl`.])

== jemalloc is a small-system win, not a general one

On jv_quartic's 65-row systems, jemalloc preloaded cut wall time by 1.40×.

On jv_quintic's large systems it was slower in all 4 rounds. Its system time was 3.1 s against glibc's 0.85 s. Neither of the following removed the regression:
- `MALLOC_CONF=dirty_decay_ms:-1,muzzy_decay_ms:-1` (system time 2.8 s);
- `oversize_threshold:0`, alone or combined with the decay settings.

The decision rule was written into the screen script before the data: a consumer gets jemalloc only when every round favours it and the median ratio is at least 1.05. So jemalloc is built into jv_cover and jv_quartic (default feature `jemalloc`) and not into jv_quintic or any other binary.

#figure(image("screen.svg", width: 100%), caption: [Allocator screen; `data/screen/runs.jsonl`, `data/quintic_conf/runs.jsonl`.])

== Fewer per-term allocations in f4_fp

The changes:
- The final linear interreduction uses packed `u128` monomials when the ordering is grevlex, n ≤ 15 and every degree is below 128. It builds the same rows, columns and column order as the exponent-vector path, which `F4_FP_PACKED=0` keeps.
- Matrix columns stay packed and are unpacked only for the terms of the new polynomials a step emits.
- The reducer rows' multipliers are unpacked only for a recording (trace) run.
- `Packed::symbolic` interns monomials in a small open-addressing table with one multiply-shift hash.

The table's first version was sized from the seed rows' total terms. That cost more in table clearing than it saved (1.245 G F4 instructions, rejected). Starting at 256 slots and doubling fixed it.

The interreduction does not charge the native-operation counter, as before.

#figure(image("instructions.svg", width: 85%), caption: [Callgrind F4 instructions over 104 Nagao tests (`jv_cover --exp ccov`, p = 53, 1 thread); `data/callgrind/instructions.json`. Whole program: 3.506 G → 3.331 G (−5.0 %). The output JSON is identical apart from timing fields.])

== Limits

- One host and one curve size per consumer.
- The wall effect (9 %) exceeds the whole-program instruction cut (5 %). Fewer allocations plausibly save cache misses that callgrind does not count, but without a PMU this is not shown.
- `f4_fp::Poly` still stores one heap exponent vector per term. The remaining conversions (the emitted terms, the basis) are bounded by the output size.
- Monomial interning and the reducer search are still about 28 % of F4's instructions.
