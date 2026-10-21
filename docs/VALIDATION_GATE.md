# Validation gate

Owner rule, 2026-10-19 (AGENTS.md, "Standing workflow rules"). No row reaches
the IC leaderboard (`docs/ic/LEADERBOARD.md`, `docs/ic/leaderboard.json`,
`docs/ic-leaderboard.html`) or `benchmarks/README.md` unless it carries:

1. a **replay certificate**: independent replay of the recorded runs, scalars verified;
2. a **validator report**: an independent recompute of the headline numbers from
   raw data that is in the repository;
3. a **rho reference matched to the protocol**: the reference the protocol names,
   the strongest available for that curve (negation, Frobenius or the
   automorphism fold), in the same unit and the same binary or process, with the
   AGENTS.md §5a band at 0, 0.1 and 1 GAE per unpriced native operation.

The gate decides what may be *shown* as a result. It does not forbid recording a
null or negative result in a research directory.

An unvalidated benchmark row may be removed from the admitted `benchmarks/README.md`
table only with an archival disposition in `benchmarks/ARCHIVE.json`. The archive
preserves the exact former row text and digest, a reason (`incomplete`, `censored`,
`documentary`, or `superseded`), and repository paths to supporting provenance.
The CI check requires each row removed from the base table to have that exact
disposition, rejects archive additions that do not match a base row, and prevents
existing archive entries from being changed or dropped.
An archive record never validates a row: if the row remains in the admitted table,
the ordinary validation gate applies. See [`benchmarks/ARCHIVE.md`](../benchmarks/ARCHIVE.md).

## `VALIDATION.json`

Lives in the row's evidence directory: for a leaderboard row, the directory of its
`source` file (`dirname(row.source)`); for a benchmarks row, `benchmarks/<dir>/`.
Paths are relative to the evidence directory and must stay inside the repository.

```json
{
  "schema": "validation-gate/v1",
  "rows": ["icv1-..."],                       // optional: slugs it covers; default all rows of the directory
  "replay_certificate": {"path": "replay-certificate.json", "sha256": "…"},
  "validator_report":   {"path": "validator-report.json",   "sha256": "…"},
  "rho_reference": {
    "kind": "negation | frobenius | automorphism_fold | negation_frobenius | no_symmetry_available",
    "protocol": "the reference the protocol names",
    "unit": "counted group operations | GAE | …",
    "binary_or_process": "the binary or process both arms ran in",
    "matches_protocol": true, "strongest_available": true,
    "same_unit": true, "same_binary_or_process": true,
    "gae_band": {"0": {"ic": 0.0, "rho": 0.0}, "0.1": {"ic": 0.0, "rho": 0.0}, "1": {"ic": 0.0, "rho": 0.0}},
    "order_holds_across_band": true           // rank two arms only where this is true
  }
}
```

The two pinned files must hash to their `sha256`, and must say:

- replay certificate: `status` is `verified`/`pass`, `independent: true`,
  `scalars_verified: true`, a non-empty `runs` list;
- validator report: `status: "pass"`, `independent: true`, a non-empty `raw_data`
  list of repository files (each may pin a `sha256`), and a non-empty `headline`
  list of `{name, claimed, recomputed, rel_tol?}` whose numbers agree within
  `rel_tol` (default 1e-9).

`python3 scripts/validation_gate.py evidence DIR` explains one directory.
`scripts/fixtures/validation_gate/` holds a complete toy example (`positive/`) and
a refused one (`negative-no-rho-reference/`); the numbers in them are not a
measurement.

## Row standing

| standing | meaning |
|:--|:--|
| validated | a complete `VALIDATION.json`, or a passing sweep verdict that names the three artifacts |
| unvalidated (pre-gate) | frozen in `docs/ic/validation/PRE_GATE.json` / `benchmarks/PRE_GATE.json` with the digest it has now |
| unvalidated | not frozen, or changed since the freeze, and not validated: the builder refuses it, CI fails |
| failed validation | a sweep verdict says `failed`: the builder refuses it |

Precedence: a failing sweep verdict, then a complete `VALIDATION.json`, then a
passing sweep verdict, then the freeze. A verdict carrying a `row_digest` that no
longer matches the row is ignored.

**Validation sweep verdicts.** Slice A lands
`research/validation_sweep_20261019/A-leaderboard/verdicts.jsonl`, slice B
`…/B-benchmarks/verdicts.jsonl`; the builder and the check read them when they
exist. One JSON object per line: `row` (a board key `board:<slug>:<source>`, a
slug, or a benchmarks directory), `verdict` (`validated` | `failed`), the three
keys `replay_certificate`, `validator_report`, `rho_reference` (non-empty: where
each artifact is), and optionally `row_digest`. **The sweeps had not landed when
this gate was written, so this line format is the gate's assumption**; a sweep
that writes another shape needs the reader in `scripts/validation_gate.py`
(`load_verdicts`, `verdict_status`) adjusted in the same PR.

## Mechanics

- `scripts/build_ic_leaderboard.py` annotates every board row with `validation`
  (standing, evidence directory, reasons), refuses `unvalidated` and `failed`
  rows, and renders the standing in `LEADERBOARD.md` and the HTML. `--allow-unvalidated`
  is for local builds and marks such rows instead; `--check` never takes it.
- The freeze digests a leaderboard row as the SHA-256 of its JSON without
  `validation` (floats rounded as the committed JSON), and a benchmarks row as
  the SHA-256 of its README line(s). Changing either makes it a changed row.
- `.github/workflows/validation-gate.yml` runs `scripts/test_validation_gate.py` and
  `scripts/validation_gate.py check --base origin/<base>` on every pull request. The
  check also fails when a freeze file gained or re-digested an entry, so the
  freeze cannot be used to launder a new row.
- `.github/workflows/audit-runs-daily.yml` and `.claude/skills/audit-runs` re-audit
  finished sessions daily with a nonce-drawn sample.

## Making a row pass

1. Put the replay certificate, validator report and raw data in the evidence
   directory (or cite repository paths).
2. Write `VALIDATION.json` with their SHA-256 and the rho reference descriptor.
3. `python3 scripts/validation_gate.py evidence <dir>` until it says `valid`.
4. Regenerate: `python3 scripts/build_ic_leaderboard.py`. If the new
   `VALIDATION.json` is read by the builder, `scripts/check_ic_leaderboard_paths.py`
   will tell you which path the `ic-leaderboard` workflow filter must add.

## Keeping the board rigorous and current

The gate above decides what may be *shown*. Four further checks keep what is shown true and fresh.
They came out of the 2026-10-19 validation sweep (slice A,
`research/validation_sweep_20261019/A-leaderboard/REPORT.md`), whose findings were: every published
number recomputed, but the standing rho had changed under Tables A and C, six sessions' counters no
longer replayed, and nothing in CI would have noticed either.

| layer | what it catches | where | cost |
|:--|:--|:--|:--|
| Recompute | a board number that does not follow from its raw records | `scripts/ic_audit/check_board_recompute.sh`, workflow `ic-leaderboard-recompute` (every PR, every push to main) | jq, 5 s |
| Reference currency | the standing rho changed after a table's reference was measured and nobody assessed it | `scripts/ic_audit/check_reference_currency.sh`, `docs/ic/validation/reference-currency.json`, same workflow | git, 1 s |
| Replay plan | a cited session whose counters stop replaying on main (new drift), a planned drift that has healed, or a cited session missing from the plan | `scripts/ic_audit/replay_plan.sh`, `check_plan_coverage.sh`, `docs/ic/validation/replay-plan.json`, workflow `ic-leaderboard-replay` (daily, on dispatch) | one release build, minutes |
| Expiry | rows that stay "unvalidated (pre-gate)" for ever | optional `"expires": "YYYY-MM-DD"` in either `PRE_GATE.json`; after that date every row still only frozen fails `validation_gate.py check` | none |

**Recompute check.** It reruns `recompute_A.sh`, `recompute_B.sh` and `recompute_C.sh` (jq, sharing no
code with the Python builder) and compares S (IC), S (reference), x reference and x floor of every
Table A, B and C row with `docs/ic/leaderboard.json`, relative tolerance 1e-8. It also fails when the
number of recomputed rows differs from the number of board rows. `test_check_board_recompute.sh`
proves it catches a 1% change of each compared field in each table and a deleted row.

**Reference currency.** A table's rho reference was measured at a commit (`measured_at`). The entry
records the git blob of each rho source file at the time its effect on the table was last assessed
(`assessed_blobs`) and the assessment itself. Change the rho source and the check fails until the entry
is updated, which means writing down either a re-measurement or a measured estimate of the effect.
Blob hashes, not commit hashes, so squash merges cannot break it.

**Replay plan.** `ecbench verify --replay-all` on each session the board cites, run daily on current
main. Each session is planned as `identical` or as `drift` with a reason. A drift entry is a visible,
dated debt, not a pass. When a session drifts that was planned as identical, or heals when it was
planned as drift, or a newly cited session has no entry, the workflow fails.

**A row cannot be both validated and unreplayable.** Until a session's counters replay on main, any
board row that depends on them stays out of the validated standing; the sweep verdicts of slice A
(`verdicts.jsonl`) record which rows those are.

### What each standing needs, for the rows on the board today

| table | replay | validator | rho reference with the 5a band | standing |
|:--|:--|:--|:--|:--|
| B | native re-execution of the ladder, 13 of 13 instances identical | recompute from per-run gae | negation map, band computed for 13 instances | validated: `docs/ic/runs/VALIDATION.json` |
| A | native replay of sampled runs identical | recompute from the 64 R1 files | **band not available**: the unit is wall-clock and the price files record no per-operation counts for the IC arm | pre-gate, until a counted band exists |
| C | native replay of sampled runs identical | recompute from the pair files | **band not available**, and the reference is S20's, not measured with the IC figure | pre-gate, until both exist |
