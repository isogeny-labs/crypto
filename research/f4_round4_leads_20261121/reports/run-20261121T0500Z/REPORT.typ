#set page(paper: "a4", margin: (x: 1.6cm, y: 1.6cm))
#set text(size: 9.5pt)
#show heading.where(level: 1): set text(size: 14pt)
#show heading.where(level: 2): set text(size: 11pt)

= F4 round 4: six leads measured, four implemented (run-20261121T0500Z)

*Standing:* `[pcc:unverified]`. One shared 4-vCPU VM, isolation L0, no PMU.

*Scope:* `jv_cover --exp dlp` (Joux–Vitse cover index calculus, $E(FF_(p^6))$, p = 53, 19,020 Nagao F4 systems).

*Protocol:* `CONFIRM_PROTOCOL.md`, frozen and pushed (`b5795d4061`) before data. *PR:* \#4859.

== Result

#table(
  columns: (auto, auto, auto, auto),
  align: (left, left, left, left),
  table.header([*comparison (10 rounds)*], [*predicted*], [*measured*], [*outcome*]),
  [H0/H1 wall: Hilbert before round 4 → Hilbert + trace, round 4], [[1.15, 1.50], ≥ 9/10], [1.231 (10/10); 9.97 → 8.14 s; CPU 1.235], [met],
  [F0/F1 wall: full F4, old → new binary], [[1.03, 1.15], ≥ 9/10], [1.060 median, 8/10], [not significant (sign rule)],
  [A/A H1/H1a], [within ±3 %], [0.988; median deviation 2.2 %], [met],
  [Counted S, H0 → H1], [1.316, deterministic], [oracle mults 9.624 G → 7.24–7.30 G], [about 1.32×; *not deterministic*],
)

All 50 runs found 31 relations from 19,020 residuals and a correct logarithm. H0, F0 and F1 reproduced their counted operations exactly.

*H1 did not.* Its counted oracle multiplications ranged over 7.240–7.303 G (0.9 %). With 4 threads, which system records the trace first depends on scheduling, and that changes the replays. The protocol had predicted a deterministic count from a 2-thread run; that prediction failed. The relations and logarithm are unaffected.

#figure(image("confirm.svg", width: 100%), caption: [Per-round wall times and paired ratios; `data/confirm/runs.jsonl`.])

== What changed

- *Trace through the Hilbert guard.*
  - `f4_record`/`f4_replay` now go through `f4_hilbert_guarded`. Before, `--trace --stop hilbert` silently ran full F4.
  - `jv_cover` refuses `--trace` without a stop rule. A full-mode replay is a full run plus a prefix: +25 % counted operations, slower.
  - With Hilbert, the trace replays only the recorded useful rows: counted S 392.3 → 298.1 (1.32×) with identical relations.
- *Recording runs charged.* The tracked elimination a recording run adds was uncounted. It now charges `count_ops` (+0.4 % counted F4 on the 104-test workload).
- *Small kernels*, output and counted operations byte-identical:
  - u32 accumulators in the deferred elimination for p ≤ 4097;
  - byte-wise pack/unpack;
  - a contiguous u64 divisor scan.
  - Together −12 % F4 instructions. Pre-sizing emitted polynomials measured about 0.
- *Frozen replay steps.* A replayed step rebuilds its matrix from a frozen superset of its symbolic preprocessing, then walks the reachable columns with integer indices.
  - It uses no hashing and builds exactly the matrix preprocessing would.
  - The superset widens by support union.
  - The first version, exact supports only, saved 0.8 %, because zero coefficients at p = 53 rarely let supports match. The union version saves 9.8 % of F4 instructions, with identical output and counted operations.

#figure(image("instructions.svg", width: 90%), caption: [Callgrind F4 instructions per build and mode; `data/instructions.json`.])

== Not done, and why

- *Lead 4, large-system allocation:* no code change. `jv_cover` has about 830 page faults per run.
- *Lead 6, signature criterion:* deferred. Under Hilbert + trace, the diverged replays (21 %) carry an estimated 47 % of counted F4. A signature criterion only trims zero reductions inside them; its upper bound is about 13 % of the total. Branch traces, which record the continuation at a divergence, target the same 47 % more directly (estimated about 30 %).

== Limits

One host; p = 53 only. The F0/F1 gain failed the pre-registered sign rule. H1's counted cost depends on thread scheduling. The Hilbert rule stays opt-in.
