# Offline tests of the capped RunPod route (draft Amendment 18)

`run_tests.sh` runs every case in a fresh sandbox copy of the route (its own git repository, receipts, ledger,
`RPC_STATE` and stub state) under `$TMPDIR`. No pod is created and no network call is made: `curl_stub.sh` is installed
as `curl` first on `PATH`, `HTTPS_PROXY` points at a dead local port, and in the A cases the stub fails every call, so
"0 API calls" means the refusal came before any call. `fixtures/smoke_tree/` is the results tree of the development
smoke pod (`cap:8:0:5`, t = 8, not data; its manifest equals `../smoke/receipts/local-sha256.txt`). The binaries come
from the image layer (`RPC_BIN_SRC`, `RPC_LAYER_SRC`, defaults in the session scratchpad).

Groups: A refusals before any API call; B launch paths against the stub; C `fetch.sh` flows against the stub; D the
outcome classifier alone (`fetch.sh --classify-dir`) on synthetic receipt sets, one per class plus the conflicting cases
of the review's M1, and the memory rule of amendment section 4.2 (D25-D34); E the registered order (`next_allowed`); F `reap.sh`; G syntax and the client pin.
Per-case output: `out/<id>.txt`. Machine-readable table: `results.tsv`. Every result here is `[pcc:unverified]`.

Run: `bash tests/run_tests.sh` from `runpod_capped/` (about one minute).

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
| B01 | PASS | a pkm-x- pod is live (serial rule) | exit 3, no create | exit 3, creates 0 |
| B02 | PASS | cpu5m out of stock: no attempt | exit 10, no create, no receipts dir | exit 10, creates 0, dir no |
| B03 | PASS | price 1.60/h x 26 h > US$40 | exit 3, no create | exit 3, creates 0 |
| B04 | PASS | ledger US$380.01 + worst case US$27.04 > US$400 | exit 3 (study ceiling) | exit 3, creates 0 |
| B05 | PASS | create response not in the reserved tier (machine.secureCloud false) | exit 4, pod deleted, receipts -> .prestart-1, ledger row closed | exit 4, pods left 0 |
| B06 | PASS | costPerHr 0.98 differs from the price read before create (1.04) | exit 4, pod deleted | exit 4, pods left 0 |
| B07 | PASS | create call timed out but the pod exists (trap set before create) | exit 4, pod found by name and deleted | exit 4, pods left 0 |
| B08 | PASS | create refused by the API, no pod exists | exit 4, receipts -> .nocreate-*, name free | exit 4 |
| B09 | PASS | runner would start more than 55 min after creation | exit 5, pod deleted, not an attempt | exit 5 |
| B10 | PASS | development launch end to end against the stub | exit 0; token and key absent from receipts and from the create call's argv; request record, deadlines, ledger row | exit 0; X-Token argv lines (rp_pinned.sh, disclosed): 12 |
| B11 | PASS | study launch pkm-x-t14-k0-c6, then pkm-x-t14-k1-c6 | first study run starts (exit 0); the next one is refused while it is in flight (exit 3) | exit 0 then 3 |
| B12 | PASS | the pod start command parses and carries the pinned server.py | bash -n ok; embedded runner sha256 f56fd08e... | bash -n 0; embedded sha256 f56fd08e |
| B13 | PASS | the boot loop copies cgroup v2 files and writes its final copy after runner_state.json | memory.max, memory.events and final-at copied | final-at fstype memory.events memory.max proc-self-cgroup snapshot-at  |
| C01 | PASS | running before the deadline | exit 41, pod kept, one check-in | exit 41 |
| C02 | PASS | completed run: collect, wait for the raw upload, delete, refuse re-collection | exit 8 ADMITTED; --delete with an empty record exit 8; with a full record exit 0; re-run exit 9 | exit 8 ADMITTED P2d; 8; 0; 9 |
| C03 | PASS | session abort while running | exit 0, ABORTED P5, pod deleted | exit 0 ABORTED P5 |
| C04 | PASS | runner idle after its start (container restarted) | exit 0, LOST P3, pod deleted | exit 0 LOST P3 |
| C05 | PASS | unreachable at two check-ins 2 h apart, REST record not readable (404) but still listed | exit 0, LOST P3 (unreachable rule) | exit 0 LOST P3 |
| C06 | PASS | unreachable twice 2 h apart, but the REST record is a healthy RUNNING pod (proxy outage only) | exit 41, pod kept | exit 41 |
| C07 | PASS | still running at the run deadline | exit 0, LIMIT P4, pod deleted | exit 0 LIMIT P4 |
| C08 | PASS | unreachable at the deadline (one reading) | exit 0, LOST P6, pod deleted although nothing could be fetched | exit 0 LOST P6 |
| C09 | PASS | runner done 90,050 s after its start | exit 0, LIMIT P2a (deleted: past the deadline) | exit 0 LIMIT P2a |
| C10 | PASS | pilot exit 137 with cgroup oom_kill 1 | exit 8, OOM P2b | exit 8 OOM P2b |
| C11 | PASS | pilot exit 137 with oom_kill 0 | exit 8, LOST P2c | exit 8 LOST P2c |
| C12 | PASS | the two fetches differ while the runner is done, before the deadline | exit 6, pod kept, nothing kept | exit 6 |
| C13 | PASS | pod gone from the API (not listed, 404) | exit 0, LOST P3, cost marked upper bound | exit 0 LOST P3 |
| C14 | PASS | pod lastStartedAt changed while the runner reports running | exit 0, LOST P3 | exit 0 LOST P3 |
| C15 | PASS | a failed delete is resumed by the next fetch.sh call | exit 7 (still listed), then exit 0 on the next call | exit 7 then 0 |
| C16 | PASS | launcher died before start (no start.json, runner idle) | exit 0, NOT_AN_ATTEMPT, receipts -> .prestart-1 | exit 0 |
| D01 | PASS | no runner start on record | NOT_AN_ATTEMPT P1 | NOT_AN_ATTEMPT P1 |
| D02 | PASS | completed, every admission rule holds | ADMITTED P2d | ADMITTED P2d |
| D03 | PASS | completed 90,050 s after the runner start | LIMIT P2a | LIMIT P2a |
| D04 | PASS | running at the deadline check-in | LIMIT P4 | LIMIT P4 |
| D05 | PASS | runner idle before the deadline | LOST P3 | LOST P3 |
| D06 | PASS | lastStartedAt changed before the deadline | LOST P3 | LOST P3 |
| D07 | PASS | unreachable twice >= 1 h apart, REST record unreadable (HTTP 500) | LOST P3 | LOST P3 |
| D07b | PASS | REST desiredStatus EXITED | LOST P3 | LOST P3 |
| D08 | PASS | unreachable twice >= 1 h apart, REST healthy: not lost | PENDING P7 | PENDING P7 |
| D08b | PASS | unreachable twice only 500 s apart, REST unreadable: not lost yet | PENDING P7 | PENDING P7 |
| D09 | PASS | session abort while running | ABORTED P5 | ABORTED P5 |
| D10 | PASS | pilot exit 137, oom_kill 1 | OOM P2b | OOM P2b |
| D11 | PASS | pilot exit 137, oom_kill 0 | LOST P2c | LOST P2c |
| D12 | PASS | pilot exit 137, no cgroup files | LOST P2c | LOST P2c |
| D13 | PASS | pilot exit 1 (a tool failure, not a loss) | NOT_ADMITTED P2e | NOT_ADMITTED P2e |
| D14 | PASS | M1 deadline vs lost: restart first seen at the deadline check-in | LOST P6 | LOST P6 |
| D14b | PASS | M1: running at the deadline, restarted afterwards | LIMIT P4 | LIMIT P4 |
| D15 | PASS | M1: one unreachable reading at the deadline | LOST P6 | LOST P6 |
| D16 | PASS | M1: completed, receipts fetched, then restarted | ADMITTED P2d | ADMITTED P2d |
| D16b | PASS | M1: restart recorded before the completion was fetched | LOST P3 | LOST P3 |
| D17 | PASS | M1: --abort on a run that is already done cannot change its class | ADMITTED P2d | ADMITTED P2d |
| D18 | PASS | M1: --abort after a restart is LOST, not ABORTED | LOST P3 | LOST P3 |
| D19 | PASS | M1: two start lines in progress.txt (a second runner start) | LOST P3 | LOST P3 |
| D20 | PASS | M1: the runner's start differs from start.json (manual re-start) | LOST P3 | LOST P3 |
| D21 | PASS | completed in time but the transfer check failed past the deadline | LOST P6 | LOST P6 |
| D22 | PASS | a file of the kept tree changed after collection (rule 6) | NOT_ADMITTED P2e | NOT_ADMITTED P2e |
| D23 | PASS | create response not in the reserved tier (rule 7) | NOT_ADMITTED P2e | NOT_ADMITTED P2e |
| D25 | PASS | memory: cgroup peak 30 GiB (below 56 GiB) | ADMITTED P2d as_m112=false source=cgroup_v2_memory.peak | ADMITTED P2d as_m112=false source=cgroup_v2_memory.peak |
| D26 | PASS | memory: cgroup peak 70 GiB (between 56 and 112 GiB): the cell's -m112 row | ADMITTED P2m3 as_m112=true source=cgroup_v2_memory.peak | ADMITTED P2m3 as_m112=true source=cgroup_v2_memory.peak |
| D27 | PASS | memory: cgroup peak 125 GiB (above 112 GiB): OOM at -m112 | OOM P2m1 as_m112=true source=cgroup_v2_memory.peak | OOM P2m1 as_m112=true source=cgroup_v2_memory.peak |
| D28 | PASS | memory: no cgroup peak, pilot VmHWM field present (64 MiB) | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB | ADMITTED P2d as_m112=false source=pilot_log_VmHWM_MiB |
| D28b | PASS | memory: no cgroup peak, pilot VmHWM field 70,000 MiB (> 56 GiB) | ADMITTED P2m3 as_m112=true source=pilot_log_VmHWM_MiB | ADMITTED P2m3 as_m112=true source=pilot_log_VmHWM_MiB |
| D29 | PASS | memory: neither a cgroup peak nor a pilot field | NOT_ADMITTED P2u as_m112=false source=none | NOT_ADMITTED P2u as_m112=false source=none |
| D30 | PASS | memory: cgroup v1 max_usage_in_bytes 60 GiB | ADMITTED P2m3 as_m112=true source=cgroup_v1_memory.max_usage_in_bytes | ADMITTED P2m3 as_m112=true source=cgroup_v1_memory.max_usage_in_bytes |
| D31 | PASS | memory: an -m112 attempt with peak 113 GiB | OOM P2p as_m112=false source=cgroup_v2_memory.peak | OOM P2p as_m112=false source=cgroup_v2_memory.peak |
| D32 | PASS | memory: an -m112 attempt with peak 100 GiB | ADMITTED P2d as_m112=false source=cgroup_v2_memory.peak | ADMITTED P2d as_m112=false source=cgroup_v2_memory.peak |
| D33 | PASS | memory: peak exactly 56 GiB (not above) | ADMITTED P2d as_m112=false source=cgroup_v2_memory.peak | ADMITTED P2d as_m112=false source=cgroup_v2_memory.peak |
| D34 | PASS | memory: peak 70 GiB and exit 137 without oom: a lost -m112 attempt | LOST P2m2 as_m112=true source=cgroup_v2_memory.peak | LOST P2m2 as_m112=true source=cgroup_v2_memory.peak |
| D24 | PASS | pod no longer listed (deleted outside the route) | LOST P3 | LOST P3 |
| E01 | PASS | nothing run yet | pkm-x-t14-k0-c6 | pkm-x-t14-k0-c6 |
| E02 | PASS | base LOST | pkm-x-t14-k0-c6-r1 | pkm-x-t14-k0-c6-r1 |
| E03 | PASS | base ABORTED (charged like LOST) | pkm-x-t14-k0-c6-r1 | pkm-x-t14-k0-c6-r1 |
| E04 | PASS | base ADMITTED, no oversize | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E05 | PASS | base ADMITTED with oversize true | pkm-x-t14-k0-c6-m112 | pkm-x-t14-k0-c6-m112 |
| E06 | PASS | base OOM | pkm-x-t14-k0-c6-m112 | pkm-x-t14-k0-c6-m112 |
| E07 | PASS | base OOM, -m112 LOST, relaunch unused | pkm-x-t14-k0-c6-m112-r1 | pkm-x-t14-k0-c6-m112-r1 |
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
| E18 | PASS | base LOST as the -m112 attempt: the -m112 relaunch follows | pkm-x-t14-k0-c6-m112-r1 | pkm-x-t14-k0-c6-m112-r1 |
| E19 | PASS | base LOST, -r1 LOST as the -m112 attempt: relaunch used | pkm-x-t14-k1-c6 | pkm-x-t14-k1-c6 |
| E20 | PASS | t13 -r3 LOST as the -m112 attempt: no relaunch | ALL_RESOLVED | ALL_RESOLVED |
| F01 | PASS | reaper: 27 h old pod without receipts, 1 h old pod, other session's pod | old pkm-x- pod deleted, young pkm-x- pod and non-pkm-x pod kept; second pass changes nothing | exit 0, left ["young1","other1"]; exit 0, left ["young1","other1"]; 3 receipt lines |
| F02 | PASS | reaper: a started run past its run deadline, with receipts and token | pod collected by fetch.sh (LIMIT P4) and deleted | exit 0 LIMIT P4 |
| F04 | PASS | reaper: receipts but no runner start 90 min after creation (launcher died) | pod deleted through fetch.sh, NOT_AN_ATTEMPT, receipts -> .prestart-1 | exit 0 |
| F03 | PASS | reaper: pod list unreadable (network blocked) | exit 1 | exit 1 |
| G-common.sh | PASS | syntax of common.sh | bash -n ok | rc 0 |
| G-run.sh | PASS | syntax of run.sh | bash -n ok | rc 0 |
| G-fetch.sh | PASS | syntax of fetch.sh | bash -n ok | rc 0 |
| G-reap.sh | PASS | syntax of reap.sh | bash -n ok | rc 0 |
| G-rp_pinned.sh | PASS | syntax of rp_pinned.sh | bash -n ok | rc 0 |
| G-make_requests_m112.sh | PASS | syntax of make_requests_m112.sh | bash -n ok | rc 0 |
| G-run_tests.sh | PASS | syntax of tests/run_tests.sh | bash -n ok | rc 0 |
| G-curl_stub.sh | PASS | syntax of tests/curl_stub.sh | bash -n ok | rc 0 |
| G-pin | PASS | pinned client copy unchanged | rp_pinned.sh sha256 b4b60199... | b4b60199 |

Summary: 127 passed, 0 failed (run 2026-11-15T00:35:28Z).
