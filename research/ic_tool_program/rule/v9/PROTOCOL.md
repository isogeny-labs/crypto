# The rule's comparison at baseline v9, with its same-session control

**Declared 2026-11-06, before any row below.** This protocol, `run.sh` and
the binaries' hashes are committed before the first row, and a resume
checkpoint holding that commit is pushed to S3 before the run.

**Everything rule v8 declared holds here** ([`../v8/PROTOCOL.md`](../v8/PROTOCOL.md),
its amendments, and through it v7's, v6's, v3's and v2's):
- §23's six sizes, 64 targets and four A/A repeats a size (408 rows a run);
- the arms' order inside one process, with rho matched (the canonicalisation
  batched sixteen lanes at a time, rule v8's default);
- the retries and stop rule;
- the claims, the pin (with amendment 1's added-counter allowance), and each
  row's logarithms against §23's;
- the reference check against the strong rho, the accounting, class and
  non-claims;
- the pricing statement under AGENTS.md §5a: both arms are timed in one
  process, so no native work is unpriced, and the 0, 0.1 and 1 GAE band
  gives one figure.

The rows under "What changes" are the only differences.

## Question

How far is a matched rho ahead of the index calculus at today's `main`?
Rule v8 (2026-10-19) measured 4.67–18.07× cold, 11.75× [10.49, 13.25] at
2^44.3 (`icv1-f2m53-tm56619371-dac20a85`). Since then the index calculus
gained, among other changes, the aimed-subset gather of #2448, whose
collect phase measured 1.093× faster [1.068, 1.136] against a 42-round A/A
(`research/ic_aimed_gather_20261023/aa2_20261029/`). The leaderboard's
Table A row for that curve still reads rule v7 (7.03×, an unmatched rho).

## What changes from rule v8

- **Two runs, one session, one host.**
  - **`v9`**: `ic` built from `main` at
    `f465f2fb1b037a57fa1d923ac8eea27ab9a603e2`, SHA-256 `3ac9e26bd2d31fe5e5acee5a942bf4c0de37d86a4d62e509b255b4ea30620b44`.
  - **`v9c`, the control**: rule v8's binary, rebuilt here from its commit
    `370e19193f6f30bf04ca2e82a2c3626932dc069e`, SHA-256 `0effb6dd2b5e34b58b954bb251f5703c37e4c12615ca75cf02951c8a826a5269`. Rule
    v8's own binary (`3500e56b…`) is not available in this container; the
    rebuild uses this host's toolchain (recorded in `runs/host.json`).
  - Each run is paired with its own in-process matched rho, so the control
    reads the distance at v8's code on this host, and the pair reads what
    changed since v8 without a cross-host comparison.
- **Interleaved by size.** v8 ran its two arms one after the other. Here,
  after both manifests, the sizes run in §23's order, and at each size `v9`
  then `v9c` (`icprog rule size`), so host drift falls on both arms alike.
  Each run's pin (`icprog rule pin`, which `rule all` runs before its first
  size) runs after its manifest and before any size; a pin that does not
  hold stops the run, as in `rule all`.
- **Harness:** `icprog` from local commit `9d76e4de0918df7d6c04f1a2ac302ca1f401e621` (main plus the
  `V9`/`V9C` comparisons and the `matched` read-out for v9; no change to how
  a row is run or checked), SHA-256 `8e6d68b85d6e434cf59b3952bfab14c768a90ef8012f7d21c5ea2d3bea05644c`. `isolated_bench`
  `2f56b833615c124624ba27192e4a0beeb43c34da71aa9742ea190aa72adb0a4e` and `koblitz_rho_fixture` `1344f0ce5be8f08a7cb2bc0d113d89d295e77fae1598b78f0ddf3d015f1a58ef` are built from `f465f2fb`.
- **The host:** this session's container: Intel Xeon @ 2.10 GHz, 4 vCPU,
  AVX-512 and VPCLMULQDQ, no hardware PMU, kernel `6.18.44-fc-v242`
  (`runs/host.json`). Rule v8 ran on an Emerald Rapids container; v8's
  figures are quoted beside v9's and never paired with them.
- **Route: local, not Iso Lab** (AGENTS.md §9a). The reachable EKS paths run
  `ecbench.spec/v1` specs only; this rule runs through `icprog rule`, and its
  control must share the host and session with `v9` to pair by target.

## Read-out

- Each run's cold ratio per size with its 95% bootstrap over targets
  (`analysis.json`), as every rule reports it.
- `icprog rule matched --comparison v9` (`matched.json`): per size, the
  paired ratios `v9 / v9c` of the cold ratio and of the index calculus's
  online time, with bootstrap intervals, and both runs' slopes.
- **The board:** if every check passes and the run validates under the
  2026-10-19 gate (replay certificate, validator report, this run's own
  in-process matched rho with the §5a band), the Koblitz Table A rows are
  re-read from rule v9's `analysis.json`, with v7's figures kept as the
  "before" marks. A failed check or validation leaves the board on v7.

## Prediction (not drawn from a run)

- The same walks and logarithms in both runs (rho is untouched by the
  change since v8 except where the rows say otherwise).
- `v9c` reproduces v8's cold ratios within this host's difference from v8's;
  no range is declared, because the host differs.
- At 2^44.3 (`n = 53`, aimed collection), the index calculus's cold time
  `v9 / v9c` lies in [0.85, 1.00]: the aimed gather alone was about 4% of
  set-up in its study, and the other changes since v8 are not predicted.

## Success and stop

The rule makes no acceptance decision. It is a measurement.

**Stopped**, as v8, if a row fails its check, a logarithm differs from §23's,
the pin fails, or the reference check finds a run's rho slower a step than the
strong walk at some size (that size is then reported as not run for that run).

## Commands

[`run.sh`](run.sh): for `v9` and `v9c`, `manifest` and `pin`; then for each size,
`size` for `v9` and then for `v9c`; then for each run `reference`, `claims`
and `analyse`; then `icprog rule matched --comparison v9`. Every step runs
under `tools/benchrec.py run <record> -- …`.

## Amendment 1 (2026-11-06, 17:20 UTC, before any row of the rerun)

**What happened.** The first attempt ran locally from 03:32 UTC. Its
background job was stopped by the session's two-hour limit at 05:32 UTC,
resumed at 15:35 UTC, and then lost with its container, which was replaced
at about 16:36 UTC. Its rows (sizes 1,47, 0,57, 0,41 and 0,53 for both runs,
and part of 1,59) were never committed and no longer exist. No
analysis was run on them; the only figures seen were the per-row "online
speedup" lines the log printed. Nothing below is chosen on those lines: the
question, the sizes, the arms, the order, the checks and the prediction are
unchanged, and the rerun starts from an empty run tree.

**Route: Iso Lab on EKS** (AGENTS.md §9a; [`launch/README.md`](launch/README.md)),
replacing "Route: local". One `isolab.eks.command-request/v1` Job on a
`c8a.2xlarge` Spot node (`cpu-l2-spot`, isolation profile `l2`, 4 CPUs,
12 GiB) runs [`launch/job.sh`](launch/job.sh): `run.sh`'s steps in `run.sh`'s
order (manifests, pins, the six sizes interleaved v9 then v9c, the reference
checks), syncing the run trees to S3 every 60 s. `claims`, `analyse` and
`matched` read only the rows and need git, so they run on the checkout after
the rows are downloaded and their hashes checked against the pod's
`SHA256SUMS`.

- **The host** becomes the pod's node (AMD EPYC, recorded in each run's
  `runs/host.json` and the Job's `host.txt`). Both runs still share one host
  and one process order, so pairing by target holds. Command Jobs have no L2
  admission, replay or A/A gate of the ecbench path; as in v8, the
  comparison is the in-process pairing of the index calculus with its
  matched rho, and the A/A repeats are §23's.
- **A Spot loss** is resumed by a new Job whose command names the lost pod's
  S3 prefix (`launch/make-request.sh`); `icprog rule size` resumes from the
  rows a run tree holds. At most two resumptions; a size split by a loss is
  reported as split.
- **The binaries** are rebuilt from the same commits (the first attempt's
  binaries were lost with the container). `isolated_bench`,
  `koblitz_rho_fixture` and `icprog` reproduce the first attempt's hashes
  exactly; the two `ic` builds do not (`ic-v9` `3ac9e26b…` → `fa00b978…`,
  `ic-v9c` `0effb6dd…` → `38819b1a…`), as same-source rebuilds here have not
  before (`research/arity_completion_20261104/launch/README.md`). Both pins
  were checked locally against rule v8's rows and held (development check,
  no row run). [`binaries.sha256`](binaries.sha256) now names the binaries
  that run; the image carries them (`launch/image.json`).
- **Bench record:** `tools/benchrec.py` cannot run inside the image, so the
  record is opened on the checkout, the Job's downloaded tree is attached with
  `add-raw` (raw data in S3 by SHA-256), and the local steps run under
  `benchrec run`.
