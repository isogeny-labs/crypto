# Native port of the tournament runner

AGENTS.md, "Implementation language: no Python", forbids running or extending
`tournament.py` for new research. This note maps the runner's stages to the
native replacement, `icprog tournament` (`src/bin/icprog/tournament*.rs`), and
lists what has been ported and what is deferred. A future registered round
runs without Python. Historical rounds keep their frozen Python evaluators as
evidence, and the native `verify` re-derives their verdicts.

## Which protocol is ported

`tournament.py` has two contract schemas.

- **Schema 1: the improvement tournament.** Every retained `runs/round-*`
  directory uses it. The latest frozen evaluator is `tournament.py`
  `1e5b27743bf3bf99a3834882ddde4992f658874438576e7532646feb655651cc`
  (894 lines; rounds 0019 to 0025). Its checker `oracle.py` has three pinned
  versions:
  - `9b760afa…` (rounds 0019 and 0020), degrees 5..31;
  - `528c14b3…` (rounds 0023 and 0024), degrees widened to 5..61;
  - `ae8f9b2a…` (round 0025), which also permits the summand-count amendment.

  **This protocol is ported in full.**
- **Schema 2: scientific admission.** The current 1,506-line `tournament.py`
  writes schema 2 for bounded campaigns v1/v2, reference qualification and
  generic workers. It adds `driver_admission.py`, `campaign_rules*.py`,
  `target_history.py`, `portfolio.py`, `qualification.py` and
  `generic_*.py`. **It is deferred** (see the deferred table below). The
  native `verify` and `run` refuse a schema-2 contract instead of
  approximating it.

## Stage map

| `tournament.py` (1e5b277) | Native | Status |
|---|---|---|
| `objhash`, `digest`, `write` (sorted keys, indent 2, exclusive create) | `tournament::objhash`, `tournament_store::digest_file`, `tournament::write` on `json.rs` (Python float repr, ASCII escapes) | ported |
| `candidates()`, `proposals_from_previous` (`propose`) | `default_candidates`, `proposals_from_previous` (`tournament propose`) | ported |
| `child_env`, `execute`: affinity pinned around spawn, no fork of the evaluator, `prlimit` AS/CORE before job delivery, watchdog started before the timing window, blocking reap, `killpg` on timeout | `tournament_exec::execute` (Rust `posix_spawn` path with its own process group; the Python version used `setsid`) | ported; the `spawn` string records the native spawn |
| `parse_profiles` (phase schema 1, `Collected` checksum) | `tournament::parse_profiles` (`str.splitlines`, `str.strip`, the regex, all ported) | ported |
| `frozen_inputs` (seal, pinned files, frozen evaluator and checker) | `frozen_inputs`. For a Python round: checks the pinned `evaluator/tournament.py` and `oracle.py` copies, accepts only `1e5b277` and the three oracle versions. For a native round: requires `evaluator/icprog` and that the running executable is that frozen copy | ported |
| `snapshot_build` (Cargo files, `src/`, literal `include*!` dependencies, worker source; offline locked build in the snapshot) | `snapshot_build` | ported |
| `prepare`: evaluator copy, candidates, cells, holdout, per-cell confirmation allocation (raises only), MT19937 fixture seeds, worker fixture generation, `Curve` check, calibration, contract, seal | `tournament::prepare` | ported. Additions in the table below |
| `run_trial`: Callgrind profile, oracle check, exclusive phase set, native rerun, answer agreement, failure classes TIMEOUT/OOM/INVALID_OR_INCOMPLETE, gzip `mtime=0`, artifact digests, exclusive receipt | `run_trial` (gzip via system zlib, byte-identical to CPython's `gzip.compress`) | ported |
| `verify_receipt` | `verify_receipt` | ported |
| `oracle.verify` and `oracle.Curve` (group law, column logs, rank mod r, descents, `factor_base_sha256`) | `tournament_oracle` (own arithmetic; Miller-Rabin replaces trial division, which tests the same predicate) | ported, three versions |
| `comparison`: per-case median ratios, per-cell log means, nested curve/target bootstrap with `random.Random(751203)`, `choice` and `choices` | `comparison` with `pyrandom` and exact `statistics.mean` (rational mean, correctly rounded), `statistics.median`, Python true division | ported |
| `gate`, `rho_gate`, A/A gate, parity margin | `gate`, `rho_gate`, `summarize` | ported. A wiring control never promotes |
| `stage_arms`: smoke failures dropped, top two development candidates, one provisional challenger | `stage_arms` | ported |
| `summarize` (including CPython 3.12 compensated `sum`) | `summarize` + `py_sum` | ported |
| `decision`: confirmation and replay gates, challenger/rho, base completeness, rho/winner, parity, strict-rho verdict | `decision` | ported |
| `run_campaign`: `flock`, earlier stages required, A/A must pass, seeded schedule and arm-order shuffles, process budget, `state.json`, no retries, interrupted trials refused | `run_campaign` | ported |
| `audit` (`verify`): sources, every receipt, every summary and the decision re-derived | `audit`. It also reads a committed `evidence/*.tar.zst` in memory (`tournament_store`), rebuilding each gzip profile with zlib level 9 and checking its PAX SHA-256 as `restore.py` does | ported |
| `status` | `tournament status` | ported |

### Native additions (all additive; frozen keys keep their meaning)

- **`native_evaluator`, `evaluator_sha256.icprog`.** The contract names the
  port (`icprog-tournament-schema1-v1`) and the semantics it reproduces. The
  frozen copy is `evaluator/icprog`.
- **`registration` and `purpose`.** A round is either `registered` (needs
  `--protocol FILE`, copied into the round and pinned as `protocol.md`) or a
  `wiring-control` (profile `wiring`: one case per stage and cell; may use a
  prebuilt worker; never promotes).
- **No default seed.** `--seed` is required, and `prepare` refuses every
  consumed or closed seed.
- **`public_targets` in each case's job.** These are the fixture's own
  points. The current worker measures only a supplied point; the current
  `tournament.py` adds the same field.
- **`run` refusals.** `run` refuses:
  - any round without `native_evaluator`, which covers every historical round,
    bounded rounds 1 to 3 and every closed registration;
  - any seed in `CLOSED_SEEDS`: all retained round seeds, `2026092551`–`53`,
    `2026092901`, `2026092902` and `2026093001`;
  - bounded-campaign fields;
  - qualification purposes;
  - a registration kind that does not match the purpose.

## Deferred, and why

| Piece | Why deferred |
|---|---|
| Schema-2 scientific admission (`driver_admission.py`: `freeze_admission`, `check_admission`, `run_record`, `online_table`; `identity.py`, `measurement.py`) | It is a separate record family with its own hashes (run numbers above 2^128, admission receipts, online-interval tables). Porting it needs its own acceptance replay on `ic-improvement-round*` or the reference-qualification archive. |
| Bounded campaign rules v1/v2 (`campaign_rules*.py`, familywise rule, `target_history.py` exclusions, supplemental exposures) | All three attempts are consumed. A new bounded campaign needs a new frozen protocol, which is out of scope here. |
| Reference qualification (`qualification.py`, rho width panels) and generic workers (`generic_*.py`) | These are separate registrations. All of them are consumed or censored. |
| `portfolio.retain` exploration slots | This belongs to the schema-2 development stage only. Schema 1 selects the top two. |
| Evaluators before `1e5b277` (rounds 0002 to 0018b) | Their semantics differ, for example `preexec_fn` timing and different oracles. They stay with their own frozen Python evaluators, and the native `verify` refuses them by hash. |
| `report.py` (REPORT.md, measurements.json) | Reporting, not execution. A native round's REPORT.md, graph and PDF follow the `research-visuals` skill. |

## Acceptance evidence (2026-10-28)

### Historical rounds

Each committed archive was verified in memory, without being restored to
disk. The native `verify` re-checks every trial receipt and artifact digest,
reruns the oracle on every profile and native report, re-parses every
Callgrind interval, and re-derives every stage summary (including the
2,000-draw bootstraps) and the decision. Field by field, its output equals
the `audit.json` the frozen Python verifier wrote for each round.

```sh
icprog tournament verify --archive evidence/round-0020.tar.zst \
  --member runs/round-0020 --manifest evidence/manifest.json
```

| Round | Oracle | Native `verify` | Frozen `audit.json` | Wall time |
|---|---|---|---|---|
| 0019 | 9b760afa | VERIFIED, 2,646 receipts, 452 source files | identical | 17 s |
| 0020 | 9b760afa | VERIFIED, 2,646 receipts, 452 source files (archive SHA-256, member count and bytes also matched `manifest.json`) | identical | 14 s |
| 0023 | 528c14b3 | VERIFIED, 3,648 receipts, 453 source files | identical | 69 s |
| 0024 | 528c14b3 | VERIFIED, 4,656 receipts, 453 source files | identical | 54 s |
| 0025 | ae8f9b2a | VERIFIED, 4,656 receipts, 453 source files | identical | 53 s |

Two further checks:

- Every gzip profile in these archives was rebuilt with system zlib and
  matched its recorded SHA-256.
- Round 0018b, which pins evaluator `f9feaafc…`, is refused by hash.

### Wiring control

**This is not a result.** It used the profile `wiring`, cells `7a1` and
holdout `13a0`, the arms `incumbent` and `batch16` plus rho, and seed
`4242000002`. It is not a registration and was not committed.

```sh
icprog tournament prepare --out $TMP/wiring-cli --source-root . --seed 4242000002 \
  --wiring-control --profile wiring --worker-binary target/release/examples/ic_tournament_worker \
  --candidates cands.json --cells 7a1 --holdout-cells 13a0 --objective rho --require-native-progress
$TMP/wiring-cli/evaluator/icprog tournament run --round $TMP/wiring-cli
$TMP/wiring-cli/evaluator/icprog tournament verify --round $TMP/wiring-cli
```

- **Stages.** All six stages ran: 69 Callgrind and native trial pairs, all
  verified, in 59 s. The A/A gate passed.
- **Decision.** The provisional challenger was `batch16`. The decision was
  `retained`, because the wiring control cannot promote and the challenger
  failed the matched-rho gate.
- **Verify.** It reported VERIFIED with 69 receipts.
- **Refusals.**
  - A second `prepare` into the same directory is refused.
  - Re-running `run` executes nothing and leaves `decision.json` unchanged.
  - A one-byte change to a native `stdout.json` is reported as
    `changed trial artifact`.
  - A different evaluator binary is refused.
  - `--seed 2026092553` is refused as a closed registration.

### Tests

The tests are in `src/bin/icprog/tournament_tests.rs` and the store and
oracle modules. Run them with:

```sh
cargo test --release --bin icprog tournament
```

The opt-in tests need environment variables:

- `historical_round_verifies_to_its_frozen_audit` needs
  `IC_TOURNAMENT_ARCHIVE` and `IC_TOURNAMENT_MEMBER`.
- `wiring_control_prepares_runs_and_verifies` needs `IC_TOURNAMENT_WORKER`
  and `IC_TOURNAMENT_HOLDOUT=13a0`.

Run both with `-- --include-ignored`.

| Result | Detail |
|---|---|
| Passed | 17 of 17 |

What the tests cover:

- exact `statistics.mean`;
- CPython 3.12 `sum`;
- `median` and true division;
- the MT19937 `random`, `choices` and `shuffle` draws;
- paired comparison eligibility and support rules;
- every promotion and rho gate threshold;
- top-two selection and dropping smoke failures;
- panel and allocation refusals;
- refusal of historical, closed, bounded and qualification registrations;
- gzip and zlib byte layout;
- primality and rank mod r.

## Commands for a future registered round

```sh
cargo build --release --locked --bin icprog
target/release/icprog tournament prepare --out runs/round-NNNN \
  --source-root . --seed <registered seed> --protocol <frozen PROTOCOL.md> \
  --objective rho --require-native-progress [--cells … --holdout-cells … --confirmation-cases …]
runs/round-NNNN/evaluator/icprog tournament run --round runs/round-NNNN
runs/round-NNNN/evaluator/icprog tournament verify --round runs/round-NNNN
```
