# DRAFT Amendment 34: a host check before start (`vm.max_map_count >= 262,144`) and up to three retries of the `t = 14` draw 0 cap-7 probe

**Status: draft, NOT frozen (2026-11-22).** When frozen it is appended to
`research/prime_fall_degree_20261027/PREREGISTRATION.md` after Amendment 33, with the SHA-256 of the route scripts
(section 6), before any job named below starts. `run.sh` refuses the study names unless the preregistration has the
heading `## Amendment 34` and those hashes. Amendment 33 stays in force except where this text says otherwise. Every
number is `[pcc:unverified]`.

## 1. Owner decision and what happened

- **Owner decision 28** (2026-11-22, about 06:50Z; `research/open_leads_execution_20261108/DECISIONS.md`): retry the
  cell with one pre-start check that refuses any pod whose host allows fewer than 262,144 memory mappings, deletes it
  before the job starts and tries another host. Same cell and flags; a quick independent review before the freeze.
- **What happened** (`runpod_capped/RESULTS_d54.md`): the Amendment 33 probe `pkm-x-t14-k0-c7-d54` ran 6.9 h on pod
  `46duu2mnh5dyz6` (EU-RO-1). The pilot aborted with exit 134 at step 34 (degree 6), after
  `memory allocation of 262144 bytes failed`, with 63.5 GiB used of a 238 GiB limit, `memory.failcnt` 0 and `oom_kill`
  0. The host's `vm.max_map_count` was 65,530 (copied by the boot loop). The static binary's allocator maps each large
  block separately, and step 33 kept 68,581 basis elements, so the mapping count, not memory, was the binding limit.
  The run never reached degree 7: it says nothing about `d_F` at `t = 14`, and `d_F >= 7` (Amendment 31) stands. Under
  Amendment 33 it is NOT_ADMITTED (rule P2e), cost US$14.44.
- **Disclosure.** The registering session has seen that run's logs up to step 33 (degree 6) and its abort lines, and
  every committed row listed in Amendment 33's disclosure. No rule below depends on them beyond the threshold, which is
  set from the abort itself.

## 2. The jobs and their order

| name | when it may run | stage and request |
|:--|:--|:--|
| `pkm-x-t14-k0-c7-d54-h1` | first | `cap:14:0:7:54000000000`; `requests_d54/pfd-x-t14-k0-c7-d54.json` (SHA-256 `089176f2…`, unchanged) |
| `pkm-x-t14-k0-c7-d54-h2` | only if h1 (or its relaunch) ended as a host-limit abort (rule P2h, section 4) | same |
| `pkm-x-t14-k0-c7-d54-h3` | only if h2 (or its relaunch) ended as a host-limit abort | same |
| `<name>-r1` for each | only after that name is LOST with relaunch `auto` (owner decision 13) | same |

- The Amendment 33 name `pkm-x-t14-k0-c7-d54` is used (decision N6 forbids reuse) and retired. At most three retries;
  after a third host-limit abort the cell goes to the owner. Any other outcome of a retry resolves the cell as
  Amendment 33 reads it.
- Unchanged from Amendment 33: the stage, `--budget 86400`, `--max-dense 54000000000`, `RAYON_NUM_THREADS = 12`, the
  frozen binaries, the pod (`cpu5m`, 32 vCPU / 256 GB), the memory floor and cgroup check, the reading table, the
  classifier rules P0-P8, the git gate, the reaper (hourly independent task, owner decision 25), `watch.sh` (owner
  decision 26), the per-job cap US$65, the guard US$595 and the ceiling US$600.

## 3. The host check

- **Where.** In `run.sh`, after the runner answers, the files are uploaded and the cgroup limit is read, and before
  `/start`: the value of `/proc/sys/vm/max_map_count` that the boot loop copies once at boot into
  `/results/cgroup/max_map_count`, read from the same pre-start fetch as the cgroup limit and recorded in
  `cgroup-prestart.json` (`recorded.max_map_count`). The runner's `/status` does not report it.
- **Threshold: at least 262,144, else refused** (also when the value is missing or not a number). The probe exhausted
  65,530 mappings at step 34 with 63.5 GiB in use; 262,144 gives four times that room (for
  comparison, review 4 S1 notes that Ubuntu 24.04 kernels ship 1,048,576). The degree-7 steps the cap allows can hold more rows than step 33 did, so a host at
  65,530 cannot be expected to pass the point where the probe stopped.
- **A refusal is not an attempt and does not use the name.** The pod is deleted (the delete logged first, as every
  session delete), its deletion confirmed by listing, `run.sh` exits 13, the receipts move to
  `receipts/<name>.hostrefused-<k>/`, and the ledger row is booked as `<name>.hostrefused-<k>/<pod id>` with its derived
  cost (closed), so decision N6's check (any row `<name>/…`) does not block the name. The next `run.sh` of the same name
  creates a new pod, which RunPod may place on another host. **At most five refused pods per name**; then `run.sh`
  refuses the name and the owner decides. A refusal whose delete is not confirmed is treated as any unconfirmed pre-start
  delete (exit 11, row left open under `<name>/…`, the name blocked).

## 4. The classifier: host-limit abort (rule P2h, classifier `a34-r1`)

After P2b, P2c, P2k and P2p and before P2u: a secured completion whose pilot `end` line has exit 134, whose run log has
at least one `memory allocation of N bytes failed` line, with cgroup `oom_kill` 0 and a memory peak under the pod's
numeric `memory.max`, is **NOT_ADMITTED with `host_limit: true`**: no row, no reading, no relaunch; it only lets the
order of section 2 offer the next retry. On the committed tree of the Amendment 33 probe
(`receipts/pkm-x-t14-k0-c7-d54/`, `results/pkm-x-t14-k0-c7-d54/collect-1/`) the offline classifier gives NOT_ADMITTED P2h
with `host_limit: true` (offline test D54). That run's committed `outcome.json` (P2e, classifier `a33-r3`) is not
rewritten.

## 5. What this changes, and the cost

- **Procedure only.** No cell, seed, draw, cap, flag, limit, budget, reading or threshold of any rule other than the new
  pre-start check changes. The probe's row would have been read exactly as Amendment 33 says.
- **Cost (derived).** The ledger stands at US$517.63 (`ledger.tsv`, which includes the probe's US$14.44). One retry's
  worst case is 2.08 × 30.8 h = US$64.06: 517.63 + 64.06 = 581.69 <= 595. A refused pod lives a few minutes (boot,
  upload and the check), about US$0.05-0.15 each; five per name add well under US$1. `run.sh` checks the guard before
  every create against the ledger as it then stands, so h2 can start only if h1's actual cost leaves room: 595 − 64.06 −
  517.63 = US$13.31, about 6.4 h of pod life for h1. A host-limit abort later than that sends the cell to the owner in
  practice, as would a third retry.

## 6. Route script hashes at this revision

| file | SHA-256 |
|:--|:--|
| `common.sh` | `446c9ecc63cde08a32f56ed8f675212ae30a750eaef9c57e54fd58086adaec4a` |
| `run.sh` | `7b6eaeb25c8b93749b2f61fd864db5fba2390da120de8d69f5c31b7019c5f74a` |
| `fetch.sh` | `ffd8d0a0ac1b31d6b47355f07debdf1a580ba665dce50492ec6804a95a53724d` |
| `reap.sh` | `90f3fbe7be95dac868d248b9a2321eea8d400fb0a06d4aff0defe65ca0e4becd` (unchanged from Amendment 33) |
| `watch.sh` | `cea7bf4d774b289505fddd6d3a4c377d862c706f43a9dd24a4a91bfd8185fb73` (unchanged from Amendment 33) |
| `requests_d54/pfd-x-t14-k0-c7-d54.json` | `089176f2a75dc228a817dc1b88852cd1953fbfcd63e0452189acd4765f70d025` (unchanged) |

At the freeze the first four lines go into the preregistration as `sha256sum` output. Offline tests: `tests/README.md`.
