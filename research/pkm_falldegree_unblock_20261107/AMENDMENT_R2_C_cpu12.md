# Amendment R2-C = PREREGISTRATION Amendment 9: 12 vCPU per job, because 14 vCPU cannot be scheduled on `cpu-spot`

**Status: frozen into `research/prime_fall_degree_20261027/PREREGISTRATION.md` as "Amendment 9" on 2026-11-08** (round 2, slug
`r2pkm`). The text was appended after Amendment 8, as lines 353-376 (24 lines added, none removed). The main session commits it. The
appended section is binding; this file is the full text with the evidence, which Amendment 9 cites as its source.

- `protocol_sha256` (the `sha256sum` of PREREGISTRATION.md, as `make_requests.sh` derives it):
  `7bfc1dec059f730ac38d4c35b4abb0c3b71b9e6cd286250cff3a7f30c0c9011d`. It was `63bdff28...` before the append.
- It is registered under decisions 4 and 6 of `research/open_leads_execution_20261108/DECISIONS.md`, with the main session's decisions of
  2026-11-08: (1) keep the wall budgets; (2) pin `r6i.4xlarge` on every request whose `seconds` exceeds 90,000.
- It changes the "one 14-vCPU job at a time" wording of decision 4 to 12 vCPU.
- The two unfrozen drafts are renumbered to follow it: R2-A becomes proposed Amendment 10 (`AMENDMENT_R2_A_stageB_m6t6.md`) and R2-B
  becomes proposed Amendment 11 (`AMENDMENT_R2_B_m4_null.md`).

Every number here is `[pcc:unverified]`. Each claim carries its evidence class: *measured* (read from a file, a command or a read-only AWS
call in this session), *cited* (a repository document says so), *derived* (arithmetic on those), *heuristic*, or *plan*.

## 1. Timing, and the state of the data

The amendment was frozen **before any request was submitted**, starting with `pfd-a`. It changes resources only and uses no recorded row,
because no PKM row exists (measured, 2026-11-08 about 12:50Z):

- `receipts/` is empty, and `results/` does not exist in this directory.
- In S3, `runs/` of `isolab-results-104186811093-us-east-2` has one PKM prefix, `runs/pfd-a-20261108/`. It holds only four `status-command`
  launcher receipts (06:52, 12:17, 12:21 and 12:44Z). Each has state `missing`, with `jobs.batch "pfd-a-20261108" not found`,
  `job_uid ""` and `pods []`.
- There is no `run-command` receipt, no pod uid prefix, and no `progress.txt`, `binary.sha256` or `runs/*.jsonl`. No PKM Job was ever
  created, so the job name `pfd-a-20261108` is free.
- `benchmarks/records/20261108T0513Z-pkm-fd-pfd-a.json` is `status: open`, with empty `commands`, `entries` and `metrics`.

## 2. Platform cause (cited, measured, derived)

- The `cpu-spot` NodePool has `limits.cpu: '16'` and `instance-cpu In ["8","16","32"]`
  (`taskq/deploy/eks-karpenter/nodepool.yaml` lines 145-191; `cost-controls.json`). So the largest node it can create has 16 vCPU (cited).
- The `cpu` EC2NodeClass reserves `kubeReserved.cpu "1"` and `systemReserved.cpu "1"` (nodepool.yaml lines 31-37), so Karpenter counts
  **14000m** allocatable on a 16-vCPU node. The node's userData reserves CPU 0's whole physical core (`reservedSystemCPUs`) and sets
  `cpuManagerPolicy: static`, `full-pcpus-only: "true"` and `strict-cpu-reservation` (lines 52-72) (cited).
- Daemonsets add about 180m. On the minors study's identical request, Karpenter reported "No instance type has enough resources" for
  `cpu 14180m`, with `karpenter.sh/nodepool In [cpu-spot]` and `instance-cpu In [16 32 8]`. The pod stayed `Pending` until it was deleted
  (`research/minors_guess_determine_20261107/eks/RELAUNCH_B.md`, from the EKS audit log; cited).
- `remote/run-command` accepts `cpu` in `{2, 4, ..., 28}`, and `submit-job` accepts `^([2468]|[12][02468])$`; both pass 14 (measured). All
  17 `requests/*.json` and the 4 `requests_pending_amendment/*.json` ask for `cpu: 14` (measured). So **no PKM request could ever be
  scheduled**.
- `full-pcpus-only` allocates whole physical cores, so on SMT nodes the request must be even. 12 vCPU (6 cores) plus 180m is 12180m, which
  fits within 14000m; 14 does not. **12 is the largest feasible value** (derived). The `isolab/jobs` quota (16 CPU, 128Gi) admits 12 CPU
  with 56Gi or 112Gi (derived).

## 3. The change (binding text: PREREGISTRATION.md, Amendment 9)

These are the binding parts, as appended:

- Every relaunch job requests 12 vCPU. `RAYON_NUM_THREADS` is 12: the launcher sets it from the request, and each job records it in
  `host.txt` together with the instance type.
- Every job whose time limit exceeds 90,000 s is pinned to `r6i.4xlarge`; jobs of 90,000 s or less are not pinned.
- The wall budgets are unchanged: 1800 s, 900 s, 86400 s and 172800 s, and the job time limits stay at most 176,400 s. A budget stop is
  censored as §2 and Amendment 7 B define.
- F4 `muladds` are compared only between rows run with the same thread count.
- No hypothesis, rule, cell, seed, memory limit, command or image changes.

The request fields that change are `job.cpu` (14 to 12), `provenance.protocol_sha256` (the new hash), one sentence appended to
`provenance.note`, and `job.instance_type` `r6i.4xlarge` on the 6 requests with `seconds` over 90,000. Nothing else changes (section 7).

## 4. Thread count: set through the request, no new image

- **The request sets it** (measured). `submit-job` (sha256 `def41b1a...`) writes
  `{name:"RAYON_NUM_THREADS",value:$cpu},{name:"OMP_NUM_THREADS",value:$cpu}` into the container env (line 79). A local `--dry-run` with
  `requests_r2c/pfd-a.json`'s fields printed the following (`r2c_validate_output.txt`, last section):
  - `RAYON_NUM_THREADS "12"` and `OMP_NUM_THREADS "12"`;
  - cpu requests and limits of 12, with 56Gi;
  - `activeDeadlineSeconds 172800`;
  - nodeSelector `{"isolab.dev/pool":"cpu-spot","node.kubernetes.io/instance-type":"r6i.4xlarge"}`;
  - the unchanged command.
- **The deployed launcher does the same** (cited). The earlier receipt
  `research/arity_completion_20261104/launch/receipts/attempts/koblitz-a1/launcher/manifest.json` (launcher
  `isolab-experiment-launcher:672afd98...`, request cpu 4) carries `RAYON_NUM_THREADS "4"`. The launcher's `bin/submit-job` comes from its
  deployed bundle, so the first PKM job's `launcher/.../manifest.json` must be checked for `RAYON_NUM_THREADS "12"` and the instance-type
  selector (plan, a launch-card check).
- `entry.sh:20` and `entry_r2.sh` run `export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-14}`. With the variable set, the default is never used.
  **No new image is needed.**
- An `env RAYON_NUM_THREADS=12 ...` command wrapper would pass the validator. It is not used, because it would change `job.command` and
  duplicate the env. `/usr/bin/env` in the base image was not checked.
- Changing the default to `:-12` was not done: it would change the image digest for a value that is never read.

## 5. Wall budgets: kept unchanged (main session decision 1, 2026-11-08)

**What the protocol says the budget is for** (cited):
- PREREGISTRATION §5: "`cmax = 7` unless a limit stops it. Per-closure limits: `--budget 1800`, `max_cols`, `max_entries`. Censored rows
  are reported, never dropped."
- §2: a closure stopped by a limit "reads `d_F >= c` (censored)".
- Amendment 7 B: "A capped run stopped by its budget or `--max-dense` is censored: it bounds `d_F` from neither side."

So the budget is a resource limit that decides where censoring happens. It is not part of what is measured. It is wall-clock time
(measured, source at f13a690df0): `hky_fp_tower.rs:563` checks `started.elapsed() > limits.budget` and returns `censored("budget", ...)`,
and each row records `budget_seconds` and `wall_seconds`. The protocol has always kept the budget in wall seconds and recorded the thread
count separately:
- Amendment 2 ran 7 threads with `--budget 1800`.
- Amendment 7 A "pins `RAYON_NUM_THREADS` and records the instance type", and keeps `--budget 1800` "unchanged".
- Amendment 8 sets `--budget 172800`.

**Option K, keep the wall budgets (adopted).**
- Let `S(n)` be the speedup on `n` threads. A system whose wall time at 14 threads would be `T14` is censored at 12 threads iff
  `T14 > B * S(12)/S(14)`. Because `12/14 <= S(12)/S(14) <= 1`, the newly censored systems have `T14` in `(0.857 B, B]` at most (derived):
  1543-1800 s, 74,057-86,400 s and 148,114-172,800 s for the three budgets.
- A censored row is still reported, as a one-sided bound or as *undecided*. So the loss is in reach (H-LFD's "largest completed `N`" can move
  down), not a bias in any value that completes.
- Compared with the only earlier rows (Amendment 2, 7 threads), 12 threads give at least as much compute per budget. That supports the
  `t <= 10` median repeat check of decision 1 (heuristic).
- The image, `run.sh`, the entry scripts, the registered constants, `seconds` and the cost all stay as they are.

**Option S, scale the budgets by 14/12 (rejected).**
- The budgets would become 2100, 1050, 100,800 and 201,600 s. That needs a new image (the constants are baked into `run.sh` and `entry*.sh`)
  and edits constants that §5 and Amendment 7 A call "unchanged".
- It needs m4 `seconds` of about 205,200, above the 176,400 maximum of decision 4.
- It costs about 17% more.
- It over-compensates. The closure's per-batch pivot step is sequential (`hky_fp_tower.rs` 576-600, measured), so `S(12)/S(14) > 12/14`.
- No 14-thread boundary was ever realised on `cpu-spot` to preserve.

## 6. Other effects of 12 threads (measured from source; consequences heuristic)

- **Closure.** The batch size `limits.batch` does not depend on the thread count. The parallel `map_init` reduction is collected in order and
  then inserted sequentially, so rows, entry counts and `muladds` should not depend on the thread count. This was not re-run at 12 vs 14.
- **F4.** The S-row chunk is `per_task * 2 * rayon::current_num_threads()` (`f4_fp_tower.rs:1694`), so `muladds` and `skipped_rows` can
  depend on the thread count. Refutation and `D` are span properties and are not expected to change.
- **Gate 3** runs `--jobs 2`: 24 threads on 12 CPUs, the same 2:1 ratio as 28 on 14. Its `--budget 900` is unchanged. How `gate3` treats a
  budget stop was not examined.
- **Memory.** F4 accumulators scale with the number of concurrent tasks, so fewer threads use slightly less memory.
- **Wall time.** `pfd-a` and `pfd-b` can take up to 14/12 times longer in their parallel parts. That makes it more likely they reach their
  job `seconds` (172,800, already below the ~3.8e5 s worst case). If they do, the later stages of the chain are lost, but the rows already
  written are kept (S3 sync every 300 s). `seconds` stays as decision 4 fixed it.

## 7. Requests (measured)

`make_requests_r2c.sh image/image.json` regenerated `requests_r2c/` (17 requests) **after** the append, so every request carries
`protocol_sha256 7bfc1dec...`.

- The script is `make_requests_r2.sh` with `make_requests.sh` inlined, writing to `requests_r2c/`. `requests/`,
  `requests_pending_amendment/` and `requests_r2c_pending_amendment/` keep their sha256.
- Its only changes are `cpu 12`, the `r6i.4xlarge` pin when `seconds > 90000`, and the note sentence.
- `requests_r2c_pending_amendment/` (4 null cells) is **not regenerated**: the script skips it unless `R2C_PENDING=1`. It still needs R2-B
  (proposed Amendment 11) frozen, and then the pin and the hash.

`r2c_validate_output.txt` shows:
- The pin pre-checks pass (section 8).
- All 17 requests in `requests_r2c/` are **VALID** under `validate_command_request.sh` (`run-command`'s jq, sha256 `1cdaa744...`).
- The `jq -S` diff against `requests/` lists exactly these changed paths:
  - `job.cpu`, `job.instance_type` (new), `provenance.note` and `provenance.protocol_sha256` for `pfd-a`, `pfd-b` and the 4 `pfd-m4-*`;
  - `job.cpu`, `provenance.note` and `provenance.protocol_sha256` for `pfd-c` and the 10 `pfd-x-*`.
- Every new note starts with the old note. With those paths deleted, the documents are identical: same image `sha256:0d6b2ecd...`, command,
  memory, `seconds`, job name and source_revision f13a690df0.
- The 4 pending requests are still VALID.

## 8. Job `seconds` and the cost bound (main session decision 2, 2026-11-08)

`seconds` is unchanged: 176,400 for the m4 jobs, 172,800 for `pfd-a` and `pfd-b`, 90,000 for the capped jobs and 86,400 for `pfd-c`. It is
the Job's `activeDeadlineSeconds`, counted from the Job's start, pending time included.

**Pin pre-checks for `r6i.4xlarge` (both pass; `r2c_validate_output.txt`).**
1. *Allowed by `cpu-spot`* (nodepool.yaml, measured with `describe-instance-types`): amd64; category `r` (allowed: c, m, r); generation 6
   (rule: greater than 5); 16 vCPU (allowed: 8, 16, 32); family `r6i` (excluded families are only c7i-flex, m7i-flex, c8i-flex and
   m8i-flex); Spot price quoted in 3 AZs.
2. *112Gi fits* (derived). The node has 131,072 MiB nominal. Subtract `kubeReserved` 512Mi, `systemReserved` 512Mi and the eviction
   threshold, 100Mi (the kubelet default, because nodepool.yaml sets none). That leaves **129,948 MiB = 126.90 GiB**. The pod needs
   112Gi plus about 104Mi of daemonsets, which is 114,792 MiB (the minors pod requested 32,872Mi for 32Gi). Margin: 15,156 MiB.
   Karpenter's own estimate also subtracts its default 7.5% VM overhead (none is set in `karpenter-values.yaml`):
   131,072 × 0.925 − 1,124 = **120,118 MiB = 117.30 GiB**, still a margin of 5,326 MiB. CPU: 12180m ≤ 14000m. The real node's allocatable
   was not measured.

**Spot price, measured 2026-11-08** (read-only `describe-spot-price-history`, us-east-2, Linux/UNIX, the 6 h before 12:46Z;
`r2c_spot_prices_20261108.txt`): `r6i.4xlarge` **0.3011-0.3243 $/h** (6 points, 3 AZs). The highest price among all eligible 16-vCPU types
was r8idb.4xlarge, 0.9620 $/h.

**Bound per job** = (`seconds`/3600 + 0.25 h) × (Spot $/h + EBS).
- The 0.25 h covers node boot plus the 2-min empty consolidation (heuristic).
- EBS is 80 GiB gp3 at an assumed $0.08/GB-month, which is 0.0088 $/h (**assumption**: list price, not in the repo).

| `seconds` | requests | pinned | price used | bound per job (derived) |
|--:|:--|:--|:--|--:|
| 176,400 | 4 `pfd-m4-*` | r6i.4xlarge | 0.3243 (r6i max, measured) | **US$16.40** |
| 172,800 | `pfd-a`, `pfd-b` | r6i.4xlarge | 0.3243 | **US$16.07** |
| 90,000 | 10 `pfd-x-*` | no | 0.9620 (highest eligible type) | US$24.51 |
| 86,400 | `pfd-c` | no | 0.9620 | US$23.54 |

- **Every job is within US$40 at the prices quoted on 2026-11-08.** Without the pin, the 48-49 h jobs would have reached US$46.8-47.8.
- **What the pin does not bound.** It fixes the instance type, not the price. The request sets no Spot maximum price. A 49.25 h job stays
  within US$40 only while r6i.4xlarge Spot stays at or below 0.803 $/h, about 2.5 times the highest quote. At the assumed On-Demand list
  price of 1.008 $/h (an assumption, not read from the repo) it would cost about US$50. So the cap holds at measured prices, with that margin;
  it is not a guarantee against a price spike (heuristic). A Spot reclaim ends the job instead.
- **Pin side effects** (heuristic): a single Spot pool can mean longer pending times and more exposure to reclaim. Pending time counts
  against `activeDeadlineSeconds`.
- **Study total** (decision 4, US$400):
  - The 17 requests come to 6 pinned (US$97.7) plus 11 unpinned at the highest eligible price (US$268.6), **US$366.4 at most**. At r6i-like
    prices for the unpinned jobs it is US$189.9.
  - Adding the 4 null jobs, once frozen and pinned (+US$65.6), gives at most US$432.0, or US$255.5 at r6i-like prices.
  - A relaunch of a lost cell (decision 3) adds its own bound.
  - So the US$400 holds for the 17 requests at worst-case prices. With the null jobs it holds only if the unpinned jobs land near r6i-like
    prices or the relaunches are counted against the remainder (derived).

## 9. Consequence stated before data

Some systems whose 14-thread wall time would have fallen in the top ~14% of their budget are censored instead. The registered readings already
treat censored rows as one-sided, or as *undecided* (Amendment 8: "A censored result reads *undecided*, never *flat*"). No rule changes. The
`rayon:` value (12) and the instance type are reported with every job, and F4 `muladds` are compared only within one thread count.

## 10. Interaction

- R2-A (proposed Amendment 10) and R2-B (proposed Amendment 11) are unfrozen drafts, renumbered on 2026-11-08 with their cross-references
  fixed.
- Freezing either of them changes `protocol_sha256` again. Then re-run `make_requests_r2c.sh` (and, for R2-B, with `R2C_PENDING=1` and the
  pin) and re-validate before submitting any later request (plan).
- The R2-1 launch card binds `requests/pfd-a.json` (sha256 `d7d2901c...`), the old PREREGISTRATION hash `63bdff28...` and `R2_READY.md`. It
  **cannot launch `requests_r2c/pfd-a.json`**; a new director card must bind the new request and the new protocol hash.
- Decision 4's "one 14-vCPU job at a time" now reads "one 12-vCPU job at a time". The 16-vCPU pool holds one node, and the anti-affinity
  rule allows one isolab job per node. A second job waits, or the 16-CPU quota rejects it, while its deadline runs. So submit only when no
  other isolab job is active (`cards/no_active_jobs.sh`).
