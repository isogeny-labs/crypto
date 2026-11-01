# Validator report: `frob_roots` in the char-2 D4 index calculus (PR #2759)

Verdict: **validated-with-caveats**. The replay, the recompute, the identical-arm checks, the
accounting audit and the rho-reference checks all pass. The caveats are in section 5. This is a counted-unit (ecbench GAE) engineering improvement over the
2026-10-15 baseline. It is admissible as evidence. It is not a wall-clock result and not a beat of rho.

Validator: a subagent (claude-opus-5-5) that did not write the code, the sessions or the report.
Snapshot: branch `claude/ic-d4-char2-lever-20261023`. The validated content is at 079e1022a, and the PR
head at validation time is 01dbef2b7. That commit merges origin/main and touches nothing in this
directory. Sessions were measured at d2eec8df3 with `git_dirty: false` and binary sha256 `13818f3e…`.
Machine-readable files: `VALIDATOR_REPORT.json` (pinned by `../VALIDATION.json`),
`REPLAY_CERTIFICATE.json`, and the replay receipts `replay-lever-m1{2,4}.audit.json` with their logs.

**Hydration.** From 2ec8a532c on, `results/lever-m12/records.jsonl` lives in the artifact store
(same sha256, c43a5ed8…). Run `tools/artifacts/artifacts.py pull research/ic_d4_char2_lever_20261023`
before `python3 scripts/validation_gate.py evidence research/ic_d4_char2_lever_20261023`. Otherwise
the gate reports that the raw_data path is not a file.

## 1. Replay certificate

I ran `ecbench verify --replay-all --exit-code` with the pinned binary
`/tmp/ecbench-lever-measured`, sha256 `13818f3e…`. That is the hash in every record and in both
`session.json` files. Settings: `RAYON_NUM_THREADS=1`, `taskset -c 3`, one session after the other.
The receipts went to new files, and the producer's receipts and session files are byte-unchanged.

| session | records | verified | replays identical | problems | elapsed | receipt sha256 |
|:--|--:|--:|--:|--:|--:|:--|
| `results/lever-m14` (ECBS1hcdb18ea9ecd1) | 4 | 4 | 4 | 0 | 8,905 s | a36e139e7c651a496c9b6c85a0e034b76fd9a91ed9892b1335dda4378a00a260 |
| `results/lever-m12` (ECBS1h081b9c5cbc94) | 12 | 12 | 12 | 0 | 7,343 s | ede7aeac0b97a54176dd7ba4e56eceae9202d05e12f2c53cff8371e61f47ffb4 |

In both receipts all twelve integrity checks are true: arms, env_class, executions, host/plan/records
hashes, spec_id, workloads, session_complete, records, record_count and replays. Apart from
`audited_unix_ms`, each receipt is identical to the producer's. Every recovered scalar equals the
planted one (16 of 16). `REPLAY_CERTIFICATE.json` lists all 16 runs.

## 2. Recompute from `records.jsonl` (the producer's cmp/tsv/tabulate.sh were not used)

I wrote my own script (Python stdlib, Decimal/Fraction). It reads only `records.jsonl` and runs 296
checks over the 16 records, all passing. For every record it checks:

- `verified`, `matches_target`, `matches_planted`, `recovered == planted`, not a warm-up, binary `13818f3e…`;
- `total_gae` equals the sum of the phase gae, and `S = total_gae / sqrt(r)` to 1e-12;
- for the IC arms:
  - solver gae = `ops / 179`. The 179 is `fq_muls_per_add`, the measured F_q multiplications in one
    affine addition on this curve (`gaudry_quartic_bin::fq_muls_per_add`);
  - linear-algebra gae = `row_ops / 179`;
  - factor-base gae = adds + sqrt_solves.

| quantity | claimed | recomputed |
|:--|:--|:--|
| m = 12, Σ S_lever / Σ S_base (3 targets), w = 0 / 0.1 / 1 | 0.8543 (cmp 0.8543049931370235) | 0.8543049931370235 at all three weights |
| m = 12 per-target lever/base | 0.85430, 0.85430, 0.85432 | 0.8543027, 0.8542952, 0.8543175 |
| m = 14, S_lever / S_base (1 target), w = 0 / 0.1 / 1 | 0.8154 (cmp 0.8153730005333587) | 0.8153730005333586 at all three weights |
| A/A ic-base-aa / ic-base, m = 12 and m = 14 | 1.0 | 1.0 exactly: the whole cost block is identical |
| m = 12 S base / lever per target | 44.03, 42.59, 41.47 / 37.62, 36.39, 35.43 | 44.0338, 42.5916, 41.4672 / 37.6182, 36.3858, 35.4261 |
| m = 14 S base / lever | 5.3701 / 4.3786 | 5.370113 / 4.378646 |
| m = 14 IC/rho, base, w = 0 / 0.1 / 1 | 4.27 / 3.88 / 2.13 | 4.2684 / 3.8804 / 2.1342 |
| m = 14 IC/rho, lever, w = 0 / 0.1 / 1 | 3.48 / 3.16 / 1.74 | 3.4804 / 3.1640 / 1.7402 |
| m = 12 IC/rho per target, base -> lever | REPORT table | t0 47.23->40.35, 42.94->36.68, 23.62->20.18; t1 50.29->42.97, 45.72->39.06, 25.15->21.48; t2 39.88->34.07, 36.26->30.98, 19.94->17.04 |
| m = 14 S / floor (sqrt(pi/4)), base -> lever | 6.06 -> 4.94 | 6.0595 -> 4.9408 |
| trials (identical in all IC arms) | 186,121; 180,033; 175,246; 749,350 | same |

All 75 headline numbers agree with the claims, within the rounding of the digits the producer printed
(`VALIDATOR_REPORT.json`, `headline`).

**Pairing.** The four arms of each target share the same `workload_sha256`, target point, planted
scalar and `algorithm_seed`. The lever's method params differ from ic-base only by `,frob_roots=1`.
ic-base reproduces the 2026-10-15 sessions exactly: same workload, seed, total gae, solver ops,
trials and recovered scalar (m = 14 t0; m = 12 t0 and t1, from
`research/gaudry_quartic_d4_char2_20261015/sessions/{char2-m14,char2-ladder}`).

**Identical between base and lever on every target.** These fields match exactly:

- the trial sequence: trials, solver calls, walk steps, target-guard probes;
- the relation set: relations, rational solutions, shape replays and fallbacks;
- the matrix: rank, rows, columns and linear-algebra row_ops;
- the group operations: relation-phase adds, doubles and scalar mults;
- the factor base: fb_sha256 and points_sha256;
- the other solver components: charpoly, echelon, normal-form and precompute F_q multiplications;
- the recovered scalar.

Identical row_ops (1,911,282,681 at m = 14) are strong indirect evidence that the relation matrices
are the same. The records do not compare the relation vectors directly.

**Where the reduction comes from.** Solver ops minus the four unchanged named components is the part
holding roots, eigenvector, split, Weil, triangularise and verify. It falls to 0.450 of base at
m = 12 and 0.408 at m = 14. Solver ops overall fall to 0.8539 at m = 12 and 0.8131 at m = 14.
Per trial at m = 12 that is 224,050 -> 191,314 F_q multiplications, consistent with the REPORT's
profile figures (224,196 -> 191,435). Peak RSS is equal within 0.3 % (32.8-33.1 MB at m = 12,
155.3-155.4 MB at m = 14).

**Frozen specs.** `PROTOCOL.md`, `specs/SPEC-lever-m12.json` and `specs/SPEC-lever-m14.json` are
byte-identical to d2eec8df3. That commit is dated 06:12:12Z, and the sessions started at 06:15:19Z and
06:15:43Z. Each session's `spec.json` equals its frozen spec. The `records.jsonl`, `plan.json`,
`host.json` and `session.json` hashes equal both the producer's audit receipts and `session.json`
`records_sha256`. The session's canonical `spec_sha256` is checked by `ecbench verify`
(`spec_id: true`); I did not re-derive its canonical form.

## 3. Multiplication accounting in the diff (`src/cryptanalysis/gaudry_quartic_bin.rs`)

- `SqMod::new` builds the table of `X^(d+j) mod h`, `j = 0..d-2`. Every `g.mul(top, hk)` is followed by
  `*muls += 1`. Zero coefficients are skipped, as in the baseline `pmod`.
- `SqMod::sq` counts each `a_i^2` (`g.mul(a, a)`, +1) and each table product `g.mul(a2, t)` for
  `t != 0` (+1). This covers every `GfTab::mul` call site in the new code. `GfTab::mul` itself counts
  nothing, so its call sites must, and they do.
- The roots of the degree-64 determinant are counted into `roots_muls`. `split_partial` roots go into
  `eigenvector_muls`, and the quartic roots into `split_muls`. All three are summed into `total_muls`,
  which becomes ecbench `ops`. A table is built once per modulus, including once per `split_roots`
  recursion node, and every build is counted.
- What the change removes is real executed work. `pmul(r, r)` computes the cross products
  `a_i a_j + a_j a_i`, which cancel in characteristic 2, and the generic `pmod` multiplies by the monic
  leading coefficient. No count was re-priced, and nothing moved to another phase.
- **Newly uncounted native work, flagged and not measured.**
  - Each table build does about (d-1)(d-2) u16 shift-copies.
  - Each table takes (d-1)·d u16 of storage, about 8 KB at d = 64.
  - The table loops test every entry for zero.
  - None of these are multiplications. The baseline's comparable uncounted work (allocating and copying
    vectors of length 2d-1 in `pmul`/`pmod`, `ptrim`) shrinks by more than this.
  - In both paths each counted multiplication carries at most one XOR, so uncounted F_q additions in
    root finding fall at least in proportion.
- I did not run the unit tests (`cargo test`, 7 pass per REPORT), because no build was possible on this
  host. I did read both new tests.

## 4. Matched rho reference

`rho-neg` = `rho.negation` (cap_multiple 64), as PROTOCOL.md names it. It runs in the same session,
same binary, same targets and algorithm seeds, and the same unit (`ecbench.gae`).

It is the strongest reference available:

- Aut(E) = {±1}: the curves are ordinary, with odd trace, so j != 0.
- There is no Frobenius or GLS fold, because j(E) lies in no proper subfield.
  - m = 14 (F_(2^56), t = -338854401): 2^29 ± t are not squares, so E is neither defined over
    F_(2^28) nor a quadratic twist of a curve over F_(2^28). Definition over F_(2^8) would force
    #E_0(F_(2^8)) in [225, 289] to divide 2r, which is impossible.
  - m = 12 (F_(2^48), t = -17036033): 2^25 ± t are not squares, which excludes F_(2^24). F_(2^16)
    would need a divisor of 10r in [65025, 66049], and there is none.
- In ecbench, the Frobenius walks `rho.signed_frobenius*` apply only to Koblitz `Instance::Binary`.
  `rho.cubic` and `rho.quartic` apply only to prime-field curves with j = 0 or 1728. At the new head,
  `rho.frobenius_orbit` and `rho.gls` refuse anything except `subfield_fp3` and `gls_fp2`.

The §5a band:

- Rho names `canonicalisations_uncharged` and emits its count: 238,801,980 at m = 14, and
  4,945,607 / 4,492,373 / 5,515,583 at m = 12. It is priced at w GAE each.
- The IC arms list no unpriced counter.
- Rho stays below ic-lever, and ic-lever below ic-base, at w = 0, 0.1 and 1 on every target. The
  order holds across the band.
- m = 14 band values in VALIDATION.json: ic-lever 4.378646 at all three weights; rho 1.258096 /
  1.383906 / 2.516190.

## 5. Caveats and limitations (none voids the result)

1. **Same host, same binary.** The replay re-executes the same binary on the same host class. It
   establishes exact reproduction and re-verified scalars. It is not an independent-host or
   independent-implementation replication. I could not rebuild, for lack of disk. The source-to-binary
   link rests on the binary's `git_commit` with `git_commit_source: runtime_worktree`, which is read
   from the worktree at run time and is not a build attestation. The scalar checks are the binary's
   own.
2. **One target at m = 14.** This follows the one-target rule in AGENTS.md. The m = 12 per-target ratios
   agree to 2.6e-5, which suggests the m = 14 ratio is representative, but other m = 14 targets were not
   measured.
3. **Prediction miss, not reported.** At m = 14 the ratio of 0.8154 is below the pre-registered
   interval [0.82, 0.95]. It is better than predicted: the root-finding part was about 31.6 % of solver
   ops rather than the 19-28 % assumed. REPORT.md does not mention the miss. The protocol's success
   condition does not depend on that interval, and it holds.
4. **Lower bound with respect to uncounted work.** The IC records say `lower_bound: false` with
   `unpriced: []`, but F_q additions (XOR), log/exp lookups, branches and copies are uncounted in every
   IC solver component, not only the eigen step the REPORT mentions. Under §5a this is an emission gap.
   It leaves lever/base unchanged as far as code reading shows. It can only widen the gap to rho, so the
   order in the band is unaffected.
5. **Counted unit, not wall clock.** The REPORT's in-process wall figure (4.595 -> 4.439 ms per trial,
   0.966) and its profile components have no committed raw artifact. In the sessions, L0 wall
   lever/base is 1.019 at m = 14, with A/A at 0.786, which is noise. So the wall-time direction check in
   protocol point 3 is asserted but not backed by an artifact in the repository. No wall claim is made.
6. **Sweep convention.** Under the sweep-verdict convention on main, `validated_with_caveats` does not
   pass. This VALIDATION.json passes `validation_gate.py evidence`, because the gate reads only the
   three artifacts. Whether caveats 1-5 bar admission to the leaderboard is the Coordinator's call.

```yaml
validation_report:
  id: VAL-20261028-d4c2lv
  task_id: PR-2759-validation
  run_ids: [ECBS1h081b9c5cbc94, ECBS1hcdb18ea9ecd1]
  artifact_checks: [spec/protocol bytes = d2eec8df3, session file hashes = receipts, binary 13818f3e, git_dirty false]
  metric_recomputations: [75 headline numbers, 296 record checks, all agree]
  control_checks: [A/A exactly 1.0, base = 2026-10-15 baseline, trials/relations/rank/row_ops identical]
  heuristic_validation_checks: [not applicable: deterministic counts, no heuristic distribution claimed]
  cost_model_checks: [unit ecbench.gae with F_q mults at measured 179 per affine add, memory reported, band 0/0.1/1]
  proof_architecture_checks: [not applicable]
  verdict: passed   # validated-with-caveats; gate `evidence` says: valid
  limitations: [same-host same-binary replay, one target at m=14, m=14 below predicted interval, uncounted XOR/lookups, no wall claim]
  artifact_paths: [validation/REPLAY_CERTIFICATE.json, validation/VALIDATOR_REPORT.json, VALIDATION.json]
```
