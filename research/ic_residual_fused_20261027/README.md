> **Record, 2026-11-09.** The collector these sessions measured (`ic_framework/residual_fused.rs`, commit 722dfc3e3e, with the byte-formula fix b4a460705a) was dropped when this branch merged main. It duplicated main's own `targets=residual-fused` (155c62aa1e, [`../ic_residual_fused_20261031`](../ic_residual_fused_20261031/README.md)), which is the one to use. To reproduce these numbers, build `ecbench` at 6d76be583f. The explanation of the extra walk steps is in [`../ic_residual_fused_steps_20261107`](../ic_residual_fused_steps_20261107/README.md). Every claim below is an unverified assertion [pcc:unverified].

# Fused residual collector (`targets=residual-fused`), 2026-10-27

Status: **local L0 sandbox measurement. NOT validated under the validation
gate** (no replay certificate, no validator report, no matched-rho band, no
`VALIDATION.json`). Not an isolab/L2 run; wall times are indicative only. The
leaderboard and `benchmarks/README.md` are untouched.

## What changed

New `Targets::ResidualFused` (`residual-fused`), `src/cryptanalysis/ic_framework/residual_fused.rs`.
The old walk does two additions per step (`point += J_j`, then `point -= P_i`
with `i` from the hash of the post-jump point, which is never looked up). The
fused walk draws `j = mix(key)>>8 mod 16` and `i = mix(key ^ 0xA5A5)>>4 mod |F|`
from the *current* residual and adds the precomputed `C_{j,i} = J_j - P_i`
once. a, b and the row update as before; equal residuals still take equal
steps. The `16*|F|` combined points are computed once and charged to the
relation phase's group-op ledger (setup). Cap: `16*|F| > 2^16` returns an error
(protocols use base sizes 2-64). Also: `FastMap` (MixHasher) table, rows stored
as sorted sparse `(col, coef)` lists in a boxed slice, no per-step dense clone,
dense expansion only on a relation. New counters: `fused_combined_jumps`,
`walk_step_adds`, `walk_steps`, `walk_adds_per_step_milli` (1000 = one add per
step in every run). `residual_table_bytes_uncharged` = bytes actually stored
per entry (key + a + b + slice header + 16 B per nonzero), excluding hash-map
control bytes. `residual` is unchanged apart from `pub(super)` on five helper
fns; `mod.rs` routes the new name; `stages.rs` parses/prints it.

## Old-path byte identity (specs/identity-p16.json: rho + res2, 4 targets, 2 rounds, 16 records)

Baseline binary built from main 0e5689117 vs this branch, `ecbench run --cpus none --allow-busy`.
Records compared after dropping record/session ids, binary hash, allocator, host
isolation telemetry, `time`, and every `*_ns` field: **16 of 16 records identical**
(S, total_gae, counters, work/bitops, recovered log, field ops). Sessions in
`sessions/identity-p16-{old,new}`.

## Counted comparison (local L0; counted units are deterministic)

Base size 2 (|F| = 4 points), `mitm-prime-blocked:m=2`, `incremental-gauss`, 2 rounds
(records are paired by workload; mean over records). Raw: `sessions/cmp-*`,
output of `ecbench table` / `compare`: `summary/cmp-*.txt`.

| curve | targets x rounds | rho S | res2 S | res2-fused S | fused/old S (95% CI) | res2-fused / rho | res2 / rho |
|---|---|---:|---:|---:|---|---:|---:|
| p16 (r 2^15.7) | 16 x 2 | 1.785 | 6.425 | 5.685 | 0.885 [0.819, 0.956] | 3.18 | 3.60 |
| p18 (r 2^17.7) | 16 x 2 | 1.400 | 4.954 | 3.824 | 0.772 [0.712, 0.837] | 2.73 | 3.54 |
| p36 (r 2^36) | 8 x 2 | 1.161 | 2.980 | 1.739 | 0.584 [0.426, 0.800] | 1.50 | 2.57 |

Gap to matched rho remains: fused is 1.5x rho (p36), down from 2.6x. Mean GAE per
solve at p36: rho 304 219, res2 781 291, res2-fused 455 851. (Earlier README of
research/ic_planner_exhaustive_20261026 quotes res2 2.94 at 36 bits; here 2.98 on a
different 8-target subset.)

Caveats, stated plainly:
- The ratio approaches 0.5 only as setup (16|F| additions, restarts) becomes small
  relative to the walk (p36); at small r the fixed costs dominate. The fused walk also
  does somewhat more lookups (p16 27 359 vs 21 352 over 32 runs), i.e. slightly longer
  walks, since the combined jump set is a different (still pseudo-random) walk.
- **§5a band, matched byte formula (re-run 2026-11-07).** The first runs priced the
  fused table with a different byte formula than `residual` (key and slice header
  included), which inflated its unpriced work. Commit b4a460705 makes both arms charge
  `residual_table_bytes_uncharged = entries * (16 + 8*d_col)` and reports the fused
  layout's real bytes as `residual_table_bytes_stored`. Re-run sessions
  `sessions/rerun-{p16,p36}`, `ecbench compare --a res2f --b res2 --weight w`
  (`summary/rerun-*-band.txt`); res2 / res2f, 95% interval:

  | curve | w = 0 | w = 0.1 | w = 1 | band |
  |---|---|---|---|---|
  | p16 | 1.130 [1.046, 1.221] | 0.946 [0.830, 1.078] | 0.809 [0.676, 0.967] | unresolved |
  | p36 | 1.714 [1.250, 2.349] | 1.054 [0.768, 1.446] | 0.883 [0.644, 1.212] | unresolved |

  The fused walk halves the group additions per step but not the native work per step
  (one table probe and one stored entry each). Once that work is priced, the cost per
  relation follows the walk length, and the fused walk does about 16% more steps here.
  So under §5a the two arms are **not ranked**: the gain is a w = 0 (counted GAE)
  result only. The lever for the band is the table itself (distinguished points or
  compact storage), not the addition count.
- Wall (local, L0, shared host, no A/A): end-to-end wall fused/old 1.20 (p16), 1.04
  (p18), 0.91 (p36); not admissible.
- Measurement quality 0/100 (L0, shared 4-core host); no A/A arm; 2 rounds only.

## Reproduce

`ecbench run --spec research/ic_residual_fused_20261027/specs/<name>.json --out <dir> --cpus none --allow-busy`
then `ecbench table --dir <dir>` and `ecbench compare --dir <dir> --a res2 --b res2f`.
Host: local L0 sandbox (Intel Xeon 2.1 GHz, 4 shared cores), not isolab.
Tests: `cargo test --release --lib ic_framework::residual`: 3 passed (old residual test, fused recovery on p12/p14/p16 bases 2-24, oversized-base refusal). Clippy was not run (the box OOM-kills a second full compile); the methods.rs help-string edit was made after the last build.
