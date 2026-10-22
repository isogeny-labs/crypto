# rho-neg cards: plan (PROTOCOL-ADDENDUM-rho-neg.md)

Card written 2026-10-22 against HEAD 9b2f5e1b36 (the card carries no commit pin).
Source `research/blindspots_20261018/instruments/blindspots_14_prime_m4_neg.rs`,
sha256 1a7d51c3adcd7108de335cd7ac3b2d83a11f5ef1246b8b959271ced1f5e8710e.

## Cards

One card, `card-n01.yaml` (`CARD-BLINDSPOT-14-RHONEG-01`), so that every command runs from
the one binary the addendum requires. The recorded commands took about 32 s in total
(`raw/receipt-card-01.json`, selftest 21.7 s, fullic-a-b28 1.34 s, every other run command
under 0.15 s), so a split is not needed to stay under the 10-minute foreground limit.
Write scope: `raw/rho-neg/` only. Binary: `raw/rho-neg/bin/b14neg`, removed by the last two
commands. Every output, stdout and stderr path is in `must_not_exist`.

Order: make-dirs, build, selftest (`--deep`), cont-rho-b16/20/24/28,
cont-fullic-a-b16/20/24/28, rho-neg-b16/20/24/28, remove-bin, remove-bin-dir.

## Continuity mapping (rows identical in every field except `ms`)

Same argv as the recorded card-01 commands, only the binary and the `--out` path differ
(`fullic` is run without `--bx`, as recorded, so bx is the per-cell default; the bx=3..7
extras at 16 bits are not rerun).

| new output (raw/rho-neg/) | command | recorded file (raw/) |
|--|--|--|
| cont-rho-b16.jsonl | `rho --bits 16 --runs 16` | rho-b16.jsonl |
| cont-rho-b20.jsonl | `rho --bits 20 --runs 16` | rho-b20.jsonl |
| cont-rho-b24.jsonl | `rho --bits 24 --runs 16` | rho-b24.jsonl |
| cont-rho-b28.jsonl | `rho --bits 28 --runs 16` | rho-b28.jsonl |
| cont-fullic-a-b16.jsonl | `fullic --bits 16 --arm a --runs 16` | fullic-a-b16.jsonl |
| cont-fullic-a-b20.jsonl | `fullic --bits 20 --arm a --runs 16` | fullic-a-b20.jsonl |
| cont-fullic-a-b24.jsonl | `fullic --bits 24 --arm a --runs 16` | fullic-a-b24.jsonl |
| cont-fullic-a-b28.jsonl | `fullic --bits 28 --arm a --runs 16` | fullic-a-b28.jsonl |

Each file has 16 rows. A mismatch in any field other than `ms` is an instrument defect and
the rho-neg rows are not used (addendum, "Continuity controls"). The recorded files are
bound by sha256 in the card, and are never written.

## rho-neg outputs (16 runs per cell, default seed 20261018, lookahead on)

`rho-neg-b16.jsonl`, `rho-neg-b20.jsonl`, `rho-neg-b24.jsonl`, `rho-neg-b28.jsonl`
(`rho-neg --bits B --runs 16 --out ...`), each with a `.stdout.log` and `.stderr.log`.

Other outputs: `build.stdout.log`, `build.stderr.log`, `selftest.stdout.log`,
`selftest.stderr.log` (must end `SELFTEST OK`), every continuity run's `.stdout.log` and
`.stderr.log`, and `receipt-n01.json`.

## Estimated time

About 1 to 2 minutes: build about 6 s, selftest about 25 to 60 s (recorded 21.7 s for the
shorter selftest; the addendum adds checks), all other commands together under 5 s.
Timeouts: build 120 s, selftest 300 s, fullic-b28 60 s, every other run 30 s.
