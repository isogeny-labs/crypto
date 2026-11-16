# Assessment of the PROTOCOL-R3 launch cards (execution only; interpretation: not_assessed)

Cards assessed (all under `research/split_prime_ecdlp_20261107/followups/p1_rr_dickson/cards/`):

| card_id | card sha256 | receipt sha256 (now) |
|---|---|---|
| CARD-p1-r3-launch (RR a1, then T a1) | 72ba074ff4816b5cee1e03d60783ae7a3777e2aa9b06e1302fddbeef7c23124d (equals receipt `card_sha256`) | 43e3846907081d10fd1993d8aec47d14dd544567e10ac1e6412daaa8ff89eb28 |
| CARD-p1-r3-launch-rr-a2 | a1332ceec00fb07388e43fec5f6f9dff90bd008d547d6b6318c138590b2ee21c (equals) | 110b9d21eb07417176127dccb9150d9516db832c844790c457c99641b9e81434 |
| CARD-p1-r3-launch-t-a2 | 907502cf8fddf7bd5a5be9785ebba0a2e6815e4515a76d85f30b95c7c2d90893 (equals) | ae1a1922275df7587c882f34a479f51b0f986751ff5ee1e6feb8ddfec395efd3 |
| CARD-p1-r3-launch-rr-a3 | e7260b99a240577f8c3173f6fa5fb546ab917956a623852103b5da0acc0a5157 (equals) | e46ec2d96486aed8723e88fdf6844931d70a0b30c8a78bf1ab0e7688700daf9f |

relay_verbatim: not_assessed (no executor text was supplied to this assessment; `check-relay` was not run). The cards ran through the harness (receipts and `*.launch.log` on disk).
outcome (cards): executed_as_specified (submission step only; see the classification below for the Jobs)
stopped_at_step: null
recommendation: proceed_to_analysis
interpretation: not_assessed

## 1. Receipts: status and exit codes

All four receipts have schema `crypto.autoresearch.task-card-receipt.v1`, status `completed`, every command `exit_code` 0, `attempts` 1, `timed_out` false, each against a `timeout_seconds` of 3600.

| card | commands (in receipt order) | seconds | receipt `commit` |
|---|---|---|---|
| CARD-p1-r3-launch | 1 launch-rr (request-rr-a1.json), 2 launch-t (request-t-a1.json) | 678.725, 676.664 | 3976c3ffd0d1... |
| CARD-p1-r3-launch-rr-a2 | 1 launch-rr (request-rr-a2.json) | 678.471 | 9bd0d0d9c8c6... |
| CARD-p1-r3-launch-t-a2 | 1 launch-t (request-t-a2.json) | 675.531 | e6941cb25cd... |
| CARD-p1-r3-launch-rr-a3 | 1 launch-rr (request-rr-a3.json) | 664.481 | 0f3aacdd50f8... |

- Each receipt argv equals the card argv (`launch-remote-command <request>`), and the order in the first card is RR then T, as the card says. `stderr_sha256` is e3b0c442... (empty) in all five command records.
- The `outputs` and `stdout_sha256` digests of the five `*.launch.stdout` files re-hash equal now (rr-a1 9b6eb367..., t-a1 ae2cf098..., rr-a2 f236a2a1..., t-a2 ab54ef82..., rr-a3 474ce1d0...).
- The receipt `commit` of each card equals the card's `commit` field. Each is also the commit whose README sha256 equals that card's README bind (checked with `git show <commit>:launch/README.md | sha256sum`: 177b2e42..., 28e7d909..., d7189b20..., d7a79296...).

## 2. Binds re-hashed now

The 9 or 10 binds of each card were re-hashed with sha256sum (checked by a script that reads the card YAML).

- Equal in all four cards: PROTOCOL-R3.md (6f433d83...), image.json, build-info.json, the request JSON(s), make-requests.sh (rr-a3 card), `check-experiment-git.json` (first card), the three CloudTrail eviction JSONs (rr-a1, t-a1, rr-a2) bound by the later cards, and `launch-remote-command`, `run-command`, `current-run-request-buildspec.yml`.
- **Changed after the run: `launch/README.md`, in all four cards.** Current sha256 is 4c18b2b2dc41b738c5fdb90b42bd24e8e2d707695743309c8e93a13e1a1e7a6f. It differs from every card's bind (177b2e42..., 28e7d909..., d7189b20..., d7a79296...). Each bound value was the file's content at that card's commit, and the README is the attempt log that kept being appended after each run (last commit touching it: 6450954fc7). `python3 tools/task_card.py lint` on the first card therefore now prints `CARD FAIL ... binds[1] ... README.md does not match its sha256`. That is expected from the later edit. No other bound file changed.
- must_not_exist: the paths were absent at start per each receipt's `dirty_before` (only the card YAML and its launch.log are listed).

## 3. Launcher result.json against the attempt log in launch/README.md

`launch/receipts/<attempt>/result.json` agrees with the printed result in `<attempt>.launch.stdout` (the stdout adds two leading lines, "Launcher:" and "Launcher receipts:", and the JSON body is identical, checked for rr-a1 by diff). `request.json` in each attempt directory re-hashes to the `request_sha256` in result.json.

| attempt | state | job_uid | pod uid | README says |
|---|---|---|---|---|
| rr-a1 | active | 4107ff5e-3043-4d51-93c4-bdf33dd56a6d | ce76e299-1aab-4fbc-8cfa-8903d2e38ea6 (Running, node ip-10-84-27-98) | pod ce76e299... (README gives no job uid for a1) |
| t-a1 | active | 539991bb-51ab-4ad3-9db8-5ac1b1173e94 | 117e17e4-94e9-4791-96cb-7db7be84f0c9 (Pending) | pod 117e17e4... |
| rr-a2 | active | 4c60cc68-f18a-4ff3-a0fa-8f8dedb4b9d2 | 45447107-a243-4c17-a083-ac025324a742 (Pending) | job 4c60cc68..., pod 45447107... |
| t-a2 | active | 8fce259e-0d10-4bc4-995f-6c29c3eed183 | 828468c7-4c6d-401e-b547-6af2f9224aac (Pending) | job 8fce259e..., pod 828468c7... |
| rr-a3 | active | 718e6d4b-36a3-4e29-b43b-5fe1ee5b3a6e | 4459cae0-2f16-4fe1-95cc-aae4ce1bdae7 (Running, node ip-10-84-19-215) | job 718e6d4b..., pod 4459cae0... |

All five match the README prefixes. The pod UID in each pod-record directory's `env.txt` (`POD_UID`) equals the result.json pod uid, and `ISOLAB_JOB_ID` equals the job name. All five requests carry the image `.../isolab/tasks@sha256:e636a363fd437b224e6fb27363df712fe1473bff57b341caefb33a38190a710e`, which `image.json` records as `image_uri`. Requests: cpu 2; memory 64Gi (RR) / 60Gi (T); `seconds` 100800; `retries` 0. The rr-a3 request alone adds `instance_type: r6a.4xlarge` (as the README says).

## 4. Pod records: provenance, build identity, limits

For each of the five record directories:

- `provenance.txt` `ms_sha256` equals `want` (RR 9ba376d81c05109b11102c645153f9204df233600fec5851993a6bd5787e3cff, T ec0af27e8c8bec63160f1c69ced941a05379b79535942db20ce32409f836590e).
- The three sha256 lines in `provenance.txt` equal `launch/build-info.json` in every directory: msolve c2722288d22c3a1eab00c0b818d0204d4a98104d059b2fd4ecb7343e8ef759fa, p1tool aa8a4b238054b0fa2281243fbbc065e86a39c2788b3b41c37d8653859629e6c8, main.rs 7e50f8c82e7bc981c180e402e90c20d94ca60d4cf73cd22aaee02c15efb84ed7. Each directory's `build-info.json` is byte-identical to `launch/build-info.json` (`cmp`).
- `VLIMIT` / `TLIMIT` in provenance equal PROTOCOL-R3.md and `job-r3.sh`:

| directory | system | run_start_utc | VLIMIT | TLIMIT | host CPU (host.txt) | node (host.txt) |
|---|---|---|---|---|---|---|
| rr-a1-results-partial | rr | 2026-11-08T21:00:31Z | 62914560 | 86400 | Intel Xeon 6975P-C | ip-10-84-27-98 |
| t-a1-results-partial | t | 2026-11-08T21:34:31Z | 58720256 | 86400 | AMD EPYC 9R14 | ip-10-84-108-1 |
| rr-a2-results-partial | rr | 2026-11-09T08:36:19Z | 62914560 | 86400 | AMD EPYC 9R45 | ip-10-84-108-140 |
| t-a2-results | t | 2026-11-09T02:38:44Z | 58720256 | 86400 | Intel Xeon Platinum 8488C | ip-10-84-12-132 |
| rr-a3-results-partial | rr | 2026-11-09T23:36:07Z | 62914560 | 86400 | AMD EPYC 7R13 | ip-10-84-19-215 |

- Recorded, not explained: `nproc` prints 1 in all five `host.txt`; `cpu.max` is `max 100000`; memory.max is 68719476736 (RR) and 64424509440 (T). `msolve_processes_at_start` is empty in all five. In t-a2 the provenance uptime line reads "up 3:48" with load average 6.04, 4.45, 4.51 at 02:38:44Z; the other four read "up 0 min" (rr-a1 shown: load 0.36).
- `runner-identity.json` is the assumed role `isolab-job` in all five (account 104186811093).

## 5. t-a2-results

- `cd t-a2-results && sha256sum -c SHA256SUMS`: all 13 listed files print OK (build-info.json, env.txt, host.txt, job-exit.json, job.log, runner-identity.json, t/provenance.txt, t/t_s2.emit.err, t/t_s2.emit.json, t/t_s2.log, t/t_s2.ms, t/t_s2.ms.plant.json, t/t_s2.parse.json).
- `t/t_s2.gb` is present on disk with size 0 but is not listed in SHA256SUMS; it hashes to e3b0c442... (the empty file). Not verified by the sums file.
- `job-exit.json`: `exit_code` 0, `system` t, `job_name` p1r3-t-a2-20261108, `pod_uid` 828468c7-4c6d-401e-b547-6af2f9224aac (equals result.json), `node_name` ip-10-84-12-132 (equals host.txt), `finished_at` 2026-11-09T08:33:10Z. `job.log` last line: "2026-11-09T08:33:10Z finished exit 0".
- `t/t_s2.parse.json` as recorded: status `cap`, D 9, D_is_lower_bound true, exit 139, widest_cols 1305345, widest_rows 847295, wall_s 21266.672415409, rounds 33. `t_s2.log` last line "exit=139 wall=21266.672415409", preceded by "timeout: the monitored command dumped core". The `.ms` equals the T `want` hash (ec0af27e...).
- run_start 02:38:44Z plus wall 21266.7 s gives 08:33:11Z, consistent with the finish time.

## 6. Partial directories (Spot-lost attempts)

None of the four has a `job-exit.json` (`ls` fails in each), `job.log` and the msolve log are both last-modified at the same second, and none has a `SHA256SUMS` file.

| directory | last line of the `*_s2.log` | job.log / log mtime | eviction JSON `instanceIdSet` | `eventTime` (BidEvictedEvent, ec2.amazonaws.com, us-east-2) |
|---|---|---|---|---|
| rr-a1-results-partial | `7   99936  104632  464562 x 390983      0.73%` | 2026-11-08 21:36:02Z | i-0c270755aeadb845c | 2026-11-08T21:35:35Z |
| t-a1-results-partial | `9   64344  380141  847295 x 1305345     0.28%` | 2026-11-08 22:51:30Z | i-038d43f9a77c1938d | 2026-11-08T22:51:20Z |
| rr-a2-results-partial | `7   99936  104632  464562 x 390983      0.73%` | 2026-11-09 11:09:53Z | i-02123e2feff6642a7 | 2026-11-09T11:09:30Z |
| rr-a3-results-partial | `7   99936  104632  464562 x 390983      0.73%` | 2026-11-10 00:29:09Z | i-0fd9aae738b9308b5 | 2026-11-10T00:28:45Z |

The instance ids and event times equal those in the README attempt log. The eviction JSONs have no pod or node field, so the link from instance id to pod is the README's statement and the matching times (not verifiable from the eviction file alone). The eviction events' `eventID`s are bdad2096..., 184137b2..., 999d3b1e..., 62702c81....

## Notes and deviations

1. **Concurrency.** PROTOCOL-R3 says the two Jobs run at the same time on separate nodes. On disk: RR a1 ran 21:00:31Z to the eviction at 21:35:35Z and T a1 started 21:34:31Z, so they overlapped about one minute. T a2 ended 08:33:10Z and RR a2 started 08:36:19Z; RR a3 started 23:36:07Z with no T pod. The README records why (a `cpu-spot` node-limit event, which I did not verify). The README also records that the NodePool limit raise was not applied.
2. **Budget.** PROTOCOL-R3 still reads "ceiling of USD 60"; its hash is unchanged. The README records the owner's later authorization (monthly budget exhausted, ceiling lifted, at most three attempts per system and two Jobs at a time). That authorization is not on disk apart from the README text. No cost figure is on disk for the attempts, so spend is unverified.
3. **RR a3 instance pin.** The request adds `instance_type: r6a.4xlarge`; PROTOCOL-R3 names no instance type, and the README gives the Spot Advisor reading as the reason (not verifiable from disk). The host CPU model differs in all five attempts (table in section 4).
4. **Timing in the README for RR a3.** README says "Ran from 23:39Z" and "49 min in"; `provenance.txt` says run_start 23:36:07Z, which makes 52.6 min to the eviction at 00:28:45Z. The 23:39 value is the launcher's return time (664.481 s run).
5. **Not run by any card.** The `smoke/`, `status-*` request/result directories and the `*-spot-eviction.cloudtrail.json` files, and the S3 sync into `*-results*`, were produced by actions that none of the four cards lists. They are outside the card argv, so I cannot say from the cards which tool produced them. The status requests are the README's source for the "Job failed, no pod" statements (the `status-*` directories were not opened in this assessment).
6. **Receipt path quirk.** In three receipts (rr-a2, t-a2, rr-a3) `dirty_after` lists `esearch/...` for the `build.json` path, and in the first receipt `dirty_before` lists `esearch/.../p1-run-r3-launch.yaml`; the leading `r` is missing. That is a tooling artefact in the receipt text, not a path written outside `write_scope`.
7. **write_scope.** Current `git status --porcelain` lists: modified `followups/README.md` and `p1_rr_dickson/RESULTS.md`; untracked `benchmarks/records/20261110T0327Z-split-prime-p1-r3-m5t4-eks.json`, `analysis/derived_r3/`, `analysis/p1r3analyze/`, `figures/plot_r3.py`. None of these appears in any receipt's `dirty_after`; they post-date the receipts and are not outputs of the card commands. All launcher outputs of the four cards sit under `launch/receipts/` and `cards/` (inside `write_scope`).
8. **Not verified (no access or out of the requested scope).** Whether the S3 objects match the local synced copies (no S3 read); AWS billing; the Karpenter node-limit event and Pending durations; the Spot Advisor reading; the contents of the `status-*` and `smoke/` directories; `check-experiment-git.json` content beyond its hash and header; the full tails of the first receipt's `dirty_after`; `PROTOCOL-R3.md` rules beyond the VLIMIT/TLIMIT values and the quoted lines.

## Classification

- The four launch cards: every command ran once, in card order, exit 0, within 3600 s; stdout and request/result identities agree; frozen inputs unchanged except `launch/README.md` (changed after each run).
- Pod-side: instance identity (`ms_sha256 == want`), build identity (msolve, p1tool, main.rs) and VLIMIT/TLIMIT equal the frozen values in all five attempts.
- Job outcomes as recorded (no meaning assigned): T a2 completed its job script (exit 0) with msolve at cap, exit 139, D 9, widest_cols 1305345. T a1, RR a1, RR a2 and RR a3 ended by Spot eviction with no `job-exit.json`.
- Classification: **executed as specified, with deviations 1 to 4 above (concurrency of the two Jobs, budget wording, RR a3 instance pin, README time rounding) and the post-run README change; no executor deviation found in the card argv or write scope. interpretation: not_assessed.** No RR vs T comparison or rule-3 label is made here.
