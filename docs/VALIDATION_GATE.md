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
