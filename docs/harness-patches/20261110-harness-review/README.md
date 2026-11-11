# crypto-autoresearcher harness review 2026-11-10: where the series lives

The ten patches that sat here were pushed as branch
`harness/review-20261110-fixes` in `isogeny-labs/crypto-autoresearcher`
(PR #3262) once this session got push access, and reconciled with PR #3180
there (commit `a3c81fa1a1`). The branch is authoritative; the patch files were
removed so two copies cannot drift.

`main` in that repository was vaulted on 2026-11-10 (`VAULT.md`: AES-256-SIV
clean/smudge filter keyed by `KEY`), so GitHub reports #3262 as conflicting and
no session without `KEY` can land it. To land it in an unlocked checkout:

```sh
export KEY=...
python3 tools/vault/vaultctl.py setup && python3 tools/vault/vaultctl.py unlock
git fetch origin harness/review-20261110-fixes
git merge origin/harness/review-20261110-fixes   # the clean filter stores the result as ciphertext
```

Verification results are in the PR description and in
`docs/session-logs/2026-11-10.md` (07:30Z entry) here.
