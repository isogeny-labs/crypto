# Querying the shared research KB

This page is the indexed entry point for sessions that search `docs/` before
they list skills. The canonical procedure, with query recipes and the worked
receipt, is `.agents/skills/research-kb/SKILL.md`; this page points at it and
does not replace it.

## When to query

Before a prior-art or "has this been tried?" statement, a new lead, mechanism,
factor base, solver or coordinate, a "first"/"best"/"not known"/negative
claim, a cross-repository question, or a subagent handoff.

## How to query

```sh
scripts/research-kb search "point decomposition Curve25519" --top-k 6
scripts/research-kb search "KN-OPEN-002" --top-k 3      # exact ids resolve first
scripts/research-kb theorem "CryptoResearch.Semaev.S3_root_iff"
scripts/research-kb status
```

Prefer the exact identifier when it is known (`LEAD-*`, `KN-*`, `DEC-*`,
`RQ-*`, a theorem name) and a distinctive phrase from the record otherwise.
Filter with `--repo` when the answer must come from one repository.

## How to record it

Under "Sources checked" in the session log, task notes or pull request:

```
- Sources checked (KB):
  - source:... — repo:path — what it changed in the plan.
  - query "..." — no result in <repos>; not evidence of absence.
```

## Read the hit, not the score

`claim_status`, `authority`, `evidence_level` and `superseded` decide how much
weight a passage can carry; a score is relevance, not truth. Read the primary
file before a claim. No result does not establish absence.

## When the catalog is down

Record `KB unavailable: <why>` and fall back to the lead atlas
(`crypto-autoresearcher:knowledge/leads/ecdlp/`) and a file search; say which
repositories you searched.

The session-start hook now checks for the shared catalog. When it is absent it
prints that fallback and the receipt to write, instead of asking for a query
that cannot run.

The binding shared rule is `crypto-autoresearcher:docs/lab-contract.md` §11.
