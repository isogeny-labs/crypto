# Validation report: genus ladder follow-up 1 (`r` bound to `p`, genus 4 and plane quartics)

Validator: independent agent session (Claude, validator role), 2026-11-15, 00:31-01:10 UTC.
Scope: the dispatch asked for provenance, an independent build, a partial replay
(p = 2053, 4099), a field-by-field comparison, a certificate review, an independent
recomputation of the per-cell means and ratios, and a verdict under the AGENTS.md gate.
Every number below comes from the validator's own commands; the producer's
`analysis.json` was read only after the validator's table existed (section 6).
Nothing in this directory other than this file was written or edited.

## 0. Artifact set validated (binding)

| item | value |
|---|---|
| raw data | `bound_r.jsonl`, 134 lines, SHA-256 `083ccc7184fde1db665b0a44cdd25b07b765bf26f42d494f61d028583f683e53` (matches `data.sha256`; `sha256sum -c data.sha256`: OK) |
| producer analysis | `analysis.json`, SHA-256 `2580b8e37c12308a67b18c5099677a443d46b8b4a1eb7386901bd6ff22ccbfd4` (matches `data.sha256`) |
| protocol | `PROTOCOL.md`, SHA-256 `9ed38dcc6eb065fb5b6bd43a2190af529791c52ebc0ab42f9485f453deb1e6d9` (matches `frozen.sha256`; identical to the copy committed in `3caef5cfc1`) |
| producer binary | SHA-256 `61ccabd2e84b33ef78c18a5d849d571f5b4d2e888d4b880e8596050ddd0b0f7a` (matches `binary.sha256`; the file at the recorded scratchpad path still exists and hashes to that value; 6 072 176 bytes) |
| commit holding the data | `eb142ccda3` (2026-11-15 00:33:22 UTC) added `bound_r.jsonl`, `data.sha256`, `analysis.json`, `RESULTS.md`, `tables.md`, `render.py`, figures; `git show` of those blobs at HEAD `2eca0deea0` gives the hashes above, and the working copies equal HEAD |
| repository HEAD | `e5e38c533efc9f67f5576d504dc4e7491ef4e4f7` when validation started (00:31 UTC; data files then untracked) and `2eca0deea0beb2f647721a5b9480557f7e3aad1a` when it ended (branch `claude/ecstatic-edison-cgo57l`; a merge of `origin/main` landed meanwhile) |
| validator binary | SHA-256 `29a7ca875b6abaeac10792596fbb0ec615471d3a9bc5d1e256d983aeeac4ff87`, 6 078 968 bytes, built at `e5e38c53` by `cargo build --release --bin genus_ladder` (7 min 48 s, rustc 1.97.0, cargo 1.97.0) |
| validator replay data | `/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/validator_replay.jsonl`, 90 lines, SHA-256 `0c1418f65a50ae756d2a0abd9f7d2bde21e94ba40b52bc43472bb3909a963550` (scratchpad, not durable) |

**Tree state.** The working tree moved during validation (HEAD advanced by a merge of
`origin/main`), so this report binds to content hashes, not to a tree: the data files
validated here are byte-identical to the blobs committed in `eb142ccda3` and present at
`2eca0deea0`. `run.log` is 0 bytes in both the study directory and the producer's
scratchpad: no stdout/stderr of the producer run was kept.

## 1. Provenance — PASS, with notes

- `git diff 14ab74ec8b e5e38c53 --stat -- src/cryptanalysis/genus_ladder.rs
  src/cryptanalysis/quartic_ladder.rs src/bin/genus_ladder.rs
  src/cryptanalysis/chained_transfer/ic_control.rs src/cryptanalysis/plane_quartic_pic.rs
  src/cryptanalysis/ic_boundary.rs` is **empty** (not merely formatting-only; also empty
  with `-w`), so the validator binary was built from ladder sources byte-identical to
  the attributed commit. `git diff 959ea31976 14ab74ec8b --stat` on the same files is
  also empty. `14ab74ec8b` is an ancestor of HEAD. Between `14ab74ec8b` and `e5e38c53`
  only `src/bin/smith_near_prime.rs`, `src/cryptanalysis/mod.rs` (+1 line) and the new
  `src/cryptanalysis/smith_trigonal.rs` changed, which is why the two binaries differ in
  hash and size. At the final HEAD `2eca0deea0` the same diff shows one unrelated line
  in `ic_boundary.rs` (`TargetSource::WalkPrimeLanes { lanes: u8 }` -> `u16`, from
  `aeeb30b883` on main); the other five files are unchanged.
- Config line of `bound_r.jsonl` vs PROTOCOL.md section 2: every flag in the protocol
  command matches (`primes [2053,4099,8209]`, `genera [4]`, `curves 3`, `seeds 2`,
  `arms acR`, `cc 2.0`, `qc 3.0`, `max_h 2`, `threads 4`, `rho_cap 1.2e8`,
  `seed_search 4000`, `quartic true`). The remaining config fields (`max_trials
  50000000`, `restart 4096`, `smith false`, `smith_census 0`) are the binary's
  defaults (`src/bin/genus_ladder.rs` lines 59-76 and 190).
- Timeline: `PROTOCOL.md` content fixed at 23:18:28 UTC (file mtime; hash equals
  `frozen.sha256`); producer run started at about 23:19:53 UTC (JSONL mtime 00:30:39
  minus `done.wall_s` 4246.5 s); the commit `3caef5cfc1` that froze the protocol landed
  at 23:33:49 UTC, i.e. during the run. "Frozen before data" holds at file level, not
  at commit level.
- The producer's `target/release/genus_ladder` (hash `61ccabd2…`) had mtime 2026-11-14
  22:02 UTC, 53 minutes before commit `14ab74ec8b` (22:55:40 UTC) exists. The binary
  predates the commit it is attributed to; the attribution is supported by the replay
  in section 4 (identical counted fields from a binary built from byte-identical ladder
  sources), not by a build record.

## 2. Independent build — PASS

`cargo build --release --bin genus_ladder` at `e5e38c53` recompiled `crypto v0.1.0`
(not a fresh-fingerprint no-op), finished in 7 min 48 s, exit 0. Binary SHA-256
`29a7ca875b6abaeac…` (full value in section 0). The hash differs from the producer's
as expected (section 1); the comparison that matters is section 4.

## 3. Replay — DONE for p = 2053 and 4099 (full dispatch command, no fallback needed)

Command (validator binary, run from `/home/user/crypto`, 00:39:45-01:01:42 UTC,
`done.wall_s` = 1317.3 s):

```sh
./target/release/genus_ladder run --primes 2053,4099 --genera 4 --curves 3 --seeds 2 \
    --arms acR --cc 2.0 --qc 3.0 --max-h 2 --threads 4 --rho-cap 1.2e8 \
    --seed-search 4000 --quartic --out <scratchpad>/validator_replay.jsonl
```

Replay output: 1 config, 12 instance, 4 cell, 72 run, 0 skip, 1 done lines. Search
walls: quartic 2053 32.9 s (46 tried), quartic 4099 80.8 s (59 tried), hyperelliptic
2053 110.3 s (109 tried), hyperelliptic 4099 1183.8 s (418 tried); the `tried` and
`rejects` counts equal the producer's cell lines. The p = 8209 cells (3 + 3 instances,
36 runs; producer search wall 3744 s for the genus-4 cell) were **not replayed**, by the
dispatch's wall budget. The driver's `--curves 0` breaks the search loop immediately for
every cell (`src/bin/genus_ladder.rs` line 210), so a quartic-only 8209 invocation was
not available; it was also not needed since the full p = 2053/4099 command fit the budget.

## 4. Field-by-field comparison — PASS (0 mismatches)

Comparison script: `<scratchpad>/vscripts/compare.py` (self-test producer-vs-producer:
2628 fields, 0 mismatches). Keys: instances by (family, p, g, curve_seed); runs by
(family, p, g, curve_seed, seed, arm). Every scalar field present on either side is
compared except `wall_ns`.

| what | compared | mismatches |
|---|---|---|
| instance lines (f, order, h, r, n_points) | 12 of the producer's 18 | 0 |
| run lines | 72 of the producer's 108 (12 cells x 6) | 0 |
| scalar fields across those 72 runs | 1752 | 0 |

Fields compared (union over arms): arm, attempts, candidates, cantor_inv, cantor_mul,
certified, core_cols, core_rows, curve_seed, decomp_trials, exhausted, family, fb_cols,
g, group_ops, h, la_add, la_mul, la_wied_mul, native, ok, p, prefilter_rejects, r,
rel_add, rel_mul, relations, rels_by_lp, seed, setup_ops, splits, steps, trials,
verify_ops, walk_ops. Not every field exists on every arm (rho rows carry
group_ops/steps/cantor_*; hyperelliptic rho also walk_ops/native; the dispatch's field
list included names that do not occur on some arms). `wall_ns` ratio replay/producer:
median 1.022, range 0.833-1.904 (same shared host, informational only).

Producer instances not covered: (hyperelliptic, 8209, 4, cs 363/393/407) and
(quartic, 8209, 3, cs 10/16/41), and their 36 runs.

## 5. Certificates — PASS on the recorded flags; not re-derivable from the data

- Producer data: 108 of 108 run lines have `certified: true` and `ok: true`; 0 `skip`
  lines; 18 instances; no duplicate keys. Replay: 72 of 72 `certified` and `ok`.
- **The dispatch's "3 rho runs skipped by the frozen cap" is not supported by this
  artifact.** `bound_r.jsonl` contains 0 skip lines; `analysis.json` reports
  `skipped: []`, `runs_total: 108`, `runs_not_certified: 0`; the producer's own
  `RESULTS.md` (line 15) correctly says "no rho run hit the cap". The sentence
  originates in the first study (`research/genus_ladder_20261114/RESULTS.md` lines
  34-35: genus 5, p = 1031). Zero skips is consistent with the cap: the largest r at
  8209 gives sqrt(pi r/4) = 5.957e7 expected steps (cap 1.2e8) for the hyperelliptic
  rho and 6.63e5 (cap 1.2e6 = rho_cap/100, `src/bin/genus_ladder.rs` line 438) for the
  quartic rho.
- What the flags mean (code identical at `14ab74ec8b`, `e5e38c53` and `2eca0deea0` for
  these two files):
  - Hyperelliptic IC (`src/cryptanalysis/genus_ladder.rs`, `run_ic`, lines 1488-1498):
    `let chk = curve.mul(&mut ops, inst.g, t);` …
    `let ok = chk == inst.target && curve.mul_rtl(inst.g, t) == inst.target;` …
    `if ok { res.recovered = Some(t); res.certified = true; … }`.
    `curve.mul` is the counted left-to-right double-and-add of the `CountedGroup`
    default (`src/cryptanalysis/ic_boundary.rs` line 1514); `mul_rtl` is the separate
    right-to-left ladder (`genus_ladder.rs` line 601). `certified` is set only inside
    that branch.
  - Hyperelliptic rho (`run_rho`, lines 1558-1561): `let certified = rr.recovered
    .map(|d| inst.curve.mul_rtl(inst.g, d) == inst.target).unwrap_or(false);` — one
    multiplication, not two.
  - Quartic Diem (`src/cryptanalysis/quartic_ladder.rs`, `run_diem`, lines 1231-1234):
    `let chk = cmul(&pq, &mut ops, &g, t); if chk == tgt { res.recovered = Some(t);
    res.certified = true; … }` — **one** counted left-to-right multiplication
    (`cmul`, line 922), not two; the dispatch's "twice" holds only for the
    hyperelliptic IC arm.
  - Quartic rho (`run_rho`, lines 1355-1358): `pq.mul_scalar(&inst.g.into(), t as
    u128) == PicClass::from(inst.target)`.
  - Planting: hyperelliptic `target = curve.mul_rtl(g, t_secret)` (`instance_from`,
    line 885); quartic `target = pq.mul_scalar(&g, t as u128)` (line 827). Each
    certificate's second (or only) path is thus the same routine that planted the
    target, and all paths share one Cantor / Picard group law. The certificate
    establishes "the recovered t satisfies [t]G = T under the implemented group law";
    it does not independently establish the group law.
  - Stronger, and in the data: the driver records `ok` as
    `o.recovered == Some(inst.t_secret)` for every arm (`src/bin/genus_ladder.rs`,
    rho line 326, IC line 362, quartic line 415, quartic rho line 446), a direct
    comparison with the planted secret. `ok: true` on 108/108 producer rows and
    72/72 replay rows.
- Limit: neither `t` nor `t_secret` is written to the JSONL, so [t]G = T cannot be
  re-verified from the data alone with solver-independent code. The validator did not
  add test code (dispatch rule). The evidence for the certificates is (a) the code
  reading above and (b) that an independently built binary reproduces `certified` and
  `ok` on every replayed row with identical counts.

## 6. Independent recomputation of the per-cell table — PASS (agrees with analysis.json)

From the producer's `bound_r.jsonl`, per (family, p, arm), n = 6 runs each:
mean of `group_ops`; mean of FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + g^2 la_mul
(fields absent on rho rows taken as 0); ratio = cell mean FPM_mid / rho cell mean FPM_mid.
Script: `<scratchpad>/vscripts/fpm.py`.

| family | p | arm | mean group_ops | mean FPM_mid | FPM_mid arm/rho |
|---|---:|---|---:|---:|---:|
| hyperelliptic | 2053 | A_full+pre | 26 728.00 | 1.821965e8 | 0.10056 |
| hyperelliptic | 2053 | C_double_lp+pre | 32 588.50 | 8.929224e7 | 0.04928 |
| hyperelliptic | 2053 | R_rho_neg | 3 915 042.17 | 1.811880e9 | 1 |
| hyperelliptic | 4099 | A_full+pre | 52 179.50 | 6.632544e8 | 0.10339 |
| hyperelliptic | 4099 | C_double_lp+pre | 73 799.83 | 2.404257e8 | 0.03748 |
| hyperelliptic | 4099 | R_rho_neg | 13 857 690.17 | 6.415335e9 | 1 |
| hyperelliptic | 8209 | A_full+pre | 103 293.50 | 2.643523e9 | 0.12246 |
| hyperelliptic | 8209 | C_double_lp+pre | 175 720.33 | 6.737942e8 | 0.03121 |
| hyperelliptic | 8209 | R_rho_neg | 46 626 683.33 | 2.158769e10 | 1 |
| quartic | 2053 | Q_diem_2lp | 454.83 | 7.032868e6 | 0.00888 |
| quartic | 2053 | Q_diem_full | 204.33 | 1.867617e8 | 0.23573 |
| quartic | 2053 | Q_rho_neg | 81 786.83 | 7.922785e8 | 1 |
| quartic | 4099 | Q_diem_2lp | 729.67 | 1.569891e7 | 0.00863 |
| quartic | 4099 | Q_diem_full | 192.50 | 7.116067e8 | 0.39124 |
| quartic | 4099 | Q_rho_neg | 187 473.00 | 1.818872e9 | 1 |
| quartic | 8209 | Q_diem_2lp | 1 687.33 | 6.132856e7 | 0.01299 |
| quartic | 8209 | Q_diem_full | 222.33 | 2.897259e9 | 0.61358 |
| quartic | 8209 | Q_rho_neg | 486 211.67 | 4.721898e9 | 1 |

Comparison with `analysis.json` (`cells[*].mean_group_ops`, `fpm_mid.mean`,
`fpm_mid_over_rho.ratio`; script `<scratchpad>/vscripts/cmp_analysis.py`): 18 cells,
48 comparable fields, **48 agree to 4 significant digits** (they agree to the 6 digits
printed). The 6 remaining fields are the rho cells' `fpm_mid_over_rho`, which the
producer leaves unset (`None`) and the validator reports as 1. The validator did not
recompute the bootstrap intervals, the GAE-band (w = 0 / 0.1 / 1) verdicts, the
exponent fits, or the B2-B4 verdicts.

Additional recomputations from the raw data (all pass): every instance has
`order == h * r`, `r` prime (Miller-Rabin, 12 bases), `1 <= h <= 2`; every run row's
(g, h, r) equals its instance's; prediction B5 holds on the validator's own
distinct-degree factorisation of `f` over F_p: the five h = 1 genus-4 curves have
irreducible f (degree 9) and the four h = 2 curves have exactly 2 irreducible factors
(degrees 4+5, 1+8, 2+7, 1+8); all nine are squarefree.

## 7. Controls and reporting contract

- Reference arm: negation-map rho runs in every cell on the same instances with the same
  seeds, in the same unit (FPM_mid recomputed above) — present and priced.
- Cheaper baseline: arm `a` (full factor base + prefilter) beside arm `c` in every
  hyperelliptic cell, `Q_diem_full` beside `Q_diem_2lp` in every quartic cell — present.
- Null / external fastest implementation: not applicable to a synthetic small-prime
  cost ladder and none is claimed; no external implementation is compared.
- One table, one unit, ratio column: the per-cell table above satisfies it for FPM_mid;
  the §5a GAE band was not recomputed by the validator (see section 6).
- m = 83 / ECC2K-130 disclosure rules: not applicable (random curves over small F_p).

## 8. Host and standing under the gate

Host: the same shared 4-vCPU container as the producer (`Intel(R) Xeon(R) Processor @
2.10GHz`, 4 logical CPUs, 16 GB RAM, Linux 6.18.44-fc-v470, hostname `vm`); load
average 2.97 at replay start, 2.06 at its end; not an isolated host, not an `ecbench`
session, no A/A interval. Wall times are therefore informational only; operation counts
stand at any isolation level (AGENTS.md §12).

**Verdict.** For the 12 cells at p = 2053 and 4099 (12 of 18 instances, 72 of 108 runs)
the replay certificate holds: a binary built independently at `e5e38c53`, whose ladder
sources are byte-identical to the attributed commit `14ab74ec8b`, reproduces every
instance (f, order, h, r) and every one of 1752 counted fields exactly, including
`certified` and `ok`, and the validator's independent per-cell means and arm/rho
FPM ratios agree with `analysis.json` to at least 4 significant digits on all 18 cells;
disposition for those 12 cells: `validated` (counted fields and recorded certificates).
For the 6 cells at p = 8209 (36 runs) the counts are internally consistent and the
analysis agrees with the raw data, but no replay was performed; disposition for them:
`insufficient_evidence` (replay not done, not a negative finding). What the replay does
not establish: it does not re-derive [t]G = T from the data (t is not recorded), it does
not independently check the Cantor / Picard group law that both the planting and the
certificates share, and it does not recompute the bootstrap, GAE-band or exponent
readings. Under the AGENTS.md validation gate the rows have a matched rho reference and
this validator report, and the replayed cells have a replay certificate, but no
isolated-host measurement and no `ecbench` session; they remain **pre-gate**, as the
protocol itself states, and no leaderboard or `benchmarks/README.md` row follows from
this report. Two statements in the dispatch are corrected: there are 0 (not 3) rho runs
skipped in this study, and only the hyperelliptic IC arm certifies with two scalar
multiplications. This report is bound to `bound_r.jsonl` = `083ccc71…` and
`analysis.json` = `2580b8e3…` (the blobs in `eb142ccda3`); it does not cover any later
revision of those files.
