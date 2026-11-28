# Program B against matched F4 arms (2026-11-23)

Every measured number is [pcc:unverified]: one shared 4-vCPU container (L0)
with two other agents running. No leaderboard or `benchmarks/README.md` row.
This README was revised on 2026-11-23 to answer an independent red-team
review; the section "Red-team review and responses" lists each item and what
the data say about it. No experiment was rerun for the revision; the only new
computation is the untimed check `scripts/cubic_check.py`
(`data/cubic_check.txt`).

**Preregistration (self-asserted, not externally anchored).** `PROTOCOL.md`
states it was frozen at 18:24 UTC before any data. Its frozen-prefix hash
reproduces as `d0d362b4…` by the recipe in `PROTOCOL.sha256`, but that file
was written at 19:22, after all the data, and nothing was committed before the
data, so the freeze time cannot be checked. Treat the freeze as unverified. The
M ladder in `scripts/explore.py` came **after** the panel and is post-hoc and
exploratory. It is not used in scoring.

## Verdict

- **By the frozen scoring rule, release 2× and target 8× both FAIL.**
  **The scientific reading is inconclusive**, because two failed conditions could
  not have been met by this design:
  - *Fastest baseline ≥ 1 s* was unreachable: the frozen calibration ladder
    stopped at M = 128, where the fastest F4 arm takes 0.10 s.
  - *A/A ≤ 10 %* for program B was unreachable: its processes last about
    20 ms, so process start-up noise dominates (15 %), and the frozen native's
    5-cases-per-process rule forbids batching more work per process.
  - Isolation (L0) and field/curve coverage (prime only) were unmet by
    construction.
  - The result is therefore not evidence against program B, and not evidence
    for it either (next point).
- **The matched panel is in an easy regime (no hard instances).** At p = 4294967291, every M in Panel B (128) and in the post-hoc
  ladder (≤ 1365) lies below p^{1/3} ≈ 1625. There, fixing S = x+y+z makes the
  residues e2, e3 equal to the true integers (3M² < p and M³ < p), so the roots
  are the integer roots of one cubic t³ − S t² + e2 t − e3 per S: an exact
  O(M) scan of 3M+1 cubics. `scripts/cubic_check.py` implements that scan in
  pure Python with exact integer arithmetic. It reproduced the root sets of all
  20 panel charts, a fresh brute force on 3 charts, and F4-visit's stored root
  hashes on 15 ladder chart-M pairs up to M = 1365, including the two M values
  where program B returned nothing [pcc:unverified]. No arm in this study was
  run in the hard regime M > p^{1/3}, where Panel A's NIST charts live.
- **"F4-visit" is a 2D enumeration tree, not Gröbner work.** On every run its
  `f4_runs` equals 1 + (M+1) + (M+1)² exactly (16,771 at M = 128), at about
  9.0 F_p multiplications per F4 call on matrices of at most 14 × 34. It
  branches over every (x, y) in [0,M]² and calls a trivial F4 at each leaf. Its
  roughly M² growth is the enumeration's. **F4-ideal is the only**
  **real Gröbner arm.**
- **Ratios on the M = 128 panel (component numbers, easy regime):**
  - program B against F4-visit (the enumeration tree): geometric mean 21.0×,
    instance-bootstrap 95 % CI [19.8, 22.3], worst chart 15.5×, best 25.7×
    [pcc:unverified];
  - program B against F4-ideal (Gröbner): 1,147× [pcc:unverified]. F4-ideal had
    no A/A duplicate (by protocol; it is never the baseline);
  - root sets are identical across all three arms and an independent brute
    force on 20/20 charts × 3 repetitions.
- **The gate is still unscoreable as written.** Its "20 preregistered hard
  original-all-roots instances" never existed. The 5 NIST charts in the
  checkout are planted controls; the 20 metric training cases cannot be
  rebuilt (inputs absent). At the real chart scale (M = 2^40…2^106, p of
  192–521 bits) no matched F4 arm can be posed: it would need domain
  polynomials of degree M+1 ≥ 2^40, and `f4_fp` requires p < 2^32.
- **Post-hoc ladder (exploratory, 3 charts, easy regime).** Program B was
  complete up to M = 768 (about 3·p^{1/4}) and 9.8–114× faster than F4-visit on
  the 12 chart-M rows at M = 448–768 where F4-visit took ≥ 1 s
  [pcc:unverified]. At M = 1024 and 1365 it hit its frozen 100,000-node cap on
  6/6 chart-M rows and returned **no roots, flagged incomplete**. The measured
  node growth is about M^5.6–M^5.9 (below). An M^8/p² box-volume argument and a
  p^{1/4} threshold are a **model**, not a finding; only one p was tested.

**Do the program-B claims survive?** No 2×/8× release claim survives, and
the panel cannot support a positive characterisation either:

- Program B is complete and quicker than both F4 arms at p ≈ 2^32 for
  M ≤ 768 [pcc:unverified], but that whole range is solvable exactly by an O(M)
  integer-cubic scan, so it is not a hard regime and the ratios do not
  transfer to M > p^{1/3}.
- Program B becomes incomplete under its frozen cap by M = 1024 at this p.
- Nothing here changes relation yield or the counted S of any ECDLP pipeline
  (survey: no study claims IC below a matched rho).

## What was run

| Item | Detail |
|---|---|
| Program B arm | `primal_parabola_20261109_v5/native.cpp`. Rebuilt from byte-identical sources by `scripts/build_program_b.sh` (native 7e4464c8…, prefix 0ff2791b…, both matching its PROVENANCE.json). It charges its own brute-force controls in every process. |
| F4-ideal arm (Gröbner) | `f4_fp::solve_visit`, no domains, on {f1, f2, D(x), D(y), D(z)}, with D(v) = ∏_{j≤M}(v−j), Grevlex, degree cap 3(M+1)+3. One F4 call per solve, matrices up to about 374k × 375k at M = 128. |
| F4-visit arm (enumeration) | `f4_fp::solve_visit` on {f1, f2}, with domains [0..M] per variable, Grevlex, cap 12. It visits 1 + (M+1) + (M+1)² nodes, one tiny F4 call each (≤ 14 × 34). |
| F5 / F6 | Not available over odd prime fields in this repo; not run (not independently re-checked by the reviewer) |
| Build | Built through `f4shim/gen_shim.py`, which includes `f4_fp.rs` and `f4_fp/*.rs` unchanged. The lib does not compile at base 71a7881ede (`src/cryptanalysis/prime_root_trace.rs:1108`, unclosed `{`). |
| Panel A | Program B on the 5 frozen NIST charts, 3 cold runs: complete, 3 ordered tuples each and the expected multiset on 5/5. Process wall for all 5 charts together (not per chart): 0.93 / 0.69 / 0.64 s (the earlier record was 0.585 s native) |
| Calibration | M ∈ {4…128}, 3 charts, 1 solve per process. The fastest arm (F4-visit) never reached 1 s (0.10 s at M = 128; F4-ideal 4.7–5.2 s), so M_hard = 128 by the frozen fallback rule |
| Panel B | p = 4294967291, M = 128, 20 charts (10 planted with 6 ordered roots each, 10 unplanted with none). 3 repetitions in shuffled order. Per chart: B, F4-visit, F4-ideal, F4-visit′, B′. Each process solves 5 copies. 300 processes in total |

## Panel B numbers (per instance = process wall / 5, median of 3 repetitions)

| Arm | Median seconds | Counted unit |
|---|---:|---|
| program B | 0.0045 | 0–7 enumeration nodes, 49–73 LLL steps |
| F4-visit (enumeration tree) | 0.097 | 151,677–151,713 F_p multiplications over 16,771 F4 calls |
| F4-ideal (Gröbner) | 5.20 | 55.6 M–63.0 M F_p multiplications in one F4 call |

- A/A median |log t/t′|: B 0.153 (p90 0.438), F4-visit 0.077 (p90 0.232).
  B's process lasts only about 20 ms, so start-up noise dominates it.
- B's per-instance time includes process start-up and its own brute-force
  controls, which penalises B, so the ratios are conservative for B.
- The counted units are not commensurable. No cross-unit ratio is formed.

| Gate condition | Release 2× | Target 8× |
|---|---|---|
| Geometric mean ≥ k (vs F4-visit) | 21.0 ✓ | 21.0 ✓ |
| Worst slowdown ≤ 1.10 (min ratio 15.5) | ✓ | ✓ |
| Root sets equal (20/20 × 3 reps × 3 arms + brute force) | ✓ | ✓ |
| Fastest baseline ≥ 1 s | ✗ (0/20; unreachable on the frozen ladder) | ✗ |
| A/A ≤ 10 % | ✗ (B 15 %; process-noise floor) | ✗ |
| ≥ 3 cold reps, 20 instances | ✓ | ✓ |
| Qualified isolation; field/curve coverage | ✗ (L0; prime only) | ✗ |
| Hard instances (M > p^{1/3}) | not tested (all M < p^{1/3}) | not tested |
| **Result by the frozen rule** | **FAIL** | **FAIL** |
| **Scientific reading** | **inconclusive** | **inconclusive** |

Predictions (as frozen; note that "F4" in P2/P3 is the enumeration tree):

- **P1 (≥ 8× against F4-ideal): held**, at 1,147×.
- **P2 (F4-visit is the stronger baseline on ≥ 18/20): held** (20/20).
- **P3 (≥ 2× against the fastest F4 arm, worst ≤ 1.10): held** on the ratio.
- **P4 (identical root sets): held.**

## The easy regime M < p^{1/3} (`scripts/cubic_check.py`)

For (x,y,z) ∈ [0,M]³, e2 ≤ 3M² and e3 ≤ M³. When both are below p, the chart
equations e2 ≡ t0 S² + t1 S + t2 and e3 ≡ u0 S² + u1 S + u2 (mod p) fix e2
and e3 as integers for each S ∈ [0, 3M]. Then {x, y, z} is the multiset of
integer roots of t³ − S t² + e2 t − e3, so 3M+1 integer cubics settle the chart
exactly. At p = 4294967291, M³ < p holds for M ≤ 1625, which covers Panel B
(M = 128, 385 cubics) and the whole post-hoc ladder (M ≤ 1365, 4,096 cubics at
M = 1365). The reviewer's argument checks out.

The script finds integer roots exactly (integer critical points, integer
bisection on monotone pieces, exact deflation), with no floating point
deciding a root. `data/cubic_check.txt` [pcc:unverified]:

| Comparison | Charts | Result |
|---|---|---|
| Stored Panel B roots (= brute-force oracle), M = 128 | s1–s20 | 20/20 match (10 × 6 roots, 10 × 0) |
| Fresh brute force over [0,M]³ | s1, s11 at M = 64; s2 at M = 128 | 3/3 match |
| F4-visit stored root count and hash | s1, s2, s11 at M = 128, 384, 768, 1024, 1365 | 15/15 match |

The unplanted charts (0 roots) are a weak test on their own; the 10 planted
panel charts and the planted ladder charts carry the check. At M = 1024 and
1365 this also gives an independent check of F4-visit's root sets, which the
original write-up lacked. The script was not timed and is not a benchmark arm.

## Post-hoc exploratory ladder (`data/explore_posthoc.csv`)

3 distinct charts (s1, s2 planted; s11 unplanted), 11 M values, 2 runs per arm
(k = 0, 1), no brute-force oracle in the run itself.

| M | B s/inst | F4-visit s/inst | Ratio | B complete |
|---|---:|---:|---:|---|
| 128 | 0.004–0.005 | 0.09–0.10 | 20–23 | 3/3 |
| 384 | 0.009–0.010 | 0.82–0.86 | 88–97 | 3/3 |
| 512 | 0.025–0.044 | 1.46–1.51 | 35–59 | 3/3 |
| 768 | 0.18–0.32 | 3.1–3.3 | 9.8–18.8 | 3/3 (41k–77k nodes) |
| 1024 | 0.40–0.44 | 5.7–5.8 | — | **0/3**: cap hit, no roots |
| 1365 | 0.40–0.43 | 10.2–10.4 | — | **0/3**: cap hit, no roots |

Measured scaling (least-squares fits of log value on log M over the complete
rows, one p only) [pcc:unverified]:

| Quantity | M range (rows) | Empirical exponent |
|---|---|---:|
| B enumeration nodes | 192–768 (24) | 5.53 |
| B enumeration nodes | 256–768 (21) | 5.58 |
| B enumeration nodes | 384–768 (15) | 5.87 |
| B nodes, median-to-median | 256→512 / 384→768 / 640→768 | 5.55 / 5.95 / 6.17 |
| B seconds per instance | 384–768 (15) | 4.90 |
| F4-visit seconds per instance | 128–1365 (33) | 1.97 |

- **Model, not measured:** the lattice box has volume about 81·M^8 against
  determinant p², which predicts nodes ∝ M^8/p² once M passes about p^{1/4}
  (= 256 here). The measured local exponent is about 5.5–6.2 and still rising,
  not 8, and no p dependence was measured because only p = 4294967291 was used.
  Program B stayed complete at M = 768 ≈ 3·p^{1/4}, so "only when M ≪ p^{1/4}"
  is not supported; the data say only that the 100k-node cap is hit somewhere
  in 768 < M ≤ 1024 at this p.
- **[derived, not measured]** Extrapolating B's local time exponent (about
  M^6.3 between M = 640 and 768) against F4-visit's M² from the median ratio of
  about 10× at M = 768 puts a crossover near M ≈ 1,300 at this p if the cap
  were lifted. This is an extrapolation within the easy regime.
- k = 0/1 spread (post-hoc, not the frozen A/A design): B's median
  |log t/t′| is 0.132 at M = 128–384 (15 pairs), 0.075 at M = 448–768 (12) and
  0.036 at M = 1024–1365 (6) [pcc:unverified]. This is consistent with the
  15 % A/A at M = 128 being a short-process noise floor rather than a property
  of the solver.

## Red-team review and responses

Blocking items:

1. **F4-visit mislabelled as genuine F4.** Accepted. Verified in the data:
   `f4_runs` = 1 + (M+1) + (M+1)² on every F4-visit run at every M, and
   `field_ops` / `f4_runs` ≈ 9.04 at M = 128. Relabelled throughout as an
   (M+1)² domain enumeration with a trivial F4 call at each leaf; F4-ideal is
   named as the only real Gröbner arm.
2. **Easy regime and missing O(M) baseline.** Accepted and verified: the
   argument is correct, and `scripts/cubic_check.py` reproduces every checked
   root set. Panel B and the ladder are now described as an easy regime; no
   hard-regime evidence is claimed. The O(M) scan was not added as a timed arm
   (no reruns in this revision); doing so is an open thread.
3. **Scaling claim unsupported.** Accepted. M^8/p² and p^{1/4} are relabelled as
   a model; the measured exponent (5.5–6.2) and the single-p limitation are
   reported. The reviewer's "about 5.6–5.9" matches the fits above.
4. **Gate outcome framing.** Accepted. FAIL is kept as the frozen-rule result;
   the reading is inconclusive because the ≥ 1 s and A/A conditions were
   unreachable (ladder cap at M = 128; about 20 ms processes under the 5-case
   rule). The reviewer's post-hoc figure (9.8–114× at M = 448–768) is
   confirmed, with the caveat that those rows are also in the easy regime.

Minor items:

- **Freeze unanchored.** Accepted. The hash recipe reproduces `d0d362b4…`, but
  `PROTOCOL.sha256` (19:22:35) postdates the data, `PROTOCOL.md`'s last write
  (18:38:23) is 23 s before `calibration.json`'s last write (18:38:46), and
  nothing was committed. The freeze and the "before calibration" wording of
  both deviations are labelled unverified above.
- **"M = 448–768, 9 charts".** Accepted: it is 3 distinct charts, 12 chart-M
  rows over 4 M values, 2 runs per arm, no brute-force oracle. Corrected.
- **"10–114×".** Accepted: the minimum is 9.83× (M = 768, s2). Corrected to
  9.8–114×.
- **F4 completeness at M ≥ 1024 not independently checked.** Partly resolved:
  the exact cubic scan reproduces F4-visit's root count and hash on all 6 chart-M
  pairs at M = 1024 and 1365 [pcc:unverified].
- **B time includes start-up and its own controls.** Agreed; noted under the
  Panel B table. It biases the ratio against B.
- **F4-ideal has no A/A duplicate.** Agreed; noted next to the 1,147× figure.
- **Panel A "worker 0.64–0.93 s".** Accepted: relabelled as process wall for
  all 5 charts together.
- **F5/F6 availability not independently checked.** Stated as such in the
  "What was run" table; this revision did not re-check it.

Where the review is incomplete: none of its numeric statements were found
wrong. One refinement: the reviewer's condition "3M² < p and M³ < p" is
dominated by M³ < p at this p (3M² < p holds up to M ≈ 37,800), so the
threshold is M ≤ 1625.

## Not done

- **A timed O(M) cubic-scan arm**, and any arm in the hard regime
  M > p^{1/3} (needs a larger p and an F4 beyond `f4_fp`'s p < 2^32).
- **Calibration above M = 128 under the frozen protocol**, and B processes
  long enough to get under the A/A limit.
- **More than one p**, needed before any statement about p dependence.
- **F5 and F6 arms:** no odd-characteristic implementations exist in the repo.
- **The gate's 20 original-S3 instances:** never preregistered by the owner.
  This run used synthetic charts in program B's exact public contract (uniform
  coefficients), not charts derived from curves.
- **The full NIST scale:** no matched F4 can be posed there.
- **Qualified isolation:** not run (no isolab-eks or S3 here).
- **A cargo-built example:** blocked by the broken base lib. The shim build is
  documented, and the example file compiles against the unchanged `f4_fp.rs`.

## Files

`PROTOCOL.md`, `scripts/` (panel, score, explore, explore_summary,
cubic_check, figure, report_html, render, build_program_b), `f4shim/`
(generator and source hashes), `data/` (calibration.json,
panelA_program_b.json, panel.jsonl.gz, per_instance.csv, summary.json,
explore_posthoc.{jsonl.gz,csv}, cubic_check.txt), `figure.svg`, `figure.png`,
`report.html`, `report.pdf`, `SESSION_LOG.md`. The example is at
`examples/rootfind_matched_f4_gate_20261123.rs`.
