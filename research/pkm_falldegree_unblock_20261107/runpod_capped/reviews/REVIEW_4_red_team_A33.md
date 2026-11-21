# Red-team review: draft Amendment 33 (runpod_capped, REST v2; probe `cap:14:0:7:54000000000`)

Reviewer: red-team (independent; I wrote none of this and had not reviewed it before). Date 2026-11-21.
Evidence reviewed: branch `claude/serene-davinci-ujoi58` at `a5c6d89147` ("retarget complete"), with the two later
development-smoke commits `88b64ac5d5` and `873847c5ed`. Those two commits change no route script: the section 10 hashes
still hold. The worktree under the reviewed paths is clean (`git status --porcelain` is empty). I edited no repository
file, made no commit, created or deleted no pod, and made no paid call. Every number below is `[pcc:unverified]`.

## Verdict: freeze with corrections

The job computes what the design says, and only that. I checked:

- the request derivation: exactly four changed paths, the re-derivation equals the pinned file, and the validator says
  VALID;
- the stage pin and `RAYON_NUM_THREADS = 12`;
- the frozen layer hashes;
- the runner and client pins (both equal the files at `d2a16c723d`).

The reading table matches the pilot's actual row fields. The classifier precedence, abort, NO_EVIDENCE, the host-side
relaunch rule, the reaper lock and the git gate all survived the v2 port. Nothing I found makes a reading wrong.

Five items must still be fixed or decided before the freeze:

- one false statement about the cost backstop;
- two frozen preconditions (Amendments 29 and 32) that the draft does not address;
- an unpriced ledger question that, if decided the other way, breaks the US$540 ceiling;
- a test count that cannot be reproduced at the branch head, plus frozen text that the smoke and the v2 API now
  contradict.

## MUST (before freeze)

**M1. The cost backstop shares the root cause of Amendment 32's loss. The draft says the opposite.** (wrong; unpriced)
- What the draft says. `AMENDMENT_R2_H_runpod_capped.md:140-141`: "Enforcement is by session check-ins and the scheduled
  reaper only (owner decision 14); Amendment 32's loss (pods nothing deleted) is the case this guards against."
- What Amendment 32 says. `PREREGISTRATION.md:1097-1098`: the pods overran because "the session that would delete it was
  down".
- Why the draft is wrong. Here both check-ins and `reap.sh` are run by the main session (`reap.sh:11`). A session outage
  therefore disables both, which is exactly Amendment 32's failure. The v2 API has no pod lifetime field: I checked the
  public spec `api.runpod.io/v2/openapi.json` 2.0.0, and `CreatePodRequest` has no TTL, stop-after or spot field. So
  nothing on the RunPod side stops the bill.
- What an outage costs. The guard is checked only at launch. During an outage the pod bills US$2.08/h without bound. At
  today's ledger, 465.93 + 2.08·h reaches the US$540 ceiling at about 35.6 h of pod life, which is 5.6 h after the
  "worst case".
- The draft also does not address Amendment 32's frozen precondition for any relaunch (`PREREGISTRATION.md:1100-1102`):
  "the client deletes each pod when its job ends, and no batch starts unless a check-in is scheduled within the shortest
  job limit". This route deletes the pod at the next session check-in after the job ends, which can be up to 4 h plus the
  2 h git-gate wait later.
- Owner decision 14 ("no enforcement independent of Claude sessions", 2026-11-15) predates the Amendment 32 loss
  (2026-11-18). Decision 24 did not revisit it.
- Fix:
  1. Correct the sentence.
  2. State how the route meets Amendment 32's two conditions.
  3. Either record the owner re-confirming decision 14 in the light of Amendment 32, or add a backstop that does not
     depend on the session. Options: a scheduled CI job that runs `reap.sh` with the key as a secret; or a boot-script
     self-stop at creation + 27 h, if a stopped (`EXITED`) pod stops compute billing.
- Cheapest test of the self-stop option: one development pod whose start command exits after 60 s. Then read
  `GET /pods/{id}` (status) and `GET /billing/pods` for the next 30 minutes.

**M2. Amendment 29 (frozen) is neither applied nor scoped out.** (unsupported)
- `PREREGISTRATION.md:982-986`: "No batch of pods starts unless the owner has confirmed the balance covers the batch's
  spend at its limits", and "every pod's results are fetched every 15 minutes".
- Neither the draft, `DECISIONS_ROUTE.md` nor decision 24 mentions either rule.
- Why it matters. The account is shared: the v2 list shows other projects' H200 pods priced at US$10.58/h. Amendment 29
  was written after an 11-pod loss attributed to a balance stop.
- Fix: obtain and record the owner's confirmation that the balance covers US$62.40, or scope Amendment 29 explicitly to
  its own route. Then either adopt the 15-minute fetch or state why it does not apply. A capped run's partial tree is
  never read, so it adds only loss evidence.

**M3. Owner ruling on UNKNOWN-5 before the freeze.** (unpriced)
- The ledger sum is 465.9320 at the branch head (smoke3 added 0.0896). Plus this job's worst case of 62.40 that is
  528.33, within the US$535 guard.
- `ledger_SOURCES.md:79-81` leaves two snapshot pods unattributed.
  - `bdr-growthlaw` (35.7037) looks like another study: `docs/session-logs/2026-11-15.md:4536` launched it under the
    benchmark record `20261115T1952Z-growth-law-cells-rerun`.
  - `pkm-dense-rerun-20261115` (37.2565) carries the study's prefix and nothing attributes it elsewhere.
- Counting that one pod alone gives 503.19 + 62.40 = 565.59, above both the guard and the US$540 ceiling.
- The draft discloses this (`:233-236`), but a freeze should not register a spend whose fit to an owner ceiling depends on
  an open question.
- Fix: put the attribution of `pkm-dense-rerun-20261115`, and of `bdr-growthlaw`, to the owner and record the answer.

**M4. "188/188 offline" cannot be reproduced at the branch head: I get 187/188.** (stated as checked, not reproducible)
- Command: `bash research/pkm_falldegree_unblock_20261107/runpod_capped/tests/run_tests.sh`, run plainly. Result: passed
  187, failed 1, 0 skipped.
- `B04b` fails: expected exit 0, observed exit 3.
- Cause. The test appends a fixed 6.75 to the live ledger (`tests/run_tests.sh:182-184`), so that 465.84 + 6.75 + 62.40
  comes to 534.99. Smoke3 added 0.0896 to the ledger, so the sum is now 535.08 > 535.
- The route's own behaviour is correct. The fixture is not hermetic.
- Fix: compute the increment from `ledger_sum` (or use a fixture ledger) in `B04` and `B04b`. Re-run at the commit that
  is frozen and cite that commit's count.

**M5. Text to be frozen that the smoke and the v2 API now contradict or make stale.** (wrong or stale)
- (a) `V2_FIELDS.md:31` is wrong. It says "A list entry has the same keys except `env` and `args`". A read-only
  `GET /v2/pods` today returned `env` (object) and `args` (string) in all 26 list entries, so every pod's `RUNNER_TOKEN`
  is in the list response. `list_pkm` projects each entry to six fields (`common.sh:116`), so nothing leaks. The document
  should still say so.
- (b) Stale floors that predate the d54 retarget:
  - `V2_FIELDS.md:41`: "`cpu5m`, 16, at least 121 GB (112 GiB)";
  - `V2_FIELDS.md:77`: "`memory.max` at least 112 GiB";
  - `run.sh:106-107`: comment says 16 vCPU (the code uses 32).
- (c) Section 0 and section 9.2 ("Not yet seen, to be recorded at a development smoke"), and `V2_FIELDS.md:72-81`:
  - smoke3 answered most of these: create 201; the GET record; `memory.max` 256,000,000,000; `DELETE` 204; gone from
    the list;
  - I add one more: a `GET` of the deleted pod returns 404 with body `{"detail":"pod not found",...}`;
  - still unobserved: token recovery from `env`, and `startedAt` and `status` on a container restart. Say so.
- (d) The 230.9 GiB "worst-case process peak under the cap" is called a worst case in three places: `:106-107`,
  `common.sh:40` and `MEMORY.md` section 5. It is not a bound. It assumes a basis of at most 24 GiB and the echelon of a
  step with `nq` at most 50,256. The echelon is not bounded by `--max-dense`, and degree-6 steps ranged `nq` 8,798 to
  83,044. Call it a central case (detail in S2).
- (e) `:92`: "If `cpu5m` at 32 vCPU shows no stock, `run.sh` exits 10".
  - The code exits 10 only when the catalog says `availability NONE` (`run.sh:122-123`).
  - At 20:45Z the catalog answered `HIGH` with an empty `dataCenters` list (`HOSTS.md` section 1). In that case the
    create fails instead (exit 4, `.nocreate`, no cost).
  - Today's GET lists only `EUR-IS-1`.
- (f) `:139-140`: "One worst-case pod life (S7): 30 h". Round-3 S2 still applies: a reaper pass can add about 50 min.
  2.08 × 30.8 = US$64.06 still fits the US$65 cap and the guard (529.99), but the text should say about 31 h.

## SHOULD

- **S1. Two kernel settings decide how an out-of-memory or allocation failure looks. Neither is recorded.**
  - `memory.oom.group`. If RunPod sets it to 1, an OOM kills the runner too. The container exits, and the result reads
    LOST P3 (relaunch owner) or NO_EVIDENCE, not OOM. The evidence goes with it, because the boot loop's last
    `memory.events` snapshot predates the kill. Smoke3's `memory.events` has an `oom_group_kill` counter, but the
    setting itself is not copied.
  - `vm.max_map_count` (hypothesis, not established).
    - `B'` is one `Vec<u32>` per pivot row (`f4_fp_tower.rs:1606,1679`). The static musl build uses the system
      allocator: no `global_allocator` in `pkm_tower_pilot`.
    - Rows above the allocator's large-allocation threshold (about 128 KiB) are separate `mmap`s. Step 35 has 685,892
      pivots averaging about 198 KB, so most of them qualify.
    - Degree-6 steps with about 220k long rows succeeded. Either the kernel merged mappings or the host limit is high
      (Ubuntu 24.04 kernels ship 1,048,576). A later step near 54·10⁹ could have about 1.08M rows.
    - A failed `mmap` aborts the pilot (exit 134). That reads P2e NOT_ADMITTED, not censored.
  - Fix: add `memory.oom.group`, `memory.swap.max` and `/proc/sys/vm/max_map_count` to `cgsnap` and to
    `cgroup-prestart.json`. This records data only and changes no rule. Then state in section 4.2 how a group OOM and an
    allocation abort are classified.
- **S2. The memory margin.** This answers the coordinator's question; see "Smoke3" below.
- **S3. `memory.max = max` is accepted** (`run.sh:276`). On a 256 GB pod that reports `max`, the design's headroom
  cannot be checked and P2p cannot fire. Refuse `max` for study names, since smoke3 showed a numeric limit, or justify
  accepting it.
- **S4. The reading text is stricter than the code.**
  - The table (`:74-75`) requires a stop line `oversize` or `budget`. The code accepts a missing stop line
    (`fetch.sh:258-259`, `// "oversize"` and `// "budget"`).
  - The code is right to do so. A deadline caught at the top of the step loop sets `timed_out` and prints no stop line
    (`f4_fp_tower.rs:2435-2438`).
  - Fix: write "and no contrary stop line" in the table. Add tests R09 and R10 for a missing stop line in both cases.
- **S5. Amendment 7 C (decomposable draws).** The reading ignores the gridzeros output. The `-m112` tree already gives
  `{"decomposable":false,"grid_zeros":0}` for t = 14 draw 0 (snapshot `X-m2-t14-k0-c7.gridzeros.jsonl`). Cite that in
  section 1.1, or require `decomposable false` in the d54 tree.
- **S6. The relaunch is nearly vacuous: disclose it.** `-r1` passes the guard only if the base attempt cost at most
  535 − 62.40 − 465.93 = US$6.67, that is a pod life of about 3.2 h or less.
- **S7. Pods the backstop cannot see or reach.**
  - When `GET /pods` fails, `reap.sh` exits 1 without acting (`reap.sh:19`). It never falls back to the receipts' known
    pod ids. `fetch.sh` at least sends `DELETE` before it confirms.
  - Round-3 S5 is still open. A pod whose pre-start delete was unconfirmed (exit 11; its receipts are moved to
    `.prestart-k`) waits until creation + 26 h.
  - A pod that appears after a timed-out create (the `.nocreate` path) has no ledger row. That leaves up to US$62.40
    unpriced.
- **S8. An OOM far below the limit means a host-level kill.** If `oom_kill > 0` with the peak well under `memory.max`,
  the kill came from the host, not from this run's size. Annotate such an outcome so the owner does not read it as "54·10⁹
  is too small".
- **S9. The trace-identity claims are slightly overstated.**
  - `MEMORY.md` section 2 says "both draws give identical step lines (every count, `B'` and nonzero figure)". Section 3
    of the amendment says "steps 1-34 [identical] on two CPU models and both draws".
  - In the snapshot logs, k0 and k1 differ in `muladds` at steps 28, 30, 31 and 32, and in `nnz` by one at step 32
    (5,206,676,542 against 5,206,676,543).
  - Rows, columns, `B'`, fresh, basis and pair counts are identical. Step 35's `B'` is 33,961,233,968 on both draws.
  - Fix: say "structurally identical".
- **S10. Token recovery from `GET .env.RUNNER_TOKEN` has never run** (round-3 S3). Exercise it once on a development
  pod.
- **S11. Hygiene in hashed or registered files.**
  - Stale comments: `common.sh:144` and `ledger.tsv:2` still say "guard US$510".
  - `ledger_SOURCES.md:83-88` repeats a paragraph, and line 81 is stale.
  - The request's inherited first note sentence still says "cap 6, --max-dense 12800000000". It is documentary, and
    the appended sentence explains it.
- **S12. Check main's amendment numbering before the freeze.** The local `origin/main` (`a80ec41ae5`) ends at Amendment
  32. The live main head `4122987c93` is not in this clone. A parallel Amendment 33 would force a re-hash, because
  `AMENDMENT_HEADING` lives in `common.sh`.
- **S13. One secret path is still open** (round-3 S9). On a failed create, if the response is not JSON, `run.sh:218`
  writes its raw first 500 bytes to `run.log`. If such a response echoed the request body, the pod token would land in
  a committed receipt.
- **S14. Time.**
  - The "fast exit" lower bound of 27,600 s ignores that filling the echelon needs at least `nq` = 50,256 reduced
    S-rows. That adds about 2,200 s.
  - The low band's upper end, 70,000 s, times the observed host spread of 1.27 is 89,000 s, past the 86,400 s budget.
    So even the low band can be censored on a slow host.
  - Both are disclosed in spirit in section 7. Making them explicit would help.

## Focus answers

**1. Does the job compute what the design says, and only that?** Yes.
- Request: `make_requests_d54.sh` output and my own diff show exactly these changes:
  - `job.command.1`: `cap:14:0:6:12800000000` becomes `cap:14:0:7:54000000000`;
  - `job.memory`: 56Gi becomes 238Gi;
  - `job.job_name`;
  - `provenance.note`: one sentence appended, 1,995 characters.
- `d54_request` sourced from `common.sh` re-derives SHA-256 `089176f2…`, equal to the pin. `validate_command_request.sh`
  reports both requests VALID. On RunPod the request is documentary: only `command[1]` is compared with the stage.
- Stage: only `cap:14:0:7:54000000000` is accepted for study names (`run.sh:38`). Admission rule 2 checks it in
  `host.txt`.
- Threads: `rp_pinned.sh start` sends `rayon:12`, and `server_pinned.py` sets `RAYON_NUM_THREADS`.
- Binaries: the layer files' SHA-256 equal `BIN_SHA_PINS`.
- Entry script: `entry_r2.sh` differs from `entry.sh` only by the `m4n:` branch and comments (I ran `diff`).
- `--max-dense` only gates. Its sole uses are `f4_fp_tower.rs:1578` (the check before `B'` is allocated) and the report
  field.
- Memory floor:
  - Code and MEMORY.md agree on 4 bytes per entry (`:1679`).
  - 4 × 54·10⁹ = 201.17 GiB; plus 36 GiB headroom, 237.17 GiB; that is 1.25 GiB under 256·10⁹ = 238.42 GiB.
- Can an OOM come before the oversize check?
  - Not at step 35. Before step 35 the peak is 64.8 GiB, and the check runs at RSS 11.8 GiB.
  - At a later degree-7 step, yes. It needs RSS before the step + `B'` (at most 201.2 GiB) + that step's echelon + about
    1-2 GiB to exceed 238.4 GiB, that is a basis plus state of roughly 27-32 GiB.
- How such an OOM is classified:
  - pilot alone killed: P2b OOM, censored, d_F ≥ 7 (tests C10, D10, R08 pass);
  - group OOM: LOST or NO_EVIDENCE, routed to the owner (S1);
  - allocation abort: P2e NOT_ADMITTED (S1).
  - None of these is a wrong reading.

**2. Is the projection sound enough for US$62.40? Is the reading table right?**
- I spot-checked the snapshot logs (`results/SNAPSHOT_20261117/pfd-x-t14-k{0,1}-c7-m112-rp/results.tar.gz`):
  - the step 35 line: `B'` 33,961,233,968; residue `0 x 50256`; memory 11,821 MB (peak 64,823);
  - the row: `degree_reached 7`, `inconsistent false`, `oversize true`, `timed_out true`, `wall_s 22598.3`;
  - three peaks: step 27 at 9,844 MiB, step 28 at 25,562 MiB, step 30 at 64,823 MiB.
- The model `RSS_before + 4·B' + 4·fresh·nq` reproduces 25,425 MiB at step 28 and 65,073 MiB at step 30 exactly (with
  c₀ = 0). The residuals are +137, −250 and +567 MiB.
- The A-phase and B-phase unit ranges reproduce (step 30: A 4.66e-12, B 2.99e-11; step 34: A 1.065e-11), and so do the
  step-35 projections (A 4,870-11,130 s; B 27,300-32,200 s). `project.py` reproduces every figure in the documents.
- How much rests on what:
  - Step 35 is deterministic and fits with about 90 GiB to spare.
  - Everything after step 35 rests on four degree-6 analogues, one of them without a trace. The design says so.
- Reading table:
  - It is correct for the row fields. The pilot writes `timed_out: r.timed_out || r.oversize` at
    `pkm_tower_pilot.rs:1506`. The stop words are at `:1187` and `:1189`. The classifier tests `oversize` before the
    budget case, and checks `oversize false` and the stop line for the budget case.
  - `inconsistent` and `oversize` are mutually exclusive: refutation breaks the loop at `:2793`, and oversize is an
    `Err` from `eliminate`.
  - `degree_reached` is the maximum degree reached (`:2448`). Smoke3 confirms it: it refuted at a degree-4 step with
    `degree_reached 5`.
- The one-draw limit is disclosed: section 1.1, line 79, citing Amendment 7 D.5. I checked D.5's text.

**3. The v2 port.** These read-only GETs match `V2_FIELDS.md` except at its line 31 (M5a):
- `GET /v2/pods`: 200, `{pods, pagination}`, 26 pods, all `EXITED`, none named `pkm-x-`;
- `GET /v2/catalog/cpus/cpu5m?...vcpuCount=32`: 200, `HIGH`, 8 GB per vCPU, vCPU 2-32, US$0.065 per vCPU-hour,
  `dataCenters [EUR-IS-1]`;
- `GET /v2/pods/klogu58zoe2owr` (the deleted smoke pod): 404.

The spec names the paging parameter `cursor`, as `list_pkm` uses it.

Rules that relax when a v2 field is missing:
- A 200 record without `status` counts as RUNNING (`fetch.sh:182`). That means no information, consistent with N2; it
  affects neither cost nor relaunch.
- The interruptible check is vacuous by design: the spec has no spot field. This is disclosed.
- `dataCenters` is not checked (M5e). A failed create is the consequence, and it costs nothing.
- The reaper goes blind when the list fails (S7).

None of these opens a relaunch or a reading.

**4. Budget.**
- Ledger: 465.9320 = billing rows 414.8386 + EKS 19.3324 + `a32-remainder` 31.6714 + smoke3 0.0896.
- The billing file has 39 pods summing 487.7988, which is the ledger's 37 rows plus the two UNKNOWN-5 pods (72.9602). The
  Amendment 31 rows sum to 370.8424.
- Checks today: 2.08 × 30 = 62.40 ≤ 65, and 465.93 + 62.40 = 528.33 ≤ 535.
- With the session up, pod life is bounded at about 30.8 h, US$64.06.
- With the session down, neither the cap nor the ceiling holds (M1). The ceiling breaks at about 35.6 h.

**5. Secrets.**
- I scanned the committed smoke3 receipts and results, `receipts_dev`, `smoke` and the test fixtures: no 48-hex runner
  token and no `rpa_` key. `pod-request.json` shows `<withheld>`.
- The API key reaches only a 0600 header file, written with the `printf` builtin (never in argv), outside the worktree.
- Saved records drop `env`, `args`, `cmd`, `entrypoint`, `ssh` and `consumerUserId`.
- The pod token appears in curl argv through `rp_pinned.sh` (disclosed; test B10 counts 13 lines).
- The list response carries every pod's `env` (M5a); the route projects it away.
- I deleted my own transient copy of the list response from the scratchpad.
- Residual risk: S13.

**6. Stated as checked, but not reproducible:**
- "188/188" (M4);
- "every count identical across draws" (S9);
- "list entries lack `env` and `args`" (M5a).

Everything else I tried reproduced:
- the section 10 hashes;
- the pins against `d2a16c723d`;
- the request and its validator;
- the `entry_r2.sh` diff;
- the code citations at `f13a690df0`;
- the memory and time arithmetic;
- the ledger sums;
- the steps 1-29 identity between 12.8·10⁹ and 27.9·10⁹.

## Smoke3 (coordinator's question): the memory.max boundary and contradictions

- **Is `>= 256e9` right?** Yes, as a comparison. The design derives `--max-dense 54000000000` from exactly 256·10⁹ − 36
  GiB. A value equal to the floor therefore leaves the design's 36 GiB headroom intact (1.25 GiB of it unassigned). A
  lower value would invalidate the headroom and is correctly refused. The margin is meant to sit in the headroom, not in
  the floor. 256·10⁹ is page-aligned (62,500,000 × 4 KiB), so the kernel stores it exactly, as smoke3 shows.
- **Is the 7.5 GiB spare enough?** For step 35 and for the low and central bands, yes, with about 70-90 GiB spare. As a
  bound for any step near the cap, no. The spare is computed against a modelled peak (basis 24 GiB, echelon 4.7 GiB at
  `nq` = 50,256, c₀ = 1 GiB). Three things are not in it:
  1. Kernel page tables charged to the cgroup: about 0.45 GiB at 230 GiB of 4 KiB pages.
  2. Allocator page rounding per `B'` row: about 1.3 GiB at 686k rows, against about 0.25 GiB at the calibrated step 30.
  3. A later step with a larger `nq`: a near-full echelon at `nq` = 83k is about 12.8 GiB.

  Any one of these, combined with a basis of about 24 GiB, can use up the spare. If the spare is exceeded, the outcome is
  OOM: censored, the same reading as an `oversize` stop. Only spend and classification clarity are at stake (S1, S8).
  - A smaller cap, say 45-50·10⁹, would give the same reach for the low and central bands (34-36·10⁹) and a clean
    `oversize` stop in place of a possible OOM. 54·10⁹ buys a chance to finish a step between about 1.3× and 1.6× of
    step 35. Either choice is defensible. Take the choice knowingly and stop calling 230.9 GiB a worst case (M5d).
- **Does smoke3 contradict the route?** No.
  - Its records satisfy every check:
    - create: 201, `cloud SECURE`, `cpu {cpu5m, 32}`, `cost` 2.08 equal to the catalog price;
    - GET record: `cpu.memory 256`;
    - ready record: `RUNNING` and `dataCenterId EUR-IS-1`;
    - pre-start `memory.max`;
    - classifier P2d, git gate, `DELETE` 204, gone from the list.
  - The runner's `nproc 128` is `os.cpu_count()`, the host's CPUs. `host.txt` shows `nproc: 32` (affinity), as the
    amendment says.
  - `MemTotal` 527,690,100 kB is the host's. Neither the runner's `/status` nor `host.txt` measures the pod; only
    `memory.max` does.
  - `dataCenterId` was already set on the create response. That does not contradict P7, which only says it may be null.
- **Does it contradict `V2_FIELDS.md`?** Not on what the smoke shows. `V2_FIELDS.md` is stale (M5b, M5c) and wrong on
  list keys (M5a, from my GET).

## What I verified myself (commands run; nothing taken from the text)

- `sha256sum` of every file in section 10: all equal.
- `git show d2a16c723d:research/pkm_falldegree_unblock_20261107/runpod/{server.py,rp.sh}` hashes equal the pins.
- The layer files' hashes equal `BIN_SHA_PINS`; `diff entry.sh entry_r2.sh`.
- The request: jq diff; re-derivation through `common.sh`; `validate_command_request.sh`.
- The offline suite, run plainly: 187/188 (B04b, M4).
- Read-only RunPod v2 GETs: list (keys and types only), catalog, deleted pod. The public OpenAPI spec (cursor
  parameter; no spot or TTL field).
- Extracted both route-F snapshot archives into scratch. Read the step 23, 27-35 lines and the row. Compared k0 and k1
  (S9), and 12.8·10⁹ against 27.9·10⁹ (steps 1-29 identical).
- `git show f13a690df0` of `f4_fp_tower.rs` and `pkm_tower_pilot.rs`: lines 1578, 1606-1694, 1679, 1694, 2435-2438,
  2448, 2698, 2793, and pilot lines 1166-1189 and 1506. No `global_allocator` in the pilot.
- Ran `python3 -I t14_bigmem/project.py`; recomputed the model residuals and the A and B units by hand.
- Ledger: grouped sums; checked the billing file's 39 ids against the ledger's 37.
- Read the smoke3 receipts and results, and scanned them for secrets.
- Read `PREREGISTRATION.md` Amendments 7 B-D, 29 and 32 and `DECISIONS.md` rows 10-24. Checked round-3 items S1-S11
  against the ported code.
