# scout-repo-wide: lab-wide work and leads (2026-11-18)

**Question.** What work and leads exist across the lab beyond the graph-IC lane?
The scout looked for four kinds:

- stale threads;
- results that were never validated;
- leads whose blocker has since been removed;
- new technical leads toward the index-calculus baseline or rho.

It also checked whether the RunPod pods reported as RUNNING are orphaned
according to the repository's own records.

**Output.** `LEADS.md` (ranked). All content is **[pcc:unverified]**.

## Commands run (all read-only)
- Read `docs/ACTIVE.md` and the section heads and cited sections of
  `docs/session-logs/2026-11-18.md`, `2026-11-17.md` and `2026-11-18-hybrid-e8.md`.
- Read the sibling `/home/user/cryptanalysis` `docs/ACTIVE.md` and
  `docs/session-logs/2026-11-17.md`. That checkout is at cfc03a37e, 2026-11-17,
  so it may be stale.
- `git grep` / `grep` for the pod names; `git log` / `git show --stat` on
  recent merges.
- `python3 tools/benchrec.py check`: it fails on
  `20261117T0334Z-hec-multi-production-cold.json`.
- `gh api repos/isogeny-labs/crypto/pulls/<n>` and `pulls?state=open`, plus
  `pulls/<n>/files` patches scanned for the pod names. GraphQL `gh pr view` is
  refused in this runtime, so REST was used.
- `scripts/research-kb status`: "no executable found". **research-kb was
  unavailable**, and no lead atlas exists (`tools/lead_atlas.py` is absent and
  there is no crypto-autoresearcher checkout).

## Results (headline)
- The infrastructure blocker with the most impact is the malformed
  hec-multi benchrec record. It still fails `check` on main, and it keeps 16
  records open, along with #4738's record.
- `replead-r1-20261116` is the orphan candidate. It has been idle since at
  least 2026-11-17 06:02Z, it was kept only to copy `/work/out/`, and no
  `runs/` directory exists in the repository.
- The `hky-j*` and `bl-ladder` pods are owned by the active draft PR #4735.
  That PR states the in-pod lifetime cap does not work.
- The largest unpromoted counted result is Table B `tw2l` at 0.41–0.48x of
  the incumbent (#4738). It has no validation and its A/A checks determinism
  only.
- Three merged PRs (#4679, #4681, #4686) carried only tracking content, yet
  their ACTIVE lines stay unticked and the work was never done.

## Failures and limits
- No knowledge-base or atlas coverage, so no novelty statements are made.
- The 42k-line session log was sampled by section heads, not read in full.
- RunPod state was not queried. Pod status comes from the task brief plus
  repository records only.
- The sibling-repository state may be stale.

## Next steps
See the ranked summary at the end of `LEADS.md`. Items A1, A2 and E need the
owner. B1/C1 is the best local, unpaid technical step.
