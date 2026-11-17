# F2 pilot: gate reachability before the freeze

Run by the F2 coordinator on 2026-11-17, before PROTOCOL.md and SPEC.md were frozen.
Standing: `[pcc:unverified]`. All pilot curves use the pilot tag
`0x4C36_0000_0000_0000`. They are not decision instances: the decision census uses
`0x4C26...` and its G1 draws use `0x4C46...`, and G10 in the decision run checks that no
decision seed or curve equals a pilot one. The binary is `l06r` built from the sources
whose hashes are in `SOURCES.sha256` (quoted in SPEC.md 1). The Rust sources were not
edited after the pilot build. `analyze.py` was edited after the pilot runs only to
(a) add the pilot ranges to G10, (b) set G1 `auto` to 200 to match the pilot,
(c) add the `--aux-prefix` and `--g1-counts` pilot options. `pilot/pilot_gates.json`
was then regenerated with the final `analyze.py` (that last invocation is not in
`pilot/pilot_commands.log`; its command line is identical to the logged one).

## 1. What was run (verbatim, `pilot/pilot_commands.log`; seconds are wall time, for budgeting only)

| command (binary `l06r/target/release/l06r`) | exit | s |
|---|---|---|
| `cargo test --release --offline` (output `pilot/test.log`) | 0 | 24 (tests) |
| `gate-enum --tag pilot --p 257 --count 20 --force C` | 0 | 104 |
| `gate-enum --tag pilot --p 257 --count 200 --force auto` | 0 | 676 |
| `gate-enum --tag pilot --p 257 --count 40 --force B` | 0 | 128 |
| small census, `--prime-target 20/10`, u = 3000/1000/500/300/300 (moved to `pilot/small/`) | 0 | 9-25 each |
| `census --tag pilot --p 257 --u-count 3000 --prime-target 120 --draw-cap 40000` | 0 | 21 |
| `census --tag pilot --p 521 --u-count 3000 --prime-target 120 --draw-cap 40000` | 0 | 40 |
| `census --tag pilot --p 1031 --u-count 3000 --prime-target 120 --draw-cap 40000` | 0 | 75 |
| `census --tag pilot --p 2053 --u-count 1000 --prime-target 120 --draw-cap 40000` | 0 | 82 |
| `census --tag pilot --p 4099 --u-count 1000 --prime-target 120 --draw-cap 40000` | 0 | 183 |
| `lagr2` | 0 | <1 |
| `gsp --samples 200000` | 0 | 103 |
| `python3 -I analyze.py pilot --census-prefix pilot_census_p --gate-prefix pilot_ge_p257_ --tag pilot --g1-counts auto=200,B=40,C=20 --aux-prefix pilot_ --gates-only > pilot/pilot_gates.json` | 0 | 4 |

The decision-size census runs used exactly the decision parameters (PROTOCOL.md 3), so
the rate-type quantities (stage-B and stage-C frequency, unresolved, error) are
calibrated at the study's own sizes and primes. Every run was single-threaded. Memory was not measured; the largest allocation in
the code is the `p^3`-byte square table of `enumchi.rs` (17 MB at p = 257, gate-enum only). The G1 `auto` run took 676 s, under the 15-minute budget.

## 2. What it printed (gates only; `pilot/pilot_gates.json`, `gate_failures: []`)

| gate | pilot result | decision threshold |
|---|---|---|
| G0 | `test result: ok. 15 passed`; 0 FAILED/panicked lines | same |
| G1 auto (p = 257) | 200/200 ok, 200 agree, 0 disagree; resolved_by bsgs 199, n2 1 | 200/200 |
| G1 B (stage A skipped) | 40/40 ok, 40 agree; resolved_by n2 40 | 40/40 |
| G1 C (stages A, B skipped) | 20/20 ok, 20 agree; resolved_by n3 20 | 20/20 |
| G2 annihilation | 17,031 ok rows, 0 failed | 100% |
| G3 Weil | 17,031 rows, 0 failed | 100% |
| G4 criterion tests | 68,124 census (curve, l), 0 failed; 2,600,000 reference samples, 0 failed | 100% |
| G5 mod-2 structure | 17,031 rows, 0 failed | 100% |
| G6 J[2] table | 135 / 105 / 30, 0 pair-rule mismatches; output byte-identical to L06 `raw/l2_lagrangians.json` | same |
| G7 sampler | 0 similitude failures, max abs z 2.658 over 12 cells; output byte-identical to L06 `raw/gsp_reference.json` | abs z <= 4 |
| G8 completeness | error 0, unresolved 0 at every p; resolved_by: p=257 bsgs 3918 / n2 12; p=521 3882 / 1; p=1031 4196 / 0; p=2053 2404 / 0; p=4099 2618 / 0; no n3 | error 0; unresolved 0 at p <= 1031 |
| G9 streams | 87,253 streams, 0 overlaps, 0 wraps, longest stream 192 draws; 0 f-regeneration mismatches | 0 / 0 |
| G10 freshness (vs L06) | 51,638 L06 curves regenerated; 0 seed collisions, 0 f collisions, 0 duplicate f | 0 / 0 |

Every census run stopped at `prime_target` (120 PRIME at each p). Chi-requested draws:
3930, 3883, 4196, 2404, 2618 (17,031 in total). Draws scanned: 9453, 9446, 11863, 10582,
11794. These ranges are frozen into G10 so the decision draws must avoid them.

Stage B was reached on 13 of 17,031 census draws (0.076%). On every one it gave a unique
chi, so stage C was never needed in the census. Stage B was also forced on 40 draws and
stage C on 20, and both agreed with the independent table enumeration on every draw.
In L06 at p = 257, 9 of 3892 census draws and 1 of 40 gate draws were ambiguous. In
the pilot no draw at any p is left without a unique chi. The unit tests also resolve
L06's two ambiguous p = 257 census draws (`i = 18`, `i = 253`) to the enumerated chi
(test 14), and L06's first 20 p = 101 gate draws by all three paths (test 13).

Stage C cost: 20 forced-C draws at p = 257 took 104 s, including the table enumeration
done for G1. Scaling by `p^3` gives about 3 minutes per stage-C draw at p = 1031, which
is why stage C is capped at p <= 1031 (at p = 2053 it would be about 8 times that).

## 3. Reproducibility check of the build

A fresh build of the frozen sources (`cargo build --release --offline --target-dir
<scratch>/fresh`) reran `census --tag pilot --p 257 --u-count 3000 --prime-target 20
--draw-cap 40000`. Its output sha256
`c8248f1f7d0cde69c0221e59b1ca931b1dd76584260f897c7f1aebff9b87d1ea` equals the pilot's
`pilot/small/pilot_census_p257.jsonl`. This is the replay procedure of PROTOCOL.md 9 in
small.

## 4. Disclosures

- To exercise the non-gate code path of `analyze.py` (populations, S4, S5), the
  coordinator ran it once in full mode on the pilot data and printed only sanity fields:
  S5 recompute mismatches 0, S5 prediction violations 0, A2 predictions true, 30 S4
  cells, and pooled pilot population sizes (ALL 11,000, PRIME 600, C2 315, ODD 1,563).
  That printout also showed `S5 n = 178`, the number of pilot PRIME curves firing at
  `l = 3`. It is a pilot-instance count and not decision data. The decision thresholds
  are L06's, fixed before any F2 data and not changed. No other criterion rate from the
  pilot was looked at.
- The pilot census JSONL files (per-curve rows, 3-5 MB each) were deleted after
  hashing, to respect the output-size rule. Their sha256 are in `pilot/pilot_data.sha256`.
  They are reproducible bit for bit with `pilot/run_pilot.sh census` (decision-size
  pilot) and the frozen binary. The small runs used the earlier version of that script,
  with the parameters shown in `pilot/pilot_commands.log`.
- `pilot/run_pilot.sh` was edited once, between the small and the decision-size census
  runs (sizes only). Its final content is the one hashed in `SOURCES.sha256`.
- No repository hook refused any command.
