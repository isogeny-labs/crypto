# Prime fall-degree study: relaunch card, access request, m = 4 extension, data recovery

Written 2026-10-29 as one track of the prime-field IC preparation workflow. This
is **preparation, not measurement**: nothing here was run, and no number below
is new. Every statement is labelled:

- **measured (source)**: a recorded value, with the repository path it comes from;
- **derived**: arithmetic on recorded values or on formulas stated here, re-derivable by hand;
- **plan**: a proposal that needs an owner decision or a run before it means anything.

Study under relaunch: `research/prime_fall_degree_20261027/` (PREREGISTRATION.md
with Amendments 1-6, DERIVED.md, RESTART.md, run.sh, isolab/). Source state read:
main at `ffa91ecdc` for that directory (last commit touching it).

Files in this directory:

| file | what it is |
|:--|:--|
| `README.md` | this document: why the route is blocked, the relaunch plan, the m >= 4 reading, the inconsistency list, owner decisions |
| `CARD_DRAFT.yaml` | the relaunch as a `crypto.autoresearch.mechanical-task-card.v1` draft for the harness host; every value an owner or director must still fix is marked `FILL:` |
| `AMENDMENT_7_DRAFT.md` | draft amendment the relaunch needs before any `t >= 12` row: resources, a capped-F4 bracket for `t >= 12`, consistent-target handling, the scorer patch |
| `AMENDMENT_8_DRAFT_m4.md` | draft amendment for the `m = 4`, `N >= 16` extension, with memory and compute estimates and keep/close readings |
| `ACCESS_REQUEST.md` | the minimal change that would let an agent submit an argv + image job without the harness host (not performed) |
| `MAC_RECOVERY.md` | steps to recover the files stranded on the owner's Mac |
| `proposed/Dockerfile`, `proposed/entry.sh` | corrected image and entry script (proposals; the originals in the study directory are unchanged) |
| `proposed/jobs/*.json` | `isolab_k8s_run` argument templates, one per job |

---

## 1. Why the EKS request route cannot run `pkm_tower_pilot`, and what can

**Measured (contracts and scripts on main):**

- The portable request `isolab.eks.request/v1`
  (`taskq/deploy/eks-karpenter/contracts/request.schema.json`) has exactly these
  `job` fields: `instance_type`, `job_name`, `memory`, `seconds` (maximum 3600),
  `spec` (an `ecbench.spec/v1` object) and `build` (`image_uri`, `source_revision`,
  `ecbench_sha256`). `additionalProperties` is false. There is no command, argv or
  CPU field. `HARNESS.md` states it directly: "Neither interface executes source
  code or shell commands supplied in a benchmark request." CPU count is fixed at four.
- The CodeBuild side (`taskq/deploy/eks-karpenter/remote/run-request`) calls only
  `isolab-eks-mcp call isolab_k8s_benchmark` and then polls status inside the build.
  CodeBuild's build timeout is 70 minutes (`HARNESS.md`), so even an ecbench job
  submitted this way cannot outlive the launcher by much.
- `isolab.eks.resumable/v1` takes an ecbench spec and admits `c8a.2xlarge` only
  (8 vCPU, 16 GiB) (`HARNESS.md`, "Resumable native computation";
  `docs/session-logs/2026-10-29.md`).
- The GitHub workflow `isolab-experiment.yml` builds one binary,
  `cargo build ... --bin ecbench` for musl (`taskq/deploy/eks-karpenter/build-bundle`),
  and its `size_sweep` mode also runs ecbench specs (`run-ic-sweep`).
- The only argv + image route is the MCP tool `isolab_k8s_run`
  (`contracts/mcp-tools.json`): `job_name`, `command` (argv, up to 256 items, no shell),
  `image` (digest-pinned), `cpu` (2 to 28, even), `memory`, `seconds`, `retries`. It
  "submits a Job without waiting". The MCP binary and kubeconfig live on the harness
  host (`ACCESS.md` §1: `/home/ec2-user/ops/eks-isolab/mcp/isolab-eks-mcp`,
  `KUBECONFIG=/home/ec2-user/ops/eks-isolab/kubeconfig`), or on a VPC-connected host
  whose IAM role an administrator has mapped to `isolab:submitters` (`ACCESS.md` §4).
- The pod it creates (`taskq/deploy/eks-karpenter/submit-job`): UID/GID 1000,
  `readOnlyRootFilesystem: true`, `/work` an 8 GiB emptyDir, `/tmp` 1 GiB,
  ephemeral storage 10 GiB, no extra environment variables beyond the lab's,
  `ISOLAB_RESULTS_URI = s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod uid>/`.
  Pools: `cpu-spot` and `cpu-l2-spot` each capped at 32 vCPU and 128 GiB, 64 vCPU combined.
- This container has neither the MCP nor a Docker daemon nor AWS identity
  (`docs/session-logs/2026-10-29.md`; also this track's constraints).

**So:** the relaunch runs from the harness host with `isolab_k8s_run`, which the owner
chose on 2026-10-28 ("Owner chose the harness-host route (option 1)",
`docs/session-logs/2026-10-28.md` 17:15Z). The minimal change that would remove the
harness-host dependency is in `ACCESS_REQUEST.md`.

This study is not a timing benchmark. Its outcomes are degrees and deterministic
multiply-add counts, so the ecbench L2/replay/A/A admission does not apply and none
is claimed. Wall-clock time still matters, because `--budget` stops are wall-clock,
so the instance type and thread count decide which rows are censored. Record both.

---

## 2. The relaunch plan (frozen-command-card-ready)

The executable form is `CARD_DRAFT.yaml`. What follows is the reasoning a director
needs to freeze it.

### 2.1 Before anything runs (main session, not the executor)

1. **Decide Amendment 7** (`AMENDMENT_7_DRAFT.md`). Without it the registered flags
   cannot test the C-step prediction at `t = 12, 13, 14` (inconsistency 8 below). It
   must be committed before any `t >= 12` row exists. First check whether the Mac
   holds `t >= 11` rows (§4): stage A at `t >= 11` was in flight when the original
   session stopped (RESTART.md).
2. **Fix the image** (inconsistency 13). Use `proposed/Dockerfile`: the same Rust as the
   lab build (1.99.1), a static musl target, the same base image digest as
   `Dockerfile.ecbench`, and all three examples (`pkm_tower_pilot`,
   `prime_fall_degree_tools`, `hky_fp_tower_check`). Then smoke-test the image
   read-only as UID 1000 (`CARD_DRAFT.yaml`, `pre` section). The original Dockerfile
   copies glibc binaries built on `rust:1.88-bookworm` into the aws-cli image. The
   lab's own ecbench image avoids exactly that by building static musl (`HARNESS.md`:
   "so it runs in the existing runtime image independently of the GitHub runner's
   glibc version"). That claim was not checked here, since this container has no Docker.
3. **Fix the entry script.** Use `proposed/entry.sh`. It adds stages `g1` (the
   `hky_fp_tower_check` example), `g3` (`prime_fall_degree_tools gate3`) and `score`.
   It hashes the binaries at start, uploads the hashes before the first stage, and
   exits non-zero if any stage's `progress.txt` line reports a non-zero exit.
4. **Patch the scorer** (inconsistency 7). The committed scorer does not read `CAP-*`
   rows, so it cannot produce the Amendment 3 reading "`=D (F4)`" for `t >= 11`. The
   rule to implement is in `AMENDMENT_7_DRAFT.md` §D. Commit it before scoring.
   Scoring is local work; the image does not need it.
5. **Freeze** the commit (`git rev-parse HEAD` on the PR head that carries 2-4).
   Build and push to `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks`, then
   read the digest with `aws ecr describe-images`. Record the source SHA, the image
   digest and the three binary SHA-256s separately.
6. Run `scripts/checkpoint.sh init`, fill `.resume/RESUME.md`, then run
   `scripts/checkpoint.sh push` and `scripts/checkpoint.sh gate` (CLAUDE.md "Resume
   checkpoints"). The 2026-10-28 checkpoint predates this plan.
7. On the harness host, run `isolab_k8s_overview {}`. Confirm that no `pfd-*` job
   exists and that `cpu-spot` is idle enough for two 14-vCPU / 56 GiB pods.
   Two 16-vCPU / 64 GiB nodes fill the pool's 32 vCPU / 128 GiB exactly
   (derived from the pool limits above).

### 2.2 Gates, as the relaunch runs them

| gate | how | pass condition | where |
|:--|:--|:--|:--|
| G1 | `hky_fp_tower_check` (6 hand cases) | prints `G1: 6 cases pass`, exit 0. `cargo test --lib hky_fp_tower` runs 0 tests and is **not** G1 (RESTART.md; session log 2026-10-28 18:10Z) | first stage of every job |
| G4 | `run.sh g4` (planted, `--hky-cmax 5`) | no planted closure contains `1`; every planted row has `planted_ok: true` | job C, before `hky-C` |
| G3 | `prime_fall_degree_tools gate3 /work/study /usr/local/bin/pkm_tower_pilot --max-n 20 --jobs 2 --budget 900` | `G3: <n> capped runs, 0 disagreements`; Amendment 3 recorded 70 runs over `m = 2...4`, `N <= 20` | job A after `hky-A`, before `d-A`; job C after `hky-C`; job B after `hky-B` |
| G2 | scorer: `d_F <= D` on every draw with both | `0 violations` in the TABLE.md header | at scoring |

Any gate failure stops the study (§4). G3 must pass before capped F4 is used as a `d_F`
instrument (§3), which is why `cap-A` and every Amendment 7 bracket follow `g3`.

### 2.3 Jobs (plan)

The registered cells are unchanged. They are split into jobs so that one Spot
reclaim loses one chain, and so that the expensive `t >= 12` systems each run alone.

| job | argv after `entry.sh` | cpu / memory | why |
|:--|:--|:--|:--|
| `pfd-a-<date>-001` | `g1 hky-A g3 d-A cap-A` | 14 / 56Gi | Amendment 2 chain (2), with G3 before the capped check |
| `pfd-c-<date>-001` | `g1 g4 hky-C g3` | 14 / 56Gi | chain (1), first half |
| `pfd-b-<date>-001` | `g1 hky-B d-B g3` | 14 / 56Gi | chain (1), second half. LAUNCH.md omitted it (inconsistency 4). Submit after job C ends: the pool cannot hold three |
| `pfd-x-t12-k0-<date>-001` and the others | Amendment 7 bracket rows, one system each | 14 / 56Gi (t = 12), 14 / 112Gi (t = 13, 14, one at a time) | only if Amendment 7 is adopted |

`seconds` per job is an owner decision (`FILL:` in the card). The registered limits
only bound it from above. Each closure row may take up to (`cmax - c_min + 1`) x 1800 s,
and the per-row times of the original run were never pushed. The 14,400 s in
LAUNCH.md has no basis in any recorded duration. `retries` stays 0: a reclaimed
job is a new submission with a new name, and its partial rows (synced every 5 min)
are kept for inspection only (LAUNCH.md).

### 2.4 Expected outputs

Per job, under `ISOLAB_RESULTS_URI`: `runs/<name>.jsonl|log` for every `run.sh`
invocation, `gates/g3.jsonl` (G3), `progress.txt` (start, end, exit per invocation),
`binary.sha256` (all three binaries). For job A the `runs/` names are
`A-m2-tower`, `A-m2-tower-big`, `A-m2-tower-p0`, `A-m2-null`, `A-m2-null-big`,
`A-m2-naive`, `DA-m2-tower`, `DA-m2-tower-p0`, `DA-m2-null`, and `CAP-m2-t<t>-k<k>-c<c>`.

**Verification by the director (assess):** the Job UID and Pod UID from
`isolab_k8s_status` equal the path segment in `ISOLAB_RESULTS_URI`, and listing the
results follows `next_token`. `binary.sha256` equals the hashes recorded at build.
Every `progress.txt` "end" line reads `exit 0`. Then:
`prime_fall_degree_tools score <dir>` with the patched scorer, run locally.

### 2.5 Reproducing the t <= 10 rows (Amendment 2)

Amendment 2 requires the recomputed `t <= 10` rows to agree with the first run. Those
rows are lost (RESTART.md). The only surviving reference is the session's report of
the lower medians of `d_F` for Kummer `m = 2` at `P1`, `t = 1...10`:
**3, 4, 4, 4, 4, 5, 5, 5, 5, 6** (RESTART.md, "not re-derived here").

The relaunch reads the reproduction check as follows (plan; it adds no rule):

1. **Primary (Amendment 2 as far as it can apply):** the `A-m2-tower` lower medians,
   `t = 1...10`, must equal that sequence entry for entry, uncensored. Any difference,
   including a censored median where an exact one was reported, is a reproducibility
   failure and stops the study. The check is weaker than Amendment 2 intended: it
   compares medians, not rows. Say so in RESULTS. If the Mac rows come back (§4), the
   scorer's built-in repeat check (same key including `x_r`; equal `last_fall_degree`,
   `first_fall_degree`, `falls`) upgrades it to row level.
2. **Also expected:** `ffd = 4` on every Kummer `m = 2` draw at `t = 2...10`, as DERIVED.md
   D3 proves and reports measured. A violation is a bug, not a result.
3. **Cross-check from committed data (diagnostic, not a gate).** The PKM round-2 rows
   came from the same example (an older build of `pkm_tower_pilot`), the same default seed
   (`0x504B4D54`) and the same target flags (`--ladder-t none --planted 0 --random 2`)
   (`research/pkm_tower_round2_20260925/run_round2.sh`). They record these `x_r` and `D` at `P1`
   (`research/pkm_tower_round2_20260925/runs/K1-kummer-m2-p1.jsonl`):

   | t | x_r (target 0, 1) | D (both) |
   |--:|:--|--:|
   | 6 | 464833983, 558177188 | 5 |
   | 7 | 1563871033, 1255255582 | 5 |
   | 8 | 1731249962, 1312284326 | 5 |
   | 9 | 417319711, 1892822788 | 5 |
   | 10 | 622174540, 1914092299 | 6 |
   | 11 | 237440363, 209365541 | 6 |

   If the relaunch's `A`/`DA` rows carry the same `x_r`, the systems are the same, and
   `DA`'s `D` must equal these values (an instrument check on `f4_fp_tower`). If they
   differ, the pilot's rng consumption changed since round 2. That is no failure, but
   §2.6 (b) then does not apply.

### 2.6 Scoring t >= 11 against C-step (Amendment 6)

Registered prediction: `d_F = 4 + floor((t - 2)/4)`, so **6, 6, 6, 7 at t = 11, 12, 13, 14**,
and `ffd = 4` unchanged. It is refuted by any `t >= 11` median that differs. A miss is a
result, not a reason to amend (RESTART.md step 4).

How each value is obtained (Amendment 5, Amendment 3, Amendment 7 if adopted):

- `ffd` and the one-sided `d_F <= 5` or `d_F >= 6` come from `A-m2-tower-big` (closure to `cmax = 5`).
  The closure's columns at `c = 5` are 35,443 / 55,455 / 83,682 / 122,438 for `N = 22, 24, 26, 28`
  (derived: the number of square-free monomials of degree at most 5 in `N` variables).
- `d_F` exactly: `= D (F4)` when uncapped F4 refutes at `D` and capped F4 at `D - 1` does not
  (Amendment 3). Under Amendment 7, for `t >= 12`: the two-cap bracket at
  `c_pred - 1` and `c_pred`.
- **Reading of censored values (plan, implied by §2 and Amendment 6, not a new rule):** a
  median that is only bounded, for example "`>= 6`" because the cap-6 run hit its budget at
  `t = 14`, neither confirms nor refutes C-step at that `t`. It reads **undecided at t**. A
  bound that excludes the prediction (`>= 8`, or `<= 5`) refutes it.

Two facts limit what this can show:

- **(a) Under the registered resources, t = 12-14 will censor (derived).** The recorded uncapped
  F4 times for Kummer `m = 2` at `P1` (round-2 build and host) are 5.8 s, 57 s and about 600 s
  at `t = 9, 10, 11` (`K1-kummer-m2-p1.jsonl`, `ms` 5848 / 57196 / 610517), about 10x per level.
  Extrapolated at that rate: about 1.7 h at `t = 12`, 17 h at `t = 13`, 7 days at `t = 14`, if `D`
  stays at 6. A rise to 7 costs more, and more threads cost less, by unknown factors. Peak
  memory (round-3 build) was 463 MB at `N = 20` (`p0`) and 1,523 MB at `N = 22`
  (RESEARCH_PKM_TOWER_ORACLE.md §12.7). That is about 3.3x per level, which extrapolates to roughly
  5, 17 and 55 GB at `N = 24, 26, 28`. At a P-224-shaped Solinas prime, whose `D`, widths and
  step counts matched the recorded ones, the `N = 24` system was OOM-killed at 6.0 GB (R18). A
  re-run with a `B'` cap stopped in step 26 at `B' = 7.86·10⁸` entries (R18b), giving only `D >= 6`
  (`research/pkm_p224_kummer_shape_20261022/README.md`).
  `run.sh d-A` uses `--budget 1800`, so `DA` at `t >= 12` is expected to stop on budget, and `cap-A`
  then has no `D` to cap. This is why Amendment 7 exists.
- **(b) t = 11 is not an out-of-sample test (derived).** The round-2 rows above already give
  `D = 6` at `t = 11` on both targets, and the cap-5 confirmation runs did not refute
  (`research/pkm_tower_round2_20260925/runs/C-K1-kummer-m2-p1-t11.jsonl`: `max_degree_bound 5`,
  `inconsistent false`). If the systems are the same (§2.5 item 3), the G3 equivalence turns this into
  `d_F = 6` at `t = 11`, and that was knowable from committed data before Amendment 6 was registered.
  Only **t = 12, 13 (predict 6) and t = 14 (predict 7)** test C-step, and t = 14 is the decisive one.
  The C-step formula also restates the PKM note's own "every four levels" pattern for `D`
  (RESEARCH_PKM_TOWER_ORACLE.md §11.10, item 2). The transcript's `d_F` medians at `t <= 10`
  equal the recorded `D` at `N = 2...20` value for value (§17: "3 4 4 4 4 5 5 5 5 6"), so
  H-gap at `m = 2` currently reads `d_F = D`, at transcript level. (Cross-track review: the §17
  sequence is recorded at `p₀` and at the torus primes `7·2^21 − 1`, `61·2^25 − 1`, not at `P1`;
  at `P1` the committed round-2 rows above give `D = 5, 5, 5, 5, 6` at `t = 6...10`, which agrees
  with the medians there. `D` at `P1` for `t <= 5` is not in a committed file read here.)

**Consistent targets at t = 14 (derived).** With `λ = |V|^m·2^{m−1}/(m!·p)`, Kummer `m = 2` at
`P1 = 2013265921` has `λ = 0.033` at `t = 13` and **`λ = 0.133` at `t = 14`**. That is above the
0.1 threshold Amendment 1 imposes on the `P0` arm. A Poisson reading gives a probability of about
1 − e^(−0.133) ≈ 12% that a given draw is decomposable, and about 23% that at least one of two is.
A decomposable draw has no `d_F`. The committed scorer would read it as censored "`> cmax`", which
would bias the decisive cell upward. Amendment 7 §C adds the rule.

---

## 3. The m >= 4 extension, and what to expect at cryptographic size

Full draft: `AMENDMENT_8_DRAFT_m4.md`. The summary follows.

**Why m >= 4.** With per-target cost `T` and sparse linear algebra, the end-to-end cost is
about `m!·r·T/|V|^{m−1} + m·|V|²`, which gives `r^{2/(m+1)}` for `T = r^{o(1)}`. So no oracle
beats rho at `m <= 3` (RESEARCH_PKM_TOWER_ORACLE.md §3.7). Cells at `m = 2, 3` measure the
mechanism. `m = 4` is the first cell that could matter.

**What is recorded at m = 4 (measured, Kummer, P1):** F4 `D = 6, 7, 7` at `N = 8, 12, 16` (§11.7, §12.7).
At `N = 16`: 76,837 columns, 53 steps, 55.5 min, 8.06 GB, 2.7·10¹³ multiply-adds. The cap-6 re-run
ended after 11 steps with 1,205 pairs above the bound, without refuting (§12.7). By the G3 equivalence
this gives **`d_F = 7` at m = 4, N = 16 (derived, conditional** on G3 extending to that system and on
the systems matching by `x_r`). The projection for `N = 20` is **240-380 GB and 0.85-2·10¹⁶
multiply-adds for uncapped F4** (§17, derived there). That is above the lab's 128 GiB pool cap.

**Closure sizes (derived):** the closure's columns at degree `≤ c` with `N = 4t` tower variables and
2 free unknowns are `Σ_k C(N,k)·C(c−k+2, 2)`:

| t | N | λ at P1 | c = 6 | c = 7 | c = 8 |
|--:|--:|--:|--:|--:|--:|
| 4 | 16 | 1.1·10⁻⁵ | 39,796 | 91,276 | 181,959 |
| 5 | 20 | 1.7·10⁻⁴ | 129,039 | 356,959 | 848,829 |
| 6 | 24 | 2.8·10⁻³ | 350,776 | 1,148,040 | 3,216,930 |
| 7 | 28 | 0.044 | 833,476 | 3,166,588 | 10,291,023 |
| 8 | 32 | **0.71** | 1,783,819 | 7,738,035 | 28,725,424 |

The F4 width at `N = 16` (76,837) is 84% of the `c = 7` column count (91,276), so the count is a fair
proxy for matrix width. At `P1`, `m = 4` stops at `t = 7` because random targets become decomposable
(`λ = 0.71` at `t = 8`). Every engine asserts `p < 2³²` (`hky_fp_tower.rs`; §17), so the `m = 4`
N-ladder at a toy prime spans at most three levels above `t = 4`. One rise per four levels, the C-step
rate at `m = 2`, cannot be resolved there. Going further needs 64-bit field arithmetic in the closure,
or targets conditioned on inconsistency (Amendment 8, owner decision).

**Back-of-envelope at cryptographic size (derived, optimistic model).** The model, stated in full:
relations needed ≈ `|V|`; queries per relation = `1/λ = m!·p/(|V|^m·2^{m−1})`, floored at 1; per-query
cost `T = cols(N, m−2, d)²` `F_p` operations, which is one sparse elimination of the degree-`d` matrix and
a floor, not an estimate of F4; linear algebra `m·|V|²`. The total is minimised over `t`. It is compared
with `log₂ √(πp/4)` group operations for rho with negation. That mixes `F_p` operations with group
operations, a few bits in rho's favour, which are ignored here.

| p | m | best t (N) | d = 4 | d = 5 | d = 6 | d = 7 | d = 8 |
|:--|--:|:--|--:|--:|--:|--:|--:|
| 2^256 | 4 | 62-65 (248-260) | −1 | +4 | +15 | **+25** | +35 |
| 2^192 | 4 | 48-49 | +6 | +16 | +26 | +35 | +44 |
| 2^160 | 4 | 40-41 | +11 | +21 | +31 | +40 | +48 |

Entries are log₂(IC total / rho) in bits; negative means below rho. At `p = 2^256` and the
relation-balanced size `t = 64` (`N = 256`), the per-query matrix has `2^33.1, 2^38.5, 2^43.7, 2^48.7`
columns at `d = 5, 6, 7, 8`. At `d = 7` that is about 1.4·10¹³ columns: at ten 8-byte entries per column,
about a petabyte per query. Across `m` at `p = 2^256`, under the same model, the largest `d` that does
not lose to rho is 4 at `m = 4`, 5 at `m = 5`, 6 at `m = 6`, 7 at `m = 7` and 7 at `m = 8`. Even a
**bounded** `d_F = 7` at `m = 4` leaves about 25 bits to rho. A `d_F` that keeps growing like C-step,
transposed for illustration as `7 + floor((t − 4)/4)` at `m = 4`, leaves about 143 bits. The recorded F4
`D` at `t = 2` is `m + 2` for `m = 2, 3, 4` (4, 5, 6; §11.7, §17). The thresholds above are at most `m`,
so a degree that tracks `m + 2` stays above the threshold at every `m` in the list. That is an extrapolation
from three values, not a measurement.

**What would keep or close the lane (plan; fixed in Amendment 8 before data):**

- **Close (for the PKM-tower exponent lane):** C-step holds at `t = 12-14`, so `d_F` grows in `N` at
  `m = 2`, *and* the `m = 4` closure reads `d_F >= 8` at `N = 20` on both targets. A degree that
  grows with `N` defeats every fixed `m` in the table above.
- **Keep, as a mechanism question only:** `d_F = 7` at `m = 4`, `N = 16` and `N = 20`, and C-step refuted
  at `t = 12` or `14` with a lower value. A flat `d_F` at `m = 4` does not beat rho, by the table. The
  lane stays open only for `m >= 6`, where the threshold `d` reaches 6-7. The next decisive measurement
  is ladder C's growth of `d_F` in `m` (H-m) at fixed `t`, extended past `m = 6` with the closure
  (`m <= 10`, at most 8 free unknowns).
- **Anything else** (one target up, one flat; censored at `N = 20`) reads as **undecided at m = 4**,
  and the lane's status does not change.

None of these outcomes is an improvement over the IC baseline under the success criterion
(CLAUDE.md). The study is a stage diagnostic of the solver axis: no `S`, no leaderboard row
(PREREGISTRATION §8).

---

## 4. Data recovery for the Mac-only files

Full steps: `MAC_RECOVERY.md`. **Measured** state (`docs/session-logs/2026-10-29.md`): the Mac
session `session_01TCbLH7A3qhgVw5cSWZKhtH` reports `computer_unreachable`. The Mac folders `git`,
`Downloads` and `Volumes` appear only under `cannot_be_used`. `send_message` to that session was
refused (`elevated_session_untrusted_device`). The worktree is
`/Volumes/SSD990-2/crypto-worktrees/prime-fall-degree-20261027`.

What to recover, in priority order: `runs/` (including any `t >= 11` rows from the stopped chains),
`gates/g3.jsonl` (the only evidence of G3's 70 runs), `derived/` (cited by DERIVED.md but not on main;
RESTART's list of lost files omits it), `progress.txt`, the scorer diff since `f1b7dfc03`, `TABLE.md`,
`score.json`, the draft `RESULTS.md` and the report draft.

Only the owner can do this, from the Mac or by answering a trust card in person. The relaunch does not
depend on it. Recovered rows join the relaunch rows as earlier copies, and any disagreement on a
deterministic field is a reproducibility failure (Amendment 2).

---

## 5. Inconsistencies found in the preregistration and launch files

Each item names its source. None has been fixed in the study directory, since this track writes only
here.

1. **G1 names a command that runs nothing.** §4 says "`cargo test --lib hky_fp_tower` passes".
   The module has no `#[test]`, so it runs 0 tests and exits 0. The six cases live in
   `examples/hky_fp_tower_check.rs` (RESTART.md; session log 2026-10-28 18:10Z). §4 was never amended.
   The original Dockerfile does not build that example either, so G1 cannot run from the measured image.
2. **G3 has no stage.** `run.sh` has no G3 invocation. Its header names a stage `cap` "capped F4 for G3",
   but the implemented stage is `cap-A`, which is Amendment 3/5's `D − 1` check, not G3, and the usage
   line omits `cap-A`. The G3 runner exists as `prime_fall_degree_tools gate3`, but neither `entry.sh`
   nor LAUNCH.md calls it.
3. **No `DC-*` rows.** Amendment 3 names "the `D` rows for the C cells … `DC-*`". `run.sh` has no `d-C`
   stage, so G2 is never checked on C cells, and the Amendment 3 procedure never runs there.
4. **LAUNCH.md drops stage B.** Its two jobs are `hky-A d-A cap-A` and `g4 hky-C`. `hky-B` and `d-B`
   (registered cells B, RESTART step 5, Amendment 2 chain (1) "C, then B") appear nowhere.
5. **LAUNCH.md reorders the chains.** Amendment 2 has (1) C, then B and (2) A, then `d-A`, `d-B`. LAUNCH.md
   moves G4 into chain C and drops `d-B`. That is harmless for results, but it does not match the amendment.
6. **`cap-A` is incomplete and fragile.** It caps once at `D − 1` and never steps down, though Amendment 3
   says "If it refutes, step the cap down". It takes `D` from any `DA` row, including a budget-stopped one
   (`timed_out: true`, `inconsistent: false`), where `D` is only the degree reached, so the cap means nothing.
   It extracts fields by grep/sed from JSON text.
7. **The committed scorer cannot score t >= 11 as registered.** `score` reads closure rows (`A-`, `B-`, `C-`)
   and F4 rows (`D*`), not `CAP-*`. A `cmax = 5` row with no `1` reads `">6"`, and nothing turns it into
   "`=D (F4)`". RESTART.md lists "the scorer edits made after commit f1b7dfc03" as lost.
8. **Amendment 5 moved t >= 11 onto F4 without F4 resources.** `run.sh d-A` keeps `--budget 1800` and
   `--max-nnz 2·10⁹`. The recorded growth (§2.6 (a)) puts `t = 12` past that budget and `t = 13, 14` far past it.
9. **Consistent targets are not handled.** §5 says random targets are "inconsistent with overwhelming
   probability". At `P1`, `m = 2`, `t = 14`, `λ = 0.133`. At `P1`, `m = 6`, `t = 6` (`run.sh hky-B` runs
   `--max-t 6`), `λ = 1.52`, so most draws are decomposable, and the closure reads them as "`d_F > cmax`".
   Amendment 1's `λ < 0.1` guard covers only the `P0` arm. At `m = 6` the practical harm is small: the
   closure at `c = 7` already has 2,210,380 columns at `t = 4`, above `max_cols`, so those rows censor on
   limits anyway (derived). But the rows are still written, and they would mix "decomposable" with
   "censored".
10. **Amendment 1's list contradicts its own criterion.** It says "`m = 6 … 10`, `t ≤ 2` (where `λ < 0.1`)",
    but at `P0` and `t = 3`, `λ = 0.015, 0.034, 0.068` for `m = 6, 7, 8` (derived). `run.sh tmax0` follows
    the list, so those cells are left out, not run wrongly. The list is conservative, not unsafe.
11. **DERIVED.md cites files that are not on main.** `derived/ns_degree.md`, `spec_chain_synthetic.md`,
    `spec_chain.md`, `spec_1d_ladder.md` and `spec_1d_rows.md` do not exist. The tools write `.jsonl` under
    those stems, and `.md` only for `spec_1d`. So the "measured" checks in D2, D4, D6 and D8, Amendment 3's
    gate results and Amendment 5's "1-D flat" are transcript-only until the Mac files return. The tools can
    regenerate them (`ns-degree`, `spec-chain`, `spec-synthetic`, `spec-1d`).
12. **C-step at t = 11 was not blind** (§2.6 (b)). Amendment 6 is registered "before any Kummer `m = 2` row at
    `t ≥ 11`" of this study. Committed PKM round-2 rows already gave `D = 6` and a non-refuting cap 5 at `t = 11`.
13. **The image is likely not runnable as written.** The glibc build on `rust:1.88-bookworm` runs on the
    aws-cli base, which `Dockerfile.ecbench` pairs with a static musl binary on purpose. Rust 1.88 is also
    older than CI's 1.98 and the lab build's 1.99.1. `COPY . .` sends the whole repository as build context.
14. **Exit status is lost.** `run.sh`'s `run()` records each binary's exit in `progress.txt` but always returns 0.
    `entry.sh` therefore exits 0 even when a stage fails. `binary.sha256` is written only after all stages,
    so a pod that dies early never uploads it.
15. **Thread count is unpinned.** `entry.sh` defaults `RAYON_NUM_THREADS` to `nproc`. In a pod with a CPU limit
    and no CPU affinity, that can be the node's CPU count. Degrees are exact and unaffected. Budget censoring
    and `ms` are affected, and possibly the multiply-add counts if parallel pivot order varies. Amendment 2
    fixed 7 threads per chain for a shared host.
16. **Amendment 2's repeat rule assumes rows that are gone.** "Its `t ≤ 10` rows are recomputed, and the repeats
    must agree": the first copies are lost, so only the median-level check of §2.5 remains.
17. **The naive control is informative only at t = 1, 2.** D5 proves `d_F ≥ 2^t`. With `cmax = 7`, every naive row
    at `t ≥ 3` reads "`> 7`" by construction. That is consistent with D5, but P1's "checked against ladder A's
    naive row" can only check `t ≤ 2`.
18. **The isogeny arm cannot reach m = 4 beyond t = 4.** At `P0`, `λ(m = 4, t = 5) = 0.44`, and at `P1` the
    isogeny builder stalls on the naive `O(p)` point count (Amendment 1). The isogeny positive control
    therefore exists at `m = 4` only for `t ≤ 4`.

---

## 6. Owner decisions needed

1. Adopt, edit or reject Amendment 7 (`t >= 12` bracket, resources, consistent-target rule, scorer patch).
   It must land before any `t >= 12` row.
2. Approve the corrected image (`proposed/Dockerfile`, `proposed/entry.sh`) for the frozen commit.
3. Set per-job `seconds` and the memory class. Job B runs after job C; the `t = 13, 14` systems run one at a time
   at 112Gi.
4. Recover the Mac files (`MAC_RECOVERY.md`), or declare them lost so RESULTS can say so.
5. Decide whether to grant the command route (`ACCESS_REQUEST.md`), or keep the harness host as the only path.
6. Adopt, edit or reject Amendment 8 (`m = 4`, `N >= 16`). Separately: lift the `p < 2³²` limit in
   `hky_fp_tower`, or accept that the toy-prime `m = 4` ladder cannot resolve a one-per-four-levels rate.
7. Rule on stage B at `P1`, `m = 6`, `t = 6` (`λ = 1.52`): drop the cell, or condition it on inconsistency.
