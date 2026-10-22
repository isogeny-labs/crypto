# Validation gate evidence for PR #1978's IC rows

**What this is.** The validation gate (owner rule 2026-10-19,
[`docs/VALIDATION_GATE.md`](../../../docs/VALIDATION_GATE.md)) admits a row to
`benchmarks/README.md` or the IC leaderboard only with a replay certificate, an
independent validator report and a matched rho reference with the §5a band.
PR #1978's rows landed after the gate's freeze, so each needed all three. This
directory holds the tool, the seed and the raw outputs; each evidence directory
holds its `VALIDATION.json`, `replay-certificate.json` and `validator-report.json`.

![Recomputed figures, verified scalars, and the distance to rho under both references](report/validation.svg)

PDF: [`report/report.pdf`](report/report.pdf).

## Result

- **16 evidence directories validated:**
  - 14 benchmark rows;
  - `rule/v8/`, which carries the leaderboard's six Table A rows;
  - `rule/v7/`, whose rows Table A cited before.

  `python3 scripts/validation_gate.py check --base origin/main` reports every
  row validated or frozen.
- **Every published figure recomputes.** 2,240 figures from the raw records:
  - the five rounds' A/A, suite and holdout intervals and rule v6/v7/v8's S
    figures agree to 1e-9, as do the 30 figures Table A shows for rule v8 and
    its control;
  - the explorations' paired cold times agree exactly, and their summaries
    agree at the three decimals they print.
- **Every recorded scalar verifies.** 7,650 records and 15,300 certificates
  (both arms of every process) pass `[s]G = T` and `[n]G = O`, and the scalar+1
  negative control is rejected each time. The curves are GF(2^19) to GF(2^61).
- **64 of 64 native replays are identical.** Four records per directory were
  drawn by the seed committed in [`REPLAY_SEED.md`](REPLAY_SEED.md) (commit
  `766d2dede`, before any replay; amendment 1, commit `e9cdf82c0`, added rule v8
  and v8c before they were replayed). Each was rerun and reproduced the same
  certificates, counts and flags:
  - every round and exploration ran with its own recorded binary, hash-checked;
  - rule v8 and v8c ran with `ic` rebuilt from their recorded commit `370e1919`
    (`binary_rebuilt` in each run), because the original binary was not on this
    host.

## The reference: measured rho, and Table A on rule v8

Each `ic price --single-target` process records two figures for rho on the same
point:
- **rho as measured:** its set-up plus its walk, as it ran;
- **a model:** the walk's counted operations priced at the canonical batched
  step the same process measures (`rho_model`, which the record calls "a model
  beside the measured interval, not a measurement of it").

Every `VALIDATION.json` names the measured rho of the row's own process as its
reference. That is the reference each protocol names. Its walk uses negation and
Frobenius, which is the strongest fold for these curves in the gate's sense.
The model is recorded beside it (`rho_canonical_step_model`) and is not a
reference.

The order holds across the §5a band: rho is faster at 0, 0.1 and 1 GAE for each
of rho's uncharged canonicalisations.

**The model overstates rho.** Rule v8 measured a matched rho, its
canonicalisation batched 16 lanes at a time. It is 1.30–1.58× faster cold than
rho as at rule v7 on the same binary (rule v8c), while the model assumes about
twice. At 2^47.2:

| reference | IC / rho at 2^47.2 |
|:--|--:|
| rule v7, rho as built | 11.51 |
| rule v8, the matched rho (measured) | **18.07** |
| rule v8, the canonical-step model | 23.32 |
| rule v8c, rho as at v7 (same binary) | 11.56 |

**Table A now cites rule v8** (owner's decision, 2026-10-22). It shows the
measured matched rho, 4.67–18.07×, with its published interval. Beside it are
the same binary's control (rho as at v7) and §23's ratio against rho as built.
An earlier commit in this PR had switched Table A to the canonical-step model.
That was replaced on finding that the model is not a measurement.

The hand-written pages that quoted rule v6 or v7 now say those ratios are
against rho as built, which was not matched, and point to rule v8's measured
figures:
- `benchmarks/README.md`;
- the programme index (`research/ic_tool_program/README.md`);
- the ledger, `IC_TOOL_PROGRAM.md`;
- the rule v6 and v7 results;
- R11, R12 and A17.

The measured v6 and v7 figures themselves are unchanged. The scoreboard already
cited rule v8.

## Rows withdrawn from `benchmarks/README.md`

- **`20261017-ic-a20-build-line-streams`.** It is a smoke check transcribed from
  a terminal, and its own README says "not a measurement". It has no raw
  records, so it cannot be validated. The research directory is unchanged.
- **`20261017-two-sylow-fp64`.** Main archived it (`benchmarks/ARCHIVE.json`).
  An old merge had restored its line here.

## How it ran

- **Tool:** [`src/bin/ic_validate.rs`](../../../src/bin/ic_validate.rs), native
  Rust. It does not use the crate's library or `icprog`. It has its own GF(2^m)
  field, Koblitz group law and statistics, and reads only the raw
  `*.price.json` and `*.isolation.jsonl` records.

      cargo build --release --bin ic_validate
      ic_validate certverify <runs.tar.xz | runs/> --out raw/certverify-<key>.json
      ic_validate recompute <round|rule|exploration|stages> <dir>
      ic_validate replay --bin <binaries> --scratch <dir> --out raw/native-replay.json <dirs...>
      ic_validate build [--check]

- **Definitions.** These come from the protocols (IC_TOOL_PROGRAM.md §5; R01
  PROTOCOL, "Accounting"), not from `icprog`'s code:
  - **Cold time:** the median over a process's repetitions of set-up plus the
    online interval.
  - **A process's figure:** its first attempt that the isolation record does not
    mark contended.
  - **Paired comparison:** the geometric mean of the ratios, with a 95% Student t
    interval.
- **Host:** `Linux 6.18.44-fc-v140 x86_64`, Emerald Rapids, the rounds' host
  class. The rounds ran on kernel build `-v139`. Replays compare outputs, not
  times.
- **Binaries:** each round's and exploration's SHA-256 matches its `analysis.json`
  (`host.binaries`) or its `binaries.sha256`. Rule v8 and v8c used `ic` rebuilt
  from `370e1919` with `--locked`. Its bytes differ from the recorded binary,
  because a build embeds its paths.
- **Rule v8c's setting.** Rule v8c set `KIC_RHO_CANON=scalar` (`rule/v8/run.sh`),
  and its replays set it too. Its records differ from rule v8's only in timings,
  so a replay confirms v8c's counts and scalars, not which canonicalisation ran.
- **Disclosures.**
  - A first replay pass was driven by a Python script, against AGENTS.md
    ("Implementation language: no Python"). Its log is
    [`raw/native-replay-attempt1-python-driver.log`](raw/native-replay-attempt1-python-driver.log).
    It stopped at A18, whose parameter files were renamed after the run
    (`k0n61-…` became the curve slug). The tool now maps the recorded name to
    the renamed file and records the mapping in each run's `params_renamed`.
    The replays reproduced identical outputs, which shows the contents are the
    same.
  - The Python tools were replaced by `ic_validate` before anything was
    published. `ic_validate` reran every step and drew the same 56 records.
