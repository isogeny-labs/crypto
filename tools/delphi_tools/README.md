# Delphi tool wrappers

Bash wrappers that let a Delphi program (repo `isogeny-labs/delphi`, see its
`docs/USAGE.md`, "Tool actions", "Claims and verifier tools", "Programs") run
this repository's gated tools from an operator allowlist. Delphi runs each tool
without a shell, stdin from /dev/null, cwd = the repository root.

| Tool | Argument (`args`) | Verifier | What it does |
|---|---|---|---|
| `card-lint` | `research/<dir>/<card>.yaml` (one) | yes | `python3 tools/task_card.py lint`; verdict pass on exit 0, else fail with the output tail |
| `card-run` | same (one) | no | `scripts/checkpoint.sh gate`, then `exec python3 tools/task_card.py run` |
| `lean-verify` | Lean project dir or `.lean` file (one) | yes | `tools/lean/proof-gate.sh --out`; pass on 0, fail on 1, exit 3 (invalid) on infrastructure error; `artifact_sha256` is the hash of the gate's receipt `build.log` |
| `isolab-dispatch` | `<40-hex sha>[,key=value...]` (one) | no | checkpoint gate, then `gh workflow run isolab-experiment.yml --ref main`; finds the run and writes `.delphi-tools/isolab/<run_id>.json` (gitignored) |
| `isolab-status` | GitHub run id (one) | yes | `gh run view`; `not_covered` while running, `pass` on success, else `fail` |
| `writeback` | round record JSON (file) | no | writes `knowledge/programs/<pid>/round-<n>.json`, regenerates `LEDGER.md`, appends a session-log entry, writes `problems/delphi-program-<pid>-r<n>.md` for a blocked round, then opens a PR on `delphi/<pid>-r<n>` with auto-merge |
| `make-tools-json` | none | no | prints a complete `tools.json` for this checkout |

`isolab-dispatch` keys: `spec_path`, `seconds`, `instance_type`, `resumable`,
`size_sweep`, `checkpoint_seconds`, `max_restores`. Example:
`<sha>,spec_path=taskq/deploy/eks-karpenter/l2-validation.json,seconds=1800`.

Verifiers print exactly one `pcc.verdict/v1` object on stdout and exit 0 even on
a fail verdict; diagnostics go to stderr. `DELPHI_CLAIM_ID`, when set, becomes
`claim_id`.

## Start a program

```sh
tools/delphi_tools/make-tools-json > /tmp/tools.json
delphi program start ... -tools /tmp/tools.json -proof-required \
  -writeback /home/user/crypto/tools/delphi_tools/writeback
```

## Environment overrides

- `DELPHI_TOOLS_ROOT`: repository root (default `git rev-parse --show-toplevel`).
- `DELPHI_TOOLS_SKIP_GATE=1`: skip the checkpoint gate (tests only; reported on stderr).
- `DELPHI_TOOLS_NO_GIT=1`: `writeback` writes files but runs no git or gh (tests only).
- `DELPHI_TOOLS_POLL_SLEEP`: seconds between run lookups in `isolab-dispatch` (default 3).
- `DELPHI_CLAIM_ID`: copied into verdicts.

## Tests

`bash tools/delphi_tools/test.sh` (offline, stub `gh`, fake proof gate);
`shellcheck -s bash` on every script. CI: `.github/workflows/delphi-tools.yml`.

## Limits

- Experiment receipts stay in private S3. `isolab-status` proves only that the
  workflow run succeeded; verifying the S3 receipts is the workflow's job.
- `lean-verify` evidence is the gate's receipt hash, not a replay certificate;
  the validation gate (`docs/VALIDATION_GATE.md`) still applies before any
  leaderboard or benchmark row.
- The lead atlas lives in crypto-autoresearcher and is not written here.
- Write-back records are the program reviewer's proposals until validated.
