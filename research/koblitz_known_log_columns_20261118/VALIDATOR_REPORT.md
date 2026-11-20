# Validator report VAL-20261118-4ae594: known-log seed columns on the small Koblitz rows

Independent validation of `research/koblitz_known_log_columns_20261118/` on branch
`claude/f4-f5-algorithm-speed-bvp7qd`, committed snapshot `7673d7fe76`. The validator did not
produce, design or run any part of the study. It read the frozen protocol, the committed tables, the
raw sessions named by the README and the source of the two new options. It did not edit any
producer file. Replay nonce `4107701254252828946`, drawn 2026-11-18T07:08:57Z, after the results
commit (07:04:08Z).

## Verdicts

| primary (frozen) | claim | recomputed, w = 0 / 0.1 / 1 (95% CI) | verdict |
|:--|:--|:--|:--|
| **A1-P1 n23**: sp-c4-d5 / gen-c4, K1 a=1 n=23 | 0.588 [0.505, 0.683] / 0.605 [0.528, 0.694] / 0.645 [0.579, 0.718] | identical (rel. diff. below 1e-14) | **passed** |
| **A1-P1 n37**: sp-c8-d5 / gen-c8, K1 a=0 n=37 | 0.381 [0.330, 0.440] / 0.398 [0.347, 0.456] / 0.442 [0.392, 0.498] | identical (rel. diff. below 1e-14) | **passed** |
| A0-P1 n23: kl-c3 / gen-c4 (first attempt) | 1.109 [0.867, 1.417] at w = 0, null | identical | failed (null), as the producer reports |
| A0-P1 n37: kl-c7 / gen-c8 (first attempt) | 0.494 [0.392, 0.623] / 0.504 / 0.529 [0.436, 0.642] | identical | passed |

"Passed" means the receipt is admissible evidence for a **T0-scope engineering gain** of the
named, validated generator-orbit IC arm, at these two curves and only these. It is not an ECDLP
claim, not a claim below rho, not evidence for a smaller exponent, and it does not authorize a
leaderboard row by itself (see "Gate" below).

## 1. Is the method a fully charged, single-target, cold DLP solver?

Yes. Every check below is on the raw records, not the producer's summary.

- **Seed columns are built inside each run and charged.** In every one of the 192 sp-c4-d5
  records the factor-base phase holds exactly 15 doublings, no additions and no scalar
  multiplications (5 doublings for each of the 3 seeds after G), 18.59 GAE. sp-c8-d5 holds 35
  doublings (5 x 7), 42.89 GAE. kl-c4 holds 1 doubling and 2 additions, as `[1]G..[4]G` needs.
  Each seed's Frobenius orbit is closed and checked, its Frobenius maps are counted
  (`frobenius_maps`) and priced through the pinned calibration, and its curve-equation checks and
  negations are listed as unpriced counters that the band charges. Source:
  `plugins.rs` at 1b8e9e5cde and 41783eef88; the seeds use `g.double(ops, ..)` and
  `g.add(ops, ..)` on the charged counter.
- **Nothing is reused across targets.** Each record is one target, run in its own process
  (`process_wall_ns` and `max_rss_kib` on every record). The factor-base phase and the pair-table
  set-up (`oracle_setup`, 372 additions at n23 and 2376 at n37) are charged in full on every
  record of every IC arm (192/192 for each arm). There is no cache read: a cached base would show
  a zero-cost factor-base phase.
- **The known logs use no target information.** They are `h * 2^(d(j-1)) mod r` (A1) or
  `h * j` (A0), logarithms of multiples of G that hold by construction. The only shared constant is
  the Frobenius eigenvalue `lambda`, a public curve constant that the incumbent and both rho arms
  also use.
- **Every solve is verified.**
  - All 7,296 records (A0: 1,536 + 1,152 + 1,536; A1: 1,536 + 1,536) are `verified` with
    `matches_target = true`.
  - The final `[d]G = Q` costs one charged scalar multiplication in the `verify` phase of every
    IC record.
  - Independently of ecbench: the validator's own GF(2^n) Koblitz arithmetic (`checks.py`)
    recovered the field polynomials from the points: x^23+x^5+1 for n23,
    x^37+x^6+x^4+x+1 for n37, and x^29+x^2+1 for n29.
  - The same arithmetic confirmed `[r]G = O` on each curve. It then confirmed `[d]G = Q` on
    736 records drawn with the nonce across every arm of all five sessions: 24 per arm in A1 and
    16 per arm in A0. 736 of 736 verified.
- **Memory.** Mean `max_rss_kib` is 17.0 MB for the candidate and the incumbent alike, and
  15.4 MB for rho. The candidate's pair table is no larger than the incumbent's: 350 entries
  against 356 at n23, 2326 against 2344 at n37. No time-memory trade is hidden.

**Relabelling or improvement?** By the definitions in AGENTS.md "Success criterion" this is
**not** a relabelling and **not** an accounting change:

- the algorithm changed (different base points; one relation instead of 3.48 at n23 and 5.98
  at n37);
- the boundary, the unit, the targets, the oracle, the verify step and the pricing are the same;
- the total cold cost fell at all three band weights;
- no cost moved out of the budget (§6).

It is a legitimate measured engineering improvement of the named baseline.

The validator agrees with the producer's own description: with every column of known logarithm,
the pipeline is a Frobenius- and negation-folded baby-step giant-step. The pair sums of the known
seeds are the baby steps, and `Q + [t]G` are the giant steps. The one "relation" is a table
collision, not the decomposition of a random point. Three consequences follow, and every place
that shows this result must state them:

1. It is evidence about the cost of the IC harness's configured arm at these sizes. It is not
   evidence about index calculus. It counts at T0 only.
2. Its asymptotic cost is about `2 sqrt(r/2n)` against `sqrt(pi r/4n)` for a folded rho, about
   1.6x above. The validator re-derived this ratio. The measured gap to rho-v5 at n37, w = 0, is
   already 1.77, close to that ceiling: the lever has little headroom left against a matched
   folded rho.
3. If this is ratcheted as the new IC baseline on these rows, the row must be labelled as a
   generic (BSGS-type, all-known-log) base. Otherwise later "IC" gains are measured against a
   generic method under the IC name without saying so.

## 2. Recomputation

Two independent paths, both from the raw `records.jsonl` (copied to validator scratch; digests
re-checked):

- The pinned binary: `ecbench compare --dir <session> --a <A> --b <B> --weight {0,0.1,1}
  --resamples 4000 --seed 20261002 --json`.
- The validator's own Python (`recompute.py`, sha256 `3382e7fd…a2c8`). It shares no code with
  `analyze.py`, `summarize.py` or ecbench.
  - It implements the §5a band from its written definition: `priced_gae(w) = G - C +
    w(U + n_C)`, with the table work priced.
  - It takes the ratio of totals paired by (workload, round), with targets as clusters.
  - It computes the linearised t interval, a percentile cluster bootstrap and a 98.75% interval
    for multiplicity.

Both reproduce every ratio and every 95% interval in `summary.json` and `a1/summary.json`: 39 A0
comparisons and all overlapping A1 comparisons, maximum relative difference 5e-15. The band
means also match `a1/tables/band-n*.txt` (for example, n23 gen-c4 0.735 / 0.954 / 2.921 and
sp-c4-d5 0.432 / 0.577 / 1.884). The phase split in the README is reproduced as well (n23:
19 / 398 / 505 / 30 = 951 against 169 / 398 / 1013 / 30 = 1610; n37: 43 / 2538 / 1744 / 39 = 4363
against 720 / 2538 / 8124 / 39 = 11421).

### Distance to rho (recomputed; paired, w = 0 / 0.1 / 1, 95% CI)

| curve | arm | / rho.signed_frobenius_strong (leaderboard reference) | / rho.signed_frobenius_v5 (strongest measured) | band order vs v5 |
|:--|:--|:--|:--|:--|
| n23 | gen-c4 (before) | 0.828 [0.751, 0.912] / 1.000 [0.913, 1.096] / 1.895 [1.742, 2.062] | 3.131 [2.788, 3.516] / 3.662 [3.277, 4.093] / 5.953 [5.375, 6.593] | above at every weight |
| n23 | sp-c4-d5 (after) | 0.487 [0.424, 0.559] / 0.605 [0.534, 0.687] / 1.222 [1.100, 1.357] | 1.840 [1.564, 2.165] / 2.216 [1.906, 2.577] / 3.839 [3.373, 4.369] | above at every weight |
| n37 | gen-c8 (before) | 2.646 [2.333, 3.002] / 2.910 [2.570, 3.294] / 3.935 [3.482, 4.447] | 4.650 [4.115, 5.255] / 5.247 [4.656, 5.912] / 7.908 [7.067, 8.848] | above at every weight |
| n37 | sp-c8-d5 (after) | 1.008 [0.901, 1.129] / 1.158 [1.039, 1.289] / 1.740 [1.571, 1.926] | 1.772 [1.560, 2.012] / 2.088 [1.848, 2.358] / 3.496 [3.131, 3.903] | above at every weight |
| n29 | gen-c1 (unchanged, A0) | 0.094 / 0.149 / 0.546 | 1.260 / 1.883 / 4.918 | above at every weight |

The honest reading:

- The candidate remains **above the strongest measured rho (v5) on both curves at every band
  weight**: 1.8x to 3.8x.
- Against rho-strong, n23 is unresolved inside the band, below at w = 0 and 0.1 and above at
  w = 1. n37 is at parity at w = 0 (the CI contains 1) and above at w = 0.1 and 1.
- rho-strong is itself 3.8x (n23) and 1.8x (n37) dearer than v5 at w = 0, mostly through its
  priced 32-scalar-multiplication set-up. So "below rho-strong at n23" must never be quoted as
  "below rho".
- v5 leaves its per-step jump-index selection untallied, so the v5 distances are slightly
  pessimistic for IC at w > 0. This was disclosed in advance (PROTOCOL §4).

## 3. Replay and digests

- **Digests.** All five `records.jsonl` hash to the `records_sha256` in `summary.json`,
  `a1/summary.json` and each `session.json`: A0 n23 `22fa0586…`, n29 `15431839…`, n37
  `d789cd18…`; A1 n23 `5f869ba3…`, n37 `488f758b…`.
  - Each session's `spec.json` is byte-identical to the frozen `specs/n*.json` and
    `a1/specs/n*.json`.
  - The pinned binaries hash to `c95535f4…48330` (A0) and `58cfff90…07f26` (A1), as
    recorded in every `session.json`.
- **Replays.** All replays ran on validator copies of the sessions.
  - `verify --replay-all` with each session's own binary reproduces all 5,472 measured runs
    identically (1,152 + 864 + 1,152 + 1,152 + 1,152), 0 problems.
  - A nonce-sampled `verify --replay 200 --replay-seed 4107701254252828946` on the A1 sessions
    reproduced 200 + 200 runs, 0 problems.
- **Cross-binary.** The A1 binary replays every measured A0 run identically: 3,168 runs, all
  arms, including the incumbent, the `-m1` controls, the progression arms `kl-c*` and both rhos.
  So the A1 binary leaves the incumbent and the progression byte-for-byte unchanged:
  `multiple_doublings` is off by default.
- **From source.** The validator rebuilt `ecbench` from HEAD (`cargo build --release --locked
  --bin ecbench`; `src/` is identical at `41783eef88` and HEAD). The resulting binary,
  `29dc9b14…07bcf`, is not byte-identical to the pinned `58cfff90…`: it is 128 bytes larger, and
  the cause was not identified. There is no build script, so the difference may come from the
  producer's build invocation or features. Functionally it reproduces **all 5,472 measured runs
  of all five sessions identically**: the committed source yields the recorded counts.
- **Link to the validated baseline.** The A1 binary also reproduces, identically, the 48 measured
  `ic-gen-layout-c8` runs of the validated study's `sessions/a0-n37` and 48 nonce-drawn
  `ic-gen` runs of its `sessions/a1-n29`. The incumbent here is the algorithm that study
  validated, and its counts have not drifted since.

## 4. Controls

- **Default-off identity (`-m1`).** gen-c4-m1 = gen-c4 (n23), gen-c1-m1 = gen-c1 (n29) and
  gen-c8-m1 = gen-c8 (n37) on **every** record, warm-up included (192/192 each). The comparison
  covers total GAE, every phase's counts and native counters (wall time excluded), the counters,
  the factor-base point digest, the algorithm seed and the recovered scalar. The ratios are
  exactly 1.000.
- **`multiple_doublings` default-off** (A1 has no `-m1`-style arm of its own). This is covered by
  the cross-binary replay above and by the off-state arm kl-c<c0> inside A1.
- **A/A.** gen/gen-aa are identical on every record in all five sessions. The two arms share
  algorithm seeds, so the A/A is a determinism check, not a noise estimate. The noise is in the
  target-clustered interval.
- **Predicted negative case.** At n29 the lever is inert (gen-c1 is already the `k = 1` base),
  and the frozen protocol predicted that kl-c2 / gen-c1 >= 1. Measured: 1.703 [1.508, 1.923] /
  1.806 / 1.934, as predicted.
- **Ratchet (A1-P2).** sp / kl, the off-state of the A1 parameter: 0.667 [0.568, 0.784] / 0.690 /
  0.741 at n23 and 0.621 [0.506, 0.762] / 0.639 / 0.684 at n37. The spread seeds, not only the
  known logs, carry the A1 gain.

## 5. Protocol order, multiplicity and forking paths

- **Order of freeze, data and code.**
  - `753f3fae10` (05:10:21Z) freezes PROTOCOL, specs, `run.sh`, `analyze.py` and
    `make_specs.py`. Their committed blobs hash to `REGISTRATION.sha256`, and all except
    PROTOCOL.md are unchanged at HEAD.
  - PROTOCOL.md changed only by the appended A1 addendum (26 insertions, 0 deletions).
  - The A0 sessions started at 05:23:49, 05:47:09 and 05:55:56Z, after the freeze. The
    `run-n*.time` lock waits (811 s, 1393 s) account for the gaps: the timed n23 command began
    at about 05:10:29Z, 8 s after the freeze commit.
  - `1c82dec2b0` (06:27:31Z) freezes A1, and its blobs hash to `a1/REGISTRATION.sha256`.
  - The A1 sessions started at 06:59:33 and 06:59:47Z. The 1926 s lock wait in
    `a1/logs/run-n23.time` accounts for the gap: the timed command began at about 06:27:37Z,
    6 s after the addendum commit.
  - Ancestry: `753f3fae10` < `4aaa117e3c` (A0 data) < `41783eef88` (A1 code) < `1c82dec2b0` <
    `7673d7fe76` (A1 data).
  - The only session directories created today are the five above, the disclosed A0 pilot
    (`kob/p23,p29,p37`: binary `b2275ffa…`, incumbent arms and rho-v5 only, 4-6 targets) and
    the disclosed A1 sizing pilot (`kob/a1pilot`, 12 targets, seed 20261118499).
  - The pilot means recompute to exactly the figures the addendum discloses (n23 gen-c4 1505,
    sp-c4-d5 932, sp-c4-d3 876, …; n37 gen-c8 10288, kl-c8 6278, sp-c8-d3 5118, …).
  - No undisclosed attempt was found.
- **Post-freeze `a1/summarize.py` fix.** It is logic-neutral. The diff splits one line so that
  `import gzip; _p = …; R = json.load(…)` is no longer inside a `#` comment. After the fix the
  file is byte-identical in logic to the A0 `summarize.py`. It only reduces `results.json` (the
  frozen `analyze.py` output), and the validator's recompute from raw data does not use it.
  (A0's `summarize.py` was not in the A0 registration; it was committed with the A0 data. It is
  a reduction script with no statistics of its own, and the recompute bypasses it.)
- **Multiplicity.**
  - The family of attempts on n23/n37 is four primary tests: A0 n23, A0 n37, A1 n23, A1 n37.
  - Bonferroni at 0.05/4, at least as strict as Holm's first step, gives 98.75% intervals. The A1
    upper ends are 0.714 / 0.722 / 0.740 (n23) and 0.459 / 0.475 / 0.515 (n37). The A0 n37 upper
    ends are 0.666 / 0.679 at w = 0 and 1. All are below 1 and below the protocol's 0.90 "not
    resolved" margin.
  - The primary arms were named before the data, and the primary was not switched to a better
    secondary arm (sp-c5-d5 at n23, 0.54x).
  - A1 uses fresh targets (seeds 20261118411 and 20261118413), disjoint from A0's (…401, …403)
    and from the pilot's (…499).

## 6. Unit tests

Run under `flock` on the shared CPU lock:

```
cargo test --release --locked --lib generator_multiples
cargo test --release --locked --lib multiple_doublings
```

Each runs one test, and each passes on a lib-test binary built from HEAD source (7619 tests in
the binary):

- `generator_multiples_seed_known_log_columns_and_default_is_unchanged`. It checks that the
  absent and `=1` settings give the same points, coefficients, native counters, operations and
  known logs. It checks the cost of `k` known columns (one doubling, `k - 2` additions, no scalar
  multiplication). It checks every base point's coefficient against `[coef * x]G = [h]P`, the
  rejected configurations, and end-to-end recovery of a planted log with one relation.
- `multiple_doublings_spread_the_known_log_seeds`. It checks `d(k-1)` doublings, the logs
  `h * 2^(d(j-1))`, every member's coefficient, the rejected configurations, and end-to-end
  recovery.

**Hazard found while testing.** The first attempt reused a `crypto_lib` test binary that a
concurrent validator had just built into the shared `target/` from its own worktree (at
`8aff81b841`, which has no `multiple_doublings`). Cargo's mtime fingerprint judged it fresh, so
`multiple_doublings` ran "0 tests". The validator forced a rebuild from this checkout by updating
the timestamps of `ic_framework/{mod,plugins}.rs`; the contents are unchanged and `git status` is
clean. It then confirmed that the binary contains the new test. A `target/` shared across
worktrees can silently test or run another tree's code. The producer reported the same trap for
`ecbench`.

## 7. Findings (none changes a verdict)

1. **Gate: the raw data is not in the repository.** The five sessions (9-13 MB each) exist only
   in the producer's scratchpad. They are not in git, and the benchrec raw objects were not
   uploaded to the store.
   - The in-repo `tables/records-n*.csv.gz` match the raw records row for row (0 mismatches).
     But they carry only `total_gae` and the recorded `S`, so they reproduce neither the band's
     w = 0 reading (0.591 from the CSV against 0.588) nor any w > 0 headline.
   - `docs/VALIDATION_GATE.md` requires recomputation "from raw data that is in the
     repository". A `VALIDATION.json` should therefore not be written until the five session
     trees are archived (AGENTS.md §13/§13a) under the digests above.
   - The scratchpad can be cleared at any time. This report records the digests that were
     verified at 07:08Z.
2. **Rho reference for the gate.** rho-v5 is the strongest rho measured on all three curves, and
   it ranks below rho-strong across the band. A `rho_reference` with `strongest_available: true`
   must name v5, or record the pending owner decision and report v5 beside strong, as the README
   does.
3. **n23 curve.** The validated generator-orbit study measured n23 on a=0 (`t5197`, log2 r 21.0).
   This study uses a=1 (`tm5197`, 22.0). On this curve the incumbent is the validated method
   (identical counts, section 3), measured in-session. gen-c4 is the better of the two
   configurations measured (gen-c3 / gen-c4 = 1.11 at w = 0, unresolved at w = 1). n37 a=0 and
   n29 a=1 are the validated study's own curves.
4. **Labels.**
   - The factor-base `description` string in the records says "orbits of [1]G..[4]G" for the
     `multiple_doublings` arms. The `params` carry the true seeds.
   - `summary.json` `mean_S` repeats the recorded S under every weight key.
   - The README phase-split table says "at w = 0" but shows the recorded GAE (calibrated natives
     at pinned rates), not the band's w = 0 reading.
   - None of these touches a ratio.
5. **Provenance.**
   - `session.json` carries `git_commit: null`.
   - The pinned A1 binary is not byte-reproducible from HEAD in this environment (`29dc9b14…`
     against `58cfff90…`). It is functionally reproducible: the HEAD build replays every measured
     run of all five sessions identically.
   - The A0 binary was built from `1b8e9e5cde` plus another lane's uncommitted test-only edit.
     The HEAD build and the A1 binary both reproduce A0's records.
   - `target/release/ecbench` is now the validator's HEAD build (`29dc9b14…`). The study's
     pinned copies in the scratchpad are untouched.
6. **Environment.** L0 shared local host, `--cpus none --allow-busy`, no Iso Lab/EKS or S3, as
   PROTOCOL §4 declares. The metric is a deterministic operation count (every replay identical),
   so isolation does not affect it. Wall time is not used.

## validation_report

```yaml
validation_report:
  id: VAL-20261118-4ae594
  task_id: null  # dispatched by the producing session; no TASK card was named
  snapshot: 7673d7fe766040b93cf3d89239e649d5d9bd236a
  run_ids:
    - ECBS1h84a1a4eb2173  # A1 n23, records 5f869ba3ae71f1724d160e30ce4701b6ad80613cb2cbd8b5657a7c19770eef65
    - A1 n37 session, records 488f758b116846957737e85b92865ea536ae28ecc872269dca76801e4a016d1e
    - A0 n23 records 22fa05865564646657fadb77a7fc8c233bfa2369125a1902eaf76d2d2fd58a25
    - A0 n29 records 154318391f4df4f0779f0bbd4259a4de89b069c85a414ce009e05ae03f1ec2ab
    - A0 n37 records d789cd183945c4d91c4e1a8081e47176ecc29515e835da6a1a9df5008a64237a
  artifact_checks:
    - records.jsonl sha256 = summary.json / a1/summary.json / session.json (5/5)
    - session spec.json byte-identical to frozen specs (5/5)
    - binary sha256 c95535f4...48330 (A0), 58cfff90...07f26 (A1) = session.json
    - REGISTRATION.sha256 and a1/REGISTRATION.sha256 = blobs at 753f3fae10 and 1c82dec2b0
    - git ancestry freeze < data for A0 and A1; session start times after each freeze
    - committed records-n*.csv.gz = raw records (0 mismatches)
  metric_recomputations:
    - A1 n23 sp-c4-d5/gen-c4 w=0/0.1/1: 0.5877 [0.5053,0.6834] / 0.6052 [0.5278,0.6939] / 0.6448 [0.5794,0.7177]
    - A1 n37 sp-c8-d5/gen-c8 w=0/0.1/1: 0.3810 [0.3298,0.4401] / 0.3979 [0.3470,0.4562] / 0.4420 [0.3924,0.4979]
    - gaps to rho-strong and rho-v5 (section 2 table); A0 primaries and n29 secondary
    - method: ecbench compare (pinned binary) and independent Python; agreement < 1e-14
  control_checks:
    - default-off -m1 identity on every record (n23, n29, n37): pass
    - multiple_doublings default-off via cross-binary replay of all A0 runs: pass
    - A/A identity: pass (determinism check; shared algorithm seeds)
    - n29 predicted negative case kl-c2/gen-c1 >= 1: observed 1.70
    - every solve verified; 736 nonce-sampled scalars re-verified with independent arithmetic
    - replay-all 5/5 sessions identical with pinned binaries; nonce-sampled replays identical
    - replay-all 5/5 sessions identical with a binary rebuilt by the validator from HEAD source (29dc9b14...)
    - unit tests generator_multiples and multiple_doublings pass on a lib-test build from HEAD source
  heuristic_validation_checks:
    - pre-registered predictions (PROTOCOL section 1, addendum A1) recorded before data; A0 n23 prediction failed and is reported
    - no correlation/bias claim; the measured quantity is a paired verified cost ratio, the null object is the A/A and the -m1 arm
    - no extrapolation claimed; BSGS-type asymptotic ceiling (~1.6x folded rho) stated and re-derived
  cost_model_checks:
    - unit ecbench.gae; S = GAE / sqrt(r); band w = 0, 0.1, 1 with table work priced; no uncounted counter
    - seed-column doublings, Frobenius maps, pair table, relations, linear algebra, verify charged per record; no cross-target reuse
    - memory reported (max RSS equal across IC arms; table entries not larger)
  proof_architecture_checks: []
  verdict:
    A1-P1 n23: passed
    A1-P1 n37: passed
    A0-P1 n23: failed (null, as reported)
    A0-P1 n37: passed
  classification: T0 engineering gain of the named validated IC arm; generic (folded BSGS-type) base; not relabelling, not accounting
  limitations:
    - raw sessions only in the producer scratchpad; gate needs them archived before VALIDATION.json
    - leaderboard reference rho-strong is not the strongest measured rho; v5 is binding for distance
    - pinned A1 binary not byte-reproducible from HEAD (functionally reproducible: identical replay of every run)
    - shared target/ across worktrees can serve another tree's build as fresh (seen in this validation; worked around)
    - L0 shared host, local; counts deterministic
  artifact_paths:
    - research/koblitz_known_log_columns_20261118/PROTOCOL.md
    - research/koblitz_known_log_columns_20261118/summary.json
    - research/koblitz_known_log_columns_20261118/a1/summary.json
    - research/koblitz_known_log_columns_20261118/a1/tables/band-n23.txt
    - research/koblitz_known_log_columns_20261118/a1/tables/band-n37.txt
    - research/ecbench_generator_orbit_20261022/sessions/a0-n37
    - research/ecbench_generator_orbit_20261022/sessions/a1-n29
  review_attestation:
    joints_owned: [whole receipt, as tasked]
    read_sibling_reports: false  # research/ic_ksum_split_20261118/VALIDATOR_REPORT.md (other study) not opened
    sources_read:
      - research/koblitz_known_log_columns_20261118/{README.md,PROTOCOL.md,REGISTRATION.sha256,registered-at.txt,run.sh,summarize.py,summary.json}
      - research/koblitz_known_log_columns_20261118/a1/{REGISTRATION.sha256,run.sh,make_specs.py,analyze.py,summarize.py,summary.json,specs/n23.json,tables/band-n23.txt,tables/records-n*.csv.gz}
      - research/koblitz_known_log_columns_20261118/tables/records-n*.csv.gz
      - git show 1b8e9e5cde, 41783eef88 (plugins.rs, mod.rs tests); git diff 1c82dec2b0..7673d7fe76 a1/summarize.py
      - src/cryptanalysis/ecbench/{band.rs,compare.rs,stats.rs} (pricing and interval definitions), methods.rs PRICED_NATIVE
      - docs/ic/calibration.json; docs/VALIDATION_GATE.md; docs/ic/SUCCESS_CRITERIA.md sections 2-3; docs/ic/LEADERBOARD.md (gen-orbit rows)
      - AGENTS.md: Success criterion, section 5a, section 6, IC measurement rules
      - research/ecbench_generator_orbit_20261022/README.md (head), sessions a0-n37 and a1-n29
      - docs/session-logs/2026-11-18.md (producer entry)
      - scratchpad sessions klc-sessions/, klc-sessions-a1/, kob/ pilots (copies for replay)
```
