# Protocol: jumps=16 ablation around sib8-p256 (L4-JUMPS), 2026-11-19

Frozen before any measured row; a 2-target smoke run checked only that the arms run. Every statement is `[pcc:unverified]`. No code change; `target/release/ecbench` (sha256 prefix b4fcf0ecdb3d0a0f), unchanged. L0 on a shared 4-core container (`--cpus none --allow-busy`): counted GAE and bitop/v1 are exact, wall times are contaminated and not interpreted.

## Why
The round-3 ablation (`research/ic_tableb_sib_20261119/ablation`) left one factor unisolated: dropping `jumps=16`. This run crosses jumps (16 or default), stride-generator targets (sg) and siblings (sib=8,pool=256) around the incumbent.

## Arms (specs/P20,P22,P24.json; 48 public targets, 2 rounds, 1 warm-up, alternate, seed 20261119412; factor base prime-projected-abscissa:size=2,scan_cap=32)
| arm | jumps | targets | siblings |
|---|---|---|---|
| rho-neg | reference | | |
| sib8-p256 (incumbent) | default | stride-generator | sib=8,pool=256 |
| tw2l-nj | default | default | no |
| tw2l-sg-nj | default | stride-generator | no |
| sib8-p256-j16 | 16 | stride-generator | sib=8,pool=256 |
| tw2l-sg-sib | 16 | stride-generator | sib=8,pool=256 |

Note: `sib8-p256-j16` and `tw2l-sg-sib` are by construction the same oracle string content (jumps=16, sg, sib8, p256), differing only in parameter order. They are the same configuration given by two routes (incumbent plus jumps=16; tw2l plus sg plus siblings), so they act as an A/A control and should match exactly in counted cost.

## Predictions (registered; ratios are arm / sib8-p256, counted GAE, w = 0)
- P-a: every run verifies; `sib8-p256-j16 / tw2l-sg-sib` is exactly 1.
- P-b: `sib8-p256-j16 / sib8-p256` in [0.95, 1.10] at every size (jumps=16 is about neutral with siblings).
- P-c: `tw2l-sg-nj / sib8-p256` in [1.2, 1.7] at w = 0 (siblings alone were 0.62-0.80 of tw2l), and in [0.95, 1.10] at w = 1.
- P-d: `tw2l-nj / sib8-p256` is at least 1.4 at w = 0 at all sizes.
- P-e: `sib8-p256` / `rho-neg` replicates 1.0 +- 0.15 at w = 0 (about 1.075, 0.972, 0.891 at P20/22/24 on earlier seeds).

## Decision
An arm replaces the incumbent only if `ecbench band` orders it below `sib8-p256` at w = 0, 0.1 and 1 at all three sizes. Expected outcome: none replaces. Compare pairs: each candidate against sib8-p256 and rho-neg, in GAE and bitop/v1. Distance to rho-neg is reported as the distance remaining. No leaderboard row (no validation gate).

## Smoke-run disclosure
The 2-target, 1-round P20 smoke run (not kept as data) showed S = 1.440 / 1.426 for sib8-p256, sib8-p256-j16 and tw2l-sg-sib alike, so the three may be bit-identical (jumps=16 inert once siblings are on, or the default). The registered P-b range includes 1.0 and stays as written; I add P-f: `sib8-p256-j16 / sib8-p256` is exactly 1.000 in counted cost at all sizes. The smoke also showed tw2l-nj and tw2l-sg-nj above sib8-p256 (about 2.1 to 3.4 vs 1.4); P-c and P-d were written before reading those numbers in detail but after they were visible, so treat them as weakly preregistered.
