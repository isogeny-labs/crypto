# Validator report: research/quartic_diem_ladder_20261121

Lead #1 ladder: Diem 2LP index calculus on prime-order plane quartics with a sampled factor
base (`2lp-sfb`) and the enumerated base (`2lp-new`), against the ideal negation rho.

- Validator role, Claude Opus 5.5, 2026-11-23. Every number below is `[pcc:unverified]`: no
  pcc claim references this study, and the validator created none.
- **Validated commit:** `6b5e2a7d668c7f5fc74105dcaad7cb4ffbcf7ad0`, branch
  `claude/hyperelliptic-index-calculus-td1fdn`, clean tree at dispatch.
  - The study tree is `3baebc98699047869f5e86959fcd5a78ba312cc5` there, at `origin/main`
    `77a9462eebb` and at `3b2b980d617` (the commit that added RESULTS.md and the 131101 rows).
  - The local `main` ref (`d8691849`) is stale and does not contain the study.
  - Later WIP snapshot commits (`f4685fdced8`, `03ddd53f3a2`, `5299e594a29`) copied this
    `validation/` directory while it was half-written. They are not authoritative.
    `git diff 6b5e2a7d668 HEAD -- <study> ':(exclude)<study>/validation'` is empty: no producer
    file changed during the validation.
- **Binary:** `genus_ladder.qsfb`. Its sha256 is `f0243cf8…e38b`, equal to `binary.sha256`. It
  was hashed and copied at dispatch, before anything else ran.
  - Filesystem birth = mtime = ctime = 2026-11-21 04:58:45 UTC.
  - Not rebuilt: no Rust builds were allowed on this host.
- **Environment:** Python 3.13.18, numpy 2.5.3, single thread. Host load average was 3 to 5 on 4
  cores, from other agents' builds.

## Disposition: **validated**

The measurement and every claim in the dispatch reproduce:

- the certificates, with independent arithmetic;
- the rows, by replay;
- the numbers, by recomputation.

The frozen verdicts hold. Four corrections to the producer's text are required (F8, F9, F10, F11).
None changes a number in `analysis.json` or a frozen verdict. Per lesson L-8d86e8fc352c they
belong in a sibling `ERRATA.md`; `RESULTS.md` stays byte-equal.

## Requirements

| # | requirement | result | evidence |
|--:|---|---|---|
| 1 | Freeze before data | pass | F1 |
| 2 | Bindings (commit, hashes, binary, seeds, commands) | pass, with limits | F2 |
| 3 | Certificates re-verified with independent code | pass: 210 of 210 rows, 40 of 40 curves | F3; `certificates.json` |
| 4 | Replay with the frozen binary, `--threads 1` | pass: 72 of 72 rows, 12 of 12 instances | F4; `replay-certificate.json`, `replay_compare.json` |
| 5 | Ratios, intervals and exponents recomputed from raw rows | pass | F5; `recompute.json` |
| 6 | Accounting fairness of the sampled factor base | pass | F6 |
| 7 | Reference and calibration | pass (conservative toward rho) | F7 |
| 8 | Protocol adherence | deviation, and its disclosure is wrong | F8; `cap_subsets.json` |
| 9 | Band rule, and the "below rho at w = 0.1" reading | not a ranking; a wording fix is required | F9 |
| 10 | Reporting contract (unit, labels) | minor corrections | F10, F11, F12 |
| 11 | Repository policy (§13 artifact store) | process finding | F13 |
| 12 | Gate manifest | `scripts/validation_gate.py evidence <study>/validation` prints `valid` | `VALIDATION.json` |

## Findings

**F1. The freeze precedes the data (Git).**

- At `17c4a093105` (2026-11-21 04:35:44 UTC) Git holds PROTOCOL.md, analyze.py and the three
  run scripts with exactly the `frozen.sha256` hashes. They have not changed since.
  - analyze.py was already final at `a4f60fac027` (04:32:14, "state at container restart").
  - Between 04:32 and 04:35, PROTOCOL.md gained only the run-script paragraph and §6c (the
    restart disclosure).
- The bound sources match `source.sha256`. Both files are at `f307f7863d7` (04:10:05), which
  precedes the run.
- Build 2 was scripted at 04:35:12 (`qsfb_buildall.sh`) and finished at 04:58:28
  (`qsfb_build2.log`, 23 min 05 s). The binary copy was made at 04:58:45. The tests ran after
  that.
- `raw/main_start.txt` is 05:31:11Z. The first measured data entered Git at `502beba9321`
  (06:38:11).
- The restart account (about 04:30) is corroborated by the 04:32 snapshot.
- The dev data are at the non-grid primes 4093 and 8191. The dev build's library source differs
  from the bound source by one test-only line. No grid-prime sfb data predates the freeze; the
  producer's scratch directory was searched.
- The first four curves at each prime are pre-check 2's curves (same orders). Their 2lp-new
  results were known before the freeze; the protocol discloses this.

**F2. Bindings.**

- `sha256sum -c` passes on `frozen.sha256` and `data.sha256`. The binary hash equals
  `binary.sha256`.
- The hash files themselves first entered Git together with the data (06:38:11). The content
  they bind was in Git earlier (F1). The binary's existence before the data rests on its file
  times and on the replay (F4).
- **Binary-to-source binding is not verified.** No rebuild was allowed, and `source.sha256` binds
  only 2 files of the crate.
- The binary-to-data binding is verified (F4).
- The exact commands are in the frozen scripts, and the config rows agree with them.
  `quartic_inhom` is absent from the config row, but every IC row carries `inhom_rows = 4`.

**F3. Certificates, re-verified from Git with independent arithmetic** (`picarith.py`,
`verify_certificates.py`).

- `picarith.py` is the validator's own Pic^0 law for smooth plane quartics: Riemann–Roch linear
  algebra over F_p, with the residual rule modulo the saturated complete intersection (c, F). It
  shares no code with the solver. Only the record encoding (RREF conic net through D, with
  D ~ x + K − P0) was taken from the instrument's documentation. It is confirmed numerically:
  [t]G reproduces the recorded T net exactly.
- Per curve, all 40 (8 primes × 5), with rows read by `git show 6b5e2a7d668:<path>`:
  - F smooth (Macaulay degree-7 test on the partials); P0 on C.
  - The instance is identical on every row.
  - r prime; h = 1; order = r.
  - (√p−1)^6 ≤ r ≤ (√p+1)^6 < 2r, compared as exact integers. With G ≠ 0 and [r]G = 0 this
    proves #J = r.
  - The G and T records are RREF nets whose base scheme has length 3 and lies on C.
  - [t_secret]G = T.
  - Every row has `recovered = t_secret`, `certified = ok = true`.
  - Negative controls reject: [t+1]G ≠ T, and a T net with one coefficient changed (base scheme
    of length 0 or 2, not 3).
  - [r]X = 0 for two random point classes.
  - #C(F_p) recounted by the validator equals `n_points`.
- Result: 40 of 40 curves pass, and **210 of 210 rows** (80 `2lp-new`, 80 `2lp-sfb`, 50
  `Q_rho_neg`).
- At p = 131101 the curves are 53, 177, 193, 228 and 288:
  - N1 = 131479, 130888, 131484, 131117, 130907;
  - all 20 IC rows certify;
  - under 7 s per curve.
- Group-law checks pass at 2053, 16411 and 131101 (`grouplaw_checks.jsonl`): associativity,
  commutativity, identity, [r−1]X + X = 0, and [a]X + [b]X = [a+b]X.

**F4. Replay** (`replay.sh`, `compare_replay.py`, `replay-certificate.json`).

- Frozen flags, one thread, the hash-checked binary.
- At 131101: all 5 curves (20 IC rows, 10 rho-skip rows).
- At each of 1031, 2053, 4099, 8209, 16411, 32771 and 65537: one curve, drawn with
  `random.Random(20261123)` (317, 105, 205, 16, 382, 32, 369). That is 28 IC rows, plus 10
  measured-rho rows at p ≤ 16411 and 4 skip rows.
- **72 of 72 rows** equal the committed rows on every field except `wall_ns` (55 fields per IC
  row, 22 per rho row). All 12 instance rows are equal.

**F5. Recomputation.**

- `analyze.py` rerun on the Git copies of the raw rows reproduces `analysis.json` byte for byte
  (sha256 `0f161ed8…`).
- `recompute.py` is independent of `analyze.py`. It builds intervals by exact enumeration of the
  5^5 cluster-bootstrap resamples, uses its own generator for slope intervals, and indexes keys
  with `[]`.
  - It reproduces all 48 cells of the RESULTS IC/rho table, every exponent, every paired ratio,
    the crossovers and the calibration rows.
  - Point estimates agree exactly. Intervals agree with analyze.py's Monte Carlo intervals to 3–4
    digits.
- No IC row lacks any key that analyze.py reads with `.get(…, 0)` (lesson L-99ee952350b2). The
  idx counters therefore always enter the band.
- The frozen verdict is UNRESOLVED in every (arm, prime) cell under both interval methods.

**F6. Accounting fairness of the sampled factor base** (source `f307f7863d7`:
`QFactorBase::Sampled`, `Ctx::new_dynamic`, `insert_new`, `lookup`, `sampled_base`).

- **No lookup is a free oracle.**
  - `Ctx::lookup` counts every lookup.
  - A miss in dynamic mode pays a counted `on_curve` check (at most 54 multiplications) and a
    counted `insert_new` (6 multiplications and one insert).
  - The index only ever contains points that were:
    - sampled by counted root finding (`rel_enum`);
    - met as residuals of counted line computations;
    - found by the counted algebraic descent.
- The enumerated arm's full index answers a non-curve candidate with a hash miss, but it paid for
  the enumeration. That asymmetry favours `2lp-new`, not `2lp-sfb`.
- **`idx_lookups` and `idx_inserts` are priced in both arms.** Both appear in every IC row, and
  both are in `N_all` for both arms (analyze.py and the validator's recomputation).
  - `2lp-new` counts one insert per enumerated point (`idx_inserts = n_points` in 80 of 80 rows).
  - `2lp-sfb` counts one insert per inserted point (`idx_inserts = idx_points` in 80 of 80 rows).
  - Together they are 0.09% of `N_all` for sfb at 131101.
- **Nothing was moved out of counted fields.**
  - The C0 dev comparison reproduces: 8 `2lp-new` rows equal the pre-check-2 binary's on all 39
    common fields.
  - In the bound source, `rel_enum` and `rel_decomp` merge into `rel` for both arms. Sampling
    work is in `rel_enum`; on-curve checks and monomial values go to `rel` (lines) or
    `rel_decomp` (descents).
- One piece of uncharged set-up information reaches the sfb arm: `inst.n_points` (from the
  instance enumeration). It is used only for the `nb` clamp and the `max_decomp` cap, and neither
  binds: `nb = ceil(3√p) ≪ n`, and the largest `decomp_trials` is 0.8% of the cap.
- Uncounted in both arms, and unpriced:
  - a `seen_lines` HashSet insert per split line;
  - a `vid` HashMap entry per large-prime occurrence;
  - 4-element sorts.
  - Lower-bound estimate: 0.05% (new) and 0.08% (sfb) of `N_all` at 131101. Immaterial.

**F7. Reference and calibration.**

- The primary reference is the ideal negation rho, sqrt(π r / 4) Pic^0 additions, with nothing
  charged at any w. This is the expected collision time on the r/2 classes {±P}.
- Generic random quartics have automorphism group {±1} on Pic^0, and the field is prime (no
  Frobenius speed-up), so negation is the strongest symmetry. This was not checked curve by
  curve.
- The executed `Q_rho_neg` (50 runs at p ≤ 16411, all certified in F3) averages 1.007 × R in
  steps. The geometric mean is 0.886; the ratio gm/mean = 0.88 matches the 0.85 expected for a
  Rayleigh-distributed collision time. So the closed form matches the executed walk.
- The executed walk's F_q work per step is 2.12 to 2.34 × A, because the canonicalisation's Pic
  negation is uncharged in `group_ops`. In FPM the ideal reference therefore favours rho by about
  2.2×.
- §5a listing gap (informational): `Q_rho_neg` rows do not emit the walk's native counters
  (canonicalisations, short-cycle window compares, DP inserts). `rho_reference_walk` tallies some
  of them, but `QRhoOut` drops them, so the calibration arm cannot itself be band-priced. It is
  not used for any verdict.
- Charging rho even one native operation per step (band.rs style) would move sfb at 131101 to
  IC/rho = 0.70 (w = 0.1) and 3.87 (w = 1). The verdict stays UNRESOLVED.

**F8. Protocol deviation: the un-enforced 110-minute cap did change the 131101 cell.**

- RESULTS deviation 4 says a rerun under the cap rule "would have used the same two admitted
  curves (228 and 288), so the data are unaffected". The committed cell row contradicts this.
  - The even process tried 144 seeds in 8,335 s (57.9 s per seed; the odd process ran at
    58.1 s per seed).
  - Curve 288, the 144th even seed, was admitted at about 139 min. Curve 228, the 114th, was
    admitted at about 110.0 min, on the cap.
- Under PROTOCOL §3 the process stops at 110 min and is rerun with the curves admitted so far.
  So 288 is excluded and 228 is borderline, and the 131101 cell has 3 or 4 curves, not 5.
- Recomputed (`cap_subsets.py`, exact bootstrap):

  | 131101, sfb | w = 0 | w = 0.1 | w = 1 | verdict |
  |---|---|---|---|---|
  | as reported {53, 177, 193, 228, 288} | 0.000192 | 0.775 [0.676, 0.885] | 7.75 [6.75, 8.85] | UNRESOLVED |
  | cap, with 228 | 0.000179 | 0.746 [0.639, 0.871] | 7.46 [6.38, 8.71] | UNRESOLVED |
  | cap, without 228 | 0.000196 | 0.781 [0.628, 0.876] | 7.81 [6.28, 8.76] | UNRESOLVED |

- `2lp-new` remains UNRESOLVED in every subset.
- The ln(IC/rho) slopes move by at most 0.013. Pooled paired FPM sfb/new is 0.683 (as reported),
  0.683 (cap, with 228) and 0.691 (cap, without 228).
- **Required:** an ERRATA entry stating that the cap would have excluded curve 288 (and possibly
  228), with this table beside the reported cell.

**F9. "Below rho at w = 0.1 at 131101" is not supported as a ranking.**

- The frozen §8 rule and AGENTS.md §5a rank only where the order holds at w = 0, 0.1 and 1. Here
  IC is below rho at w = 0 and 0.1 and above it at w = 1, so the verdict is **UNRESOLVED**.
- The w = 0.1 band point is 0.775 [0.676, 0.885]. All five curves are below 1 (0.63 to 0.90).
  A log-scale t-interval over the 5 curves is [0.62, 0.97]. Under the cap subsets it is 0.746 to
  0.781.
- That is a band-point reading, not a result. RESULTS "Next steps" item 1 says the gate is
  "needed before any 'below rho' statement at w = 0.1". Validation does not license such a
  statement either.
- **Required wording:** "the w = 0.1 band point of IC/rho is 0.775 [0.676, 0.885]; the band
  verdict is UNRESOLVED."
- `VALIDATION.json` records `order_holds_across_band: false`.

**F10. The unit of "2lp-sfb costs 0.683 of 2lp-new".**

- 0.683 [0.640, 0.729] is the pooled paired ratio in FPM mid: Pic multiplications + 10 ×
  inversions + `rel_mul` + 9 × `la_mul`, with additions excluded.
- In the band's units (validator recompute):

  | unit | pooled paired sfb/new |
  |---|---|
  | `group_ops` (w = 0) | 0.846 [0.747, 0.964] |
  | all native operations (dominates w = 0.1 and w = 1) | 0.606 [0.573, 0.639] |

- The step A gain therefore holds in every unit, with every upper bound below 1. **Required:**
  cite 0.683 together with its unit.
- "la_add is 85% of native operations" holds for sfb at 131101 (0.852; 2.78e8 of 3.26e8). For
  2lp-new at 131101 the share is 0.55.

**F11. Labels of the sensitivity readings.**

- "Ranked at count-calibrated price" is not AGENTS.md §5a's "ranked at calibrated price", which
  needs a timing `w_cal`.
- §5a's calibrated weight also applies to field-arithmetic counters only, while the
  count-calibrated row prices hash operations and additions as well.
- The FPM reading (IC at about 1/1000 of the ideal rho at 131101) excludes all additions. With
  `rel_add`, `la_add` and the idx operations priced as one multiplication each it is 0.0027
  (sfb) and 0.0039 (new). Still far below 1.
- **Required:** label these as count-model sensitivities, never as a §5a ranking.

**F12. Minor descriptive errors.** None of these affects a number in `analysis.json`.

- A per curve is 3,990 to 4,795 FPM per Pic operation (about 4,007 at 131101). RESULTS says
  "about 3,600 to 4,000" and "about 3,900".
- Builds took 19 and 23 min, not "about 45 min each".
- The 5-curve percentile intervals under-cover relative to a t-interval. For example, at 65537
  the sfb w = 0.1 t-interval is [0.76, 1.05]. No verdict depends on this.

**F13. Repository policy.**

- `raw/run.jsonl` (241,976 B, `.jsonl` > 64 KiB) is tracked in Git, against AGENTS.md §13:
  `tools/artifacts/artifacts.py plan` selects it, and there is no `.artifacts.json` in `raw/`.
- It reached `main` through WIP snapshot commits. If it moves to the store, the `raw_data` entry
  in `validator-report.json` will need the restored file.

## Claims of the dispatch

| claim | status | recomputed |
|---|---|---|
| 2lp-sfb costs 0.683 of 2lp-new, pooled | verified; unit is FPM mid (F10) | 0.6828 [0.640, 0.729]; band units 0.846 (w = 0), 0.606 (native) |
| sfb IC/rho at 131101: 0.0002 (w = 0) | verified | 0.000192 [0.000151, 0.000247] |
| 0.77 [0.68, 0.88] (w = 0.1) | verified (F8 subsets: 0.746 to 0.781) | 0.7749 [0.6755, 0.8849] |
| 7.75 [6.75, 8.85] (w = 1) | verified | 7.747 [6.754, 8.846] |
| UNRESOLVED at every prime | verified, both arms, both interval methods, cap subsets | — |
| Gap closing at every w | verified | sfb upper bounds −0.779, −0.341, −0.341; new −0.754, −0.335, −0.335 |
| la_add is 85% of native ops | verified for sfb at 131101 | 0.852 |

## Not done

- No rebuild of the binary (not allowed on this host), so the binary-to-source binding is
  unverified (F2).
- The 14 `quartic_ladder` tests were not rerun; that needs cargo. The producer's log shows 14
  passed and 2 ignored.
- No pcc claim was created.
- Smaller primes were replayed as a sample: 7 of 35 curves. All 40 curves' certificates were
  checked.

## Reproduce (from the repository root; scratch `OUT` outside the repository)

```sh
V=research/quartic_diem_ladder_20261121/validation
python3 -I $V/verify_certificates.py . 6b5e2a7d668 $OUT/certificates.json --count-points
python3 -I $V/recompute.py . 6b5e2a7d668 $OUT/recompute.json
python3 -I $V/cap_subsets.py . 6b5e2a7d668 $OUT/cap_subsets.json
sh $V/replay.sh /path/to/genus_ladder.qsfb $OUT/replay
python3 -I $V/compare_replay.py . 6b5e2a7d668 $OUT/replay_compare.json $OUT/replay/*.jsonl
python3 -I $V/build_gate_artifacts.py . 6b5e2a7d668 $V   # reads the four outputs from $V
python3 scripts/validation_gate.py evidence $V
```
