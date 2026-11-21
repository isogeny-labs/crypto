# Offline tests of the capped RunPod route (Amendment 33 draft, REST v2)

`run_tests.sh` runs every case in a fresh sandbox copy of the route (its own git repository with a local bare
`origin`, receipts, ledger, `RPC_STATE` and stub state) under `$TMPDIR`. No pod is created and no network call is
made: `curl_stub.sh` is installed as `curl` first on `PATH` and answers in the RunPod REST v2 shapes of
`../V2_FIELDS.md` (pod list envelope with cursor pages, create with HTTP 201, GET, DELETE with 204 or 404, the CPU
catalog) and as the pod proxy; it has no route for REST v1 or GraphQL. `HTTPS_PROXY` points at a dead local port, and in
the A cases the stub fails every call, so "0 API calls" means the refusal came before any call.
`fixtures/smoke_tree/` is the results tree of the first development smoke pod (`cap:8:0:5`, t = 8, not data). `fixtures/ledger_fixture.tsv` (one row of US$500) stands in for the ledger, so no case reads the live `ledger.tsv`.

**Runs.** `bash tests/run_tests.sh` from `runpod_capped/` writes nothing in the repository and prints the results;
`bash tests/run_tests.sh --write` also replaces `tests/out/`, `tests/results.tsv` and this file.

**The image layer.** 61 of the 204 cases (launch, fetch, four reaper cases and five refusals that come after the
binary check) need the image layer: `RPC_LAYER_SRC` (the layer tarball, SHA-256 `a0f7e28e…`, `image.json`
`layer.digest`) and `RPC_BIN_SRC` (its `usr/local/bin`; defaults in the session scratchpad). Without them those
cases print `SKIP-NEEDS-LAYER`: a run without the layer gives 143 results and 61 skips; a run with
it gives 204 results.

Groups: A refusals before any API call (allow-list, order, the one stage cap:14:0:7:54000000000, the pinned request); B launch paths on the 32 vCPU / 256 GB pod (v2 create
body and checks, the study guard, pagination); C `fetch.sh` flows, with the git gate in C02-C02c; D the outcome
classifier alone (`fetch.sh --classify-dir`) on synthetic receipt sets with v2 field names, with the memory rule of owner decision 24 in D25-D37; R the reading at cap 7; E the order
(`next_allowed`); F `reap.sh` (F10, F11: a fresh `RPC_STATE`, the independent reaper of owner decision 25); W `watch.sh` (owner decision 26); G syntax, pins, the request and the absence of v1 calls. Per-case output:
`out/<id>.txt`. Machine-readable table: `results.tsv`. Every result here is `[pcc:unverified]`.

| id | result | case | expected | observed |
|:--|:--|:--|:--|:--|
| A01 | PASS | a draft Amendment 18 name, no longer on the allow-list (pkm-x-t14-k0-c6) | exit 3, 0 API calls | exit 3, 0 API calls |
| A02 | PASS | a second relaunch name (-r2) | exit 3, 0 API calls | exit 3, 0 API calls |
| A03 | PASS | SHA_FILE set | exit 3, 0 API calls | exit 3, 0 API calls |
| A04 | PASS | receipts directory for the name already exists | exit 3, 0 API calls | exit 3, 0 API calls |
| A05 | PASS | results directory for the name already exists | exit 3, 0 API calls | exit 3, 0 API calls |
| A06 | PASS | order: -r1 requested with no base run | exit 3, 0 API calls | exit 3, 0 API calls |
| A07 | PASS | order: -r1 requested when the base is ADMITTED | exit 3, 0 API calls | exit 3, 0 API calls |
| A08 | PASS | order: -r1 requested when the base is LOST with relaunch owner | exit 3, 0 API calls | exit 3, 0 API calls |
| A09 | PASS | order: -r1 requested while the base is in flight (receipts, no outcome) | exit 3, 0 API calls | exit 3, 0 API calls |
| A10 | PASS | stage cap 7 at the -m112 --max-dense for the base name (only cap:14:0:7:54000000000) | exit 3, 0 API calls | exit 3, 0 API calls |
| A11 | PASS | stage cap 6 at --max-dense 54e9 for the base name | exit 3, 0 API calls | exit 3, 0 API calls |
| A12 | PASS | '## Amendment 33' not in PREREGISTRATION.md | exit 3, 0 API calls | exit 3, 0 API calls |
| A13 | PASS | rp_pinned.sh modified (one line appended) | exit 3, 0 API calls | exit 3, 0 API calls |
| A14 | PASS | server_pinned.py modified (one line appended) | exit 3, 0 API calls | exit 3, 0 API calls |
| A15 | PASS | pfd_grid_zeros with one byte appended | exit 3, 0 API calls | exit 3, 0 API calls |
| A16 | PASS | RPC_LAYER missing for a study name | exit 3, 0 API calls | exit 3, 0 API calls |
| A17 | PASS | ledger complete_as_of 25 h old (stale) | exit 3, 0 API calls | exit 3, 0 API calls |
| A18 | PASS | ledger with a malformed row | exit 3, 0 API calls | exit 3, 0 API calls |
| A19 | PASS | RPC_STATE inside the git worktree | exit 3, 0 API calls | exit 3, 0 API calls |
| A20 | PASS | development stage with t = 12 | exit 3, 0 API calls | exit 3, 0 API calls |
| A21 | PASS | a name with two earlier pre-start failures | exit 3, 0 API calls | exit 3, 0 API calls |
| A22 | PASS | request file edited (seconds 90001): sha256 differs from the pin | exit 3, 0 API calls | exit 3, 0 API calls |
| A22b | PASS | request file edited and the pin updated: differs from the derivation (Amendments 7 B, 14) | exit 3, 0 API calls | exit 3, 0 API calls |
| A23 | PASS | order: -r1 after the -r1 was lost (no third attempt) | exit 3, 0 API calls | exit 3, 0 API calls |
| A24 | PASS | N1: -r1 after an ABORTED base (the attempt is used) | exit 3, 0 API calls | exit 3, 0 API calls |
| A25 | PASS | dev name with an existing receipts_dev dir | exit 3, 0 API calls | exit 3, 0 API calls |
| A26 | PASS | N6: the name already has a ledger row (receipts lost) | exit 3, 0 API calls | exit 3, 0 API calls |
| A27 | PASS | S3: heading present but the route scripts' hashes are not in the preregistration | exit 3, 0 API calls | exit 3, 0 API calls |
| A28 | PASS | S3: image.json layer digest disagrees with the pin in common.sh | exit 3, 0 API calls | exit 3, 0 API calls |
| A29 | PASS | development stage with --max-dense 999 | exit 3, 0 API calls | exit 3, 0 API calls |
| A30 | PASS | a draft Amendment 18 name (pkm-x-t14-k1-c6) | exit 3, 0 API calls | exit 3, 0 API calls |
| A31 | PASS | a draft Amendment 18 name (pkm-x-t13-k0-c6-r3) | exit 3, 0 API calls | exit 3, 0 API calls |
| A32 | PASS | draw 1 of the same cap (needs a later owner decision) | exit 3, 0 API calls | exit 3, 0 API calls |
| A33 | PASS | the retired name of owner decision 23 (pkm-x-t14-k0-c7-m112) | exit 3, 0 API calls | exit 3, 0 API calls |
| B01 | PASS | a pkm-x- pod is live (serial rule) | exit 3, no create | exit 3, creates 0 |
| B02 | PASS | catalog availability NONE for cpu5m at 16 vCPU: no attempt | exit 10, no create, no receipts dir | exit 10, creates 0, dir no |
| B03 | PASS | price 0.066 x 32 = 2.112/h x 30.8 h = US$65.05 > US$65 | exit 3 (per-job guard), no create | exit 3, creates 0 |
| B04 | PASS | fixture ledger US$500 + 30.94, + worst case US$64.064 (2.08 x 30.8 h) = 595.004 > US$595 | exit 3 (study guard) | exit 3, creates 0 |
| B04b | PASS | fixture ledger US$500 + 30.93, + worst case US$64.064 = 594.994 <= US$595 | exit 0 (started) | exit 0, creates 1 |
| B05 | PASS | create response cloud COMMUNITY (not the SECURE tier) | exit 4, delete logged then made, receipts -> .prestart-1, ledger row closed | exit 4, pods left 0 |
| B06 | PASS | create response cost 0.98 differs from the catalog price (0.065 x 32 = 2.08) | exit 4, pod deleted | exit 4, pods left 0 |
| B07 | PASS | create call timed out but the pod exists (trap set before create) | exit 4, pod found by name and deleted | exit 4, pods left 0 |
| B08 | PASS | create refused by the API (HTTP 400), no pod exists | exit 4, receipts -> .nocreate-*, name free | exit 4 |
| B09 | PASS | runner would start more than 55 min after creation | exit 5, pod deleted, not an attempt | exit 5 |
| B10 | PASS | development launch end to end against the v2 stub | exit 0; v2 create body keys only; token and key absent from receipts and create argv; command and env dropped from saved records; cgroup limit read before start; data center read when ready; ledger row at 30 h; no v1 or GraphQL call | exit 0; X-Token argv lines (rp_pinned.sh, disclosed): 13 |
| B11 | PASS | study launch pkm-x-t14-k0-c7-d54, then pkm-x-t14-k0-c7-d54-r1 | the study run starts with stage cap:14:0:7:54000000000 and the pinned request (exit 0); its -r1 is refused while it is in flight (exit 3) | exit 0 then 3 |
| B12 | PASS | the pod start command parses and carries server_pinned.py (= runpod/server.py at d2a16c723d) | bash -n ok; embedded runner sha256 f56fd08e... | bash -n 0; embedded sha256 f56fd08e |
| B13 | PASS | the boot loop copies cgroup v2 files and writes its final copy after runner_state.json | memory.max, memory.events and final-at copied | final-at fstype max_map_count memory.events memory.max proc-self-cgroup snapshot-at  |
| B14 | PASS | S4: cgroup memory.max 255,999,999,999 bytes (one byte under the 256,000,000,000 floor) | exit 4 before start, pod deleted | exit 4 |
| B15 | PASS | S4: no cgroup memory limit file before start (unobservable: owner) | exit 4 before start, pod deleted | exit 4 |
| B16 | PASS | N6: receipts and token lost after a launch, pod gone: the same name is refused | exit 0, then exit 3 (ledger row) | exit 0 then 3 |
| B17 | PASS | GET record cpu.memory 128 GB, below the 256 GB floor | exit 4 before start, pod deleted | exit 4 |
| B18 | PASS | the record read when the runner answers has no data center id | exit 4 before upload, pod deleted | exit 4 |
| B19 | PASS | v2 list pagination: a live pkm-x- pod on page 2 (serial rule) | exit 3, the second page was read | exit 3, creates 0 |
| B20 | PASS | create answered 202 (queued) instead of 201: not accepted | exit 4, pod deleted | exit 4 |
| B02b | PASS | catalog availability HIGH with an empty dataCenters list counts as no stock (review 4 M5e) | exit 10, no create | exit 10, creates 0 |
| B21 | PASS | study run on a pod whose memory.max reads max: refused (review 4 S3) | exit 4 before start, pod deleted | exit 4 |
| B22 | PASS | development run with memory.max max is accepted; the S1 settings are recorded (no rule reads them) | exit 0; memory.oom.group and max_map_count recorded in cgroup-prestart.json | exit 0 |
| B23 | PASS | a failed create's response body is copied to run.log only as redacted JSON (review 4 S13) | exit 4; run.log names the size of the non-JSON body; the token is in no receipt | exit 4 |
| C01 | PASS | running before the deadline | exit 41, pod kept, one check-in with v2 status and startedAt | exit 41 |
| C02 | PASS | completed run: collect, the git gate (decision 22), delete, refuse re-collection | exit 8 ADMITTED; gate: uncommitted 8, committed but not pushed 8, pushed 0; re-run 9 | exit 8 ADMITTED P2d; 8; 8; 0; 9 |
| C02b | PASS | git gate: a kept file changed after the commit (manifest mismatch) | exit 8, pod kept | exit 8 then 8 |
| C02c | PASS | git gate: the manifest receipt local-sha256-1.txt is not in the pushed commit | exit 8, pod kept | exit 8 then 8 |
| C03 | PASS | session abort while running | exit 0, ABORTED P5, pod deleted | exit 0 ABORTED P5 |
| C04 | PASS | runner idle after its start (container restarted) | exit 0, LOST P3, relaunch owner | exit 0 LOST P3 owner |
| C05 | PASS | runner unreachable, REST v2 status EXITED | exit 0, LOST P3, relaunch owner (v2 has no status-change text: fail closed) | exit 0 LOST P3 owner |
| C06 | PASS | unreachable twice 2 h apart, REST healthy RUNNING: wait | exit 41, pod kept | exit 41 |
| C07 | PASS | still running at the run deadline | exit 0, LIMIT P4, pod deleted | exit 0 LIMIT P4 |
| C08 | PASS | N2: unreachable at the run deadline, before the hard deadline | exit 41, pod kept (no information is not a loss) | exit 41 |
| C08b | PASS | N2: still unreachable at the hard deadline | exit 0, NO_EVIDENCE P7, pod deleted for cost | exit 0 NO_EVIDENCE P7 |
| C09 | PASS | runner done 90,050 s after its start | exit 8 (git gate wait), LIMIT P2a | exit 8 LIMIT P2a |
| C10 | PASS | pilot exit 137 with cgroup oom_kill 1 | exit 8, OOM P2b | exit 8 OOM P2b |
| C11 | PASS | pilot exit 137 with oom_kill 0 | exit 8, LOST P2c, relaunch owner | exit 8 LOST P2c owner |
| C12 | PASS | N2: the two fetches differ while the runner is done (no information) | exit 6, pod kept, collection 1 abandoned, nothing kept | exit 6 |
| C13 | PASS | owner decision 13: pod gone (not listed, v2 GET 404) before any logged session action | exit 0, LOST P3, relaunch auto, cost upper bound | exit 0 LOST P3 auto |
| C14 | PASS | S1: v2 startedAt changed while the runner reports running under s0 | exit 41, pod kept | exit 41 |
| C15 | PASS | a failed delete is resumed by the next fetch.sh call | exit 7 (still listed), then exit 0 on the next call | exit 7 then 0 |
| C16 | PASS | launcher died before start (no start.json, runner idle) | exit 0, NOT_AN_ATTEMPT, receipts -> .prestart-1 | exit 0 |
| C17 | PASS | N6: the pod token file is missing (v2 GET returns env) | exit 41, token recovered into RPC_STATE only | exit 41 |
| C18 | PASS | N3b: fetch.sh died after keeping the tree, before collect-1.json | exit 8, orphan recovered (verified), ADMITTED P2d, no second collection | exit 8 ADMITTED P2d |
| C18b | PASS | N3b: an orphan that disagrees with a later collection of the same pod goes to the owner | orphan kept (unverifiable), new collection 2, trees differ: NO_EVIDENCE P0b | exit 8 NO_EVIDENCE P0b |
| C19 | PASS | S2: a truncated check-in line (fail closed) | exit 12 (owner), pod kept | exit 12 |
| D01 | PASS | no runner start on record | NOT_AN_ATTEMPT P1 | NOT_AN_ATTEMPT P1 |
| D02 | PASS | completed, every admission rule holds (v2 receipts) | ADMITTED P2d | ADMITTED P2d |
| D03 | PASS | completed 90,050 s after the runner start | LIMIT P2a | LIMIT P2a |
| D04 | PASS | running at the deadline check-in | LIMIT P4 | LIMIT P4 |
| D05 | PASS | runner idle before the deadline | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D06 | PASS | S1: startedAt changed while the runner runs under s0: not evidence | PENDING P8 | PENDING P8 |
| D06b | PASS | startedAt changed and the runner is idle | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D07 | PASS | N2: unreachable twice >= 1 h apart, REST unreadable (HTTP 500): no information | PENDING P8 | PENDING P8 |
| D07b | PASS | v2 status EXITED: loss evidence; no v2 status-change text, so relaunch owner (fail closed) | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D07c | PASS | a v1-style lastStatusChange field in a check-in is ignored (never host-side evidence) | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
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
| D23 | PASS | create response cloud COMMUNITY (rule 7) | NOT_ADMITTED P2e | NOT_ADMITTED P2e |
| D23b | PASS | GET record cpu.memory 64, below the 256 GB floor (rule 7) | NOT_ADMITTED P2e | NOT_ADMITTED P2e |
| D23c | PASS | request record with an interruptible field (rule 7) | NOT_ADMITTED P2e | NOT_ADMITTED P2e |
| D23d | PASS | create response cost 2.09 differs from the catalog price 2.08 (rule 7) | NOT_ADMITTED P2e | NOT_ADMITTED P2e |
| D23e | PASS | no GET record after create: cpu.memory unobservable (rule 7 fails closed) | NOT_ADMITTED P2e | NOT_ADMITTED P2e |
| D24 | PASS | owner decision 13: pod gone from the list and v2 GET 404, before any logged session action | LOST P3 relaunch=auto | LOST P3 relaunch=auto |
| D24c | PASS | pod gone and GET 404 after a logged session delete | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D24d | PASS | pod absent from the list but GET returns a healthy RUNNING record: no loss evidence, wait | PENDING P8 | PENDING P8 |
| D24b | PASS | S6: missing from the list but GET unreadable (not corroborated) | PENDING P8 | PENDING P8 |
| D43 | PASS | R1: gone with GET 404 after the last alive check-in, before any session action | LOST P3 relaunch=auto | LOST P3 relaunch=auto |
| D43c | PASS | R1: gone with GET 404 only after the first loss (runner idle): not tied to the loss | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D44 | PASS | R2 probe P2: runner seen done, then gone with GET 404 before the run deadline | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D44b | PASS | R2 probe P3: runner seen done, then gone with GET 404 after the run deadline | LOST P6 relaunch=owner | LOST P6 relaunch=owner |
| D45 | PASS | v2 status ERROR | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D46 | PASS | v2 status STARTING after the runner's start (the container is starting again) | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D25 | PASS | memory: pilot 30 GiB under the 256,000,000,000-byte limit | ADMITTED P2d source=pilot_log_VmHWM_MiB | ADMITTED P2d source=pilot_log_VmHWM_MiB |
| D26 | PASS | memory: pilot 245,760 MiB (257,698,037,760 bytes) above memory.max 256,000,000,000: censored | OOM P2p source=pilot_log_VmHWM_MiB | OOM P2p source=pilot_log_VmHWM_MiB |
| D27 | PASS | memory: pilot 244,140 MiB (255,999,836,160 bytes), just under memory.max | ADMITTED P2d source=pilot_log_VmHWM_MiB | ADMITTED P2d source=pilot_log_VmHWM_MiB |
| D28 | PASS | memory: pilot 64 MiB and cgroup 250 GiB -> pilot (primary); cgroup ignored | ADMITTED P2d source=pilot_log_VmHWM_MiB | ADMITTED P2d source=pilot_log_VmHWM_MiB |
| D28b | PASS | memory: no pilot line, cgroup v2 peak 257,000,000,000 above memory.max -> cgroup fallback | OOM P2p source=cgroup_v2_memory.peak | OOM P2p source=cgroup_v2_memory.peak |
| D29 | PASS | memory: no pilot line, no cgroup peak file -> none | NOT_ADMITTED P2u source=none | NOT_ADMITTED P2u source=none |
| D30 | PASS | memory: no pilot line, cgroup v1 max_usage 100 GiB -> v1 fallback | ADMITTED P2d source=cgroup_v1_memory.max_usage_in_bytes | ADMITTED P2d source=cgroup_v1_memory.max_usage_in_bytes |
| D31 | PASS | memory: cgroup oom_kill 1 with exit 0 in the end lines -> OOM, censored | OOM P2k source=pilot_log_VmHWM_MiB | OOM P2k source=pilot_log_VmHWM_MiB |
| D32 | PASS | memory: a 112Gi-shaped stage with pilot 113 GiB is no longer OOM (owner decision 15's step removed) | ADMITTED P2d source=pilot_log_VmHWM_MiB | ADMITTED P2d source=pilot_log_VmHWM_MiB |
| D34 | PASS | memory: exit 137, oom_kill 0 | LOST P2c source=pilot_log_VmHWM_MiB | LOST P2c source=pilot_log_VmHWM_MiB |
| D36 | PASS | memory: exit 137 with oom_kill 1 -> OOM whatever the peak | OOM P2b source=pilot_log_VmHWM_MiB | OOM P2b source=pilot_log_VmHWM_MiB |
| D33 | PASS | memory: memory.max 'max' (no numeric limit) and pilot 300,000 MiB: P2p cannot fire | ADMITTED P2d | ADMITTED P2d |
| D37 | PASS | memory: no cgroup files in the tree; the limit read before start (cgroup-prestart.json) is used for P2p | OOM P2p | OOM P2p |
| R01 | PASS | refutes at cap 7 (inconsistent true) -> d_F = 7 | ADMITTED P2d d_F== 7 censored=- | ADMITTED P2d d_F== 7 censored=- |
| R02 | PASS | degree 7 completed without refuting, oversize and timed_out false -> d_F >= 8 | ADMITTED P2d d_F=>= 8 censored=- | ADMITTED P2d d_F=>= 8 censored=- |
| R03 | PASS | oversize at 54e9 (stop line oversize) -> censored, d_F >= 7 stands | ADMITTED P2d d_F=>= 7 censored=size | ADMITTED P2d d_F=>= 7 censored=size |
| R04 | PASS | stopped by the wall budget (stop line budget) -> censored | ADMITTED P2d d_F=>= 7 censored=budget | ADMITTED P2d d_F=>= 7 censored=budget |
| R05 | PASS | row oversize true but the stop line says budget -> no reading (owner) | ADMITTED P2d d_F=none censored=- | ADMITTED P2d d_F=none censored=- |
| R06 | PASS | degree 6 reached, nothing stopped it -> no reading (owner) | ADMITTED P2d d_F=none censored=- | ADMITTED P2d d_F=none censored=- |
| R07 | PASS | cgroup oom_kill 1 -> OOM, censored at the new size | OOM P2k d_F=>= 7 censored=memory | OOM P2k d_F=>= 7 censored=memory |
| R08 | PASS | exit 137 with oom_kill 1 -> OOM, censored | OOM P2b d_F=>= 7 censored=memory | OOM P2b d_F=>= 7 censored=memory |
| R09 | PASS | runner ended after the run deadline -> LIMIT, censored | LIMIT P2a d_F=>= 7 censored=time limit | LIMIT P2a d_F=>= 7 censored=time limit |
| R10 | PASS | review 4 S4: oversize true with no stop line (no contrary stop line) -> censored by size | ADMITTED P2d d_F=>= 7 censored=size | ADMITTED P2d d_F=>= 7 censored=size |
| R11 | PASS | review 4 S4: a deadline caught at the top of the step loop prints no stop line -> censored by the budget | ADMITTED P2d d_F=>= 7 censored=budget | ADMITTED P2d d_F=>= 7 censored=budget |
| D31b | PASS | review 4 S8: an OOM with the peak (30 GiB) under 90 percent of memory.max is annotated as a likely host-level or group kill | oom_note present | peak below 90 percent of memory.max: a h |
| D38 | PASS | runner idle and status EXITED | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D39 | PASS | status EXITED after the session logged a delete | LOST P3 relaunch=owner | LOST P3 relaunch=owner |
| D40 | PASS | S2: a truncated check-in line fails closed | NO_EVIDENCE P0 | NO_EVIDENCE P0 |
| D40b | PASS | S2: an unparsable collection record fails closed | NO_EVIDENCE P0 | NO_EVIDENCE P0 |
| D40c | PASS | S2: an unparsable GET record after create fails closed | NO_EVIDENCE P0 | NO_EVIDENCE P0 |
| D41 | PASS | N3b: two done collections of the pod disagree file by file | NO_EVIDENCE P0b | NO_EVIDENCE P0b |
| D41b | PASS | N3b: two done collections agree | ADMITTED P2d | ADMITTED P2d |
| D42 | PASS | S2: a zero-byte start.json (no response) is read as absent; the start comes from the check-in | PENDING P8 | PENDING P8 |
| E01 | PASS | nothing run yet | pkm-x-t14-k0-c7-d54 | pkm-x-t14-k0-c7-d54 |
| E02 | PASS | base LOST with host-side evidence (relaunch auto) | pkm-x-t14-k0-c7-d54-r1 | pkm-x-t14-k0-c7-d54-r1 |
| E03 | PASS | N3a: base LOST without host-side evidence (relaunch owner) | ALL_RESOLVED | ALL_RESOLVED |
| E04 | PASS | N1: base ABORTED uses the attempt; no relaunch | ALL_RESOLVED | ALL_RESOLVED |
| E05 | PASS | base ADMITTED | ALL_RESOLVED | ALL_RESOLVED |
| E06 | PASS | base ADMITTED with oversize true: censored at 54e9, no further size step | ALL_RESOLVED | ALL_RESOLVED |
| E07 | PASS | base OOM at the 256 GB pod: censored, no further size step | ALL_RESOLVED | ALL_RESOLVED |
| E08 | PASS | base LIMIT (censored, not re-run) | ALL_RESOLVED | ALL_RESOLVED |
| E09 | PASS | base NOT_ADMITTED (owner) | ALL_RESOLVED | ALL_RESOLVED |
| E10 | PASS | N2: base NO_EVIDENCE goes to the owner | ALL_RESOLVED | ALL_RESOLVED |
| E11 | PASS | base in flight | BLOCKED:pkm-x-t14-k0-c7-d54 | BLOCKED:pkm-x-t14-k0-c7-d54 |
| E12 | PASS | base LOST auto, relaunch in flight | BLOCKED:pkm-x-t14-k0-c7-d54-r1 | BLOCKED:pkm-x-t14-k0-c7-d54-r1 |
| E13 | PASS | base LOST auto, relaunch LOST auto: no third attempt | ALL_RESOLVED | ALL_RESOLVED |
| E14 | PASS | base LOST auto, relaunch ADMITTED | ALL_RESOLVED | ALL_RESOLVED |
| F01 | PASS | reaper: 27 h old pod without receipts, 1 h old pod, other session's pod | pod past creation + 26 h deleted through v2 (logged first); young and non-pkm-x pods kept; second pass changes nothing | exit 0, left ["young1","other1"]; exit 0, left ["young1","other1"]; 3 receipt lines |
| F08 | PASS | S1: a due pod whose per-name lock is held by another process | pod untouched, skip recorded | exit 0, pods 1 |
| F03 | PASS | reaper: pod list unreadable (network blocked) | exit 1 | exit 1 |
| F03b | PASS | review 4 / owner decision 25: an unreadable pod list is reported, not silent | message names the blind pass | REAPER BLIND: the RunPod pod list (GET https://api.runpod.io |
| F11 | PASS | owner decision 25: reaper in a fresh checkout with an RPC_STATE that does not exist yet (no token, no lock files) | exit 0, pod deleted, RPC_STATE created | exit 0, pods 0 |
| F07 | PASS | S6: unparsable createdAt and a null name: reported, not deleted | exit 1, nothing deleted | exit 1, pods 2 |
| F09 | PASS | v2 list pagination in the reaper | the page-2 pod is seen and a delete is sent (the stub keeps page 2, so it stays listed: exit 7) | exit 7 |
| F02 | PASS | reaper: a run past its hard deadline, with receipts | pod collected by fetch.sh (LIMIT P4) and deleted | exit 0 LIMIT P4 |
| F04 | PASS | reaper: launcher died, no deadline recorded, creation + 26 h passed | pod deleted through fetch.sh, NOT_AN_ATTEMPT | exit 0 |
| F05 | PASS | N3c, the review's probe: start.json missing, runner running, created 90 min ago | pod untouched; the next fetch.sh waits (41) | exit 0, pods 1, fetch 41 |
| F06 | PASS | N3c: past the run deadline but before the hard deadline, runner unreachable | pod untouched | exit 0, pods 1 |
| F10 | PASS | owner decision 25: the hourly reaper in a fresh checkout, a run past its hard deadline | exit 0; fetch.sh recovers the token from the v2 record into the new RPC_STATE, collects and deletes | exit 0 LIMIT P4 |
| W01 | PASS | a running job: watch.sh calls fetch.sh at its interval until MAX_WALL_S | exit 20 (segment over), pod kept, two or more check-ins | exit 20, check-ins 3 |
| W02 | PASS | a completed job: watch.sh stops for the git gate and deletes nothing | exit 8 at once (collected; the git gate is the session's), pod kept | exit 8 ADMITTED P2d |
| W04 | PASS | past the hard deadline: fetch.sh collects and deletes; watch.sh ends | exit 0, LIMIT P4, deleted by fetch.sh | exit 0 LIMIT P4 |
| W03 | PASS | watch.sh refuses an interval above 900 s | exit 2 | exit 2 |
| W05 | PASS | watch.sh refuses a segment longer than 2 h | exit 2 | exit 2 |
| G-common.sh | PASS | syntax of common.sh | bash -n ok | rc 0 |
| G-run.sh | PASS | syntax of run.sh | bash -n ok | rc 0 |
| G-fetch.sh | PASS | syntax of fetch.sh | bash -n ok | rc 0 |
| G-reap.sh | PASS | syntax of reap.sh | bash -n ok | rc 0 |
| G-watch.sh | PASS | syntax of watch.sh | bash -n ok | rc 0 |
| G-rp_pinned.sh | PASS | syntax of rp_pinned.sh | bash -n ok | rc 0 |
| G-make_requests_d54.sh | PASS | syntax of make_requests_d54.sh | bash -n ok | rc 0 |
| G-make_requests_m112.sh | PASS | syntax of make_requests_m112.sh | bash -n ok | rc 0 |
| G-run_tests.sh | PASS | syntax of tests/run_tests.sh | bash -n ok | rc 0 |
| G-curl_stub.sh | PASS | syntax of tests/curl_stub.sh | bash -n ok | rc 0 |
| G-pin | PASS | pinned client copy unchanged | rp_pinned.sh sha256 b4b60199... | b4b60199 |
| G-server | PASS | vendored runner = runpod/server.py at d2a16c723d | server_pinned.py sha256 f56fd08e... | f56fd08e |
| G-v2 | PASS | REST v2 port (comments excluded) | no REST v1 or GraphQL URL and no v1 field name in the route's code | 0 hits |
| G-rpuse | PASS | pod-proxy calls only through the pinned client | rp_pinned.sh used only for upload, start, status, fetch (its v1 create, pod, delete are never called) | 0 hits |
| G-request | PASS | the one request (Amendment 7 B, owner decision 24) | request sha256 = pin = make_requests output; equals its derivation; four changed paths | pin 089176f2 |

Summary of this run (image layer available: 1): 204 passed, 0 failed, 0 SKIP-NEEDS-LAYER (run 2026-11-21T23:33:55Z).
