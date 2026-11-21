# Offline tests of the capped RunPod route (draft Amendment 18)

`run_tests.sh` runs every case in a fresh sandbox copy of the route (its own git repository, receipts, ledger,
`RPC_STATE` and stub state) under `$TMPDIR`. No pod is created and no network call is made: `curl_stub.sh` is installed
as `curl` first on `PATH`, `HTTPS_PROXY` points at a dead local port, and in the A cases the stub fails every call, so
"0 API calls" means the refusal came before any call. `fixtures/smoke_tree/` is the results tree of the development
smoke pod (`cap:8:0:5`, t = 8, not data; its manifest equals `../smoke/receipts/local-sha256.txt`; its two `.log`
files are re-included by `../.gitignore`).

**Runs.** `bash tests/run_tests.sh` from `runpod_capped/` writes nothing in the repository and prints the results;
`bash tests/run_tests.sh --write` also replaces `tests/out/`, `tests/results.tsv` and this file.

**The image layer.** 46 of the 169 cases (launch, fetch, four reaper cases and five refusals that come after the
binary check) need the image layer: `RPC_LAYER_SRC` (the layer tarball, SHA-256 `a0f7e28e…`, `image.json`
`layer.digest`) and `RPC_BIN_SRC` (its `usr/local/bin`; defaults in the session scratchpad). Without them those
cases print `SKIP-NEEDS-LAYER`: a run without the layer gives 123 results and 46 skips; a run with
it gives 169 results. The layer is the study layer of the image `image/image.json` names (ECR); fetching it
by digest needs read access to that registry, which this container does not have.

Groups: A refusals before any API call; B launch paths; C `fetch.sh` flows; D the outcome classifier alone
(`fetch.sh --classify-dir`) on synthetic receipt sets: one per class, the review's conflicting cases, the memory rule
(D25-D37c), the round-2 cases (D38-D42) and the round-3 cases (D43-D44b); E the registered order (`next_allowed`); F `reap.sh`; G syntax and the
client pin. Per-case output: `out/<id>.txt`. Machine-readable table: `results.tsv`. Every result here is
`[pcc:unverified]`.

| id | result | case | expected | observed |
|:--|:--|:--|:--|:--|
| A01 | PASS | name not on the allow-list (pkm-x-t14-k0-c6-r9-anything) | exit 3, 0 API calls | exit 3, 0 API calls |
| A02 | PASS | a second relaunch name (-r2) | exit 3, 0 API calls | exit 3, 0 API calls |
| A03 | PASS | SHA_FILE set | exit 3, 0 API calls | exit 3, 0 API calls |
| A04 | PASS | receipts directory for the name already exists | exit 3, 0 API calls | exit 3, 0 API calls |
| A05 | PASS | results directory for the name already exists | exit 3, 0 API calls | exit 3, 0 API calls |
| A06 | PASS | order: t14-k1 requested before t14-k0 | exit 3, 0 API calls | exit 3, 0 API calls |
| A07 | PASS | order: -r1 requested with no base run | exit 3, 0 API calls | exit 3, 0 API calls |
| A08 | PASS | order: -r1 requested when the base is ADMITTED | exit 3, 0 API calls | exit 3, 0 API calls |
| A09 | PASS | order: -m112 requested when the base is ADMITTED without oversize | exit 3, 0 API calls | exit 3, 0 API calls |
| A10 | PASS | order: t14-k1 requested while t14-k0 is in flight (receipts, no outcome) | exit 3, 0 API calls | exit 3, 0 API calls |
| A11 | PASS | stage differs from the request (--max-dense 27900000000 on a 56Gi name) | exit 3, 0 API calls | exit 3, 0 API calls |
| A12 | PASS | '## Amendment 18' not in PREREGISTRATION.md | exit 3, 0 API calls | exit 3, 0 API calls |
| A13 | PASS | rp_pinned.sh modified (one line appended) | exit 3, 0 API calls | exit 3, 0 API calls |
| A14 | PASS | runpod/server.py modified (one line appended) | exit 3, 0 API calls | exit 3, 0 API calls |
| A15 | PASS | pfd_grid_zeros with one byte appended | exit 3, 0 API calls | exit 3, 0 API calls |
| A16 | PASS | RPC_LAYER missing for a study name | exit 3, 0 API calls | exit 3, 0 API calls |
| A17 | PASS | ledger complete_as_of 'never' (stale) | exit 3, 0 API calls | exit 3, 0 API calls |
| A18 | PASS | ledger with a malformed row | exit 3, 0 API calls | exit 3, 0 API calls |
| A19 | PASS | RPC_STATE inside the git worktree | exit 3, 0 API calls | exit 3, 0 API calls |
| A20 | PASS | development stage with t = 12 | exit 3, 0 API calls | exit 3, 0 API calls |
| A21 | PASS | a name with two earlier pre-start failures | exit 3, 0 API calls | exit 3, 0 API calls |
| A22 | PASS | -m112 request file edited (seconds 90001) | exit 3, 0 API calls | exit 3, 0 API calls |
| A23 | PASS | order: t13 -r3 requested while t14-k1 has not run | exit 3, 0 API calls | exit 3, 0 API calls |
| A24 | PASS | order: -r1 after the -r1 was lost (no third attempt) | exit 3, 0 API calls | exit 3, 0 API calls |
| A25 | PASS | order: t13 -r3 ABORTED has no relaunch (all resolved) | exit 3, 0 API calls | exit 3, 0 API calls |
| A26 | PASS | dev name with an existing receipts_dev dir | exit 3, 0 API calls | exit 3, 0 API calls |
| A27 | PASS | N6: the name already has a ledger row (receipts lost) | exit 3, 0 API calls | exit 3, 0 API calls |
| A28 | PASS | S3: heading present but the route scripts' hashes are not in the preregistration | exit 3, 0 API calls | exit 3, 0 API calls |
| A29 | PASS | N1: -r1 after an ABORTED base (the attempt is used) | exit 3, 0 API calls | exit 3, 0 API calls |
| A30 | PASS | S3: image.json layer digest disagrees with the pin in common.sh | exit 3, 0 API calls | exit 3, 0 API calls |
| B01 | PASS | a pkm-x- pod is live (serial rule) | exit 3, no create | exit 3, creates 0 |
| B02 | PASS | cpu5m out of stock: no attempt | exit 10, no create, no receipts dir | exit 10, creates 0, dir no |
| B03 | PASS | price 1.34/h x 30 h (the one worst-case pod life) > US$40 | exit 3, no create | exit 3, creates 0 |
| B04 | PASS | ledger US$370.01 + worst case US$31.20 > US$400 | exit 3 (study ceiling) | exit 3, creates 0 |
| B05 | PASS | create response not in the reserved tier (machine.secureCloud false) | exit 4, delete logged then made, receipts -> .prestart-1, ledger row closed | exit 4, pods left 0 |
| B06 | PASS | costPerHr 0.98 differs from the price read before create (1.04) | exit 4, pod deleted | exit 4, pods left 0 |
| B07 | PASS | create call timed out but the pod exists (trap set before create) | exit 4, pod found by name and deleted | exit 4, pods left 0 |
| B08 | PASS | create refused by the API, no pod exists | exit 4, receipts -> .nocreate-*, name free | exit 4 |
| B09 | PASS | runner would start more than 55 min after creation | exit 5, pod deleted, not an attempt | exit 5 |
| B10 | PASS | development launch end to end against the stub | exit 0; token and key absent from receipts and create argv; cgroup limit read before start; ledger row at 30 h; push reminder | exit 0; X-Token argv lines (rp_pinned.sh, disclosed): 13 |
| B11 | PASS | study launch pkm-x-t14-k0-c6, then pkm-x-t14-k1-c6 | first study run starts (exit 0); the next one is refused while it is in flight (exit 3) | exit 0 then 3 |
| B12 | PASS | the pod start command parses and carries the pinned server.py | bash -n ok; embedded runner sha256 f56fd08e... | bash -n 0; embedded sha256 f56fd08e |
| B13 | PASS | the boot loop copies cgroup v2 files and writes its final copy after runner_state.json | memory.max, memory.events and final-at copied | final-at fstype memory.events memory.max proc-self-cgroup snapshot-at  |
| B14 | PASS | S4: cgroup memory.max 60,000,000,000 bytes (< 112 GiB) | exit 4 before start, pod deleted | exit 4 |
| B15 | PASS | S4: no cgroup memory limit file before start (unobservable: owner) | exit 4 before start, pod deleted | exit 4 |
| B16 | PASS | N6: receipts and token lost after a launch, pod gone: the same name is refused | exit 0, then exit 3 (ledger row) | exit 0 then 3 |
| C01 | PASS | running before the deadline | exit 41, pod kept, one check-in with lastStatusChange | exit 41 |
| C02 | PASS | completed run: collect, wait for the raw upload, delete, refuse re-collection | exit 8 ADMITTED; empty record exit 8; full record exit 0; re-run exit 9; classifier pinned in outcome.json | exit 8 ADMITTED P2d; 8; 0; 9 |
| C03 | PASS | session abort while running | exit 0, ABORTED P5, pod deleted | exit 0 ABORTED P5 |
| C04 | PASS | runner idle after its start (container restarted) | exit 0, LOST P3, relaunch owner (no RunPod-side status change) | exit 0 LOST P3 owner |
| C05 | PASS | runner unreachable, REST EXITED with a RunPod-side lastStatusChange | exit 0, LOST P3, relaunch auto | exit 0 LOST P3 auto |
| C06 | PASS | unreachable twice 2 h apart, REST healthy RUNNING: wait | exit 41, pod kept | exit 41 |
| C07 | PASS | still running at the run deadline | exit 0, LIMIT P4, pod deleted | exit 0 LIMIT P4 |
| C08 | PASS | N2: unreachable at the run deadline, before the hard deadline | exit 41, pod kept (no information is not a loss) | exit 41 |
| C08b | PASS | N2: still unreachable at the hard deadline | exit 0, NO_EVIDENCE P7, pod deleted for cost | exit 0 NO_EVIDENCE P7 |
| C09 | PASS | runner done 90,050 s after its start | exit 8 (upload wait), LIMIT P2a | exit 8 LIMIT P2a |
| C10 | PASS | pilot exit 137 with cgroup oom_kill 1 | exit 8, OOM P2b | exit 8 OOM P2b |
| C11 | PASS | pilot exit 137 with oom_kill 0 | exit 8, LOST P2c, relaunch owner | exit 8 LOST P2c owner |
| C12 | PASS | N2: the two fetches differ while the runner is done (no information) | exit 6, pod kept, collection 1 abandoned, nothing kept | exit 6 |
| C13 | PASS | owner decision 13: pod gone (not listed, GET 404) before any logged session action | exit 0, LOST P3, relaunch auto, cost upper bound | exit 0 LOST P3 auto |
| C14 | PASS | S1: lastStartedAt changed while the runner reports running under s0 | exit 41, pod kept | exit 41 |
| C15 | PASS | a failed delete is resumed by the next fetch.sh call | exit 7 (still listed), then exit 0 on the next call | exit 7 then 0 |
| C16 | PASS | launcher died before start (no start.json, runner idle) | exit 0, NOT_AN_ATTEMPT, receipts -> .prestart-1 | exit 0 |
| C17 | PASS | N6: the pod token file is missing | exit 41, token recovered into RPC_STATE only | exit 41 |
| C18 | PASS | N3b: fetch.sh died after keeping the tree, before collect-1.json | exit 8, orphan recovered (verified), ADMITTED P2d, no second collection | exit 8 ADMITTED P2d |
| C18b | PASS | N3b: an orphan that disagrees with a later collection of the same pod goes to the owner | orphan kept (unverifiable), new collection 2, trees differ: NO_EVIDENCE P0b | exit 8 NO_EVIDENCE P0b |
| C19 | PASS | S2: a truncated check-in line (fail closed) | exit 12 (owner), pod kept | exit 12 |
| D01 | PASS | no runner start on record | NOT_AN_ATTEMPT P1 | NOT_AN_ATTEMPT P1 |
| D02 | PASS | completed, every admission rule holds | ADMITTED P2d | ADMITTED P2d |
| D03 | PASS | completed 90,050 s after the runner start | LIMIT P2a | LIMIT P2a |
| D04 | PASS | running at the deadline check-in | LIMIT P4 | LIMIT P4 |
| D05 | PASS | runner idle before the deadline, no RunPod-side status change | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D06 | PASS | S1: lastStartedAt changed while the runner runs under s0: not evidence | PENDING P8 | PENDING P8 |
| D06b | PASS | lastStartedAt changed and the runner is idle | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D07 | PASS | N2: unreachable twice >= 1 h apart, REST unreadable (HTTP 500): no information | PENDING P8 | PENDING P8 |
| D07b | PASS | REST desiredStatus EXITED with a RunPod-side status change | LOST P3 relaunch=auto | LOST P3 relaunch=auto |
| D08 | PASS | unreachable twice >= 1 h apart, REST healthy: wait | PENDING P8 | PENDING P8 |
| D08b | PASS | N2 probe: session network down at two check-ins 2 h apart (live null, REST 0, list null) | PENDING P8 | PENDING P8 |
| D09 | PASS | session abort while running | ABORTED P5 | ABORTED P5 |
| D10 | PASS | pilot exit 137, oom_kill 1 | OOM P2b | OOM P2b |
| D11 | PASS | pilot exit 137, oom_kill 0 | LOST P2c relaunch=owner | LOST P2c relaunch=owner |
| D12 | PASS | pilot exit 137, no cgroup files | LOST P2c relaunch=owner | LOST P2c relaunch=owner |
| D13 | PASS | pilot exit 1 (a tool failure, not a loss) | NOT_ADMITTED P2e | NOT_ADMITTED P2e |
| D14 | PASS | restart first seen at the deadline check-in | LOST P6 relaunch=owner | LOST P6 relaunch=owner |
| D14b | PASS | running at the deadline, restarted afterwards | LIMIT P4 | LIMIT P4 |
| D15 | PASS | N2: unreachable through the hard deadline | NO_EVIDENCE P7 | NO_EVIDENCE P7 |
| D16 | PASS | completed, receipts fetched, then restarted | ADMITTED P2d | ADMITTED P2d |
| D16b | PASS | restart recorded before the completion was fetched | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D17 | PASS | --abort on a run that is already done cannot change its class | ADMITTED P2d | ADMITTED P2d |
| D18 | PASS | --abort after a restart is LOST, not ABORTED | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D19 | PASS | two start lines in progress.txt (a second runner start) | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D20 | PASS | the runner's start differs from start.json (manual re-start) | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D21 | PASS | N2: completed in time but the transfer check still failed at the hard deadline | NO_EVIDENCE P7 | NO_EVIDENCE P7 |
| D22 | PASS | a file of the kept tree changed after collection (rule 6) | NOT_ADMITTED P2e | NOT_ADMITTED P2e |
| D23 | PASS | create response not in the reserved tier (rule 7) | NOT_ADMITTED P2e | NOT_ADMITTED P2e |
| D24 | PASS | owner decision 13: pod gone from the list and GET 404, before any logged session action | LOST P3 relaunch=auto | LOST P3 relaunch=auto |
| D24c | PASS | pod gone and GET 404 after a logged session delete | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D24d | PASS | pod absent from the list but GET returns a healthy RUNNING record: no loss evidence, wait | PENDING P8 | PENDING P8 |
| D24b | PASS | S6: missing from the list but GET unreadable (not corroborated) | PENDING P8 | PENDING P8 |
| D43 | PASS | R1 probe P1: an earlier RunPod-side change, then a restart whose status names the user | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D43b | PASS | R1: a RunPod-side change after the last alive check-in and before the loss | LOST P3 relaunch=auto | LOST P3 relaunch=auto |
| D43c | PASS | R1: the only RunPod-side change was seen while the runner was still alive | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D44 | PASS | R2 probe P2: runner seen done, then gone with GET 404 before the run deadline | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D44b | PASS | R2 probe P3: runner seen done, then gone with GET 404 after the run deadline | LOST P6 relaunch=owner | LOST P6 relaunch=owner |
| D25 | PASS | memory: pilot 30,720 MiB (30 GiB), no cgroup file -> pilot, below 56 GiB | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB |
| D26 | PASS | memory: pilot 71,680 MiB (70 GiB), no cgroup file -> pilot, between 56 and 112 GiB, oversize false: the cell's -m112 row | ADMITTED P2m3 as_m112=true source=pilot_log_VmHWM_MiB | ADMITTED P2m3 as_m112=true source=pilot_log_VmHWM_MiB |
| D27 | PASS | memory: pilot 128,000 MiB (125 GiB), no cgroup file -> pilot, above 112 GiB: OOM at -m112 | OOM P2m1 as_m112=true source=pilot_log_VmHWM_MiB | OOM P2m1 as_m112=true source=pilot_log_VmHWM_MiB |
| D28 | PASS | memory: pilot 64 MiB and cgroup 70 GiB -> pilot (primary); cgroup ignored | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB |
| D28b | PASS | memory: no pilot line, cgroup v2 70 GiB -> cgroup fallback | ADMITTED P2m3 as_m112=true source=cgroup_v2_memory.peak | ADMITTED P2m3 as_m112=true source=cgroup_v2_memory.peak |
| D29 | PASS | memory: no pilot line, no cgroup file -> none | NOT_ADMITTED P2u as_m112=false source=none | NOT_ADMITTED P2u as_m112=false source=none |
| D30 | PASS | memory: no pilot line, cgroup v1 max_usage 60 GiB -> v1 fallback | ADMITTED P2m3 as_m112=true source=cgroup_v1_memory.max_usage_in_bytes | ADMITTED P2m3 as_m112=true source=cgroup_v1_memory.max_usage_in_bytes |
| D31 | PASS | memory: -m112 attempt, pilot 115,712 MiB (113 GiB) -> OOM (owner decision 15) | OOM P2p as_m112=false source=pilot_log_VmHWM_MiB | OOM P2p as_m112=false source=pilot_log_VmHWM_MiB |
| D32 | PASS | memory: -m112 attempt, pilot 102,400 MiB (100 GiB), no cgroup file -> pilot | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB |
| D33 | PASS | memory: pilot exactly 57,344 MiB (56 GiB, not above), no cgroup file -> pilot | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB |
| D34 | PASS | memory: pilot 70 GiB, exit 137, oom_kill 0 -> pilot: a lost -m112 attempt | LOST P2m2 as_m112=true source=pilot_log_VmHWM_MiB | LOST P2m2 as_m112=true source=pilot_log_VmHWM_MiB |
| D35 | PASS | memory: cgroup 120 GiB (> 112 GiB) but pilot 92,160 MiB (90 GiB) -> pilot: not OOM, the cell's -m112 row | ADMITTED P2m3 as_m112=true source=pilot_log_VmHWM_MiB | ADMITTED P2m3 as_m112=true source=pilot_log_VmHWM_MiB |
| D35b | PASS | memory: -m112 attempt, cgroup 120 GiB but pilot 90 GiB -> pilot: P2p does not fire | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB |
| D36 | PASS | memory: pilot 30 GiB but exit 137 with oom_kill 1 -> OOM whatever the peak | OOM P2b as_m112=false source=pilot_log_VmHWM_MiB | OOM P2b as_m112=false source=pilot_log_VmHWM_MiB |
| D37 | PASS | N4: pilot 70 GiB with row oversize true: not the -m112 attempt; Amendment 14's oversize path (-m112 follows) | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB |
| D37b | PASS | N4: pilot 125 GiB with row oversize true: the oversize row, as Amendment 14 says | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB |
| D37c | PASS | boundary: pilot exactly 112 GiB (not above), oversize false | ADMITTED P2m3 as_m112=true source=pilot_log_VmHWM_MiB | ADMITTED P2m3 as_m112=true source=pilot_log_VmHWM_MiB |
| D38 | PASS | N3a: status change that names the user | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D39 | PASS | N3a: RunPod-side change recorded only after the session logged a delete | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D40 | PASS | S2: a truncated check-in line fails closed | NO_EVIDENCE P0 | NO_EVIDENCE P0 |
| D40b | PASS | S2: an unparsable collection record fails closed | NO_EVIDENCE P0 | NO_EVIDENCE P0 |
| D41 | PASS | N3b: two done collections of the pod disagree file by file | NO_EVIDENCE P0b | NO_EVIDENCE P0b |
| D41b | PASS | N3b: two done collections agree | ADMITTED P2d | ADMITTED P2d |
| D42 | PASS | S2: a zero-byte start.json (no response) is read as absent; the start comes from the check-in | PENDING P8 | PENDING P8 |
| E01 | PASS | nothing run yet | pkm-x-t14-k0-c6 | pkm-x-t14-k0-c6 |
| E02 | PASS | base LOST with host-side evidence (relaunch auto) | pkm-x-t14-k0-c6-r1 | pkm-x-t14-k0-c6-r1 |
| E03 | PASS | N1: base ABORTED uses the attempt; no relaunch | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E04 | PASS | base ADMITTED, no oversize | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E05 | PASS | base ADMITTED with oversize true | pkm-x-t14-k0-c6-m112 | pkm-x-t14-k0-c6-m112 |
| E06 | PASS | base OOM | pkm-x-t14-k0-c6-m112 | pkm-x-t14-k0-c6-m112 |
| E07 | PASS | base OOM, -m112 LOST (auto), relaunch unused | pkm-x-t14-k0-c6-m112-r1 | pkm-x-t14-k0-c6-m112-r1 |
| E08 | PASS | base LOST, -r1 OOM, -m112 LOST: relaunch already used | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E09 | PASS | base LIMIT (censored, not re-run) | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E10 | PASS | base NOT_ADMITTED (owner) | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E11 | PASS | both t14 cells resolved | pkm-x-t13-k0-c6-r3 | pkm-x-t13-k0-c6-r3 |
| E12 | PASS | t13 -r3 LOST: no relaunch | ALL_RESOLVED | ALL_RESOLVED |
| E13 | PASS | t13 -r3 OOM | pkm-x-t13-k0-c6-m112 | pkm-x-t13-k0-c6-m112 |
| E14 | PASS | base in flight | BLOCKED:pkm-x-t14-k0-c6 | BLOCKED:pkm-x-t14-k0-c6 |
| E15 | PASS | t13 -m112 LOST: no relaunch for t13 | ALL_RESOLVED | ALL_RESOLVED |
| E16 | PASS | base ADMITTED as the -m112 attempt (peak 56-112 GiB): cell resolved | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E17 | PASS | base OOM as the -m112 attempt (peak > 112 GiB): censored, no -m112 run | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E18 | PASS | base LOST (auto) as the -m112 attempt: the -m112 relaunch follows | pkm-x-t14-k0-c6-m112-r1 | pkm-x-t14-k0-c6-m112-r1 |
| E19 | PASS | base LOST, -r1 LOST as the -m112 attempt: relaunch used | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E20 | PASS | t13 -r3 LOST as the -m112 attempt: no relaunch | ALL_RESOLVED | ALL_RESOLVED |
| E21 | PASS | N3a: base LOST without host-side evidence (relaunch owner) | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E22 | PASS | N2: base NO_EVIDENCE goes to the owner | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E23 | PASS | N4: as_m112 with row oversize true is not the -m112 attempt (the review's case) | pkm-x-t14-k0-c6-m112 | pkm-x-t14-k0-c6-m112 |
| E24 | PASS | -m112 ABORTED: no -m112-r1 | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| F01 | PASS | reaper: 27 h old pod without receipts, 1 h old pod, other session's pod | pod past creation + 26 h deleted (logged first); young and non-pkm-x pods kept; second pass changes nothing | exit 0, left ["young1","other1"]; exit 0, left ["young1","other1"]; 3 receipt lines |
| F08 | PASS | S1: a due pod whose per-name lock is held by another process | pod untouched, skip recorded | exit 0, pods 1 |
| F03 | PASS | reaper: pod list unreadable (network blocked) | exit 1 | exit 1 |
| F07 | PASS | S6: unparsable createdAt and a null name: reported, not deleted | exit 1, nothing deleted | exit 1, pods 2 |
| F02 | PASS | reaper: a run past its hard deadline, with receipts | pod collected by fetch.sh (LIMIT P4) and deleted | exit 0 LIMIT P4 |
| F04 | PASS | reaper: launcher died, no deadline recorded, creation + 26 h passed | pod deleted through fetch.sh, NOT_AN_ATTEMPT | exit 0 |
| F05 | PASS | N3c, the review's probe: start.json missing, runner running, created 90 min ago | pod untouched; the next fetch.sh waits (41) | exit 0, pods 1, fetch 41 |
| F06 | PASS | N3c: past the run deadline but before the hard deadline, runner unreachable | pod untouched | exit 0, pods 1 |
| G-common.sh | PASS | syntax of common.sh | bash -n ok | rc 0 |
| G-run.sh | PASS | syntax of run.sh | bash -n ok | rc 0 |
| G-fetch.sh | PASS | syntax of fetch.sh | bash -n ok | rc 0 |
| G-reap.sh | PASS | syntax of reap.sh | bash -n ok | rc 0 |
| G-rp_pinned.sh | PASS | syntax of rp_pinned.sh | bash -n ok | rc 0 |
| G-make_requests_m112.sh | PASS | syntax of make_requests_m112.sh | bash -n ok | rc 0 |
| G-run_tests.sh | PASS | syntax of tests/run_tests.sh | bash -n ok | rc 0 |
| G-curl_stub.sh | PASS | syntax of tests/curl_stub.sh | bash -n ok | rc 0 |
| G-pin | PASS | pinned client copy unchanged | rp_pinned.sh sha256 b4b60199... | b4b60199 |

Summary of this run (image layer available: 1): 169 passed, 0 failed, 0 SKIP-NEEDS-LAYER (run 2026-11-15T03:38:33Z).
