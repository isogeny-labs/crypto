# Session log: rootfind_matched_f4_gate_20261123

## 2026-11-23 18:10–19:30 UTC (Claude, parallel-workflow agent, L0 local)

### What was done
- Read `research/rootfind_algo_survey_20261123/SURVEY.md` and its addendum.
  Located the gate text in `rootfinding_checkpoint_20261109/v3/REPORT.md:101`,
  `v4/REPORT.md:15` and `rootfinding_metric_20261110_v1/REPORT.md:21`.
- Established that the "20 preregistered hard instances" were never frozen.
  The closest frozen list in the checkout is the 5 planted NIST charts
  (`dual_ladder_20261109_v3/planted_inputs.json`). The 20 metric training
  cases depend on `curved_blocks_20261109_v6` and `raw-profile.json.gz`, and
  neither is in the checkout.
- Rebuilt program B's exact chart solver (`primal_parabola_20261109_v5/native.cpp`)
  from byte-identical sources (`scripts/build_program_b.sh`). The hashes match
  its PROVENANCE.json.
- Froze `PROTOCOL.md` (sha256 d0d362b4…) before any data. Two deviations were
  then logged in it before calibration: the lib build is broken, and
  `f4_fp::solve` scans all of F_p.
- Wrote `examples/rootfind_matched_f4_gate_20261123.rs` (F4-ideal and F4-visit
  arms) and `f4shim/gen_shim.py`, a standalone crate over the byte-identical
  `f4_fp.rs`.
- Ran Panel A (program B on the 5 frozen NIST charts, 3 runs), the
  calibration ladder, and the 20-chart M=128 panel (3 interleaved reps,
  A/A duplicates). Then ran a post-hoc exploratory M ladder (`scripts/explore.py`).
- Results ([pcc:unverified], L0):
  - Panel A: program B was complete on 5/5 frozen NIST charts in 3/3 runs
    (worker 0.64–0.93 s).
  - Panel B (M=128, 20 charts, 3 reps, 300 processes): root sets were equal
    across all arms and the brute force on every run.
    - B against fastest F4 (F4-visit): geometric mean 21.0× [19.8, 22.3].
    - B against F4-ideal: 1,147×.
    - Fastest baseline: 0.097 s per instance (< 1 s).
    - B's A/A median: 15 %.
    - So the 2× and 8× gates FAIL on measured conditions.
  - Post-hoc ladder: B was complete and 10–114× faster for M ≤ 768, and hit
    its 100k-node cap (incomplete, no roots) at M = 1024 and 1365, where
    F4-visit found all roots.

### Decisions made
- Built the F4 arms outside cargo's workspace because HEAD's lib does not
  compile and this task may not edit existing files. f4_fp itself is included
  unchanged.
- M_hard = 128 by the frozen rule: no ladder M reached ≥ 1 s for the fastest
  F4 arm, so the largest M at which an F4 arm completed was used, and the ≥ 1 s
  condition is recorded as unmet.
- Added the post-hoc M ladder (labelled exploratory, not gate-scored) because
  the lattice box volume ~81·M^8 against det p^2 predicts a crossover that the
  frozen ladder could not reach.

### Open threads
- No F5/F6 arm exists over odd prime fields in the repo, so those arms were
  not run.
- The 20 hard instances in the gate's sense (original S3 systems, not
  conditional charts) still need to be preregistered by the program owner.
- Qualified isolation (isolab-eks L2) was not available here.
- `src/cryptanalysis/prime_root_trace.rs:1108` (unclosed `{`) breaks the lib
  build at base 71a7881ede. It needs a separate fix PR.

### Errors
- The first `cargo build` of the example failed on the broken base lib. It also
  rebuilt dependency rlibs (`hashed_ecdl_tensor`), so the Nov-18
  `libcrypto_lib` rlib could no longer be linked against (E0460). This was
  worked around with the shim crate (115 MB in scratch).
- `f4_fp::solve` hung at p ≈ 2^32 (gdb: `roots_univariate` scans 0..p). This was
  worked around with `solve_visit` (deviation 2), before calibration.
- Disk fell below 800 MB (758 MB) after the builds, mostly through other
  agents' use. No further builds were started after that point.

## 2026-11-23 20:55–21:10 UTC (Claude, revision for the red-team review, L0 local, no reruns)

### What was done
- Read the review (4 blocking, 8 minor items) and checked each against
  `data/`. Verified: every F4-visit run has `f4_runs` = 1 + (M+1) + (M+1)²
  (16,771 at M = 128; also at every ladder M), `field_ops`/`f4_runs` ≈ 9.04;
  the post-hoc ladder has 3 distinct charts, 12 chart-M rows at M = 448–768 with
  ratio 9.83–113.65; `PROTOCOL.md`'s frozen-prefix hash reproduces as d0d362b4…;
  file mtimes match the reviewer's (PROTOCOL.md 18:38:23, calibration.json
  18:38:46, PROTOCOL.sha256 19:22:35).
- Fitted empirical exponents from `data/explore_posthoc.csv` (complete rows,
  one p): B nodes 5.53 (M 192–768), 5.58 (256–768), 5.87 (384–768); B time 4.90
  (384–768); F4-visit time 1.97 (128–1365). k=0/1 B spread: 0.132 (M 128–384),
  0.075 (448–768), 0.036 (1024–1365). All [pcc:unverified].
- Added `scripts/cubic_check.py` (exact O(M) integer-cubic scan, pure Python,
  untimed) and `data/cubic_check.txt`: 20/20 Panel B charts match the stored
  oracle roots, 3/3 match a fresh brute force, 15/15 ladder chart-M pairs
  (M = 128…1365) match F4-visit's stored root count and hash. ALL MATCH
  [pcc:unverified].
- Rewrote `README.md` (verdict, relabelled arms, easy-regime section, model vs
  measured scaling, gate FAIL-by-rule / inconclusive reading, red-team section).
  Updated labels and added p^(1/4), p^(1/3) markers in `scripts/figure.py`;
  regenerated `figure.svg`, `figure.png`, `report.html`, `report.pdf`.

### Decisions made
- Kept FAIL as the frozen-rule result and added "inconclusive" as the
  scientific reading, because the ≥ 1 s and A/A conditions were unreachable by
  design (ladder cap at M = 128; ~20 ms processes under the 5-case rule).
- Accepted all four blocking items: the data confirm each. No review number was
  found wrong; noted that M³ < p is the binding condition (M ≤ 1625).
- Did not time the cubic scan or add it as an arm: this revision reruns
  nothing, and an untimed check avoids an unrecorded benchmark.

### Open threads
- A timed O(M) cubic-scan arm, and any matched arm in the hard regime
  M > p^(1/3) (needs larger p and an F4 beyond `f4_fp`'s p < 2^32).
- More than one p, before any statement about p dependence.
- An externally anchored freeze (commit the protocol before data) for any
  follow-up run.

### Errors
- The first regenerated `figure.svg` had an unescaped `<` in a title, so the
  PNG render showed a parse error; escaped it as `&lt;` and re-rendered.
- `report_html.py` applies bold per line, so bold spans across wrapped README
  lines rendered as literal `**`; rewrapped those spans onto single lines.
