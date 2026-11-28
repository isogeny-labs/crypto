# Protocol: sibling walk at k = 8, m = 8 + reuse of formed points against the lean+sib incumbent on K1 n37 (2026-11-18)

Frozen before any data of this study (hashes in `REGISTRATION.sha256`; the registration commit is named in
`registered-at.txt`, written after it). Class: **confirmatory engineering-lever study**, one curve (K1 a = 0, n = 37),
T0 scope. Separately registered from `research/koblitz_k1_sibwalk_20261118` (whose n37 test failed at w = 1 with the
pilot's k = 9); that study's n37 result stands as measured and is not replaced by this one. Every number is
`[pcc:unverified]` until a validator signs it.

## 0. What was seen before freezing (disclosed)

- `koblitz_k1_sibwalk_20261118` (code 217638524d, binary sha256 `7b182360…fd315`, validation pending): n37 candidate
  k = 9, m = 8 read sw/ls 0.785 [0.735, 0.839] / 0.874 [0.824, 0.926] / 1.050 [1.006, 1.095] at w = 0 / 0.1 / 1 (failed
  at w = 1). Changing k changes the walk (different `targets_tried`), and the pilot that chose k = 9 had 96 pairs.
- Its count model (`model.py`, copied unchanged from `koblitz_k1_sibwalk_20261118/prof/model.py`) reproduces exactly all
  432 n37 `ls` records and all 432 n37 `sw` (k = 9, m = 8) records, and the 432 n23 `sw` (k = 4, m = 8) records of that
  study (checked before this freeze). On that study's n37 `ls` records (seeds 20261118487-489, not reused here) it gives,
  for k = 8 (unchanged walk: same targets as ls), ladder_verify, orbit_key_reuse and walk_sib = m, the ratio of pooled
  means sw/ls at w = 0 / 0.1 / 1:

  | m | 1 | 4 | **8** | 10 | 12 | 16 | 24 |
  |:--|:--|:--|:--|:--|:--|:--|:--|
  | w = 0 | 0.808 | 0.749 | **0.736** | 0.734 | 0.733 | 0.732 | 0.733 |
  | w = 0.1 | 0.864 | 0.825 | **0.817** | 0.815 | 0.815 | 0.814 | 0.815 |
  | w = 1 | 0.976 | 0.976 | **0.977** | 0.977 | 0.977 | 0.978 | 0.978 |

- No session of this study's specs exists before this freeze. Code at HEAD is unchanged since 217638524d in `src/`,
  `Cargo.toml`, `Cargo.lock` (`git diff --stat 217638524d HEAD` empty for those paths); the binary
  `bins/ecbench-sw-217638524d` in the scratchpad has sha256
  `7b182360f531492214010854acb92ff1218a1f7fe29f5ac03cc8c3ac221fd315` and `strings` shows `walk_sib`, `ladder_verify`,
  `orbit_key_reuse` (102 matching lines). It is reused, not rebuilt.

## 1. Candidate rule (fixed before data): k = 8, m = 8, no pilot

`sw` = the incumbent `ls` (lean_layout + sib on sp-c8-d5) with `walk_sib=8`, `ladder_verify=1` (oracle) and
`orbit_key_reuse=1` (base). Why fixed m = 8 and no fresh pilot over m in {8, 12, 16}:

1. At k = 8 the candidate walks the same targets as `ls` in the same order, so `targets_tried` and the recovered log are
   those of `ls` and m changes only how the walk is grouped. The effect of m is then a deterministic function of the
   trial counts, which the count model reproduces exactly on every record checked (above). A pilot would measure that
   same deterministic function with noise from the target draw; it adds no information about m.
2. The model's m curve is flat beyond 8: going to 12 or 16 gains at most 0.004 at w = 0 and 0.003 at w = 0.1, and loses
   0.001 at w = 1, the binding weight of the rule. The extra set-up (m+1 operations) and block tail grow with m.
3. m = 8 is the value the previous pilot picked on both n23 and n37 and the value of the derived counterfactual named in
   the open thread; it was in the previous grid and needs no code beyond 217638524d (which accepts 0..=64).

## 2. Arms (per session; identical arm list in all 3 sessions)

| arm | definition | role |
|:--|:--|:--|
| rho-strong | `rho.signed_frobenius_strong` | reference (leaderboard's current) |
| rho-v5 | `rho.signed_frobenius_v5` | reference (strongest measured) |
| ls | lean_layout + sib on sp-c8-d5 (`make_specs.py`) | incumbent |
| ls-aa | ls again | A/A control |
| ls-off | ls + `orbit_key_reuse=0` / `walk_sib=0,ladder_verify=0` | default-off control |
| **sw** | sp-c8-d5 + lean_layout + sib + `orbit_key_reuse=1` / `walk_sib=8,ladder_verify=1` | **candidate** |

Curve K1 a = 0, n = 37. Three replicate sessions, each 48 public targets, 3 rounds, 1 warm-up, order `alternate`.
Fresh target seeds (unused anywhere in the repository at freeze): r1 20261118491, r2 20261118492, r3 20261118493;
measurement seed = target seed + 1000. Sessions run r1, r2, r3 (`run.sh`).

Binary: sha256 `7b182360f531492214010854acb92ff1218a1f7fe29f5ac03cc8c3ac221fd315` (built from 217638524d for
`koblitz_k1_sibwalk_20261118`); `run.sh` refuses any other. Run under the CPU lock, `--cpus none --allow-busy` (L0),
launched through `tools/benchrec.py run`.

## 3. Pricing, statistics, decision

`ecbench verify --replay-all` on every session (0 problems required). `ecbench band --ranks` per session and pooled.
`ecbench compare --weight w --resamples 4000 --seed 20261002`, paired by target, per replicate and pooled. Upper bound
used = the larger of compare's two 95% upper ends (`ci95`, `ci95_bootstrap`). `analyze.py` and `summarize.py` are the
previous study's frozen scripts restricted to n37 (diff: the session directory and the curve list only).

- **Primary (one test):** sw / ls. **Improvement iff the upper 95% bound is < 1 at each of w = 0, 0.1, 1, pooled AND in
  each of the 3 replicates.** Otherwise reported per weight and replicate as measured; no pass.
- **Gap (never a gate):** sw / rho-v5 and sw / rho-strong beside ls / rho-v5 and ls / rho-strong, pooled, at the three
  weights, with the band order (§5a: ranked only where the order holds at all three weights).
- **Controls (void the study if they fail):** ls-off and ls-aa identical to ls on every record (cost, phase counts and
  native maps without wall time, counters, outcome, `points_sha256`; parameter echoes excluded); every solve `verified`
  with `matches_target`; sw does the same `targets_tried` and recovers the same log as ls on every (replicate, round,
  target).
- **Secondary (descriptive):** `model_check.py` (model vs every ls and sw record; model ratio from the ls records vs
  measured), phase split, native counts, wall times (L0).

## 4. Model expectation (not a measurement)

From the table in section 0 (other seeds): about 0.74 / 0.82 / 0.98, with tight intervals because the walk is
unchanged (the n23 analogue was 0.963-0.968 at w = 1). Expected to pass, with the w = 1 margin about 0.02.

## 5. Deviations declared in advance

- No EKS/Iso Lab, no AWS/S3: local L0 run; raw sessions gzipped in the scratchpad (`sw8-sessions/`), SHA-256 of each
  uncompressed `records.jsonl` in `summary.json`; compact tables in `tables/`.
- No conductor and no `research-kb` server in this container; no novelty claimed.
- Registration is a local commit before data (not pushed).
- rho-v5 leaves its per-step jump-index selection untallied (koblitz_rho_reference_20261115 section 1).
- Report figures and README are written after data (`make_report.py`).
