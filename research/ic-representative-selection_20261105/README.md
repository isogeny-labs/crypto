# IC representative selection on ℓ = 2 volcanoes: preregistered, not run

**State (2026-11-05): design frozen, nothing measured.** No job has been
submitted to Iso Lab, EKS or any cloud host, and Stage 0 (class generation)
has not run. This directory holds the preregistration and the native tooling
the run needs. It contains no result.

Source: crypto-autoresearcher lead `LEAD-STR-class-representative-ic-selection`
(`knowledge/leads/ecdlp/structure/`), next action. Prior measurements:
[`../isogeny-volcano-cost-v2_20261017`](../isogeny-volcano-cost-v2_20261017/README.md)
(cited as unverified assertions `[pcc:unverified]`; no pcc claim id exists for
them).

## The design in one table

| | 24-bit p (12 classes) | 28-bit p (12 classes) |
|:--|:--|:--|
| **A** floor instance on its floor curve | 8 groups × ≤ 4 floor curves × 8 targets × 5 rounds | same |
| **B** the instance carried to the crater (3 verified 2-isogenies, every field op charged) | 8 groups × 32 targets × 5 rounds | same |
| **B0** carried to its sibling floor curve (negative control) | as A | same |
| **Aaa** A under an independent seed (A/A) | as A | same |
| **Cdef / Csel** crater instance on model 0 / on the best of 16 models chosen on calibration targets, measured on held-out targets | ≤ 16 crater curves × 8 held-out targets × 5 rounds (all 12 classes) | first 6 classes (secondary) |
| **Crho** the selection rule run on rho (negative control); **Cdefaa** A/A | as C | as C |
| positive controls | planted 33-point base (yield 1.0635), BSGS exactness across A/B/B0, analysis power (×0.96 on A/A data) | same |
| matched rho | `rho-neg` on every workload; gap reported at w = 0, 0.1, 1 | same |

Primary family (Holm, α = 0.05): H_B24, H_B28, H_C24; each must hold at every
§5a weight (0, 0.1, 1 GAE per native operation). Power (derived from v2.1
variance, PROTOCOL.md §5): H_B ≈ 0.90 at w = 0 for the v2-sized 4.5% effect
(0.67 pessimistic); at w = 1 the 24-bit power is about 0.5 because the charged
ascent costs 1–2 S, so that case is reported `unresolved_across_band`; H_C24
0.97 if the true model spread is 5%, 0.31 if it is 3%.

Why 24 and 28 bits (the lead says 24–32): 24 bits replicates v2 on fresh
classes and is where the charged ascent matters most (≈ 1.3–2.5% of S at w =
1); at 28 bits the ascent is below 0.5% of S, so B is decided by the floor
effect alone. 32 bits is left out: one IC run there is about 9.5 s online on
c8a (`taskq/deploy/eks-karpenter/validation/ic-size-sweep-20261026`), so the
≈ 100,000 IC runs these powers need would take several hundred job-hours on
the single L2 worker. It is the next tier if 28 bits supports B.

## Files

| file | what |
|:--|:--|
| [`PROTOCOL.md`](PROTOCOL.md) | the narrative preregistration (hypotheses, population, arms, controls, power, route, analysis, decision rule) |
| [`protocol.json`](protocol.json) | `experiment.protocol/v1`, validated and frozen with `crypto-autoresearcher/tools/experiment_protocol.py` |
| [`design.json`](design.json) | every frozen parameter: class search, caps, seeds, targets, shards, cost model |
| [`comparisons.json`](comparisons.json) | the frozen analysis plan as data: 32 comparisons, the hypotheses, gates and power controls |
| [`INPUTS.md`](INPUTS.md) | amendment A1, the inputs freeze: pending until Stage 0 runs |
| [`run/`](run/) | thin shell for Stages 0–3 and [`run/CARD.md`](run/CARD.md), the route the director compiles into cards |
| [`native/`](native/) | `repsel`, a standalone Rust crate (only dependency `serde_json`): `ascend`, `specs`, `select`, `hold`, `mask`, `analyse`, `unmask` |

### Why new code

Nothing in the repository carries an instance up a volcano with the work
charged and then runs IC there: `transfer.isogeny` (ecbench) takes one step
with the smallest kernel and runs a generic method only, and `volcano-rho`
transports downward from its root without counting. Arm C needs a selection
step that sees only calibration data, and the paired, masked, band-priced
analysis needs one program. So `repsel` is minimal and native (AGENTS.md
"Implementation language"); ecbench stays the instrument and volcano-rho the
class generator, unchanged.

Tests (`cargo test --release --manifest-path native/Cargo.toml`: 16 tests):
field arithmetic and root finding; the ascent on the committed v2 class
`l2-s1001` (every floor curve reaches an exported crater vertex along its BFS
parents, path heights 0-1-2-3, sibling heights 0-1-0, deterministic counts);
the selection rule (ties, ineligible models); spec arms against the Iso Lab
contract; band pricing; the statistic (a planted 10% effect is found, a null
is not, seeded replay); Holm and the verdicts; and an end-to-end pipeline
(fixture class → transport → specs → synthetic sessions with a planted 5% B
effect → analysis, determinism and transport checks).

## How to run it (after review, by the experiment route only)

From the repository root, at the frozen commit:

```sh
D=research/ic-representative-selection_20261105
scripts/checkpoint.sh gate                    # RESUME.md pushed; needs a valid upload token
$D/run/00-build.sh                            # volcano-rho, ecbench, repsel; hashes to inputs/tools.sha256
$D/run/01-classes.sh 24 && $D/run/01-classes.sh 28   # class rule; selection logs
$D/run/02-specs.sh                            # specs, labels, jobs.tsv, MANIFEST.sha256
# file INPUTS.md (A1: digests, image receipt, mask nonce), amend protocol.json, commit, push
# then per job, through the director/executor cards of run/CARD.md:
$D/run/03-launch.sh <job>; $D/run/04-collect.sh <job>
$D/run/05-hold.sh <tier> <class>              # after a class's cal jobs; commit+push selection.json first
$D/run/06-analyse.sh                          # masked analysis, unmask, ecbench band cross-check
```

## What blocks running

- **Checkpoint token.** This session's `CHECKPOINT_UPLOAD_TOKEN` has expired,
  so `scripts/checkpoint.sh push` fails. `.resume/RESUME.md` is filled; the
  push is pending a fresh token, and the run gate is not bypassed.
- **Credentials and route.** The executor needs its AWS identity (account
  104186811093) for `remote/launch-remote`, and a `publish_only` dispatch of
  `isolab-experiment.yml` on `main` (repository Actions access) for the image.
- **Publication.** Commit pushed and the PR reviewed (`check-experiment-git`
  receipt) before any build or trial; nothing is pushed yet.
- **Stage 0** must run first (builds `volcano-rho` and `ecbench`; local, light)
  and A1 must be filed.
- **Conductor** is not installed in this environment (the hooks only warn);
  no Conductor task was attached.
