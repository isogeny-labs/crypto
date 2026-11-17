# Validation-gate checkout repair, 23 October 2026

**Question.** PR #2437's prior-head `gate` job [failed before checkout completed](https://github.com/aburan28/crypto/actions/runs/39989976478/job/120538926945). Its GitHub annotation reports `System.IO.IOException: No space left on device` writing the hosted runner's worker diagnostic log. No validation test ran in that job. This is an infrastructure failure, not a validation verdict on the study.

**Change.** `.github/workflows/validation-gate.yml` now uses a two-commit checkout and a cone-mode sparse working tree containing `scripts`, `benchmarks`, `docs/ic`, `papers`, the validation-sweep records, and six research evidence directories currently required by the admitted rows. For a PR, it checks that the synthetic merge's second parent is the submitted head, then passes its first parent as the exact base to the gate. A new validated row citing another directory must add that directory to the sparse list; missing evidence fails the gate.

**Local control and measurement.** The baseline worktree's `research/` directory alone was measured with `du -sh research` at 7.3 GiB. On a separate sparse worktree at source commit `a53a7e2bfa20080a424b9a2484a16932b6e3c116`, the selected cone-mode paths occupied 740 MiB total by `du -sh .`. These are different storage scopes; the numbers demonstrate working-tree reduction but are not a paired runtime or total-runner-disk ratio. Host: Mac16,7, macOS arm64, Python 3.13.1. The exact path list, command output and checkout revision are in [ci-sparse-environment.txt](runs/ci-sparse-environment.txt).

```sh
git sparse-checkout set --cone benchmarks docs/ic papers research/validation_sweep_20261019 research/ic_oracle_setup_blocked_20261020 research/ic_tool_program research/isogeny-volcano-cost-v2_20261017 research/pdp_split_slots_20261012 research/two_sylow_fp64_20261017 research/volcano_ecdlp_census_20261017 scripts
git read-tree -mu HEAD
python3 scripts/test_validation_gate.py
python3 scripts/validation_gate.py check --base origin/main
du -sh .
```

**Correctness.** The [test output](runs/ci-sparse-tests.stderr) records 49 tests passing. The [gate output](runs/ci-sparse-gate.stdout) is `validation gate: every row is validated or frozen pre-gate`; both commands exited zero. Local testing validates path coverage and gate logic at the recorded revision. The new PR head's GitHub-hosted execution remains the release check; it must complete successfully before merge. This change makes no ECDLP performance claim.
