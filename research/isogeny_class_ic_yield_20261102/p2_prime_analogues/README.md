# P2: scaled analogues of ECCp-131's isogeny class (builder output, not yet run)

Protocol (frozen, predicted null): [`PROTOCOL.md`](PROTOCOL.md). Parent design:
[`../DESIGN.md`](../DESIGN.md). Nothing here is a result: `smoke/` and
`calibration/` exercise the harness and size the run.

| path | what |
|:--|:--|
| `p2tool/` | Rust (no Python). `p2tool build` makes `classes.json` (needs libpari by `dlopen`, run once on a machine that has it); `p2tool spec` writes an `ecbench.spec/v1`; `p2tool analyze` is the variance partition |
| `classes.json` | the 10 classes (5 sizes x classes A, B): every curve, j, verified `#E`, isogeny edges, iso models |
| `run_class.sh` | one (class, m) cell: spec, `ecbench run`, `ecbench verify --replay`, analysis |
| `smoke/` | 24-bit class A, 12 curves + 4 iso pairs, m=2 and m=3, sessions, audits, analyses |
| `calibration/` | one-curve timing/yield runs at all five sizes, used for the cost table below |

## Classes (all verified: `#E = n` prime, h Hilbert roots, class connected, no off-class rational neighbour)

| id | D | h | p | log2 p | t | n = p+1-t |
|:--|--:|--:|--:|--:|--:|--:|
| c24A | -84171 | 128 | 13640833 | 23.70 | 7381 | 13633453 |
| c24B | -414651 | 300 | 13834393 | 23.72 | 7411 | 13826983 |
| c28A | -84171 | 128 | 166443943 | 27.31 | 25801 | 166418143 |
| c28B | -414651 | 300 | 243479263 | 27.86 | 31201 | 243448063 |
| c32A | -84171 | 128 | 4127505313 | 31.94 | 128491 | 4127376823 |
| c32B | -414651 | 300 | 4029877543 | 31.91 | 126961 | 4029750583 |
| c36A | -84171 | 128 | 65800222783 | 35.94 | 513031 | 65799709753 |
| c36B | -414651 | 300 | 68427077473 | 35.99 | 523171 | 68426554303 |
| c40A | -84171 | 128 | 1082308376983 | 39.98 | 2080681 | 1082306296303 |
| c40B | -414651 | 300 | 1095111075763 | 39.99 | 2092951 | 1095108982813 |

`classes.json` is committed output; rebuilding it needs PARI 2.17
(`P2_LIBPARI=<libpari.so> p2tool build --out classes.json`, 45 s). The run
does not need PARI.

## Build and exact commands

```bash
# once per machine (repo root); p2tool is a standalone crate (its own [workspace])
cargo build --release --bin ecbench
cargo build --release --manifest-path research/isogeny_class_ic_yield_20261102/p2_prime_analogues/p2tool/Cargo.toml
# the second build lands in p2tool/target/release/p2tool (P2TOOL overrides)

D=research/isogeny_class_ic_yield_20261102/p2_prime_analogues
O=$D/runs                     # output root; one directory per cell
# sizes 24..36, both classes, m=2 (T=4 targets, 4 rounds, R=100 relations/run) and m=3
for c in c24A c24B c28A c28B c32A c32B c36A c36B; do
  $D/run_class.sh $c 2 $O/${c}_m2 --targets 4 --rounds 4 --aa $([ ${c:1:2} -le 28 ] && echo 1 || echo 0)
  $D/run_class.sh $c 3 $O/${c}_m3 --targets $([ ${c:1:2} = 24 ] && echo 8 || echo 4) --rounds 4 --aa $([ ${c:1:2} -le 28 ] && echo 1 || echo 0)
done
# 40 bits is the expensive row: either the full spec ...
$D/run_class.sh c40A 2 $O/c40A_m2 --targets 4 --rounds 4 --aa 0
# ... or the reduced one (R=30 relations per run, analysis then reports its own power)
$D/run_class.sh c40A 2 $O/c40A_m2 --targets 4 --rounds 4 --aa 0 --relations 30
```

Spec flags: `--first K` (first K curves, default all), `--iso K` (default all 8
iso models, their base curves are added), `--targets`, `--rounds` (even),
`--relations R`, `--y`, `--table-cap` (default 2^22 pair-table entries),
`--aa 0|1`, `--kind yield|solve`, `--isolation L0|L1|L2`. `run_class.sh`
honours `ECBENCH`, `P2TOOL`, `ISOLATION` (default L2: `ecbench` refuses a busy
host), `ALLOW_BUSY=1` (smoke only), `REPLAYS`. The cells are independent and
may be sharded over pods; every cell writes its own `analysis.json`.
The director freezes the route (isolab-eks default, AGENTS.md section 9a).
`ecbench run` is one session at a time per host (benchmark lock).

Analysis of a cell alone: `p2tool analyze --classes classes.json --class c24A
--m 2 --session <session dir> [--session ...] --out analysis.json`
(`--boots`, `--perms`, `--plants`, `--seed` default 2000, 2000, 200, 20261102).

## Cost (measured per run on this 4-core host under load average 5 to 7, then summed)

Per-run process wall, seconds (`calibration/calibration.md`): m=2 0.04, 0.5, 0.7,
1.9, 22.5 and m=3 0.03, 0.15, 0.5, 2.2, 21 at 24, 28, 32, 36, 40 bits (R = 100
relations per run, m=3 attempts refitted after calibration), plus about 0.17 s
of harness overhead per run. Runs per cell = h x targets x (rounds + 1 warm-up)
x arms. CPU-hours (class A / class B):

| bits | m=2 (T=4) | m=3 (T=4; 24 bits T=8) |
|--:|--:|--:|
| 24 | 0.45 / 1.05 | 0.85 / 2.0 |
| 28 | 1.4 / 3.4 | 0.7 / 1.6 |
| 32 | 1.2 / 2.9 | 0.95 / 2.2 |
| 36 | 2.9 / 6.9 | 3.4 / 7.9 |
| 40, full R=100 | 32 / 76 | 30 / 71 |
| 40, R=30, T=4 | 10.5 / 25 | 9.9 / 23 |
| 40, R=30, T=2 | 5.3 / 12 | 4.9 / 12 |

Sizes 24 to 36, both classes, both m: about 40 CPU-hours. 40 bits adds 68
(R=30, T=4) up to 248 (full). The 40-bit m=2 row is dominated by 2.6e7
decomposition attempts per run (predicted yield 3.9e-6 at a pair table capped at
2^22 entries; measured 4.3e-6). Fewer relations per run lowers power: the
analysis prints the measured power of the planted 20% effect per cell, and the
protocol marks a cell with power below 0.9 "not informative at 20%".

## What was not done (see the report that came with this directory)

F4 solving degree / Dreg (no F4 stage exists in the prime pipeline; PROTOCOL
section 3), full-DLP solve time beyond the `--kind solve` option (not run), a
rho reference (not needed: no cross-method claim), `ecbench` runs at L2 (the
builder host is L1, shared), the full measurement itself.

## Smoke results (24-bit class A, 12 curves + 4 iso pairs; harness check only, isolation L1)

| | m=2 (|F|=370, 9959 attempts/run, 16 cells/curve) | m=3 (|F|=70, 1253 attempts/run, 32 cells/curve) |
|:--|--:|--:|
| rows / audit | 1200, `ecbench verify` OK, 6 replays reproduced | 2400, OK, 6 replays reproduced |
| in-session A/A counts identical | 320/320 | 640/640 |
| yield per attempt (main) | 0.0200 (predicted 0.0201) | 0.033 (predicted 0.0335) |
| yield ICC raw / bootstrap upper 95 / perm p | -0.022 / 0.10 / 0.83 | -0.003 / 0.061 / 0.54 |
| max/min curve ratio (null q95) | 1.05 (1.09) | 1.08 (1.14) |
| A/A round-halves q95 abs log ratio | 0.054 | 0.081 |
| iso-model rms log ratio | 0.046 | 0.060 |
| planted +20% detected (ICC, max z, max/min) | 1.0, 1.0, 1.0 (median planted 1.19) | 1.0, 1.0, 1.0 (median planted 1.17) |

Not data of the protocol (a different spec, subset of curves, a busy host).
A first m=3 smoke with the yield law before refit (4N^3/n, 3x too many
expected relations, so 3x too few attempts) gave planted-effect power
0.73 to 0.85, below the 0.9 the protocol requires: the refit and R = min(100, 0.6|F|)
are the fix, and the run plan keeps 8 targets per curve at 24 bits for m=3.
`smoke/run_class_test/` is the end-to-end test of `run_class.sh`.
