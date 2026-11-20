# Protocol: sibling giant-step walk + reuse of formed points against the lean+sib incumbent on K1 n23, n29, n37 (2026-11-18)

Frozen before any confirmatory data (hashes in `REGISTRATION.sha256`; the registration commit is named in
`registered-at.txt`, written after it). Class: **confirmatory engineering-lever study** on the Koblitz IC arms, T0 scope.
Every number is `[pcc:unverified]` until a validator signs it.

## 0. What was seen before freezing (disclosed)

- `research/koblitz_lean_sib_confirm_20261118` (validation pending): incumbents `ls` = lean_layout + sib on sp-c4-d5 (n23),
  gen-c1 (n29), sp-c8-d5 (n37). Its raw sessions were profiled per phase at w = 0 / 0.1 / 1 (band = group ops + w x
  native ops); a count model reproduces all 1,296 `ls` records exactly (`prof/model.py`). Mean per solve, w = 0 / 1:
  n23 table 186 / 1104, relations 390 / 1170, verify 30 / 30, base 15 / 291; n29 table 15 / 87, relations 31 / 93,
  verify 19.6, base 0 / 59; n37 table 1188 / 7104, relations 1554 / 4661, verify 38, base 35 / 923.
- Levers chosen from that profile (code commit 217638524d, all default off, unit test
  `k1_sibling_walk_ladder_verify_and_key_reuse_keep_the_solve`):
  1. `walk_sib=m` (oracle): the stride-generator targets Q + [k]G, same order, formed in blocks of 2m+1 around a centre;
     C + [i]G full and x(C - [i]G) from one inversion, charged one addition + one `sibling_second_points` (priced at w);
     x-only probe = one counted canonicalisation + lookup; a hit on an abscissa is formed by one more addition and
     probed again in full (counted). Set-up m+1 operations. Expected: about (m+1)/(2m+1) additions per target at w = 0,
     neutral at w = 1 apart from the set-up, a lift and the block tail.
  2. Reuse of points already formed: `ladder_verify=1` (oracle) reads the chain [2^i]G the base formed for its seeds
     (`ladder_mul`: popcount(d)-1 additions, doublings only above the chain); `orbit_key_reuse=1` (base) takes each seed's
     orbit key from its formed Frobenius images instead of n-1 squarings (k > 1 only). Both only where k > 1.
- Not used: `tau_verify` (count relabel per VAL-20261118-43658a), lazy negations (would change the base's point vector).
- Pilot (separate seeds 20261118471-473, `pilot/`, rule `pilot/SELECTION_RULE.md` written before pilot data):
  one session per curve, 48 targets x 2 rounds, `verify --replay-all` 0 problems. n23: picked c4-m8 (all three point ratios < 1; smallest mean); its point ratios to ls 0.701 / 0.785 / 0.966; n29: picked c1-m1 (no arm below 1 at all three weights; smallest maximum); its point ratios to ls 0.909 / 0.942 / 1.015; n37: picked c9-m8 (all three point ratios < 1; smallest mean); its point ratios to ls 0.676 / 0.760 / 0.930. Raw pilot records (scratchpad `sw-pilot/`) SHA-256: n23 4efc35a7…e1a1, n29 f60b7381…4bf0, n37 22b5d567…46e. Full table `pilot/selection.json`. The pilot shows k changes are noisy at 96 pairs (n37 k=9 read 0.930 at w=1 in the pilot, while the count model on the A1 sessions' trial counts, other seeds, put k=9 at about 1.04): the rule is applied as written anyway.
- No session of this study's confirmatory specs exists before this freeze.

## 1. Arms (per session; identical arm list in all 3 sessions of a curve)

| arm | definition | role |
|:--|:--|:--|
| rho-strong | `rho.signed_frobenius_strong` | reference (leaderboard's current) |
| rho-v5 | `rho.signed_frobenius_v5` | reference (strongest measured) |
| ls | lean_layout + sib on sp-c4-d5 / gen-c1 / sp-c8-d5 (`make_specs.py`) | incumbent |
| ls-aa | ls again | A/A control |
| ls-off | ls + `orbit_key_reuse=0` / `walk_sib=0,ladder_verify=0` | default-off control |
| **sw** | n23 k=4, `walk_sib=8`, `ladder_verify=1`, `orbit_key_reuse=1`; n29 k=1 (gen-c1), `walk_sib=1`; n37 **k=9** (sp-c9-d5), `walk_sib=8`, `ladder_verify=1`, `orbit_key_reuse=1` (all with lean_layout=1, sib=1; `make_specs.py` RULE) | **candidate** |

Curves K1 a=1 n=23, a=1 n=29, a=0 n=37. Three replicate sessions per curve, each 48 public targets, 3 rounds, 1 warm-up,
order `alternate`. Fresh target seeds (unused at freeze): n23 20261118481/482/483, n29 20261118484/485/486, n37
20261118487/488/489; measurement seed = target seed + 1000. Session order interleaved (`run.sh`).

Binary: ecbench built from 217638524d in a clean sparse scratch worktree (own CARGO_TARGET_DIR, CARGO_BUILD_JOBS=1,
`--locked`), sha256 `7b182360f531492214010854acb92ff1218a1f7fe29f5ac03cc8c3ac221fd315` (`strings` shows `walk_sib`, `ladder_verify`, `orbit_key_reuse`, `walk_sibling_lifts`; target dir deleted after); `run.sh` refuses any other. Run under the CPU lock, `--cpus none --allow-busy` (L0),
launched through `tools/benchrec.py run`.

## 2. Model expectation (not a measurement)

From the pilot and the count model (expected, not measured): n23 sw/ls about 0.70 / 0.79 / 0.97 (same walk: tight
intervals); n29 about 0.91 / 0.94 / **1.015**, i.e. **expected to fail the rule at w = 1** (the sibling walk is neutral at
w = 1 by its pricing, its set-up and the lifted hit cost a few operations, and n29 has no doubling chain or orbit key to
reuse); n37 (k changes 8 -> 9, so targets differ and the intervals are wider) about 0.68 / 0.76 between 0.93 and 1.04 at
w = 1: **the w = 1 verdict at n37 is uncertain**.

## 3. Pricing, statistics, decision

`ecbench verify --replay-all` on every session (0 problems required). `ecbench band --ranks` per session and pooled.
`ecbench compare --weight w --resamples 4000 --seed 20261002`, paired by target, per replicate and pooled (pairs within
a session). Upper bound used = the larger of compare's two 95% upper ends (`ci95`, `ci95_bootstrap`).

- **Primary (one test per curve):** sw / ls. **Improvement on curve n iff the upper 95% bound is < 1 at each of
  w = 0, 0.1, 1, pooled AND in each of the 3 replicates.** Otherwise reported per weight and replicate as measured; the
  curve does not pass. Separate verdicts per curve.
- **Gap (never a gate):** sw / rho-v5 and sw / rho-strong beside ls / rho-v5 and ls / rho-strong, pooled, at the three
  weights, with the band order (§5a: ranked only where the order holds at all three weights).
- **Controls (void the curve if they fail):** ls-off and ls-aa identical to ls on every record (cost, phase counts and
  native maps without wall time, counters, outcome, `points_sha256`; parameter echoes excluded); every solve `verified`
  with `matches_target`; where sw keeps ls's k, sw does the same `targets_tried` and recovers the same log as ls on
  every (replicate, round, target).
- **Secondary (descriptive):** phase split, native counts, wall times (L0).

Analysis: `analyze.py` (ecbench calls), `summarize.py` (reduction and rule), `make_report.py` (figures and PDF, written
after data).

## 4. Deviations declared in advance

- No EKS/Iso Lab, no AWS/S3 here: local L0 run; raw sessions gzipped in the scratchpad, SHA-256 in `summary.json`.
- No conductor and no `research-kb` server in this container; no novelty claimed (sibling points and stored-ladder
  checks are textbook and already used here by `wwalk`/`ksum`/rho v5).
- Registration is a local commit before data (not pushed).
- rho-v5 leaves its per-step jump-index selection untallied (koblitz_rho_reference_20261115 section 1).
- The container restarted during the first release build (no data existed yet); the build was restarted from the same
  commit in the same clean worktree under the CPU lock (`wt-sw-build.interrupted.log` kept in the scratchpad).
- The pilot ran under the same benchmark record as the confirmatory run (`benchmarks/records/20261118T1917Z-koblitz-k1-sibwalk.json`).
- The pilot grid's largest m (8) was picked on n23 and n37; m > 8 was not piloted.
