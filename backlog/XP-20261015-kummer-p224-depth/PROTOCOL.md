# XP-20261015-kummer-p224-depth: protocol

**Status: frozen** by the commit that adds this file, before any run at the
card's cells on the measuring host. Card: [`card.json`](card.json); narrative
[`README.md`](README.md). Engine and prior measurements:
[`research/f4_generic_field_20261015/`](../../research/f4_generic_field_20261015/README.md)
(merged in PR #1942, merge commit `3a4a5cec8`). Every statement is *recorded*
(a committed row), *derived* (a calculation shown here) or *predicted* (stated
before the runs).

## 1. Question

Is the Kummer tower's F4 solving degree `D` at fixed `(m, N)` the same at the
45-bit prime `p_40 = 27·2^40 + 1 = 29686813949953` (`v₂(p − 1) = 40`) and at
the NIST P-224 prime `2^224 − 2^96 + 1` (`v₂(p − 1) = 96`) as at the 31-bit
`p₁ = 15·2^27 + 1`, where every recorded value was measured?

H_same (the study's hypothesis, carried over): `D`, width (`max_cols`) and
step count equal the `p₁` reference in every cell.

## 2. Instrument, fixed

| | value |
|:--|:--|
| binary | `examples/pkm_tower_pilot` (`--engine generic`), release build at `b7cafe04485eb97d873dffef698e7337cfb38cfe` (main, contains `3a4a5cec8`), the same binary `research/pkm_lead1_m3n21_20261016` used (sha256 `d25eadb1…c265d`); `pkm_tower_check` from the same build |
| backends | `auto`: `Fp52` (AVX-512 IFMA) at `p_40`, `FpWide<4+spare>` at P-224, as the study recorded |
| fixed flags | `--engine generic --kinds kummer --controls tower --ladder-t none --planted 0 --random 2 --trace` (seed: the pilot's default, `1347112276`, the study's) |
| targets | each `(prime, m, N, target)` is its own process, `--targets T`, `T ∈ {0, 1}`. Target 0 is the study's target 0 (same seed and draw), so it repeats a recorded row where one exists; target 1 is new |
| caps | per process: `prlimit --as` (stage-specific, §3) and `--budget` seconds; no `--max-dense` |
| check | `p_40` rows: `pkm_tower_check verify` (independent exhaustive count over `V^m`). P-224 rows: `--verify` in-row (`BigUint` re-check, the study's G4), because the checker reads `u64` primes only |
| commands | [`research/pkm_kummer_wide_field_depth_20261018/run.sh`](../../research/pkm_kummer_wide_field_depth_20261018/run.sh); `progress.txt` there logs every start and end |

## 3. Stages and cells

`(m, N)` ↔ tower `t`: `N = m·t`. References are *recorded* `p₁` (or `p₂`) rows.

**Stage 0: identity and memory sizing** (2 threads, `--as` 32 GB, budget 1 h).

| cell | prime | must equal (recorded) |
|:--|:--|:--|
| `m = 4, N = 12`, target 0 | `p_40` | `research/f4_generic_field_20261015/runs/ext/m4N12-v40.jsonl`: `D` 7, `max_cols` 9452, `steps` 29, `muladds` 51,598,623,198, `inconsistent` true, same `x_r` |
| `m = 4, N = 12`, target 0 | P-224 | `runs/ext/m4N12-P-224.jsonl`: the same fields, same `x_r` |

A difference in any listed field stops the round until explained. Wall time
and new fields are exempt. The traces' peak memory gives the measured memory
multiple `R_p = peak(prime) / peak(p₁)` for this cell (`p₁` reference: 171 MB,
PKM note, recorded); stage 2 is sized with it.

**Stage 1: the card's cells** (2 threads each, the study's thread count, so
`muladds` compare exactly; `--as` 32 GB; budget 48 h).

| cell | `p₁` reference `D / width / steps` | at `p_40` | at P-224 |
|:--|:--|:--|:--|
| `m = 2, N = 20` (`t = 10`) | 6 / 33446 / 24 (K1) | targets 0, 1 | targets 0, 1 |
| `m = 3, N = 15` (`t = 5`) | 7 / 31397 / 24 (M3) | targets 0, 1 | targets 0, 1 (**new** at P-224) |
| `m = 4, N = 12` (`t = 3`) | 7 / 9452 / 29 (M4) | target 1 (target 0 is stage 0) | target 1 (target 0 is stage 0) |

Target-0 rows that the study recorded at the same prime must also equal it on
`muladds` and `x_r` (a replication check, as stage 0).

**Stage 2: one level deeper** (beyond the card; pre-registered here; never
measured at either prime).

| cell | `p₁` reference (recorded) | primes | threads |
|:--|:--|:--|--:|
| `m = 4, N = 16` (`t = 4`) | 7 / 76837 / 53, refutes; 8.1 GB peak, 2.70·10¹³ muladds (round 3, `M4b`) | `p_40`, P-224 | 8 at `p_40`, 16 at P-224 |
| `m = 3, N = 18` (`t = 6`) | 7 / 106288 / 28, refutes; 12.7 GB peak, 8.99·10¹³ muladds (round 7, `M3b`) | `p_40`, P-224 | 8 at `p_40`, 16 at P-224 |

Targets 0 and 1 each. **Admission rule, fixed now:** a stage-2 process starts
only if its derived peak `peak(p₁) × R_p` (`R_p` from stage 0) is at most
0.8 × the host's `MemAvailable` at launch, after subtracting the reservations
of processes already running. Its `--as` cap is `1.25 × peak(p₁) × R_p`,
rounded up to a GB. Its budget is 604,800 s (the card's timeout). A cell that
the rule does not admit within 7 days of stage 1's end is recorded as *not run
(resources)*. *Derived* single-core times, from the study's measured cost per
multiply-add (0.720 ns at `p_40`, 14.47 ns at P-224): `m = 4, N = 16` 5.4 h at
`p_40`, 109 h at P-224; `m = 3, N = 18` 18 h at `p_40`, 361 h at P-224 — before
threading, whose speed-up is not measured.

## 4. Decision rule, fixed (the study's §6 applied per cell)

- **H_same supported in a cell**: every finished target at that prime has `D`,
  `max_cols` and `steps` equal to the `p₁` reference.
- **H_same rejected in a cell**: a finished target differs in `D`. A
  difference in width or steps with equal `D` is reported as such and is not a
  rejection of the card's question (which is `D`).
- **Censored**: a process stopped by its budget, its `--as` cap or a host loss
  carries no reading; it is recorded with its trace's last step. A host loss
  (spot interruption, reboot) repeats the process whole with the same flags
  and binary, keeping the lost attempt's files (as the PKM rounds did); a cap
  or budget stop is not re-run.
- **Instrument first**: a failed stage-0 identity, a checker disagreement, or
  `planted_ok = false` voids the readings at that prime until explained.
- **The card's answer** (decision value): H_same supported in every stage-1
  cell at both primes ⇒ the `p₁` ladder speaks for P-224 at these sizes.
  Rejected anywhere ⇒ every `p₁` extrapolation is suspect. Stage 2 extends the
  range of that answer; it does not change stage 1's reading.

**Inadmissible**: changing the binary, seed, flags, thread counts or caps of a
stage after it starts; dropping a target; re-running a finished process.

## 5. Predictions (before the runs)

1. Stage 0 identity holds at both primes, field for field.
2. H_same holds in every stage-1 cell at both primes, including `m = 3,
   N = 15` at P-224, and every target-0 row repeats the study's row exactly.
3. Stage 2: H_same holds in every cell that runs; `muladds` equal between
   `p_40` and P-224 at the same thread count. The P-224 `m = 3, N = 18` cell
   is predicted not admitted (derived peak about 12.7 GB × `R_p` with `R_p`
   near 8 is about 100 GB, more than 0.8 × what is free while other work
   runs).

## 6. Before this protocol (disclosure)

Read, not run: the study's rows and README, the round-3 and round-7 `p₁` rows.
Nothing of this card has run on the measuring host (32 vCPU Xeon 6975P with
AVX-512 IFMA, 124 GB, EC2 spot; shared with the shape-search card's stage A,
five single-thread processes, and later its stage B). Not isolated: wall times
are descriptive only and enter no reading.
