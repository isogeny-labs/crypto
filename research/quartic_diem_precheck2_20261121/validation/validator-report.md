# Validator report: quartic_diem_precheck2_20261121 (lead #1 pre-check 2)

Disposition: **validated-with-errata**.

- Validated from commit `2190b15c4c124c2b19d1011b9e914921e35c039f` on branch
  `claude/hyperelliptic-index-calculus-td1fdn`, on 2026-11-21 (validator session, one thread).
  The tree was clean when the dispatch arrived. While this validation ran, another agent
  edited `src/cryptanalysis/quartic_ladder.rs` and `src/bin/genus_ladder.rs` in the
  working tree (mtime 03:46:12Z, new sha256 `7ce83dd8...` and `a4ef03f7...`). Every source
  statement below is about the committed blobs. I extracted them with `git show
  2190b15c4c:<path>`. Their sha256 `fac0a5da...` and `29a2239d...` equal `source.sha256`.
- Every number in the study is still `[pcc:unverified]`, and this report does not change
  that. The study is a pre-check with no rho and no validation gate (PROTOCOL.md §9). This
  report is not a replay certificate, and it does not admit any leaderboard or
  `benchmarks/` row.
- Validator scripts are in `validation/`. Outputs are in `validation/out/`, binary replays
  in `validation/replay/`, and the rank-diagnosis replay in `validation/diag/`. No producer
  file was modified, nothing was committed or pushed, and no Rust was built.

## Claims under test and verdict

| # | claim (`[pcc:unverified]`) | verdict |
|---|---|---|
| A | With the fixed instrument, the FPM-mid exponent is 0.897 [0.858, 0.934] and the `la_mul` exponent 0.971, so neither K1 nor K2 fires | **holds**: arithmetic recomputed, verdict PROCEED-TO-LADDER robust; interpretation erratum (F11) |
| B | The NEW arm costs 0.391 of OLD pooled (0.144 at 65537) | **holds** for FPM mid and GAE w >= 0.01; at GAE w = 0 the arms tie (F9) |
| C | The algebraic descent returns exactly the scan's relations | **holds** (C1 56/56 on every field; independent port and brute force on 748 nets, F6) |
| D | The Wiedemann retries came from `t` being undetermined (collinear decomposition rows) and are cut by the T kick plus linked rows | **partly holds**: "t undetermined" is confirmed on grid rows. "Singular by construction" is contradicted on grid rows. The cut comes mainly from the linked policy (F12, F13) |

## Requirement table

| requirement | result | evidence |
|---|---|---|
| Frozen artefacts hash to the recorded values | pass | `sha256sum -c frozen.sha256 data.sha256`; `source.sha256` = committed blobs; binary sha256 `57e45b7c...` = `binary.sha256` (hashed at dispatch, before any rebuild) |
| Freeze before grid data | pass (weak binding, F1) | producer transcript timeline (F1); no grid-prime record on disk before 02:46:27Z |
| Draft-to-frozen changes | pass (F2) | `git show 46e8af44e7:.../PROTOCOL.md` vs HEAD |
| `analysis.json` reproduces from raw | pass | `python3 analyze.py raw/*.jsonl` output is byte-identical to `analysis.json` |
| Independent recomputation of exponents, intervals and paired ratios | pass (F8) | `validation/recompute.py` -> `out/recompute.json` |
| Binary replay of raw rows | pass, 63/63 rows identical on every field except `wall_ns` | `validation/replay.sh`, `compare_replay.py` -> `out/compare_replay.json` |
| Replay with `--quartic-diag` (uncounted) | pass, 64/64 rows identical on every raw field | `validation/diag/diag_old_grid.jsonl`, `out/diag_check.json` |
| Instances (prime order, Hasse-Weil, point counts) | pass | `validation/instances.py` -> `out/instances.json` |
| Descent correctness, independent code | pass | `validation/decomp_check.py` -> `out/decomp_check.json` |
| Accounting of `decompose_alg`, T kick, DSU, enumeration | pass, minor notes (F4, F5) | source reading at `2190b15c4c` |
| Certificates re-verified with independent code | **not possible** (F3) | records carry only booleans |
| Controls: OLD reference arm priced and identical to pre-check 1 (C0) | pass, 48/48 on every pre-check-1 field | `out/recompute.json` `C0` |
| Reporting contract (one unit, phases, ratios, band) | pass with errata (F9, F10, F11) | RESULTS.md |
| §5a band from `band.rs` (L-546d37ca7750) | not applicable | no rho arm and no ecbench session; the protocol's own GAE_w band was recomputed |
| m=83 / ECC2K-130 rules | not applicable | |

## Findings

**F1. Freeze timeline: the order holds, but the binding is weak.** The producer's
transcript (`~/.claude/projects/.../subagents/agent-ae10dd605bf620bb5.jsonl`) and the file
mtimes show this sequence:

- The last source edit was at 02:03:06Z. The release build started at 02:03:13Z and
  finished about 02:15Z (`qdiem2_build.log`).
- The binary was copied and hashed at 02:17:45Z.
- PROTOCOL.md was written at 02:18:44Z. The final wording edit and the writing of
  `frozen.sha256`, `source.sha256` and `binary.sha256` happened in one command at
  02:46:18Z.
- `raw/run.start` is 02:46:27Z. The main run ended at 02:49:03Z.
- The 65537 runs started at 02:49:14Z. The interim analysis ran at 02:49:23Z.
- `PROTOCOL.md` and `analyze.py` are byte-identical to the frozen hashes, so nothing
  changed after the data.

Development data are 11 files in the session scratch `qdiem2_dev/`. They cover only
p = 4093 and 8191 (curve seeds 45, 89, 20, 106) and a 65537 search pilot over seeds 1-3
with no admitted instance. No grid-prime record dated before the freeze exists. The
4-linked-row setting was chosen from `dev3_k3`/`dev3_k4`, which used the final binary: NEW
retried in 6/12 runs with 3 rows and 1/12 with 4, as PROTOCOL §2 states. The binding is
weak in three ways:

- The protocol was not committed before the run; it arrives in the same commit as the
  data (`2190b15c4c`, 03:19Z).
- `.resume/RESUME.md` has no entry for this run.
- The development records that chose the kick and K = 4, and that back the §2 diagnosis,
  are not committed.

Under the sabotage rule, the order rests on mtimes and a session-local transcript.
Errata: commit the development records, or their sha256 (listed under Reproduction).
Freeze future protocols in a commit or checkpoint before launch.

**F2. Draft protocol in the WIP commits.**

- `2a765f254a` (00:49Z) has no PROTOCOL.md, only `analyze.py` and the source.
- `46e8af44e7` (02:00:53Z) has a DRAFT. Its sections 2 and 5-9 say "FILLED AT FREEZE".
  Its metrics and fit sections equal the frozen ones.

The frozen protocol differs from the draft in four ways:

- The drop rule applies per arm instead of only to the primary arm. This makes no
  difference here, because nothing was dropped.
- `inhom_linked` was added as a descriptor.
- The arms, grid, budget and kill rule were added.
- The K1/K2 rules use the same metrics and the same 1.40 threshold as pre-check 1.

Between the draft and the freeze, `analyze.py` changed only by the arm rename `_gt` to
`_kick`. The instrument changed from a G+T walk step (`step_gt`, tried on development
curves at 02:00Z) to the T kick after counted rows (02:03Z). That change used development
data only. PROTOCOL.md does not mention the abandoned G+T variant, and should.

**F3. Certificates cannot be re-verified independently.** Run records carry only
`certified` (`[t]G == T` in the solver's own Pic arithmetic) and `ok` (`recovered ==
t_secret`). They do not record the recovered `t`, `G`, `T` or `t_secret`, so no
solver-independent check is possible from the artefacts. Mitigations:

- `run_diem_with` never reads `t_secret`. In the HEAD source, it appears only in
  `make_instance`, `make_instance_from_classes` and the tests.
- The binary replay reproduces `certified = ok = true` on 56 runs (63 rows), and the diag
  replay on another 64 rows.

This is acceptable for a scaling pre-check, which claims no solve as its result. It must
be fixed before the ladder stage: emit the recovered `t` and the encodings of `G` and `T`,
or a replay certificate.

**F4. Accounting of the new code is complete.** I read `Ctx::decompose_alg` (lines
668-846 at HEAD). Every F_p operation is counted into `rel_decomp`: the pivot elimination,
the resultant cubic, Cantor-Zassenhaus (`proots`, `ppowmod`, `pgcd`, `psplit`, `pdiv`),
the evaluations, the Fermat inversions (`finv` = counted `ppow`), Tonelli-Shanks or the
`q = 3 mod 4` root (`Sqrt::sqrt`), the three-conic check and the normalisation. The only
uncounted steps are zero tests, the random draws in `psplit`, and the negation
`q - h[0]`. Other new work:

- **T kick:** one `pq.add` with `ops.adds += 1`. Its F_p cost enters `cantor_*` through the
  Pic counter.
- **DSU under Linked:** `dsu.find` counts path-halving steps into `dsu.ops`, which goes to
  `la_add`. A find that is already at the root costs 0. The `vid` hash lookups are
  uncharged, as in `push_rel` for every arm.
- **`t_seen` scan:** counted into `la_add`.
- **Certificate `cmul`:** counted in every arm.

Nothing moved out of a counted field:

- All four arms have the same record keys.
- ALG equals OLD on every field except `rel_mul`, `rel_add` and `rel_decomp_*` (56/56).
- `rel_mul - rel_decomp_mul` and `rel_add - rel_decomp_add` are equal between ALG and OLD
  in 56/56 pairs, so only the descent share changed.
- Under Legacy, the "linked" diagnostic uses an uncounted, non-compressing `dsu_root`.
  That leaves OLD's counts unchanged (C0 48/48) and gives OLD nothing NEW pays for.

**F5. Point enumeration and point index: charged, and not an oracle.**

- `Ctx::new` runs inside every run of every arm. Its `rational_points` (O(p log p)) is
  counted in `rel_enum`, which is part of `rel_mul`. So is `val2`, 6 multiplications per
  point.
- Building the `idx` hash map (about p inserts) and `Sqrt::new`'s non-residue search are
  uncounted. They are the same in all arms and negligible.
- The `idx` lookup in `decompose_alg` runs only on candidates that already lie on all
  three conics. For a typical class, D is never collinear: a collinear D has h^0(D) = 2,
  which makes the class exceptional. In that case the rational base points of the net in
  P^2 are exactly the rational points of supp D, which lie on C. So the lookup is a label
  map, the same use `Ctx::line` makes in every arm, and not an on-curve test.
- Empirical check: in 748 nets, 452 of them on the grid curves at p = 1031 and 2053, no
  candidate that passed the conic test was rejected by the lookup
  (`candidates_on_conics_not_on_curve = 0`), and brute force over all of P^2(F_p) found no
  extra base point.
- Verdict: fair. The index needs the full enumeration, which every arm pays for. This is
  the next sublinear fix, as RESULTS states.

**F6. The algebraic descent equals the scan.** `validation/decomp_check.py` ports
`decompose_alg` and finds the roots of g by brute force instead of Cantor-Zassenhaus. It
compares the port with a scan over C(F_p) and with brute force over P^2(F_p). The nets come
from constructed effective divisors:

- A: three non-collinear rational points.
  - 360 nets on the 8 grid curves at 1031 and 2053.
  - 240 nets on random smooth quartics at p = 31, 37 and 101. Half of those curves pass
    through the coordinate points.
- D: 2Q1 + Q2, with tangency at Q1. 92 nets on grid curves.
- B: a rational point plus a degree-2 point. 32 nets.
- C: a degree-3 point. 24 nets.

Results:

- 748/748 nets agree after the fallback rule. Every type-A net returned its three points.
  Types B, C and D never decompose.
- The two port `Err` cases were collinear D, a common-component net that cannot occur for
  a typical class. The Rust code sends these to the counted scan, and the result then
  agrees.
- The branches were exercised: v = 0, 1, 2 and the `<xy, xz, yz>` net; 14 pivot-quadratic
  roots; 68 roots at infinity.
- The binary agrees on the grid: C1 holds 56/56 on every field, and `decomp_fallbacks` is
  0 in all 224 runs.

**F7. The instances are correct.** All 28 records have h = 1 and r = order. r is prime
(Miller-Rabin with 13 fixed bases, deterministic for n < 3.3e24), and the order lies in the
Hasse-Weil interval. The 24 grid instances equal pre-check 1's records. I recomputed
`n_points` by brute force for the 16 curves with p <= 8209, and all agree. The 65537
search followed the protocol:

- The odd list tried 29 seeds and kept 13 and 57 (495.8 s).
- The even list tried 83 seeds and kept 156 and 166 (1417.6 s).
- Each process stopped at its second admission.

**F8. The statistics reproduce.** `analyze.py` re-run on the raw files gives
`analysis.json` byte for byte. My own script indexes every record key with `[]`, and every
key exists (L-99ee952350b2). It uses a different generator. Every point estimate matches
exactly, and the bootstrap intervals match within 0.005:

| statistic | producer | validator |
|---|---|---|
| NEW `la_mul` | 0.971 [0.919, 1.024] | 0.971 [0.920, 1.024] |
| NEW FPM mid | 0.897 [0.858, 0.934] | 0.897 [0.858, 0.935] |
| NEW t-intervals (la, FPM) | [0.854, 1.088], [0.792, 1.002] | same |
| NEW six-prime fit (la, FPM) | 1.018 [0.933, 1.099], 0.898 [0.833, 0.957] | 1.018 [0.936, 1.097], 0.898 [0.831, 0.958] |
| OLD FPM mid | 1.329 [1.236, 1.419] | 1.329 [1.234, 1.419] |
| NEW/OLD FPM, pooled | 0.391 [0.349, 0.437] | 0.391 [0.350, 0.437] |
| NEW/OLD FPM, 65537 | 0.144 [0.107, 0.215] | 0.144 [0.107, 0.215] |
| ln-ratio slope, FPM | -0.432 [-0.502, -0.359] | -0.432 [-0.502, -0.358] |
| ALG/OLD FPM, pooled | 0.526 | 0.526 |
| KICK/ALG FPM, pooled | 0.899 [0.788, 1.016] | 0.899 [0.788, 1.028] |
| NEW/KICK FPM, pooled | 0.827 [0.747, 0.910] | 0.827 [0.745, 0.910] |

All sensitivity readings also reproduce: `la+add` 1.181, FPM low 0.870, FPM high 0.882,
GAE 0.638 / 1.063 / 1.064. None comes near 1.40. Even the widest upper bound, `la_mul +
la_add` at 1.231, is far below it, so the verdict is robust. One limitation: a percentile
bootstrap over 4 curves per prime is mildly anti-conservative, because the n/(n-1)
variance factor is missing. The t-intervals are the safer secondary reading, and they do
not change the verdict either.

**F9. Band disclosure for NEW/OLD (erratum).** The pooled 0.391 is in FPM mid. In GAE
terms, NEW is cheaper than OLD at every prime for w >= 0.01 (0.239 pooled at w = 0.1 and
w = 1). At w = 0 (group operations only), NEW/OLD = 1.032 [0.880, 1.197]: per prime it
ranges from 0.80 to 1.39, and NEW does slightly more Pic operations. The whole gain is in
native F_p and mod-r work, as the fixes intend. RESULTS reports GAE w = 1 only and should
state the w = 0 tie beside the 0.391. NEW/OLD at 1031 is 1.002 in FPM mid, so the
improvement appears only from p >= 2053.

**F10. Labels in the RESULTS per-prime table (erratum).**

- The table is headed "geometric means", but the `decomp_trials` column holds arithmetic
  means: 308, 348, 436, 955, 1826, 3856, 4750. The geometric means are 287, 314, 390, 785,
  1590, 3420, 4230.
- "OLD FPM mid at 65537 is 8.54e8, of which the scan descent accounts for 6.9e8" sets an
  arithmetic-mean `rel_decomp_mul` (6.89e8) against a geometric-mean total.
- The other cells match: `la_mul`, FPM, `rel_mul`, `rel_enum`, LA share, attempts, 11/56
  retries and the OLD attempt means.

**F11. Why the slope is below 1 (interpretation erratum).** A sub-1 local slope is
consistent with Diem's O~(q), but RESULTS gives the wrong reason. It says the slope comes
from "fixed costs ... the starts of the decomposition walk cost two scalar multiplications
each". Splitting NEW's FPM mid (`validation/components.py`) shows otherwise:

- The Pic-operation part (`cantor_mul + 10 cantor_inv`) is 52% of FPM mid at 1031 and 15%
  at 65537. Its exponent is 0.61.
- The rest (`rel_mul + 9 la_mul`) has exponent 1.022. Within it, `rel_enum` scales as
  1.107 (p log p) and `la_mul` as 0.971.
- `group_ops - decomp_trials` covers the walk starts, the kicks and the certificate. It is
  only 138-311 operations. At 65537, `group_ops` is about 4460, of which about 4230 are
  per-trial walk steps.
- So the low exponent comes from a sublinear term with a large constant: one Pic addition
  (about 3600-4000 F_p multiplications) per decomposition trial, with trials growing as
  p^0.735. The starts are not the cause.

That term is lower order. The fitted 0.897 is therefore a finite-range mixture, it must
not be extrapolated, and the asymptotic slope of this instrument is about 1 or slightly
above.

Is part of the gain an artefact of NEW doing less work than the method needs? No. Every
NEW run recovers the planted logarithm, attempts and retries are charged, `la_skipped = 0`,
and the linked test uses only attacker-side state. But NEW/OLD measures the removal of
instrument defects against a deliberately defective baseline. It is not a gain over the
lab's index-calculus baseline in the success-criterion sense. RESULTS says this
("not a success claim").

**F12. "The frozen first attempt is singular in t by construction" is contradicted on grid
rows (erratum).** The development curves showed 0/24 first-attempt successes in `diag1`
and 0/12 in `dev3_k4` OLD. On the grid, OLD succeeds on its first attempt in 20/56 runs:
6/8, 4/8, 4/8, 4/8, 0/8, 1/8 and 1/8 by prime. My uncounted diag replay of OLD and ALG on
the 16 grid curves with p <= 8209 (`validation/diag/`, 64 rows identical to raw) finds a
full-rank first attempt in 18/32 OLD runs. Some symmetry-breaking row, such as P0 entering
the core (the protocol's own exception), occurs often at small p. "By construction" is
true only for those development curves and not for this grid. The four development
curves were unrepresentative of the grid on this point.

**F13. Retry mechanism: partly confirmed.**

- That failures leave `t` undetermined is confirmed on grid rows. In my diag replay, 19/19
  failed OLD and ALG attempts have `rank = rank_no_t` with outcome 2 (wrong `t`). The same
  holds for 39/39 in development and 8/8 in the committed post-hoc NEW diagnosis.
- The cut in retries comes mainly from the descent-last/linked policy. NEW/KICK attempts
  are 0.635 [0.555, 0.719]. The kick alone gives KICK/ALG 0.870 [0.737, 1.028], and at
  65537 KICK is worse (first-attempt success 0/8). So "cut by the T-kick plus linked rows"
  holds only for the combination.
- In RESULTS, "as few as 1 or 2 core rows with a t entry" is literally true, but the 8 NEW
  failures have `t_weight` 1, 2, 2, 4, 4, 8, 14, 23. A low `t` weight does not explain them
  all. The proposed rank-aware stop is an untested hypothesis.

**F14. Binary-to-source binding (not rebuilt).** I did not rebuild, as instructed. Four
things support the binding:

- The transcript timeline in F1: the build started after the last source edit, and the
  copy came before any later build.
- The binary contains the strings `2lp-kick` and `kick_t`, and lacks `2lp-gt` and
  `step_gt`, the pre-02:03Z names.
- The default path reproduces pre-check 1's frozen instrument (`ca72267899`) on every
  field (C0 48/48).
- 127 replayed rows match the raw data.

The binary is therefore the one that produced the data. That it was compiled from exactly
`fac0a5da`/`29a2239d` is attested by the transcript, not by a rebuild.

## Controls

- The reference arm (OLD) is priced in every unit and reproduces pre-check 1 exactly (C0).
- The decomposition contrasts (ALG/OLD, KICK/ALG, NEW/KICK) are all reported.
- There is no rho, null or external fastest implementation, by design (pre-check;
  PROTOCOL §9). The next stage, the ladder, needs the L10-corrected matched rho and the
  §5a band.

## Reproduction

```sh
cd research/quartic_diem_precheck2_20261121
sha256sum -c frozen.sha256 data.sha256
python3 analyze.py raw/run.jsonl raw/run65537_odd.jsonl raw/run65537_even.jsonl | cmp - analysis.json
P=../quartic_diem_precheck_20261120/raw/run.jsonl
python3 -I validation/recompute.py $PWD $P > validation/out/recompute.json
python3 -I validation/components.py $PWD > validation/out/components.json
python3 -I validation/instances.py $PWD $P > validation/out/instances.json
python3 -I validation/decomp_check.py $PWD > validation/out/decomp_check.json
validation/replay.sh <scratch>/bin/genus_ladder.qdiem2 validation/replay   # 7 cells, 4 arms, 2 seeds, 1 thread
python3 -I validation/compare_replay.py $PWD validation/replay > validation/out/compare_replay.json
# rank diagnosis (uncounted), OLD and ALG on the 16 grid curves with p <= 8209:
genus_ladder.qdiem2 run --primes 1031,2053,4099,8209 --genera 2 --arms - \
  --curve-seeds 1031=1,2053=1,4099=1,8209=1 --quartic --quartic-arms 2lp,2lp-alg --quartic-inhom 4 \
  --quartic-seeds 1031=54/115/244/290,2053=43/72/73/105,4099=35/205/220/257,8209=10/16/114/165 \
  --quartic-diag --qc 3.0 --max-h 1 --curves 4 --seeds 2 --rho-cap 1000 --threads 1 \
  --out validation/diag/diag_old_grid.jsonl
python3 -I validation/diag_check.py $PWD validation/diag/diag_old_grid.jsonl <scratch>/qdiem2_dev > validation/out/diag_check.json
```

Replayed cells (all four arms, solver seeds 1 and 2): 1031/244, 4099/220, 8209/114,
16411/93, 32771/217, 65537/156 and 65537/13. These include NEW runs with two attempts and
OLD runs with up to 9 attempts.

Uncommitted development records, in the session scratch `qdiem2_dev/` (sha256 prefixes):

- `diag1` `b1cc3401`, `dev1` `74edcfd0`
- `dev_k2` `330777c9`, `dev_k4` `3f334535`, `dev_k8` `c9262812`
- `dev2_k2` `f35e6fd1`, `dev2_k4` `e54ef189`, `dev2_k8` `6cb56933`
- `dev3_k3` `5795278e`, `dev3_k4` `7290cb60`
- `pilot65537_search` `65c9f53e`

## Lessons applied

- L-99ee952350b2: every key indexed with `[]`; key sets asserted equal for C0 and C1.
- L-26895912385b: ratios recomputed from unrounded records, with paired ratios reported.
- L-3f39e1d8b3a9: binary hashed at dispatch.
- L-1a5a6b64022c: this file was written only when every check was complete.
- L-3013ef925e8d: recounted on all grid rows. This exposed F12.
