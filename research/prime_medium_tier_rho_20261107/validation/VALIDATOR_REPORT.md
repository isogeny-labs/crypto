# Validator report: prime medium-tier rho reference bound

Validator session, 2026-11-08. Study: `research/prime_medium_tier_rho_20261107`.
Validated from commit `f13a690df016d0a457bd40579fbb9d054f486adf` (main). The study
directory was clean at that commit. I wrote only into this `validation/` directory.
The machine-readable report is `validator-report.json`. Every number below is
`[pcc:unverified]`: no pcc claim or keyring signature exists for this study.

## Disposition: `validated`

The six sessions, their counts and certificates, the twelve sealed medium bounds,
the per-job P4, P7 and P8 comparisons, and the R(N) point fits all recompute from
the raw records. My code and my replay sample produced the same numbers. The
§9 verdict, recomputed, is **`inconclusive`**, which is what the producer states.

- The M line is `setup_excess`.
- P4 has no lower end above 1.2 in any job.

There are eight discrepancies. None of them changes the verdict. One is a
misclassified secondary prediction (D1, P8). The rest are a specification gap,
provenance labels, wording and omissions (D2 to D8).

Scope: this is a rho-only reference study with no IC arm. It does not validate an
IC leaderboard row, and the gate's `{ic, rho}` band pair has no IC side (C15).

## Checks

| id | requirement | result | evidence (measured / derived) |
|:--|:--|:--|:--|
| C1 | Frozen artifact set, named commit, clean tree | pass | `git rev-parse HEAD` = f13a690; `git status --ignored` on the study shows only ignored card logs and `sources/` |
| C2 | Protocol frozen before data; `protocol_sha256` recomputes | pass | sha256 of the sorted, compact JSON of `protocol.json` without `freeze` = `5643e3e7…4cd6`. Freeze commit 189b78587b at 21:15:31Z; first session 21:27:16Z. Since 529aa8ee only the arms' `instrument.commit`/`binary_sha256` were filled in; `PROTOCOL.md` is byte-identical |
| C3 | Session files hash to the receipts, S3, `session.json` digests and artifact manifests | pass | `recompute/hashes.json`: 0 problems. Both buckets hold every `records.jsonl` with the manifest size. The six files are 18,190,983 bytes, in `raw/` (gitignored) |
| C4 | Six jobs admitted for counts under §10 | pass | `job-exit.json` gives 42 in every job, and every `l2-gate.json` has `accepted: false`. L1 record counts per job: 1, 14, 57, 50, 67, 46, matching the records' isolation levels. Every session is `complete`, with 1680/560/140/140/140/140 executions, all verified |
| C5 | `verify --replay-all`: 0 problems, every replay identical | pass | Pod `audit.json` and local `Jn.audit.json`: `ok`, 0 problems, 1440/480/120×4 replays identical. The bounds pin the local audit hashes |
| C6 | Image, ecbench sha256, source revision | pass | Manifest digest `292bdc3d…cae5` recomputed from ECR. Layer `1ea30931…` holds `usr/local/bin/ecbench` with sha256 `a18af09f…d3b9` (streamed, not stored). GitHub artifact `isolab-build-42413370386-1` has digest `457ef103…`, equal to the bundle zip the frozen binary came from. `build-bundle`, `isolab-experiment.yml` and `build_provenance.rs` are identical at dbd587a01a and e374755 |
| C7 | The `git_commit e374755b` explanation | pass, with note | GitHub API, run 42413370386: `workflow_dispatch` on `main`, `head_sha` e374755bd7. ecbench uses `option_env!("CRYPTO_BUILD_GIT_COMMIT").or(option_env!("GITHUB_SHA"))` (`src/build_provenance.rs`, `ecbench/host.rs`), and `build-bundle` sets neither variable. The binary contains the string `e374755bd7…` and not `dbd587a01a`. `receipts/image-publication.json` names dbd587a01a but mentions neither e374755 nor GITHUB_SHA (D3) |
| C8 | Certificates checked by solver-independent code | pass | `recompute/certificates.json`: all 2800 records pass (2400 measured). Checks: p and r prime; G on the curve; [r]G = O; r in the Hasse interval with r > 4√p, so #E = r; a, b ≠ 0, so Aut = {±1}; target = [planted]G; recovered = planted and [recovered]G = target; S = total_gae/√r; the rho counter identity |
| C9 | Nonce-sampled exact replay | pass | Nonce `fea12c9b689d3b3ed11162cb78f2b8f8` (/dev/urandom, 04:52:18Z). Sample: 3 runs per (job, arm), 90 in total. `ecbench verify --replay-seq` reproduced 90/90 with 0 problems (`replay/Jn.verify-replay-seq.json`). My own `profile-input` → `exec` → field comparison found 90/90 identical (`replay/exec-replay-compare.json`). Replay host class ECBHW1he9e33daae9e7 |
| C10 | Design as preregistered | pass | `recompute/design.json`: 19/19 seeds recompute from the rule. Curves per job, targets 8/8/4/4/4/4, rounds 6 + 1 warm-up, `alternate`, L2, 300 s, and the arms all match, with default parameters. Workloads are disjoint across jobs, and the records' curves equal `curves.json` |
| C11 | M and its interval | pass | M = 1.0934308362429292 (sealed 1.0934308362429286). With the harness's resampler (seed 20261005 ^ 0xF): [1.0325593293474007, 1.1558904712271187]; sealed [1.0325593293474, 1.1558904712271172]. With my own RNG: [1.0316, 1.1584] |
| C12 | P4 per job | pass | Ratio of totals with a linearised t interval over workload clusters, using my own t quantile. All six jobs agree with `comparisons/rho-neg__rho-plain.json` to 2e-15 |
| C13 | §9 verdict | pass | M ≤ 1.10 and its upper end < 1.30, so the M line is `setup_excess`; `within_1.10_strict` is false. P4's lower end exceeds 1.2 in no job, under either the linearised or the bootstrap interval. Verdict: **`inconclusive`** |
| C14 | §5a band on every arm, the reference included | pass | Unpriced counters are listed and priced at w = 0, 0.1 and 1 for all four arms. All twelve sealed records reproduce: values, both intervals and α |
| C15 | Matched-rho reference with the band for this row | not applicable | There is no IC arm. The row is the reference itself: `rho.negation`, as the protocol names it, in the same binary and sessions, unit `ecbench.gae`. Negation is the strongest symmetry for these curves. The order of rho.negation and rho.plain flips inside the band (below) |
| C16 | Secondary predictions as RESULTS.md reports them | fail (minor) | P8 at J1 (D1) |

## Recomputed headline numbers (measured counts, derived statistics)

| quantity | producer | validator | agreement |
|:--|:--|:--|:--|
| M, rho.negation ratio to floor at w = 0 | 1.093 [1.033, 1.156] | 1.0934 [1.0326, 1.1559] (harness resampler); [1.0316, 1.1584] (own RNG) | 1e-15; own-RNG interval within Monte-Carlo error |
| rho.negation at w = 0.1 / w = 1 | 1.203 [1.136, 1.271] / 2.185 [2.063, 2.310] | 1.2026 [1.1356, 1.2714] / 2.1853 [2.0632, 2.3104] | 1e-15 |
| rho.plain | 1.433 [1.343, 1.522] | 1.4334 [1.3428, 1.5223] | 1e-15 |
| bsgs.negation | 1.111 [1.032, 1.185] | 1.1106 [1.0319, 1.1845] | 1e-15 |
| rho.negation_v2 | 1.069 [1.003, 1.138] | 1.0690 [1.0031, 1.1376] | 1e-15 |
| α: neg / plain / bsgs / v2 | 0.497 / 0.506 / 0.500 / 0.491 | 0.49689 / 0.50609 / 0.49962 / 0.49121 | 1e-15 |
| P4 J1 | 1.285 [1.177, 1.404] | 1.2855 [1.1766, 1.4043] | 2e-15 |
| P4 J2 | 1.286 [1.094, 1.512] | 1.2859 [1.0940, 1.5115] | 2e-15 |
| P4 J3 | 1.352 [0.909, 2.009] | 1.3515 [0.9093, 2.0088] | 2e-15 |
| P4 J4 | 1.414 [0.688, 2.906] | 1.4139 [0.6880, 2.9057] | 2e-15 |
| P4 J5 | 1.663 [0.986, 2.805] | 1.6633 [0.9864, 2.8047] | 2e-15 |
| P4 J6 | 1.217 [0.737, 2.011] | 1.2175 [0.7372, 2.0105] | 2e-15 |
| P7, A/A (every job) | 1.0000 | 1.0000; every pair's counters identical | exact |
| P8 J1 | 1.002 [0.934, 1.074] | 1.0016 [0.9338, 1.0743] | 1e-15 |
| R(N) model A β, χ² | 0.578, 13.25 | 0.578, 13.249 | exact (point) |
| R(N) model B R∞, χ² | 1.084, 5.49 | 1.0842, 5.492 | exact (point) |
| weighted mean R, medium / toy | 1.083 / 1.380 | 1.0830 / 1.3796 | exact |

`validator-report.json` lists all 180 compared numbers. Each entry gives the
producer value, the recomputed value and `rel_tol` 1e-9. None fails.

**§5a band and reference order (derived).** Ratios to the floor:

| arm | w = 0 | w = 0.1 | w = 1 |
|:--|:--|:--|:--|
| rho.negation | 1.093 | 1.203 | 2.185 |
| rho.plain | 1.433 | 1.433 | 1.433 |
| bsgs.negation | 1.111 | 1.222 | 2.221 |
| rho.negation_v2 | 1.069 | 1.176 | 2.140 |

The J1 paired ratio rho-plain / rho-neg is 1.285, 1.169 and 0.643 at w = 0, 0.1
and 1. rho.negation is therefore not the cheaper arm across the whole band.
Ranking it the strongest reference holds at w ≤ 0.1 and is unresolved at w = 1,
so an IC gap priced at w = 1 needs that stated.

The unpriced counters:

- rho.negation: `canonicalisations_uncharged`, about 0.97 per √r.
- rho.plain: `canonicalisations_uncharged`, always 0.
- bsgs.negation: `inserts_uncharged` and `lookups_uncharged`.
- v2: `canonicalisations_uncharged`, `distinguished_point_table_inserts_uncharged` and `distinguished_point_table_queries_uncharged`.

## Discrepancies

- **D1 (minor).** RESULTS.md says "P8 holds at J1: 1.002 [0.934, 1.074]". PROTOCOL.md §7 P8 requires the interval to contain 1 and to lie inside [0.93, 1.07]. The upper end, 1.0743, is above 1.07, so P8 fails at J1 as written. The numbers are right; the classification is wrong.
- **D2 (specification gap).** P4 does not say whether its interval is per job or pooled.
  - Per job, as the §6.1 compares compute it, P4 fails in every job, under both the linearised and the bootstrap interval.
  - Pooled over all six jobs, P4 is 1.3109 [1.2239, 1.4041] over 80 workloads and would clear 1.2. This reading was not preregistered; I computed it only as a sensitivity.
  - §10 forbids choosing the pooled reading after seeing the data, so the verdict stays `inconclusive`.
  - Derived, not measured: √2 × 1.0135 / 1.0934 = 1.311. The 1.41 prediction assumed rho.negation sits at its floor.
- **D3 (provenance label).** The sessions' `git_commit` e374755bd7 is not the build source.
  - e374755 and dbd587a01a are siblings (merge-base 8bb2629dd3). They differ in five `src/` files, including `ic_boundary.rs` and `ecbench/methods.rs`.
  - A rebuild from the session's `git_commit` would not give a18af09f. The binding goes through the binary's sha256 and `build-info.json`.
  - RESULTS.md explains this correctly. The e374755 link, though, rests on the GitHub API and not on `receipts/image-publication.json`.
- **D4 (reproducibility binding).** The protocol's `instrument.build` is `cargo build --release --bin ecbench`, but a18af09f came from `build-bundle`: `cargo +1.99.1 build --locked --release --no-default-features --target x86_64-unknown-linux-musl --bin ecbench`, with `RUSTFLAGS=-C target-cpu=x86-64`. The sessions record `git_dirty: null`; only `build-bundle` asserts a clean tree.
- **D5 (wording).** PROTOCOL.md §5 says the cluster bootstrap runs "over the sizes' workloads". The harness resamples sizes, then runs, with no workload stage. A sizes → workloads → runs sensitivity gives [1.0215, 1.1629], and the M line does not change.
- **D6 (omission).** RESULTS.md does not state P1, P5, P9 or the Holm family over the secondary methods. Recomputed:
  - P1 holds on its point: 1.093 is inside [1.00, 1.10].
  - P5: the point β = 0.578 is outside 0.8–1, though its interval [0.21, 1.89] contains that range. Model A is not rejected (p 0.51).
  - P9: no curve effect after Holm. The smallest p is 0.088 with run-level SEs and 0.141 with the sealed size intervals.
- **D7 (minor).** `build-info.json` gives `source_repository` as `aburan28/crypto`, a value hard-coded in `build-bundle`. The workflow ran in `isogeny-labs/crypto`.
- **D8 (informational).** The twelve sealed bounds are already in `docs/bounds/frontier.json` and `FRONTIER.md`, which is outside the gate's scope. No IC-leaderboard or `benchmarks/README.md` row cites this study.

## Limitations

- **No independent rebuild.** The replays ran the producer-pinned binary a18af09f. I checked that binary against the ECR layer and against the GitHub artifact digest. Because no rebuild was done (see Errors), the replay tests cross-host determinism, not source or compiler independence. Even with disk, the local rustc 1.97.0 could not have reproduced a binary built with 1.99.1.
- **Validator code is stdlib Python.** The dispatch asked for it, but AGENTS.md "Implementation language: no Python" keeps such tools out of the repository. The scripts stay in the scratchpad, and their sha256 values are in `validator-report.json`. If this report is to back a `VALIDATION.json`, a native re-implementation is an open thread.
- **Gate file check.** The gate's `check_validator_report` on this report, with the study directory as base, finds only that the six `sessions/Jn/records.jsonl` are not on disk. They are artifact-store files; pull them with `tools/artifacts` before a file-level gate check. Their sha256 values are pinned in `raw_data`.
- **Not re-run or not re-derived:**
  - The curve generator. The curves were checked arithmetically instead.
  - The `uniform_scalar_sha256_v1` target derivation. Instead, target = [planted]G was checked on every record, and the sampled child inputs equal their records.
  - The R(N) bootstrap intervals.
- **Not assessed:** wall time (not admitted under §10), the missing experiment export, and the gaps in the benchmark record.
- **Not consulted:** research-kb and Conductor, which are not installed here.

## Errors

- My first source export was `git archive` of the whole dbd587a01a tree into the scratchpad. It filled the shared disk to 0 bytes free (about 4.1 GB). I stopped it and deleted the tree, the coordinator confirmed, and the disk is back at 7.4 GB free. I then used the frozen binary, as the coordinator directed.
- My first phase-accounting check (gae = adds + doubles) was wrong for counter-priced phases. I replaced it with the rho counter identity, which holds on every record.
- My first harness-resampler replica used the unsalted seed. The harness salts it per dimension: ^0xF for the ratio, ^0x5 for S. With the salts, the sealed intervals reproduce to 2e-15.

## Files

- `validator-report.json`: the gate fields (`status: pass`, `independent: true`, `raw_data` with sha256, `headline` with 180 entries), plus the checks, discrepancies, limitations, replay and rho-reference sections.
- `replay/`: the nonce and the sample, six `ecbench verify --replay-seq` receipts, and `exec-replay-compare.json`.
- `recompute/`: `hashes.json`, `certificates.json`, `estimates.json`, two harness-resampler `.jsonl` files, `design.json`, `rfit-points.json`, and `ecr-manifest-292bdc3d.json`.
- `raw/` (gitignored, 19 MB): the S3 copies of the six sessions.
- `.gitignore`: keeps `raw/` out of Git.

`VALIDATION.json` was not written, and no producer file was changed.
