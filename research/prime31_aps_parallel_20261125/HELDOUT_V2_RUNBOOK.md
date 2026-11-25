# Prime31 APS/rho V2 cold equal-core runbook

Status: registered and awaiting admission. The V2 public targets have not
entered either solver. This file fixes the execution and analysis rule before
that happens. See `HELDOUT_V2_REGISTRATION.md` for target, curve, seed,
independent Sage replay, and input hashes.

SHA-256 pins for this prepared implementation:

| File | Digest |
| --- | --- |
| `prime31_aps_one_relation.rs` | `255c29f710a65d006c935a9a3586862db3f9e00f0eb261a6d1d227024774eb87` |
| `run_heldout_v2.py` | `56d4a07baa4aa0ebb39510c476adb4985b951b776ee8eb2ffce74a9552c2d70d` |
| `analyze_heldout_v2.py` | `543b2ca3f62c45aa6061785b6c15d17b70b191b382a1fa4f4b7bfd86d4de22eb` |
| `run_aa_control_v2.py` | `0f5cc01051ce9e552edf5b6807c5914a5075682713bd03dc73d057da29cb6df4` |
| `check_aa_control_v2.py` | `533770fc170c3ea02d21648e9000699843193f237b06266b204cac287d7f8ff7` |

## Frozen comparison

- One target per fresh child process, in the 720-call order of
  `heldout_v2_seed_manifest.json`: 48 public Q, five independent replicate
  streams, and three arms per stream. The public Q and resource budget are
  shared; APS and rho use their declared distinct source streams. Folded and
  unfolded rho use the same walk and logical-lane restart seeds.
- Primary comparison: queued APS against folded shared-DP rho. Unfolded rho
  is a secondary diagnostic, never selected after reading V2 outcomes.
- Four distinct physical cores per process. At most one child at a time on a
  dedicated Linux host. A fixed 2 GiB address-space limit and 30-second
  external per-call watchdog apply to every arm. Record external cold wall,
  child user+system CPU, peak RSS, exit status, timeout, native report and
  all worker/cell work. Record same-host isolation and A/A control evidence.
- Native APS: labelled point source `[384,384,16384]`, one packed S3 index,
  bounded eight-cell channel, 24 cells of 16 rows, four consumers, and the
  native source seed in the V2 manifest. The full point source currently
  precedes queue consumption; this measures parallel S3 consumption, not
  generation/consumption overlap. Charge the point source in every cold call.
  Streaming the third partition into workers under the same S3 lookup would
  miss points that arrive later. Inverting the lookup requires materializing
  up to `2 * 384 * 384 = 294912` pair-root entries before those points are
  consumed, even when the present row search stops early. That construction
  needs its own cost and parity screen before entering a future comparison.
- Native rho: four workers, 64 logical lanes, 64 shared jumps, DP bits 5,
  two-million-step cap, paired V2 walk seed and 64 explicit restart seeds.
  Each worker report and all speculative or peer-stopped work stay in the
  native report. A step-cap exhaustion, timeout, crash, interruption or wrong
  answer is a retained outcome. An error inside a cell marks its partial pair
  count unknown and the queue's work accounting incomplete.

## Admission and execution

`heldout_v2_admission.template.json` has `admitted: false`. First fill an
immutable `AA_ADMISSION.json` with the exact binary path/hash, source commit, compiler
flags, four Linux CPU IDs, dedicated-host evidence path/hash and same-host A/A
control evidence path/hash left blank. On the selected host, run
`run_aa_control_v2.py` with this file while admission remains false.
It runs five A/A rounds for each of the three arms on archived development Q0,
using the five case-0 V2 seed streams but no V2 public target. Within each
round the arm order goes forward for A and backward for B. Review the
`check_aa_control_v2.py` wall/CPU noise report and isolation evidence, then
hash the raw A/A JSONL into a separate immutable `V2_ADMISSION.json` with
`admitted: true`. Preserve the original `AA_ADMISSION.json` for the A/A checker.
A/A repeats check timing
noise and semantic stability; they do not count as independent solver
replicates.
The runner validates these fields and the seed-manifest digest before any
solver invocation. A/A and isolation evidence must be reviewed by the
operator; the runner checks their existence and hashes, not their scientific
adequacy. Resource use needs explicit authorization for the chosen host.

```sh
python3 run_aa_control_v2.py --admission AA_ADMISSION.json --output AA_RAW.jsonl
python3 run_aa_control_v2.py --admission AA_ADMISSION.json --output AA_RAW.jsonl --execute
python3 check_aa_control_v2.py --admission AA_ADMISSION.json --records AA_RAW.jsonl
python3 run_heldout_v2.py --admission V2_ADMISSION.json --output V2_RAW.jsonl
python3 run_heldout_v2.py --admission V2_ADMISSION.json --output V2_RAW.jsonl --execute
python3 run_heldout_v2.py --admission V2_ADMISSION.json --output V2_RAW.jsonl --execute --resume
python3 analyze_heldout_v2.py --admission V2_ADMISSION.json --records V2_RAW.jsonl
```

The commands without `--execute` are preflight only. The heldout `--execute`
starts the 720 calls after admission. The
runner fsyncs a start event before each child and its outcome after the child.
On `--resume`, it checks the exact manifest/admission/schedule prefix. An
orphan start is recorded as interrupted and charged as a censored call; it is
not replaced. Stop any surviving child before resuming after host failure.
If a crash leaves a torn JSONL line, preserve the original bytes and prepare
a reviewed recovery copy before resuming; the runner rejects malformed lines.
Keep the original JSONL, binary, admission, host evidence and A/A evidence.
Do not edit a raw record; append an explicit amendment outside the JSONL.

## Prospectively fixed analysis

`analyze_heldout_v2.py` checks all 720 outcomes, public Q, native seed,
four-worker configuration, caller-only scalar, queue-cell coverage and rho
fold mode. It reports every arm's status counts, total measured wall/CPU and
peak RSS. Interrupted calls have unknown time and memory, so totals remain
descriptive lower bounds when any are present. The primary paired statistic
is the geometric mean of rho/APS cold wall ratios over all 240 case/replicate
pairs, with a 95% percentile bootstrap interval resampling the 48 cases as
clusters and retaining their five replicates. The bootstrap has 2,000 draws
from the manifest's analysis seed. Child CPU uses the same paired analysis;
unfolded rho is secondary. If any member of a pair lacks a verified answer or
complete measurement, the script withholds that paired statistic. Report
success rates and censored cases regardless. A positive interval here is
evidence on prime31 under this budget, not an asymptotic or larger-curve
claim. Independent replay and the repository validation gates remain separate.
