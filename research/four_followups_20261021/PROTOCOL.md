# Four ECDLP follow-ups: frozen protocol

Frozen before candidate execution on 2026-10-21 UTC. The comparison cohort is
the 21 exact-order Scholten models and 60 public known-answer targets per model
in `research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst`.
Each target is a separate cold solve. The matched source negation-rho and
projective collector controls are the frozen records in that archive; the
wrapped external-law collector is the independently replayed record set in
`research/genus2_external_gate_20261021/results/`.

## Requested leads and completion gates

| Lead | Required execution and verification |
| --- | --- |
| Small factor base / large-prime graph | Run the frozen parameter grid on all 1,260 targets; source-check each scalar, retain failures, compare complete charged cost with the wrapped external collector and rho on the identical inputs. |
| Complete inversion-free genus-2 law | Implement the Ozbay Gürler–Hisil complete law, independently cross-check exceptional and ordinary additions on all 21 models, run the same 1,260 target cohort, and price full cold cost against rho. An affine law or a partial formula does not fulfill this lead. |
| Point-driven pencil collector | Use the checked transfer on all 21 source models, run 1,260 one-target solves with full setup, sieve, verification and matrix work, independently replay and price against two-point and rho controls. The existing 20-target `F_739` gate is only a prerequisite. |
| Kummer walk arithmetic | Establish a compatible Kummer model and a sign/lift-safe relation-label rule, test differential additions against Jacobian arithmetic including exceptions, then run the same 1,260-target full collector and compare with rho. A Kummer scalar-multiplication benchmark alone does not fulfill this lead. |

No stage result is called an end-to-end result. Every arm records counts for
construction, transfer, initialization, relation work, elimination, and source
verification. Price the full 12 arithmetic by 3 logical-operation grid used by
the comparator, including field additions, hashes, graph and scalar work.
The representative price is `M + 0.5S + 10I` and 0.1 source-addition
equivalents per declared native logical operation. Retain per-target records,
seeds, workload IDs, failures, and all 36 price cells. Wall time is diagnostic
until matched isolation and external-tool checks are complete.

## Factor-base / graph parameter grid

This first new implementation uses the wrapped external law and the same
two-candidate signed graph. The full-base, 16-jump arm reproduces the incumbent
algorithm. A small base is the target-independent set of x coordinates with
`mix(x xor p) mod d = 0`, where `d` is 2 or 4. A split relation is admitted
when at least one root x is in that set; the other root may be a large-prime
node. A split relation with both roots outside the set is rejected but its
candidate addition and split test are still charged. This explicitly tests a
one-large-prime policy against the incumbent's two-large-prime graph; it does
not presume a speed gain. The frozen primary grid is:

| Arm | Base denominator d | Jump table size |
| --- | ---: | ---: |
| Existing-policy replay | 1 | 16 |
| Half-base | 2 | 16 |
| Quarter-base | 4 | 16 |
| Eight jumps | 1 | 8 |
| Thirty-two jumps | 1 | 32 |

The source workload, algorithm seed, cap, transfer, signed elimination and
verification are held fixed. Each variant consumes its deterministic stream
of random draws; jump-size changes intentionally alter that stream. All five
arms must run to completion or retain an explicit exhaustion/failure record.
The run files use one JSON object per target and an arm-specific immutable
name. A single-target test is only a correctness gate, never the full result.

## Publication format

The standardized `results/manifest.json` identifies the revision, source
archive digest, arm protocol, commands, output digests, counts, failures and
references. The `results/targets.jsonl` table has one row per arm and target;
`results/price-grid.csv` carries the 36-cell prices and ratios; `REPORT.md`,
SVG/PNG graph and PDF give the same values without significance judgments.
All missing results remain explicit nulls with a requirement status.
