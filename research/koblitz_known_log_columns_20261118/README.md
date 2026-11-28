# Known-log seed columns on the small Koblitz rows (2026-11-18) `[pcc:unverified]`

Engineering lever on the validated generator-orbit IC arm (`research/ecbench_generator_orbit_20261022`) for the K1 rows n23, n29 and n37.
Every number here is `[pcc:unverified]`: counted GAE from local ecbench builds, L0, **no independent validation yet** (see "Standing").
Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen at 753f3fae10 before any data (`REGISTRATION.sha256`, `registered-at.txt`), with
addendum A1 frozen at 1c82dec2b0 after the A0 data and before any A1 data (`a1/REGISTRATION.sha256`). Report: [`REPORT.pdf`](REPORT.pdf);
figures `figures/fig1-phase-split.{svg,png}`, `figures/fig2-ratios.{svg,png}`.

## Result

| curve (log2 r) | incumbent gen-orbit arm | candidate | candidate / incumbent, w = 0 / 0.1 / 1 (95% CI) | standing of the read |
|:--|:--|:--|:--|:--|
| K1 n23 a=1 (22.00) | gen-c4 | A0 kl-c3 (primary) | 1.109 [0.867, 1.417] / 1.074 [0.850, 1.356] / 0.993 [0.810, 1.219] | null |
| K1 n23 | gen-c4 | **A1 sp-c4-d5 (primary)** | **0.588 [0.505, 0.683] / 0.605 [0.528, 0.694] / 0.645 [0.579, 0.718]** | improvement at all three weights |
| K1 n37 a=0 (27.78) | gen-c8 | A0 kl-c7 (primary) | 0.494 [0.392, 0.623] / 0.504 [0.404, 0.628] / 0.529 [0.436, 0.642] | improvement |
| K1 n37 | gen-c8 | **A1 sp-c8-d5 (primary)** | **0.381 [0.330, 0.440] / 0.398 [0.347, 0.456] / 0.442 [0.392, 0.498]** | improvement at all three weights |
| K1 n29 a=1 (15.37) | gen-c1 | A0 kl-c2 (secondary) | 1.703 [1.508, 1.923] / 1.806 / 1.934 | worse; lever inert at n29 (gen-c1 already is the one-column known-log base) |

A1 against A0's progression base (the off-state of the A1 parameter): sp-c4-d5 / kl-c4 0.667 [0.568, 0.784] / 0.690 / 0.741 (n23);
sp-c8-d5 / kl-c8 0.621 [0.506, 0.762] / 0.639 / 0.684 (n37). Default-off controls: gen-c<c0>-m1 = gen-c<c0> on every record (ratio 1.000
exactly), A/A 1.000; every solve verified (A0: 1536 + 1152 + 1536 records, A1: 1536 + 1536; `verify --replay-all` 0 problems on all five sessions).

## Distance to rho, before and after (paired ratios, w = 0 / 0.1 / 1)

| curve | arm | / rho.signed_frobenius_strong (leaderboard reference) | / rho.signed_frobenius_v5 |
|:--|:--|:--|:--|
| n23 | gen-c4 (before, A1 session) | 0.828 [0.751, 0.912] / 1.000 [0.913, 1.096] / 1.895 [1.742, 2.062] | 3.131 [2.788, 3.516] / 3.662 [3.277, 4.093] / 5.953 [5.375, 6.593] |
| n23 | sp-c4-d5 (after) | 0.487 [0.424, 0.559] / 0.605 [0.534, 0.687] / 1.222 [1.100, 1.357] | 1.840 [1.564, 2.165] / 2.216 [1.906, 2.577] / 3.839 [3.373, 4.369] |
| n37 | gen-c8 (before, A1 session) | 2.646 [2.333, 3.002] / 2.910 [2.570, 3.294] / 3.935 [3.482, 4.447] | 4.650 [4.115, 5.255] / 5.247 [4.656, 5.912] / 7.908 [7.067, 8.848] |
| n37 | sp-c8-d5 (after) | 1.008 [0.901, 1.129] / 1.158 [1.039, 1.289] / 1.740 [1.571, 1.926] | 1.772 [1.560, 2.012] / 2.088 [1.848, 2.358] / 3.496 [3.131, 3.903] |
| n29 | gen-c1 (unchanged; A0 session) | 0.094 [0.083, 0.107] / 0.149 [0.135, 0.163] / 0.546 [0.517, 0.577] | 1.260 [1.100, 1.443] / 1.883 [1.695, 2.092] / 4.918 [4.559, 5.306] |

Against v5 the IC arm is still above rho on all three curves at every weight (distance remaining at w = 0.1: n23 2.2x, n37 2.1x, n29 1.9x).
Against strong, n23 is below strong at w = 0 and 0.1 and above at w = 1 (order flips: no rank); n37 is at parity at w = 0 and above at 0.1 and 1.
The leaderboard's reference is unchanged (strong); the owner decision on v5 is pending.

## Phase split (mean GAE per solve at w = 0; A1 sessions for n23/n37, A0 for n29)

| curve | arm | factor base | pair table | relations | verify | total | relations found | trials | table entries |
|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| n23 | gen-c4 | 169 | 398 | 1013 | 30 | 1610 | 3.48 | 937 | 356 |
| n23 | kl-c4 (progression) | 7 | 398 | 994 | 30 | 1429 | 1 | 922 | 260 |
| n23 | sp-c4-d5 | 19 | 398 | 505 | 30 | 951 | 1 | 468 | 350 |
| n37 | gen-c8 | 720 | 2538 | 8124 | 39 | 11421 | 5.98 | 7558 | 2344 |
| n37 | kl-c8 | 15 | 2538 | 4442 | 39 | 7033 | 1 | 4135 | 1812 |
| n37 | sp-c8-d5 | 43 | 2538 | 1744 | 39 | 4363 | 1 | 1623 | 2326 |
| n29 | gen-c1 | 1 | 32 | 40 | 21 | 94 | 1 | ~40 | 26 |

## What the lever is (cost chain)

`compact-orbit-scan:...,generator_orbit=1,generator_multiples=k[,multiple_doublings=d]` seeds columns 0..k with the signed Frobenius orbits of
known multiples of G: the progression `[1]G..[k]G` (A0) or `[2^(d(j-1))]G` (A1; d doublings each). Each such column costs one to d group
operations instead of the scan's lift, cofactor projection, order check and eigenvalue check (3+ scalar multiplications), and needs no
relation of its own: at `k = columns` one relation pins the target. Real counted operations fall in two phases: factor base (n23 169 -> 19,
n37 720 -> 43) and relation search (n23 1013 -> 505, n37 8124 -> 1744: one relation instead of 3.5 / 6). The progression seeds give fewer
distinct pair sums (260 vs 356 at n23) and their pair sums lie along the stride-G target walk, so hits cluster: that is why A0 failed at
n23; the spread seeds restore the table (350 entries) and the hit rate. The pair table itself is unchanged.

Named so nobody mistakes it: with every column of known logarithm the pipeline is a Frobenius- and negation-folded baby-step giant-step
(the gen-orbit README already said this of `c = 1`). Its asymptotic cost is about `2 sqrt(r / 2n)` group operations against
`sqrt(pi r / 4n)` for a folded rho, about 1.6x above: this lever cannot pass a matched folded rho asymptotically and is no evidence for a
smaller exponent or for subexponential IC. It is a T0-scope engineering gain over the named IC baseline at these three sizes.

## Standing and deviations

- **Validation: not done.** This agent had no subagent tool, so no independent validator ran; there is no `VALIDATION.json`, and so (gate,
  docs/VALIDATION_GATE.md) no leaderboard row, no leaderboard component section and no `benchmarks/README.md` row. `benchmarks/20261118-koblitz-known-log-columns/`
  holds copies of the report and figures for browsing. What a validator needs is in "For the validator" below.
- No EKS/Iso Lab and no S3 (AWS token rejected; checkpoint token expired): local L0, `--cpus none --allow-busy`; raw sessions
  (`records.jsonl`, 9-13 MB each) stay in the scratchpad (`klc-sessions/`, `klc-sessions-a1/`), their SHA-256 in `summary.json` and
  `a1/summary.json`; compact per-record tables `tables/records-n*.csv.gz`, `a1/tables/records-n*.csv.gz`; full compare output
  `results.json.gz`, `a1/results.json.gz`. The benchrec raw objects are not uploaded.
- The container restarted during A0 after the n23 and n29 sessions had completed and before n37 started; n37 ran afterwards from the
  same spec and pinned binary under a second `benchrec.py run` (no partial session existed).
- A1's registered `a1/summarize.py` had a one-line syntax slip from a `sed` edit (the `R = ...` load landed inside a comment); fixed after
  the A1 data, logic unchanged (it reduces `results.json` only).
- A0 binary sha256 `c95535f4...48330` (commit 1b8e9e5cde; the tree also held another lane's test-only edit); A1 binary sha256
  `58cfff90...07f26` (code at 41783eef88). HEAD did not compile (duplicated fields in `f4_fp.rs` from a merge); fixed in 1b9590d88e.
- Pilots that sized the levers were disclosed in the protocol (incumbent-only phase split for A0; a 12-target A1 sizing pilot).
- `cargo test --release --lib ic_framework`: 207 passed, 3 failed (`openssl_independent_binary_group_control...` and
  `planner::tests::every_cited_file_exists`: files absent from this sparse clone; `the_symmetrised_koblitz_pipeline...`: a different base
  and oracle that this change does not touch; not re-checked on a clean HEAD). Both new tests pass.
- Multiplicity: A1 is the thread's second attempt on n23/n37 (A0 primary at n23 failed). Both A1 primary upper ends are at most 0.72,
  well under the 0.90 multiplicity margin of the protocol.

## For the validator

Recompute from `a1/tables/records-n{23,37}.csv.gz` (paired by target index and round) or from the scratchpad sessions:
sp-c4-d5 / gen-c4 and sp-c8-d5 / gen-c8 at w = 0, 0.1, 1 (`ecbench compare --weight w --resamples 4000 --seed 20261002`), and the
gap rows above; replay with `ecbench verify --dir <session> --replay-all`; check that `generator_multiples` absent / =1 leaves the base
unchanged (A0 `-m1` arms; unit tests) and that the known logs `h * 2^(d(j-1))` are right (unit test checks every base point's coefficient).
