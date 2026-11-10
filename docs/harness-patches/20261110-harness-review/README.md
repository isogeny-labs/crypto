# crypto-autoresearcher patch series: harness review 2026-11-10

Ten patches implementing `docs/autoresearcher-harness-review-20261110.md`
(I1–I12, I6b, instruction diet) for `isogeny-labs/crypto-autoresearcher`,
authored on branch `harness/review-20261110-fixes` from main `3ba0739cb9`.
They live here because the authoring session had read-only access to that
repository. Apply in the harness checkout:

```sh
git checkout -b harness/review-20261110-fixes 3ba0739cb9   # or rebase onto main
git am /path/to/crypto/docs/harness-patches/20261110-harness-review/*.patch
python3 tools/check_instruction_budget.py && python3 tools/check_lab_parity.py
```

Verification results and the per-patch contents are in
`crypto:docs/session-logs/2026-11-10.md` (07:30Z entry) and in each patch's
message. Delete this directory once the series is on the harness `main`.
